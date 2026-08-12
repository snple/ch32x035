//! Peripheral reset helpers
//!
//! Provides the [`Resettable`] trait, implemented for the PAC peripheral
//! types. Calling `init()` on a peripheral resets it and enables its clock,
//! which is the standard first step before configuring any peripheral
//! (mirrors the pico-rs `Resettable` trait).
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, reset::Resettable};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let uart = dp.USART1;
//! uart.init(&mut rcc); // reset + enable clock
//! ```

use crate::pac;
use crate::rcc::{AhbPeriph, Apb1Periph, Apb2Periph, Rcc};

/// A peripheral that can be reset and clock-enabled via the RCC.
pub trait Resettable {
    /// Enable the peripheral's clock
    fn enable_clock(&self, rcc: &mut Rcc);

    /// Assert and release the peripheral reset
    fn reset(&self, rcc: &mut Rcc);

    /// Reset the peripheral and enable its clock
    fn init(&self, rcc: &mut Rcc) {
        self.reset(rcc);
        self.enable_clock(rcc);
    }
}

/// Implement `Resettable` for an APB2 peripheral
macro_rules! impl_apb2 {
    ($periph:ty, $variant:ident) => {
        impl Resettable for $periph {
            fn enable_clock(&self, rcc: &mut Rcc) {
                rcc.apb2_enable(Apb2Periph::$variant);
            }
            fn reset(&self, rcc: &mut Rcc) {
                rcc.apb2_reset(Apb2Periph::$variant);
                rcc.apb2_clear_reset(Apb2Periph::$variant);
            }
        }
    };
}

/// Implement `Resettable` for an APB1 peripheral
macro_rules! impl_apb1 {
    ($periph:ty, $variant:ident) => {
        impl Resettable for $periph {
            fn enable_clock(&self, rcc: &mut Rcc) {
                rcc.apb1_enable(Apb1Periph::$variant);
            }
            fn reset(&self, rcc: &mut Rcc) {
                rcc.apb1_reset(Apb1Periph::$variant);
                rcc.apb1_clear_reset(Apb1Periph::$variant);
            }
        }
    };
}

/// Implement `Resettable` for an AHB peripheral
macro_rules! impl_ahb {
    ($periph:ty, $variant:ident) => {
        impl Resettable for $periph {
            fn enable_clock(&self, rcc: &mut Rcc) {
                rcc.ahb_enable(AhbPeriph::$variant);
            }
            fn reset(&self, rcc: &mut Rcc) {
                rcc.ahb_reset(crate::rcc::AhbReset::$variant);
                rcc.ahb_clear_reset(crate::rcc::AhbReset::$variant);
            }
        }
    };
}

// APB2
impl_apb2!(pac::AFIO, Afio);
impl_apb2!(pac::GPIOA, Iopa);
impl_apb2!(pac::GPIOB, Iopb);
impl_apb2!(pac::GPIOC, Iopc);
impl_apb2!(pac::ADC, Adc1);
impl_apb2!(pac::TIM1, Tim1);
impl_apb2!(pac::SPI1, Spi1);
impl_apb2!(pac::USART1, Usart1);

// APB1
impl_apb1!(pac::TIM2, Tim2);
impl_apb1!(pac::TIM3, Tim3);
impl_apb1!(pac::WWDG, Wwdg);
impl_apb1!(pac::USART2, Usart2);
impl_apb1!(pac::USART3, Usart3);
impl_apb1!(pac::UART4, Uart4);
impl_apb1!(pac::I2C1, I2c1);
impl_apb1!(pac::PWR, Pwr);

// AHB
impl_ahb!(pac::USBFS, Usbfs);

/// DMA1 has no reset line in the CH32X035 (AHB reset register has none), so
/// only the clock is enabled.
impl Resettable for pac::DMA {
    fn enable_clock(&self, rcc: &mut Rcc) {
        rcc.ahb_enable(AhbPeriph::Dma1);
    }
    fn reset(&self, _rcc: &mut Rcc) {
        // no reset line available
    }
}
