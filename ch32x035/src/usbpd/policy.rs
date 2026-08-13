//! USB PD policy engines: blocking Sink and Source state machines.
//!
//! Built on the [`super::port`] link layer and the [`super::protocol`] types,
//! this module drives a complete PD power negotiation:
//!
//! - [`PdSink`] (receiving power): detects the CC connection, waits for
//!   `Source_Capabilities`, sends a `Request` (fixed, PPS or AVS), then waits
//!   for `Accept` and `PS_RDY`.
//! - [`PdSource`] (supplying power): detects the CC connection, advertises
//!   `Source_Capabilities`, waits for a `Request`, then sends `Accept` and
//!   `PS_RDY`. Once [`Ready`](SourceState::Ready) it keeps answering
//!   re-requests, `Get_Source_Cap` and `Soft_Reset` so power can be
//!   maintained.
//!
//! Both engines are **blocking**: [`poll`](PdSink::poll) advances the state
//! machine and busy-waits (with the supplied [`DelayUs`]) while waiting for
//! messages or timeouts. CC detection is simplified (a single 0.22 V
//! comparator via [`Usbpd::check_cc`]); production code may want the
//! multi-threshold detection used by the WCH SDK.
//!
//! # Example (sink)
//!
//! ```ignore
//! use ch32x035::usbpd::{policy::{PdSink, SinkState}, Request};
//!
//! let mut sink = PdSink::new(port);
//! loop {
//!     sink.poll(&delay);
//!     match sink.state() {
//!         SinkState::NeedRequest => {
//!             // Request the fixed PDO closest to 5 V at its maximum current.
//!             sink.request(Request::FixedVoltage { voltage_mv: 5000, current_ma: None }, &delay);
//!         }
//!         SinkState::Ready => break, // power available
//!         SinkState::Error => break,
//!         _ => {}
//!     }
//! }
//! ```

use crate::delay::DelayUs;
use crate::usbpd::phy::{CcPort, CcPull, SopType};
use crate::usbpd::port::Port;
use crate::usbpd::protocol::{
    ApdoType, ControlMessage, DataMessage, DataRole, Message, Pdo, PdoType, PowerRole, Rdo,
    SpecRev,
};

/// A sink-side power request, applied with [`PdSink::request`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Request {
    /// Request a fixed PDO by 1-based position; the current is clamped to the
    /// PDO's maximum.
    Fixed {
        /// 1-based PDO position.
        position: u8,
        /// Requested current (mA), clamped to the PDO maximum.
        current_ma: u16,
    },
    /// Request the fixed PDO whose voltage is closest to `voltage_mv`.
    FixedVoltage {
        /// Target voltage (mV).
        voltage_mv: u16,
        /// Requested current (mA); `None` requests the PDO maximum.
        current_ma: Option<u16>,
    },
    /// Request a PPS (programmable power supply) output.
    Pps {
        /// Output voltage (mV).
        voltage_mv: u16,
        /// Operating current (mA), clamped to the APDO maximum.
        current_ma: u16,
    },
    /// Request an AVS (adjustable voltage supply) output.
    Avs {
        /// Output voltage (mV).
        voltage_mv: u16,
        /// Operating current (mA).
        current_ma: u16,
    },
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
    /// `Source_Capabilities` received; waiting for the application to send a
    /// [`Request`] via [`PdSink::request`].
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
    link: Port,
    state: SinkState,
    cc_port: CcPort,
    pdo: [Pdo; 7],
    pdo_count: u8,
    requested_mv: u16,
    requested_ma: u16,
}

impl PdSink {
    /// Create a sink engine over the given [`Port`].
    pub fn new(link: Port) -> Self {
        Self {
            link,
            state: SinkState::Idle,
            cc_port: CcPort::Cc1,
            pdo: [Pdo::from_bits(0); 7],
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
    pub fn cc_port(&self) -> CcPort {
        self.cc_port
    }

    /// The voltage (mV) that was requested.
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
            Some(self.pdo[i])
        } else {
            None
        }
    }

    /// Advance the negotiation (blocking).
    ///
    /// `delay` is used for all timeouts and debounce waits.
    pub fn poll(&mut self, delay: &impl DelayUs) {
        match self.state {
            SinkState::Idle => self.detect(),
            SinkState::AwaitSrcCap => self.await_src_cap(delay),
            SinkState::AwaitAccept => self.await_accept(delay),
            SinkState::AwaitPsRdy => self.await_ps_rdy(delay),
            SinkState::NeedRequest => {} // application sends a `Request`
            SinkState::Ready | SinkState::Error => {}
        }
    }

