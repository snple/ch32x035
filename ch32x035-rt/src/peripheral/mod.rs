//! WCH QingKe V4C Core Peripherals
//!
//! This module provides access to the core peripherals of the CH32X035:
//!
//! - **PFIC** - Programmable Fast Interrupt Controller (WCH, replaces NVIC)
//! - **SYST** - SysTick timer (64-bit, WCH custom, at 0xE000_F000)
//!
//! # WCH PFIC vs standard RISC-V
//!
//! The CH32X035 does **not** use the standard RISC-V PLIC/CLINT. Instead it
//! uses WCH's PFIC at 0xE000_E000 with a layout similar to an NVIC
//! (IENR/IRER/IPSR/IPRR/IACTR arrays plus an 8-bit priority register per
//! interrupt). See [`pfic`] for details.

use core::sync::atomic::{AtomicBool, Ordering};

pub mod pfic;
pub mod syst;

// Re-export peripheral types
pub use pfic::PFIC;
pub use syst::SYST;

/// Singleton guard: the CH32X035 is single-core, so one flag is enough.
static TAKEN: AtomicBool = AtomicBool::new(false);

/// Core Peripherals container
///
/// Use `Peripherals::take()` to get ownership.
#[allow(non_snake_case)]
pub struct Peripherals {
    /// Programmable Fast Interrupt Controller
    pub PFIC: PFIC,
    /// SysTick timer
    pub SYST: SYST,
}

impl Peripherals {
    /// Take the core peripherals singleton.
    ///
    /// Returns `Some(Peripherals)` on the first call, `None` on subsequent
    /// calls.
    #[inline]
    pub fn take() -> Option<Self> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Unsafely create an instance of the core peripherals
    ///
    /// # Safety
    ///
    /// This bypasses the singleton check. The caller must ensure that
    /// no other code has access to the peripherals to prevent data races.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self {
            PFIC: unsafe { PFIC::steal() },
            SYST: unsafe { SYST::steal() },
        }
    }

    /// Release the peripherals back to the pool
    ///
    /// After calling this, `take()` will succeed again.
    #[inline]
    pub fn release(self) {
        TAKEN.store(false, Ordering::Release);
    }
}

// =============================================================================
// Peripheral Base Addresses
// =============================================================================

/// PFIC base address
pub const PFIC_BASE: u32 = 0xE000_E000;

/// SysTick base address
pub const SYST_BASE: u32 = 0xE000_F000;

// =============================================================================
// Peripheral Wrapper Macro
// =============================================================================

/// Helper macro to create peripheral wrappers
macro_rules! peripheral {
    ($name:ident, $reg:ty, $base:expr) => {
        impl $name {
            /// Pointer to the peripheral's register block
            pub const PTR: *const $reg = $base as *const $reg;

            /// Steal an instance of this peripheral
            ///
            /// # Safety
            ///
            /// Ensure that the new instance of the peripheral cannot be used in a way
            /// that may race with any existing instances.
            #[inline]
            pub unsafe fn steal() -> Self {
                Self {
                    _marker: PhantomData,
                }
            }

            /// Returns a raw pointer to the register block
            #[inline(always)]
            pub const fn ptr() -> *const $reg {
                Self::PTR
            }
        }

        impl core::ops::Deref for $name {
            type Target = $reg;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::PTR }
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
}

pub(crate) use peripheral;

/// A volatile cell for memory-mapped register access.
///
/// Similar to `vcell::VolatileCell`, this provides volatile read/write access
/// to a memory location with proper semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: core::cell::UnsafeCell<T>,
}

unsafe impl<T: Send> Send for VolatileCell<T> {}
unsafe impl<T> Sync for VolatileCell<T> {}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: core::cell::UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> VolatileCell<T> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> T {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}
