//! # ch32x035-usb
//!
//! USB device stack for the WCH CH32X035 (USBFS full-speed device
//! controller).
//!
//! The MCU-independent part of the stack (bus trait, control pipe, device
//! state machine, CDC-ACM and HID classes) lives in the shared
//! [`usb-stack`](https://github.com/snple/usb-stack) crate and is re-exported
//! here under [`device`]; this crate adds the [`UsbfsBus`] peripheral driver.
//!
//! ## Quick start
//!
//! ```ignore
//! use ch32x035::afio::{Afio, UsbPhySupply};
//! use ch32x035_usb::ch32x035::{Config, UsbfsBus};
//! use ch32x035_usb::device::class::cdc_acm::CdcAcm;
//! use ch32x035_usb::{UsbBusAllocator, UsbDeviceBuilder};
//!
//! let mut afio = Afio::init(dp.AFIO, &mut rcc);
//! let alloc = UsbBusAllocator::new(UsbfsBus::new(
//!     dp.USBFS,
//!     &mut afio,
//!     &mut rcc,
//!     Config::default(),
//! ));
//! let mut serial = CdcAcm::new(&alloc);
//! let mut dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678)
//!     .product("My Device")
//!     .build();
//!
//! loop {
//!     if dev.poll(&mut [&mut serial]) {
//!         let mut buf = [0u8; 64];
//!         if let Ok(n) = serial.read(&mut buf) {
//!             let _ = serial.write(&buf[..n]);
//!         }
//!     }
//! }
//! ```
//!
//! PC16 (D−) and PC17 (D+) must be configured as inputs by the application
//! before enumeration.

#![no_std]
#![deny(missing_docs)]

/// Logging facade (re-exported from [`usb_stack`]).
pub use usb_stack::log;

// ── Shared stack ──────────────────────────────────────────────────────────────
//
// Everything that is MCU-independent lives in `usb-stack`. The vocabulary both
// directions of that crate share (`endpoint`, `descriptor`, `request`) sits at
// its root; the device stack — bus trait, control pipe, state machine and
// classes — lives under `device`.

pub use usb_stack::device;

pub use usb_stack::device::{
    DescriptorWriter, DeviceInfo, DeviceState, Direction, Endpoint, EndpointAddress, EndpointIn,
    EndpointOut, EndpointType, In, InterfaceNumber, IsoSynchronization, IsoUsage, Out, PollResult,
    StringIndex, UsbBus, UsbBusAllocator, UsbClass, UsbDevice, UsbDeviceBuilder, UsbError,
    UsbResult,
};
pub use usb_stack::{BULK_MAX_PACKET_SIZE, CONTROL_MAX_PACKET_SIZE, descriptor, endpoint, request};

// ── CH32X035 implementation ───────────────────────────────────────────────────

pub mod ch32x035;

pub use ch32x035::{Config, UsbfsBus};

/// Number of USBFS endpoints, including EP0.
///
/// Note that the shared stack can track up to [`usb_stack::MAX_ENDPOINTS`];
/// the USBFS peripheral itself only provides eight.
pub const MAX_ENDPOINTS: usize = 8;
