//! USB PD passive sniffer example.
//!
//! Taps the CC line between a USB PD Source and Sink and decodes every BMC
//! frame on it **without participating** in the negotiation: the PHY is put
//! into receive-only mode (no Rp/Rd pull-ups, no comparator driving), and
//! each frame is timestamped with a free-running 1 MHz timer and printed
//! over defmt/RTT.
//!
//! What each log line shows:
//! - `t=...` — µs timestamp (TIM2 free-running counter);
//! - the SOP type (`SOP` / `SOP'` / `SOP''`, from `STATUS.BMC_AUX`);
//! - the sender, from the header's power-role bit: `SRC->` / `SNK->`;
//! - the message-type name, MsgID, NDO count and the raw 16-bit header;
//! - the raw frame bytes in hex (header + data objects + the 4-byte CRC the
//!   PHY captures and validates automatically) — handy for a Wireshark-style
//!   hex dump;
//! - chunked extended messages are reassembled with
//!   [`ChunkedAssembler`](ch32x035::usbpd::ChunkedAssembler) and only the
//!   completed payload is printed;
//! - `Hard_Reset` / `Cable_Reset` (from `IF_RX_RESET`) and `BUF_ERR` (a
//!   missed frame) are reported as well.
//!
//! GoodCRC frames (the most frequent) are logged at `debug` level so they
//! can be filtered out; everything else is `info`.
//!
//! Wiring:
//! - **PC14 = CC1**, **PC15 = CC2** — connect them to the CC line between
//!   the two devices under test. The active CC (the one carrying the link)
//!   is auto-selected once at startup via `check_cc`.
//! - No pull-ups/pull-downs are enabled, so the sniffer does not disturb the
//!   link. For production use add a high-impedance buffer before the tap.
//! - View the output with a debug probe + RTT (e.g. `probe-rs` / `cargo-embed`).
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin usbpd_sniffer --release
//! ```
//!
//! Limitations (by design, not bugs): frames are polled in the main loop, so
//! the timestamp is taken a few µs *after* the frame actually arrived; if
//! logging ever falls behind the bus the DMA buffer is overwritten and a
//! `BUF_ERR` is logged. For tighter timing latch a timer inside the
//! `IF_RX_ACT` interrupt; for burst-heavy traffic drain a ring buffer instead
//! of logging in-place.

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::timer::Timer2;
use ch32x035::usbpd::{
    CRC_LEN, CcPort, CcPull, ChunkedAssembler, Header, PowerRole, RxMessage, Usbpd,
    parse_extended_chunk,
};
use ch32x035::{CorePeripherals, Delay, main, pac};
use ch32x035_examples::board::Board;

/// PD DMA receive buffer, 4-byte aligned (the PHY DMA requires it).
/// Max frame = 2-byte header + 7×4-byte data objects + 4-byte CRC = 34 bytes;
/// a max-size extended-message chunk is 2 + 2 + 26 + 4 = 34 bytes.
const RX_CAPACITY: usize = 34;

#[repr(align(4))]
struct Align4([u8; RX_CAPACITY]);

static mut RX: Align4 = Align4([0; RX_CAPACITY]);

/// `STATUS` bits (the HAL exposes the raw value via [`Usbpd::status`]).
const ST_IF_RX_ACT: u32 = 1 << 5; // a frame was received (DMA done)
const ST_IF_RX_RESET: u32 = 1 << 6; // Hard_Reset / Cable_Reset seen
const ST_BUF_ERR: u32 = 1 << 2; // DMA/buffer overrun (a frame was lost)
const ST_BMC_AUX_MASK: u32 = 0b11; // STATUS[1:0] = received SOP type