    /// Send a power request, transitioning to
    /// [`AwaitAccept`](SinkState::AwaitAccept).
    ///
    /// Works from [`NeedRequest`](SinkState::NeedRequest) (initial request) or
    /// [`Ready`](SinkState::Ready) (renegotiation). Returns `true` if the
    /// `Request` was sent.
    pub fn request(&mut self, req: Request, delay: &impl DelayUs) -> bool {
        let Some((_, rdo, mv, ma)) = self.build_request(req) else {
            self.state = SinkState::Error;
            return false;
        };
        self.requested_mv = mv;
        self.requested_ma = ma;
        let ok = self.send_data(DataMessage::Request, &[rdo.bits()], delay);
        self.state = if ok {
            SinkState::AwaitAccept
        } else {
            SinkState::Error
        };
        ok
    }

    /// Reset the engine to [`Idle`](SinkState::Idle) so the negotiation can
    /// be retried (e.g. after [`Error`](SinkState::Error)).
    pub fn reset(&mut self) {
        self.state = SinkState::Idle;
        self.pdo_count = 0;
        self.requested_mv = 0;
        self.requested_ma = 0;
        self.link.reset_msg_id();
    }

    /// Consume the engine and return the underlying [`Port`], so the link can
    /// be reused for another role.
    pub fn into_port(self) -> Port {
        self.link
    }

    /// Detect the CC connection and select the active port.
    fn detect(&mut self) {
        for p in [CcPort::Cc1, CcPort::Cc2] {
            if self.link.phy().check_cc(p) {
                self.cc_port = p;
                self.link.phy().select_cc(p);
                self.link.start();
                self.state = SinkState::AwaitSrcCap;
                return;
            }
        }
    }

    /// Wait for `Source_Capabilities`, answering `Soft_Reset` / `Get_Sink_Cap`.
    fn await_src_cap(&mut self, delay: &impl DelayUs) {
        let mut elapsed = 0u32;
        while elapsed < 5000 {
            match self.link.receive(100, delay) {
                Some(msg) => match msg.header().message_type() {
                    t if t == DataMessage::SourceCapabilities as u8 => {
                        let n = (msg.num_data_objects() as usize).min(7);
                        self.pdo_count = n as u8;
                        for (i, slot) in self.pdo.iter_mut().take(n).enumerate() {
                            *slot = Pdo::from_bits(msg.object(i));
                        }
                        self.state = SinkState::NeedRequest;
                        return;
                    }
                    t if t == ControlMessage::SoftReset as u8 => {
                        self.send_control(ControlMessage::Accept, delay);
                    }
                    t if t == ControlMessage::GetSinkCap as u8 => {
                        self.send_sink_caps(delay);
                    }
                    _ => {}
                },
                None => elapsed += 100,
            }
        }
        self.state = SinkState::Error;
    }

    /// Wait for `Accept`, failing on `Reject` / `Wait`.
    fn await_accept(&mut self, delay: &impl DelayUs) {
        self.await_control(delay, 500, |t| match t {
            t if t == ControlMessage::Accept as u8 => Some(SinkState::AwaitPsRdy),
            t if t == ControlMessage::Reject as u8 || t == ControlMessage::Wait as u8 => {
                Some(SinkState::Error)
            }
            _ => None,
        });
    }

    /// Wait for `PS_RDY`.
    fn await_ps_rdy(&mut self, delay: &impl DelayUs) {
        self.await_control(delay, 500, |t| match t {
            t if t == ControlMessage::PsRdy as u8 => Some(SinkState::Ready),
            _ => None,
        });
    }

    /// Shared wait loop for control-message-driven state transitions.
    fn await_control(
        &mut self,
        delay: &impl DelayUs,
        timeout_ms: u32,
        f: impl Fn(u8) -> Option<SinkState>,
    ) {
        let mut elapsed = 0u32;
        while elapsed < timeout_ms {
            match self.link.receive(100, delay) {
                Some(msg) => {
                    if let Some(next) = f(msg.header().message_type()) {
                        self.state = next;
                        return;
                    }
                }
                None => elapsed += 100,
            }
        }
        self.state = SinkState::Error;
    }

