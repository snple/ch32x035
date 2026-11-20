//! AWU
//! AWU configuration

#![allow(dead_code)]

pub const BASE: u32 = 0x40026400;

/// R16_AWU_CSR Register
///
/// Status Control register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AWUEN** `[1]` (RW)
///
///   AWU Enable
#[repr(transparent)]
pub struct R16AwuCsr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for R16AwuCsr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl R16AwuCsr {
    // ======== Field Constants ========

    /// **AWUEN**
    ///
    /// AWU Enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const AWUEN_OFFSET: u32 = 1;
    pub const AWUEN_MASK: u32 = 0x00000002;
    pub const AWUEN: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **AWUEN** `[1]` (RW)
    ///
    /// AWU Enable
    #[inline(always)]
    pub fn awuen(&self) -> bool {
        (self.value.read_bits() & Self::AWUEN_MASK) != 0
    }

    /// Set AWUEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_awuen(&self) {
        self.value.modify_bits(|r| r | Self::AWUEN_MASK);
    }

    /// Clear AWUEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_awuen(&self) {
        self.value.modify_bits(|r| r & !Self::AWUEN_MASK);
    }
}

/// R16_AWU_CSR value type for batch read/modify/write operations
///
/// Use `r16awucsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `r16awucsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct R16AwuCsrVal {
    bits: u32,
}

impl R16AwuCsrVal {
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

    /// Read **AWUEN** `[1:1]`
    ///
    /// AWU Enable
    #[inline(always)]
    pub const fn awuen(&self) -> bool {
        (self.bits & R16AwuCsr::AWUEN_MASK) != 0
    }

    /// Set **AWUEN** `[1:1]`
    ///
    /// AWU Enable
    #[inline(always)]
    pub const fn with_awuen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | R16AwuCsr::AWUEN_MASK
            } else {
                self.bits & !R16AwuCsr::AWUEN_MASK
            },
        }
    }
}

impl Default for R16AwuCsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl R16AwuCsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> R16AwuCsrVal {
        R16AwuCsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: R16AwuCsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(R16AwuCsrVal) -> R16AwuCsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// R16_AWU_WR Register
///
/// AWU Window register
///
/// Address offset: 0x0004
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **AWU_APR** `[5:0]` (RW)
///
///   AWU_APR value
#[repr(transparent)]
pub struct R16AwuWr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for R16AwuWr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl R16AwuWr {
    // ======== Field Constants ========

    /// **AWU_APR**
    ///
    /// AWU_APR value
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const AWU_APR_OFFSET: u32 = 0;
    pub const AWU_APR_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **AWU_APR** `[5:0]` (RW)
    ///
    /// AWU_APR value
    #[inline(always)]
    pub fn awu_apr(&self) -> u32 {
        (self.value.read_bits() & Self::AWU_APR_MASK) >> Self::AWU_APR_OFFSET
    }

    /// Write **AWU_APR** `[5:0]` (RW)
    ///
    /// AWU_APR value
    #[inline(always)]
    pub fn write_awu_apr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AWU_APR_MASK) | ((value << Self::AWU_APR_OFFSET) & Self::AWU_APR_MASK)
        });
    }
}

/// R16_AWU_WR value type for batch read/modify/write operations
///
/// Use `r16awuwr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `r16awuwr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct R16AwuWrVal {
    bits: u32,
}

impl R16AwuWrVal {
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
        Self { bits: 0x0000003f }
    }

    /// Read **AWU_APR** `[5:0]`
    ///
    /// AWU_APR value
    #[inline(always)]
    pub const fn awu_apr(&self) -> u32 {
        (self.bits & R16AwuWr::AWU_APR_MASK) >> R16AwuWr::AWU_APR_OFFSET
    }

    /// Set **AWU_APR** `[5:0]`
    ///
    /// AWU_APR value
    #[inline(always)]
    pub const fn with_awu_apr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !R16AwuWr::AWU_APR_MASK)
                | ((value << R16AwuWr::AWU_APR_OFFSET) & R16AwuWr::AWU_APR_MASK),
        }
    }
}

impl Default for R16AwuWrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl R16AwuWr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> R16AwuWrVal {
        R16AwuWrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: R16AwuWrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(R16AwuWrVal) -> R16AwuWrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// R16_AWU_PSC Register
///
/// R16_AWU_PSC
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AWU_TBR** `[3:0]` (RW)
///
///   AWU_TBR value
#[repr(transparent)]
pub struct R16AwuPsc {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for R16AwuPsc {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl R16AwuPsc {
    // ======== Field Constants ========

    /// **AWU_TBR**
    ///
    /// AWU_TBR value
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const AWU_TBR_OFFSET: u32 = 0;
    pub const AWU_TBR_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **AWU_TBR** `[3:0]` (RW)
    ///
    /// AWU_TBR value
    #[inline(always)]
    pub fn awu_tbr(&self) -> u32 {
        (self.value.read_bits() & Self::AWU_TBR_MASK) >> Self::AWU_TBR_OFFSET
    }

    /// Write **AWU_TBR** `[3:0]` (RW)
    ///
    /// AWU_TBR value
    #[inline(always)]
    pub fn write_awu_tbr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AWU_TBR_MASK) | ((value << Self::AWU_TBR_OFFSET) & Self::AWU_TBR_MASK)
        });
    }
}

/// R16_AWU_PSC value type for batch read/modify/write operations
///
/// Use `r16awupsc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `r16awupsc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct R16AwuPscVal {
    bits: u32,
}

impl R16AwuPscVal {
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

    /// Read **AWU_TBR** `[3:0]`
    ///
    /// AWU_TBR value
    #[inline(always)]
    pub const fn awu_tbr(&self) -> u32 {
        (self.bits & R16AwuPsc::AWU_TBR_MASK) >> R16AwuPsc::AWU_TBR_OFFSET
    }

    /// Set **AWU_TBR** `[3:0]`
    ///
    /// AWU_TBR value
    #[inline(always)]
    pub const fn with_awu_tbr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !R16AwuPsc::AWU_TBR_MASK)
                | ((value << R16AwuPsc::AWU_TBR_OFFSET) & R16AwuPsc::AWU_TBR_MASK),
        }
    }
}

impl Default for R16AwuPscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl R16AwuPsc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> R16AwuPscVal {
        R16AwuPscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: R16AwuPscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(R16AwuPscVal) -> R16AwuPscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Status Control register
    pub r16_awu_csr: R16AwuCsr,
    _reserved0: [u8; 2],
    /// 0x0004
    /// AWU Window register
    pub r16_awu_wr: R16AwuWr,
    _reserved1: [u8; 2],
    /// 0x0008
    /// R16_AWU_PSC
    pub r16_awu_psc: R16AwuPsc,
}