/// `STATUS[1:0]` (`BMC_AUX`) encodings for a received frame.
const AUX_SOP0: u32 = 0b01; // SOP
const AUX_SOP1: u32 = 0b10; // SOP' (or Hard_Reset when IF_RX_RESET is set)
const AUX_SOP2: u32 = 0b11; // SOP'' (or Cable_Reset when IF_RX_RESET is set)

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        AFIO,
        USBPD,
        TIM2,
        ..
    } = dp;
    let cp = CorePeripherals::take().unwrap();

    let mut board = Board::init(RCC, GPIOA, GPIOB, GPIOC);

    // CC1 = PC14, CC2 = PC15 as floating inputs (no pull — the PHY handles it).
    let mut cc1 = board.pins.pc::<14>();
    let mut cc2 = board.pins.pc::<15>();
    cc1.into_input();
    cc2.into_input();

    let mut usbpd = Usbpd::init(USBPD, &AFIO, &mut board.rcc);

    // Free-running 1 MHz counter (PCLK1 = 48 MHz / (47 + 1)) for timestamps.
    let mut tim = Timer2::init(TIM2, &mut board.rcc, board.clocks);
    tim.set_prescaler(47);
    tim.set_period(0xFFFF);
    tim.start();

    let delay = Delay::init(cp.SYST, board.clocks.sysclk_hz());

    // Find which CC actually carries the link (the other is idle/floating),
    // keeping both pulls off so the sniffer is purely passive.
    usbpd.set_cc_pull(CcPort::Cc1, CcPull::NoPull);
    usbpd.set_cc_pull(CcPort::Cc2, CcPull::NoPull);
    let cc1_on = usbpd.check_cc(CcPort::Cc1, &delay);
    let cc2_on = usbpd.check_cc(CcPort::Cc2, &delay);
    let active = if cc1_on { CcPort::Cc1 } else { CcPort::Cc2 };
    usbpd.select_cc(active);
    defmt::info!(
        "USB PD sniffer on CC{} (CC1={} CC2={})",
        active as u8 + 1,
        cc1_on,
        cc2_on
    );

    // SAFETY: RX is a `static` that lives forever; the PHY is its only writer
    // and we re-borrow it each loop iteration to re-arm the DMA.
    let rx: &'static mut [u8] = unsafe { &mut (*core::ptr::addr_of_mut!(RX)).0 };
    // SAFETY: start a receive into the static buffer.
    unsafe { usbpd.start_receive(&mut *rx) };

    let mut overflows: u64 = 0;
    let mut asm = ChunkedAssembler::new();

    loop {
        let status = usbpd.status();
        let got_rx = status & ST_IF_RX_ACT != 0;
        let got_rst = status & ST_IF_RX_RESET != 0;
        let got_err = status & ST_BUF_ERR != 0;

        if got_rx || got_rst || got_err {
            let ts = stamp(&mut tim, &mut overflows);

            // Copy the frame out first, then re-arm so the DMA is free again
            // as quickly as possible (fewest lost frames).
            let mut frame = [0u8; RX_CAPACITY];
            let mut frame_len = 0usize;
            let sop = status & ST_BMC_AUX_MASK;
            if got_rx {
                frame_len = (usbpd.received_len() as usize).min(frame.len());
                frame[..frame_len].copy_from_slice(&rx[..frame_len]);
            }

            // Re-arm the receiver (also clears the RX_ACT / RX_RESET flags).
            // SAFETY: `rx` stays valid for the whole program.
            unsafe { usbpd.start_receive(&mut *rx) };

            if got_rx {
                process_frame(ts, sop, &frame[..frame_len], &mut asm);
            }
            if got_rst {
                match status & ST_BMC_AUX_MASK {
                    AUX_SOP1 => defmt::info!("[{}us] **** Hard_Reset ****", ts),
                    AUX_SOP2 => defmt::info!("[{}us] **** Cable_Reset ****", ts),
                    _ => defmt::info!("[{}us] **** Reset ****", ts),
                }
                asm.reset(); // a reset aborts any in-progress extended message
            }
            if got_err {
                defmt::warn!("[{}us] BUF_ERR: missed a frame (log too slow?)", ts);
            }
        }
    }
}

/// Read the free-running 1 MHz counter as a µs timestamp, catching any
/// missed 16-bit overflows first (the loop runs far faster than 65.5 ms).
fn stamp(tim: &mut Timer2, overflows: &mut u64) -> u64 {
    while tim.is_update_flag() {
        tim.clear_update_flag();
        *overflows = overflows.wrapping_add(1);
    }
    (*overflows << 16) | tim.count() as u64
}

