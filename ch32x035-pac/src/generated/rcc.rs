//! RCC
//! Reset and clock control

#![allow(dead_code)]

pub const BASE: u32 = 0x40021000;

/// CTLR Register
///
/// Clock control register
///
/// Address offset: 0x0000
/// Reset value: 0x00000083
///
/// ## Fields
///
/// - **HSICAL** `[15:8]` (RO)
///
///   Internal High Speed clock
///   Calibration
///
/// - **HSITRIM** `[7:3]` (RW)
///
///   Internal High Speed clock
///   trimming
///
/// - **HSIRDY** `[1]` (RO)
///
///   Internal High Speed clock ready
///   flag
///
/// - **HSION** `[0]` (RW)
///
///   Internal High Speed clock
///   enable
#[repr(transparent)]
pub struct Ctlr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctlr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctlr {
    // ======== Field Constants ========

    /// **HSICAL**
    ///
    /// Internal High Speed clock
    /// Calibration
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const HSICAL_OFFSET: u32 = 8;
    pub const HSICAL_MASK: u32 = 0x0000ff00;

    /// **HSITRIM**
    ///
    /// Internal High Speed clock
    /// trimming
    ///
    /// Bits `[7:3]`, mask `0x000000f8`
    pub const HSITRIM_OFFSET: u32 = 3;
    pub const HSITRIM_MASK: u32 = 0x000000f8;

    /// **HSIRDY**
    ///
    /// Internal High Speed clock ready
    /// flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HSIRDY_OFFSET: u32 = 1;
    pub const HSIRDY_MASK: u32 = 0x00000002;
    pub const HSIRDY: u32 = 1 << 1;

    /// **HSION**
    ///
    /// Internal High Speed clock
    /// enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HSION_OFFSET: u32 = 0;
    pub const HSION_MASK: u32 = 0x00000001;
    pub const HSION: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **HSICAL** `[15:8]` (RO)
    ///
    /// Internal High Speed clock
    /// Calibration
    #[inline(always)]
    pub fn hsical(&self) -> u32 {
        (self.value.read_bits() & Self::HSICAL_MASK) >> Self::HSICAL_OFFSET
    }

    /// **HSITRIM** `[7:3]` (RW)
    ///
    /// Internal High Speed clock
    /// trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> u32 {
        (self.value.read_bits() & Self::HSITRIM_MASK) >> Self::HSITRIM_OFFSET
    }

    /// Write **HSITRIM** `[7:3]` (RW)
    ///
    /// Internal High Speed clock
    /// trimming
    #[inline(always)]
    pub fn write_hsitrim(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HSITRIM_MASK) | ((value << Self::HSITRIM_OFFSET) & Self::HSITRIM_MASK)
        });
    }

    /// **HSIRDY** `[1]` (RO)
    ///
    /// Internal High Speed clock ready
    /// flag
    #[inline(always)]
    pub fn hsirdy(&self) -> bool {
        (self.value.read_bits() & Self::HSIRDY_MASK) != 0
    }

    /// **HSION** `[0]` (RW)
    ///
    /// Internal High Speed clock
    /// enable
    #[inline(always)]
    pub fn hsion(&self) -> bool {
        (self.value.read_bits() & Self::HSION_MASK) != 0
    }

    /// Set HSION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hsion(&self) {
        self.value.modify_bits(|r| r | Self::HSION_MASK);
    }

    /// Clear HSION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hsion(&self) {
        self.value.modify_bits(|r| r & !Self::HSION_MASK);
    }
}

/// CTLR value type for batch read/modify/write operations
///
/// Use `ctlr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctlr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtlrVal {
    bits: u32,
}

impl CtlrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000083 }
    }

    /// Read **HSICAL** `[15:8]`
    ///
    /// Internal High Speed clock
    /// Calibration
    #[inline(always)]
    pub const fn hsical(&self) -> u32 {
        (self.bits & Ctlr::HSICAL_MASK) >> Ctlr::HSICAL_OFFSET
    }

    /// Read **HSITRIM** `[7:3]`
    ///
    /// Internal High Speed clock
    /// trimming
    #[inline(always)]
    pub const fn hsitrim(&self) -> u32 {
        (self.bits & Ctlr::HSITRIM_MASK) >> Ctlr::HSITRIM_OFFSET
    }

    /// Read **HSIRDY** `[1:1]`
    ///
    /// Internal High Speed clock ready
    /// flag
    #[inline(always)]
    pub const fn hsirdy(&self) -> bool {
        (self.bits & Ctlr::HSIRDY_MASK) != 0
    }

    /// Read **HSION** `[0:0]`
    ///
    /// Internal High Speed clock
    /// enable
    #[inline(always)]
    pub const fn hsion(&self) -> bool {
        (self.bits & Ctlr::HSION_MASK) != 0
    }

    /// Set **HSITRIM** `[7:3]`
    ///
    /// Internal High Speed clock
    /// trimming
    #[inline(always)]
    pub const fn with_hsitrim(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::HSITRIM_MASK)
                | ((value << Ctlr::HSITRIM_OFFSET) & Ctlr::HSITRIM_MASK),
        }
    }

    /// Set **HSION** `[0:0]`
    ///
    /// Internal High Speed clock
    /// enable
    #[inline(always)]
    pub const fn with_hsion(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::HSION_MASK
            } else {
                self.bits & !Ctlr::HSION_MASK
            },
        }
    }
}

impl Default for CtlrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctlr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtlrVal {
        CtlrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtlrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtlrVal) -> CtlrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR0 Register
///
/// Clock configuration register
/// (RCC_CFGR0)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MCO** `[26:24]` (RW)
///
///   Microcontroller clock
///   output
///
/// - **HPRE** `[7:4]` (RW)
///
///   AHB prescaler
#[repr(transparent)]
pub struct Cfgr0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr0 {
    // ======== Field Constants ========

    /// **MCO**
    ///
    /// Microcontroller clock
    /// output
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const MCO_OFFSET: u32 = 24;
    pub const MCO_MASK: u32 = 0x07000000;

    /// **HPRE**
    ///
    /// AHB prescaler
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const HPRE_OFFSET: u32 = 4;
    pub const HPRE_MASK: u32 = 0x000000f0;

    // ======== Field Accessors ========

    /// **MCO** `[26:24]` (RW)
    ///
    /// Microcontroller clock
    /// output
    #[inline(always)]
    pub fn mco(&self) -> u32 {
        (self.value.read_bits() & Self::MCO_MASK) >> Self::MCO_OFFSET
    }

    /// Write **MCO** `[26:24]` (RW)
    ///
    /// Microcontroller clock
    /// output
    #[inline(always)]
    pub fn write_mco(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MCO_MASK) | ((value << Self::MCO_OFFSET) & Self::MCO_MASK)
        });
    }

    /// **HPRE** `[7:4]` (RW)
    ///
    /// AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> u32 {
        (self.value.read_bits() & Self::HPRE_MASK) >> Self::HPRE_OFFSET
    }

    /// Write **HPRE** `[7:4]` (RW)
    ///
    /// AHB prescaler
    #[inline(always)]
    pub fn write_hpre(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HPRE_MASK) | ((value << Self::HPRE_OFFSET) & Self::HPRE_MASK)
        });
    }
}

/// CFGR0 value type for batch read/modify/write operations
///
/// Use `cfgr0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr0Val {
    bits: u32,
}

impl Cfgr0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **MCO** `[26:24]`
    ///
    /// Microcontroller clock
    /// output
    #[inline(always)]
    pub const fn mco(&self) -> u32 {
        (self.bits & Cfgr0::MCO_MASK) >> Cfgr0::MCO_OFFSET
    }

    /// Read **HPRE** `[7:4]`
    ///
    /// AHB prescaler
    #[inline(always)]
    pub const fn hpre(&self) -> u32 {
        (self.bits & Cfgr0::HPRE_MASK) >> Cfgr0::HPRE_OFFSET
    }

    /// Set **MCO** `[26:24]`
    ///
    /// Microcontroller clock
    /// output
    #[inline(always)]
    pub const fn with_mco(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr0::MCO_MASK) | ((value << Cfgr0::MCO_OFFSET) & Cfgr0::MCO_MASK),
        }
    }

    /// Set **HPRE** `[7:4]`
    ///
    /// AHB prescaler
    #[inline(always)]
    pub const fn with_hpre(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr0::HPRE_MASK)
                | ((value << Cfgr0::HPRE_OFFSET) & Cfgr0::HPRE_MASK),
        }
    }
}

impl Default for Cfgr0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr0Val {
        Cfgr0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr0Val) -> Cfgr0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// APB2PRSTR Register
///
/// APB2 peripheral reset register
/// (RCC_APB2PRSTR)
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USART1RST** `[14]` (RW)
///
///   USART1 reset
///
/// - **SPI1RST** `[12]` (RW)
///
///   SPI 1 reset
///
/// - **TIM1RST** `[11]` (RW)
///
///   TIM1 timer reset
///
/// - **ADC1RST** `[9]` (RW)
///
///   ADC 1 interface reset
///
/// - **IOPCRST** `[4]` (RW)
///
///   IO port C reset
///
/// - **IOPBRST** `[3]` (RW)
///
///   IO port B reset
///
/// - **IOPARST** `[2]` (RW)
///
///   IO port A reset
///
/// - **AFIORST** `[0]` (RW)
///
///   Alternate function I/O
///   reset
#[repr(transparent)]
pub struct Apb2prstr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2prstr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2prstr {
    // ======== Field Constants ========

    /// **USART1RST**
    ///
    /// USART1 reset
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USART1RST_OFFSET: u32 = 14;
    pub const USART1RST_MASK: u32 = 0x00004000;
    pub const USART1RST: u32 = 1 << 14;