    /// Turn a [`Request`] into a (position, RDO, voltage, current) tuple.
    fn build_request(&self, req: Request) -> Option<(u8, Rdo, u16, u16)> {
        match req {
            Request::Fixed {
                position,
                current_ma,
            } => {
                if position == 0 || position > self.pdo_count {
                    return None;
                }
                let p = self.pdo[(position - 1) as usize];
                let ma = current_ma.min(p.max_current_ma());
                Some((position, Rdo::fixed(position, ma, ma), p.voltage_mv(), ma))
            }
            Request::FixedVoltage {
                voltage_mv,
                current_ma,
            } => {
                let mut best: Option<(usize, Pdo)> = None;
                let mut best_delta = u16::MAX;
                for (i, p) in self.pdo[..self.pdo_count as usize].iter().copied().enumerate() {
                    if p.pdo_type() != PdoType::FixedSupply {
                        continue;
                    }
                    let delta = p.voltage_mv().abs_diff(voltage_mv);
                    if delta < best_delta {
                        best_delta = delta;
                        best = Some((i, p));
                    }
                }
                let (i, p) = best?;
                let position = i as u8 + 1;
                let ma = current_ma.unwrap_or(p.max_current_ma()).min(p.max_current_ma());
                Some((position, Rdo::fixed(position, ma, ma), p.voltage_mv(), ma))
            }
            Request::Pps {
                voltage_mv,
                current_ma,
            } => {
                for (i, p) in self.pdo[..self.pdo_count as usize].iter().copied().enumerate() {
                    if p.pdo_type() != PdoType::Apdo || p.apdo_type() != ApdoType::Pps {
                        continue;
                    }
                    if (p.min_voltage_mv()..=p.max_voltage_mv()).contains(&voltage_mv) {
                        let ma = current_ma.min(p.max_current_ma());
                        let position = i as u8 + 1;
                        return Some((position, Rdo::pps(position, voltage_mv, ma), voltage_mv, ma));
                    }
                }
                None
            }
            Request::Avs {
                voltage_mv,
                current_ma,
            } => {
                for (i, p) in self.pdo[..self.pdo_count as usize].iter().copied().enumerate() {
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
                        let position = i as u8 + 1;
                        return Some((position, Rdo::avs(position, voltage_mv, ma), voltage_mv, ma));
                    }
                }
                None
            }
        }
    }

    /// Send a control message as a sink.
    fn send_control(&mut self, msg: ControlMessage, delay: &impl DelayUs) -> bool {
        let m = Message::control(msg, PowerRole::Sink, DataRole::Ufp, SpecRev::Rev3_0);
        self.link.send_message(&m, SopType::Sop0, delay).is_ok()
    }

    /// Send a data message as a sink.
    fn send_data(&mut self, msg: DataMessage, objects: &[u32], delay: &impl DelayUs) -> bool {
        let Ok(m) = Message::data(
            msg,
            PowerRole::Sink,
            DataRole::Ufp,
            SpecRev::Rev3_0,
            objects,
        ) else {
            return false;
        };
        self.link.send_message(&m, SopType::Sop0, delay).is_ok()
    }

    /// Advertise a fixed 5 V / 1 A sink capability.
    fn send_sink_caps(&mut self, delay: &impl DelayUs) {
        let cap = Pdo::fixed(5000, 1000);
        self.send_data(DataMessage::SinkCapabilities, &[cap.bits()], delay);
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
    link: Port,
    state: SourceState,
    cc_port: CcPort,
    src_caps: &'static [Pdo],
    requested_position: u8,
    requested_mv: u16,
    requested_ma: u16,
}

