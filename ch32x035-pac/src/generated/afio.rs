//! AFIO
//! Alternate function I/O

#![allow(dead_code)]

pub const BASE: u32 = 0x40010000;

/// PCFR1 Register
///
/// AF remap and debug I/O configuration
/// register (AFIO_PCFR1)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SW_CFG** `[26:24]` (RW)
///
///   SW_CFG Configuration
///
/// - **PLOC_RM** `[23]` (RW)
///
///   IO2W_RM remapping
///
/// - **TIM3RM** `[22:21]` (RW)
///
///   TIM3 remapping
///
/// - **TIM2RM** `[20:18]` (RW)
///
///   TIM2 remapping
///
/// - **TIM1RM** `[17:15]` (RW)
///
///   TIM1 remapping
///
/// - **USART4RM** `[14:12]` (RW)
///
///   USART4 remapping
///
/// - **USART3RM** `[11:10]` (RW)
///
///   USART3 remapping
///
/// - **USART2RM** `[9:7]` (RW)
///
///   USART2 remapping
///
/// - **USART1RM** `[6:5]` (RW)
///
///   USART1 remapping
///
/// - **I2C1RM** `[4:2]` (RW)
///
///   I2C1 remapping
///
/// - **SPI1RM** `[1:0]` (RW)
///
///   SPI1 remapping
#[repr(transparent)]
pub struct Pcfr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pcfr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pcfr1 {
    // ======== Field Constants ========

    /// **SW_CFG**
    ///
    /// SW_CFG Configuration
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const SW_CFG_OFFSET: u32 = 24;
    pub const SW_CFG_MASK: u32 = 0x07000000;

    /// **PLOC_RM**
    ///
    /// IO2W_RM remapping
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const PLOC_RM_OFFSET: u32 = 23;
    pub const PLOC_RM_MASK: u32 = 0x00800000;
    pub const PLOC_RM: u32 = 1 << 23;

    /// **TIM3RM**
    ///
    /// TIM3 remapping
    ///
    /// Bits `[22:21]`, mask `0x00600000`
    pub const TIM3RM_OFFSET: u32 = 21;
    pub const TIM3RM_MASK: u32 = 0x00600000;

    /// **TIM2RM**
    ///
    /// TIM2 remapping
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const TIM2RM_OFFSET: u32 = 18;
    pub const TIM2RM_MASK: u32 = 0x001c0000;

    /// **TIM1RM**
    ///
    /// TIM1 remapping
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const TIM1RM_OFFSET: u32 = 15;
    pub const TIM1RM_MASK: u32 = 0x00038000;

    /// **USART4RM**
    ///
    /// USART4 remapping
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const USART4RM_OFFSET: u32 = 12;
    pub const USART4RM_MASK: u32 = 0x00007000;

    /// **USART3RM**
    ///
    /// USART3 remapping
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const USART3RM_OFFSET: u32 = 10;
    pub const USART3RM_MASK: u32 = 0x00000c00;

    /// **USART2RM**
    ///
    /// USART2 remapping
    ///
    /// Bits `[9:7]`, mask `0x00000380`
    pub const USART2RM_OFFSET: u32 = 7;
    pub const USART2RM_MASK: u32 = 0x00000380;

    /// **USART1RM**
    ///
    /// USART1 remapping
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const USART1RM_OFFSET: u32 = 5;
    pub const USART1RM_MASK: u32 = 0x00000060;

    /// **I2C1RM**
    ///
    /// I2C1 remapping
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const I2C1RM_OFFSET: u32 = 2;
    pub const I2C1RM_MASK: u32 = 0x0000001c;

    /// **SPI1RM**
    ///
    /// SPI1 remapping
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SPI1RM_OFFSET: u32 = 0;
    pub const SPI1RM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SW_CFG** `[26:24]` (RW)
    ///
    /// SW_CFG Configuration
    #[inline(always)]
    pub fn sw_cfg(&self) -> u32 {
        (self.value.read_bits() & Self::SW_CFG_MASK) >> Self::SW_CFG_OFFSET
    }

    /// Write **SW_CFG** `[26:24]` (RW)
    ///
    /// SW_CFG Configuration
    #[inline(always)]
    pub fn write_sw_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SW_CFG_MASK) | ((value << Self::SW_CFG_OFFSET) & Self::SW_CFG_MASK)
        });
    }

    /// **PLOC_RM** `[23]` (RW)
    ///
    /// IO2W_RM remapping
    #[inline(always)]
    pub fn ploc_rm(&self) -> bool {
        (self.value.read_bits() & Self::PLOC_RM_MASK) != 0
    }

    /// Set PLOC_RM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ploc_rm(&self) {
        self.value.modify_bits(|r| r | Self::PLOC_RM_MASK);
    }

    /// Clear PLOC_RM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ploc_rm(&self) {
        self.value.modify_bits(|r| r & !Self::PLOC_RM_MASK);
    }

    /// **TIM3RM** `[22:21]` (RW)
    ///
    /// TIM3 remapping
    #[inline(always)]
    pub fn tim3rm(&self) -> u32 {
        (self.value.read_bits() & Self::TIM3RM_MASK) >> Self::TIM3RM_OFFSET
    }

    /// Write **TIM3RM** `[22:21]` (RW)
    ///
    /// TIM3 remapping
    #[inline(always)]
    pub fn write_tim3rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIM3RM_MASK) | ((value << Self::TIM3RM_OFFSET) & Self::TIM3RM_MASK)
        });
    }

    /// **TIM2RM** `[20:18]` (RW)
    ///
    /// TIM2 remapping
    #[inline(always)]
    pub fn tim2rm(&self) -> u32 {
        (self.value.read_bits() & Self::TIM2RM_MASK) >> Self::TIM2RM_OFFSET
    }

    /// Write **TIM2RM** `[20:18]` (RW)
    ///
    /// TIM2 remapping
    #[inline(always)]
    pub fn write_tim2rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIM2RM_MASK) | ((value << Self::TIM2RM_OFFSET) & Self::TIM2RM_MASK)
        });
    }

    /// **TIM1RM** `[17:15]` (RW)
    ///
    /// TIM1 remapping
    #[inline(always)]
    pub fn tim1rm(&self) -> u32 {
        (self.value.read_bits() & Self::TIM1RM_MASK) >> Self::TIM1RM_OFFSET
    }

    /// Write **TIM1RM** `[17:15]` (RW)
    ///
    /// TIM1 remapping
    #[inline(always)]
    pub fn write_tim1rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIM1RM_MASK) | ((value << Self::TIM1RM_OFFSET) & Self::TIM1RM_MASK)
        });
    }

    /// **USART4RM** `[14:12]` (RW)
    ///
    /// USART4 remapping
    #[inline(always)]
    pub fn usart4rm(&self) -> u32 {
        (self.value.read_bits() & Self::USART4RM_MASK) >> Self::USART4RM_OFFSET
    }

    /// Write **USART4RM** `[14:12]` (RW)
    ///
    /// USART4 remapping
    #[inline(always)]
    pub fn write_usart4rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USART4RM_MASK) | ((value << Self::USART4RM_OFFSET) & Self::USART4RM_MASK)
        });
    }

    /// **USART3RM** `[11:10]` (RW)
    ///
    /// USART3 remapping
    #[inline(always)]
    pub fn usart3rm(&self) -> u32 {
        (self.value.read_bits() & Self::USART3RM_MASK) >> Self::USART3RM_OFFSET
    }

    /// Write **USART3RM** `[11:10]` (RW)
    ///
    /// USART3 remapping
    #[inline(always)]
    pub fn write_usart3rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USART3RM_MASK) | ((value << Self::USART3RM_OFFSET) & Self::USART3RM_MASK)
        });
    }

    /// **USART2RM** `[9:7]` (RW)
    ///
    /// USART2 remapping
    #[inline(always)]
    pub fn usart2rm(&self) -> u32 {
        (self.value.read_bits() & Self::USART2RM_MASK) >> Self::USART2RM_OFFSET
    }

    /// Write **USART2RM** `[9:7]` (RW)
    ///
    /// USART2 remapping
    #[inline(always)]
    pub fn write_usart2rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USART2RM_MASK) | ((value << Self::USART2RM_OFFSET) & Self::USART2RM_MASK)
        });
    }

    /// **USART1RM** `[6:5]` (RW)
    ///
    /// USART1 remapping
    #[inline(always)]
    pub fn usart1rm(&self) -> u32 {
        (self.value.read_bits() & Self::USART1RM_MASK) >> Self::USART1RM_OFFSET
    }

    /// Write **USART1RM** `[6:5]` (RW)
    ///
    /// USART1 remapping
    #[inline(always)]
    pub fn write_usart1rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USART1RM_MASK) | ((value << Self::USART1RM_OFFSET) & Self::USART1RM_MASK)
        });
    }

    /// **I2C1RM** `[4:2]` (RW)
    ///
    /// I2C1 remapping
    #[inline(always)]
    pub fn i2c1rm(&self) -> u32 {
        (self.value.read_bits() & Self::I2C1RM_MASK) >> Self::I2C1RM_OFFSET
    }

    /// Write **I2C1RM** `[4:2]` (RW)
    ///
    /// I2C1 remapping
    #[inline(always)]
    pub fn write_i2c1rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::I2C1RM_MASK) | ((value << Self::I2C1RM_OFFSET) & Self::I2C1RM_MASK)
        });
    }

    /// **SPI1RM** `[1:0]` (RW)
    ///
    /// SPI1 remapping
    #[inline(always)]
    pub fn spi1rm(&self) -> u32 {
        (self.value.read_bits() & Self::SPI1RM_MASK) >> Self::SPI1RM_OFFSET
    }

    /// Write **SPI1RM** `[1:0]` (RW)
    ///
    /// SPI1 remapping
    #[inline(always)]
    pub fn write_spi1rm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPI1RM_MASK) | ((value << Self::SPI1RM_OFFSET) & Self::SPI1RM_MASK)
        });
    }
}

