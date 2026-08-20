//! Context switching primitives (RISC-V, QingKe V4C).
//!
//! Cooperative switching only happens at explicit yield points in ordinary
//! task context, so we only need to save/restore the callee-saved registers
//! (`ra`, `s0..=s11`) and the stack pointer. Interrupts (which the QingKe
//! hardware stack handles separately) never trigger a context switch here.

use core::arch::global_asm;

use crate::task::Task;

/// Number of callee-saved registers saved per context: `ra` + `s0..=s11`.
pub(crate) const CONTEXT_REGS: usize = 13;

/// Size in bytes of one saved context.
pub(crate) const CONTEXT_BYTES: usize = CONTEXT_REGS * 4;

/// Magic value written at the bottom of every task stack; if it is gone the
/// task overflowed its stack.
pub(crate) const CANARY: u32 = 0xDEAD_BEEF;

global_asm!(
    r#"
    .align 2
    .global rtos_switch
    .type rtos_switch, @function
    // a0 = pointer to the old task's saved-sp slot (*mut *mut u32)
    // a1 = new task's saved stack pointer
    rtos_switch:
        addi sp, sp, -52
        sw ra, 0(sp)
        sw s0, 4(sp)
        sw s1, 8(sp)
        sw s2, 12(sp)
        sw s3, 16(sp)
        sw s4, 20(sp)
        sw s5, 24(sp)
        sw s6, 28(sp)
        sw s7, 32(sp)
        sw s8, 36(sp)
        sw s9, 40(sp)
        sw s10, 44(sp)
        sw s11, 48(sp)
        sw sp, 0(a0)       // persist old sp
        mv sp, a1          // switch stacks
        lw ra, 0(sp)
        lw s0, 4(sp)
        lw s1, 8(sp)
        lw s2, 12(sp)
        lw s3, 16(sp)
        lw s4, 20(sp)
        lw s5, 24(sp)
        lw s6, 28(sp)
        lw s7, 32(sp)
        lw s8, 36(sp)
        lw s9, 40(sp)
        lw s10, 44(sp)
        lw s11, 48(sp)
        addi sp, sp, 52
        // Enable global interrupts (WCH gintenr + MIE). A task switched in
        // from its initial frame has no critical-section frame of its own,
        // so it must not inherit the previous task's disabled-interrupt
        // state; cooperative tasks always run with interrupts enabled.
        li t0, 0x88
        csrs 0x800, t0
        ret

    .align 2
    .global rtos_start_first
    .type rtos_start_first, @function
    // a0 = first task's saved stack pointer (initial frame already built)
    // Enters the task; never returns.
    rtos_start_first:
        mv sp, a0
        lw ra, 0(sp)
        lw s0, 4(sp)
        lw s1, 8(sp)
        lw s2, 12(sp)
        lw s3, 16(sp)
        lw s4, 20(sp)
        lw s5, 24(sp)
        lw s6, 28(sp)
        lw s7, 32(sp)
        lw s8, 36(sp)
        lw s9, 40(sp)
        lw s10, 44(sp)
        lw s11, 48(sp)
        addi sp, sp, 52
        li t0, 0x88
        csrs 0x800, t0
        ret
    "#
);

unsafe extern "C" {
    /// Save the current context into `*old_sp_slot` and resume `new_sp`.
    ///
    /// Returns (in the resumed task) only after that task is switched to
    /// again.
    pub(crate) fn rtos_switch(old_sp_slot: *mut *mut u32, new_sp: *mut u32);

    /// Load the initial context of the first task and enter it.
    pub(crate) fn rtos_start_first(sp: *mut u32) -> !;
}

/// Build the initial stack frame for `entry` and write the canary.
///
/// Frame layout (lowest address first): `[ra, s0, s1, ..., s11]`, 52 bytes,
/// with the top of the stack aligned down to 16 bytes. Returns the saved
/// stack pointer stored in the task's `Task::sp`.
///
/// # Safety
///
/// `stack` must point to at least `size` writable bytes.
pub(crate) unsafe fn init_stack_frame(
    entry: unsafe extern "C" fn() -> !,
    stack: *mut u8,
    size: usize,
) -> *mut u32 {
    let base = stack as usize;
    // Canary at the very bottom of the stack.
    unsafe { (base as *mut u32).write_volatile(CANARY) };

    // Align the top down to 16 bytes (RISC-V ABI stack alignment).
    let top = (base + size) & !15usize;
    let frame = (top - CONTEXT_BYTES) as *mut u32;

    unsafe { frame.write_volatile(entry as usize as u32) }; // ra
    for i in 1..CONTEXT_REGS {
        unsafe { frame.add(i).write_volatile(0) }; // s0..s11
    }
    frame
}

/// Check a task's stack canary; panics if it was overwritten.
pub(crate) fn check_canary(t: &Task) {
    let magic = unsafe { (t.stack_bottom as *const u32).read_volatile() };
    if magic != CANARY {
        panic!("ch32x035-rtos: stack overflow in task '{}'", t.name);
    }
}