    /// **SPI1RST**
    ///
    /// SPI 1 reset
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI1RST_OFFSET: u32 = 12;
    pub const SPI1RST_MASK: u32 = 0x00001000;
    pub const SPI1RST: u32 = 1 << 12;

    /// **TIM1RST**
    ///
    /// TIM1 timer reset
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TIM1RST_OFFSET: u32 = 11;
    pub const TIM1RST_MASK: u32 = 0x00000800;
    pub const TIM1RST: u32 = 1 << 11;

    /// **ADC1RST**
    ///
    /// ADC 1 interface reset
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ADC1RST_OFFSET: u32 = 9;
    pub const ADC1RST_MASK: u32 = 0x00000200;
    pub const ADC1RST: u32 = 1 << 9;

    /// **IOPCRST**
    ///
    /// IO port C reset
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IOPCRST_OFFSET: u32 = 4;
    pub const IOPCRST_MASK: u32 = 0x00000010;
    pub const IOPCRST: u32 = 1 << 4;

    /// **IOPBRST**
    ///
    /// IO port B reset
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IOPBRST_OFFSET: u32 = 3;
    pub const IOPBRST_MASK: u32 = 0x00000008;
    pub const IOPBRST: u32 = 1 << 3;

    /// **IOPARST**
    ///
    /// IO port A reset
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IOPARST_OFFSET: u32 = 2;
    pub const IOPARST_MASK: u32 = 0x00000004;
    pub const IOPARST: u32 = 1 << 2;

    /// **AFIORST**
    ///
    /// Alternate function I/O
    /// reset
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const AFIORST_OFFSET: u32 = 0;
    pub const AFIORST_MASK: u32 = 0x00000001;
    pub const AFIORST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USART1RST** `[14]` (RW)
    ///
    /// USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> bool {
        (self.value.read_bits() & Self::USART1RST_MASK) != 0
    }

    /// Set USART1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart1rst(&self) {
        self.value.modify_bits(|r| r | Self::USART1RST_MASK);
    }

    /// Clear USART1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart1rst(&self) {
        self.value.modify_bits(|r| r & !Self::USART1RST_MASK);
    }

    /// **SPI1RST** `[12]` (RW)
    ///
    /// SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> bool {
        (self.value.read_bits() & Self::SPI1RST_MASK) != 0
    }

    /// Set SPI1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi1rst(&self) {
        self.value.modify_bits(|r| r | Self::SPI1RST_MASK);
    }

    /// Clear SPI1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi1rst(&self) {
        self.value.modify_bits(|r| r & !Self::SPI1RST_MASK);
    }

    /// **TIM1RST** `[11]` (RW)
    ///
    /// TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> bool {
        (self.value.read_bits() & Self::TIM1RST_MASK) != 0
    }

    /// Set TIM1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim1rst(&self) {
        self.value.modify_bits(|r| r | Self::TIM1RST_MASK);
    }

    /// Clear TIM1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim1rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIM1RST_MASK);
    }

    /// **ADC1RST** `[9]` (RW)
    ///
    /// ADC 1 interface reset
    #[inline(always)]
    pub fn adc1rst(&self) -> bool {
        (self.value.read_bits() & Self::ADC1RST_MASK) != 0
    }

    /// Set ADC1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_adc1rst(&self) {
        self.value.modify_bits(|r| r | Self::ADC1RST_MASK);
    }

    /// Clear ADC1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_adc1rst(&self) {
        self.value.modify_bits(|r| r & !Self::ADC1RST_MASK);
    }

    /// **IOPCRST** `[4]` (RW)
    ///
    /// IO port C reset
    #[inline(always)]
    pub fn iopcrst(&self) -> bool {
        (self.value.read_bits() & Self::IOPCRST_MASK) != 0
    }

    /// Set IOPCRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iopcrst(&self) {
        self.value.modify_bits(|r| r | Self::IOPCRST_MASK);
    }

    /// Clear IOPCRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iopcrst(&self) {
        self.value.modify_bits(|r| r & !Self::IOPCRST_MASK);
    }

    /// **IOPBRST** `[3]` (RW)
    ///
    /// IO port B reset
    #[inline(always)]
    pub fn iopbrst(&self) -> bool {
        (self.value.read_bits() & Self::IOPBRST_MASK) != 0
    }

    /// Set IOPBRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iopbrst(&self) {
        self.value.modify_bits(|r| r | Self::IOPBRST_MASK);
    }

    /// Clear IOPBRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iopbrst(&self) {
        self.value.modify_bits(|r| r & !Self::IOPBRST_MASK);
    }

    /// **IOPARST** `[2]` (RW)
    ///
    /// IO port A reset
    #[inline(always)]
    pub fn ioparst(&self) -> bool {
        (self.value.read_bits() & Self::IOPARST_MASK) != 0
    }

    /// Set IOPARST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ioparst(&self) {
        self.value.modify_bits(|r| r | Self::IOPARST_MASK);
    }

    /// Clear IOPARST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ioparst(&self) {
        self.value.modify_bits(|r| r & !Self::IOPARST_MASK);
    }

    /// **AFIORST** `[0]` (RW)
    ///
    /// Alternate function I/O
    /// reset
    #[inline(always)]
    pub fn afiorst(&self) -> bool {
        (self.value.read_bits() & Self::AFIORST_MASK) != 0
    }

    /// Set AFIORST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_afiorst(&self) {
        self.value.modify_bits(|r| r | Self::AFIORST_MASK);
    }

    /// Clear AFIORST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_afiorst(&self) {
        self.value.modify_bits(|r| r & !Self::AFIORST_MASK);
    }
}

/// APB2PRSTR value type for batch read/modify/write operations
///
/// Use `apb2prstr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2prstr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2prstrVal {
    bits: u32,
}

impl Apb2prstrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **USART1RST** `[14:14]`
    ///
    /// USART1 reset
    #[inline(always)]
    pub const fn usart1rst(&self) -> bool {
        (self.bits & Apb2prstr::USART1RST_MASK) != 0
    }

    /// Read **SPI1RST** `[12:12]`
    ///
    /// SPI 1 reset
    #[inline(always)]
    pub const fn spi1rst(&self) -> bool {
        (self.bits & Apb2prstr::SPI1RST_MASK) != 0
    }

    /// Read **TIM1RST** `[11:11]`
    ///
    /// TIM1 timer reset
    #[inline(always)]
    pub const fn tim1rst(&self) -> bool {
        (self.bits & Apb2prstr::TIM1RST_MASK) != 0
    }

    /// Read **ADC1RST** `[9:9]`
    ///
    /// ADC 1 interface reset
    #[inline(always)]
    pub const fn adc1rst(&self) -> bool {
        (self.bits & Apb2prstr::ADC1RST_MASK) != 0
    }

    /// Read **IOPCRST** `[4:4]`
    ///
    /// IO port C reset
    #[inline(always)]
    pub const fn iopcrst(&self) -> bool {
        (self.bits & Apb2prstr::IOPCRST_MASK) != 0
    }

    /// Read **IOPBRST** `[3:3]`
    ///
    /// IO port B reset
    #[inline(always)]
    pub const fn iopbrst(&self) -> bool {
        (self.bits & Apb2prstr::IOPBRST_MASK) != 0
    }

    /// Read **IOPARST** `[2:2]`
    ///
    /// IO port A reset
    #[inline(always)]
    pub const fn ioparst(&self) -> bool {
        (self.bits & Apb2prstr::IOPARST_MASK) != 0
    }

    /// Read **AFIORST** `[0:0]`
    ///
    /// Alternate function I/O
    /// reset
    #[inline(always)]
    pub const fn afiorst(&self) -> bool {
        (self.bits & Apb2prstr::AFIORST_MASK) != 0
    }

    /// Set **USART1RST** `[14:14]`
    ///
    /// USART1 reset
    #[inline(always)]
    pub const fn with_usart1rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::USART1RST_MASK
            } else {
                self.bits & !Apb2prstr::USART1RST_MASK
            },
        }
    }

    /// Set **SPI1RST** `[12:12]`
    ///
    /// SPI 1 reset
    #[inline(always)]
    pub const fn with_spi1rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::SPI1RST_MASK
            } else {
                self.bits & !Apb2prstr::SPI1RST_MASK
            },
        }
    }

    /// Set **TIM1RST** `[11:11]`
    ///
    /// TIM1 timer reset
    #[inline(always)]
    pub const fn with_tim1rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::TIM1RST_MASK
            } else {
                self.bits & !Apb2prstr::TIM1RST_MASK
            },
        }
    }

    /// Set **ADC1RST** `[9:9]`
    ///
    /// ADC 1 interface reset
    #[inline(always)]
    pub const fn with_adc1rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::ADC1RST_MASK
            } else {
                self.bits & !Apb2prstr::ADC1RST_MASK
            },
        }
    }

    /// Set **IOPCRST** `[4:4]`
    ///
    /// IO port C reset
    #[inline(always)]
    pub const fn with_iopcrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::IOPCRST_MASK
            } else {
                self.bits & !Apb2prstr::IOPCRST_MASK
            },
        }
    }

    /// Set **IOPBRST** `[3:3]`
    ///
    /// IO port B reset
    #[inline(always)]
    pub const fn with_iopbrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::IOPBRST_MASK
            } else {
                self.bits & !Apb2prstr::IOPBRST_MASK
            },
        }
    }

    /// Set **IOPARST** `[2:2]`
    ///
    /// IO port A reset
    #[inline(always)]
    pub const fn with_ioparst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::IOPARST_MASK
            } else {
                self.bits & !Apb2prstr::IOPARST_MASK
            },
        }
    }

    /// Set **AFIORST** `[0:0]`
    ///
    /// Alternate function I/O
    /// reset
    #[inline(always)]
    pub const fn with_afiorst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2prstr::AFIORST_MASK
            } else {
                self.bits & !Apb2prstr::AFIORST_MASK
            },
        }
    }
}

