//! # ch32x035
//!
//! Hardware Abstraction Layer (HAL) for the CH32X035 microcontroller
//! (WCH, RISC-V QingKe V4C core).
//!
//! This crate is organized in the style of the `pico-rs` project:
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
pub use rcc::{Clocks, Rcc};
pub use time::{Duration, Instant};
pub use timer::{Timer1Pwm, Timer2, Timer2Pwm, Timer3};
pub use uart::Uart1;
pub use usbpd::{CcComparator, CcPort, CcPull, SopType, Usbpd};

/// A wrapper that allows access to a peripheral through `&self` only.
///
/// This is the "escape hatch" used by `unsafe fn steal()` implementations: it
/// prevents calling `&mut self` methods (which would require ownership) while
/// still allowing read/shared access. Modeled on the pico-rs `Steal<T>`.
#[derive(Debug)]
pub struct Steal<T> {
    inner: T,
}

impl<T> Steal<T> {
    /// Create a new wrapper
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Get a reference to the inner value
    pub fn get(&self) -> &T {
        &self.inner
    }
}

impl<T> core::ops::Deref for Steal<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// SAFETY: a `Steal<T>` is only ever created via `unsafe fn steal()`, and the
// caller guarantees exclusive ownership semantics.
unsafe impl<T> Send for Steal<T> {}
