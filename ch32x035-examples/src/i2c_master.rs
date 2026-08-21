//! I2C1 blocking master example.
//!
//! Configures the CH32X035 as an I2C master at 100 kHz (standard mode) and
//! scans the bus for responding 7-bit slaves. Every address in the standard
//! range (`0x08`..=`0x77`) is probed with a NACK-only transaction; an
//! acknowledge means a device is present.
//!
//! The I2C1 pins are **PC16** (SCL) and **PC17** (SDA) via I2C1 remap 2,
//! configured as alternate push-pull; the I2C peripheral switches them to
//! open-drain automatically. Connect external pull-ups (e.g. 4.7 kΩ to 3.3 V)
//! and wire them to the slave board's SCL/SDA. PC16/PC17 are bonded out on
//! every CH32X035 package.
//!
//! With a second board running as an I2C slave (see `i2c_slave`):
//! - the scan reports the slave's address (`0x50` here);
//! - a follow-up `write_read` reads the slave's reply back.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin i2c_master --release
//! ```

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::afio::{Afio, I2c1Remap};
use ch32x035::i2c::{Config, Error, I2c1};
use ch32x035::{main, pac};
use ch32x035_examples::board::Board;

/// Probe one 7-bit address with a START + address(NACK) + STOP.
///
/// Returns `true` if the slave acknowledged the address.
fn probe(i2c: &mut I2c1, addr: u8) -> bool {
    match i2c.try_write(addr, &[]) {
        Ok(()) => true,
        // A NACK surfaces as Acknowledge; anything else is a genuine bus fault.
        Err(Error::Acknowledge) => false,
        Err(e) => {
            defmt::warn!("bus error probing 0x{:02x}: {:?}", addr, e);
            false
        }
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
        AFIO,
        I2C1,
        ..
    } = dp;

    let mut board = Board::init(RCC, GPIOA, GPIOB, GPIOC);

    // I2C1 remap 2: SCL = PC16, SDA = PC17. The I2C peripheral switches them
    // to open-drain automatically, so configure them as alternate push-pull.
    let mut afio = Afio::init(AFIO, &mut board.rcc);
    afio.set_i2c1_remap(I2c1Remap::Remap2);

    let mut scl = board.pins.pc::<16>();
    let mut sda = board.pins.pc::<17>();
    scl.into_alt_push_pull();
    sda.into_alt_push_pull();

    let mut i2c = I2c1::init(I2C1, &mut board.rcc, board.clocks, Config::default());

    defmt::info!("I2C master scanning bus at 0x08..=0x77 ...");
    for addr in 0x08..=0x77u8 {
        if probe(&mut i2c, addr) {
            defmt::info!("device found at 0x{:02x}", addr);
        }
    }

    // Talk to the slave from the `i2c_slave` example: write a register address,
    // then read the reply back.
    const SLAVE_ADDR: u8 = 0x50;
    let mut reply = [0u8; 32];
    match i2c.try_write_read(SLAVE_ADDR, &[0x00], &mut reply) {
        Ok(n) => defmt::info!("slave replied with {} byte(s)", n),
        Err(Error::Acknowledge) => defmt::info!("slave 0x{:02x} not present", SLAVE_ADDR),
        Err(e) => defmt::warn!("write_read to 0x{:02x} failed: {:?}", SLAVE_ADDR, e),
    }

    loop {
        // Idle; the scan above already demonstrated the bus.
        core::hint::spin_loop();
    }
}
