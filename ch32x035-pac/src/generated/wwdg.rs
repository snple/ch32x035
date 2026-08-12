//! WWDG
//! Window watchdog

#![allow(dead_code)]

pub const BASE: u32 = 0x40002c00;

/// CTLR Register
///
/// Control register (WWDG_CR)
///
/// Address offset: 0x0000
/// Reset value: 0x0000007f
///
/// ## Fields
///
/// - **WDGA** `[7]` (RW)
///
///   Activation bit
///
/// - **T** `[6:0]` (RW)
///
///   7-bit counter (MSB to LSB)
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

    /// **WDGA**
    ///
    /// Activation bit
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const WDGA_OFFSET: u32 = 7;
    pub const WDGA_MASK: u32 = 0x00000080;
    pub const WDGA: u32 = 1 << 7;

    /// **T**
    ///
    /// 7-bit counter (MSB to LSB)
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const T_OFFSET: u32 = 0;
    pub const T_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **WDGA** `[7]` (RW)
    ///
    /// Activation bit
    #[inline(always)]
    pub fn wdga(&self) -> bool {
        (self.value.read_bits() & Self::WDGA_MASK) != 0
    }

    /// Set WDGA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdga(&self) {
        self.value.modify_bits(|r| r | Self::WDGA_MASK);
    }

    /// Clear WDGA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdga(&self) {
        self.value.modify_bits(|r| r & !Self::WDGA_MASK);
    }

    /// **T** `[6:0]` (RW)
    ///
    /// 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub fn t(&self) -> u32 {
        (self.value.read_bits() & Self::T_MASK) >> Self::T_OFFSET
    }

    /// Write **T** `[6:0]` (RW)
    ///
    /// 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub fn write_t(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::T_MASK) | ((value << Self::T_OFFSET) & Self::T_MASK));
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
        Self { bits: 0x0000007f }
    }

    /// Read **WDGA** `[7:7]`
    ///
    /// Activation bit
    #[inline(always)]
    pub const fn wdga(&self) -> bool {
        (self.bits & Ctlr::WDGA_MASK) != 0
    }

    /// Read **T** `[6:0]`
    ///
    /// 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub const fn t(&self) -> u32 {
        (self.bits & Ctlr::T_MASK) >> Ctlr::T_OFFSET
    }

    /// Set **WDGA** `[7:7]`
    ///
    /// Activation bit
    #[inline(always)]
    pub const fn with_wdga(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::WDGA_MASK
            } else {
                self.bits & !Ctlr::WDGA_MASK
            },
        }
    }

    /// Set **T** `[6:0]`
    ///
    /// 7-bit counter (MSB to LSB)
    #[inline(always)]
    pub const fn with_t(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::T_MASK) | ((value << Ctlr::T_OFFSET) & Ctlr::T_MASK),
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

/// CFGR Register
///
/// Configuration register
/// (WWDG_CFR)
///
/// Address offset: 0x0004
/// Reset value: 0x0000007f
///
/// ## Fields
///
/// - **EWI** `[9]` (RW)
///
///   Early Wakeup Interrupt
///
/// - **WDGTB** `[8:7]` (RW)
///
///   Timer Base
///
/// - **W** `[6:0]` (RW)
///
///   7-bit window value
#[repr(transparent)]
pub struct Cfgr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr {
    // ======== Field Constants ========

    /// **EWI**
    ///
    /// Early Wakeup Interrupt
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EWI_OFFSET: u32 = 9;
    pub const EWI_MASK: u32 = 0x00000200;
    pub const EWI: u32 = 1 << 9;

    /// **WDGTB**
    ///
    /// Timer Base
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const WDGTB_OFFSET: u32 = 7;
    pub const WDGTB_MASK: u32 = 0x00000180;

    /// **W**
    ///
    /// 7-bit window value
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const W_OFFSET: u32 = 0;
    pub const W_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EWI** `[9]` (RW)
    ///
    /// Early Wakeup Interrupt
    #[inline(always)]
    pub fn ewi(&self) -> bool {
        (self.value.read_bits() & Self::EWI_MASK) != 0
    }

