//! USB PD (Power Delivery) driver.
//!
//! Organized into three layers, mirroring the USB PD architecture:
//!
//! - [`phy`] - the transceiver: BMC encode/decode, CC detection and DMA
//!   receive/transmit. All frame-level work (preamble, SOP, CRC) is done in
//!   hardware by the CH32X035.
//! - [`protocol`] - PD messages: the [`Header`], data objects ([`Pdo`]),
//!   build/parse helpers and request/response primitives with automatic
//!   GoodCRC acknowledgement and Message-ID management.
//! - [`policy`] - blocking Source ([`PdSource`]) and Sink ([`PdSink`])
//!   policy engines that drive a full power negotiation.
//!
//! The most common types are re-exported at this level, so both
//! `usbpd::Usbpd` and `usbpd::phy::Usbpd` work.

pub mod phy;
pub mod policy;
pub mod protocol;

pub use phy::{CcComparator, CcPort, CcPull, SopType, Usbpd};
pub use policy::{PdSink, PdSource, SinkState, SourceState};
pub use protocol::{
    ApdoType, ChunkedAssembler, ControlMessage, DataMessage, DataRole, EprModeAction,
    ExtendedHeader, ExtendedMsgType, Header, MAX_CHUNK_DATA, MAX_CHUNKS, MAX_EXTENDED_DATA,
    PdError, Pdo, PdoType, PowerRole, RxExtendedChunk, RxMessage, SpecRev, build_extended_chunk,
    build_message, make_avs_rdo, make_epr_avs_pdo, make_epr_mode, make_epr_request, make_fixed_pdo,
    make_pps_rdo, make_request_rdo, make_spr_avs_pdo, parse_extended_chunk, parse_message,
};