impl Default for Apb2prstrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Apb2prstr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2prstrVal {
        Apb2prstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Apb2prstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Apb2prstrVal) -> Apb2prstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// APB1PRSTR Register
///
/// APB1 peripheral reset register
/// (RCC_APB1PRSTR)
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWRRST** `[28]` (RW)
///
///   Power interface reset
///
/// - **I2C1RST** `[21]` (RW)
///
///   I2C1 reset
///
/// - **USART4RST** `[19]` (RW)
///
///   USART 4 reset
///
/// - **USART3RST** `[18]` (RW)
///
///   USART 3 reset
///
/// - **USART2RST** `[17]` (RW)
///
///   USART 2 reset
///
/// - **WWDGRST** `[11]` (RW)
///
///   Window watchdog reset
///
/// - **TIM3RST** `[1]` (RW)
///
///   Timer 3 reset
///
/// - **TIM2RST** `[0]` (RW)
///
///   Timer 2 reset
#[repr(transparent)]
pub struct Apb1prstr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb1prstr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb1prstr {
    // ======== Field Constants ========

    /// **PWRRST**
    ///
    /// Power interface reset
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const PWRRST_OFFSET: u32 = 28;
    pub const PWRRST_MASK: u32 = 0x10000000;
    pub const PWRRST: u32 = 1 << 28;

    /// **I2C1RST**
    ///
    /// I2C1 reset
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const I2C1RST_OFFSET: u32 = 21;
    pub const I2C1RST_MASK: u32 = 0x00200000;
    pub const I2C1RST: u32 = 1 << 21;

    /// **USART4RST**
    ///
    /// USART 4 reset
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const USART4RST_OFFSET: u32 = 19;
    pub const USART4RST_MASK: u32 = 0x00080000;
    pub const USART4RST: u32 = 1 << 19;

    /// **USART3RST**
    ///
    /// USART 3 reset
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const USART3RST_OFFSET: u32 = 18;
    pub const USART3RST_MASK: u32 = 0x00040000;
    pub const USART3RST: u32 = 1 << 18;

    /// **USART2RST**
    ///
    /// USART 2 reset
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const USART2RST_OFFSET: u32 = 17;
    pub const USART2RST_MASK: u32 = 0x00020000;
    pub const USART2RST: u32 = 1 << 17;

    /// **WWDGRST**
    ///
    /// Window watchdog reset
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const WWDGRST_OFFSET: u32 = 11;
    pub const WWDGRST_MASK: u32 = 0x00000800;
    pub const WWDGRST: u32 = 1 << 11;

    /// **TIM3RST**
    ///
    /// Timer 3 reset
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIM3RST_OFFSET: u32 = 1;
    pub const TIM3RST_MASK: u32 = 0x00000002;
    pub const TIM3RST: u32 = 1 << 1;

    /// **TIM2RST**
    ///
    /// Timer 2 reset
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIM2RST_OFFSET: u32 = 0;
    pub const TIM2RST_MASK: u32 = 0x00000001;
    pub const TIM2RST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PWRRST** `[28]` (RW)
    ///
    /// Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> bool {
        (self.value.read_bits() & Self::PWRRST_MASK) != 0
    }

    /// Set PWRRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwrrst(&self) {
        self.value.modify_bits(|r| r | Self::PWRRST_MASK);
    }

    /// Clear PWRRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwrrst(&self) {
        self.value.modify_bits(|r| r & !Self::PWRRST_MASK);
    }

    /// **I2C1RST** `[21]` (RW)
    ///
    /// I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> bool {
        (self.value.read_bits() & Self::I2C1RST_MASK) != 0
    }

    /// Set I2C1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_i2c1rst(&self) {
        self.value.modify_bits(|r| r | Self::I2C1RST_MASK);
    }

    /// Clear I2C1RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_i2c1rst(&self) {
        self.value.modify_bits(|r| r & !Self::I2C1RST_MASK);
    }

    /// **USART4RST** `[19]` (RW)
    ///
    /// USART 4 reset
    #[inline(always)]
    pub fn usart4rst(&self) -> bool {
        (self.value.read_bits() & Self::USART4RST_MASK) != 0
    }

    /// Set USART4RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart4rst(&self) {
        self.value.modify_bits(|r| r | Self::USART4RST_MASK);
    }

    /// Clear USART4RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart4rst(&self) {
        self.value.modify_bits(|r| r & !Self::USART4RST_MASK);
    }

    /// **USART3RST** `[18]` (RW)
    ///
    /// USART 3 reset
    #[inline(always)]
    pub fn usart3rst(&self) -> bool {
        (self.value.read_bits() & Self::USART3RST_MASK) != 0
    }

    /// Set USART3RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart3rst(&self) {
        self.value.modify_bits(|r| r | Self::USART3RST_MASK);
    }

    /// Clear USART3RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart3rst(&self) {
        self.value.modify_bits(|r| r & !Self::USART3RST_MASK);
    }

    /// **USART2RST** `[17]` (RW)
    ///
    /// USART 2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> bool {
        (self.value.read_bits() & Self::USART2RST_MASK) != 0
    }

    /// Set USART2RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart2rst(&self) {
        self.value.modify_bits(|r| r | Self::USART2RST_MASK);
    }

    /// Clear USART2RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart2rst(&self) {
        self.value.modify_bits(|r| r & !Self::USART2RST_MASK);
    }

    /// **WWDGRST** `[11]` (RW)
    ///
    /// Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&self) -> bool {
        (self.value.read_bits() & Self::WWDGRST_MASK) != 0
    }

    /// Set WWDGRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wwdgrst(&self) {
        self.value.modify_bits(|r| r | Self::WWDGRST_MASK);
    }

    /// Clear WWDGRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wwdgrst(&self) {
        self.value.modify_bits(|r| r & !Self::WWDGRST_MASK);
    }

    /// **TIM3RST** `[1]` (RW)
    ///
    /// Timer 3 reset
    #[inline(always)]
    pub fn tim3rst(&self) -> bool {
        (self.value.read_bits() & Self::TIM3RST_MASK) != 0
    }

    /// Set TIM3RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim3rst(&self) {
        self.value.modify_bits(|r| r | Self::TIM3RST_MASK);
    }

    /// Clear TIM3RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim3rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIM3RST_MASK);
    }

    /// **TIM2RST** `[0]` (RW)
    ///
    /// Timer 2 reset
    #[inline(always)]
    pub fn tim2rst(&self) -> bool {
        (self.value.read_bits() & Self::TIM2RST_MASK) != 0
    }

    /// Set TIM2RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim2rst(&self) {
        self.value.modify_bits(|r| r | Self::TIM2RST_MASK);
    }

    /// Clear TIM2RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim2rst(&self) {
        self.value.modify_bits(|r| r & !Self::TIM2RST_MASK);
    }
}

/// APB1PRSTR value type for batch read/modify/write operations
///
/// Use `apb1prstr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb1prstr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb1prstrVal {
    bits: u32,
}

impl Apb1prstrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **PWRRST** `[28:28]`
    ///
    /// Power interface reset
    #[inline(always)]
    pub const fn pwrrst(&self) -> bool {
        (self.bits & Apb1prstr::PWRRST_MASK) != 0
    }

    /// Read **I2C1RST** `[21:21]`
    ///
    /// I2C1 reset
    #[inline(always)]
    pub const fn i2c1rst(&self) -> bool {
        (self.bits & Apb1prstr::I2C1RST_MASK) != 0
    }

    /// Read **USART4RST** `[19:19]`
    ///
    /// USART 4 reset
    #[inline(always)]
    pub const fn usart4rst(&self) -> bool {
        (self.bits & Apb1prstr::USART4RST_MASK) != 0
    }

    /// Read **USART3RST** `[18:18]`
    ///
    /// USART 3 reset
    #[inline(always)]
    pub const fn usart3rst(&self) -> bool {
        (self.bits & Apb1prstr::USART3RST_MASK) != 0
    }

    /// Read **USART2RST** `[17:17]`
    ///
    /// USART 2 reset
    #[inline(always)]
    pub const fn usart2rst(&self) -> bool {
        (self.bits & Apb1prstr::USART2RST_MASK) != 0
    }

    /// Read **WWDGRST** `[11:11]`
    ///
    /// Window watchdog reset
    #[inline(always)]
    pub const fn wwdgrst(&self) -> bool {
        (self.bits & Apb1prstr::WWDGRST_MASK) != 0
    }

    /// Read **TIM3RST** `[1:1]`
    ///
    /// Timer 3 reset
    #[inline(always)]
    pub const fn tim3rst(&self) -> bool {
        (self.bits & Apb1prstr::TIM3RST_MASK) != 0
    }

    /// Read **TIM2RST** `[0:0]`
    ///
    /// Timer 2 reset
    #[inline(always)]
    pub const fn tim2rst(&self) -> bool {
        (self.bits & Apb1prstr::TIM2RST_MASK) != 0
    }

    /// Set **PWRRST** `[28:28]`
    ///
    /// Power interface reset
    #[inline(always)]
    pub const fn with_pwrrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::PWRRST_MASK
            } else {
                self.bits & !Apb1prstr::PWRRST_MASK
            },
        }
    }

    /// Set **I2C1RST** `[21:21]`
    ///
    /// I2C1 reset
    #[inline(always)]
    pub const fn with_i2c1rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::I2C1RST_MASK
            } else {
                self.bits & !Apb1prstr::I2C1RST_MASK
            },
        }
    }

    /// Set **USART4RST** `[19:19]`
    ///
    /// USART 4 reset
    #[inline(always)]
    pub const fn with_usart4rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::USART4RST_MASK
            } else {
                self.bits & !Apb1prstr::USART4RST_MASK
            },
        }
    }

    /// Set **USART3RST** `[18:18]`
    ///
    /// USART 3 reset
    #[inline(always)]
    pub const fn with_usart3rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::USART3RST_MASK
            } else {
                self.bits & !Apb1prstr::USART3RST_MASK
            },
        }
    }

    /// Set **USART2RST** `[17:17]`
    ///
    /// USART 2 reset
    #[inline(always)]
    pub const fn with_usart2rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::USART2RST_MASK
            } else {
                self.bits & !Apb1prstr::USART2RST_MASK
            },
        }
    }

    /// Set **WWDGRST** `[11:11]`
    ///
    /// Window watchdog reset
    #[inline(always)]
    pub const fn with_wwdgrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::WWDGRST_MASK
            } else {
                self.bits & !Apb1prstr::WWDGRST_MASK
            },
        }
    }

    /// Set **TIM3RST** `[1:1]`
    ///
    /// Timer 3 reset
    #[inline(always)]
    pub const fn with_tim3rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::TIM3RST_MASK
            } else {
                self.bits & !Apb1prstr::TIM3RST_MASK
            },
        }
    }

    /// Set **TIM2RST** `[0:0]`
    ///
    /// Timer 2 reset
    #[inline(always)]
    pub const fn with_tim2rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1prstr::TIM2RST_MASK
            } else {
                self.bits & !Apb1prstr::TIM2RST_MASK
            },
        }
    }
}

