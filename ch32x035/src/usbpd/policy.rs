//! USB PD policy engines: blocking Source and Sink state machines.
//!
//! Built on top of the PHY ([`super::phy`]) and protocol ([`super::protocol`])
//! layers, this module drives a full PD power negotiation:
//!
//! - [`PdSink`] (receiving power): detect the CC connection, wait for
//!   `Source_Capabilities`, send a `Request` (fixed or PPS), then wait for
//!   `Accept` and `PS_RDY`.
//! - [`PdSource`] (supplying power): detect the CC connection, send
//!   `Source_Capabilities`, wait for a `Request`, then send `Accept` and
//!   `PS_RDY`. After the negotiation it keeps responding (re-request,
//!   `Get_Source_Cap`, `Soft_Reset`) so power delivery can be maintained.
//!
//! Both engines are **blocking**: [`poll`](PdSink::poll) advances the state
//! machine and busy-waits (with the supplied [`DelayUs`]) while waiting for
//! messages or timeouts. The CC detection is simplified (a single 0.22 V
//! comparator via [`Usbpd::check_cc`]); production code may want the
//! multi-threshold detection used by the WCH SDK.
//!
//! # Example (sink)
//!
//! ```ignore
//! use ch32x035::{Rcc, delay::DelayUs, usbpd::policy::{PdSink, SinkState}};
//!
//! static mut RX: [u8; 34] = [0; 34];
//! static mut TX: [u8; 34] = [0; 34];
//!
//! let mut sink = unsafe { PdSink::new(usbpd, &mut RX, &mut TX) };
//! let mut delay = /* a DelayUs implementor */;
//! loop {
//!     sink.poll(&mut delay);
//!     match sink.state() {
//!         // Source_Capabilities received: request 5 V at max current,
//!         // or a PPS output:
//!         SinkState::NeedRequest => {
//!             sink.request_voltage(5000, None, &mut delay);
//!             // sink.request_pps(9000, 1500, &mut delay); // 9 V PPS
//!         }
//!         SinkState::Ready => break, // power available
//!         SinkState::Error => panic!("PD negotiation failed"),
//!         _ => {}
//!     }
//! }
//! ```

use crate::delay::DelayUs;
use crate::usbpd::phy::{CcPort, CcPull, SopType, Usbpd};
use crate::usbpd::protocol::{
    ApdoType, ControlMessage, DataMessage, DataRole, Header, Pdo, PdoType, PowerRole, RxMessage,
    SpecRev, build_message, make_avs_rdo, make_fixed_pdo, make_pps_rdo, make_request_rdo,
    parse_message,
};

/// Borrow the RX DMA buffer as read-only.
///
/// Module-internal: the engine's buffers are `'static` by construction of
/// [`PdSink`]/[`PdSource`] and never aliased concurrently (single-threaded,
/// blocking policy).
fn rx_slice(ptr: *mut u8, len: usize) -> &'static [u8] {
    // SAFETY: see the function docs above.
    unsafe { core::slice::from_raw_parts(ptr as *const u8, len) }
}

/// Borrow the TX DMA buffer as mutable.
///
/// Module-internal: the caller (`start_receive`) hands the buffer to the DMA
/// and drops the reference immediately, so no aliasing occurs.
fn rx_slice_mut(ptr: *mut u8, len: usize) -> &'static mut [u8] {
    // SAFETY: see the function docs above.
    unsafe { core::slice::from_raw_parts_mut(ptr, len) }
}

/// Put the USBPD receive path back into receive mode (DMA target = `rx`).
fn start_receive(usbpd: &mut Usbpd, rx_ptr: *mut u8, rx_len: usize) {
    let buf = rx_slice_mut(rx_ptr, rx_len);
    // SAFETY: the buffer is 'static (see `rx_slice_mut`); the DMA only
    // writes it while receive mode is active, which the caller sequences.
    unsafe { usbpd.start_receive(buf) };
}

/// Blocking wait (up to `timeout_ms`) for a message, returning it parsed.
///
/// Re-enters receive mode before returning so the next frame can arrive.
fn wait_message(
    usbpd: &mut Usbpd,
    rx_ptr: *mut u8,
    rx_len: usize,
    delay: &mut impl DelayUs,
    timeout_ms: u32,
) -> Option<RxMessage<'static>> {
    for _ in 0..timeout_ms {
        if usbpd.is_rx_complete() {
            let len = usbpd.received_len() as usize;
            usbpd.clear_all_flags();
            let rx = rx_slice(rx_ptr, rx_len);
            let msg = parse_message(&rx[..len.min(rx_len)]);
            start_receive(usbpd, rx_ptr, rx_len);
            return msg;
        }
        delay.delay_ms(1);
    }
    start_receive(usbpd, rx_ptr, rx_len);
    None
}

