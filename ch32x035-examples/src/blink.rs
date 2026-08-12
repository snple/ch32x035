//! Blink an LED on GPIOA pin 0 using the SysTick-based delay.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin blink --release
//! ```
//! Flash with WCH-Link (`wchisp` or the WCH OpenOCD).

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::{main, Delay, DelayUs, CorePeripherals, pac};
use ch32x035_examples::board::Board;

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

    let mut led = board.pins.pa::<0>();
    led.into_output();

    let delay = Delay::init(cp.SYST, board.clocks.sysclk_hz());

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
