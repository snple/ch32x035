//! CH32X035（RISC-V QingKe V4C）的协作式（非抢占）多任务内核。
//!
//! # 概览
//!
//! 任务由 [`task`] 属性宏注册。调度是协作式的：任务一直运行到主动调用
//! [`yield_now`] 让出、[`sleep`] 睡眠，或阻塞在 [`sync::Semaphore`] /
//! [`sync::Mutex`] 上。静态优先级只决定让出点选择哪个任务运行；高优先级任务
//! 不会抢占正在运行的任务。
//!
//! 内核由 WCH SysTick 定时器以 1 ms 为周期驱动。无任务可运行时执行 idle 任务，
//! 后者用 `wfi` 等待中断。
//!
//! # 约束
//!
//! - 任务签名必须为 `fn() -> !`，无参数。
//! - 禁止在临界区（`interrupt::free`）内调用任何让出/阻塞
//!   函数（`yield_now`、`sleep`、`Semaphore::take`、`Mutex::lock`）：
//!   内核会在入口断言并 panic（`rtos_switch` 另会强制恢复全局中断使能）。
//! - 任务恒以全局中断使能运行；中断处理函数中不得调用阻塞 API。
//! - 内核独占 WCH SysTick；不要使用 HAL 的 `Delay` 忙等驱动。
//! - 中断处理函数中需要唤醒任务时用 [`sync::Semaphore::give_from_isr`]。

#![no_std]

pub use ch32x035_rtos_macro::task;

pub mod context;
pub mod scheduler;
pub mod sync;
pub mod task;
/// [`task`] 宏展开时使用的别名。
pub use task as task_mod;

mod tick;

pub use scheduler::{sleep, start, task_infos, yield_now};
