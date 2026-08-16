//! Shared board initialization for the CH32X035 examples.
//!
//! Examples destructure the `Peripherals` they need and pass the common
//! ones to [`Board::init`]:
//!
//! ```ignore
//! let dp = pac::Peripherals::take().unwrap();
//! let pac::Peripherals { RCC, GPIOA, GPIOB, GPIOC, .. } = dp;
//! let board = Board::init(RCC, GPIOA, GPIOB, GPIOC);
//! ```

use ch32x035::{Clocks, Rcc, gpio::Pins, pac, rcc::Config};

/// Standard initialization for a CH32X035 application.
pub struct Board {
    /// RCC driver (owns the RCC peripheral)
    pub rcc: Rcc,
    /// System clocks (48 MHz HSI after reset)
    pub clocks: Clocks,
    /// GPIO pins
    pub pins: Pins,
}

impl Board {
    /// Build the board from the extracted peripherals: RCC clock setup and
    /// GPIO ports A/B/C initialization.
    pub fn init(
        rcc_periph: pac::RCC,
        gpioa: pac::GPIOA,
        gpiob: pac::GPIOB,
        gpioc: pac::GPIOC,
    ) -> Self {
        let mut rcc = Rcc::new(rcc_periph);
        let clocks = rcc.freeze(Config::default());
        let pins = Pins::init(gpioa, gpiob, gpioc, &mut rcc);

        Board { rcc, clocks, pins }
    }
}