/// Decode and log one captured frame.
fn process_frame(ts: u64, sop: u32, bytes: &[u8], asm: &mut ChunkedAssembler) {
    let sop_name = match sop {
        AUX_SOP0 => "SOP",
        AUX_SOP1 => "SOP'",
        AUX_SOP2 => "SOP''",
        _ => "SOP?",
    };

    // Extended message (chunked): the PHY captures header + ext header +
    // chunk data + CRC, so strip the CRC before parsing.
    if let Some(chunk) = parse_extended_chunk(&bytes[..bytes.len().saturating_sub(CRC_LEN)]) {
        let hdr = chunk.header();
        let ext = chunk.extended_header();
        match asm.push(hdr, ext, chunk.data()) {
            Ok(true) => {
                defmt::info!(
                    "[{}us] {} {} EXT {} size={} bytes={:?}",
                    ts,
                    sop_name,
                    msg_type_name(hdr),
                    ext_msg_type_name(asm.message_type().unwrap_or(0)),
                    asm.data().len(),
                    asm.data(),
                );
                asm.reset();
            }
            Ok(false) => {
                defmt::debug!(
                    "[{}us] {} {} ext chunk #{} ({}/{} bytes)",
                    ts,
                    sop_name,
                    msg_type_name(hdr),
                    ext.chunk_number(),
                    asm.data().len(),
                    ext.data_size(),
                );
            }
            Err(e) => {
                defmt::warn!(
                    "[{}us] {} {} ext reassembly error: {}",
                    ts,
                    sop_name,
                    msg_type_name(hdr),
                    defmt::Debug2Format(&e),
                );
            }
        }
        return;
    }

    // Ordinary (non-extended) frame.
    match RxMessage::decode(bytes) {
        Some(msg) => {
            let hdr = msg.header();
            let from = match hdr.port_power_role() {
                PowerRole::Source => "SRC->",
                PowerRole::Sink => "SNK->",
            };
            if msg.num_data_objects() == 0 && !hdr.extended() {
                // GoodCRC (and other control messages) are the most frequent
                // frames; keep them at debug level.
                defmt::debug!(
                    "[{}us] {} {} {} id={} bytes={:?}",
                    ts,
                    sop_name,
                    from,
                    msg_type_name(hdr),
                    hdr.message_id(),
                    bytes,
                );
            } else {
                defmt::info!(
                    "[{}us] {} {} {} id={} ndo={} hdr={=u16:04x} bytes={:?}",
                    ts,
                    sop_name,
                    from,
                    msg_type_name(hdr),
                    hdr.message_id(),
                    msg.num_data_objects(),
                    hdr.bits(),
                    bytes,
                );
            }
        }
        None => {
            defmt::warn!(
                "[{}us] {} undecodable frame ({} bytes)",
                ts,
                sop_name,
                bytes.len(),
            );
        }
    }
}

/// Human-readable name for a non-extended message type (control vs data is
/// distinguished by the NDO count in the header).
fn msg_type_name(hdr: Header) -> &'static str {
    if hdr.num_data_objects() == 0 {
        match hdr.message_type() {
            0x01 => "GoodCRC",
            0x02 => "GotoMin",
            0x03 => "Accept",
            0x04 => "Reject",
            0x05 => "Ping",
            0x06 => "PS_RDY",
            0x07 => "Get_Source_Cap",
            0x08 => "Get_Sink_Cap",
            0x09 => "DR_Swap",
            0x0A => "PR_Swap",
            0x0B => "VCONN_Swap",
            0x0C => "Wait",
            0x0D => "Soft_Reset",
            0x0E => "Data_Reset",
            0x0F => "Data_Reset_Complete",
            0x10 => "Not_Supported",
            0x11 => "Get_Source_Cap_Ext",
            0x12 => "Get_Status",
            0x13 => "FR_Swap",
            0x14 => "Get_PPS_Status",
            0x15 => "Get_Country_Codes",
            0x16 => "Get_Sink_Cap_Ext",
            0x17 => "Get_Source_Info",
            0x18 => "Get_Revision",
            _ => "Ctrl?",
        }
    } else {
        match hdr.message_type() {
            0x01 => "Source_Capabilities",
            0x02 => "Request",
            0x03 => "BIST",
            0x04 => "Sink_Capabilities",
            0x05 => "Battery_Status",
            0x06 => "Alert",
            0x07 => "Get_Country_Info",
            0x08 => "Enter_USB",
            0x09 => "EPR_Request",
            0x0A => "EPR_Mode",
            0x0B => "Source_Info",
            0x0C => "Revision",
            0x0F => "Vendor_Defined",
            _ => "Data?",
        }
    }
}

/// Human-readable name for an extended message type.
///
/// Values match [`ExtendedMsgType`](ch32x035::usbpd::ExtendedMsgType) (USB PD
/// 3.2, per the workspace `usbpd` crate; the EPR values are confirmed by real
/// captured EPR negotiation frames).
fn ext_msg_type_name(t: u8) -> &'static str {
    match t {
        0x01 => "Source_Cap_Ext",
        0x02 => "Status",
        0x03 => "Get_Battery_Cap",
        0x04 => "Get_Battery_Status",
        0x05 => "Battery_Cap",
        0x06 => "Get_Manufacturer_Info",
        0x07 => "Manufacturer_Info",
        0x08 => "Security_Request",
        0x09 => "Security_Response",
        0x0A => "Firmware_Update_Request",
        0x0B => "Firmware_Update_Response",
        0x0C => "PPS_Status",
        0x0D => "Country_Info",
        0x0E => "Country_Codes",
        0x0F => "Sink_Cap_Ext",
        0x10 => "Extended_Control",
        0x11 => "EPR_Source_Capabilities",
        0x12 => "EPR_Sink_Capabilities",
        0x1E => "Vendor_Defined_Ext",
        _ => "Ext?",
    }
}