impl Default for Apb1prstrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Apb1prstr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb1prstrVal {
        Apb1prstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Apb1prstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Apb1prstrVal) -> Apb1prstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHBPCENR Register
///
/// AHB Peripheral Clock enable register
/// (RCC_AHBPCENR)
///
/// Address offset: 0x0014
/// Reset value: 0x00000014
///
/// ## Fields
///
/// - **USBPD** `[17]` (RW)
///
///   USBPD clock enable
///
/// - **USBFSEN** `[12]` (RW)
///
///   USBFS clock enable
///
/// - **SRAMEN** `[2]` (RW)
///
///   SRAM interface clock
///   enable
///
/// - **DMA1EN** `[0]` (RW)
///
///   DMA clock enable
#[repr(transparent)]
pub struct Ahbpcenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ahbpcenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ahbpcenr {
    // ======== Field Constants ========

    /// **USBPD**
    ///
    /// USBPD clock enable
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const USBPD_OFFSET: u32 = 17;
    pub const USBPD_MASK: u32 = 0x00020000;
    pub const USBPD: u32 = 1 << 17;

    /// **USBFSEN**
    ///
    /// USBFS clock enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USBFSEN_OFFSET: u32 = 12;
    pub const USBFSEN_MASK: u32 = 0x00001000;
    pub const USBFSEN: u32 = 1 << 12;

    /// **SRAMEN**
    ///
    /// SRAM interface clock
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SRAMEN_OFFSET: u32 = 2;
    pub const SRAMEN_MASK: u32 = 0x00000004;
    pub const SRAMEN: u32 = 1 << 2;

    /// **DMA1EN**
    ///
    /// DMA clock enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DMA1EN_OFFSET: u32 = 0;
    pub const DMA1EN_MASK: u32 = 0x00000001;
    pub const DMA1EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USBPD** `[17]` (RW)
    ///
    /// USBPD clock enable
    #[inline(always)]
    pub fn usbpd(&self) -> bool {
        (self.value.read_bits() & Self::USBPD_MASK) != 0
    }

    /// Set USBPD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbpd(&self) {
        self.value.modify_bits(|r| r | Self::USBPD_MASK);
    }

    /// Clear USBPD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbpd(&self) {
        self.value.modify_bits(|r| r & !Self::USBPD_MASK);
    }

    /// **USBFSEN** `[12]` (RW)
    ///
    /// USBFS clock enable
    #[inline(always)]
    pub fn usbfsen(&self) -> bool {
        (self.value.read_bits() & Self::USBFSEN_MASK) != 0
    }

    /// Set USBFSEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbfsen(&self) {
        self.value.modify_bits(|r| r | Self::USBFSEN_MASK);
    }

    /// Clear USBFSEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbfsen(&self) {
        self.value.modify_bits(|r| r & !Self::USBFSEN_MASK);
    }

    /// **SRAMEN** `[2]` (RW)
    ///
    /// SRAM interface clock
    /// enable
    #[inline(always)]
    pub fn sramen(&self) -> bool {
        (self.value.read_bits() & Self::SRAMEN_MASK) != 0
    }

    /// Set SRAMEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sramen(&self) {
        self.value.modify_bits(|r| r | Self::SRAMEN_MASK);
    }

    /// Clear SRAMEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sramen(&self) {
        self.value.modify_bits(|r| r & !Self::SRAMEN_MASK);
    }

    /// **DMA1EN** `[0]` (RW)
    ///
    /// DMA clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> bool {
        (self.value.read_bits() & Self::DMA1EN_MASK) != 0
    }

    /// Set DMA1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma1en(&self) {
        self.value.modify_bits(|r| r | Self::DMA1EN_MASK);
    }

    /// Clear DMA1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma1en(&self) {
        self.value.modify_bits(|r| r & !Self::DMA1EN_MASK);
    }
}

/// AHBPCENR value type for batch read/modify/write operations
///
/// Use `ahbpcenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpcenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbpcenrVal {
    bits: u32,
}

impl AhbpcenrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000014 }
    }

    /// Read **USBPD** `[17:17]`
    ///
    /// USBPD clock enable
    #[inline(always)]
    pub const fn usbpd(&self) -> bool {
        (self.bits & Ahbpcenr::USBPD_MASK) != 0
    }

    /// Read **USBFSEN** `[12:12]`
    ///
    /// USBFS clock enable
    #[inline(always)]
    pub const fn usbfsen(&self) -> bool {
        (self.bits & Ahbpcenr::USBFSEN_MASK) != 0
    }

    /// Read **SRAMEN** `[2:2]`
    ///
    /// SRAM interface clock
    /// enable
    #[inline(always)]
    pub const fn sramen(&self) -> bool {
        (self.bits & Ahbpcenr::SRAMEN_MASK) != 0
    }

    /// Read **DMA1EN** `[0:0]`
    ///
    /// DMA clock enable
    #[inline(always)]
    pub const fn dma1en(&self) -> bool {
        (self.bits & Ahbpcenr::DMA1EN_MASK) != 0
    }

    /// Set **USBPD** `[17:17]`
    ///
    /// USBPD clock enable
    #[inline(always)]
    pub const fn with_usbpd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbpcenr::USBPD_MASK
            } else {
                self.bits & !Ahbpcenr::USBPD_MASK
            },
        }
    }

    /// Set **USBFSEN** `[12:12]`
    ///
    /// USBFS clock enable
    #[inline(always)]
    pub const fn with_usbfsen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbpcenr::USBFSEN_MASK
            } else {
                self.bits & !Ahbpcenr::USBFSEN_MASK
            },
        }
    }

    /// Set **SRAMEN** `[2:2]`
    ///
    /// SRAM interface clock
    /// enable
    #[inline(always)]
    pub const fn with_sramen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbpcenr::SRAMEN_MASK
            } else {
                self.bits & !Ahbpcenr::SRAMEN_MASK
            },
        }
    }

    /// Set **DMA1EN** `[0:0]`
    ///
    /// DMA clock enable
    #[inline(always)]
    pub const fn with_dma1en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbpcenr::DMA1EN_MASK
            } else {
                self.bits & !Ahbpcenr::DMA1EN_MASK
            },
        }
    }
}

impl Default for AhbpcenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ahbpcenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbpcenrVal {
        AhbpcenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbpcenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbpcenrVal) -> AhbpcenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// APB2PCENR Register
///
/// APB2 peripheral clock enable register
/// (RCC_APB2PCENR)
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USART1EN** `[14]` (RW)
///
///   USART1 clock enable
///
/// - **SPI1EN** `[12]` (RW)
///
///   SPI 1 clock enable
///
/// - **TIM1EN** `[11]` (RW)
///
///   TIM1 Timer clock enable
///
/// - **ADC1EN** `[9]` (RW)
///
///   ADC1 interface clock
///   enable
///
/// - **IOPCEN** `[4]` (RW)
///
///   I/O port C clock enable
///
/// - **IOPBEN** `[3]` (RW)
///
///   I/O port B clock enable
///
/// - **IOPAEN** `[2]` (RW)
///
///   I/O port A clock enable
///
/// - **AFIOEN** `[0]` (RW)
///
///   Alternate function I/O clock
///   enable
#[repr(transparent)]
pub struct Apb2pcenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2pcenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2pcenr {
    // ======== Field Constants ========

    /// **USART1EN**
    ///
    /// USART1 clock enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const USART1EN_OFFSET: u32 = 14;
    pub const USART1EN_MASK: u32 = 0x00004000;
    pub const USART1EN: u32 = 1 << 14;

    /// **SPI1EN**
    ///
    /// SPI 1 clock enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SPI1EN_OFFSET: u32 = 12;
    pub const SPI1EN_MASK: u32 = 0x00001000;
    pub const SPI1EN: u32 = 1 << 12;

