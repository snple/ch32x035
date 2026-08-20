//! Interrupt management
//!
//! Thin wrappers over the PFIC interrupt controller, plus a single-core
//! critical section ([`free`] + [`CriticalSection`] token) and an interrupt-safe
//! [`Mutex`] — both modeled on the rp2350 HAL.
//!
//! The `critical-section` crate impl is registered separately (behind the
//! `critical-section-impl` feature) **only** so that third-party crates such as
//! `defmt` can enter critical sections. Application and kernel code should use
//! [`free`] / [`Mutex`] here, not the `critical-section` crate directly.
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

use core::arch::asm;
use core::cell::UnsafeCell;

use crate::pac::Interrupt;
use crate::rt::peripheral::PFIC;

/// Read the WCH global-interrupt gate (CSR 0x800).
///
/// Bit 3 is the global interrupt enable, bit 7 the saved enable state used
/// by `mret` to restore interrupts after an ISR. This is the register the
/// WCH SDK `__enable_irq` / `__disable_irq` operate on.
#[inline]
#[allow(asm_sub_register)]
fn read_gintenr() -> u32 {
    let value: u32;
    unsafe {
        asm!(
            "csrr {0}, 0x800",
            out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

/// Write the WCH global-interrupt gate (CSR 0x800).
#[inline]
#[allow(asm_sub_register)]
unsafe fn write_gintenr(value: u32) {
    unsafe {
        asm!(
            "csrw 0x800, {0}",
            in(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

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

/// Token proving code is running in a critical section (interrupts disabled).
///
/// Obtainable only from [`free`]; on a single-core chip this is proof that no
/// interrupt can concurrently touch protected state.
#[derive(Clone, Copy)]
pub struct CriticalSection {
    _marker: core::marker::PhantomData<*const ()>,
}

impl CriticalSection {
    /// Create a new critical section token.
    ///
    /// # Safety
    ///
    /// Must only be called while interrupts are actually disabled.
    #[inline(always)]
    pub(crate) unsafe fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

/// A mutual exclusion primitive for protecting shared data on a single-core
/// system, accessed only inside a critical section.
///
/// Similar to `std::sync::Mutex`, but access is gated by a [`CriticalSection`]
/// token (interrupts disabled) rather than by an OS lock.
///
/// # Example
///
/// ```ignore
/// use core::cell::RefCell;
/// use ch32x035::interrupt::{self, Mutex};
///
/// static COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
///
/// interrupt::free(|cs| {
///     *COUNTER.borrow(cs).borrow_mut() += 1;
/// });
/// ```
pub struct Mutex<T> {
    data: UnsafeCell<T>,
}

// SAFETY: access to `data` is only possible with a `CriticalSection` token,
// which proves interrupts are disabled; on a single-core system this prevents
// data races.
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    /// Create a new `Mutex` containing `value`.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
        }
    }

    /// Borrow the data inside the mutex.
    ///
    /// Requires a [`CriticalSection`] token proving interrupts are disabled.
    #[inline(always)]
    pub fn borrow(&self, _cs: CriticalSection) -> &T {
        // SAFETY: holding a `CriticalSection` token means interrupts are off;
        // on a single core nothing else can access the data.
        unsafe { &*self.data.get() }
    }

    /// Get a raw pointer to the contained data.
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.data.get()
    }
}

/// Execute `f` in a critical section (interrupts disabled).
///
/// The closure receives a [`CriticalSection`] token. Interrupts are restored
/// to their previous state on return.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    let state = unsafe { cs_acquire() };
    // SAFETY: `CriticalSection::new` is valid while interrupts are disabled.
    let r = f(unsafe { CriticalSection::new() });
    unsafe { cs_release(state) };
    r
}

/// Acquire a critical section and return the restore state.
///
/// Saves the WCH global-interrupt gate (CSR 0x800) and disables global
/// interrupts. The returned state restores the gate on release.
///
/// The save/restore is symmetric (same CSR on both paths), so nesting works
/// naturally: an inner acquire observes an already-disabled gate, restores it
/// on its release, and the outer release restores the originally-enabled gate.
///
/// # Safety
///
/// The returned state must be passed to [`cs_release`].
#[inline]
pub unsafe fn cs_acquire() -> u32 {
    let state = read_gintenr();
    crate::rt::interrupt::disable();
    state
}

/// Release a critical section using the state from [`cs_acquire`].
///
/// Restores the saved WCH global-interrupt gate, re-enabling interrupts if
/// they were enabled before the matching acquire.
///
/// # Safety
///
/// `state` must come from a matching [`cs_acquire`] call.
#[inline]
pub unsafe fn cs_release(state: u32) {
    // SAFETY: restoring a previously-saved gate value.
    unsafe { write_gintenr(state) };
}
