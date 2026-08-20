//! `critical-section` implementation (private)
//!
//! Registers a critical-section implementation for the CH32X035. It saves the
//! WCH global-interrupt gate (CSR 0x800) and disables global interrupts on
//! acquire, then restores the saved gate on release. The actual logic lives in
//! [`crate::interrupt`].

use critical_section::{RawRestoreState, set_impl};

use crate::interrupt;

struct Ch32x035CriticalSection;

set_impl!(Ch32x035CriticalSection);

unsafe impl critical_section::Impl for Ch32x035CriticalSection {
    #[inline]
    unsafe fn acquire() -> RawRestoreState {
        unsafe { interrupt::cs_acquire() }
    }

    #[inline]
    unsafe fn release(state: RawRestoreState) {
        unsafe { interrupt::cs_release(state) }
    }
}