/// Send a control message (no data objects), waiting for its GoodCRC.
#[allow(clippy::too_many_arguments)]
fn send_control(
    usbpd: &mut Usbpd,
    tx_ptr: *mut u8,
    tx_len: usize,
    rx_ptr: *mut u8,
    rx_len: usize,
    msg: ControlMessage,
    role: (PowerRole, DataRole),
    spec: SpecRev,
    delay: &mut impl DelayUs,
) -> bool {
    send_data(
        usbpd,
        tx_ptr,
        tx_len,
        rx_ptr,
        rx_len,
        msg as u8,
        &[],
        role,
        spec,
        delay,
    )
}

/// Send a data message, waiting for its GoodCRC.
#[allow(clippy::too_many_arguments)]
fn send_data(
    usbpd: &mut Usbpd,
    tx_ptr: *mut u8,
    tx_len: usize,
    rx_ptr: *mut u8,
    rx_len: usize,
    msg_type: u8,
    data: &[u32],
    role: (PowerRole, DataRole),
    spec: SpecRev,
    delay: &mut impl DelayUs,
) -> bool {
    let header = Header::new()
        .with_message_type(msg_type)
        .with_port_power_role(role.0)
        .with_port_data_role(role.1)
        .with_spec_revision(spec);
    let tx = rx_slice_mut(tx_ptr, tx_len);
    let len = match build_message(tx, header, data) {
        Ok(l) => l,
        Err(_) => return false,
    };
    let rx = rx_slice(rx_ptr, rx_len);
    usbpd
        .send_message(tx, len, SopType::Sop0, rx, delay)
        .is_ok()
}

// ============================================================================
// Sink
// ============================================================================

/// Sink (power consumer) policy state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SinkState {
    /// Not connected; polling the CC lines.
    Idle,
    /// Connected; waiting for `Source_Capabilities`.
    AwaitSrcCap,
    /// `Source_Capabilities` received; waiting for the application to
    /// request a PDO (via [`PdSink::request_voltage`] or [`PdSink::request`]).
    NeedRequest,
    /// `Request` sent; waiting for `Accept`.
    AwaitAccept,
    /// `Accept` received; waiting for `PS_RDY`.
    AwaitPsRdy,
    /// Negotiation complete; power is available.
    Ready,
    /// Negotiation failed (timeout, NACK or unsupported response).
    Error,
}

/// PD Sink policy engine.
///
/// Drives a blocking PD power negotiation. Create with [`new`](Self::new),
/// then call [`poll`](Self::poll) in a loop until [`state`](Self::state) is
/// [`Ready`](SinkState::Ready) or [`Error`](SinkState::Error).
pub struct PdSink {
    usbpd: Usbpd,
    state: SinkState,
    rx_ptr: *mut u8,
    rx_len: usize,
    tx_ptr: *mut u8,
    tx_len: usize,
    port: CcPort,
    pdo: [u32; 7],
    pdo_count: u8,
    requested_mv: u16,
    requested_ma: u16,
}

impl PdSink {
    /// Create a sink engine, consuming the [`Usbpd`] driver.
    ///
    /// `rx`/`tx` are the PD DMA buffers (must be 4-byte aligned).
    ///
    /// # Safety
    ///
    /// Both buffers must stay alive and unmoved for the whole negotiation,
    /// and must not be accessed concurrently with the engine.
    pub unsafe fn new(usbpd: Usbpd, rx: &'static mut [u8], tx: &'static mut [u8]) -> Self {
        debug_assert!(
            (rx.as_ptr() as usize).is_multiple_of(4),
            "PD RX buffer must be 4-byte aligned"
        );
        debug_assert!(
            (tx.as_ptr() as usize).is_multiple_of(4),
            "PD TX buffer must be 4-byte aligned"
        );
        Self {
            usbpd,
            state: SinkState::Idle,
            rx_ptr: rx.as_mut_ptr(),
            rx_len: rx.len(),
            tx_ptr: tx.as_mut_ptr(),
            tx_len: tx.len(),
            port: CcPort::Cc1,
            pdo: [0; 7],
            pdo_count: 0,
            requested_mv: 0,
            requested_ma: 0,
        }
    }