    /// **TIM1EN**
    ///
    /// TIM1 Timer clock enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TIM1EN_OFFSET: u32 = 11;
    pub const TIM1EN_MASK: u32 = 0x00000800;
    pub const TIM1EN: u32 = 1 << 11;

    /// **ADC1EN**
    ///
    /// ADC1 interface clock
    /// enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ADC1EN_OFFSET: u32 = 9;
    pub const ADC1EN_MASK: u32 = 0x00000200;
    pub const ADC1EN: u32 = 1 << 9;

    /// **IOPCEN**
    ///
    /// I/O port C clock enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IOPCEN_OFFSET: u32 = 4;
    pub const IOPCEN_MASK: u32 = 0x00000010;
    pub const IOPCEN: u32 = 1 << 4;

    /// **IOPBEN**
    ///
    /// I/O port B clock enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IOPBEN_OFFSET: u32 = 3;
    pub const IOPBEN_MASK: u32 = 0x00000008;
    pub const IOPBEN: u32 = 1 << 3;

    /// **IOPAEN**
    ///
    /// I/O port A clock enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IOPAEN_OFFSET: u32 = 2;
    pub const IOPAEN_MASK: u32 = 0x00000004;
    pub const IOPAEN: u32 = 1 << 2;

    /// **AFIOEN**
    ///
    /// Alternate function I/O clock
    /// enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const AFIOEN_OFFSET: u32 = 0;
    pub const AFIOEN_MASK: u32 = 0x00000001;
    pub const AFIOEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USART1EN** `[14]` (RW)
    ///
    /// USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> bool {
        (self.value.read_bits() & Self::USART1EN_MASK) != 0
    }

    /// Set USART1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart1en(&self) {
        self.value.modify_bits(|r| r | Self::USART1EN_MASK);
    }

    /// Clear USART1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart1en(&self) {
        self.value.modify_bits(|r| r & !Self::USART1EN_MASK);
    }

    /// **SPI1EN** `[12]` (RW)
    ///
    /// SPI 1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> bool {
        (self.value.read_bits() & Self::SPI1EN_MASK) != 0
    }

    /// Set SPI1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi1en(&self) {
        self.value.modify_bits(|r| r | Self::SPI1EN_MASK);
    }

    /// Clear SPI1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi1en(&self) {
        self.value.modify_bits(|r| r & !Self::SPI1EN_MASK);
    }

    /// **TIM1EN** `[11]` (RW)
    ///
    /// TIM1 Timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> bool {
        (self.value.read_bits() & Self::TIM1EN_MASK) != 0
    }

    /// Set TIM1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim1en(&self) {
        self.value.modify_bits(|r| r | Self::TIM1EN_MASK);
    }

    /// Clear TIM1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim1en(&self) {
        self.value.modify_bits(|r| r & !Self::TIM1EN_MASK);
    }

    /// **ADC1EN** `[9]` (RW)
    ///
    /// ADC1 interface clock
    /// enable
    #[inline(always)]
    pub fn adc1en(&self) -> bool {
        (self.value.read_bits() & Self::ADC1EN_MASK) != 0
    }

    /// Set ADC1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_adc1en(&self) {
        self.value.modify_bits(|r| r | Self::ADC1EN_MASK);
    }

    /// Clear ADC1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_adc1en(&self) {
        self.value.modify_bits(|r| r & !Self::ADC1EN_MASK);
    }

    /// **IOPCEN** `[4]` (RW)
    ///
    /// I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> bool {
        (self.value.read_bits() & Self::IOPCEN_MASK) != 0
    }

    /// Set IOPCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iopcen(&self) {
        self.value.modify_bits(|r| r | Self::IOPCEN_MASK);
    }

    /// Clear IOPCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iopcen(&self) {
        self.value.modify_bits(|r| r & !Self::IOPCEN_MASK);
    }

    /// **IOPBEN** `[3]` (RW)
    ///
    /// I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> bool {
        (self.value.read_bits() & Self::IOPBEN_MASK) != 0
    }

    /// Set IOPBEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iopben(&self) {
        self.value.modify_bits(|r| r | Self::IOPBEN_MASK);
    }

    /// Clear IOPBEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iopben(&self) {
        self.value.modify_bits(|r| r & !Self::IOPBEN_MASK);
    }

    /// **IOPAEN** `[2]` (RW)
    ///
    /// I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> bool {
        (self.value.read_bits() & Self::IOPAEN_MASK) != 0
    }

    /// Set IOPAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iopaen(&self) {
        self.value.modify_bits(|r| r | Self::IOPAEN_MASK);
    }

    /// Clear IOPAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iopaen(&self) {
        self.value.modify_bits(|r| r & !Self::IOPAEN_MASK);
    }

    /// **AFIOEN** `[0]` (RW)
    ///
    /// Alternate function I/O clock
    /// enable
    #[inline(always)]
    pub fn afioen(&self) -> bool {
        (self.value.read_bits() & Self::AFIOEN_MASK) != 0
    }

    /// Set AFIOEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_afioen(&self) {
        self.value.modify_bits(|r| r | Self::AFIOEN_MASK);
    }

    /// Clear AFIOEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_afioen(&self) {
        self.value.modify_bits(|r| r & !Self::AFIOEN_MASK);
    }
}

/// APB2PCENR value type for batch read/modify/write operations
///
/// Use `apb2pcenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2pcenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2pcenrVal {
    bits: u32,
}

impl Apb2pcenrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **USART1EN** `[14:14]`
    ///
    /// USART1 clock enable
    #[inline(always)]
    pub const fn usart1en(&self) -> bool {
        (self.bits & Apb2pcenr::USART1EN_MASK) != 0
    }

    /// Read **SPI1EN** `[12:12]`
    ///
    /// SPI 1 clock enable
    #[inline(always)]
    pub const fn spi1en(&self) -> bool {
        (self.bits & Apb2pcenr::SPI1EN_MASK) != 0
    }

    /// Read **TIM1EN** `[11:11]`
    ///
    /// TIM1 Timer clock enable
    #[inline(always)]
    pub const fn tim1en(&self) -> bool {
        (self.bits & Apb2pcenr::TIM1EN_MASK) != 0
    }

    /// Read **ADC1EN** `[9:9]`
    ///
    /// ADC1 interface clock
    /// enable
    #[inline(always)]
    pub const fn adc1en(&self) -> bool {
        (self.bits & Apb2pcenr::ADC1EN_MASK) != 0
    }

    /// Read **IOPCEN** `[4:4]`
    ///
    /// I/O port C clock enable
    #[inline(always)]
    pub const fn iopcen(&self) -> bool {
        (self.bits & Apb2pcenr::IOPCEN_MASK) != 0
    }

    /// Read **IOPBEN** `[3:3]`
    ///
    /// I/O port B clock enable
    #[inline(always)]
    pub const fn iopben(&self) -> bool {
        (self.bits & Apb2pcenr::IOPBEN_MASK) != 0
    }

    /// Read **IOPAEN** `[2:2]`
    ///
    /// I/O port A clock enable
    #[inline(always)]
    pub const fn iopaen(&self) -> bool {
        (self.bits & Apb2pcenr::IOPAEN_MASK) != 0
    }

    /// Read **AFIOEN** `[0:0]`
    ///
    /// Alternate function I/O clock
    /// enable
    #[inline(always)]
    pub const fn afioen(&self) -> bool {
        (self.bits & Apb2pcenr::AFIOEN_MASK) != 0
    }

    /// Set **USART1EN** `[14:14]`
    ///
    /// USART1 clock enable
    #[inline(always)]
    pub const fn with_usart1en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::USART1EN_MASK
            } else {
                self.bits & !Apb2pcenr::USART1EN_MASK
            },
        }
    }

    /// Set **SPI1EN** `[12:12]`
    ///
    /// SPI 1 clock enable
    #[inline(always)]
    pub const fn with_spi1en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::SPI1EN_MASK
            } else {
                self.bits & !Apb2pcenr::SPI1EN_MASK
            },
        }
    }

    /// Set **TIM1EN** `[11:11]`
    ///
    /// TIM1 Timer clock enable
    #[inline(always)]
    pub const fn with_tim1en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::TIM1EN_MASK
            } else {
                self.bits & !Apb2pcenr::TIM1EN_MASK
            },
        }
    }

    /// Set **ADC1EN** `[9:9]`
    ///
    /// ADC1 interface clock
    /// enable
    #[inline(always)]
    pub const fn with_adc1en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::ADC1EN_MASK
            } else {
                self.bits & !Apb2pcenr::ADC1EN_MASK
            },
        }
    }

    /// Set **IOPCEN** `[4:4]`
    ///
    /// I/O port C clock enable
    #[inline(always)]
    pub const fn with_iopcen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::IOPCEN_MASK
            } else {
                self.bits & !Apb2pcenr::IOPCEN_MASK
            },
        }
    }

    /// Set **IOPBEN** `[3:3]`
    ///
    /// I/O port B clock enable
    #[inline(always)]
    pub const fn with_iopben(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::IOPBEN_MASK
            } else {
                self.bits & !Apb2pcenr::IOPBEN_MASK
            },
        }
    }

    /// Set **IOPAEN** `[2:2]`
    ///
    /// I/O port A clock enable
    #[inline(always)]
    pub const fn with_iopaen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::IOPAEN_MASK
            } else {
                self.bits & !Apb2pcenr::IOPAEN_MASK
            },
        }
    }

    /// Set **AFIOEN** `[0:0]`
    ///
    /// Alternate function I/O clock
    /// enable
    #[inline(always)]
    pub const fn with_afioen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2pcenr::AFIOEN_MASK
            } else {
                self.bits & !Apb2pcenr::AFIOEN_MASK
            },
        }
    }
}

