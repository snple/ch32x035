//! USB PD (Power Delivery) driver.
//!
//! The driver is split into four layers, mirroring the USB PD architecture:
//!
//! - [`phy`] — the CH32X035 transceiver: CC detection and DMA
//!   receive/transmit. All frame-level work (preamble, SOP, CRC) is done in
//!   hardware.
//! - [`protocol`] — pure data types: message headers, data objects
//!   ([`Pdo`] / [`Rdo`]) and encode/decode helpers. No hardware access.
//! - [`port`] — the blocking link layer: transmits messages with automatic
//!   GoodCRC acknowledgement and Message-ID management, and receives packets
//!   on top of the [`phy`] layer.
//! - [`policy`] — blocking Source ([`PdSource`]) and Sink ([`PdSink`]) policy
//!   engines that drive a complete power negotiation.
//!
//! The most common types are re-exported at this level for convenience.

pub mod phy;
pub mod policy;
pub mod port;
pub mod protocol;

pub use phy::{CcComparator, CcPort, CcPull, SopType, Usbpd};
pub use policy::{PdSink, PdSource, Request, SinkState, SourceState};
pub use port::Port;
pub use protocol::{
    ApdoType, ChunkedAssembler, ControlMessage, DataMessage, DataRole, EprModeAction,
    ExtendedHeader, ExtendedMsgType, Header, Message, PdError, Pdo, PdoType, PowerRole, Rdo,
    RxExtendedChunk, RxMessage, SpecRev, MAX_CHUNK_DATA, MAX_CHUNKS, MAX_DATA_OBJECTS,
    MAX_EXTENDED_DATA, parse_extended_chunk,
};