    /// The current state.
    pub fn state(&self) -> SinkState {
        self.state
    }

    /// The CC port that is connected.
    pub fn port(&self) -> CcPort {
        self.port
    }

    /// The voltage (mV) of the PDO that was requested.
    pub fn requested_mv(&self) -> u16 {
        self.requested_mv
    }

    /// The current (mA) that was requested.
    pub fn requested_ma(&self) -> u16 {
        self.requested_ma
    }

    /// The number of PDOs advertised by the source.
    ///
    /// Valid once the state is [`NeedRequest`](SinkState::NeedRequest) or
    /// later.
    pub fn pdo_count(&self) -> u8 {
        self.pdo_count
    }

    /// The `i`-th advertised PDO (0-based), if any.
    pub fn pdo(&self, i: usize) -> Option<Pdo> {
        if i < self.pdo_count as usize {
            Some(Pdo::from_bits(self.pdo[i]))
        } else {
            None
        }
    }

    /// Advance the negotiation (blocking).
    ///
    /// `delay` is used for all timeouts and debounce waits.
    pub fn poll(&mut self, delay: &mut impl DelayUs) {
        match self.state {
            SinkState::Idle => {
                for port in [CcPort::Cc1, CcPort::Cc2] {
                    if self.usbpd.check_cc(port) {
                        self.port = port;
                        self.usbpd.select_cc(port);
                        start_receive(&mut self.usbpd, self.rx_ptr, self.rx_len);
                        self.state = SinkState::AwaitSrcCap;
                        return;
                    }
                }
            }
            SinkState::AwaitSrcCap => {
                let mut elapsed = 0u32;
                loop {
                    if let Some(msg) =
                        wait_message(&mut self.usbpd, self.rx_ptr, self.rx_len, delay, 100)
                    {
                        match msg.header().message_type() {
                            t if t == DataMessage::SourceCapabilities as u8 => {
                                let n = msg.header().num_data_objects() as usize;
                                self.pdo_count = n.min(7) as u8;
                                for i in 0..n.min(7) {
                                    self.pdo[i] = msg.data_object(i);
                                }
                                self.state = SinkState::NeedRequest;
                                return;
                            }
                            t if t == ControlMessage::SoftReset as u8 => {
                                send_control(
                                    &mut self.usbpd,
                                    self.tx_ptr,
                                    self.tx_len,
                                    self.rx_ptr,
                                    self.rx_len,
                                    ControlMessage::Accept,
                                    (PowerRole::Sink, DataRole::Ufp),
                                    SpecRev::Rev3_0,
                                    delay,
                                );
                            }
                            t if t == ControlMessage::GetSinkCap as u8 => {
                                let cap = make_fixed_pdo(5000, 1000);
                                send_data(
                                    &mut self.usbpd,
                                    self.tx_ptr,
                                    self.tx_len,
                                    self.rx_ptr,
                                    self.rx_len,
                                    DataMessage::SinkCapabilities as u8,
                                    &[cap],
                                    (PowerRole::Sink, DataRole::Ufp),
                                    SpecRev::Rev3_0,
                                    delay,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        elapsed += 100;
                        if elapsed >= 5000 {
                            self.state = SinkState::Error;
                            return;
                        }
                    }
                }
            }
            SinkState::AwaitAccept => {
                let mut elapsed = 0u32;
                loop {
                    if let Some(msg) =
                        wait_message(&mut self.usbpd, self.rx_ptr, self.rx_len, delay, 100)
                    {
                        match msg.header().message_type() {
                            t if t == ControlMessage::Accept as u8 => {
                                self.state = SinkState::AwaitPsRdy;
                                return;
                            }
                            t if t == ControlMessage::Reject as u8
                                || t == ControlMessage::Wait as u8 =>
                            {
                                self.state = SinkState::Error;
                                return;
                            }
                            _ => {}
                        }
                    } else {
                        elapsed += 100;
                        if elapsed >= 500 {
                            self.state = SinkState::Error;
                            return;
                        }
                    }
                }
            }
            SinkState::AwaitPsRdy => {
                let mut elapsed = 0u32;
                loop {
                    if let Some(msg) =
                        wait_message(&mut self.usbpd, self.rx_ptr, self.rx_len, delay, 100)
                    {
                        match msg.header().message_type() {
                            t if t == ControlMessage::PsRdy as u8 => {
                                self.state = SinkState::Ready;
                                return;
                            }
                            _ => {}
                        }
                    } else {
                        elapsed += 100;
                        if elapsed >= 500 {
                            self.state = SinkState::Error;
                            return;
                        }
                    }
                }
            }
            SinkState::NeedRequest => {
                // Wait for the application to call `request_voltage`/`request`.
            }
            SinkState::Ready | SinkState::Error => {}
        }
    }

    /// Request the advertised fixed-supply PDO whose voltage is closest to
    /// `voltage_mv`.
    ///
    /// `current_ma` overrides the requested current; `None` requests the
    /// PDO's maximum. Works from
    /// [`NeedRequest`](SinkState::NeedRequest) (initial request) or
    /// [`Ready`](SinkState::Ready) (renegotiation). Returns `true` if a
    /// `Request` was sent.
    pub fn request_voltage(
        &mut self,
        voltage_mv: u16,
        current_ma: Option<u16>,
        delay: &mut impl DelayUs,
    ) -> bool {
        let mut best: Option<(u8, Pdo)> = None;
        let mut best_delta = u16::MAX;
        for i in 0..self.pdo_count as usize {
            let p = Pdo::from_bits(self.pdo[i]);
            if p.pdo_type() != PdoType::FixedSupply {
                continue;
            }
            let delta = p.voltage_mv().abs_diff(voltage_mv);
            if delta < best_delta {
                best_delta = delta;
                best = Some((i as u8 + 1, p));
            }
        }
        match best {
            Some((position, pdo)) => {
                let ma = current_ma
                    .unwrap_or(pdo.max_current_ma())
                    .min(pdo.max_current_ma());
                self.send_request(position, ma, delay)
            }
            None => {
                self.state = SinkState::Error;
                false
            }
        }
    }

    /// Request the 1-based PDO `position` with the given current.
    ///
    /// The current is clamped to the PDO's maximum. Works from
    /// [`NeedRequest`](SinkState::NeedRequest) or
    /// [`Ready`](SinkState::Ready). Returns `true` if a `Request` was sent.
    pub fn request(&mut self, position: u8, current_ma: u16, delay: &mut impl DelayUs) -> bool {
        if position == 0 || position > self.pdo_count {
            self.state = SinkState::Error;
            return false;
        }
        let pdo = Pdo::from_bits(self.pdo[(position - 1) as usize]);
        let ma = current_ma.min(pdo.max_current_ma());
        self.send_request(position, ma, delay)
    }

    /// Request a PPS (PD 3.0 programmable power) output voltage from the
    /// APDO whose voltage range contains `voltage_mv`.
    ///
    /// The current is clamped to the APDO's maximum. Works from
    /// [`NeedRequest`](SinkState::NeedRequest) or
    /// [`Ready`](SinkState::Ready). Returns `true` if a `Request` was sent.
    pub fn request_pps(
        &mut self,
        voltage_mv: u16,
        current_ma: u16,
        delay: &mut impl DelayUs,
    ) -> bool {
        for i in 0..self.pdo_count as usize {
            let p = Pdo::from_bits(self.pdo[i]);
            if p.pdo_type() != PdoType::Apdo {
                continue;
            }
            if voltage_mv >= p.min_voltage_mv() && voltage_mv <= p.max_voltage_mv() {
                let ma = current_ma.min(p.max_current_ma());
                let rdo = make_pps_rdo(i as u8 + 1, voltage_mv, ma);
                return self.send_rdo(voltage_mv, ma, rdo, delay);
            }
        }
        self.state = SinkState::Error;
        false
    }

    /// Request an AVS (adjustable voltage supply) output from the matching
    /// SPR/EPR AVS APDO.
    ///
    /// For an SPR AVS APDO the current is clamped to the relevant band's
    /// maximum (15–20 V band if `voltage_mv >= 15 V`, else 9–15 V band); for
    /// an EPR AVS APDO the requested current is used directly (the APDO only
    /// caps power). Works from [`NeedRequest`](SinkState::NeedRequest) or
    /// [`Ready`](SinkState::Ready). Returns `true` if a `Request` was sent.
    pub fn request_avs(
        &mut self,
        voltage_mv: u16,
        current_ma: u16,
        delay: &mut impl DelayUs,
    ) -> bool {
        for i in 0..self.pdo_count as usize {
            let p = Pdo::from_bits(self.pdo[i]);
            if p.pdo_type() != PdoType::Apdo {
                continue;
            }
            let (min_v, max_v, max_ma) = match p.apdo_type() {
                ApdoType::SprAvs => {
                    let band = if voltage_mv >= 15000 {
                        p.spr_avs_current_15_20v_ma()
                    } else {
                        p.spr_avs_current_9_15v_ma()
                    };
                    (9000, 20000, band)
                }
                ApdoType::EprAvs => (p.epr_min_voltage_mv(), p.epr_max_voltage_mv(), 0),
                _ => continue,
            };
            if voltage_mv >= min_v && voltage_mv <= max_v {
                let ma = if max_ma == 0 {
                    current_ma
                } else {
                    current_ma.min(max_ma)
                };
                let rdo = make_avs_rdo(i as u8 + 1, voltage_mv, ma);
                return self.send_rdo(voltage_mv, ma, rdo, delay);
            }
        }
        self.state = SinkState::Error;
        false
    }

    /// Reset the engine to [`Idle`](SinkState::Idle) so the negotiation can
    /// be retried (e.g. after [`Error`](SinkState::Error)).
    pub fn reset(&mut self) {
        self.state = SinkState::Idle;
        self.pdo_count = 0;
        self.requested_mv = 0;
        self.requested_ma = 0;
    }

    /// Build and send a Request RDO, transitioning to
    /// [`AwaitAccept`](SinkState::AwaitAccept).
    fn send_request(&mut self, position: u8, current_ma: u16, delay: &mut impl DelayUs) -> bool {
        let pdo = Pdo::from_bits(self.pdo[(position - 1) as usize]);
        let rdo = make_request_rdo(position, current_ma, current_ma);
        self.send_rdo(pdo.voltage_mv(), current_ma, rdo, delay)
    }

    /// Send a pre-built RDO as a Request, recording the requested voltage
    /// and current and transitioning to
    /// [`AwaitAccept`](SinkState::AwaitAccept).
    fn send_rdo(
        &mut self,
        requested_mv: u16,
        requested_ma: u16,
        rdo: u32,
        delay: &mut impl DelayUs,
    ) -> bool {
        self.requested_mv = requested_mv;
        self.requested_ma = requested_ma;
        let ok = send_data(
            &mut self.usbpd,
            self.tx_ptr,
            self.tx_len,
            self.rx_ptr,
            self.rx_len,
            DataMessage::Request as u8,
            &[rdo],
            (PowerRole::Sink, DataRole::Ufp),
            SpecRev::Rev3_0,
            delay,
        );
        self.state = if ok {
            SinkState::AwaitAccept
        } else {
            SinkState::Error
        };
        ok
    }

    /// Consume the engine and return the underlying [`Usbpd`] driver, so the
    /// transceiver can be reconfigured (e.g. for a different role).
    pub fn release(self) -> Usbpd {
        self.usbpd
    }
}

// ============================================================================
// Source
// ============================================================================

/// Source (power supplier) policy state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceState {
    /// Not connected; polling the CC lines.
    Idle,
    /// Connected to a sink; debouncing.
    AwaitSink,
    /// Sending `Source_Capabilities`.
    SendSrcCap,
    /// Waiting for a `Request`.
    AwaitRequest,
    /// Sending `Accept` then `PS_RDY`.
    SendAccept,
    /// Negotiation complete; power delivered. Keeps responding to the sink.
    Ready,
    /// Negotiation failed.
    Error,
}

/// PD Source policy engine.
///
/// Drives a blocking PD power-supply negotiation. [`poll`](Self::poll)
/// advances the state machine until [`Ready`](SourceState::Ready) or
/// [`Error`](SourceState::Error); while [`Ready`](SourceState::Ready) it keeps
/// listening and answers re-requests, `Get_Source_Cap` and `Soft_Reset`.
pub struct PdSource {
    usbpd: Usbpd,
    state: SourceState,
    rx_ptr: *mut u8,
    rx_len: usize,
    tx_ptr: *mut u8,
    tx_len: usize,
    port: CcPort,
    src_caps: &'static [u32],
    requested_position: u8,
    requested_mv: u16,
    requested_ma: u16,
}

impl PdSource {
    /// Create a source engine, consuming the [`Usbpd`] driver.
    ///
    /// `rx`/`tx` are the PD DMA buffers (4-byte aligned); `src_caps` is the
    /// Source_Capabilities PDO table (up to 7 entries, 1-7 PDOs).
    ///
    /// # Safety
    ///
    /// `rx`/`tx` must stay alive and unmoved for the whole negotiation and
    /// not be accessed concurrently.
    pub unsafe fn new(
        usbpd: Usbpd,
        rx: &'static mut [u8],
        tx: &'static mut [u8],
        src_caps: &'static [u32],
    ) -> Self {
        debug_assert!(
            (rx.as_ptr() as usize).is_multiple_of(4),
            "PD RX buffer must be 4-byte aligned"
        );
        debug_assert!(
            (tx.as_ptr() as usize).is_multiple_of(4),
            "PD TX buffer must be 4-byte aligned"
        );
        Self {
            usbpd,
            state: SourceState::Idle,
            rx_ptr: rx.as_mut_ptr(),
            rx_len: rx.len(),
            tx_ptr: tx.as_mut_ptr(),
            tx_len: tx.len(),
            port: CcPort::Cc1,
            src_caps,
            requested_position: 0,
            requested_mv: 0,
            requested_ma: 0,
        }
    }

    /// The current state.
    pub fn state(&self) -> SourceState {
        self.state
    }

    /// The CC port that is connected.
    pub fn port(&self) -> CcPort {
        self.port
    }

    /// The PDO position requested by the sink (0 if none yet).
    pub fn requested_position(&self) -> u8 {
        self.requested_position
    }

    /// The advertised PDO requested by the sink, if any.
    pub fn requested_pdo(&self) -> Option<Pdo> {
        if self.requested_position == 0 || (self.requested_position as usize) > self.src_caps.len()
        {
            None
        } else {
            Some(Pdo::from_bits(
                self.src_caps[(self.requested_position - 1) as usize],
            ))
        }
    }

    /// The output voltage (mV) requested by the sink (0 if none yet).
    pub fn requested_mv(&self) -> u16 {
        self.requested_mv
    }

    /// The output current (mA) requested by the sink (0 if none yet).
    pub fn requested_ma(&self) -> u16 {
        self.requested_ma
    }

    /// Reset the engine to [`Idle`](SourceState::Idle) so the negotiation can
    /// be retried (e.g. after [`Error`](SourceState::Error)).
    pub fn reset(&mut self) {
        self.state = SourceState::Idle;
        self.requested_position = 0;
        self.requested_mv = 0;
        self.requested_ma = 0;
    }

    /// Record the sink's request from an RDO and its target PDO.
    fn record_request(&mut self, position: u8, rdo: u32) {
        self.requested_position = position;
        let pdo = Pdo::from_bits(self.src_caps[(position - 1) as usize]);
        self.requested_mv = match pdo.pdo_type() {
            PdoType::FixedSupply => pdo.voltage_mv(),
            PdoType::Apdo => pdo.max_voltage_mv(),
            _ => 0,
        };
        self.requested_ma = match pdo.pdo_type() {
            PdoType::FixedSupply => ((rdo & 0x3FF) * 10) as u16,
            PdoType::Apdo => ((rdo & 0x7F) * 50) as u16,
            _ => 0,
        };
    }

    /// Advance the negotiation (blocking).
    ///
    /// While [`Ready`](SourceState::Ready), each call waits up to 100 ms for
    /// a message so the source can answer re-requests, `Get_Source_Cap` and
    /// `Soft_Reset` while keeping power applied.
    pub fn poll(&mut self, delay: &mut impl DelayUs) {
        match self.state {
            SourceState::Idle => {
                // Put a 330 µA pull-up on both CC lines and detect the sink.
                for port in [CcPort::Cc1, CcPort::Cc2] {
                    self.usbpd.set_cc_pull(port, CcPull::Pull330uA);
                    if self.usbpd.check_cc(port) {
                        self.port = port;
                        self.usbpd.select_cc(port);
                        start_receive(&mut self.usbpd, self.rx_ptr, self.rx_len);
                        self.state = SourceState::AwaitSink;
                        return;
                    }
                }
            }
            SourceState::AwaitSink => {
                delay.delay_ms(160);
                self.state = SourceState::SendSrcCap;
            }
            SourceState::SendSrcCap => {
                let ok = send_data(
                    &mut self.usbpd,
                    self.tx_ptr,
                    self.tx_len,
                    self.rx_ptr,
                    self.rx_len,
                    DataMessage::SourceCapabilities as u8,
                    self.src_caps,
                    (PowerRole::Source, DataRole::Dfp),
                    SpecRev::Rev3_0,
                    delay,
                );
                self.state = if ok {
                    SourceState::AwaitRequest
                } else {
                    SourceState::Error
                };
            }
            SourceState::AwaitRequest => {
                let mut elapsed = 0u32;
                loop {
                    if let Some(msg) =
                        wait_message(&mut self.usbpd, self.rx_ptr, self.rx_len, delay, 100)
                    {
                        match msg.header().message_type() {
                            t if t == DataMessage::Request as u8 => {
                                let rdo = msg.data_object(0);
                                let pos = ((rdo >> 28) & 0xF) as u8;
                                if pos == 0 || (pos as usize) > self.src_caps.len() {
                                    // Not one of our PDOs: reject it.
                                    send_control(
                                        &mut self.usbpd,
                                        self.tx_ptr,
                                        self.tx_len,
                                        self.rx_ptr,
                                        self.rx_len,
                                        ControlMessage::Reject,
                                        (PowerRole::Source, DataRole::Dfp),
                                        SpecRev::Rev3_0,
                                        delay,
                                    );
                                } else {
                                    self.record_request(pos, rdo);
                                    self.state = SourceState::SendAccept;
                                    return;
                                }
                            }
                            t if t == ControlMessage::GetSourceCap as u8 => {
                                // Re-advertise our capabilities.
                                self.state = SourceState::SendSrcCap;
                                return;
                            }
                            t if t == ControlMessage::SoftReset as u8 => {
                                send_control(
                                    &mut self.usbpd,
                                    self.tx_ptr,
                                    self.tx_len,
                                    self.rx_ptr,
                                    self.rx_len,
                                    ControlMessage::Accept,
                                    (PowerRole::Source, DataRole::Dfp),
                                    SpecRev::Rev3_0,
                                    delay,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        elapsed += 100;
                        if elapsed >= 5000 {
                            self.state = SourceState::Error;
                            return;
                        }
                    }
                }
            }
            SourceState::SendAccept => {
                let ok = send_control(
                    &mut self.usbpd,
                    self.tx_ptr,
                    self.tx_len,
                    self.rx_ptr,
                    self.rx_len,
                    ControlMessage::Accept,
                    (PowerRole::Source, DataRole::Dfp),
                    SpecRev::Rev3_0,
                    delay,
                );
                if !ok {
                    self.state = SourceState::Error;
                    return;
                }
                delay.delay_ms(2);
                let ok = send_control(
                    &mut self.usbpd,
                    self.tx_ptr,
                    self.tx_len,
                    self.rx_ptr,
                    self.rx_len,
                    ControlMessage::PsRdy,
                    (PowerRole::Source, DataRole::Dfp),
                    SpecRev::Rev3_0,
                    delay,
                );
                self.state = if ok {
                    SourceState::Ready
                } else {
                    SourceState::Error
                };
            }
            SourceState::Ready => {
                if let Some(msg) =
                    wait_message(&mut self.usbpd, self.rx_ptr, self.rx_len, delay, 100)
                {
                    match msg.header().message_type() {
                        t if t == DataMessage::Request as u8 => {
                            let rdo = msg.data_object(0);
                            let pos = ((rdo >> 28) & 0xF) as u8;
                            if pos != 0 && (pos as usize) <= self.src_caps.len() {
                                self.record_request(pos, rdo);
                                self.state = SourceState::SendAccept;
                            }
                        }
                        t if t == ControlMessage::GetSourceCap as u8 => {
                            self.state = SourceState::SendSrcCap;
                        }
                        t if t == ControlMessage::SoftReset as u8 => {
                            send_control(
                                &mut self.usbpd,
                                self.tx_ptr,
                                self.tx_len,
                                self.rx_ptr,
                                self.rx_len,
                                ControlMessage::Accept,
                                (PowerRole::Source, DataRole::Dfp),
                                SpecRev::Rev3_0,
                                delay,
                            );
                        }
                        _ => {}
                    }
                }
            }
            SourceState::Error => {}
        }
    }

    /// Consume the engine and return the underlying [`Usbpd`] driver, so the
    /// transceiver can be reconfigured (e.g. for a different role).
    pub fn release(self) -> Usbpd {
        self.usbpd
    }
}
