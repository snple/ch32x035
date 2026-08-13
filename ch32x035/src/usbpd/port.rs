//! USB PD link layer: blocking transmit/receive on top of the raw
//! [`super::phy::Usbpd`] transceiver.
//!
//! A [`Port`] owns the transceiver together with the two DMA buffers and
//! implements everything above the frame level: Message-ID management,
//! GoodCRC acknowledgement, receive re-arming and (chunked) extended-message
//! transmission. The policy engines in [`super::policy`] are thin state
//! machines built on top of this.

use crate::delay::DelayUs;

use super::phy::{SopType, Usbpd};
use super::protocol::{
    ControlMessage, ExtendedHeader, Header, MAX_CHUNK_DATA, MAX_EXTENDED_DATA, Message, PdError,
    RxExtendedChunk, RxMessage, encode_extended_chunk, parse_extended_chunk,
};

/// A USB PD link: a [`Usbpd`] transceiver plus the RX/TX DMA buffers.
///
/// This is the lowest blocking abstraction — it sends a built [`Message`] and
/// waits for its GoodCRC, and receives incoming packets (re-arming the
/// receive path after each one). The Sink/Source policy engines own one of
/// these.
pub struct Port {
    phy: Usbpd,
    rx: &'static mut [u8],
    tx: &'static mut [u8],
    /// Outgoing Message ID, incremented by 2 after each acknowledged send.
    msg_id: u8,
    /// Header template for automatic GoodCRC replies (our role + spec).
    own: Header,
}

impl Port {
    /// Create a link, consuming the [`Usbpd`] transceiver.
    ///
    /// `rx`/`tx` are the PD DMA buffers. They must be 4-byte aligned.
    ///
    /// # Safety
    ///
    /// Both buffers must stay alive and unaliased for the whole lifetime of
    /// the link, and must not be accessed while a receive is in progress.
    pub unsafe fn new(phy: Usbpd, rx: &'static mut [u8], tx: &'static mut [u8]) -> Self {
        debug_assert!(
            (rx.as_ptr() as usize).is_multiple_of(4),
            "PD RX buffer must be 4-byte aligned"
        );
        debug_assert!(
            (tx.as_ptr() as usize).is_multiple_of(4),
            "PD TX buffer must be 4-byte aligned"
        );
        Self {
            phy,
            rx,
            tx,
            msg_id: 0,
            own: Header::new(),
        }
    }

    /// Borrow the underlying transceiver (e.g. for CC configuration).
    pub fn phy(&mut self) -> &mut Usbpd {
        &mut self.phy
    }

    /// Consume the link and return the underlying transceiver.
    pub fn into_phy(self) -> Usbpd {
        self.phy
    }

    /// The outgoing Message ID (an even value `0, 2, ..., 14`).
    pub const fn msg_id(&self) -> u8 {
        self.msg_id
    }

    /// Reset the outgoing Message ID to 0 (e.g. after a fresh connection).
    pub fn reset_msg_id(&mut self) {
        self.msg_id = 0;
    }

    /// Set the header template used for automatic GoodCRC replies.
    ///
    /// `own` is your power/data role and spec revision (typically
    /// [`Header::new`] with the role and spec builders). Every message
    /// received by [`receive`](Self::receive) is acknowledged automatically,
    /// unless it is itself a GoodCRC or an extended-message chunk.
    pub fn set_ack_header(&mut self, own: Header) {
        self.own = own;
    }

    /// Arm the receive path so the next packet is captured into `rx`.
    ///
    /// Called automatically by [`receive`](Self::receive) and after every
    /// acknowledged send; call it explicitly before the first receive.
    pub fn start(&mut self) {
        // SAFETY: `rx` is a 'static, unaliased buffer. The DMA writes to it
        // only while receive mode is armed, and this module sequences every
        // re-arm so a receive never overlaps another read of the buffer.
        unsafe { self.phy.start_receive(&mut *self.rx) };
    }

    /// Block (up to `timeout_ms` milliseconds) for an incoming message.
    ///
    /// Returns the decoded message, or `None` on timeout. The receive path is
    /// left re-armed for the next packet either way. Non-GoodCRC messages are
    /// acknowledged automatically with the header set by
    /// [`set_ack_header`](Self::set_ack_header).
    pub fn receive(&mut self, timeout_ms: u32, delay: &impl DelayUs) -> Option<RxMessage> {
        for _ in 0..timeout_ms {
            if self.phy.is_rx_complete() {
                let len = self.phy.received_len() as usize;
                self.phy.clear_all_flags();
                let msg = RxMessage::decode(&self.rx[..len.min(self.rx.len())]);
                if let Some(m) = &msg {
                    self.ack(m.header().bits(), delay);
                }
                self.start();
                return msg;
            }
            delay.delay_ms(1);
        }
        self.start();
        None
    }

    /// Block (up to `timeout_ms` milliseconds) for an extended-message chunk.
    ///
    /// Returns the decoded chunk, or `None` on timeout. The receive path is
    /// left re-armed for the next packet either way. Unlike
    /// [`receive`](Self::receive), the chunk is **not** acknowledged
    /// automatically: acknowledge it yourself with
    /// [`send_goodcrc`](Self::send_goodcrc) on the matching SOP type (SOP').
    pub fn receive_chunk(
        &mut self,
        timeout_ms: u32,
        delay: &impl DelayUs,
    ) -> Option<RxExtendedChunk> {
        for _ in 0..timeout_ms {
            if self.phy.is_rx_complete() {
                let len = self.phy.received_len() as usize;
                self.phy.clear_all_flags();
                let chunk = parse_extended_chunk(&self.rx[..len.min(self.rx.len())]);
                self.start();
                return chunk;
            }
            delay.delay_ms(1);
        }
        self.start();
        None
    }

