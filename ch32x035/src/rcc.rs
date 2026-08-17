//! Reset and Clock Control (RCC)
//!
//! Provides:
//! - peripheral clock-enable / disable for the APB1/APB2/AHB buses
//! - peripheral reset helpers
//! - clock-tree configuration (HSI trim, AHB prescaler, MCO output)
//! - [`Clocks`], which tracks the current system clock frequency
//!
//! The CH32X035 has a minimal clock tree: a fixed **48 MHz** HSI oscillator
//! feeds an AHB prescaler (`HPRE` in `CFGR0`) that produces the core / HCLK /
//! PCLK1 / PCLK2 clocks. There is no PLL and no external-crystal option, so
//! the prescaler is the only frequency control available. After reset the
//! prescaler is `Div1`, so the system runs at 48 MHz.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, rcc::{Apb2Periph, Config}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = rcc.freeze(Config::default());
//! rcc.apb2_enable(Apb2Periph::Usart1 | Apb2Periph::Iopa);
//! ```

use enumflags2::BitFlags;

use crate::Steal;
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

/// AHB prescaler (`HPRE`) values.
///
/// Divides the 48 MHz HSI clock to produce the core / HCLK / PCLK1 / PCLK2
/// clocks. This prescaler is the only frequency knob in the CH32X035 clock
/// tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HclkPrescaler {
    /// SYSCLK not divided (48 MHz)
    Div1,
    /// SYSCLK divided by 2 (24 MHz)
    Div2,
    /// SYSCLK divided by 3 (16 MHz)
    Div3,
    /// SYSCLK divided by 4 (12 MHz)
    Div4,
    /// SYSCLK divided by 5 (9.6 MHz)
    Div5,
    /// SYSCLK divided by 6 (8 MHz)
    Div6,
    /// SYSCLK divided by 7
    Div7,
    /// SYSCLK divided by 8 (6 MHz)
    Div8,
    /// SYSCLK divided by 16 (3 MHz)
    Div16,
    /// SYSCLK divided by 32 (1.5 MHz)
    Div32,
    /// SYSCLK divided by 64
    Div64,
    /// SYSCLK divided by 128
    Div128,
    /// SYSCLK divided by 256
    Div256,
}

impl HclkPrescaler {
    /// Raw `HPRE` field value written to `CFGR0`.
    pub const fn bits(self) -> u32 {
        match self {
            Self::Div1 => 0b0000,
            Self::Div2 => 0b0001,
            Self::Div3 => 0b0010,
            Self::Div4 => 0b0011,
            Self::Div5 => 0b0100,
            Self::Div6 => 0b0101,
            Self::Div7 => 0b0110,
            Self::Div8 => 0b0111,
            Self::Div16 => 0b1011,
            Self::Div32 => 0b1100,
            Self::Div64 => 0b1101,
            Self::Div128 => 0b1110,
            Self::Div256 => 0b1111,
        }
    }

    /// Clock divisor as an integer.
    pub const fn divisor(self) -> u32 {
        match self {
            Self::Div1 => 1,
            Self::Div2 => 2,
            Self::Div3 => 3,
            Self::Div4 => 4,
            Self::Div5 => 5,
            Self::Div6 => 6,
            Self::Div7 => 7,
            Self::Div8 => 8,
            Self::Div16 => 16,
            Self::Div32 => 32,
            Self::Div64 => 64,
            Self::Div128 => 128,
            Self::Div256 => 256,
        }
    }

    /// Decode a raw `HPRE` field value (4 bits).
    ///
    /// Encodings `0b1000`..`0b1010` are reserved but alias the power-of-two
    /// prescalers; they are mapped to their effective divisor so that reading
    /// back the register always yields a correct frequency.
    pub const fn from_bits(bits: u32) -> Self {
        match bits & 0xF {
            0b0000 => Self::Div1,
            0b0001 => Self::Div2,
            0b0010 => Self::Div3,
            0b0011 => Self::Div4,
            0b0100 => Self::Div5,
            0b0101 => Self::Div6,
            0b0110 => Self::Div7,
            0b0111 => Self::Div8,
            0b1000 => Self::Div2,
            0b1001 => Self::Div4,
            0b1010 => Self::Div8,
            0b1011 => Self::Div16,
            0b1100 => Self::Div32,
            0b1101 => Self::Div64,
            0b1110 => Self::Div128,
            _ => Self::Div256,
        }
    }
}

