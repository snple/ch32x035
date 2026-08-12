//! Interrupt Management (RISC-V mstatus.MIE)
//!
//! On the CH32X035 the global interrupt enable/disable is controlled by the
//! `MIE` (and `MPIE`) bits of the `mstatus` CSR, following the WCH convention
//! (`__disable_irq` / `__enable_irq` from `core_riscv.h`):
//!
//! - Disable: clear `mstatus.MIE | mstatus.MPIE` (`csrc 0x800, 0x88`) + `fence.i`
//! - Enable:  set   `mstatus.MIE | mstatus.MPIE` (`csrs 0x800, 0x88`)
//!
//! Individual interrupt sources are enabled/disabled via the PFIC
//! (see [`crate::peripheral::pfic`]).

use core::arch::asm;

use crate::register::mstatus;

/// Mask of the global-interrupt bits used by WCH (`MIE` = bit 3, `MPIE` = bit 7).
const MIE_BITS: u32 = 0x88;

/// Disable all maskable interrupts
///
/// Clears `mstatus.MIE` and `mstatus.MPIE` (WCH convention) and issues a
/// `fence.i` so the effect is immediately visible.
#[inline(always)]
pub fn disable() {
    unsafe {
        asm!(
            "csrc 0x800, {0}",
            "fence.i",
            in(reg) MIE_BITS,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Enable all maskable interrupts
///
/// Sets `mstatus.MIE` and `mstatus.MPIE` (WCH convention).
///
/// # Safety
///
/// Enabling interrupts can cause interrupt handlers to run at any time.
/// Ensure all interrupt handlers are properly configured and that enabling
/// interrupts won't corrupt any state being modified.
#[inline(always)]
pub unsafe fn enable() {
    unsafe {
        asm!(
            "csrs 0x800, {0}",
            in(reg) MIE_BITS,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Check if interrupts are currently enabled
#[inline(always)]
pub fn is_enabled() -> bool {
    mstatus::read().mie()
}