/// PCFR1 value type for batch read/modify/write operations
///
/// Use `pcfr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pcfr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pcfr1Val {
    bits: u32,
}

impl Pcfr1Val {
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

    /// Read **SW_CFG** `[26:24]`
    ///
    /// SW_CFG Configuration
    #[inline(always)]
    pub const fn sw_cfg(&self) -> u32 {
        (self.bits & Pcfr1::SW_CFG_MASK) >> Pcfr1::SW_CFG_OFFSET
    }

    /// Read **PLOC_RM** `[23:23]`
    ///
    /// IO2W_RM remapping
    #[inline(always)]
    pub const fn ploc_rm(&self) -> bool {
        (self.bits & Pcfr1::PLOC_RM_MASK) != 0
    }

    /// Read **TIM3RM** `[22:21]`
    ///
    /// TIM3 remapping
    #[inline(always)]
    pub const fn tim3rm(&self) -> u32 {
        (self.bits & Pcfr1::TIM3RM_MASK) >> Pcfr1::TIM3RM_OFFSET
    }

    /// Read **TIM2RM** `[20:18]`
    ///
    /// TIM2 remapping
    #[inline(always)]
    pub const fn tim2rm(&self) -> u32 {
        (self.bits & Pcfr1::TIM2RM_MASK) >> Pcfr1::TIM2RM_OFFSET
    }

    /// Read **TIM1RM** `[17:15]`
    ///
    /// TIM1 remapping
    #[inline(always)]
    pub const fn tim1rm(&self) -> u32 {
        (self.bits & Pcfr1::TIM1RM_MASK) >> Pcfr1::TIM1RM_OFFSET
    }

    /// Read **USART4RM** `[14:12]`
    ///
    /// USART4 remapping
    #[inline(always)]
    pub const fn usart4rm(&self) -> u32 {
        (self.bits & Pcfr1::USART4RM_MASK) >> Pcfr1::USART4RM_OFFSET
    }

    /// Read **USART3RM** `[11:10]`
    ///
    /// USART3 remapping
    #[inline(always)]
    pub const fn usart3rm(&self) -> u32 {
        (self.bits & Pcfr1::USART3RM_MASK) >> Pcfr1::USART3RM_OFFSET
    }

    /// Read **USART2RM** `[9:7]`
    ///
    /// USART2 remapping
    #[inline(always)]
    pub const fn usart2rm(&self) -> u32 {
        (self.bits & Pcfr1::USART2RM_MASK) >> Pcfr1::USART2RM_OFFSET
    }

    /// Read **USART1RM** `[6:5]`
    ///
    /// USART1 remapping
    #[inline(always)]
    pub const fn usart1rm(&self) -> u32 {
        (self.bits & Pcfr1::USART1RM_MASK) >> Pcfr1::USART1RM_OFFSET
    }

    /// Read **I2C1RM** `[4:2]`
    ///
    /// I2C1 remapping
    #[inline(always)]
    pub const fn i2c1rm(&self) -> u32 {
        (self.bits & Pcfr1::I2C1RM_MASK) >> Pcfr1::I2C1RM_OFFSET
    }

    /// Read **SPI1RM** `[1:0]`
    ///
    /// SPI1 remapping
    #[inline(always)]
    pub const fn spi1rm(&self) -> u32 {
        (self.bits & Pcfr1::SPI1RM_MASK) >> Pcfr1::SPI1RM_OFFSET
    }

    /// Set **SW_CFG** `[26:24]`
    ///
    /// SW_CFG Configuration
    #[inline(always)]
    pub const fn with_sw_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::SW_CFG_MASK)
                | ((value << Pcfr1::SW_CFG_OFFSET) & Pcfr1::SW_CFG_MASK),
        }
    }

    /// Set **PLOC_RM** `[23:23]`
    ///
    /// IO2W_RM remapping
    #[inline(always)]
    pub const fn with_ploc_rm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pcfr1::PLOC_RM_MASK
            } else {
                self.bits & !Pcfr1::PLOC_RM_MASK
            },
        }
    }

    /// Set **TIM3RM** `[22:21]`
    ///
    /// TIM3 remapping
    #[inline(always)]
    pub const fn with_tim3rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::TIM3RM_MASK)
                | ((value << Pcfr1::TIM3RM_OFFSET) & Pcfr1::TIM3RM_MASK),
        }
    }

    /// Set **TIM2RM** `[20:18]`
    ///
    /// TIM2 remapping
    #[inline(always)]
    pub const fn with_tim2rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::TIM2RM_MASK)
                | ((value << Pcfr1::TIM2RM_OFFSET) & Pcfr1::TIM2RM_MASK),
        }
    }

    /// Set **TIM1RM** `[17:15]`
    ///
    /// TIM1 remapping
    #[inline(always)]
    pub const fn with_tim1rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::TIM1RM_MASK)
                | ((value << Pcfr1::TIM1RM_OFFSET) & Pcfr1::TIM1RM_MASK),
        }
    }

    /// Set **USART4RM** `[14:12]`
    ///
    /// USART4 remapping
    #[inline(always)]
    pub const fn with_usart4rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::USART4RM_MASK)
                | ((value << Pcfr1::USART4RM_OFFSET) & Pcfr1::USART4RM_MASK),
        }
    }

    /// Set **USART3RM** `[11:10]`
    ///
    /// USART3 remapping
    #[inline(always)]
    pub const fn with_usart3rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::USART3RM_MASK)
                | ((value << Pcfr1::USART3RM_OFFSET) & Pcfr1::USART3RM_MASK),
        }
    }

    /// Set **USART2RM** `[9:7]`
    ///
    /// USART2 remapping
    #[inline(always)]
    pub const fn with_usart2rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::USART2RM_MASK)
                | ((value << Pcfr1::USART2RM_OFFSET) & Pcfr1::USART2RM_MASK),
        }
    }

    /// Set **USART1RM** `[6:5]`
    ///
    /// USART1 remapping
    #[inline(always)]
    pub const fn with_usart1rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::USART1RM_MASK)
                | ((value << Pcfr1::USART1RM_OFFSET) & Pcfr1::USART1RM_MASK),
        }
    }

    /// Set **I2C1RM** `[4:2]`
    ///
    /// I2C1 remapping
    #[inline(always)]
    pub const fn with_i2c1rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::I2C1RM_MASK)
                | ((value << Pcfr1::I2C1RM_OFFSET) & Pcfr1::I2C1RM_MASK),
        }
    }

    /// Set **SPI1RM** `[1:0]`
    ///
    /// SPI1 remapping
    #[inline(always)]
    pub const fn with_spi1rm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pcfr1::SPI1RM_MASK)
                | ((value << Pcfr1::SPI1RM_OFFSET) & Pcfr1::SPI1RM_MASK),
        }
    }
}

impl Default for Pcfr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pcfr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pcfr1Val {
        Pcfr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pcfr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pcfr1Val) -> Pcfr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EXTICR1 Register
///
/// External interrupt configuration register 1
/// (AFIO_EXTICR1)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EXTI15** `[31:30]` (RW)
///
///   EXTI15 configuration
///
/// - **EXTI14** `[29:28]` (RW)
///
///   EXTI14 configuration
///
/// - **EXTI13** `[27:26]` (RW)
///
///   EXTI13 configuration
///
/// - **EXTI12** `[25:24]` (RW)
///
///   EXTI12 configuration
///
/// - **EXTI11** `[23:22]` (RW)
///
///   EXTI11 configuration
///
/// - **EXTI10** `[21:20]` (RW)
///
///   EXTI10 configuration
///
/// - **EXTI9** `[19:18]` (RW)
///
///   EXTI9 configuration
///
/// - **EXTI8** `[17:16]` (RW)
///
///   EXTI8 configuration
///
/// - **EXTI7** `[15:14]` (RW)
///
///   EXTI7 configuration
///
/// - **EXTI6** `[13:12]` (RW)
///
///   EXTI6 configuration
///
/// - **EXTI5** `[11:10]` (RW)
///
///   EXTI5 configuration
///
/// - **EXTI4** `[9:8]` (RW)
///
///   EXTI4 configuration
///
/// - **EXTI3** `[7:6]` (RW)
///
///   EXTI3 configuration
///
/// - **EXTI2** `[5:4]` (RW)
///
///   EXTI2 configuration
///
/// - **EXTI1** `[3:2]` (RW)
///
///   EXTI1 configuration
///
/// - **EXTI0** `[1:0]` (RW)
///
///   EXTI0 configuration
#[repr(transparent)]
pub struct Exticr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Exticr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Exticr1 {
    // ======== Field Constants ========

