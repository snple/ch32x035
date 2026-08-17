//! I2C1 blocking slave example.
//!
//! Configures the CH32X035 as an I2C slave at address `0x50` using the
//! blocking [`I2c1Slave`] driver; clock stretching (the default) pauses the
//! bus while the slave handles each transaction. The I2C1 pins are **PC16**
//! (SCL) and **PC17** (SDA) via I2C1 remap 2, configured as alternate
//! push-pull; the I2C peripheral switches them to open-drain automatically;
//! connect external pull-ups (e.g. 4.7 kΩ to 3.3 V). PC16/PC17 are bonded
//! out on every CH32X035 package.
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

use ch32x035::afio::{Afio, I2c1Remap};
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
        AFIO,
        I2C1,
        ..
    } = dp;

    let mut board = Board::init(RCC, GPIOA, GPIOB, GPIOC);

    // I2C1 remap 2: SCL = PC16, SDA = PC17. These pins exist on every
    // CH32X035 package (unlike the PB6/PB7 the older example used). The I2C
    // peripheral switches them to open-drain automatically, so configure
    // them as alternate push-pull.
    let mut afio = Afio::init(AFIO, &mut board.rcc);
    afio.set_i2c1_remap(I2c1Remap::Remap2);

    let mut scl = board.pins.pc::<16>();
    let mut sda = board.pins.pc::<17>();
    scl.into_alt_push_pull();
    sda.into_alt_push_pull();

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