impl Default for Apb2pcenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Apb2pcenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2pcenrVal {
        Apb2pcenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Apb2pcenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Apb2pcenrVal) -> Apb2pcenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// APB1PCENR Register
///
/// APB1 peripheral clock enable register
/// (RCC_APB1PCENR)
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWREN** `[28]` (RW)
///
///   Power interface clock
///   enable
///
/// - **I2C1EN** `[21]` (RW)
///
///   I2C 1 clock enable
///
/// - **UART4EN** `[19]` (RW)
///
///   UART 4 clock enable
///
/// - **USART3EN** `[18]` (RW)
///
///   USART 3 clock enable
///
/// - **USART2EN** `[17]` (RW)
///
///   USART 2 clock enable
///
/// - **WWDGEN** `[11]` (RW)
///
///   Window watchdog clock
///   enable
///
/// - **TIM3EN** `[1]` (RW)
///
///   Timer 3 clock enable
///
/// - **TIM2EN** `[0]` (RW)
///
///   Timer 2 clock enable
#[repr(transparent)]
pub struct Apb1pcenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb1pcenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb1pcenr {
    // ======== Field Constants ========

    /// **PWREN**
    ///
    /// Power interface clock
    /// enable
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const PWREN_OFFSET: u32 = 28;
    pub const PWREN_MASK: u32 = 0x10000000;
    pub const PWREN: u32 = 1 << 28;

    /// **I2C1EN**
    ///
    /// I2C 1 clock enable
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const I2C1EN_OFFSET: u32 = 21;
    pub const I2C1EN_MASK: u32 = 0x00200000;
    pub const I2C1EN: u32 = 1 << 21;

    /// **UART4EN**
    ///
    /// UART 4 clock enable
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const UART4EN_OFFSET: u32 = 19;
    pub const UART4EN_MASK: u32 = 0x00080000;
    pub const UART4EN: u32 = 1 << 19;

    /// **USART3EN**
    ///
    /// USART 3 clock enable
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const USART3EN_OFFSET: u32 = 18;
    pub const USART3EN_MASK: u32 = 0x00040000;
    pub const USART3EN: u32 = 1 << 18;

    /// **USART2EN**
    ///
    /// USART 2 clock enable
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const USART2EN_OFFSET: u32 = 17;
    pub const USART2EN_MASK: u32 = 0x00020000;
    pub const USART2EN: u32 = 1 << 17;

    /// **WWDGEN**
    ///
    /// Window watchdog clock
    /// enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const WWDGEN_OFFSET: u32 = 11;
    pub const WWDGEN_MASK: u32 = 0x00000800;
    pub const WWDGEN: u32 = 1 << 11;

    /// **TIM3EN**
    ///
    /// Timer 3 clock enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIM3EN_OFFSET: u32 = 1;
    pub const TIM3EN_MASK: u32 = 0x00000002;
    pub const TIM3EN: u32 = 1 << 1;

    /// **TIM2EN**
    ///
    /// Timer 2 clock enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIM2EN_OFFSET: u32 = 0;
    pub const TIM2EN_MASK: u32 = 0x00000001;
    pub const TIM2EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PWREN** `[28]` (RW)
    ///
    /// Power interface clock
    /// enable
    #[inline(always)]
    pub fn pwren(&self) -> bool {
        (self.value.read_bits() & Self::PWREN_MASK) != 0
    }

    /// Set PWREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwren(&self) {
        self.value.modify_bits(|r| r | Self::PWREN_MASK);
    }

    /// Clear PWREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwren(&self) {
        self.value.modify_bits(|r| r & !Self::PWREN_MASK);
    }

    /// **I2C1EN** `[21]` (RW)
    ///
    /// I2C 1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> bool {
        (self.value.read_bits() & Self::I2C1EN_MASK) != 0
    }

    /// Set I2C1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_i2c1en(&self) {
        self.value.modify_bits(|r| r | Self::I2C1EN_MASK);
    }

    /// Clear I2C1EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_i2c1en(&self) {
        self.value.modify_bits(|r| r & !Self::I2C1EN_MASK);
    }

    /// **UART4EN** `[19]` (RW)
    ///
    /// UART 4 clock enable
    #[inline(always)]
    pub fn uart4en(&self) -> bool {
        (self.value.read_bits() & Self::UART4EN_MASK) != 0
    }

    /// Set UART4EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uart4en(&self) {
        self.value.modify_bits(|r| r | Self::UART4EN_MASK);
    }

    /// Clear UART4EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uart4en(&self) {
        self.value.modify_bits(|r| r & !Self::UART4EN_MASK);
    }

    /// **USART3EN** `[18]` (RW)
    ///
    /// USART 3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> bool {
        (self.value.read_bits() & Self::USART3EN_MASK) != 0
    }

    /// Set USART3EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart3en(&self) {
        self.value.modify_bits(|r| r | Self::USART3EN_MASK);
    }

    /// Clear USART3EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart3en(&self) {
        self.value.modify_bits(|r| r & !Self::USART3EN_MASK);
    }

    /// **USART2EN** `[17]` (RW)
    ///
    /// USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> bool {
        (self.value.read_bits() & Self::USART2EN_MASK) != 0
    }

    /// Set USART2EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usart2en(&self) {
        self.value.modify_bits(|r| r | Self::USART2EN_MASK);
    }

    /// Clear USART2EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usart2en(&self) {
        self.value.modify_bits(|r| r & !Self::USART2EN_MASK);
    }

    /// **WWDGEN** `[11]` (RW)
    ///
    /// Window watchdog clock
    /// enable
    #[inline(always)]
    pub fn wwdgen(&self) -> bool {
        (self.value.read_bits() & Self::WWDGEN_MASK) != 0
    }

    /// Set WWDGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wwdgen(&self) {
        self.value.modify_bits(|r| r | Self::WWDGEN_MASK);
    }

    /// Clear WWDGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wwdgen(&self) {
        self.value.modify_bits(|r| r & !Self::WWDGEN_MASK);
    }

    /// **TIM3EN** `[1]` (RW)
    ///
    /// Timer 3 clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> bool {
        (self.value.read_bits() & Self::TIM3EN_MASK) != 0
    }

    /// Set TIM3EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim3en(&self) {
        self.value.modify_bits(|r| r | Self::TIM3EN_MASK);
    }

    /// Clear TIM3EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim3en(&self) {
        self.value.modify_bits(|r| r & !Self::TIM3EN_MASK);
    }

    /// **TIM2EN** `[0]` (RW)
    ///
    /// Timer 2 clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> bool {
        (self.value.read_bits() & Self::TIM2EN_MASK) != 0
    }

    /// Set TIM2EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim2en(&self) {
        self.value.modify_bits(|r| r | Self::TIM2EN_MASK);
    }

    /// Clear TIM2EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim2en(&self) {
        self.value.modify_bits(|r| r & !Self::TIM2EN_MASK);
    }
}

/// APB1PCENR value type for batch read/modify/write operations
///
/// Use `apb1pcenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb1pcenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb1pcenrVal {
    bits: u32,
}

impl Apb1pcenrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **PWREN** `[28:28]`
    ///
    /// Power interface clock
    /// enable
    #[inline(always)]
    pub const fn pwren(&self) -> bool {
        (self.bits & Apb1pcenr::PWREN_MASK) != 0
    }

    /// Read **I2C1EN** `[21:21]`
    ///
    /// I2C 1 clock enable
    #[inline(always)]
    pub const fn i2c1en(&self) -> bool {
        (self.bits & Apb1pcenr::I2C1EN_MASK) != 0
    }

    /// Read **UART4EN** `[19:19]`
    ///
    /// UART 4 clock enable
    #[inline(always)]
    pub const fn uart4en(&self) -> bool {
        (self.bits & Apb1pcenr::UART4EN_MASK) != 0
    }

    /// Read **USART3EN** `[18:18]`
    ///
    /// USART 3 clock enable
    #[inline(always)]
    pub const fn usart3en(&self) -> bool {
        (self.bits & Apb1pcenr::USART3EN_MASK) != 0
    }

    /// Read **USART2EN** `[17:17]`
    ///
    /// USART 2 clock enable
    #[inline(always)]
    pub const fn usart2en(&self) -> bool {
        (self.bits & Apb1pcenr::USART2EN_MASK) != 0
    }

    /// Read **WWDGEN** `[11:11]`
    ///
    /// Window watchdog clock
    /// enable
    #[inline(always)]
    pub const fn wwdgen(&self) -> bool {
        (self.bits & Apb1pcenr::WWDGEN_MASK) != 0
    }

    /// Read **TIM3EN** `[1:1]`
    ///
    /// Timer 3 clock enable
    #[inline(always)]
    pub const fn tim3en(&self) -> bool {
        (self.bits & Apb1pcenr::TIM3EN_MASK) != 0
    }

    /// Read **TIM2EN** `[0:0]`
    ///
    /// Timer 2 clock enable
    #[inline(always)]
    pub const fn tim2en(&self) -> bool {
        (self.bits & Apb1pcenr::TIM2EN_MASK) != 0
    }

    /// Set **PWREN** `[28:28]`
    ///
    /// Power interface clock
    /// enable
    #[inline(always)]
    pub const fn with_pwren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::PWREN_MASK
            } else {
                self.bits & !Apb1pcenr::PWREN_MASK
            },
        }
    }

    /// Set **I2C1EN** `[21:21]`
    ///
    /// I2C 1 clock enable
    #[inline(always)]
    pub const fn with_i2c1en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::I2C1EN_MASK
            } else {
                self.bits & !Apb1pcenr::I2C1EN_MASK
            },
        }
    }

    /// Set **UART4EN** `[19:19]`
    ///
    /// UART 4 clock enable
    #[inline(always)]
    pub const fn with_uart4en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::UART4EN_MASK
            } else {
                self.bits & !Apb1pcenr::UART4EN_MASK
            },
        }
    }

    /// Set **USART3EN** `[18:18]`
    ///
    /// USART 3 clock enable
    #[inline(always)]
    pub const fn with_usart3en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::USART3EN_MASK
            } else {
                self.bits & !Apb1pcenr::USART3EN_MASK
            },
        }
    }

    /// Set **USART2EN** `[17:17]`
    ///
    /// USART 2 clock enable
    #[inline(always)]
    pub const fn with_usart2en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::USART2EN_MASK
            } else {
                self.bits & !Apb1pcenr::USART2EN_MASK
            },
        }
    }

    /// Set **WWDGEN** `[11:11]`
    ///
    /// Window watchdog clock
    /// enable
    #[inline(always)]
    pub const fn with_wwdgen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::WWDGEN_MASK
            } else {
                self.bits & !Apb1pcenr::WWDGEN_MASK
            },
        }
    }

    /// Set **TIM3EN** `[1:1]`
    ///
    /// Timer 3 clock enable
    #[inline(always)]
    pub const fn with_tim3en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::TIM3EN_MASK
            } else {
                self.bits & !Apb1pcenr::TIM3EN_MASK
            },
        }
    }

    /// Set **TIM2EN** `[0:0]`
    ///
    /// Timer 2 clock enable
    #[inline(always)]
    pub const fn with_tim2en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb1pcenr::TIM2EN_MASK
            } else {
                self.bits & !Apb1pcenr::TIM2EN_MASK
            },
        }
    }
}