    /// **EXTI15**
    ///
    /// EXTI15 configuration
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const EXTI15_OFFSET: u32 = 30;
    pub const EXTI15_MASK: u32 = 0xc0000000;

    /// **EXTI14**
    ///
    /// EXTI14 configuration
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const EXTI14_OFFSET: u32 = 28;
    pub const EXTI14_MASK: u32 = 0x30000000;

    /// **EXTI13**
    ///
    /// EXTI13 configuration
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const EXTI13_OFFSET: u32 = 26;
    pub const EXTI13_MASK: u32 = 0x0c000000;

    /// **EXTI12**
    ///
    /// EXTI12 configuration
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const EXTI12_OFFSET: u32 = 24;
    pub const EXTI12_MASK: u32 = 0x03000000;

    /// **EXTI11**
    ///
    /// EXTI11 configuration
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const EXTI11_OFFSET: u32 = 22;
    pub const EXTI11_MASK: u32 = 0x00c00000;

    /// **EXTI10**
    ///
    /// EXTI10 configuration
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const EXTI10_OFFSET: u32 = 20;
    pub const EXTI10_MASK: u32 = 0x00300000;

    /// **EXTI9**
    ///
    /// EXTI9 configuration
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const EXTI9_OFFSET: u32 = 18;
    pub const EXTI9_MASK: u32 = 0x000c0000;

    /// **EXTI8**
    ///
    /// EXTI8 configuration
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const EXTI8_OFFSET: u32 = 16;
    pub const EXTI8_MASK: u32 = 0x00030000;

    /// **EXTI7**
    ///
    /// EXTI7 configuration
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const EXTI7_OFFSET: u32 = 14;
    pub const EXTI7_MASK: u32 = 0x0000c000;

    /// **EXTI6**
    ///
    /// EXTI6 configuration
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const EXTI6_OFFSET: u32 = 12;
    pub const EXTI6_MASK: u32 = 0x00003000;

    /// **EXTI5**
    ///
    /// EXTI5 configuration
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const EXTI5_OFFSET: u32 = 10;
    pub const EXTI5_MASK: u32 = 0x00000c00;

    /// **EXTI4**
    ///
    /// EXTI4 configuration
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const EXTI4_OFFSET: u32 = 8;
    pub const EXTI4_MASK: u32 = 0x00000300;

    /// **EXTI3**
    ///
    /// EXTI3 configuration
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const EXTI3_OFFSET: u32 = 6;
    pub const EXTI3_MASK: u32 = 0x000000c0;

    /// **EXTI2**
    ///
    /// EXTI2 configuration
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const EXTI2_OFFSET: u32 = 4;
    pub const EXTI2_MASK: u32 = 0x00000030;

    /// **EXTI1**
    ///
    /// EXTI1 configuration
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EXTI1_OFFSET: u32 = 2;
    pub const EXTI1_MASK: u32 = 0x0000000c;

    /// **EXTI0**
    ///
    /// EXTI0 configuration
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EXTI0_OFFSET: u32 = 0;
    pub const EXTI0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EXTI15** `[31:30]` (RW)
    ///
    /// EXTI15 configuration
    #[inline(always)]
    pub fn exti15(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI15_MASK) >> Self::EXTI15_OFFSET
    }

    /// Write **EXTI15** `[31:30]` (RW)
    ///
    /// EXTI15 configuration
    #[inline(always)]
    pub fn write_exti15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI15_MASK) | ((value << Self::EXTI15_OFFSET) & Self::EXTI15_MASK)
        });
    }

    /// **EXTI14** `[29:28]` (RW)
    ///
    /// EXTI14 configuration
    #[inline(always)]
    pub fn exti14(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI14_MASK) >> Self::EXTI14_OFFSET
    }

    /// Write **EXTI14** `[29:28]` (RW)
    ///
    /// EXTI14 configuration
    #[inline(always)]
    pub fn write_exti14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI14_MASK) | ((value << Self::EXTI14_OFFSET) & Self::EXTI14_MASK)
        });
    }

    /// **EXTI13** `[27:26]` (RW)
    ///
    /// EXTI13 configuration
    #[inline(always)]
    pub fn exti13(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI13_MASK) >> Self::EXTI13_OFFSET
    }

    /// Write **EXTI13** `[27:26]` (RW)
    ///
    /// EXTI13 configuration
    #[inline(always)]
    pub fn write_exti13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI13_MASK) | ((value << Self::EXTI13_OFFSET) & Self::EXTI13_MASK)
        });
    }

    /// **EXTI12** `[25:24]` (RW)
    ///
    /// EXTI12 configuration
    #[inline(always)]
    pub fn exti12(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI12_MASK) >> Self::EXTI12_OFFSET
    }

    /// Write **EXTI12** `[25:24]` (RW)
    ///
    /// EXTI12 configuration
    #[inline(always)]
    pub fn write_exti12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI12_MASK) | ((value << Self::EXTI12_OFFSET) & Self::EXTI12_MASK)
        });
    }

    /// **EXTI11** `[23:22]` (RW)
    ///
    /// EXTI11 configuration
    #[inline(always)]
    pub fn exti11(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI11_MASK) >> Self::EXTI11_OFFSET
    }

    /// Write **EXTI11** `[23:22]` (RW)
    ///
    /// EXTI11 configuration
    #[inline(always)]
    pub fn write_exti11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI11_MASK) | ((value << Self::EXTI11_OFFSET) & Self::EXTI11_MASK)
        });
    }

    /// **EXTI10** `[21:20]` (RW)
    ///
    /// EXTI10 configuration
    #[inline(always)]
    pub fn exti10(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI10_MASK) >> Self::EXTI10_OFFSET
    }

    /// Write **EXTI10** `[21:20]` (RW)
    ///
    /// EXTI10 configuration
    #[inline(always)]
    pub fn write_exti10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI10_MASK) | ((value << Self::EXTI10_OFFSET) & Self::EXTI10_MASK)
        });
    }

    /// **EXTI9** `[19:18]` (RW)
    ///
    /// EXTI9 configuration
    #[inline(always)]
    pub fn exti9(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI9_MASK) >> Self::EXTI9_OFFSET
    }

    /// Write **EXTI9** `[19:18]` (RW)
    ///
    /// EXTI9 configuration
    #[inline(always)]
    pub fn write_exti9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI9_MASK) | ((value << Self::EXTI9_OFFSET) & Self::EXTI9_MASK)
        });
    }

    /// **EXTI8** `[17:16]` (RW)
    ///
    /// EXTI8 configuration
    #[inline(always)]
    pub fn exti8(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI8_MASK) >> Self::EXTI8_OFFSET
    }

    /// Write **EXTI8** `[17:16]` (RW)
    ///
    /// EXTI8 configuration
    #[inline(always)]
    pub fn write_exti8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI8_MASK) | ((value << Self::EXTI8_OFFSET) & Self::EXTI8_MASK)
        });
    }

    /// **EXTI7** `[15:14]` (RW)
    ///
    /// EXTI7 configuration
    #[inline(always)]
    pub fn exti7(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI7_MASK) >> Self::EXTI7_OFFSET
    }

    /// Write **EXTI7** `[15:14]` (RW)
    ///
    /// EXTI7 configuration
    #[inline(always)]
    pub fn write_exti7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI7_MASK) | ((value << Self::EXTI7_OFFSET) & Self::EXTI7_MASK)
        });
    }

    /// **EXTI6** `[13:12]` (RW)
    ///
    /// EXTI6 configuration
    #[inline(always)]
    pub fn exti6(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI6_MASK) >> Self::EXTI6_OFFSET
    }

    /// Write **EXTI6** `[13:12]` (RW)
    ///
    /// EXTI6 configuration
    #[inline(always)]
    pub fn write_exti6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI6_MASK) | ((value << Self::EXTI6_OFFSET) & Self::EXTI6_MASK)
        });
    }

    /// **EXTI5** `[11:10]` (RW)
    ///
    /// EXTI5 configuration
    #[inline(always)]
    pub fn exti5(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI5_MASK) >> Self::EXTI5_OFFSET
    }

    /// Write **EXTI5** `[11:10]` (RW)
    ///
    /// EXTI5 configuration
    #[inline(always)]
    pub fn write_exti5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI5_MASK) | ((value << Self::EXTI5_OFFSET) & Self::EXTI5_MASK)
        });
    }

    /// **EXTI4** `[9:8]` (RW)
    ///
    /// EXTI4 configuration
    #[inline(always)]
    pub fn exti4(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI4_MASK) >> Self::EXTI4_OFFSET
    }

    /// Write **EXTI4** `[9:8]` (RW)
    ///
    /// EXTI4 configuration
    #[inline(always)]
    pub fn write_exti4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI4_MASK) | ((value << Self::EXTI4_OFFSET) & Self::EXTI4_MASK)
        });
    }

    /// **EXTI3** `[7:6]` (RW)
    ///
    /// EXTI3 configuration
    #[inline(always)]
    pub fn exti3(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI3_MASK) >> Self::EXTI3_OFFSET
    }

    /// Write **EXTI3** `[7:6]` (RW)
    ///
    /// EXTI3 configuration
    #[inline(always)]
    pub fn write_exti3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI3_MASK) | ((value << Self::EXTI3_OFFSET) & Self::EXTI3_MASK)
        });
    }

    /// **EXTI2** `[5:4]` (RW)
    ///
    /// EXTI2 configuration
    #[inline(always)]
    pub fn exti2(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI2_MASK) >> Self::EXTI2_OFFSET
    }

    /// Write **EXTI2** `[5:4]` (RW)
    ///
    /// EXTI2 configuration
    #[inline(always)]
    pub fn write_exti2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI2_MASK) | ((value << Self::EXTI2_OFFSET) & Self::EXTI2_MASK)
        });
    }

    /// **EXTI1** `[3:2]` (RW)
    ///
    /// EXTI1 configuration
    #[inline(always)]
    pub fn exti1(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI1_MASK) >> Self::EXTI1_OFFSET
    }

    /// Write **EXTI1** `[3:2]` (RW)
    ///
    /// EXTI1 configuration
    #[inline(always)]
    pub fn write_exti1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI1_MASK) | ((value << Self::EXTI1_OFFSET) & Self::EXTI1_MASK)
        });
    }

    /// **EXTI0** `[1:0]` (RW)
    ///
    /// EXTI0 configuration
    #[inline(always)]
    pub fn exti0(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI0_MASK) >> Self::EXTI0_OFFSET
    }

    /// Write **EXTI0** `[1:0]` (RW)
    ///
    /// EXTI0 configuration
    #[inline(always)]
    pub fn write_exti0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI0_MASK) | ((value << Self::EXTI0_OFFSET) & Self::EXTI0_MASK)
        });
    }
}

