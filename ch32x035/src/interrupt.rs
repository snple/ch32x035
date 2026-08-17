//! Interrupt management
//!
//! Thin wrappers over the PFIC interrupt controller plus the
//! [`critical-section`] integration used by [`crate::interrupt::free`].
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{interrupt, pac::Interrupt};
//!
//! interrupt::enable(Interrupt::USART1_IRQ);
//! interrupt::free(|cs| {
//!     // critical section
//! });
//! ```

use core::sync::atomic::{AtomicU8, Ordering};

use critical_section::{CriticalSection, RawRestoreState};

use crate::pac::Interrupt;
use crate::rt::peripheral::PFIC;

/// Critical-section nesting counter.
static CS_NESTING: AtomicU8 = AtomicU8::new(0);

/// Enable an interrupt
#[inline]
pub fn enable(interrupt: Interrupt) {
    unsafe { PFIC::unmask(interrupt) }
}

/// Disable an interrupt
#[inline]
pub fn disable(interrupt: Interrupt) {
    PFIC::mask(interrupt);
}

/// Check if an interrupt is enabled
#[inline]
pub fn is_enabled(interrupt: Interrupt) -> bool {
    PFIC::is_enabled_int(interrupt)
}

/// Set an interrupt as pending
#[inline]
pub fn pend(interrupt: Interrupt) {
    PFIC::pend_int(interrupt);
}

/// Clear a pending interrupt
#[inline]
pub fn unpend(interrupt: Interrupt) {
    PFIC::unpend_int(interrupt);
}

/// Check if an interrupt is pending
#[inline]
pub fn is_pending(interrupt: Interrupt) -> bool {
    PFIC::is_pending_int(interrupt)
}

/// Set the priority of an interrupt (lower = higher priority)
#[inline]
pub fn set_priority(interrupt: Interrupt, priority: u8) {
    PFIC::set_priority_int(interrupt, priority);
}

/// Get the priority of an interrupt
#[inline]
pub fn priority(interrupt: Interrupt) -> u8 {
    PFIC::priority_int(interrupt)
}

/// Check if the code is currently running inside an interrupt handler
///
/// The CH32X035 runs in machine mode; `mcause` bit 31 indicates an interrupt.
#[inline]
pub fn in_interrupt() -> bool {
    crate::rt::register::mcause::read().is_interrupt()
}

/// Execute `f` in a critical section (interrupts disabled, nesting-aware)
///
/// Interrupts are restored to their previous state on return, even if `f`
/// panics (the panic handler disables interrupts anyway).
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    let state = unsafe { cs_acquire() };
    // SAFETY: `CriticalSection::new` is safe to call inside a critical section.
    let r = f(unsafe { CriticalSection::new() });
    unsafe { cs_release(state) };
    r
}

/// Acquire a critical section and return the restore state
///
/// Saves `mstatus`, disables global interrupts and increments the nesting
/// counter. The returned state restores `mstatus` on the outermost release.
///
/// # Safety
///
/// The returned state must be passed to [`cs_release`].
#[inline]
pub unsafe fn cs_acquire() -> RawRestoreState {
    let mstatus = crate::rt::register::mstatus::read_raw();
    crate::rt::interrupt::disable();

    let prev = CS_NESTING.fetch_add(1, Ordering::AcqRel);
    if prev == 0 { mstatus } else { 0 }
}

/// Release a critical section using the state from [`cs_acquire`]
///
/// Decrements the nesting counter; on the outermost release restores the
/// saved `mstatus` (re-enabling interrupts if they were enabled).
///
/// # Safety
///
/// `state` must come from a matching [`cs_acquire`] call.
#[inline]
pub unsafe fn cs_release(state: RawRestoreState) {
    let prev = CS_NESTING.fetch_sub(1, Ordering::AcqRel);
    if prev == 1 {
        // SAFETY: restoring a previously-saved mstatus value.
        unsafe { crate::rt::register::mstatus::write_raw(state) };
    }
}
