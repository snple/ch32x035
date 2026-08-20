//! SysTick 定时器配置与 tick 中断处理。
//!
//! WCH SysTick 每 1 ms 触发一次。`SysTick` 向量入口调用 [`SysTick_impl`] 后用
//! `mret` 返回（WCH 中断协议），期间推进全局 tick 并唤醒到期任务。
//!
//! # 独立中断栈
//!
//! QingKe 硬件栈（CSR 0x804）只保存中断上下文，ISR 的软件调用链仍会占用当前
//! 任务的栈。为避免 ISR 占满任务栈，向量入口把任务 sp 存入 `mscratch`，并把 sp
//! 切换到下面汇编里定义的独立 `__rtos_isr_stack`，出口再恢复任务 sp。每次进入
//! 都从固定栈顶开始，栈水位不会随中断次数累积。
//!
//! # 中断路径（WCH 特有）
//!
//! CH32X035 的 SysTick 中断（向量索引 12）通过 **PFIC**（WCH SDK 的
//! `NVIC_EnableIRQ(SysTick_IRQn)`）使能，而不是标准 `mie` CSR。且处理函数必须用
//! **`mret`** 返回（WCH SDK `WCH-Interrupt-fast` 协议）以恢复全局中断门
//! （CSR 0x800）；普通 `ret` 只会让中断触发一次。

use ch32x035::rt::peripheral::{PFIC, SYST};

/// SysTick IRQ 号（向量表索引 12），与 WCH SDK 一致。
const SYSTICK_IRQ: u16 = 12;

/// 配置 SysTick 每 1 ms 中断一次。
///
/// `sysclk_hz` 是系统时钟频率（如复位后的 48 MHz）。镜像 WCH SDK
/// `SYSTICK_Init_Config`：复位计数器、写比较值、`CTLR = 0xF`
/// （STE|STIE|STCLK|STRE），然后在 PFIC 中使能 SysTick 中断。
pub(crate) fn init(mut syst: SYST, sysclk_hz: u32) {
    let period = (sysclk_hz / 1000).max(1);
    syst.set_compare(period);
    syst.set_count(0);
    syst.clear_flag();
    // CTLR = 0xF：计数器使能、中断使能、HCLK 时钟源、自动重载。
    unsafe { (*SYST::PTR).ctlr.write_bits(0xF) };

    // SysTick 中断必须在 PFIC 中使能（最低优先级）。
    PFIC::set_priority(SYSTICK_IRQ, 15);
    unsafe { PFIC::enable(SYSTICK_IRQ) };
}

/// tick 实际工作，由 `SysTick` 蹦床调用。
///
/// 运行在机器中断上下文（运行于独立 ISR 栈），不得调用任何阻塞/让出 API。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SysTick_impl() {
    let mut syst = unsafe { SYST::steal() };
    syst.clear_flag();
    let now = crate::scheduler::tick_inc();
    crate::scheduler::on_tick(now);
}

// ISR 独立栈（定义在 .bss，由启动代码清零）与 SysTick 向量表入口。
//
// 入口流程：
//   1. `csrw mscratch, sp` —— 把被中断任务的 sp 存进 mscratch；
//   2. `la sp, __rtos_isr_stack_top` —— sp 切换到独立 ISR 栈顶；
//   3. 执行 tick 工作；
//   4. `csrr sp, mscratch` —— 恢复任务 sp；
//   5. `mret` —— 返回（恢复 WCH 全局中断门，`WCH-Interrupt-fast` 协议）。
//
// 没有 `mret` 中断只会触发一次。
core::arch::global_asm!(
    r#"
    .section .bss
    .balign 16
    __rtos_isr_stack:
        .zero 1024
    __rtos_isr_stack_top:

    .section .text
    .align 2
    .global SysTick
    .type SysTick, @function
    SysTick:
        csrw mscratch, sp
        la t0, __rtos_isr_stack_top
        mv sp, t0
        call SysTick_impl
        csrr sp, mscratch
        mret
    "#
);