/// EXTICR1 value type for batch read/modify/write operations
///
/// Use `exticr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `exticr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Exticr1Val {
    bits: u32,
}

impl Exticr1Val {
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

    /// Read **EXTI15** `[31:30]`
    ///
    /// EXTI15 configuration
    #[inline(always)]
    pub const fn exti15(&self) -> u32 {
        (self.bits & Exticr1::EXTI15_MASK) >> Exticr1::EXTI15_OFFSET
    }

    /// Read **EXTI14** `[29:28]`
    ///
    /// EXTI14 configuration
    #[inline(always)]
    pub const fn exti14(&self) -> u32 {
        (self.bits & Exticr1::EXTI14_MASK) >> Exticr1::EXTI14_OFFSET
    }

    /// Read **EXTI13** `[27:26]`
    ///
    /// EXTI13 configuration
    #[inline(always)]
    pub const fn exti13(&self) -> u32 {
        (self.bits & Exticr1::EXTI13_MASK) >> Exticr1::EXTI13_OFFSET
    }

    /// Read **EXTI12** `[25:24]`
    ///
    /// EXTI12 configuration
    #[inline(always)]
    pub const fn exti12(&self) -> u32 {
        (self.bits & Exticr1::EXTI12_MASK) >> Exticr1::EXTI12_OFFSET
    }

    /// Read **EXTI11** `[23:22]`
    ///
    /// EXTI11 configuration
    #[inline(always)]
    pub const fn exti11(&self) -> u32 {
        (self.bits & Exticr1::EXTI11_MASK) >> Exticr1::EXTI11_OFFSET
    }

    /// Read **EXTI10** `[21:20]`
    ///
    /// EXTI10 configuration
    #[inline(always)]
    pub const fn exti10(&self) -> u32 {
        (self.bits & Exticr1::EXTI10_MASK) >> Exticr1::EXTI10_OFFSET
    }

    /// Read **EXTI9** `[19:18]`
    ///
    /// EXTI9 configuration
    #[inline(always)]
    pub const fn exti9(&self) -> u32 {
        (self.bits & Exticr1::EXTI9_MASK) >> Exticr1::EXTI9_OFFSET
    }

    /// Read **EXTI8** `[17:16]`
    ///
    /// EXTI8 configuration
    #[inline(always)]
    pub const fn exti8(&self) -> u32 {
        (self.bits & Exticr1::EXTI8_MASK) >> Exticr1::EXTI8_OFFSET
    }

    /// Read **EXTI7** `[15:14]`
    ///
    /// EXTI7 configuration
    #[inline(always)]
    pub const fn exti7(&self) -> u32 {
        (self.bits & Exticr1::EXTI7_MASK) >> Exticr1::EXTI7_OFFSET
    }

    /// Read **EXTI6** `[13:12]`
    ///
    /// EXTI6 configuration
    #[inline(always)]
    pub const fn exti6(&self) -> u32 {
        (self.bits & Exticr1::EXTI6_MASK) >> Exticr1::EXTI6_OFFSET
    }

    /// Read **EXTI5** `[11:10]`
    ///
    /// EXTI5 configuration
    #[inline(always)]
    pub const fn exti5(&self) -> u32 {
        (self.bits & Exticr1::EXTI5_MASK) >> Exticr1::EXTI5_OFFSET
    }

    /// Read **EXTI4** `[9:8]`
    ///
    /// EXTI4 configuration
    #[inline(always)]
    pub const fn exti4(&self) -> u32 {
        (self.bits & Exticr1::EXTI4_MASK) >> Exticr1::EXTI4_OFFSET
    }

    /// Read **EXTI3** `[7:6]`
    ///
    /// EXTI3 configuration
    #[inline(always)]
    pub const fn exti3(&self) -> u32 {
        (self.bits & Exticr1::EXTI3_MASK) >> Exticr1::EXTI3_OFFSET
    }

    /// Read **EXTI2** `[5:4]`
    ///
    /// EXTI2 configuration
    #[inline(always)]
    pub const fn exti2(&self) -> u32 {
        (self.bits & Exticr1::EXTI2_MASK) >> Exticr1::EXTI2_OFFSET
    }

    /// Read **EXTI1** `[3:2]`
    ///
    /// EXTI1 configuration
    #[inline(always)]
    pub const fn exti1(&self) -> u32 {
        (self.bits & Exticr1::EXTI1_MASK) >> Exticr1::EXTI1_OFFSET
    }

    /// Read **EXTI0** `[1:0]`
    ///
    /// EXTI0 configuration
    #[inline(always)]
    pub const fn exti0(&self) -> u32 {
        (self.bits & Exticr1::EXTI0_MASK) >> Exticr1::EXTI0_OFFSET
    }