/// Microcontroller clock output (`MCO`) source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McoSource {
    /// No clock output.
    NoClock,
    /// System clock (after the AHB prescaler).
    SysClk,
    /// 48 MHz HSI oscillator.
    Hsi,
}

impl McoSource {
    /// Raw `MCO` field value written to `CFGR0`.
    pub const fn bits(self) -> u32 {
        match self {
            Self::NoClock => 0b000,
            Self::SysClk => 0b100,
            Self::Hsi => 0b101,
        }
    }
}

/// Clock-tree configuration for [`Rcc::freeze`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// AHB prescaler dividing the 48 MHz HSI clock.
    pub hclk: HclkPrescaler,
    /// Optional HSI trim value (`HSITRIM`, 5 bits). `None` keeps the factory
    /// calibration.
    pub hsi_trim: Option<u8>,
    /// Optional `MCO` output source. `None` leaves the output unchanged.
    pub mco: Option<McoSource>,
}

impl Default for Config {
    /// Run at the full 48 MHz HSI with factory HSI trim and no MCO output.
    fn default() -> Self {
        Self {
            hclk: HclkPrescaler::Div1,
            hsi_trim: None,
            mco: None,
        }
    }
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

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle to `RCC` is live.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            rcc: unsafe { pac::RCC::steal() },
        })
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
        self.rcc.ahbpcenr.modify_bits(|v| v | periphs.into().bits());
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

    // =====================================================================
    // Clock tree
    // =====================================================================

    /// Apply a clock-tree configuration and return the resulting [`Clocks`].
    ///
    /// Configures the HSI trim, the AHB prescaler (`HPRE`) and the MCO output
    /// as requested. The CH32X035 has no PLL or clock-source switch, so the
    /// prescaler is the only frequency control available.
    pub fn freeze(&mut self, config: Config) -> Clocks {
        if let Some(trim) = config.hsi_trim {
            self.rcc.ctlr.write_hsitrim(u32::from(trim) & 0x1F);
        }
        if let Some(mco) = config.mco {
            self.rcc.cfgr0.write_mco(mco.bits());
        }
        self.rcc.cfgr0.write_hpre(config.hclk.bits());

        Clocks::new(self)
    }

    /// Read the currently configured AHB prescaler.
    pub fn hclk_prescaler(&self) -> HclkPrescaler {
        HclkPrescaler::from_bits(self.rcc.cfgr0.hpre())
    }
}

impl Clocks {
    /// Create a `Clocks` snapshot by reading the current RCC configuration.
    ///
    /// The system clock is the 48 MHz HSI divided by the AHB prescaler
    /// (`HPRE`). `APB1` and `APB2` run at the same frequency (the CH32X035
    /// has no APB prescalers).
    pub fn new(rcc: &Rcc) -> Self {
        Self {
            sysclk_hz: HSI_HZ / rcc.hclk_prescaler().divisor(),
        }
    }

    /// Reconstruct the current clock configuration without an `Rcc` handle.
    ///
    /// Equivalent to [`Clocks::new`] reading the current RCC state.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle to `RCC` is live.
    #[inline]
    pub unsafe fn steal() -> Self {
        let rcc = unsafe { pac::RCC::steal() };
        let prescaler = HclkPrescaler::from_bits(rcc.cfgr0.hpre());
        Self {
            sysclk_hz: HSI_HZ / prescaler.divisor(),
        }
    }

    /// Create a `Clocks` with an explicit system clock frequency (Hz).
    ///
    /// Use this if you configure the prescaler via raw register access.
    pub fn from_frequency(sysclk_hz: u32) -> Self {
        Self { sysclk_hz }
    }

    /// The system (core) clock frequency in Hz
    pub fn sysclk_hz(&self) -> u32 {
        self.sysclk_hz
    }

    /// The AHB (HCLK) clock frequency in Hz (equal to SYSCLK on the CH32X035)
    pub fn hclk_hz(&self) -> u32 {
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
