//! # ch32x035
//!
//! Hardware Abstraction Layer (HAL) for the CH32X035 microcontroller
//! (WCH, RISC-V QingKe V4C core).
//!
//! This crate is organized in the style of the `rp2350` project:
//!
//! - [`pac`] - low-level register access (re-export of `ch32x035-pac`)
//! - [`rt`] - runtime support (re-export of `ch32x035-rt`)
//! - [`gpio`], [`rcc`], [`uart`], [`spi`], [`i2c`], [`adc`], [`timer`], ...
//!   - type-safe, ownership-based peripheral drivers
//!
//! ## Features
//!
//! - `defmt` - defmt logging support
//! - `critical-section-impl` - register the critical-section implementation
//! - `panic-handler` - provide a default panic handler
//! - `run-from-ram` - link the image for RAM execution
//! - `ch32x035r8t6` / `ch32x035c8t6` / `ch32x035g8u6` / `ch32x035g8r6` /
//!   `ch32x035f8u6` / `ch32x035f7p6` / `ch32x035d8u6` - select exactly one
//!   sub-model (package); see [`package`]. This gates which GPIO pins are
//!   available at compile time.
//!
//! ## Example
//!
//! See `ch32x035-examples`.

#![no_std]
#![deny(missing_docs)]
#![allow(clippy::missing_safety_doc)]

pub mod adc;
pub mod afio;
pub mod awu;
pub mod delay;
pub mod dma;
pub mod esig;
pub mod exti;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
pub mod opa;
pub mod package;
pub mod pwr;
pub mod rcc;
pub mod reset;
pub mod spi;
pub mod startup;
pub mod time;
pub mod timer;
pub mod uart;
pub mod usbpd;
pub mod watchdog;

mod critical_section_impl;
mod panic_handler;
mod spin;

#[cfg(feature = "defmt")]
pub use defmt;

// Re-export the macros, PAC and RT
pub use ch32x035_macro::{bind, exception, main, ramfunc};
pub use ch32x035_pac as pac;
pub use ch32x035_pac::{__INTERRUPTS, Interrupt, Peripherals, Vector};
pub use ch32x035_rt as rt;
pub use ch32x035_rt::peripheral::Peripherals as CorePeripherals;

// Top-level type re-exports
pub use delay::{Delay, DelayUs};
pub use gpio::{Pin, Pins};
pub use package::MODEL;
pub use rcc::{Clocks, Rcc};
pub use time::{Duration, Instant};
pub use timer::{Timer1Pwm, Timer2, Timer2Pwm, Timer3};
pub use uart::Uart1;
pub use usbpd::{
    CcComparator, CcPort, CcPull, PdSink, PdSource, SinkState, SopType, SourceState, Usbpd,
};

/// Zero-cost wrapper returned by `steal()` methods.
///
/// Implements [`Deref`](core::ops::Deref) but **not** `DerefMut`, so the
/// caller can only obtain `&T` — preventing access to `&mut self` methods
/// (e.g. `init`, `release`). Modeled on the rp2350 `Steal<T>`.
pub struct Steal<T>(pub(crate) T);

impl<T> core::ops::Deref for Steal<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for Steal<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

// SAFETY: a `Steal<T>` is only ever created via `unsafe fn steal()`, and the
// caller guarantees exclusive ownership semantics.
unsafe impl<T> Send for Steal<T> {}
