//! I2C1 interrupt-driven slave example.
//!
//! Configures the CH32X035 as an I2C slave at address `0x50` and handles
//! master read/write transactions entirely from the `I2C1_EV_IRQ` /
//! `I2C1_ER_IRQ` interrupts. The I2C1 pins are **PB6** (SCL) and **PB7**
//! (SDA), configured as alternate-function open-drain; connect external
//! pull-ups (e.g. 4.7 kΩ to 3.3 V).
//!
//! With a second board running as I2C master (or a logic analyzer):
//! - master writes N bytes → they land in `RX[..N]` and `write_ready()`
//!   goes high;
//! - master reads → the slave transmits the bytes passed to `respond()`.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin i2c_slave --release
//! ```

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::i2c::I2c1SlaveIrq;
use ch32x035::{Interrupt, bind, interrupt, main, pac};
use ch32x035_examples::board::Board;

/// Receive buffer for bytes written by the master.
static mut RX: [u8; 32] = [0; 32];

/// The interrupt-driven I2C slave instance.
static I2C: I2c1SlaveIrq = I2c1SlaveIrq::new();

/// I2C1 event interrupt: address match, data, stop handling.
#[bind(I2C1_EV_IRQ)]
fn i2c1_ev() {
    I2C.on_event();
}

/// I2C1 error interrupt: NACK and bus-error handling.
#[bind(I2C1_ER_IRQ)]
fn i2c1_er() {
    I2C.on_error();
}

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

    // SAFETY: RX stays alive for the whole program and is only written by
    // the ISRs (under the critical section) and read by the main loop after
    // `write_ready()` is set, so there is no data race.
    unsafe {
        let rx: &'static mut [u8] = &mut *core::ptr::addr_of_mut!(RX);
        I2C.init(I2C1, &mut board.rcc, board.clocks, 0x50, rx);
    }

    interrupt::enable(Interrupt::I2C1_EV_IRQ);
    interrupt::enable(Interrupt::I2C1_ER_IRQ);

    loop {
        // Master is requesting a read: provide the response data.
        if I2C.read_requested() {
            I2C.respond(b"hello from ch32x035 slave!");
        }

        // Master finished a write: process the received bytes.
        if I2C.write_ready() {
            let n = I2C.rx_len();
            // `RX[..n]` holds the bytes the master wrote. Process them here.
            let _ = n;
            I2C.clear_write_ready();
        }

        // Recover from bus errors.
        if I2C.has_error() {
            I2C.clear_error();
        }
    }
}