impl Default for Apb1pcenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Apb1pcenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb1pcenrVal {
        Apb1pcenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Apb1pcenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Apb1pcenrVal) -> Apb1pcenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSTSCKR Register
///
/// Control/status register
/// (RCC_RSTSCKR)
///
/// Address offset: 0x0024
/// Reset value: 0x0c000000
///
/// ## Fields
///
/// - **LPWRRSTF** `[31]` (RW)
///
///   Low-power reset flag
///
/// - **WWDGRSTF** `[30]` (RW)
///
///   Window watchdog reset flag
///
/// - **IWDGRSTF** `[29]` (RW)
///
///   Independent watchdog reset
///   flag
///
/// - **SFTRSTF** `[28]` (RW)
///
///   Software reset flag
///
/// - **PORRSTF** `[27]` (RW)
///
///   POR/PDR reset flag
///
/// - **PINRSTF** `[26]` (RW)
///
///   PIN reset flag
///
/// - **OPARSTF** `[25]` (RW)
///
///   OPA reset flag
///
/// - **RMVF** `[24]` (RW)
///
///   Remove reset flag
#[repr(transparent)]
pub struct Rstsckr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rstsckr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rstsckr {
    // ======== Field Constants ========

    /// **LPWRRSTF**
    ///
    /// Low-power reset flag
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LPWRRSTF_OFFSET: u32 = 31;
    pub const LPWRRSTF_MASK: u32 = 0x80000000;
    pub const LPWRRSTF: u32 = 1 << 31;

    /// **WWDGRSTF**
    ///
    /// Window watchdog reset flag
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const WWDGRSTF_OFFSET: u32 = 30;
    pub const WWDGRSTF_MASK: u32 = 0x40000000;
    pub const WWDGRSTF: u32 = 1 << 30;

    /// **IWDGRSTF**
    ///
    /// Independent watchdog reset
    /// flag
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const IWDGRSTF_OFFSET: u32 = 29;
    pub const IWDGRSTF_MASK: u32 = 0x20000000;
    pub const IWDGRSTF: u32 = 1 << 29;

    /// **SFTRSTF**
    ///
    /// Software reset flag
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SFTRSTF_OFFSET: u32 = 28;
    pub const SFTRSTF_MASK: u32 = 0x10000000;
    pub const SFTRSTF: u32 = 1 << 28;

    /// **PORRSTF**
    ///
    /// POR/PDR reset flag
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const PORRSTF_OFFSET: u32 = 27;
    pub const PORRSTF_MASK: u32 = 0x08000000;
    pub const PORRSTF: u32 = 1 << 27;

    /// **PINRSTF**
    ///
    /// PIN reset flag
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const PINRSTF_OFFSET: u32 = 26;
    pub const PINRSTF_MASK: u32 = 0x04000000;
    pub const PINRSTF: u32 = 1 << 26;

    /// **OPARSTF**
    ///
    /// OPA reset flag
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const OPARSTF_OFFSET: u32 = 25;
    pub const OPARSTF_MASK: u32 = 0x02000000;
    pub const OPARSTF: u32 = 1 << 25;

    /// **RMVF**
    ///
    /// Remove reset flag
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const RMVF_OFFSET: u32 = 24;
    pub const RMVF_MASK: u32 = 0x01000000;
    pub const RMVF: u32 = 1 << 24;

    // ======== Field Accessors ========

    /// **LPWRRSTF** `[31]` (RW)
    ///
    /// Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> bool {
        (self.value.read_bits() & Self::LPWRRSTF_MASK) != 0
    }

    /// Set LPWRRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lpwrrstf(&self) {
        self.value.modify_bits(|r| r | Self::LPWRRSTF_MASK);
    }

    /// Clear LPWRRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lpwrrstf(&self) {
        self.value.modify_bits(|r| r & !Self::LPWRRSTF_MASK);
    }

    /// **WWDGRSTF** `[30]` (RW)
    ///
    /// Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> bool {
        (self.value.read_bits() & Self::WWDGRSTF_MASK) != 0
    }

    /// Set WWDGRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wwdgrstf(&self) {
        self.value.modify_bits(|r| r | Self::WWDGRSTF_MASK);
    }

    /// Clear WWDGRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wwdgrstf(&self) {
        self.value.modify_bits(|r| r & !Self::WWDGRSTF_MASK);
    }

    /// **IWDGRSTF** `[29]` (RW)
    ///
    /// Independent watchdog reset
    /// flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> bool {
        (self.value.read_bits() & Self::IWDGRSTF_MASK) != 0
    }

    /// Set IWDGRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iwdgrstf(&self) {
        self.value.modify_bits(|r| r | Self::IWDGRSTF_MASK);
    }

    /// Clear IWDGRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iwdgrstf(&self) {
        self.value.modify_bits(|r| r & !Self::IWDGRSTF_MASK);
    }

    /// **SFTRSTF** `[28]` (RW)
    ///
    /// Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> bool {
        (self.value.read_bits() & Self::SFTRSTF_MASK) != 0
    }

    /// Set SFTRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sftrstf(&self) {
        self.value.modify_bits(|r| r | Self::SFTRSTF_MASK);
    }

    /// Clear SFTRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sftrstf(&self) {
        self.value.modify_bits(|r| r & !Self::SFTRSTF_MASK);
    }

    /// **PORRSTF** `[27]` (RW)
    ///
    /// POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> bool {
        (self.value.read_bits() & Self::PORRSTF_MASK) != 0
    }

    /// Set PORRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_porrstf(&self) {
        self.value.modify_bits(|r| r | Self::PORRSTF_MASK);
    }

    /// Clear PORRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_porrstf(&self) {
        self.value.modify_bits(|r| r & !Self::PORRSTF_MASK);
    }

    /// **PINRSTF** `[26]` (RW)
    ///
    /// PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> bool {
        (self.value.read_bits() & Self::PINRSTF_MASK) != 0
    }

    /// Set PINRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinrstf(&self) {
        self.value.modify_bits(|r| r | Self::PINRSTF_MASK);
    }

    /// Clear PINRSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinrstf(&self) {
        self.value.modify_bits(|r| r & !Self::PINRSTF_MASK);
    }

    /// **OPARSTF** `[25]` (RW)
    ///
    /// OPA reset flag
    #[inline(always)]
    pub fn oparstf(&self) -> bool {
        (self.value.read_bits() & Self::OPARSTF_MASK) != 0
    }

    /// Set OPARSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oparstf(&self) {
        self.value.modify_bits(|r| r | Self::OPARSTF_MASK);
    }

    /// Clear OPARSTF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oparstf(&self) {
        self.value.modify_bits(|r| r & !Self::OPARSTF_MASK);
    }

    /// **RMVF** `[24]` (RW)
    ///
    /// Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> bool {
        (self.value.read_bits() & Self::RMVF_MASK) != 0
    }

    /// Set RMVF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rmvf(&self) {
        self.value.modify_bits(|r| r | Self::RMVF_MASK);
    }

    /// Clear RMVF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rmvf(&self) {
        self.value.modify_bits(|r| r & !Self::RMVF_MASK);
    }
}

/// RSTSCKR value type for batch read/modify/write operations
///
/// Use `rstsckr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rstsckr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RstsckrVal {
    bits: u32,
}

