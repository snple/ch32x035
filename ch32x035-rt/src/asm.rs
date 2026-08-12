//! RISC-V (QingKe V4C) Assembly Instructions
//!
//! Low-level assembly instructions for the WCH QingKe V4C core used in the
//! CH32X035. These are implemented using inline assembly for optimal
//! performance.
//!
//! ## Power / Wait Instructions
//!
//! - `wfi()` - Wait for interrupt
//! - `wfe()` - Wait for event
//!
//! ## Trap Instructions
//!
//! - `ecall()` - Environment call (enters machine-mode trap handler)
//! - `mret()` - Return from machine-mode trap
//! - `ebreak()` - Breakpoint (enters trap handler)
//! - `udf()` - Undefined instruction (guaranteed trap)
//!
//! ## Memory Barriers
//!
//! - `fence()` - Full memory fence
//! - `fencei()` - Instruction fence (flush instruction cache / pipeline)

use core::arch::asm;

// =============================================================================
// Basic Instructions
// =============================================================================

/// No operation
///
/// Useful to prevent delay loops from being optimized away.
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack, preserves_flags)) }
}

/// Wait for interrupt
///
/// Puts the processor into a low-power sleep state until an interrupt occurs.
/// This instruction only returns after an interrupt is received.
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) }
}

/// Wait for event
///
/// If the event register is clear, puts the processor into a low-power state
/// until either an event or an interrupt occurs.
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe", options(nomem, nostack, preserves_flags)) }
}

// =============================================================================
// Trap Instructions
// =============================================================================

/// Environment call
///
/// Triggers an environment call trap. In machine mode this invokes the
/// `EcallMMode` trap handler.
#[inline(always)]
pub fn ecall() {
    unsafe { asm!("ecall", options(nomem, nostack, preserves_flags)) }
}

/// Return from machine-mode trap
///
/// Restores the machine mode state from `mstatus`/`mepc` and resumes
/// execution at `mepc`.
#[inline(always)]
pub fn mret() {
    unsafe { asm!("mret", options(nomem, nostack, preserves_flags)) }
}

/// Breakpoint
///
/// Triggers a debug breakpoint. If no debugger is connected, this causes a
/// trap (the `BreakPoint` handler).
#[inline(always)]
pub fn ebreak() {
    unsafe { asm!("ebreak", options(nomem, nostack, preserves_flags)) }
}

/// Undefined instruction
///
/// Causes an undefined instruction trap. This can be used as a `panic!`-like
/// abort mechanism that will never return.
#[inline(always)]
pub fn udf() -> ! {
    unsafe {
        // `unimp` is the standard RISC-V "always illegal" instruction.
        asm!("unimp", options(noreturn, nomem, nostack));
    }
}

// =============================================================================
// Memory Barriers
// =============================================================================

/// Full memory fence
///
/// Ensures that all memory accesses before the fence are completed before
/// any memory accesses after the fence.
#[inline(always)]
pub fn fence() {
    unsafe { asm!("fence", options(nomem, nostack, preserves_flags)) }
}

/// Instruction fence
///
/// Flushes the instruction pipeline and caches. Must be called after
/// self-modifying code, e.g. after copying code into RAM or changing the
/// vector table.
#[inline(always)]
pub fn fencei() {
    unsafe { asm!("fence.i", options(nomem, nostack, preserves_flags)) }
}

// =============================================================================
// Delay
// =============================================================================

/// Block for at least `cycles` CPU cycles
///
/// This is implemented as a counted loop in assembly to provide a timing
/// reference that is independent of compiler optimizations. However, the
/// actual delay may be longer due to interrupts being serviced.
///
/// Each iteration takes approximately 3 cycles on the QingKe V4C core.
#[inline(always)]
pub fn delay(cycles: u32) {
    unsafe {
        asm!(
            "1:",
            "addi {0}, {0}, -1",
            "bnez {0}, 1b",
            inout(reg) cycles => _,
            options(nomem, nostack),
        )
    }
}