    /// Send a message and wait for its GoodCRC acknowledgement.
    ///
    /// The header's MsgID field is injected automatically and incremented by
    /// 2 after a successful send. The message is transmitted with up to 3
    /// attempts, each waiting up to 750 µs for GoodCRC; the receive path is
    /// re-armed afterwards so the reply can arrive.
    ///
    /// # Errors
    ///
    /// [`PdError::NoAck`] if no GoodCRC is received after the retries.
    pub fn send_message(
        &mut self,
        msg: &Message,
        sop: SopType,
        delay: &impl DelayUs,
    ) -> Result<(), PdError> {
        let len = msg.encode(self.tx)?;
        self.send_raw(len, sop, delay)
    }

    /// Send an extended message, splitting `data` into chunks.
    ///
    /// `header` carries the extended message type plus your power/data role
    /// and spec revision (the extended bit and NDO are set automatically).
    /// `data` is transmitted as [`MAX_CHUNK_DATA`]-byte chunks on `sop`
    /// (typically [`SopType::Sop1`]), each waiting for its own GoodCRC.
    ///
    /// # Errors
    ///
    /// [`PdError::TooLong`] if `data.len() > MAX_EXTENDED_DATA`; any
    /// [`PdError`] from [`send_message`](Self::send_message) on failure.
    pub fn send_extended(
        &mut self,
        header: Header,
        data: &[u8],
        sop: SopType,
        delay: &impl DelayUs,
    ) -> Result<(), PdError> {
        let total = data.len();
        if total > MAX_EXTENDED_DATA {
            return Err(PdError::TooLong);
        }
        let mut offset = 0usize;
        let mut chunk = 0u8;
        while offset < total {
            let n = core::cmp::min(MAX_CHUNK_DATA, total - offset);
            let more = offset + n < total;
            let ext_header = ExtendedHeader::new(total as u16)
                .with_chunk_number(chunk)
                .with_chunked(more);
            let len =
                encode_extended_chunk(self.tx, header, ext_header, &data[offset..offset + n])?;
            self.send_raw(len, sop, delay)?;
            offset += n;
            chunk += 1;
        }
        Ok(())
    }

    /// Answer a received message with a GoodCRC acknowledgement.
    ///
    /// `rx_header` is the raw 16-bit header of the received message. The reply
    /// echoes its Message ID and sets the message type to GoodCRC; the other
    /// fields come from `own` (typically your power/data role and spec
    /// revision). No GoodCRC is expected in return.
    pub fn send_goodcrc(
        &mut self,
        rx_header: u16,
        own: Header,
        sop: SopType,
    ) -> Result<(), PdError> {
        let header = own
            .with_message_type(ControlMessage::GoodCrc as u8)
            .with_message_id(((rx_header >> 9) & 0x7) as u8)
            .with_num_data_objects(0);
        self.tx[0] = header.bits() as u8;
        self.tx[1] = (header.bits() >> 8) as u8;
        self.phy.send(&self.tx[..2], sop);
        Ok(())
    }

    /// Acknowledge a received message with a GoodCRC.
    ///
    /// GoodCRC messages (which are themselves acknowledgements) and
    /// extended-message chunks (which must be acknowledged on SOP') are
    /// skipped; extended-message users should call
    /// [`send_goodcrc`](Self::send_goodcrc) themselves with the right SOP.
    fn ack(&mut self, rx_header: u16, delay: &impl DelayUs) {
        let header = Header::from_bits(rx_header);
        if header.message_type() == ControlMessage::GoodCrc as u8 || header.extended() {
            return;
        }
        // Let the bus settle before answering (WCH SDK `PD_Process.c`).
        delay.delay_us(30);
        let _ = self.send_goodcrc(rx_header, self.own, SopType::Sop0);
    }

    /// Send `len` already-encoded bytes with Message-ID injection and GoodCRC.
    fn send_raw(&mut self, len: usize, sop: SopType, delay: &impl DelayUs) -> Result<(), PdError> {
        // Inject the outgoing Message ID (header byte 1, bits [3:1]).
        self.tx[1] = (self.tx[1] & !0x0E) | (self.msg_id & 0x0E);

        for _ in 0..3 {
            // Transmit (blocks until TX_END).
            self.phy.send(&self.tx[..len], sop);

            // Wait for GoodCRC, up to ~750 µs.
            let mut acked = false;
            for _ in 0..250 {
                if self.phy.is_rx_complete() {
                    self.phy.clear_all_flags();
                    let good = self.phy.received_len() == 6
                        && self.rx.first().copied().unwrap_or(0) & 0x1F
                            == ControlMessage::GoodCrc as u8;
                    if good {
                        acked = true;
                        break;
                    }
                    // Spurious frame: re-arm and keep waiting for the GoodCRC.
                    self.start();
                }
                delay.delay_us(3);
            }
            if acked {
                self.msg_id = (self.msg_id + 2) & 0x0E;
                self.start();
                return Ok(());
            }
        }
        self.start();
        Err(PdError::NoAck)
    }
}