impl RstsckrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0c000000 }
    }

    /// Read **LPWRRSTF** `[31:31]`
    ///
    /// Low-power reset flag
    #[inline(always)]
    pub const fn lpwrrstf(&self) -> bool {
        (self.bits & Rstsckr::LPWRRSTF_MASK) != 0
    }

    /// Read **WWDGRSTF** `[30:30]`
    ///
    /// Window watchdog reset flag
    #[inline(always)]
    pub const fn wwdgrstf(&self) -> bool {
        (self.bits & Rstsckr::WWDGRSTF_MASK) != 0
    }

    /// Read **IWDGRSTF** `[29:29]`
    ///
    /// Independent watchdog reset
    /// flag
    #[inline(always)]
    pub const fn iwdgrstf(&self) -> bool {
        (self.bits & Rstsckr::IWDGRSTF_MASK) != 0
    }

    /// Read **SFTRSTF** `[28:28]`
    ///
    /// Software reset flag
    #[inline(always)]
    pub const fn sftrstf(&self) -> bool {
        (self.bits & Rstsckr::SFTRSTF_MASK) != 0
    }

    /// Read **PORRSTF** `[27:27]`
    ///
    /// POR/PDR reset flag
    #[inline(always)]
    pub const fn porrstf(&self) -> bool {
        (self.bits & Rstsckr::PORRSTF_MASK) != 0
    }

    /// Read **PINRSTF** `[26:26]`
    ///
    /// PIN reset flag
    #[inline(always)]
    pub const fn pinrstf(&self) -> bool {
        (self.bits & Rstsckr::PINRSTF_MASK) != 0
    }

    /// Read **OPARSTF** `[25:25]`
    ///
    /// OPA reset flag
    #[inline(always)]
    pub const fn oparstf(&self) -> bool {
        (self.bits & Rstsckr::OPARSTF_MASK) != 0
    }

    /// Read **RMVF** `[24:24]`
    ///
    /// Remove reset flag
    #[inline(always)]
    pub const fn rmvf(&self) -> bool {
        (self.bits & Rstsckr::RMVF_MASK) != 0
    }

    /// Set **LPWRRSTF** `[31:31]`
    ///
    /// Low-power reset flag
    #[inline(always)]
    pub const fn with_lpwrrstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::LPWRRSTF_MASK
            } else {
                self.bits & !Rstsckr::LPWRRSTF_MASK
            },
        }
    }

    /// Set **WWDGRSTF** `[30:30]`
    ///
    /// Window watchdog reset flag
    #[inline(always)]
    pub const fn with_wwdgrstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::WWDGRSTF_MASK
            } else {
                self.bits & !Rstsckr::WWDGRSTF_MASK
            },
        }
    }

    /// Set **IWDGRSTF** `[29:29]`
    ///
    /// Independent watchdog reset
    /// flag
    #[inline(always)]
    pub const fn with_iwdgrstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::IWDGRSTF_MASK
            } else {
                self.bits & !Rstsckr::IWDGRSTF_MASK
            },
        }
    }

    /// Set **SFTRSTF** `[28:28]`
    ///
    /// Software reset flag
    #[inline(always)]
    pub const fn with_sftrstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::SFTRSTF_MASK
            } else {
                self.bits & !Rstsckr::SFTRSTF_MASK
            },
        }
    }

    /// Set **PORRSTF** `[27:27]`
    ///
    /// POR/PDR reset flag
    #[inline(always)]
    pub const fn with_porrstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::PORRSTF_MASK
            } else {
                self.bits & !Rstsckr::PORRSTF_MASK
            },
        }
    }

    /// Set **PINRSTF** `[26:26]`
    ///
    /// PIN reset flag
    #[inline(always)]
    pub const fn with_pinrstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::PINRSTF_MASK
            } else {
                self.bits & !Rstsckr::PINRSTF_MASK
            },
        }
    }

    /// Set **OPARSTF** `[25:25]`
    ///
    /// OPA reset flag
    #[inline(always)]
    pub const fn with_oparstf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::OPARSTF_MASK
            } else {
                self.bits & !Rstsckr::OPARSTF_MASK
            },
        }
    }

    /// Set **RMVF** `[24:24]`
    ///
    /// Remove reset flag
    #[inline(always)]
    pub const fn with_rmvf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rstsckr::RMVF_MASK
            } else {
                self.bits & !Rstsckr::RMVF_MASK
            },
        }
    }
}

impl Default for RstsckrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rstsckr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RstsckrVal {
        RstsckrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RstsckrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RstsckrVal) -> RstsckrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHBRSTR Register
///
/// AHB reset register
/// (RCC_APHBRSTR)
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USBPDRST** `[17]` (RW)
///
///   USBPD reset
///
/// - **PIOCRST** `[13]` (RW)
///
///   PIOC reset
///
/// - **USBFSRST** `[12]` (RW)
///
///   USBFS reset
#[repr(transparent)]
pub struct Ahbrstr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ahbrstr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ahbrstr {
    // ======== Field Constants ========

    /// **USBPDRST**
    ///
    /// USBPD reset
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const USBPDRST_OFFSET: u32 = 17;
    pub const USBPDRST_MASK: u32 = 0x00020000;
    pub const USBPDRST: u32 = 1 << 17;

    /// **PIOCRST**
    ///
    /// PIOC reset
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const PIOCRST_OFFSET: u32 = 13;
    pub const PIOCRST_MASK: u32 = 0x00002000;
    pub const PIOCRST: u32 = 1 << 13;

    /// **USBFSRST**
    ///
    /// USBFS reset
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USBFSRST_OFFSET: u32 = 12;
    pub const USBFSRST_MASK: u32 = 0x00001000;
    pub const USBFSRST: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **USBPDRST** `[17]` (RW)
    ///
    /// USBPD reset
    #[inline(always)]
    pub fn usbpdrst(&self) -> bool {
        (self.value.read_bits() & Self::USBPDRST_MASK) != 0
    }

    /// Set USBPDRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbpdrst(&self) {
        self.value.modify_bits(|r| r | Self::USBPDRST_MASK);
    }

    /// Clear USBPDRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbpdrst(&self) {
        self.value.modify_bits(|r| r & !Self::USBPDRST_MASK);
    }

    /// **PIOCRST** `[13]` (RW)
    ///
    /// PIOC reset
    #[inline(always)]
    pub fn piocrst(&self) -> bool {
        (self.value.read_bits() & Self::PIOCRST_MASK) != 0
    }

    /// Set PIOCRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_piocrst(&self) {
        self.value.modify_bits(|r| r | Self::PIOCRST_MASK);
    }

    /// Clear PIOCRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_piocrst(&self) {
        self.value.modify_bits(|r| r & !Self::PIOCRST_MASK);
    }

    /// **USBFSRST** `[12]` (RW)
    ///
    /// USBFS reset
    #[inline(always)]
    pub fn usbfsrst(&self) -> bool {
        (self.value.read_bits() & Self::USBFSRST_MASK) != 0
    }

    /// Set USBFSRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbfsrst(&self) {
        self.value.modify_bits(|r| r | Self::USBFSRST_MASK);
    }

    /// Clear USBFSRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbfsrst(&self) {
        self.value.modify_bits(|r| r & !Self::USBFSRST_MASK);
    }
}

/// AHBRSTR value type for batch read/modify/write operations
///
/// Use `ahbrstr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbrstr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbrstrVal {
    bits: u32,
}

impl AhbrstrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **USBPDRST** `[17:17]`
    ///
    /// USBPD reset
    #[inline(always)]
    pub const fn usbpdrst(&self) -> bool {
        (self.bits & Ahbrstr::USBPDRST_MASK) != 0
    }

    /// Read **PIOCRST** `[13:13]`
    ///
    /// PIOC reset
    #[inline(always)]
    pub const fn piocrst(&self) -> bool {
        (self.bits & Ahbrstr::PIOCRST_MASK) != 0
    }

    /// Read **USBFSRST** `[12:12]`
    ///
    /// USBFS reset
    #[inline(always)]
    pub const fn usbfsrst(&self) -> bool {
        (self.bits & Ahbrstr::USBFSRST_MASK) != 0
    }

    /// Set **USBPDRST** `[17:17]`
    ///
    /// USBPD reset
    #[inline(always)]
    pub const fn with_usbpdrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbrstr::USBPDRST_MASK
            } else {
                self.bits & !Ahbrstr::USBPDRST_MASK
            },
        }
    }

    /// Set **PIOCRST** `[13:13]`
    ///
    /// PIOC reset
    #[inline(always)]
    pub const fn with_piocrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbrstr::PIOCRST_MASK
            } else {
                self.bits & !Ahbrstr::PIOCRST_MASK
            },
        }
    }

    /// Set **USBFSRST** `[12:12]`
    ///
    /// USBFS reset
    #[inline(always)]
    pub const fn with_usbfsrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahbrstr::USBFSRST_MASK
            } else {
                self.bits & !Ahbrstr::USBFSRST_MASK
            },
        }
    }
}

impl Default for AhbrstrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ahbrstr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbrstrVal {
        AhbrstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbrstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbrstrVal) -> AhbrstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Clock control register
    pub ctlr: Ctlr,
    /// 0x0004
    /// Clock configuration register
    /// (RCC_CFGR0)
    pub cfgr0: Cfgr0,
    _reserved0: [u8; 4],
    /// 0x000c
    /// APB2 peripheral reset register
    /// (RCC_APB2PRSTR)
    pub apb2prstr: Apb2prstr,
    /// 0x0010
    /// APB1 peripheral reset register
    /// (RCC_APB1PRSTR)
    pub apb1prstr: Apb1prstr,
    /// 0x0014
    /// AHB Peripheral Clock enable register
    /// (RCC_AHBPCENR)
    pub ahbpcenr: Ahbpcenr,
    /// 0x0018
    /// APB2 peripheral clock enable register
    /// (RCC_APB2PCENR)
    pub apb2pcenr: Apb2pcenr,
    /// 0x001c
    /// APB1 peripheral clock enable register
    /// (RCC_APB1PCENR)
    pub apb1pcenr: Apb1pcenr,
    _reserved1: [u8; 4],
    /// 0x0024
    /// Control/status register
    /// (RCC_RSTSCKR)
    pub rstsckr: Rstsckr,
    /// 0x0028
    /// AHB reset register
    /// (RCC_APHBRSTR)
    pub ahbrstr: Ahbrstr,
}
