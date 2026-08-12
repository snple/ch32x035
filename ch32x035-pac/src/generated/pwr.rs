//! PWR
//! Power control

#![allow(dead_code)]

pub const BASE: u32 = 0x40007000;

/// CTLR Register
///
/// Power control register
/// (PWR_CTLR)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LP** `[11:10]` (RW)
///
///   LP Config
///
/// - **LP_REG** `[9]` (RW)
///
///   LP_REG enable
///
/// - **PLS** `[6:5]` (RW)
///
///   PVD Level Selection
///
/// - **PDDS** `[1]` (RW)
///
///   Power Down Deep Sleep
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

    /// **LP**
    ///
    /// LP Config
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const LP_OFFSET: u32 = 10;
    pub const LP_MASK: u32 = 0x00000c00;

    /// **LP_REG**
    ///
    /// LP_REG enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LP_REG_OFFSET: u32 = 9;
    pub const LP_REG_MASK: u32 = 0x00000200;
    pub const LP_REG: u32 = 1 << 9;

    /// **PLS**
    ///
    /// PVD Level Selection
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const PLS_OFFSET: u32 = 5;
    pub const PLS_MASK: u32 = 0x00000060;

    /// **PDDS**
    ///
    /// Power Down Deep Sleep
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PDDS_OFFSET: u32 = 1;
    pub const PDDS_MASK: u32 = 0x00000002;
    pub const PDDS: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **LP** `[11:10]` (RW)
    ///
    /// LP Config
    #[inline(always)]
    pub fn lp(&self) -> u32 {
        (self.value.read_bits() & Self::LP_MASK) >> Self::LP_OFFSET
    }

    /// Write **LP** `[11:10]` (RW)
    ///
    /// LP Config
    #[inline(always)]
    pub fn write_lp(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::LP_MASK) | ((value << Self::LP_OFFSET) & Self::LP_MASK));
    }

    /// **LP_REG** `[9]` (RW)
    ///
    /// LP_REG enable
    #[inline(always)]
    pub fn lp_reg(&self) -> bool {
        (self.value.read_bits() & Self::LP_REG_MASK) != 0
    }

    /// Set LP_REG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_reg(&self) {
        self.value.modify_bits(|r| r | Self::LP_REG_MASK);
    }

    /// Clear LP_REG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_reg(&self) {
        self.value.modify_bits(|r| r & !Self::LP_REG_MASK);
    }

    /// **PLS** `[6:5]` (RW)
    ///
    /// PVD Level Selection
    #[inline(always)]
    pub fn pls(&self) -> u32 {
        (self.value.read_bits() & Self::PLS_MASK) >> Self::PLS_OFFSET
    }

    /// Write **PLS** `[6:5]` (RW)
    ///
    /// PVD Level Selection
    #[inline(always)]
    pub fn write_pls(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PLS_MASK) | ((value << Self::PLS_OFFSET) & Self::PLS_MASK)
        });
    }

    /// **PDDS** `[1]` (RW)
    ///
    /// Power Down Deep Sleep
    #[inline(always)]
    pub fn pdds(&self) -> bool {
        (self.value.read_bits() & Self::PDDS_MASK) != 0
    }

    /// Set PDDS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pdds(&self) {
        self.value.modify_bits(|r| r | Self::PDDS_MASK);
    }

    /// Clear PDDS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pdds(&self) {
        self.value.modify_bits(|r| r & !Self::PDDS_MASK);
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

    /// Read **LP** `[11:10]`
    ///
    /// LP Config
    #[inline(always)]
    pub const fn lp(&self) -> u32 {
        (self.bits & Ctlr::LP_MASK) >> Ctlr::LP_OFFSET
    }

    /// Read **LP_REG** `[9:9]`
    ///
    /// LP_REG enable
    #[inline(always)]
    pub const fn lp_reg(&self) -> bool {
        (self.bits & Ctlr::LP_REG_MASK) != 0
    }

    /// Read **PLS** `[6:5]`
    ///
    /// PVD Level Selection
    #[inline(always)]
    pub const fn pls(&self) -> u32 {
        (self.bits & Ctlr::PLS_MASK) >> Ctlr::PLS_OFFSET
    }

    /// Read **PDDS** `[1:1]`
    ///
    /// Power Down Deep Sleep
    #[inline(always)]
    pub const fn pdds(&self) -> bool {
        (self.bits & Ctlr::PDDS_MASK) != 0
    }

    /// Set **LP** `[11:10]`
    ///
    /// LP Config
    #[inline(always)]
    pub const fn with_lp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::LP_MASK) | ((value << Ctlr::LP_OFFSET) & Ctlr::LP_MASK),
        }
    }

    /// Set **LP_REG** `[9:9]`
    ///
    /// LP_REG enable
    #[inline(always)]
    pub const fn with_lp_reg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::LP_REG_MASK
            } else {
                self.bits & !Ctlr::LP_REG_MASK
            },
        }
    }

    /// Set **PLS** `[6:5]`
    ///
    /// PVD Level Selection
    #[inline(always)]
    pub const fn with_pls(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::PLS_MASK) | ((value << Ctlr::PLS_OFFSET) & Ctlr::PLS_MASK),
        }
    }

    /// Set **PDDS** `[1:1]`
    ///
    /// Power Down Deep Sleep
    #[inline(always)]
    pub const fn with_pdds(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::PDDS_MASK
            } else {
                self.bits & !Ctlr::PDDS_MASK
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

/// CSR Register
///
/// Power control register
/// (PWR_CSR)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **Flash_ackck** `[9]` (RO)
///
///   Flash Status flag
///
/// - **PVDO** `[2]` (RO)
///
///   PVD Output
#[repr(transparent)]
pub struct Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Csr {
    // ======== Field Constants ========

    /// **Flash_ackck**
    ///
    /// Flash Status flag
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FLASH_ACKCK_OFFSET: u32 = 9;
    pub const FLASH_ACKCK_MASK: u32 = 0x00000200;
    pub const FLASH_ACKCK: u32 = 1 << 9;

    /// **PVDO**
    ///
    /// PVD Output
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PVDO_OFFSET: u32 = 2;
    pub const PVDO_MASK: u32 = 0x00000004;
    pub const PVDO: u32 = 1 << 2;

    // ======== Field Accessors ========

    /// **Flash_ackck** `[9]` (RO)
    ///
    /// Flash Status flag
    #[inline(always)]
    pub fn flash_ackck(&self) -> bool {
        (self.value.read_bits() & Self::FLASH_ACKCK_MASK) != 0
    }

    /// **PVDO** `[2]` (RO)
    ///
    /// PVD Output
    #[inline(always)]
    pub fn pvdo(&self) -> bool {
        (self.value.read_bits() & Self::PVDO_MASK) != 0
    }
}

/// CSR value type for batch read/modify/write operations
///
/// Use `csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsrVal {
    bits: u32,
}

impl CsrVal {
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

    /// Read **Flash_ackck** `[9:9]`
    ///
    /// Flash Status flag
    #[inline(always)]
    pub const fn flash_ackck(&self) -> bool {
        (self.bits & Csr::FLASH_ACKCK_MASK) != 0
    }

    /// Read **PVDO** `[2:2]`
    ///
    /// PVD Output
    #[inline(always)]
    pub const fn pvdo(&self) -> bool {
        (self.bits & Csr::PVDO_MASK) != 0
    }
}

impl Default for CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CsrVal {
        CsrVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Power control register
    /// (PWR_CTLR)
    pub ctlr: Ctlr,
    /// 0x0004
    /// Power control register
    /// (PWR_CSR)
    pub csr: Csr,
}
