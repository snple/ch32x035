//! 上下文切换（RISC-V, QingKe V4C）。
//!
//! 协作式切换只发生在任务显式让出点（普通任务上下文），因此只需保存被调用者
//! 保存的寄存器（`ra` + `s0..=s11`）和栈指针。中断从不在此触发切换。
//!
//! # 中断状态不变量
//!
//! 每个任务都运行在「全局中断使能」状态下。调度器在临界区内完成队列修改，然后
//! 在临界区**之外**执行栈切换，因此 `rtos_switch` 无需触碰中断使能状态；只有
//! `rtos_start_first` 进入的首个任务需要显式建立该不变量。

use core::arch::global_asm;

use crate::task::Task;

/// 每次上下文保存的寄存器数：`ra` + `s0..=s11`。
pub(crate) const CONTEXT_REGS: usize = 13;

/// 一份保存上下文的字节数。
pub(crate) const CONTEXT_BYTES: usize = CONTEXT_REGS * 4;

/// 任务栈底的金丝雀值；被覆盖说明栈溢出。
pub(crate) const CANARY: u32 = 0xDEAD_BEEF;

global_asm!(
    r#"
    .align 2
    .global rtos_switch
    .type rtos_switch, @function
    // a0 = 旧任务保存栈指针的槽位（*mut *mut u32）
    // a1 = 新任务的保存栈指针（*mut u32）
    //
    // 把调用者的 ra + s0..s11 压到自己的栈上，栈指针写入 *a0，切到新任务的栈
    // 并恢复其寄存器。不触碰中断状态——调用方已退出临界区，中断处于使能状态。
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
        sw sp, 0(a0)
        mv sp, a1
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
        ret

    .align 2
    .global rtos_start_first
    .type rtos_start_first, @function
    // a0 = 首个任务的初始栈指针（*mut u32）
    //
    // 建立「任务恒以全局中断使能运行」的内核不变量（CSR 0x800 = MIE|MPIE），
    // 然后装载初始帧并跳入任务入口。永不返回。
    rtos_start_first:
        li t0, 0x88
        csrs 0x800, t0
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
        ret
    "#
);

unsafe extern "C" {
    /// 把当前上下文保存进 `*old_sp_slot` 并恢复 `new_sp`。
    ///
    /// 仅在任务被再次切换回来时返回。调用方必须已退出临界区（中断使能）。
    pub(crate) fn rtos_switch(old_sp_slot: *mut *mut u32, new_sp: *mut u32);

    /// 装载首个任务的初始上下文并进入。永不返回。
    pub(crate) fn rtos_start_first(sp: *mut u32) -> !;
}

/// 构建任务入口的初始栈帧并写入金丝雀。
///
/// 帧布局（低地址在前）：`[ra, s0, s1, ..., s11]`，共 [`CONTEXT_BYTES`] 字节，
/// 栈顶 16 字节对齐。返回存入 `Task::sp` 的保存栈指针。
///
/// # Safety
///
/// `stack` 必须指向至少 `size` 字节的可写内存。
pub(crate) unsafe fn init_stack_frame(
    entry: unsafe extern "C" fn() -> !,
    stack: *mut u8,
    size: usize,
) -> *mut u32 {
    let base = stack as usize;
    // 金丝雀写在栈的最底部。
    unsafe { (base as *mut u32).write_volatile(CANARY) };

    // 栈顶 16 字节对齐（RISC-V ABI 栈对齐要求）。
    let top = (base + size) & !15usize;
    let frame = (top - CONTEXT_BYTES) as *mut u32;

    unsafe { frame.write_volatile(entry as usize as u32) }; // ra
    for i in 1..CONTEXT_REGS {
        unsafe { frame.add(i).write_volatile(0) }; // s0..s11
    }
    frame
}

/// 校验任务栈金丝雀；被覆盖则 panic 并报任务名。
pub(crate) fn check_canary(t: &Task) {
    let magic = unsafe { (t.stack_bottom as *const u32).read_volatile() };
    if magic != CANARY {
        panic!("ch32x035-rtos: stack overflow in task '{}'", t.name);
    }
}
