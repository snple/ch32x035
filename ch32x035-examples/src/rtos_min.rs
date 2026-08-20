//! `ch32x035-rtos` 最小可用性检查：两个协作任务。
//!
//! 构建：
//! ```sh
//! cargo build -p ch32x035-examples --bin rtos_min --release \
//!     --target riscv32imac-unknown-none-elf
//! ```
//! 预期 RTT 输出：`[hi]` 与 `[lo]` 交错，`[hi]` 频率约为 `[lo]` 的两倍
//! （500 ms vs 1000 ms 睡眠）。

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use ch32x035::{CorePeripherals, main, pac};
use ch32x035_examples::board::Board;
use ch32x035_rtos::{sleep, start, task};

#[task(priority = 2, name = "hi")]
fn task_hi() -> ! {
    loop {
        info!("[hi] running");
        sleep(500);
    }
}

#[task(priority = 1, name = "lo")]
fn task_lo() -> ! {
    loop {
        info!("[lo] running");
        sleep(1000);
    }
}

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        ..
    } = dp;
    let board = Board::init(RCC, GPIOA, GPIOB, GPIOC);
    let cp = CorePeripherals::take().unwrap();

    info!("rtos_min starting");
    start(cp.SYST, board.clocks.sysclk_hz())
}
