//! Reset and Clock Control (RCC)
//!
//! Provides:
//! - peripheral clock-enable / disable for the APB1/APB2/AHB buses
//! - peripheral reset helpers
//! - [`Clocks`], which tracks the current system clock frequency
//!
//! The CH32X035 boots from the internal HSI oscillator at **48 MHz** with no
//! PLL. The vendor SVD's `CFGR0` only exposes `MCO` and `HPRE`, so this HAL
//! does not (yet) configure the PLL; `sysclk_hz()` returns 48 MHz.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, rcc::{Apb2Periph, Clocks}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! rcc.apb2_enable(Apb2Periph::Usart1 | Apb2Periph::Iopa);
//! let clocks = Clocks::new(&rcc);
//! ```

use enumflags2::BitFlags;

use crate::pac;

/// APB2 bus peripherals (clock enable / reset bit positions)
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apb2Periph {
    /// Alternate function I/O clock
    Afio = 1 << 0,
    /// I/O port A clock
    Iopa = 1 << 2,
    /// I/O port B clock
    Iopb = 1 << 3,
    /// I/O port C clock
    Iopc = 1 << 4,
    /// ADC1 clock
    Adc1 = 1 << 9,
    /// TIM1 clock
    Tim1 = 1 << 11,
    /// SPI1 clock
    Spi1 = 1 << 12,
    /// USART1 clock
    Usart1 = 1 << 14,
}

/// APB1 bus peripherals (clock enable / reset bit positions)
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apb1Periph {
    /// TIM2 clock
    Tim2 = 1 << 0,
    /// TIM3 clock
    Tim3 = 1 << 1,
    /// Window watchdog clock
    Wwdg = 1 << 11,
    /// USART2 clock
    Usart2 = 1 << 17,
    /// USART3 clock
    Usart3 = 1 << 18,
    /// UART4 clock
    Uart4 = 1 << 19,
    /// I2C1 clock
    I2c1 = 1 << 21,
    /// Power control clock
    Pwr = 1 << 28,
}

/// AHB bus peripherals (clock enable bit positions)
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbPeriph {
    /// DMA1 clock
    Dma1 = 1 << 0,
    /// SRAM clock
    Sram = 1 << 2,
    /// USBFS clock
    Usbfs = 1 << 12,
    /// USBPD clock
    Usbpd = 1 << 17,
}

/// AHB bus reset bits (subset of [`AhbPeriph`] that have reset lines)
#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbReset {
    /// USBFS reset
    Usbfs = 1 << 12,
    /// PIOC reset
    Pioc = 1 << 13,
    /// USBPD reset
    Usbpd = 1 << 17,
}

/// Reset and Clock Control driver
pub struct Rcc {
    rcc: pac::RCC,
}

/// The system clock configuration.
#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    sysclk_hz: u32,
}

/// Internal high-speed oscillator frequency (48 MHz)
pub const HSI_HZ: u32 = 48_000_000;

impl Rcc {
    /// Create a new RCC driver, consuming the RCC peripheral.
    pub fn new(rcc: pac::RCC) -> Self {
        Self { rcc }
    }

    /// Get a reference to the raw RCC register block
    pub fn regs(&self) -> &pac::rcc::RegisterBlock {
        &self.rcc
    }

    /// Consume the driver and return the underlying peripheral
    pub fn release(self) -> pac::RCC {
        self.rcc
    }

    // =====================================================================
    // APB2
    // =====================================================================

    /// Enable clocks for the given APB2 peripherals
    pub fn apb2_enable<I: Into<BitFlags<Apb2Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb2pcenr
            .modify_bits(|v| v | periphs.into().bits());
    }

    /// Disable clocks for the given APB2 peripherals
    pub fn apb2_disable<I: Into<BitFlags<Apb2Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb2pcenr
            .modify_bits(|v| v & !periphs.into().bits());
    }

    /// Assert reset for the given APB2 peripherals
    pub fn apb2_reset<I: Into<BitFlags<Apb2Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb2prstr
            .modify_bits(|v| v | periphs.into().bits());
    }

    /// Release reset for the given APB2 peripherals
    pub fn apb2_clear_reset<I: Into<BitFlags<Apb2Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb2prstr
            .modify_bits(|v| v & !periphs.into().bits());
    }

    // =====================================================================
    // APB1
    // =====================================================================

    /// Enable clocks for the given APB1 peripherals
    pub fn apb1_enable<I: Into<BitFlags<Apb1Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb1pcenr
            .modify_bits(|v| v | periphs.into().bits());
    }

    /// Disable clocks for the given APB1 peripherals
    pub fn apb1_disable<I: Into<BitFlags<Apb1Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb1pcenr
            .modify_bits(|v| v & !periphs.into().bits());
    }

    /// Assert reset for the given APB1 peripherals
    pub fn apb1_reset<I: Into<BitFlags<Apb1Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb1prstr
            .modify_bits(|v| v | periphs.into().bits());
    }

    /// Release reset for the given APB1 peripherals
    pub fn apb1_clear_reset<I: Into<BitFlags<Apb1Periph>>>(&mut self, periphs: I) {
        self.rcc
            .apb1prstr
            .modify_bits(|v| v & !periphs.into().bits());
    }

    // =====================================================================
    // AHB
    // =====================================================================

    /// Enable clocks for the given AHB peripherals
    pub fn ahb_enable<I: Into<BitFlags<AhbPeriph>>>(&mut self, periphs: I) {
        self.rcc
            .ahbpcenr
            .modify_bits(|v| v | periphs.into().bits());
    }

    /// Disable clocks for the given AHB peripherals
    pub fn ahb_disable<I: Into<BitFlags<AhbPeriph>>>(&mut self, periphs: I) {
        self.rcc
            .ahbpcenr
            .modify_bits(|v| v & !periphs.into().bits());
    }

    /// Assert reset for the given AHB peripherals
    pub fn ahb_reset<I: Into<BitFlags<AhbReset>>>(&mut self, periphs: I) {
        self.rcc.ahbrstr.modify_bits(|v| v | periphs.into().bits());
    }

    /// Release reset for the given AHB peripherals
    pub fn ahb_clear_reset<I: Into<BitFlags<AhbReset>>>(&mut self, periphs: I) {
        self.rcc.ahbrstr.modify_bits(|v| v & !periphs.into().bits());
    }
}

impl Clocks {
    /// Create a `Clocks` snapshot from the current RCC configuration.
    ///
    /// On the CH32X035 the system clock comes from the 48 MHz HSI after
    /// reset, and the SVD does not expose PLL configuration, so this returns
    /// the HSI frequency.
    pub fn new(_rcc: &Rcc) -> Self {
        Self { sysclk_hz: HSI_HZ }
    }

    /// Create a `Clocks` with an explicit system clock frequency (Hz).
    ///
    /// Use this if you configure the PLL via raw register access.
    pub fn from_frequency(sysclk_hz: u32) -> Self {
        Self { sysclk_hz }
    }

    /// The system (core) clock frequency in Hz
    pub fn sysclk_hz(&self) -> u32 {
        self.sysclk_hz
    }

    /// The APB1 clock frequency in Hz (equal to SYSCLK on the CH32X035)
    pub fn apb1_hz(&self) -> u32 {
        self.sysclk_hz
    }

    /// The APB2 clock frequency in Hz (equal to SYSCLK on the CH32X035)
    pub fn apb2_hz(&self) -> u32 {
        self.sysclk_hz
    }
}
