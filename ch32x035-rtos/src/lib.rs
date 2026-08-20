//! A minimal cooperative (non-preemptive) multitasking kernel for the
//! CH32X035 (RISC-V QingKe V4C).
//!
//! # Overview
//!
//! Tasks are registered with the [`task`] attribute macro. The scheduler is
//! cooperative: a task runs until it explicitly yields with
//! [`yield_now`], blocks for a while with [`sleep`], or blocks waiting on a
//! [`sync::Semaphore`] / [`sync::Mutex`]. Static priorities decide *which*
//! task runs next at a yield point; higher-priority tasks do not preempt a
//! running task.
//!
//! The kernel is tick driven by the WCH SysTick timer (1 ms ticks). While a
//! task is blocked the kernel runs the idle task, which executes `wfi` to
//! wait for interrupts and avoid busy-waiting.
//!
//! # Example
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use ch32x035::{main, pac};
//! use ch32x035_rtos::{task, sleep};
//! use ch32x035_examples::board::Board;
//!
//! #[task(priority = 2, stack = 1024)]
//! fn hi() -> ! {
//!     loop {
//!         // ...
//!         sleep(250);
//!     }
//! }
//!
//! #[main]
//! fn main() -> ! {
//!     let dp = pac::Peripherals::take().unwrap();
//!     let board = Board::init(dp.RCC, dp.GPIOA, dp.GPIOB, dp.GPIOC);
//!     let cp = ch32x035::CorePeripherals::take().unwrap();
//!     ch32x035_rtos::start(cp.SYST, board.clocks.sysclk_hz())
//! }
//! ```
//!
//! # Constraints
//!
//! - Tasks must have signature `fn() -> !` and take no arguments.
//! - Never call a blocking/yielding function (`yield_now`, `sleep`,
//!   `Semaphore::take`, `Mutex::lock`) from inside a critical section.
//! - The kernel owns the SysTick timer; the HAL `Delay` busy-wait driver
//!   must not be used while the scheduler is running.
//! - Interrupt handlers must not call blocking APIs; use
//!   [`sync::Semaphore::give_from_isr`] instead, which only pokes the
//!   scheduler.

#![no_std]

pub use ch32x035_rtos_macro::task;

pub mod context;
pub mod scheduler;
pub mod sync;
pub mod task;
/// Alias of [`task`] used by the `#[task]` macro expansion.
pub use task as task_mod;

mod tick;

pub use scheduler::{sleep, start, task_infos, yield_now};
