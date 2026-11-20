//! Common types and utilities for register access

#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// A volatile cell for memory-mapped register access.
///
/// Similar to `vcell::VolatileCell`, this provides volatile read/write access
/// to a memory location with proper semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for VolatileCell<T> {}
unsafe impl<T> Sync for VolatileCell<T> {}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
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
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// A volatile 8-bit register cell.
///
/// Presents the same `u32`-based `read_bits` / `write_bits` / `modify_bits`
/// API as [`VolatileCell<u32>`] so register accessors are width-agnostic,
/// while the actual bus access stays 8 bits wide. Used by byte-spaced
/// peripherals such as USBFS and USBPD.
#[repr(transparent)]
pub struct VolatileCellU8 {
    value: UnsafeCell<u8>,
}

unsafe impl Send for VolatileCellU8 {}
unsafe impl Sync for VolatileCellU8 {}

impl VolatileCellU8 {
    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.value.get()
    }

    /// Read the register with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { read_volatile(self.value.get()) as u32 }
    }

    /// Write the register with volatile semantics (upper bits ignored)
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { write_volatile(self.value.get(), value as u8) }
    }

    /// Modify the register using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// A volatile 16-bit register cell.
///
/// See [`VolatileCellU8`]; the actual bus access is 16 bits wide.
#[repr(transparent)]
pub struct VolatileCellU16 {
    value: UnsafeCell<u16>,
}

unsafe impl Send for VolatileCellU16 {}
unsafe impl Sync for VolatileCellU16 {}

impl VolatileCellU16 {
    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u16 {
        self.value.get()
    }

    /// Read the register with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { read_volatile(self.value.get()) as u32 }
    }

    /// Write the register with volatile semantics (upper bits ignored)
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { write_volatile(self.value.get(), value as u16) }
    }

    /// Modify the register using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// Spin until condition is true
#[inline(always)]
pub fn wait_until<F: Fn() -> bool>(cond: F) {
    while !cond() {
        core::hint::spin_loop();
    }
}

/// Spin while condition is true
#[inline(always)]
pub fn wait_while<F: Fn() -> bool>(cond: F) {
    while cond() {
        core::hint::spin_loop();
    }
}
