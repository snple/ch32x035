//! Print "Hello" over USART1 (default pins: PA9 = TX, PA10 = RX).
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin hello_uart --release
//! ```

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::{
    main, pac,
    uart::{Config, Uart1},
};
use ch32x035_examples::board::Board;

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        USART1,
        ..
    } = dp;

    let mut board = Board::init(RCC, GPIOA, GPIOB, GPIOC);

    let mut uart = Uart1::init(USART1, &mut board.rcc, board.clocks, Config::default());

    let mut counter: u32 = 0;
    loop {
        uart.write_str("Hello from ch32x035! count = ");
        // Small manual number printer (no fmt/alloc on this target)
        let mut buf = [0u8; 10];
        let mut n = counter;
        let mut idx = buf.len();
        loop {
            idx -= 1;
            buf[idx] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        for &b in &buf[idx..] {
            uart.write_byte(b);
        }
        uart.write_str("\r\n");
        counter += 1;
    }
}