    /// Set EWI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ewi(&self) {
        self.value.modify_bits(|r| r | Self::EWI_MASK);
    }

    /// Clear EWI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ewi(&self) {
        self.value.modify_bits(|r| r & !Self::EWI_MASK);
    }

    /// **WDGTB** `[8:7]` (RW)
    ///
    /// Timer Base
    #[inline(always)]
    pub fn wdgtb(&self) -> u32 {
        (self.value.read_bits() & Self::WDGTB_MASK) >> Self::WDGTB_OFFSET
    }

    /// Write **WDGTB** `[8:7]` (RW)
    ///
    /// Timer Base
    #[inline(always)]
    pub fn write_wdgtb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDGTB_MASK) | ((value << Self::WDGTB_OFFSET) & Self::WDGTB_MASK)
        });
    }

    /// **W** `[6:0]` (RW)
    ///
    /// 7-bit window value
    #[inline(always)]
    pub fn w(&self) -> u32 {
        (self.value.read_bits() & Self::W_MASK) >> Self::W_OFFSET
    }

    /// Write **W** `[6:0]` (RW)
    ///
    /// 7-bit window value
    #[inline(always)]
    pub fn write_w(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::W_MASK) | ((value << Self::W_OFFSET) & Self::W_MASK));
    }
}

/// CFGR value type for batch read/modify/write operations
///
/// Use `cfgr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CfgrVal {
    bits: u32,
}

impl CfgrVal {
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
        Self { bits: 0x0000007f }
    }

    /// Read **EWI** `[9:9]`
    ///
    /// Early Wakeup Interrupt
    #[inline(always)]
    pub const fn ewi(&self) -> bool {
        (self.bits & Cfgr::EWI_MASK) != 0
    }

    /// Read **WDGTB** `[8:7]`
    ///
    /// Timer Base
    #[inline(always)]
    pub const fn wdgtb(&self) -> u32 {
        (self.bits & Cfgr::WDGTB_MASK) >> Cfgr::WDGTB_OFFSET
    }

    /// Read **W** `[6:0]`
    ///
    /// 7-bit window value
    #[inline(always)]
    pub const fn w(&self) -> u32 {
        (self.bits & Cfgr::W_MASK) >> Cfgr::W_OFFSET
    }

    /// Set **EWI** `[9:9]`
    ///
    /// Early Wakeup Interrupt
    #[inline(always)]
    pub const fn with_ewi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr::EWI_MASK
            } else {
                self.bits & !Cfgr::EWI_MASK
            },
        }
    }

    /// Set **WDGTB** `[8:7]`
    ///
    /// Timer Base
    #[inline(always)]
    pub const fn with_wdgtb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr::WDGTB_MASK)
                | ((value << Cfgr::WDGTB_OFFSET) & Cfgr::WDGTB_MASK),
        }
    }

    /// Set **W** `[6:0]`
    ///
    /// 7-bit window value
    #[inline(always)]
    pub const fn with_w(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr::W_MASK) | ((value << Cfgr::W_OFFSET) & Cfgr::W_MASK),
        }
    }
}

impl Default for CfgrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CfgrVal {
        CfgrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CfgrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CfgrVal) -> CfgrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATR Register
///
/// Status register (WWDG_SR)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EWIF** `[0]` (RW)
///
///   Early Wakeup Interrupt Flag
#[repr(transparent)]
pub struct Statr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Statr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Statr {
    // ======== Field Constants ========

    /// **EWIF**
    ///
    /// Early Wakeup Interrupt Flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EWIF_OFFSET: u32 = 0;
    pub const EWIF_MASK: u32 = 0x00000001;
    pub const EWIF: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EWIF** `[0]` (RW)
    ///
    /// Early Wakeup Interrupt Flag
    #[inline(always)]
    pub fn ewif(&self) -> bool {
        (self.value.read_bits() & Self::EWIF_MASK) != 0
    }

    /// Set EWIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ewif(&self) {
        self.value.modify_bits(|r| r | Self::EWIF_MASK);
    }

    /// Clear EWIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ewif(&self) {
        self.value.modify_bits(|r| r & !Self::EWIF_MASK);
    }
}

/// STATR value type for batch read/modify/write operations
///
/// Use `statr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `statr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatrVal {
    bits: u32,
}

impl StatrVal {
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

    /// Read **EWIF** `[0:0]`
    ///
    /// Early Wakeup Interrupt Flag
    #[inline(always)]
    pub const fn ewif(&self) -> bool {
        (self.bits & Statr::EWIF_MASK) != 0
    }

    /// Set **EWIF** `[0:0]`
    ///
    /// Early Wakeup Interrupt Flag
    #[inline(always)]
    pub const fn with_ewif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::EWIF_MASK
            } else {
                self.bits & !Statr::EWIF_MASK
            },
        }
    }
}

impl Default for StatrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Statr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatrVal {
        StatrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StatrVal) -> StatrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control register (WWDG_CR)
    pub ctlr: Ctlr,
    /// 0x0004
    /// Configuration register
    /// (WWDG_CFR)
    pub cfgr: Cfgr,
    /// 0x0008
    /// Status register (WWDG_SR)
    pub statr: Statr,
}
