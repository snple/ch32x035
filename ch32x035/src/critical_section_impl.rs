//! `critical-section` implementation (private)
//!
//! Registers a critical-section implementation for the CH32X035 **solely** for
//! third-party crates such as `defmt` that rely on the `critical-section`
//! crate. Application and kernel code should use [`crate::interrupt::free`] /
//! [`crate::interrupt::Mutex`] instead.
//!
//! The actual logic lives in [`crate::interrupt`] (`cs_acquire` / `cs_release`).

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
