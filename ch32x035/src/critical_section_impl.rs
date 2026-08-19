//! `critical-section` implementation (private)
//!
//! Registers a critical-section implementation for the CH32X035 that saves
//! `mstatus` and disables global interrupts (`mstatus.MIE`), tracking nesting
//! so interrupts are only re-enabled on the outermost release. The actual
//! logic lives in [`crate::interrupt`].

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