    /// Set **EXTI15** `[31:30]`
    ///
    /// EXTI15 configuration
    #[inline(always)]
    pub const fn with_exti15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI15_MASK)
                | ((value << Exticr1::EXTI15_OFFSET) & Exticr1::EXTI15_MASK),
        }
    }

    /// Set **EXTI14** `[29:28]`
    ///
    /// EXTI14 configuration
    #[inline(always)]
    pub const fn with_exti14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI14_MASK)
                | ((value << Exticr1::EXTI14_OFFSET) & Exticr1::EXTI14_MASK),
        }
    }

    /// Set **EXTI13** `[27:26]`
    ///
    /// EXTI13 configuration
    #[inline(always)]
    pub const fn with_exti13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI13_MASK)
                | ((value << Exticr1::EXTI13_OFFSET) & Exticr1::EXTI13_MASK),
        }
    }

    /// Set **EXTI12** `[25:24]`
    ///
    /// EXTI12 configuration
    #[inline(always)]
    pub const fn with_exti12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI12_MASK)
                | ((value << Exticr1::EXTI12_OFFSET) & Exticr1::EXTI12_MASK),
        }
    }

    /// Set **EXTI11** `[23:22]`
    ///
    /// EXTI11 configuration
    #[inline(always)]
    pub const fn with_exti11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI11_MASK)
                | ((value << Exticr1::EXTI11_OFFSET) & Exticr1::EXTI11_MASK),
        }
    }

    /// Set **EXTI10** `[21:20]`
    ///
    /// EXTI10 configuration
    #[inline(always)]
    pub const fn with_exti10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI10_MASK)
                | ((value << Exticr1::EXTI10_OFFSET) & Exticr1::EXTI10_MASK),
        }
    }

    /// Set **EXTI9** `[19:18]`
    ///
    /// EXTI9 configuration
    #[inline(always)]
    pub const fn with_exti9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI9_MASK)
                | ((value << Exticr1::EXTI9_OFFSET) & Exticr1::EXTI9_MASK),
        }
    }

    /// Set **EXTI8** `[17:16]`
    ///
    /// EXTI8 configuration
    #[inline(always)]
    pub const fn with_exti8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI8_MASK)
                | ((value << Exticr1::EXTI8_OFFSET) & Exticr1::EXTI8_MASK),
        }
    }

    /// Set **EXTI7** `[15:14]`
    ///
    /// EXTI7 configuration
    #[inline(always)]
    pub const fn with_exti7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI7_MASK)
                | ((value << Exticr1::EXTI7_OFFSET) & Exticr1::EXTI7_MASK),
        }
    }

    /// Set **EXTI6** `[13:12]`
    ///
    /// EXTI6 configuration
    #[inline(always)]
    pub const fn with_exti6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI6_MASK)
                | ((value << Exticr1::EXTI6_OFFSET) & Exticr1::EXTI6_MASK),
        }
    }

    /// Set **EXTI5** `[11:10]`
    ///
    /// EXTI5 configuration
    #[inline(always)]
    pub const fn with_exti5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI5_MASK)
                | ((value << Exticr1::EXTI5_OFFSET) & Exticr1::EXTI5_MASK),
        }
    }

    /// Set **EXTI4** `[9:8]`
    ///
    /// EXTI4 configuration
    #[inline(always)]
    pub const fn with_exti4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI4_MASK)
                | ((value << Exticr1::EXTI4_OFFSET) & Exticr1::EXTI4_MASK),
        }
    }

    /// Set **EXTI3** `[7:6]`
    ///
    /// EXTI3 configuration
    #[inline(always)]
    pub const fn with_exti3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI3_MASK)
                | ((value << Exticr1::EXTI3_OFFSET) & Exticr1::EXTI3_MASK),
        }
    }

    /// Set **EXTI2** `[5:4]`
    ///
    /// EXTI2 configuration
    #[inline(always)]
    pub const fn with_exti2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI2_MASK)
                | ((value << Exticr1::EXTI2_OFFSET) & Exticr1::EXTI2_MASK),
        }
    }

    /// Set **EXTI1** `[3:2]`
    ///
    /// EXTI1 configuration
    #[inline(always)]
    pub const fn with_exti1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI1_MASK)
                | ((value << Exticr1::EXTI1_OFFSET) & Exticr1::EXTI1_MASK),
        }
    }

    /// Set **EXTI0** `[1:0]`
    ///
    /// EXTI0 configuration
    #[inline(always)]
    pub const fn with_exti0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr1::EXTI0_MASK)
                | ((value << Exticr1::EXTI0_OFFSET) & Exticr1::EXTI0_MASK),
        }
    }
}

impl Default for Exticr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Exticr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Exticr1Val {
        Exticr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Exticr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Exticr1Val) -> Exticr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EXTICR2 Register
///
/// External interrupt configuration register 2
/// (AFIO_EXTICR2)
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EXTI23** `[15:14]` (RW)
///
///   EXTI23 configuration
///
/// - **EXTI22** `[13:12]` (RW)
///
///   EXTI22 configuration
///
/// - **EXTI21** `[11:10]` (RW)
///
///   EXTI21 configuration
///
/// - **EXTI20** `[9:8]` (RW)
///
///   EXTI20 configuration
///
/// - **EXTI19** `[7:6]` (RW)
///
///   EXTI19 configuration
///
/// - **EXTI18** `[5:4]` (RW)
///
///   EXTI18 configuration
///
/// - **EXTI17** `[3:2]` (RW)
///
///   EXTI17 configuration
///
/// - **EXTI16** `[1:0]` (RW)
///
///   EXTI16 configuration
#[repr(transparent)]
pub struct Exticr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Exticr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Exticr2 {
    // ======== Field Constants ========

    /// **EXTI23**
    ///
    /// EXTI23 configuration
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const EXTI23_OFFSET: u32 = 14;
    pub const EXTI23_MASK: u32 = 0x0000c000;

    /// **EXTI22**
    ///
    /// EXTI22 configuration
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const EXTI22_OFFSET: u32 = 12;
    pub const EXTI22_MASK: u32 = 0x00003000;

    /// **EXTI21**
    ///
    /// EXTI21 configuration
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const EXTI21_OFFSET: u32 = 10;
    pub const EXTI21_MASK: u32 = 0x00000c00;

    /// **EXTI20**
    ///
    /// EXTI20 configuration
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const EXTI20_OFFSET: u32 = 8;
    pub const EXTI20_MASK: u32 = 0x00000300;

    /// **EXTI19**
    ///
    /// EXTI19 configuration
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const EXTI19_OFFSET: u32 = 6;
    pub const EXTI19_MASK: u32 = 0x000000c0;

    /// **EXTI18**
    ///
    /// EXTI18 configuration
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const EXTI18_OFFSET: u32 = 4;
    pub const EXTI18_MASK: u32 = 0x00000030;

    /// **EXTI17**
    ///
    /// EXTI17 configuration
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EXTI17_OFFSET: u32 = 2;
    pub const EXTI17_MASK: u32 = 0x0000000c;

    /// **EXTI16**
    ///
    /// EXTI16 configuration
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EXTI16_OFFSET: u32 = 0;
    pub const EXTI16_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EXTI23** `[15:14]` (RW)
    ///
    /// EXTI23 configuration
    #[inline(always)]
    pub fn exti23(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI23_MASK) >> Self::EXTI23_OFFSET
    }

    /// Write **EXTI23** `[15:14]` (RW)
    ///
    /// EXTI23 configuration
    #[inline(always)]
    pub fn write_exti23(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI23_MASK) | ((value << Self::EXTI23_OFFSET) & Self::EXTI23_MASK)
        });
    }

    /// **EXTI22** `[13:12]` (RW)
    ///
    /// EXTI22 configuration
    #[inline(always)]
    pub fn exti22(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI22_MASK) >> Self::EXTI22_OFFSET
    }

    /// Write **EXTI22** `[13:12]` (RW)
    ///
    /// EXTI22 configuration
    #[inline(always)]
    pub fn write_exti22(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI22_MASK) | ((value << Self::EXTI22_OFFSET) & Self::EXTI22_MASK)
        });
    }

    /// **EXTI21** `[11:10]` (RW)
    ///
    /// EXTI21 configuration
    #[inline(always)]
    pub fn exti21(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI21_MASK) >> Self::EXTI21_OFFSET
    }

    /// Write **EXTI21** `[11:10]` (RW)
    ///
    /// EXTI21 configuration
    #[inline(always)]
    pub fn write_exti21(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI21_MASK) | ((value << Self::EXTI21_OFFSET) & Self::EXTI21_MASK)
        });
    }

    /// **EXTI20** `[9:8]` (RW)
    ///
    /// EXTI20 configuration
    #[inline(always)]
    pub fn exti20(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI20_MASK) >> Self::EXTI20_OFFSET
    }

    /// Write **EXTI20** `[9:8]` (RW)
    ///
    /// EXTI20 configuration
    #[inline(always)]
    pub fn write_exti20(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI20_MASK) | ((value << Self::EXTI20_OFFSET) & Self::EXTI20_MASK)
        });
    }

    /// **EXTI19** `[7:6]` (RW)
    ///
    /// EXTI19 configuration
    #[inline(always)]
    pub fn exti19(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI19_MASK) >> Self::EXTI19_OFFSET
    }

    /// Write **EXTI19** `[7:6]` (RW)
    ///
    /// EXTI19 configuration
    #[inline(always)]
    pub fn write_exti19(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI19_MASK) | ((value << Self::EXTI19_OFFSET) & Self::EXTI19_MASK)
        });
    }

    /// **EXTI18** `[5:4]` (RW)
    ///
    /// EXTI18 configuration
    #[inline(always)]
    pub fn exti18(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI18_MASK) >> Self::EXTI18_OFFSET
    }

    /// Write **EXTI18** `[5:4]` (RW)
    ///
    /// EXTI18 configuration
    #[inline(always)]
    pub fn write_exti18(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI18_MASK) | ((value << Self::EXTI18_OFFSET) & Self::EXTI18_MASK)
        });
    }

    /// **EXTI17** `[3:2]` (RW)
    ///
    /// EXTI17 configuration
    #[inline(always)]
    pub fn exti17(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI17_MASK) >> Self::EXTI17_OFFSET
    }

    /// Write **EXTI17** `[3:2]` (RW)
    ///
    /// EXTI17 configuration
    #[inline(always)]
    pub fn write_exti17(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI17_MASK) | ((value << Self::EXTI17_OFFSET) & Self::EXTI17_MASK)
        });
    }

    /// **EXTI16** `[1:0]` (RW)
    ///
    /// EXTI16 configuration
    #[inline(always)]
    pub fn exti16(&self) -> u32 {
        (self.value.read_bits() & Self::EXTI16_MASK) >> Self::EXTI16_OFFSET
    }

    /// Write **EXTI16** `[1:0]` (RW)
    ///
    /// EXTI16 configuration
    #[inline(always)]
    pub fn write_exti16(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTI16_MASK) | ((value << Self::EXTI16_OFFSET) & Self::EXTI16_MASK)
        });
    }
}

