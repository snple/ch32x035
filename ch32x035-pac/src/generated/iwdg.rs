//! IWDG
//! Independent watchdog

#![allow(dead_code)]

pub const BASE: u32 = 0x40003000;

/// CTLR Register
///
/// Key register (IWDG_CTLR)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **KEY** `[15:0]` (WO)
///
///   Key value
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

    /// **KEY**
    ///
    /// Key value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY_OFFSET: u32 = 0;
    pub const KEY_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **KEY** `[15:0]` (WO)
    ///
    /// Key value
    #[inline(always)]
    pub fn write_key(&self, value: u32) {
        self.value
            .write_bits((value << Self::KEY_OFFSET) & Self::KEY_MASK);
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

    /// Set **KEY** `[15:0]`
    ///
    /// Key value
    #[inline(always)]
    pub const fn with_key(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr::KEY_MASK) | ((value << Ctlr::KEY_OFFSET) & Ctlr::KEY_MASK),
        }
    }
}

impl Default for CtlrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctlr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtlrVal) {
        self.value.write_bits(val.bits())
    }
}

/// PSCR Register
///
/// Prescaler register (IWDG_PSCR)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PR** `[2:0]` (RW)
///
///   Prescaler divider
#[repr(transparent)]
pub struct Pscr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pscr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pscr {
    // ======== Field Constants ========

    /// **PR**
    ///
    /// Prescaler divider
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const PR_OFFSET: u32 = 0;
    pub const PR_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **PR** `[2:0]` (RW)
    ///
    /// Prescaler divider
    #[inline(always)]
    pub fn pr(&self) -> u32 {
        (self.value.read_bits() & Self::PR_MASK) >> Self::PR_OFFSET
    }

    /// Write **PR** `[2:0]` (RW)
    ///
    /// Prescaler divider
    #[inline(always)]
    pub fn write_pr(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PR_MASK) | ((value << Self::PR_OFFSET) & Self::PR_MASK));
    }
}

/// PSCR value type for batch read/modify/write operations
///
/// Use `pscr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pscr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PscrVal {
    bits: u32,
}

impl PscrVal {
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

    /// Read **PR** `[2:0]`
    ///
    /// Prescaler divider
    #[inline(always)]
    pub const fn pr(&self) -> u32 {
        (self.bits & Pscr::PR_MASK) >> Pscr::PR_OFFSET
    }

    /// Set **PR** `[2:0]`
    ///
    /// Prescaler divider
    #[inline(always)]
    pub const fn with_pr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pscr::PR_MASK) | ((value << Pscr::PR_OFFSET) & Pscr::PR_MASK),
        }
    }
}

impl Default for PscrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pscr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PscrVal {
        PscrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PscrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PscrVal) -> PscrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RLDR Register
///
/// Reload register (IWDG_RLDR)
///
/// Address offset: 0x0008
/// Reset value: 0x00000fff
///
/// ## Fields
///
/// - **RL** `[11:0]` (RW)
///
///   Watchdog counter reload
///   value
#[repr(transparent)]
pub struct Rldr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rldr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rldr {
    // ======== Field Constants ========

    /// **RL**
    ///
    /// Watchdog counter reload
    /// value
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const RL_OFFSET: u32 = 0;
    pub const RL_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **RL** `[11:0]` (RW)
    ///
    /// Watchdog counter reload
    /// value
    #[inline(always)]
    pub fn rl(&self) -> u32 {
        (self.value.read_bits() & Self::RL_MASK) >> Self::RL_OFFSET
    }

    /// Write **RL** `[11:0]` (RW)
    ///
    /// Watchdog counter reload
    /// value
    #[inline(always)]
    pub fn write_rl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::RL_MASK) | ((value << Self::RL_OFFSET) & Self::RL_MASK));
    }
}

/// RLDR value type for batch read/modify/write operations
///
/// Use `rldr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rldr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RldrVal {
    bits: u32,
}

impl RldrVal {
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
        Self { bits: 0x00000fff }
    }

    /// Read **RL** `[11:0]`
    ///
    /// Watchdog counter reload
    /// value
    #[inline(always)]
    pub const fn rl(&self) -> u32 {
        (self.bits & Rldr::RL_MASK) >> Rldr::RL_OFFSET
    }

    /// Set **RL** `[11:0]`
    ///
    /// Watchdog counter reload
    /// value
    #[inline(always)]
    pub const fn with_rl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rldr::RL_MASK) | ((value << Rldr::RL_OFFSET) & Rldr::RL_MASK),
        }
    }
}

impl Default for RldrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rldr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RldrVal {
        RldrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RldrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RldrVal) -> RldrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATR Register
///
/// Status register (IWDG_SR)
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RVU** `[1]` (RO)
///
///   Watchdog counter reload value
///   update
///
/// - **PVU** `[0]` (RO)
///
///   Watchdog prescaler value
///   update
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

    /// **RVU**
    ///
    /// Watchdog counter reload value
    /// update
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RVU_OFFSET: u32 = 1;
    pub const RVU_MASK: u32 = 0x00000002;
    pub const RVU: u32 = 1 << 1;

    /// **PVU**
    ///
    /// Watchdog prescaler value
    /// update
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PVU_OFFSET: u32 = 0;
    pub const PVU_MASK: u32 = 0x00000001;
    pub const PVU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RVU** `[1]` (RO)
    ///
    /// Watchdog counter reload value
    /// update
    #[inline(always)]
    pub fn rvu(&self) -> bool {
        (self.value.read_bits() & Self::RVU_MASK) != 0
    }

    /// **PVU** `[0]` (RO)
    ///
    /// Watchdog prescaler value
    /// update
    #[inline(always)]
    pub fn pvu(&self) -> bool {
        (self.value.read_bits() & Self::PVU_MASK) != 0
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

    /// Read **RVU** `[1:1]`
    ///
    /// Watchdog counter reload value
    /// update
    #[inline(always)]
    pub const fn rvu(&self) -> bool {
        (self.bits & Statr::RVU_MASK) != 0
    }

    /// Read **PVU** `[0:0]`
    ///
    /// Watchdog prescaler value
    /// update
    #[inline(always)]
    pub const fn pvu(&self) -> bool {
        (self.bits & Statr::PVU_MASK) != 0
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
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Key register (IWDG_CTLR)
    pub ctlr: Ctlr,
    /// 0x0004
    /// Prescaler register (IWDG_PSCR)
    pub pscr: Pscr,
    /// 0x0008
    /// Reload register (IWDG_RLDR)
    pub rldr: Rldr,
    /// 0x000c
    /// Status register (IWDG_SR)
    pub statr: Statr,
}
