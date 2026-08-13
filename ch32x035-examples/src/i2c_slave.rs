//! I2C1 blocking slave example.
//!
//! Configures the CH32X035 as an I2C slave at address `0x50` using the
//! blocking [`I2c1Slave`] driver; clock stretching (the default) pauses the
//! bus while the slave handles each transaction. The I2C1 pins are **PB6**
//! (SCL) and **PB7** (SDA), configured as alternate-function open-drain;
//! connect external pull-ups (e.g. 4.7 kΩ to 3.3 V).
//!
//! With a second board running as I2C master:
//! - master writes N bytes → they land in `rx[..N]`;
//! - master reads → the slave transmits `tx`.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin i2c_slave --release
//! ```

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::i2c::{I2c1Slave, Transfer};
use ch32x035::{main, pac};
use ch32x035_examples::board::Board;

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        I2C1,
        ..
    } = dp;

    let mut board = Board::init(RCC, GPIOA, GPIOB, GPIOC);

    // I2C1: SCL = PB6, SDA = PB7 (alternate-function open-drain)
    let mut scl = board.pins.pb::<6>();
    let mut sda = board.pins.pb::<7>();
    scl.into_alt_open_drain();
    sda.into_alt_open_drain();

    let mut slave = I2c1Slave::init(I2C1, &mut board.rcc, board.clocks, 0x50);

    let mut rx = [0u8; 32];
    let tx = b"hello from ch32x035 slave!";

    loop {
        match slave.listen() {
            Transfer::Write => {
                let n = slave.read(&mut rx);
                // `rx[..n]` holds the bytes the master wrote.
                defmt::info!("master wrote {} byte(s)", n);
            }
            Transfer::Read => {
                slave.write(tx);
            }
        }
    }
}