/// EXTICR2 value type for batch read/modify/write operations
///
/// Use `exticr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `exticr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Exticr2Val {
    bits: u32,
}

impl Exticr2Val {
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

    /// Read **EXTI23** `[15:14]`
    ///
    /// EXTI23 configuration
    #[inline(always)]
    pub const fn exti23(&self) -> u32 {
        (self.bits & Exticr2::EXTI23_MASK) >> Exticr2::EXTI23_OFFSET
    }

    /// Read **EXTI22** `[13:12]`
    ///
    /// EXTI22 configuration
    #[inline(always)]
    pub const fn exti22(&self) -> u32 {
        (self.bits & Exticr2::EXTI22_MASK) >> Exticr2::EXTI22_OFFSET
    }

    /// Read **EXTI21** `[11:10]`
    ///
    /// EXTI21 configuration
    #[inline(always)]
    pub const fn exti21(&self) -> u32 {
        (self.bits & Exticr2::EXTI21_MASK) >> Exticr2::EXTI21_OFFSET
    }

    /// Read **EXTI20** `[9:8]`
    ///
    /// EXTI20 configuration
    #[inline(always)]
    pub const fn exti20(&self) -> u32 {
        (self.bits & Exticr2::EXTI20_MASK) >> Exticr2::EXTI20_OFFSET
    }

    /// Read **EXTI19** `[7:6]`
    ///
    /// EXTI19 configuration
    #[inline(always)]
    pub const fn exti19(&self) -> u32 {
        (self.bits & Exticr2::EXTI19_MASK) >> Exticr2::EXTI19_OFFSET
    }

    /// Read **EXTI18** `[5:4]`
    ///
    /// EXTI18 configuration
    #[inline(always)]
    pub const fn exti18(&self) -> u32 {
        (self.bits & Exticr2::EXTI18_MASK) >> Exticr2::EXTI18_OFFSET
    }

    /// Read **EXTI17** `[3:2]`
    ///
    /// EXTI17 configuration
    #[inline(always)]
    pub const fn exti17(&self) -> u32 {
        (self.bits & Exticr2::EXTI17_MASK) >> Exticr2::EXTI17_OFFSET
    }

    /// Read **EXTI16** `[1:0]`
    ///
    /// EXTI16 configuration
    #[inline(always)]
    pub const fn exti16(&self) -> u32 {
        (self.bits & Exticr2::EXTI16_MASK) >> Exticr2::EXTI16_OFFSET
    }

    /// Set **EXTI23** `[15:14]`
    ///
    /// EXTI23 configuration
    #[inline(always)]
    pub const fn with_exti23(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI23_MASK)
                | ((value << Exticr2::EXTI23_OFFSET) & Exticr2::EXTI23_MASK),
        }
    }

    /// Set **EXTI22** `[13:12]`
    ///
    /// EXTI22 configuration
    #[inline(always)]
    pub const fn with_exti22(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI22_MASK)
                | ((value << Exticr2::EXTI22_OFFSET) & Exticr2::EXTI22_MASK),
        }
    }

    /// Set **EXTI21** `[11:10]`
    ///
    /// EXTI21 configuration
    #[inline(always)]
    pub const fn with_exti21(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI21_MASK)
                | ((value << Exticr2::EXTI21_OFFSET) & Exticr2::EXTI21_MASK),
        }
    }

    /// Set **EXTI20** `[9:8]`
    ///
    /// EXTI20 configuration
    #[inline(always)]
    pub const fn with_exti20(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI20_MASK)
                | ((value << Exticr2::EXTI20_OFFSET) & Exticr2::EXTI20_MASK),
        }
    }

    /// Set **EXTI19** `[7:6]`
    ///
    /// EXTI19 configuration
    #[inline(always)]
    pub const fn with_exti19(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI19_MASK)
                | ((value << Exticr2::EXTI19_OFFSET) & Exticr2::EXTI19_MASK),
        }
    }

    /// Set **EXTI18** `[5:4]`
    ///
    /// EXTI18 configuration
    #[inline(always)]
    pub const fn with_exti18(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI18_MASK)
                | ((value << Exticr2::EXTI18_OFFSET) & Exticr2::EXTI18_MASK),
        }
    }

    /// Set **EXTI17** `[3:2]`
    ///
    /// EXTI17 configuration
    #[inline(always)]
    pub const fn with_exti17(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI17_MASK)
                | ((value << Exticr2::EXTI17_OFFSET) & Exticr2::EXTI17_MASK),
        }
    }

    /// Set **EXTI16** `[1:0]`
    ///
    /// EXTI16 configuration
    #[inline(always)]
    pub const fn with_exti16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Exticr2::EXTI16_MASK)
                | ((value << Exticr2::EXTI16_OFFSET) & Exticr2::EXTI16_MASK),
        }
    }
}

impl Default for Exticr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Exticr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Exticr2Val {
        Exticr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Exticr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Exticr2Val) -> Exticr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTLR Register
