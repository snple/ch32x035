//! Minimal sanity check for `ch32x035-rtos`: two cooperative tasks.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin rtos_min --release
//! ```
//! Expected RTT output: `[hi]` and `[lo]` interleaved, `[hi]` twice as often
//! as `[lo]` (500 ms vs 1000 ms sleeps).

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use ch32x035::{CorePeripherals, main, pac};
use ch32x035_examples::board::Board;
use ch32x035_rtos::{sleep, start, task};

#[task(priority = 2, stack = 1024)]
fn task_hi() -> ! {
    loop {
        info!("[hi] running");
        sleep(500);
    }
}

#[task(priority = 1, stack = 1024)]
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
