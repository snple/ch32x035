//! System startup initialization
//!
//! Called by the [`crate::main`] macro before the user code runs. On the
//! CH32X035 the `ch32x035-rt` reset handler already performs all core
//! initialization (stack, `.data`/`.bss`, vector table, interrupts, WCH core
//! configuration), so this is intentionally a no-op that exists for symmetry
//! with the rp2350 `#[main]` flow and as a place to hook early init.

/// Perform early system initialization.
///
/// # Safety
///
/// This function may only be called once, before any user code that relies on
/// the system being fully initialized.
#[inline]
pub unsafe fn init() {
    // The CH32X035 RT reset handler configures everything we need at boot:
    //   - stack pointer
    //   - .data / .bss initialization
    //   - WCH core config (pipelining, interrupt nesting + hardware stack)
    //   - mstatus MIE (global interrupts enabled)
    //   - mtvec → vector table (WCH vectored mode)
    //
    // No additional work is required here.
}