///
/// CTLR
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PB6_FILT_EN** `[27]` (RW)
///
///   PB6_FILT_EN configuration
///
/// - **PB5_FILT_EN** `[26]` (RW)
///
///   PB5_FILT_EN configuration
///
/// - **PA4_FILT_EN** `[25]` (RW)
///
///   PA4_FILT_EN configuration
///
/// - **PA3_FILT_EN** `[24]` (RW)
///
///   PA3_FILT_EN configuration
///
/// - **UDM_BC_CMPO** `[19]` (RO)
///
///   UDM_BC_CMPO configuration
///
/// - **UDP_BC_CMPO** `[18]` (RO)
///
///   UDP_BC_CMPO configuration
///
/// - **UDM_BC_VSRC** `[17]` (RW)
///
///   UDM_BC_VSRC configuration
///
/// - **UDP_BC_VSRC** `[16]` (RW)
///
///   UDP_BC_VSRC configuration
///
/// - **USBPD_IN_HVT** `[9]` (RW)
///
///   USBPD_IN_HVT configuration
///
/// - **USBPD_PHY_V33** `[8]` (RW)
///
///   USBPD_PHY_V33 configuration
///
/// - **USB_IOEN** `[7]` (RW)
///
///   USB_IOEN configuration
///
/// - **USB_PHY_V33** `[6]` (RW)
///
///   USB_PHY_V33 configuration
///
/// - **UDP_PUE** `[3:2]` (RW)
///
///   UDP_PUE configuration
///
/// - **UDM_PUE** `[1:0]` (RW)
///
///   UDM_PUE configuration
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

    /// **PB6_FILT_EN**
    ///
    /// PB6_FILT_EN configuration
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const PB6_FILT_EN_OFFSET: u32 = 27;
    pub const PB6_FILT_EN_MASK: u32 = 0x08000000;
    pub const PB6_FILT_EN: u32 = 1 << 27;

    /// **PB5_FILT_EN**
    ///
    /// PB5_FILT_EN configuration
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const PB5_FILT_EN_OFFSET: u32 = 26;
    pub const PB5_FILT_EN_MASK: u32 = 0x04000000;
    pub const PB5_FILT_EN: u32 = 1 << 26;

    /// **PA4_FILT_EN**
    ///
    /// PA4_FILT_EN configuration
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const PA4_FILT_EN_OFFSET: u32 = 25;
    pub const PA4_FILT_EN_MASK: u32 = 0x02000000;
    pub const PA4_FILT_EN: u32 = 1 << 25;

    /// **PA3_FILT_EN**
    ///
    /// PA3_FILT_EN configuration
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PA3_FILT_EN_OFFSET: u32 = 24;
    pub const PA3_FILT_EN_MASK: u32 = 0x01000000;
    pub const PA3_FILT_EN: u32 = 1 << 24;

    /// **UDM_BC_CMPO**
    ///
    /// UDM_BC_CMPO configuration
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const UDM_BC_CMPO_OFFSET: u32 = 19;
    pub const UDM_BC_CMPO_MASK: u32 = 0x00080000;
    pub const UDM_BC_CMPO: u32 = 1 << 19;

    /// **UDP_BC_CMPO**
    ///
    /// UDP_BC_CMPO configuration
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const UDP_BC_CMPO_OFFSET: u32 = 18;
    pub const UDP_BC_CMPO_MASK: u32 = 0x00040000;
    pub const UDP_BC_CMPO: u32 = 1 << 18;

    /// **UDM_BC_VSRC**
    ///
    /// UDM_BC_VSRC configuration
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const UDM_BC_VSRC_OFFSET: u32 = 17;
    pub const UDM_BC_VSRC_MASK: u32 = 0x00020000;
    pub const UDM_BC_VSRC: u32 = 1 << 17;

    /// **UDP_BC_VSRC**
    ///
    /// UDP_BC_VSRC configuration
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const UDP_BC_VSRC_OFFSET: u32 = 16;
    pub const UDP_BC_VSRC_MASK: u32 = 0x00010000;
    pub const UDP_BC_VSRC: u32 = 1 << 16;

    /// **USBPD_IN_HVT**
    ///
    /// USBPD_IN_HVT configuration
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USBPD_IN_HVT_OFFSET: u32 = 9;
    pub const USBPD_IN_HVT_MASK: u32 = 0x00000200;
    pub const USBPD_IN_HVT: u32 = 1 << 9;

    /// **USBPD_PHY_V33**
    ///
    /// USBPD_PHY_V33 configuration
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USBPD_PHY_V33_OFFSET: u32 = 8;
    pub const USBPD_PHY_V33_MASK: u32 = 0x00000100;
    pub const USBPD_PHY_V33: u32 = 1 << 8;

    /// **USB_IOEN**
    ///
    /// USB_IOEN configuration
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_IOEN_OFFSET: u32 = 7;
    pub const USB_IOEN_MASK: u32 = 0x00000080;
    pub const USB_IOEN: u32 = 1 << 7;

    /// **USB_PHY_V33**
    ///
    /// USB_PHY_V33 configuration
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USB_PHY_V33_OFFSET: u32 = 6;
    pub const USB_PHY_V33_MASK: u32 = 0x00000040;
    pub const USB_PHY_V33: u32 = 1 << 6;

    /// **UDP_PUE**
    ///
    /// UDP_PUE configuration
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const UDP_PUE_OFFSET: u32 = 2;
    pub const UDP_PUE_MASK: u32 = 0x0000000c;

    /// **UDM_PUE**
    ///
    /// UDM_PUE configuration
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const UDM_PUE_OFFSET: u32 = 0;
    pub const UDM_PUE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **PB6_FILT_EN** `[27]` (RW)
    ///
    /// PB6_FILT_EN configuration
    #[inline(always)]
    pub fn pb6_filt_en(&self) -> bool {
        (self.value.read_bits() & Self::PB6_FILT_EN_MASK) != 0
    }

    /// Set PB6_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pb6_filt_en(&self) {
        self.value.modify_bits(|r| r | Self::PB6_FILT_EN_MASK);
    }

    /// Clear PB6_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pb6_filt_en(&self) {
        self.value.modify_bits(|r| r & !Self::PB6_FILT_EN_MASK);
    }

    /// **PB5_FILT_EN** `[26]` (RW)
    ///
    /// PB5_FILT_EN configuration
    #[inline(always)]
    pub fn pb5_filt_en(&self) -> bool {
        (self.value.read_bits() & Self::PB5_FILT_EN_MASK) != 0
    }

    /// Set PB5_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pb5_filt_en(&self) {
        self.value.modify_bits(|r| r | Self::PB5_FILT_EN_MASK);
    }

    /// Clear PB5_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pb5_filt_en(&self) {
        self.value.modify_bits(|r| r & !Self::PB5_FILT_EN_MASK);
    }

    /// **PA4_FILT_EN** `[25]` (RW)
    ///
    /// PA4_FILT_EN configuration
    #[inline(always)]
    pub fn pa4_filt_en(&self) -> bool {
        (self.value.read_bits() & Self::PA4_FILT_EN_MASK) != 0
    }

    /// Set PA4_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pa4_filt_en(&self) {
        self.value.modify_bits(|r| r | Self::PA4_FILT_EN_MASK);
    }

    /// Clear PA4_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pa4_filt_en(&self) {
        self.value.modify_bits(|r| r & !Self::PA4_FILT_EN_MASK);
    }

    /// **PA3_FILT_EN** `[24]` (RW)
    ///
    /// PA3_FILT_EN configuration
    #[inline(always)]
    pub fn pa3_filt_en(&self) -> bool {
        (self.value.read_bits() & Self::PA3_FILT_EN_MASK) != 0
    }

    /// Set PA3_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pa3_filt_en(&self) {
        self.value.modify_bits(|r| r | Self::PA3_FILT_EN_MASK);
    }

    /// Clear PA3_FILT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pa3_filt_en(&self) {
        self.value.modify_bits(|r| r & !Self::PA3_FILT_EN_MASK);
    }

    /// **UDM_BC_CMPO** `[19]` (RO)
    ///
    /// UDM_BC_CMPO configuration
    #[inline(always)]
    pub fn udm_bc_cmpo(&self) -> bool {
        (self.value.read_bits() & Self::UDM_BC_CMPO_MASK) != 0
    }

    /// **UDP_BC_CMPO** `[18]` (RO)
    ///
    /// UDP_BC_CMPO configuration
    #[inline(always)]
    pub fn udp_bc_cmpo(&self) -> bool {
        (self.value.read_bits() & Self::UDP_BC_CMPO_MASK) != 0
    }

    /// **UDM_BC_VSRC** `[17]` (RW)
    ///
    /// UDM_BC_VSRC configuration
    #[inline(always)]
    pub fn udm_bc_vsrc(&self) -> bool {
        (self.value.read_bits() & Self::UDM_BC_VSRC_MASK) != 0
    }

    /// Set UDM_BC_VSRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_udm_bc_vsrc(&self) {
        self.value.modify_bits(|r| r | Self::UDM_BC_VSRC_MASK);
    }

    /// Clear UDM_BC_VSRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_udm_bc_vsrc(&self) {
        self.value.modify_bits(|r| r & !Self::UDM_BC_VSRC_MASK);
    }

    /// **UDP_BC_VSRC** `[16]` (RW)
    ///
    /// UDP_BC_VSRC configuration
    #[inline(always)]
    pub fn udp_bc_vsrc(&self) -> bool {
        (self.value.read_bits() & Self::UDP_BC_VSRC_MASK) != 0
    }

    /// Set UDP_BC_VSRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_udp_bc_vsrc(&self) {
        self.value.modify_bits(|r| r | Self::UDP_BC_VSRC_MASK);
    }

    /// Clear UDP_BC_VSRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_udp_bc_vsrc(&self) {
        self.value.modify_bits(|r| r & !Self::UDP_BC_VSRC_MASK);
    }

    /// **USBPD_IN_HVT** `[9]` (RW)
    ///
    /// USBPD_IN_HVT configuration
    #[inline(always)]
    pub fn usbpd_in_hvt(&self) -> bool {
        (self.value.read_bits() & Self::USBPD_IN_HVT_MASK) != 0
    }

    /// Set USBPD_IN_HVT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbpd_in_hvt(&self) {
        self.value.modify_bits(|r| r | Self::USBPD_IN_HVT_MASK);
    }

    /// Clear USBPD_IN_HVT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbpd_in_hvt(&self) {
        self.value.modify_bits(|r| r & !Self::USBPD_IN_HVT_MASK);
    }

    /// **USBPD_PHY_V33** `[8]` (RW)
    ///
    /// USBPD_PHY_V33 configuration
    #[inline(always)]
    pub fn usbpd_phy_v33(&self) -> bool {
        (self.value.read_bits() & Self::USBPD_PHY_V33_MASK) != 0
    }

    /// Set USBPD_PHY_V33 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbpd_phy_v33(&self) {
        self.value.modify_bits(|r| r | Self::USBPD_PHY_V33_MASK);
    }

    /// Clear USBPD_PHY_V33 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbpd_phy_v33(&self) {
        self.value.modify_bits(|r| r & !Self::USBPD_PHY_V33_MASK);
    }

    /// **USB_IOEN** `[7]` (RW)
    ///
    /// USB_IOEN configuration
    #[inline(always)]
    pub fn usb_ioen(&self) -> bool {
        (self.value.read_bits() & Self::USB_IOEN_MASK) != 0
    }

    /// Set USB_IOEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_ioen(&self) {
        self.value.modify_bits(|r| r | Self::USB_IOEN_MASK);
    }

    /// Clear USB_IOEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_ioen(&self) {
        self.value.modify_bits(|r| r & !Self::USB_IOEN_MASK);
    }

    /// **USB_PHY_V33** `[6]` (RW)
    ///
    /// USB_PHY_V33 configuration
    #[inline(always)]
    pub fn usb_phy_v33(&self) -> bool {
        (self.value.read_bits() & Self::USB_PHY_V33_MASK) != 0
    }

    /// Set USB_PHY_V33 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_phy_v33(&self) {
        self.value.modify_bits(|r| r | Self::USB_PHY_V33_MASK);
    }

    /// Clear USB_PHY_V33 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_phy_v33(&self) {
        self.value.modify_bits(|r| r & !Self::USB_PHY_V33_MASK);
    }

    /// **UDP_PUE** `[3:2]` (RW)
    ///
    /// UDP_PUE configuration
    #[inline(always)]
    pub fn udp_pue(&self) -> u32 {
        (self.value.read_bits() & Self::UDP_PUE_MASK) >> Self::UDP_PUE_OFFSET
    }

    /// Write **UDP_PUE** `[3:2]` (RW)
    ///
    /// UDP_PUE configuration
    #[inline(always)]
    pub fn write_udp_pue(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::UDP_PUE_MASK) | ((value << Self::UDP_PUE_OFFSET) & Self::UDP_PUE_MASK)
        });
    }

    /// **UDM_PUE** `[1:0]` (RW)
    ///
    /// UDM_PUE configuration
    #[inline(always)]
    pub fn udm_pue(&self) -> u32 {
        (self.value.read_bits() & Self::UDM_PUE_MASK) >> Self::UDM_PUE_OFFSET
    }

    /// Write **UDM_PUE** `[1:0]` (RW)
    ///
    /// UDM_PUE configuration
    #[inline(always)]
    pub fn write_udm_pue(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::UDM_PUE_MASK) | ((value << Self::UDM_PUE_OFFSET) & Self::UDM_PUE_MASK)
        });
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
        Self { bits: 0x00000000 }
    }

    /// Read **PB6_FILT_EN** `[27:27]`
    ///
    /// PB6_FILT_EN configuration
    #[inline(always)]
    pub const fn pb6_filt_en(&self) -> bool {
        (self.bits & Ctlr::PB6_FILT_EN_MASK) != 0
    }

    /// Read **PB5_FILT_EN** `[26:26]`
    ///
    /// PB5_FILT_EN configuration
    #[inline(always)]
    pub const fn pb5_filt_en(&self) -> bool {
        (self.bits & Ctlr::PB5_FILT_EN_MASK) != 0
    }

    /// Read **PA4_FILT_EN** `[25:25]`
    ///
    /// PA4_FILT_EN configuration
    #[inline(always)]
    pub const fn pa4_filt_en(&self) -> bool {
        (self.bits & Ctlr::PA4_FILT_EN_MASK) != 0
    }

    /// Read **PA3_FILT_EN** `[24:24]`
    ///
    /// PA3_FILT_EN configuration
    #[inline(always)]
    pub const fn pa3_filt_en(&self) -> bool {
        (self.bits & Ctlr::PA3_FILT_EN_MASK) != 0
    }

    /// Read **UDM_BC_CMPO** `[19:19]`
    ///
    /// UDM_BC_CMPO configuration
    #[inline(always)]
    pub const fn udm_bc_cmpo(&self) -> bool {
        (self.bits & Ctlr::UDM_BC_CMPO_MASK) != 0
    }

    /// Read **UDP_BC_CMPO** `[18:18]`
    ///
    /// UDP_BC_CMPO configuration
    #[inline(always)]
    pub const fn udp_bc_cmpo(&self) -> bool {
        (self.bits & Ctlr::UDP_BC_CMPO_MASK) != 0
    }

    /// Read **UDM_BC_VSRC** `[17:17]`
    ///
    /// UDM_BC_VSRC configuration
    #[inline(always)]
    pub const fn udm_bc_vsrc(&self) -> bool {
        (self.bits & Ctlr::UDM_BC_VSRC_MASK) != 0
    }

    /// Read **UDP_BC_VSRC** `[16:16]`
    ///
    /// UDP_BC_VSRC configuration
    #[inline(always)]
    pub const fn udp_bc_vsrc(&self) -> bool {
        (self.bits & Ctlr::UDP_BC_VSRC_MASK) != 0
    }

    /// Read **USBPD_IN_HVT** `[9:9]`
    ///
    /// USBPD_IN_HVT configuration
    #[inline(always)]
    pub const fn usbpd_in_hvt(&self) -> bool {
        (self.bits & Ctlr::USBPD_IN_HVT_MASK) != 0
    }

    /// Read **USBPD_PHY_V33** `[8:8]`
    ///
    /// USBPD_PHY_V33 configuration
    #[inline(always)]
    pub const fn usbpd_phy_v33(&self) -> bool {
        (self.bits & Ctlr::USBPD_PHY_V33_MASK) != 0
    }

    /// Read **USB_IOEN** `[7:7]`
    ///
    /// USB_IOEN configuration
    #[inline(always)]
    pub const fn usb_ioen(&self) -> bool {
        (self.bits & Ctlr::USB_IOEN_MASK) != 0
    }

    /// Read **USB_PHY_V33** `[6:6]`
    ///
    /// USB_PHY_V33 configuration
    #[inline(always)]
    pub const fn usb_phy_v33(&self) -> bool {
        (self.bits & Ctlr::USB_PHY_V33_MASK) != 0
    }

    /// Read **UDP_PUE** `[3:2]`
    ///
    /// UDP_PUE configuration
    #[inline(always)]
    pub const fn udp_pue(&self) -> u32 {
        (self.bits & Ctlr::UDP_PUE_MASK) >> Ctlr::UDP_PUE_OFFSET
    }

    /// Read **UDM_PUE** `[1:0]`
    ///
    /// UDM_PUE configuration
    #[inline(always)]
    pub const fn udm_pue(&self) -> u32 {
        (self.bits & Ctlr::UDM_PUE_MASK) >> Ctlr::UDM_PUE_OFFSET
    }

    /// Set **PB6_FILT_EN** `[27:27]`
    ///
    /// PB6_FILT_EN configuration
    #[inline(always)]
    pub const fn with_pb6_filt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::PB6_FILT_EN_MASK
            } else {
                self.bits & !Ctlr::PB6_FILT_EN_MASK
            },
        }
    }

    /// Set **PB5_FILT_EN** `[26:26]`
    ///
    /// PB5_FILT_EN configuration
    #[inline(always)]
    pub const fn with_pb5_filt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::PB5_FILT_EN_MASK
            } else {
                self.bits & !Ctlr::PB5_FILT_EN_MASK
            },
        }
    }

    /// Set **PA4_FILT_EN** `[25:25]`
    ///
    /// PA4_FILT_EN configuration
    #[inline(always)]
    pub const fn with_pa4_filt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::PA4_FILT_EN_MASK
            } else {
                self.bits & !Ctlr::PA4_FILT_EN_MASK
            },
        }
    }

    /// Set **PA3_FILT_EN** `[24:24]`
    ///
    /// PA3_FILT_EN configuration
    #[inline(always)]
    pub const fn with_pa3_filt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::PA3_FILT_EN_MASK
            } else {
                self.bits & !Ctlr::PA3_FILT_EN_MASK
            },
        }
    }

    /// Set **UDM_BC_VSRC** `[17:17]`
    ///
    /// UDM_BC_VSRC configuration
    #[inline(always)]
    pub const fn with_udm_bc_vsrc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::UDM_BC_VSRC_MASK
            } else {
                self.bits & !Ctlr::UDM_BC_VSRC_MASK
            },
        }
    }

    /// Set **UDP_BC_VSRC** `[16:16]`
    ///
    /// UDP_BC_VSRC configuration
    #[inline(always)]
    pub const fn with_udp_bc_vsrc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::UDP_BC_VSRC_MASK
            } else {
                self.bits & !Ctlr::UDP_BC_VSRC_MASK
            },
        }
    }

    /// Set **USBPD_IN_HVT** `[9:9]`
    ///
    /// USBPD_IN_HVT configuration
    #[inline(always)]
    pub const fn with_usbpd_in_hvt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::USBPD_IN_HVT_MASK
            } else {
                self.bits & !Ctlr::USBPD_IN_HVT_MASK
            },
        }
    }

    /// Set **USBPD_PHY_V33** `[8:8]`
    ///
    /// USBPD_PHY_V33 configuration
    #[inline(always)]
    pub const fn with_usbpd_phy_v33(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::USBPD_PHY_V33_MASK
            } else {
                self.bits & !Ctlr::USBPD_PHY_V33_MASK
            },
        }
    }

    /// Set **USB_IOEN** `[7:7]`
    ///
    /// USB_IOEN configuration
    #[inline(always)]
    pub const fn with_usb_ioen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::USB_IOEN_MASK
            } else {
                self.bits & !Ctlr::USB_IOEN_MASK
            },
        }
    }

    /// Set **USB_PHY_V33** `[6:6]`
    ///
    /// USB_PHY_V33 configuration
    #[inline(always)]
    pub const fn with_usb_phy_v33(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::USB_PHY_V33_MASK
            } else {
                self.bits & !Ctlr::USB_PHY_V33_MASK
            },
        }
    }

    /// Set **UDP_PUE** `[3:2]`
    ///
    /// UDP_PUE configuration
    #[inline(always)]
    pub const fn with_udp_pue(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::UDP_PUE_MASK)
                | ((value << Ctlr::UDP_PUE_OFFSET) & Ctlr::UDP_PUE_MASK),
        }
    }

    /// Set **UDM_PUE** `[1:0]`
    ///
    /// UDM_PUE configuration
    #[inline(always)]
    pub const fn with_udm_pue(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::UDM_PUE_MASK)
                | ((value << Ctlr::UDM_PUE_OFFSET) & Ctlr::UDM_PUE_MASK),
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

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4],
    /// 0x0004
    /// AF remap and debug I/O configuration
    /// register (AFIO_PCFR1)
    pub pcfr1: Pcfr1,
    /// 0x0008
    /// External interrupt configuration register 1
    /// (AFIO_EXTICR1)
    pub exticr1: Exticr1,
    /// 0x000c
    /// External interrupt configuration register 2
    /// (AFIO_EXTICR2)
    pub exticr2: Exticr2,
    _reserved1: [u8; 8],
    /// 0x0018
    /// CTLR
    pub ctlr: Ctlr,
}
