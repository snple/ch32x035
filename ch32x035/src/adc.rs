//! ADC driver
//!
//! Single-conversion, software-triggered ADC for the CH32X035's ADC1 (APB2).
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, Clocks, adc::Adc};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = Clocks::new(&rcc);
//! let mut adc = Adc::init(dp.ADC, &mut rcc, clocks);
//! let value = adc.read_channel(0); // PA0
//! ```

use crate::pac;
use crate::rcc::{Clocks, Rcc};
use crate::reset::Resettable;

/// ADC1 driver (12-bit, single conversion)
pub struct Adc {
    adc: pac::ADC,
    #[allow(dead_code)]
    clocks: Clocks,
}

impl Adc {
    /// Create a new ADC driver, resetting and clock-enabling the peripheral.
    ///
    /// The ADC is powered on (`ADON`) but not yet started.
    pub fn init(adc: pac::ADC, rcc: &mut Rcc, clocks: Clocks) -> Self {
        adc.init(rcc);

        let a = Self { adc, clocks };

        // Power on the ADC (ADON = 1)
        a.adc.ctlr2.set_adon();

        a
    }

    /// Perform a single conversion on the given channel (0..=17).
    ///
    /// Blocks until the conversion completes and returns the 12-bit result.
    pub fn read_channel(&mut self, channel: u8) -> u16 {
        debug_assert!(channel <= 17, "invalid ADC channel");
        let channel = channel as u32;

        // Configure the sequence: one conversion of `channel` (SQ1)
        self.adc.rsqr3.write_sq1(channel);
        self.adc.rsqr1.write_l(0); // length = 1 conversion

        // Clear any pending EOC flag
        self.adc.statr.clr_eoc();

        // Start a software-triggered conversion
        self.adc.ctlr2.set_swstart();

        // Wait for end of conversion
        while !self.adc.statr.eoc() {}

        // Read the 12-bit result
        (self.adc.rdatar_dr_act_dcg.read_bits() & 0x0FFF) as u16
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::ADC {
        self.adc
    }
}