impl PdSource {
    /// Create a source engine over the given [`Port`].
    ///
    /// `src_caps` is the Source_Capabilities PDO table (up to 7 entries).
    pub fn new(link: Port, src_caps: &'static [Pdo]) -> Self {
        debug_assert!(
            src_caps.len() <= 7,
            "Source_Capabilities holds at most 7 PDOs"
        );
        Self {
            link,
            state: SourceState::Idle,
            cc_port: CcPort::Cc1,
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
    pub fn cc_port(&self) -> CcPort {
        self.cc_port
    }

    /// The PDO position requested by the sink (0 if none yet).
    pub fn requested_position(&self) -> u8 {
        self.requested_position
    }

    /// The advertised PDO requested by the sink, if any.
    pub fn requested_pdo(&self) -> Option<Pdo> {
        if self.requested_position == 0 {
            return None;
        }
        self.src_caps
            .get((self.requested_position - 1) as usize)
            .copied()
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
        self.link.reset_msg_id();
    }

    /// Consume the engine and return the underlying [`Port`], so the link can
    /// be reused for another role.
    pub fn into_port(self) -> Port {
        self.link
    }

    /// Advance the negotiation (blocking).
    ///
    /// While [`Ready`](SourceState::Ready), each call waits up to 100 ms for
    /// a message so the source can answer re-requests, `Get_Source_Cap` and
    /// `Soft_Reset` while keeping power applied.
    pub fn poll(&mut self, delay: &impl DelayUs) {
        match self.state {
            SourceState::Idle => self.detect(),
            SourceState::AwaitSink => {
                delay.delay_ms(160);
                self.state = SourceState::SendSrcCap;
            }
            SourceState::SendSrcCap => {
                self.state = if self.send_src_caps(delay) {
                    SourceState::AwaitRequest
                } else {
                    SourceState::Error
                };
            }
            SourceState::AwaitRequest => self.await_request(delay),
            SourceState::SendAccept => self.send_accept(delay),
            SourceState::Ready => self.ready(delay),
            SourceState::Error => {}
        }
    }

    /// Put a 330 µA pull-up on both CC lines and detect the sink.
    fn detect(&mut self) {
        for p in [CcPort::Cc1, CcPort::Cc2] {
            let connected = {
                let phy = self.link.phy();
                phy.set_cc_pull(p, CcPull::Pull330uA);
                phy.check_cc(p)
            };
            if connected {
                self.cc_port = p;
                self.link.phy().select_cc(p);
                self.link.start();
                self.state = SourceState::AwaitSink;
                return;
            }
        }
    }

    /// Wait for the sink's `Request`, answering `Get_Source_Cap` / `Soft_Reset`.
    fn await_request(&mut self, delay: &impl DelayUs) {
        let mut elapsed = 0u32;
        while elapsed < 5000 {
            match self.link.receive(100, delay) {
                Some(msg) => match msg.header().message_type() {
                    t if t == DataMessage::Request as u8 => {
                        let rdo = Rdo::from_bits(msg.object(0));
                        let pos = rdo.position();
                        if pos == 0 || (pos as usize) > self.src_caps.len() {
                            // Not one of our PDOs: reject it.
                            self.send_control(ControlMessage::Reject, delay);
                        } else {
                            self.record_request(rdo);
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
                        self.send_control(ControlMessage::Accept, delay);
                    }
                    _ => {}
                },
                None => elapsed += 100,
            }
        }
        self.state = SourceState::Error;
    }

    /// Send `Accept` then `PS_RDY`.
    fn send_accept(&mut self, delay: &impl DelayUs) {
        if !self.send_control(ControlMessage::Accept, delay) {
            self.state = SourceState::Error;
            return;
        }
        delay.delay_ms(2);
        self.state = if self.send_control(ControlMessage::PsRdy, delay) {
            SourceState::Ready
        } else {
            SourceState::Error
        };
    }

    /// Keep answering the sink while power is delivered.
    fn ready(&mut self, delay: &impl DelayUs) {
        if let Some(msg) = self.link.receive(100, delay) {
            match msg.header().message_type() {
                t if t == DataMessage::Request as u8 => {
                    let rdo = Rdo::from_bits(msg.object(0));
                    let pos = rdo.position();
                    if pos != 0 && (pos as usize) <= self.src_caps.len() {
                        self.record_request(rdo);
                        self.state = SourceState::SendAccept;
                    }
                }
                t if t == ControlMessage::GetSourceCap as u8 => {
                    self.state = SourceState::SendSrcCap;
                }
                t if t == ControlMessage::SoftReset as u8 => {
                    self.send_control(ControlMessage::Accept, delay);
                }
                _ => {}
            }
        }
    }

    /// Record the sink's request from its RDO and target PDO.
    fn record_request(&mut self, rdo: Rdo) {
        let pos = rdo.position();
        self.requested_position = pos;
        let Some(pdo) = self.src_caps.get((pos as usize).wrapping_sub(1)) else {
            self.requested_mv = 0;
            self.requested_ma = 0;
            return;
        };
        match pdo.pdo_type() {
            PdoType::FixedSupply => {
                self.requested_mv = pdo.voltage_mv();
                self.requested_ma = rdo.operating_current_ma();
            }
            PdoType::Apdo => match pdo.apdo_type() {
                ApdoType::Pps => {
                    self.requested_mv = rdo.pps_output_voltage_mv();
                    self.requested_ma = rdo.output_current_ma();
                }
                ApdoType::EprAvs | ApdoType::SprAvs => {
                    self.requested_mv = rdo.avs_output_voltage_mv();
                    self.requested_ma = rdo.output_current_ma();
                }
            },
            _ => {
                self.requested_mv = 0;
                self.requested_ma = 0;
            }
        }
    }

    /// Advertise the configured Source_Capabilities.
    fn send_src_caps(&mut self, delay: &impl DelayUs) -> bool {
        let mut objects = [0u32; 7];
        for (i, p) in self.src_caps.iter().enumerate() {
            objects[i] = p.bits();
        }
        let Ok(m) = Message::data(
            DataMessage::SourceCapabilities,
            PowerRole::Source,
            DataRole::Dfp,
            SpecRev::Rev3_0,
            &objects[..self.src_caps.len()],
        ) else {
            return false;
        };
        self.link.send_message(&m, SopType::Sop0, delay).is_ok()
    }

    /// Send a control message as a source.
    fn send_control(&mut self, msg: ControlMessage, delay: &impl DelayUs) -> bool {
        let m = Message::control(msg, PowerRole::Source, DataRole::Dfp, SpecRev::Rev3_0);
        self.link.send_message(&m, SopType::Sop0, delay).is_ok()
    }
}
