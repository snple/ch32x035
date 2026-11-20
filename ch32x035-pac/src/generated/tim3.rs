//! TIM3
//! General purpose timer

#![allow(dead_code)]

pub const BASE: u32 = 0x40000400;

/// CTLR1 Register
///
/// control register 1
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CAPLVL** `[15]` (RW)
///
///   CAPLVL
///
/// - **CAPOV** `[14]` (RW)
///
///   CAPOV
///
/// - **CKD** `[9:8]` (RW)
///
///   Clock division
///
/// - **ARPE** `[7]` (RW)
///
///   Auto-reload preload enable
///
/// - **OPM** `[3]` (RW)
///
///   One-pulse mode
///
/// - **URS** `[2]` (RW)
///
///   Update request source
///
/// - **UDIS** `[1]` (RW)
///
///   Update disable
///
/// - **CEN** `[0]` (RW)
///
///   Counter enable
#[repr(transparent)]
pub struct Ctlr1 {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Ctlr1 {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctlr1 {
    // ======== Field Constants ========

    /// **CAPLVL**
    ///
    /// CAPLVL
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CAPLVL_OFFSET: u32 = 15;
    pub const CAPLVL_MASK: u32 = 0x00008000;
    pub const CAPLVL: u32 = 1 << 15;

    /// **CAPOV**
    ///
    /// CAPOV
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CAPOV_OFFSET: u32 = 14;
    pub const CAPOV_MASK: u32 = 0x00004000;
    pub const CAPOV: u32 = 1 << 14;

    /// **CKD**
    ///
    /// Clock division
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const CKD_OFFSET: u32 = 8;
    pub const CKD_MASK: u32 = 0x00000300;

    /// **ARPE**
    ///
    /// Auto-reload preload enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ARPE_OFFSET: u32 = 7;
    pub const ARPE_MASK: u32 = 0x00000080;
    pub const ARPE: u32 = 1 << 7;

    /// **OPM**
    ///
    /// One-pulse mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OPM_OFFSET: u32 = 3;
    pub const OPM_MASK: u32 = 0x00000008;
    pub const OPM: u32 = 1 << 3;

    /// **URS**
    ///
    /// Update request source
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const URS_OFFSET: u32 = 2;
    pub const URS_MASK: u32 = 0x00000004;
    pub const URS: u32 = 1 << 2;

    /// **UDIS**
    ///
    /// Update disable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const UDIS_OFFSET: u32 = 1;
    pub const UDIS_MASK: u32 = 0x00000002;
    pub const UDIS: u32 = 1 << 1;

    /// **CEN**
    ///
    /// Counter enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CEN_OFFSET: u32 = 0;
    pub const CEN_MASK: u32 = 0x00000001;
    pub const CEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CAPLVL** `[15]` (RW)
    ///
    /// CAPLVL
    #[inline(always)]
    pub fn caplvl(&self) -> bool {
        (self.value.read_bits() & Self::CAPLVL_MASK) != 0
    }

    /// Set CAPLVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_caplvl(&self) {
        self.value.modify_bits(|r| r | Self::CAPLVL_MASK);
    }

    /// Clear CAPLVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_caplvl(&self) {
        self.value.modify_bits(|r| r & !Self::CAPLVL_MASK);
    }

    /// **CAPOV** `[14]` (RW)
    ///
    /// CAPOV
    #[inline(always)]
    pub fn capov(&self) -> bool {
        (self.value.read_bits() & Self::CAPOV_MASK) != 0
    }

    /// Set CAPOV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_capov(&self) {
        self.value.modify_bits(|r| r | Self::CAPOV_MASK);
    }

    /// Clear CAPOV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_capov(&self) {
        self.value.modify_bits(|r| r & !Self::CAPOV_MASK);
    }

    /// **CKD** `[9:8]` (RW)
    ///
    /// Clock division
    #[inline(always)]
    pub fn ckd(&self) -> u32 {
        (self.value.read_bits() & Self::CKD_MASK) >> Self::CKD_OFFSET
    }

    /// Write **CKD** `[9:8]` (RW)
    ///
    /// Clock division
    #[inline(always)]
    pub fn write_ckd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CKD_MASK) | ((value << Self::CKD_OFFSET) & Self::CKD_MASK)
        });
    }

    /// **ARPE** `[7]` (RW)
    ///
    /// Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&self) -> bool {
        (self.value.read_bits() & Self::ARPE_MASK) != 0
    }

    /// Set ARPE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arpe(&self) {
        self.value.modify_bits(|r| r | Self::ARPE_MASK);
    }

    /// Clear ARPE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arpe(&self) {
        self.value.modify_bits(|r| r & !Self::ARPE_MASK);
    }

    /// **OPM** `[3]` (RW)
    ///
    /// One-pulse mode
    #[inline(always)]
    pub fn opm(&self) -> bool {
        (self.value.read_bits() & Self::OPM_MASK) != 0
    }

    /// Set OPM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_opm(&self) {
        self.value.modify_bits(|r| r | Self::OPM_MASK);
    }

    /// Clear OPM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_opm(&self) {
        self.value.modify_bits(|r| r & !Self::OPM_MASK);
    }

    /// **URS** `[2]` (RW)
    ///
    /// Update request source
    #[inline(always)]
    pub fn urs(&self) -> bool {
        (self.value.read_bits() & Self::URS_MASK) != 0
    }

    /// Set URS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_urs(&self) {
        self.value.modify_bits(|r| r | Self::URS_MASK);
    }

    /// Clear URS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_urs(&self) {
        self.value.modify_bits(|r| r & !Self::URS_MASK);
    }

    /// **UDIS** `[1]` (RW)
    ///
    /// Update disable
    #[inline(always)]
    pub fn udis(&self) -> bool {
        (self.value.read_bits() & Self::UDIS_MASK) != 0
    }

    /// Set UDIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_udis(&self) {
        self.value.modify_bits(|r| r | Self::UDIS_MASK);
    }

    /// Clear UDIS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_udis(&self) {
        self.value.modify_bits(|r| r & !Self::UDIS_MASK);
    }

    /// **CEN** `[0]` (RW)
    ///
    /// Counter enable
    #[inline(always)]
    pub fn cen(&self) -> bool {
        (self.value.read_bits() & Self::CEN_MASK) != 0
    }

    /// Set CEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cen(&self) {
        self.value.modify_bits(|r| r | Self::CEN_MASK);
    }

    /// Clear CEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cen(&self) {
        self.value.modify_bits(|r| r & !Self::CEN_MASK);
    }
}

/// CTLR1 value type for batch read/modify/write operations
///
/// Use `ctlr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctlr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ctlr1Val {
    bits: u32,
}

impl Ctlr1Val {
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

    /// Read **CAPLVL** `[15:15]`
    ///
    /// CAPLVL
    #[inline(always)]
    pub const fn caplvl(&self) -> bool {
        (self.bits & Ctlr1::CAPLVL_MASK) != 0
    }

    /// Read **CAPOV** `[14:14]`
    ///
    /// CAPOV
    #[inline(always)]
    pub const fn capov(&self) -> bool {
        (self.bits & Ctlr1::CAPOV_MASK) != 0
    }

    /// Read **CKD** `[9:8]`
    ///
    /// Clock division
    #[inline(always)]
    pub const fn ckd(&self) -> u32 {
        (self.bits & Ctlr1::CKD_MASK) >> Ctlr1::CKD_OFFSET
    }

    /// Read **ARPE** `[7:7]`
    ///
    /// Auto-reload preload enable
    #[inline(always)]
    pub const fn arpe(&self) -> bool {
        (self.bits & Ctlr1::ARPE_MASK) != 0
    }

    /// Read **OPM** `[3:3]`
    ///
    /// One-pulse mode
    #[inline(always)]
    pub const fn opm(&self) -> bool {
        (self.bits & Ctlr1::OPM_MASK) != 0
    }

    /// Read **URS** `[2:2]`
    ///
    /// Update request source
    #[inline(always)]
    pub const fn urs(&self) -> bool {
        (self.bits & Ctlr1::URS_MASK) != 0
    }

    /// Read **UDIS** `[1:1]`
    ///
    /// Update disable
    #[inline(always)]
    pub const fn udis(&self) -> bool {
        (self.bits & Ctlr1::UDIS_MASK) != 0
    }

    /// Read **CEN** `[0:0]`
    ///
    /// Counter enable
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        (self.bits & Ctlr1::CEN_MASK) != 0
    }

    /// Set **CAPLVL** `[15:15]`
    ///
    /// CAPLVL
    #[inline(always)]
    pub const fn with_caplvl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CAPLVL_MASK
            } else {
                self.bits & !Ctlr1::CAPLVL_MASK
            },
        }
    }

    /// Set **CAPOV** `[14:14]`
    ///
    /// CAPOV
    #[inline(always)]
    pub const fn with_capov(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CAPOV_MASK
            } else {
                self.bits & !Ctlr1::CAPOV_MASK
            },
        }
    }

    /// Set **CKD** `[9:8]`
    ///
    /// Clock division
    #[inline(always)]
    pub const fn with_ckd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::CKD_MASK) | ((value << Ctlr1::CKD_OFFSET) & Ctlr1::CKD_MASK),
        }
    }

    /// Set **ARPE** `[7:7]`
    ///
    /// Auto-reload preload enable
    #[inline(always)]
    pub const fn with_arpe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::ARPE_MASK
            } else {
                self.bits & !Ctlr1::ARPE_MASK
            },
        }
    }

    /// Set **OPM** `[3:3]`
    ///
    /// One-pulse mode
    #[inline(always)]
    pub const fn with_opm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::OPM_MASK
            } else {
                self.bits & !Ctlr1::OPM_MASK
            },
        }
    }

    /// Set **URS** `[2:2]`
    ///
    /// Update request source
    #[inline(always)]
    pub const fn with_urs(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::URS_MASK
            } else {
                self.bits & !Ctlr1::URS_MASK
            },
        }
    }

    /// Set **UDIS** `[1:1]`
    ///
    /// Update disable
    #[inline(always)]
    pub const fn with_udis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::UDIS_MASK
            } else {
                self.bits & !Ctlr1::UDIS_MASK
            },
        }
    }

    /// Set **CEN** `[0:0]`
    ///
    /// Counter enable
    #[inline(always)]
    pub const fn with_cen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CEN_MASK
            } else {
                self.bits & !Ctlr1::CEN_MASK
            },
        }
    }
}

impl Default for Ctlr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctlr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ctlr1Val {
        Ctlr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ctlr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ctlr1Val) -> Ctlr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SMCFGR Register
///
/// slave mode control register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TS** `[6:4]` (RW)
///
///   Trigger selection
///
/// - **SMS** `[2:0]` (RW)
///
///   Slave mode selection
#[repr(transparent)]
pub struct Smcfgr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Smcfgr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Smcfgr {
    // ======== Field Constants ========

    /// **TS**
    ///
    /// Trigger selection
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const TS_OFFSET: u32 = 4;
    pub const TS_MASK: u32 = 0x00000070;

    /// **SMS**
    ///
    /// Slave mode selection
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SMS_OFFSET: u32 = 0;
    pub const SMS_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **TS** `[6:4]` (RW)
    ///
    /// Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> u32 {
        (self.value.read_bits() & Self::TS_MASK) >> Self::TS_OFFSET
    }

    /// Write **TS** `[6:4]` (RW)
    ///
    /// Trigger selection
    #[inline(always)]
    pub fn write_ts(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::TS_MASK) | ((value << Self::TS_OFFSET) & Self::TS_MASK));
    }

    /// **SMS** `[2:0]` (RW)
    ///
    /// Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> u32 {
        (self.value.read_bits() & Self::SMS_MASK) >> Self::SMS_OFFSET
    }

    /// Write **SMS** `[2:0]` (RW)
    ///
    /// Slave mode selection
    #[inline(always)]
    pub fn write_sms(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMS_MASK) | ((value << Self::SMS_OFFSET) & Self::SMS_MASK)
        });
    }
}

/// SMCFGR value type for batch read/modify/write operations
///
/// Use `smcfgr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `smcfgr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SmcfgrVal {
    bits: u32,
}

impl SmcfgrVal {
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

    /// Read **TS** `[6:4]`
    ///
    /// Trigger selection
    #[inline(always)]
    pub const fn ts(&self) -> u32 {
        (self.bits & Smcfgr::TS_MASK) >> Smcfgr::TS_OFFSET
    }

    /// Read **SMS** `[2:0]`
    ///
    /// Slave mode selection
    #[inline(always)]
    pub const fn sms(&self) -> u32 {
        (self.bits & Smcfgr::SMS_MASK) >> Smcfgr::SMS_OFFSET
    }

    /// Set **TS** `[6:4]`
    ///
    /// Trigger selection
    #[inline(always)]
    pub const fn with_ts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Smcfgr::TS_MASK) | ((value << Smcfgr::TS_OFFSET) & Smcfgr::TS_MASK),
        }
    }

    /// Set **SMS** `[2:0]`
    ///
    /// Slave mode selection
    #[inline(always)]
    pub const fn with_sms(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Smcfgr::SMS_MASK)
                | ((value << Smcfgr::SMS_OFFSET) & Smcfgr::SMS_MASK),
        }
    }
}

impl Default for SmcfgrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Smcfgr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SmcfgrVal {
        SmcfgrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SmcfgrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SmcfgrVal) -> SmcfgrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMAINTENR Register
///
/// DMA/Interrupt enable register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIE** `[6]` (RW)
///
///   Trigger interrupt enable
///
/// - **CC2IE** `[2]` (RW)
///
///   Capture/Compare 2 interrupt
///   enable
///
/// - **CC1IE** `[1]` (RW)
///
///   Capture/Compare 1 interrupt
///   enable
///
/// - **UIE** `[0]` (RW)
///
///   Update interrupt enable
#[repr(transparent)]
pub struct Dmaintenr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Dmaintenr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmaintenr {
    // ======== Field Constants ========

    /// **TIE**
    ///
    /// Trigger interrupt enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TIE_OFFSET: u32 = 6;
    pub const TIE_MASK: u32 = 0x00000040;
    pub const TIE: u32 = 1 << 6;

    /// **CC2IE**
    ///
    /// Capture/Compare 2 interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CC2IE_OFFSET: u32 = 2;
    pub const CC2IE_MASK: u32 = 0x00000004;
    pub const CC2IE: u32 = 1 << 2;

    /// **CC1IE**
    ///
    /// Capture/Compare 1 interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CC1IE_OFFSET: u32 = 1;
    pub const CC1IE_MASK: u32 = 0x00000002;
    pub const CC1IE: u32 = 1 << 1;

    /// **UIE**
    ///
    /// Update interrupt enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UIE_OFFSET: u32 = 0;
    pub const UIE_MASK: u32 = 0x00000001;
    pub const UIE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIE** `[6]` (RW)
    ///
    /// Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> bool {
        (self.value.read_bits() & Self::TIE_MASK) != 0
    }

    /// Set TIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tie(&self) {
        self.value.modify_bits(|r| r | Self::TIE_MASK);
    }

    /// Clear TIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tie(&self) {
        self.value.modify_bits(|r| r & !Self::TIE_MASK);
    }

    /// **CC2IE** `[2]` (RW)
    ///
    /// Capture/Compare 2 interrupt
    /// enable
    #[inline(always)]
    pub fn cc2ie(&self) -> bool {
        (self.value.read_bits() & Self::CC2IE_MASK) != 0
    }

    /// Set CC2IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2ie(&self) {
        self.value.modify_bits(|r| r | Self::CC2IE_MASK);
    }

    /// Clear CC2IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2ie(&self) {
        self.value.modify_bits(|r| r & !Self::CC2IE_MASK);
    }

    /// **CC1IE** `[1]` (RW)
    ///
    /// Capture/Compare 1 interrupt
    /// enable
    #[inline(always)]
    pub fn cc1ie(&self) -> bool {
        (self.value.read_bits() & Self::CC1IE_MASK) != 0
    }

    /// Set CC1IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1ie(&self) {
        self.value.modify_bits(|r| r | Self::CC1IE_MASK);
    }

    /// Clear CC1IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1ie(&self) {
        self.value.modify_bits(|r| r & !Self::CC1IE_MASK);
    }

    /// **UIE** `[0]` (RW)
    ///
    /// Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> bool {
        (self.value.read_bits() & Self::UIE_MASK) != 0
    }

    /// Set UIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uie(&self) {
        self.value.modify_bits(|r| r | Self::UIE_MASK);
    }

    /// Clear UIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uie(&self) {
        self.value.modify_bits(|r| r & !Self::UIE_MASK);
    }
}

/// DMAINTENR value type for batch read/modify/write operations
///
/// Use `dmaintenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaintenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaintenrVal {
    bits: u32,
}

impl DmaintenrVal {
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

    /// Read **TIE** `[6:6]`
    ///
    /// Trigger interrupt enable
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        (self.bits & Dmaintenr::TIE_MASK) != 0
    }

    /// Read **CC2IE** `[2:2]`
    ///
    /// Capture/Compare 2 interrupt
    /// enable
    #[inline(always)]
    pub const fn cc2ie(&self) -> bool {
        (self.bits & Dmaintenr::CC2IE_MASK) != 0
    }

    /// Read **CC1IE** `[1:1]`
    ///
    /// Capture/Compare 1 interrupt
    /// enable
    #[inline(always)]
    pub const fn cc1ie(&self) -> bool {
        (self.bits & Dmaintenr::CC1IE_MASK) != 0
    }

    /// Read **UIE** `[0:0]`
    ///
    /// Update interrupt enable
    #[inline(always)]
    pub const fn uie(&self) -> bool {
        (self.bits & Dmaintenr::UIE_MASK) != 0
    }

    /// Set **TIE** `[6:6]`
    ///
    /// Trigger interrupt enable
    #[inline(always)]
    pub const fn with_tie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::TIE_MASK
            } else {
                self.bits & !Dmaintenr::TIE_MASK
            },
        }
    }

    /// Set **CC2IE** `[2:2]`
    ///
    /// Capture/Compare 2 interrupt
    /// enable
    #[inline(always)]
    pub const fn with_cc2ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC2IE_MASK
            } else {
                self.bits & !Dmaintenr::CC2IE_MASK
            },
        }
    }

    /// Set **CC1IE** `[1:1]`
    ///
    /// Capture/Compare 1 interrupt
    /// enable
    #[inline(always)]
    pub const fn with_cc1ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC1IE_MASK
            } else {
                self.bits & !Dmaintenr::CC1IE_MASK
            },
        }
    }

    /// Set **UIE** `[0:0]`
    ///
    /// Update interrupt enable
    #[inline(always)]
    pub const fn with_uie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::UIE_MASK
            } else {
                self.bits & !Dmaintenr::UIE_MASK
            },
        }
    }
}

impl Default for DmaintenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmaintenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaintenrVal {
        DmaintenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaintenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaintenrVal) -> DmaintenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTFR Register
///
/// status register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CC2OF** `[10]` (WO)
///
///   Capture/compare 2 overcapture
///   flag
///
/// - **CC1OF** `[9]` (WO)
///
///   Capture/Compare 1 overcapture
///   flag
///
/// - **TIF** `[6]` (WO)
///
///   Trigger interrupt flag
///
/// - **CC2IF** `[2]` (WO)
///
///   Capture/Compare 2 interrupt
///   flag
///
/// - **CC1IF** `[1]` (WO)
///
///   Capture/compare 1 interrupt
///   flag
///
/// - **UIF** `[0]` (WO)
///
///   Update interrupt flag
#[repr(transparent)]
pub struct Intfr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Intfr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intfr {
    // ======== Field Constants ========

    /// **CC2OF**
    ///
    /// Capture/compare 2 overcapture
    /// flag
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CC2OF_OFFSET: u32 = 10;
    pub const CC2OF_MASK: u32 = 0x00000400;
    pub const CC2OF: u32 = 1 << 10;

    /// **CC1OF**
    ///
    /// Capture/Compare 1 overcapture
    /// flag
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CC1OF_OFFSET: u32 = 9;
    pub const CC1OF_MASK: u32 = 0x00000200;
    pub const CC1OF: u32 = 1 << 9;

    /// **TIF**
    ///
    /// Trigger interrupt flag
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TIF_OFFSET: u32 = 6;
    pub const TIF_MASK: u32 = 0x00000040;
    pub const TIF: u32 = 1 << 6;

    /// **CC2IF**
    ///
    /// Capture/Compare 2 interrupt
    /// flag
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CC2IF_OFFSET: u32 = 2;
    pub const CC2IF_MASK: u32 = 0x00000004;
    pub const CC2IF: u32 = 1 << 2;

    /// **CC1IF**
    ///
    /// Capture/compare 1 interrupt
    /// flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CC1IF_OFFSET: u32 = 1;
    pub const CC1IF_MASK: u32 = 0x00000002;
    pub const CC1IF: u32 = 1 << 1;

    /// **UIF**
    ///
    /// Update interrupt flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UIF_OFFSET: u32 = 0;
    pub const UIF_MASK: u32 = 0x00000001;
    pub const UIF: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CC2OF** (WO)
    ///
    /// Capture/compare 2 overcapture
    /// flag
    #[inline(always)]
    pub fn trigger_cc2of(&self) {
        self.value.write_bits(Self::CC2OF_MASK);
    }

    /// Trigger **CC1OF** (WO)
    ///
    /// Capture/Compare 1 overcapture
    /// flag
    #[inline(always)]
    pub fn trigger_cc1of(&self) {
        self.value.write_bits(Self::CC1OF_MASK);
    }

    /// Trigger **TIF** (WO)
    ///
    /// Trigger interrupt flag
    #[inline(always)]
    pub fn trigger_tif(&self) {
        self.value.write_bits(Self::TIF_MASK);
    }

    /// Trigger **CC2IF** (WO)
    ///
    /// Capture/Compare 2 interrupt
    /// flag
    #[inline(always)]
    pub fn trigger_cc2if(&self) {
        self.value.write_bits(Self::CC2IF_MASK);
    }

    /// Trigger **CC1IF** (WO)
    ///
    /// Capture/compare 1 interrupt
    /// flag
    #[inline(always)]
    pub fn trigger_cc1if(&self) {
        self.value.write_bits(Self::CC1IF_MASK);
    }

    /// Trigger **UIF** (WO)
    ///
    /// Update interrupt flag
    #[inline(always)]
    pub fn trigger_uif(&self) {
        self.value.write_bits(Self::UIF_MASK);
    }
}

/// INTFR value type for batch read/modify/write operations
///
/// Use `intfr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intfr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntfrVal {
    bits: u32,
}

impl IntfrVal {
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

    /// Set **CC2OF** `[10:10]`
    ///
    /// Capture/compare 2 overcapture
    /// flag
    #[inline(always)]
    pub const fn with_cc2of(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC2OF_MASK
            } else {
                self.bits & !Intfr::CC2OF_MASK
            },
        }
    }

    /// Set **CC1OF** `[9:9]`
    ///
    /// Capture/Compare 1 overcapture
    /// flag
    #[inline(always)]
    pub const fn with_cc1of(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC1OF_MASK
            } else {
                self.bits & !Intfr::CC1OF_MASK
            },
        }
    }

    /// Set **TIF** `[6:6]`
    ///
    /// Trigger interrupt flag
    #[inline(always)]
    pub const fn with_tif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::TIF_MASK
            } else {
                self.bits & !Intfr::TIF_MASK
            },
        }
    }

    /// Set **CC2IF** `[2:2]`
    ///
    /// Capture/Compare 2 interrupt
    /// flag
    #[inline(always)]
    pub const fn with_cc2if(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC2IF_MASK
            } else {
                self.bits & !Intfr::CC2IF_MASK
            },
        }
    }

    /// Set **CC1IF** `[1:1]`
    ///
    /// Capture/compare 1 interrupt
    /// flag
    #[inline(always)]
    pub const fn with_cc1if(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC1IF_MASK
            } else {
                self.bits & !Intfr::CC1IF_MASK
            },
        }
    }

    /// Set **UIF** `[0:0]`
    ///
    /// Update interrupt flag
    #[inline(always)]
    pub const fn with_uif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::UIF_MASK
            } else {
                self.bits & !Intfr::UIF_MASK
            },
        }
    }
}

impl Default for IntfrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intfr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntfrVal) {
        self.value.write_bits(val.bits())
    }
}

/// SWEVGR Register
///
/// event generation register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TG** `[6]` (WO)
///
///   Trigger generation
///
/// - **CC2G** `[2]` (WO)
///
///   Capture/compare 2
///   generation
///
/// - **CC1G** `[1]` (WO)
///
///   Capture/compare 1
///   generation
///
/// - **UG** `[0]` (WO)
///
///   Update generation
#[repr(transparent)]
pub struct Swevgr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Swevgr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Swevgr {
    // ======== Field Constants ========

    /// **TG**
    ///
    /// Trigger generation
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TG_OFFSET: u32 = 6;
    pub const TG_MASK: u32 = 0x00000040;
    pub const TG: u32 = 1 << 6;

    /// **CC2G**
    ///
    /// Capture/compare 2
    /// generation
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CC2G_OFFSET: u32 = 2;
    pub const CC2G_MASK: u32 = 0x00000004;
    pub const CC2G: u32 = 1 << 2;

    /// **CC1G**
    ///
    /// Capture/compare 1
    /// generation
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CC1G_OFFSET: u32 = 1;
    pub const CC1G_MASK: u32 = 0x00000002;
    pub const CC1G: u32 = 1 << 1;

    /// **UG**
    ///
    /// Update generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UG_OFFSET: u32 = 0;
    pub const UG_MASK: u32 = 0x00000001;
    pub const UG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TG** (WO)
    ///
    /// Trigger generation
    #[inline(always)]
    pub fn trigger_tg(&self) {
        self.value.write_bits(Self::TG_MASK);
    }

    /// Trigger **CC2G** (WO)
    ///
    /// Capture/compare 2
    /// generation
    #[inline(always)]
    pub fn trigger_cc2g(&self) {
        self.value.write_bits(Self::CC2G_MASK);
    }

    /// Trigger **CC1G** (WO)
    ///
    /// Capture/compare 1
    /// generation
    #[inline(always)]
    pub fn trigger_cc1g(&self) {
        self.value.write_bits(Self::CC1G_MASK);
    }

    /// Trigger **UG** (WO)
    ///
    /// Update generation
    #[inline(always)]
    pub fn trigger_ug(&self) {
        self.value.write_bits(Self::UG_MASK);
    }
}

/// SWEVGR value type for batch read/modify/write operations
///
/// Use `swevgr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swevgr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwevgrVal {
    bits: u32,
}

impl SwevgrVal {
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

    /// Set **TG** `[6:6]`
    ///
    /// Trigger generation
    #[inline(always)]
    pub const fn with_tg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::TG_MASK
            } else {
                self.bits & !Swevgr::TG_MASK
            },
        }
    }

    /// Set **CC2G** `[2:2]`
    ///
    /// Capture/compare 2
    /// generation
    #[inline(always)]
    pub const fn with_cc2g(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::CC2G_MASK
            } else {
                self.bits & !Swevgr::CC2G_MASK
            },
        }
    }

    /// Set **CC1G** `[1:1]`
    ///
    /// Capture/compare 1
    /// generation
    #[inline(always)]
    pub const fn with_cc1g(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::CC1G_MASK
            } else {
                self.bits & !Swevgr::CC1G_MASK
            },
        }
    }

    /// Set **UG** `[0:0]`
    ///
    /// Update generation
    #[inline(always)]
    pub const fn with_ug(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::UG_MASK
            } else {
                self.bits & !Swevgr::UG_MASK
            },
        }
    }
}

impl Default for SwevgrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Swevgr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwevgrVal) {
        self.value.write_bits(val.bits())
    }
}

/// CHCTLR1_Output Register
///
/// capture/compare mode register 1 (output
/// mode)
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OC2CE** `[15]` (RW)
///
///   Output compare 2 clear
///   enable
///
/// - **OC2M** `[14:12]` (RW)
///
///   Output compare 2 mode
///
/// - **OC2PE** `[11]` (RW)
///
///   Output compare 2 preload
///   enable
///
/// - **OC2FE** `[10]` (RW)
///
///   Output compare 2 fast
///   enable
///
/// - **CC2S** `[9:8]` (RW)
///
///   Capture/Compare 2
///   selection
///
/// - **OC1CE** `[7]` (RW)
///
///   Output compare 1 clear
///   enable
///
/// - **OC1M** `[6:4]` (RW)
///
///   Output compare 1 mode
///
/// - **OC1PE** `[3]` (RW)
///
///   Output compare 1 preload
///   enable
///
/// - **OC1FE** `[2]` (RW)
///
///   Output compare 1 fast
///   enable
///
/// - **CC1S** `[1:0]` (RW)
///
///   Capture/Compare 1
///   selection
#[repr(transparent)]
pub struct Chctlr1Output {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Chctlr1Output {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chctlr1Output {
    // ======== Field Constants ========

    /// **OC2CE**
    ///
    /// Output compare 2 clear
    /// enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OC2CE_OFFSET: u32 = 15;
    pub const OC2CE_MASK: u32 = 0x00008000;
    pub const OC2CE: u32 = 1 << 15;

    /// **OC2M**
    ///
    /// Output compare 2 mode
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const OC2M_OFFSET: u32 = 12;
    pub const OC2M_MASK: u32 = 0x00007000;

    /// **OC2PE**
    ///
    /// Output compare 2 preload
    /// enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OC2PE_OFFSET: u32 = 11;
    pub const OC2PE_MASK: u32 = 0x00000800;
    pub const OC2PE: u32 = 1 << 11;

    /// **OC2FE**
    ///
    /// Output compare 2 fast
    /// enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OC2FE_OFFSET: u32 = 10;
    pub const OC2FE_MASK: u32 = 0x00000400;
    pub const OC2FE: u32 = 1 << 10;

    /// **CC2S**
    ///
    /// Capture/Compare 2
    /// selection
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const CC2S_OFFSET: u32 = 8;
    pub const CC2S_MASK: u32 = 0x00000300;

    /// **OC1CE**
    ///
    /// Output compare 1 clear
    /// enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OC1CE_OFFSET: u32 = 7;
    pub const OC1CE_MASK: u32 = 0x00000080;
    pub const OC1CE: u32 = 1 << 7;

    /// **OC1M**
    ///
    /// Output compare 1 mode
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const OC1M_OFFSET: u32 = 4;
    pub const OC1M_MASK: u32 = 0x00000070;

    /// **OC1PE**
    ///
    /// Output compare 1 preload
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OC1PE_OFFSET: u32 = 3;
    pub const OC1PE_MASK: u32 = 0x00000008;
    pub const OC1PE: u32 = 1 << 3;

    /// **OC1FE**
    ///
    /// Output compare 1 fast
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OC1FE_OFFSET: u32 = 2;
    pub const OC1FE_MASK: u32 = 0x00000004;
    pub const OC1FE: u32 = 1 << 2;

    /// **CC1S**
    ///
    /// Capture/Compare 1
    /// selection
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CC1S_OFFSET: u32 = 0;
    pub const CC1S_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **OC2CE** `[15]` (RW)
    ///
    /// Output compare 2 clear
    /// enable
    #[inline(always)]
    pub fn oc2ce(&self) -> bool {
        (self.value.read_bits() & Self::OC2CE_MASK) != 0
    }

    /// Set OC2CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc2ce(&self) {
        self.value.modify_bits(|r| r | Self::OC2CE_MASK);
    }

    /// Clear OC2CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc2ce(&self) {
        self.value.modify_bits(|r| r & !Self::OC2CE_MASK);
    }

    /// **OC2M** `[14:12]` (RW)
    ///
    /// Output compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> u32 {
        (self.value.read_bits() & Self::OC2M_MASK) >> Self::OC2M_OFFSET
    }

    /// Write **OC2M** `[14:12]` (RW)
    ///
    /// Output compare 2 mode
    #[inline(always)]
    pub fn write_oc2m(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OC2M_MASK) | ((value << Self::OC2M_OFFSET) & Self::OC2M_MASK)
        });
    }

    /// **OC2PE** `[11]` (RW)
    ///
    /// Output compare 2 preload
    /// enable
    #[inline(always)]
    pub fn oc2pe(&self) -> bool {
        (self.value.read_bits() & Self::OC2PE_MASK) != 0
    }

    /// Set OC2PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc2pe(&self) {
        self.value.modify_bits(|r| r | Self::OC2PE_MASK);
    }

    /// Clear OC2PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc2pe(&self) {
        self.value.modify_bits(|r| r & !Self::OC2PE_MASK);
    }

    /// **OC2FE** `[10]` (RW)
    ///
    /// Output compare 2 fast
    /// enable
    #[inline(always)]
    pub fn oc2fe(&self) -> bool {
        (self.value.read_bits() & Self::OC2FE_MASK) != 0
    }

    /// Set OC2FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc2fe(&self) {
        self.value.modify_bits(|r| r | Self::OC2FE_MASK);
    }

    /// Clear OC2FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc2fe(&self) {
        self.value.modify_bits(|r| r & !Self::OC2FE_MASK);
    }

    /// **CC2S** `[9:8]` (RW)
    ///
    /// Capture/Compare 2
    /// selection
    #[inline(always)]
    pub fn cc2s(&self) -> u32 {
        (self.value.read_bits() & Self::CC2S_MASK) >> Self::CC2S_OFFSET
    }

    /// Write **CC2S** `[9:8]` (RW)
    ///
    /// Capture/Compare 2
    /// selection
    #[inline(always)]
    pub fn write_cc2s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC2S_MASK) | ((value << Self::CC2S_OFFSET) & Self::CC2S_MASK)
        });
    }

    /// **OC1CE** `[7]` (RW)
    ///
    /// Output compare 1 clear
    /// enable
    #[inline(always)]
    pub fn oc1ce(&self) -> bool {
        (self.value.read_bits() & Self::OC1CE_MASK) != 0
    }

    /// Set OC1CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc1ce(&self) {
        self.value.modify_bits(|r| r | Self::OC1CE_MASK);
    }

    /// Clear OC1CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc1ce(&self) {
        self.value.modify_bits(|r| r & !Self::OC1CE_MASK);
    }

    /// **OC1M** `[6:4]` (RW)
    ///
    /// Output compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> u32 {
        (self.value.read_bits() & Self::OC1M_MASK) >> Self::OC1M_OFFSET
    }

    /// Write **OC1M** `[6:4]` (RW)
    ///
    /// Output compare 1 mode
    #[inline(always)]
    pub fn write_oc1m(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OC1M_MASK) | ((value << Self::OC1M_OFFSET) & Self::OC1M_MASK)
        });
    }

    /// **OC1PE** `[3]` (RW)
    ///
    /// Output compare 1 preload
    /// enable
    #[inline(always)]
    pub fn oc1pe(&self) -> bool {
        (self.value.read_bits() & Self::OC1PE_MASK) != 0
    }

    /// Set OC1PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc1pe(&self) {
        self.value.modify_bits(|r| r | Self::OC1PE_MASK);
    }

    /// Clear OC1PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc1pe(&self) {
        self.value.modify_bits(|r| r & !Self::OC1PE_MASK);
    }

    /// **OC1FE** `[2]` (RW)
    ///
    /// Output compare 1 fast
    /// enable
    #[inline(always)]
    pub fn oc1fe(&self) -> bool {
        (self.value.read_bits() & Self::OC1FE_MASK) != 0
    }

    /// Set OC1FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc1fe(&self) {
        self.value.modify_bits(|r| r | Self::OC1FE_MASK);
    }

    /// Clear OC1FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc1fe(&self) {
        self.value.modify_bits(|r| r & !Self::OC1FE_MASK);
    }

    /// **CC1S** `[1:0]` (RW)
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub fn cc1s(&self) -> u32 {
        (self.value.read_bits() & Self::CC1S_MASK) >> Self::CC1S_OFFSET
    }

    /// Write **CC1S** `[1:0]` (RW)
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub fn write_cc1s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC1S_MASK) | ((value << Self::CC1S_OFFSET) & Self::CC1S_MASK)
        });
    }
}

/// CHCTLR1_Output value type for batch read/modify/write operations
///
/// Use `chctlr1output.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chctlr1output.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chctlr1OutputVal {
    bits: u32,
}

impl Chctlr1OutputVal {
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

    /// Read **OC2CE** `[15:15]`
    ///
    /// Output compare 2 clear
    /// enable
    #[inline(always)]
    pub const fn oc2ce(&self) -> bool {
        (self.bits & Chctlr1Output::OC2CE_MASK) != 0
    }

    /// Read **OC2M** `[14:12]`
    ///
    /// Output compare 2 mode
    #[inline(always)]
    pub const fn oc2m(&self) -> u32 {
        (self.bits & Chctlr1Output::OC2M_MASK) >> Chctlr1Output::OC2M_OFFSET
    }

    /// Read **OC2PE** `[11:11]`
    ///
    /// Output compare 2 preload
    /// enable
    #[inline(always)]
    pub const fn oc2pe(&self) -> bool {
        (self.bits & Chctlr1Output::OC2PE_MASK) != 0
    }

    /// Read **OC2FE** `[10:10]`
    ///
    /// Output compare 2 fast
    /// enable
    #[inline(always)]
    pub const fn oc2fe(&self) -> bool {
        (self.bits & Chctlr1Output::OC2FE_MASK) != 0
    }

    /// Read **CC2S** `[9:8]`
    ///
    /// Capture/Compare 2
    /// selection
    #[inline(always)]
    pub const fn cc2s(&self) -> u32 {
        (self.bits & Chctlr1Output::CC2S_MASK) >> Chctlr1Output::CC2S_OFFSET
    }

    /// Read **OC1CE** `[7:7]`
    ///
    /// Output compare 1 clear
    /// enable
    #[inline(always)]
    pub const fn oc1ce(&self) -> bool {
        (self.bits & Chctlr1Output::OC1CE_MASK) != 0
    }

    /// Read **OC1M** `[6:4]`
    ///
    /// Output compare 1 mode
    #[inline(always)]
    pub const fn oc1m(&self) -> u32 {
        (self.bits & Chctlr1Output::OC1M_MASK) >> Chctlr1Output::OC1M_OFFSET
    }

    /// Read **OC1PE** `[3:3]`
    ///
    /// Output compare 1 preload
    /// enable
    #[inline(always)]
    pub const fn oc1pe(&self) -> bool {
        (self.bits & Chctlr1Output::OC1PE_MASK) != 0
    }

    /// Read **OC1FE** `[2:2]`
    ///
    /// Output compare 1 fast
    /// enable
    #[inline(always)]
    pub const fn oc1fe(&self) -> bool {
        (self.bits & Chctlr1Output::OC1FE_MASK) != 0
    }

    /// Read **CC1S** `[1:0]`
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub const fn cc1s(&self) -> u32 {
        (self.bits & Chctlr1Output::CC1S_MASK) >> Chctlr1Output::CC1S_OFFSET
    }

    /// Set **OC2CE** `[15:15]`
    ///
    /// Output compare 2 clear
    /// enable
    #[inline(always)]
    pub const fn with_oc2ce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr1Output::OC2CE_MASK
            } else {
                self.bits & !Chctlr1Output::OC2CE_MASK
            },
        }
    }

    /// Set **OC2M** `[14:12]`
    ///
    /// Output compare 2 mode
    #[inline(always)]
    pub const fn with_oc2m(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Output::OC2M_MASK)
                | ((value << Chctlr1Output::OC2M_OFFSET) & Chctlr1Output::OC2M_MASK),
        }
    }

    /// Set **OC2PE** `[11:11]`
    ///
    /// Output compare 2 preload
    /// enable
    #[inline(always)]
    pub const fn with_oc2pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr1Output::OC2PE_MASK
            } else {
                self.bits & !Chctlr1Output::OC2PE_MASK
            },
        }
    }

    /// Set **OC2FE** `[10:10]`
    ///
    /// Output compare 2 fast
    /// enable
    #[inline(always)]
    pub const fn with_oc2fe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr1Output::OC2FE_MASK
            } else {
                self.bits & !Chctlr1Output::OC2FE_MASK
            },
        }
    }

    /// Set **CC2S** `[9:8]`
    ///
    /// Capture/Compare 2
    /// selection
    #[inline(always)]
    pub const fn with_cc2s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Output::CC2S_MASK)
                | ((value << Chctlr1Output::CC2S_OFFSET) & Chctlr1Output::CC2S_MASK),
        }
    }

    /// Set **OC1CE** `[7:7]`
    ///
    /// Output compare 1 clear
    /// enable
    #[inline(always)]
    pub const fn with_oc1ce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr1Output::OC1CE_MASK
            } else {
                self.bits & !Chctlr1Output::OC1CE_MASK
            },
        }
    }

    /// Set **OC1M** `[6:4]`
    ///
    /// Output compare 1 mode
    #[inline(always)]
    pub const fn with_oc1m(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Output::OC1M_MASK)
                | ((value << Chctlr1Output::OC1M_OFFSET) & Chctlr1Output::OC1M_MASK),
        }
    }

    /// Set **OC1PE** `[3:3]`
    ///
    /// Output compare 1 preload
    /// enable
    #[inline(always)]
    pub const fn with_oc1pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr1Output::OC1PE_MASK
            } else {
                self.bits & !Chctlr1Output::OC1PE_MASK
            },
        }
    }

    /// Set **OC1FE** `[2:2]`
    ///
    /// Output compare 1 fast
    /// enable
    #[inline(always)]
    pub const fn with_oc1fe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr1Output::OC1FE_MASK
            } else {
                self.bits & !Chctlr1Output::OC1FE_MASK
            },
        }
    }

    /// Set **CC1S** `[1:0]`
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub const fn with_cc1s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Output::CC1S_MASK)
                | ((value << Chctlr1Output::CC1S_OFFSET) & Chctlr1Output::CC1S_MASK),
        }
    }
}

impl Default for Chctlr1OutputVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Chctlr1Output {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chctlr1OutputVal {
        Chctlr1OutputVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chctlr1OutputVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chctlr1OutputVal) -> Chctlr1OutputVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CHCTLR1_Input Register
///
/// capture/compare mode register 1 (input
/// mode)
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IC2F** `[15:12]` (RW)
///
///   Input capture 2 filter
///
/// - **IC2PSC** `[11:10]` (RW)
///
///   Input capture 2 prescaler
///
/// - **CC2S** `[9:8]` (RW)
///
///   Capture/compare 2
///   selection
///
/// - **IC1F** `[7:4]` (RW)
///
///   Input capture 1 filter
///
/// - **IC1PSC** `[3:2]` (RW)
///
///   Input capture 1 prescaler
///
/// - **CC1S** `[1:0]` (RW)
///
///   Capture/Compare 1
///   selection
#[repr(transparent)]
pub struct Chctlr1Input {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Chctlr1Input {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chctlr1Input {
    // ======== Field Constants ========

    /// **IC2F**
    ///
    /// Input capture 2 filter
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const IC2F_OFFSET: u32 = 12;
    pub const IC2F_MASK: u32 = 0x0000f000;

    /// **IC2PSC**
    ///
    /// Input capture 2 prescaler
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const IC2PSC_OFFSET: u32 = 10;
    pub const IC2PSC_MASK: u32 = 0x00000c00;

    /// **CC2S**
    ///
    /// Capture/compare 2
    /// selection
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const CC2S_OFFSET: u32 = 8;
    pub const CC2S_MASK: u32 = 0x00000300;

    /// **IC1F**
    ///
    /// Input capture 1 filter
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const IC1F_OFFSET: u32 = 4;
    pub const IC1F_MASK: u32 = 0x000000f0;

    /// **IC1PSC**
    ///
    /// Input capture 1 prescaler
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const IC1PSC_OFFSET: u32 = 2;
    pub const IC1PSC_MASK: u32 = 0x0000000c;

    /// **CC1S**
    ///
    /// Capture/Compare 1
    /// selection
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CC1S_OFFSET: u32 = 0;
    pub const CC1S_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **IC2F** `[15:12]` (RW)
    ///
    /// Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&self) -> u32 {
        (self.value.read_bits() & Self::IC2F_MASK) >> Self::IC2F_OFFSET
    }

    /// Write **IC2F** `[15:12]` (RW)
    ///
    /// Input capture 2 filter
    #[inline(always)]
    pub fn write_ic2f(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC2F_MASK) | ((value << Self::IC2F_OFFSET) & Self::IC2F_MASK)
        });
    }

    /// **IC2PSC** `[11:10]` (RW)
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&self) -> u32 {
        (self.value.read_bits() & Self::IC2PSC_MASK) >> Self::IC2PSC_OFFSET
    }

    /// Write **IC2PSC** `[11:10]` (RW)
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub fn write_ic2psc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC2PSC_MASK) | ((value << Self::IC2PSC_OFFSET) & Self::IC2PSC_MASK)
        });
    }

    /// **CC2S** `[9:8]` (RW)
    ///
    /// Capture/compare 2
    /// selection
    #[inline(always)]
    pub fn cc2s(&self) -> u32 {
        (self.value.read_bits() & Self::CC2S_MASK) >> Self::CC2S_OFFSET
    }

    /// Write **CC2S** `[9:8]` (RW)
    ///
    /// Capture/compare 2
    /// selection
    #[inline(always)]
    pub fn write_cc2s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC2S_MASK) | ((value << Self::CC2S_OFFSET) & Self::CC2S_MASK)
        });
    }

    /// **IC1F** `[7:4]` (RW)
    ///
    /// Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> u32 {
        (self.value.read_bits() & Self::IC1F_MASK) >> Self::IC1F_OFFSET
    }

    /// Write **IC1F** `[7:4]` (RW)
    ///
    /// Input capture 1 filter
    #[inline(always)]
    pub fn write_ic1f(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC1F_MASK) | ((value << Self::IC1F_OFFSET) & Self::IC1F_MASK)
        });
    }

    /// **IC1PSC** `[3:2]` (RW)
    ///
    /// Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> u32 {
        (self.value.read_bits() & Self::IC1PSC_MASK) >> Self::IC1PSC_OFFSET
    }

    /// Write **IC1PSC** `[3:2]` (RW)
    ///
    /// Input capture 1 prescaler
    #[inline(always)]
    pub fn write_ic1psc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC1PSC_MASK) | ((value << Self::IC1PSC_OFFSET) & Self::IC1PSC_MASK)
        });
    }

    /// **CC1S** `[1:0]` (RW)
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub fn cc1s(&self) -> u32 {
        (self.value.read_bits() & Self::CC1S_MASK) >> Self::CC1S_OFFSET
    }

    /// Write **CC1S** `[1:0]` (RW)
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub fn write_cc1s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC1S_MASK) | ((value << Self::CC1S_OFFSET) & Self::CC1S_MASK)
        });
    }
}

/// CHCTLR1_Input value type for batch read/modify/write operations
///
/// Use `chctlr1input.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chctlr1input.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chctlr1InputVal {
    bits: u32,
}

impl Chctlr1InputVal {
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

    /// Read **IC2F** `[15:12]`
    ///
    /// Input capture 2 filter
    #[inline(always)]
    pub const fn ic2f(&self) -> u32 {
        (self.bits & Chctlr1Input::IC2F_MASK) >> Chctlr1Input::IC2F_OFFSET
    }

    /// Read **IC2PSC** `[11:10]`
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub const fn ic2psc(&self) -> u32 {
        (self.bits & Chctlr1Input::IC2PSC_MASK) >> Chctlr1Input::IC2PSC_OFFSET
    }

    /// Read **CC2S** `[9:8]`
    ///
    /// Capture/compare 2
    /// selection
    #[inline(always)]
    pub const fn cc2s(&self) -> u32 {
        (self.bits & Chctlr1Input::CC2S_MASK) >> Chctlr1Input::CC2S_OFFSET
    }

    /// Read **IC1F** `[7:4]`
    ///
    /// Input capture 1 filter
    #[inline(always)]
    pub const fn ic1f(&self) -> u32 {
        (self.bits & Chctlr1Input::IC1F_MASK) >> Chctlr1Input::IC1F_OFFSET
    }

    /// Read **IC1PSC** `[3:2]`
    ///
    /// Input capture 1 prescaler
    #[inline(always)]
    pub const fn ic1psc(&self) -> u32 {
        (self.bits & Chctlr1Input::IC1PSC_MASK) >> Chctlr1Input::IC1PSC_OFFSET
    }

    /// Read **CC1S** `[1:0]`
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub const fn cc1s(&self) -> u32 {
        (self.bits & Chctlr1Input::CC1S_MASK) >> Chctlr1Input::CC1S_OFFSET
    }

    /// Set **IC2F** `[15:12]`
    ///
    /// Input capture 2 filter
    #[inline(always)]
    pub const fn with_ic2f(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::IC2F_MASK)
                | ((value << Chctlr1Input::IC2F_OFFSET) & Chctlr1Input::IC2F_MASK),
        }
    }

    /// Set **IC2PSC** `[11:10]`
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub const fn with_ic2psc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::IC2PSC_MASK)
                | ((value << Chctlr1Input::IC2PSC_OFFSET) & Chctlr1Input::IC2PSC_MASK),
        }
    }

    /// Set **CC2S** `[9:8]`
    ///
    /// Capture/compare 2
    /// selection
    #[inline(always)]
    pub const fn with_cc2s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::CC2S_MASK)
                | ((value << Chctlr1Input::CC2S_OFFSET) & Chctlr1Input::CC2S_MASK),
        }
    }

    /// Set **IC1F** `[7:4]`
    ///
    /// Input capture 1 filter
    #[inline(always)]
    pub const fn with_ic1f(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::IC1F_MASK)
                | ((value << Chctlr1Input::IC1F_OFFSET) & Chctlr1Input::IC1F_MASK),
        }
    }

    /// Set **IC1PSC** `[3:2]`
    ///
    /// Input capture 1 prescaler
    #[inline(always)]
    pub const fn with_ic1psc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::IC1PSC_MASK)
                | ((value << Chctlr1Input::IC1PSC_OFFSET) & Chctlr1Input::IC1PSC_MASK),
        }
    }

    /// Set **CC1S** `[1:0]`
    ///
    /// Capture/Compare 1
    /// selection
    #[inline(always)]
    pub const fn with_cc1s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::CC1S_MASK)
                | ((value << Chctlr1Input::CC1S_OFFSET) & Chctlr1Input::CC1S_MASK),
        }
    }
}

impl Default for Chctlr1InputVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Chctlr1Input {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chctlr1InputVal {
        Chctlr1InputVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chctlr1InputVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chctlr1InputVal) -> Chctlr1InputVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CCER Register
///
/// capture/compare enable
/// register
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CC2P** `[5]` (RW)
///
///   Capture/Compare 2 output
///   Polarity
///
/// - **CC2E** `[4]` (RW)
///
///   Capture/Compare 2 output
///   enable
///
/// - **CC1P** `[1]` (RW)
///
///   Capture/Compare 1 output
///   Polarity
///
/// - **CC1E** `[0]` (RW)
///
///   Capture/Compare 1 output
///   enable
#[repr(transparent)]
pub struct Ccer {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Ccer {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ccer {
    // ======== Field Constants ========

    /// **CC2P**
    ///
    /// Capture/Compare 2 output
    /// Polarity
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CC2P_OFFSET: u32 = 5;
    pub const CC2P_MASK: u32 = 0x00000020;
    pub const CC2P: u32 = 1 << 5;

    /// **CC2E**
    ///
    /// Capture/Compare 2 output
    /// enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CC2E_OFFSET: u32 = 4;
    pub const CC2E_MASK: u32 = 0x00000010;
    pub const CC2E: u32 = 1 << 4;

    /// **CC1P**
    ///
    /// Capture/Compare 1 output
    /// Polarity
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CC1P_OFFSET: u32 = 1;
    pub const CC1P_MASK: u32 = 0x00000002;
    pub const CC1P: u32 = 1 << 1;

    /// **CC1E**
    ///
    /// Capture/Compare 1 output
    /// enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CC1E_OFFSET: u32 = 0;
    pub const CC1E_MASK: u32 = 0x00000001;
    pub const CC1E: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CC2P** `[5]` (RW)
    ///
    /// Capture/Compare 2 output
    /// Polarity
    #[inline(always)]
    pub fn cc2p(&self) -> bool {
        (self.value.read_bits() & Self::CC2P_MASK) != 0
    }

    /// Set CC2P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2p(&self) {
        self.value.modify_bits(|r| r | Self::CC2P_MASK);
    }

    /// Clear CC2P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2p(&self) {
        self.value.modify_bits(|r| r & !Self::CC2P_MASK);
    }

    /// **CC2E** `[4]` (RW)
    ///
    /// Capture/Compare 2 output
    /// enable
    #[inline(always)]
    pub fn cc2e(&self) -> bool {
        (self.value.read_bits() & Self::CC2E_MASK) != 0
    }

    /// Set CC2E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2e(&self) {
        self.value.modify_bits(|r| r | Self::CC2E_MASK);
    }

    /// Clear CC2E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2e(&self) {
        self.value.modify_bits(|r| r & !Self::CC2E_MASK);
    }

    /// **CC1P** `[1]` (RW)
    ///
    /// Capture/Compare 1 output
    /// Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> bool {
        (self.value.read_bits() & Self::CC1P_MASK) != 0
    }

    /// Set CC1P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1p(&self) {
        self.value.modify_bits(|r| r | Self::CC1P_MASK);
    }

    /// Clear CC1P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1p(&self) {
        self.value.modify_bits(|r| r & !Self::CC1P_MASK);
    }

    /// **CC1E** `[0]` (RW)
    ///
    /// Capture/Compare 1 output
    /// enable
    #[inline(always)]
    pub fn cc1e(&self) -> bool {
        (self.value.read_bits() & Self::CC1E_MASK) != 0
    }

    /// Set CC1E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1e(&self) {
        self.value.modify_bits(|r| r | Self::CC1E_MASK);
    }

    /// Clear CC1E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1e(&self) {
        self.value.modify_bits(|r| r & !Self::CC1E_MASK);
    }
}

/// CCER value type for batch read/modify/write operations
///
/// Use `ccer.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ccer.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CcerVal {
    bits: u32,
}

impl CcerVal {
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

    /// Read **CC2P** `[5:5]`
    ///
    /// Capture/Compare 2 output
    /// Polarity
    #[inline(always)]
    pub const fn cc2p(&self) -> bool {
        (self.bits & Ccer::CC2P_MASK) != 0
    }

    /// Read **CC2E** `[4:4]`
    ///
    /// Capture/Compare 2 output
    /// enable
    #[inline(always)]
    pub const fn cc2e(&self) -> bool {
        (self.bits & Ccer::CC2E_MASK) != 0
    }

    /// Read **CC1P** `[1:1]`
    ///
    /// Capture/Compare 1 output
    /// Polarity
    #[inline(always)]
    pub const fn cc1p(&self) -> bool {
        (self.bits & Ccer::CC1P_MASK) != 0
    }

    /// Read **CC1E** `[0:0]`
    ///
    /// Capture/Compare 1 output
    /// enable
    #[inline(always)]
    pub const fn cc1e(&self) -> bool {
        (self.bits & Ccer::CC1E_MASK) != 0
    }

    /// Set **CC2P** `[5:5]`
    ///
    /// Capture/Compare 2 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc2p(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC2P_MASK
            } else {
                self.bits & !Ccer::CC2P_MASK
            },
        }
    }

    /// Set **CC2E** `[4:4]`
    ///
    /// Capture/Compare 2 output
    /// enable
    #[inline(always)]
    pub const fn with_cc2e(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC2E_MASK
            } else {
                self.bits & !Ccer::CC2E_MASK
            },
        }
    }

    /// Set **CC1P** `[1:1]`
    ///
    /// Capture/Compare 1 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc1p(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC1P_MASK
            } else {
                self.bits & !Ccer::CC1P_MASK
            },
        }
    }

    /// Set **CC1E** `[0:0]`
    ///
    /// Capture/Compare 1 output
    /// enable
    #[inline(always)]
    pub const fn with_cc1e(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC1E_MASK
            } else {
                self.bits & !Ccer::CC1E_MASK
            },
        }
    }
}

impl Default for CcerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ccer {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CcerVal {
        CcerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CcerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CcerVal) -> CcerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNT Register
///
/// counter
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CNT** `[15:0]` (RW)
///
///   counter value
#[repr(transparent)]
pub struct Cnt {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Cnt {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cnt {
    // ======== Field Constants ========

    /// **CNT**
    ///
    /// counter value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CNT_OFFSET: u32 = 0;
    pub const CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CNT** `[15:0]` (RW)
    ///
    /// counter value
    #[inline(always)]
    pub fn cnt(&self) -> u32 {
        (self.value.read_bits() & Self::CNT_MASK) >> Self::CNT_OFFSET
    }

    /// Write **CNT** `[15:0]` (RW)
    ///
    /// counter value
    #[inline(always)]
    pub fn write_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNT_MASK) | ((value << Self::CNT_OFFSET) & Self::CNT_MASK)
        });
    }
}

/// CNT value type for batch read/modify/write operations
///
/// Use `cnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CntVal {
    bits: u32,
}

impl CntVal {
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

    /// Read **CNT** `[15:0]`
    ///
    /// counter value
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        (self.bits & Cnt::CNT_MASK) >> Cnt::CNT_OFFSET
    }

    /// Set **CNT** `[15:0]`
    ///
    /// counter value
    #[inline(always)]
    pub const fn with_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cnt::CNT_MASK) | ((value << Cnt::CNT_OFFSET) & Cnt::CNT_MASK),
        }
    }
}

impl Default for CntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CntVal {
        CntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CntVal) -> CntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PSC Register
///
/// prescaler
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PSC** `[15:0]` (RW)
///
///   Prescaler value
#[repr(transparent)]
pub struct Psc {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Psc {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Psc {
    // ======== Field Constants ========

    /// **PSC**
    ///
    /// Prescaler value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PSC_OFFSET: u32 = 0;
    pub const PSC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PSC** `[15:0]` (RW)
    ///
    /// Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> u32 {
        (self.value.read_bits() & Self::PSC_MASK) >> Self::PSC_OFFSET
    }

    /// Write **PSC** `[15:0]` (RW)
    ///
    /// Prescaler value
    #[inline(always)]
    pub fn write_psc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSC_MASK) | ((value << Self::PSC_OFFSET) & Self::PSC_MASK)
        });
    }
}

/// PSC value type for batch read/modify/write operations
///
/// Use `psc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `psc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PscVal {
    bits: u32,
}

impl PscVal {
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

    /// Read **PSC** `[15:0]`
    ///
    /// Prescaler value
    #[inline(always)]
    pub const fn psc(&self) -> u32 {
        (self.bits & Psc::PSC_MASK) >> Psc::PSC_OFFSET
    }

    /// Set **PSC** `[15:0]`
    ///
    /// Prescaler value
    #[inline(always)]
    pub const fn with_psc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Psc::PSC_MASK) | ((value << Psc::PSC_OFFSET) & Psc::PSC_MASK),
        }
    }
}

impl Default for PscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Psc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PscVal {
        PscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PscVal) -> PscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ATRLR Register
///
/// auto-reload register
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ATRLR** `[15:0]` (RW)
///
///   Auto-reload value
#[repr(transparent)]
pub struct Atrlr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Atrlr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrlr {
    // ======== Field Constants ========

    /// **ATRLR**
    ///
    /// Auto-reload value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ATRLR_OFFSET: u32 = 0;
    pub const ATRLR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ATRLR** `[15:0]` (RW)
    ///
    /// Auto-reload value
    #[inline(always)]
    pub fn atrlr(&self) -> u32 {
        (self.value.read_bits() & Self::ATRLR_MASK) >> Self::ATRLR_OFFSET
    }

    /// Write **ATRLR** `[15:0]` (RW)
    ///
    /// Auto-reload value
    #[inline(always)]
    pub fn write_atrlr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ATRLR_MASK) | ((value << Self::ATRLR_OFFSET) & Self::ATRLR_MASK)
        });
    }
}

/// ATRLR value type for batch read/modify/write operations
///
/// Use `atrlr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrlr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AtrlrVal {
    bits: u32,
}

impl AtrlrVal {
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

    /// Read **ATRLR** `[15:0]`
    ///
    /// Auto-reload value
    #[inline(always)]
    pub const fn atrlr(&self) -> u32 {
        (self.bits & Atrlr::ATRLR_MASK) >> Atrlr::ATRLR_OFFSET
    }

    /// Set **ATRLR** `[15:0]`
    ///
    /// Auto-reload value
    #[inline(always)]
    pub const fn with_atrlr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrlr::ATRLR_MASK)
                | ((value << Atrlr::ATRLR_OFFSET) & Atrlr::ATRLR_MASK),
        }
    }
}

impl Default for AtrlrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrlr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AtrlrVal {
        AtrlrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AtrlrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AtrlrVal) -> AtrlrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH1CVR Register
///
/// capture/compare register 1
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH1CVR** `[15:0]` (RW)
///
///   Capture/Compare 1 value
#[repr(transparent)]
pub struct Ch1cvr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Ch1cvr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1cvr {
    // ======== Field Constants ========

    /// **CH1CVR**
    ///
    /// Capture/Compare 1 value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH1CVR_OFFSET: u32 = 0;
    pub const CH1CVR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH1CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare 1 value
    #[inline(always)]
    pub fn ch1cvr(&self) -> u32 {
        (self.value.read_bits() & Self::CH1CVR_MASK) >> Self::CH1CVR_OFFSET
    }

    /// Write **CH1CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare 1 value
    #[inline(always)]
    pub fn write_ch1cvr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH1CVR_MASK) | ((value << Self::CH1CVR_OFFSET) & Self::CH1CVR_MASK)
        });
    }
}

/// CH1CVR value type for batch read/modify/write operations
///
/// Use `ch1cvr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1cvr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1cvrVal {
    bits: u32,
}

impl Ch1cvrVal {
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

    /// Read **CH1CVR** `[15:0]`
    ///
    /// Capture/Compare 1 value
    #[inline(always)]
    pub const fn ch1cvr(&self) -> u32 {
        (self.bits & Ch1cvr::CH1CVR_MASK) >> Ch1cvr::CH1CVR_OFFSET
    }

    /// Set **CH1CVR** `[15:0]`
    ///
    /// Capture/Compare 1 value
    #[inline(always)]
    pub const fn with_ch1cvr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1cvr::CH1CVR_MASK)
                | ((value << Ch1cvr::CH1CVR_OFFSET) & Ch1cvr::CH1CVR_MASK),
        }
    }
}

impl Default for Ch1cvrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1cvr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1cvrVal {
        Ch1cvrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1cvrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1cvrVal) -> Ch1cvrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH2CVR Register
///
/// capture/compare register 2
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH2CVR** `[15:0]` (RW)
///
///   Capture/Compare 2 value
#[repr(transparent)]
pub struct Ch2cvr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Ch2cvr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2cvr {
    // ======== Field Constants ========

    /// **CH2CVR**
    ///
    /// Capture/Compare 2 value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH2CVR_OFFSET: u32 = 0;
    pub const CH2CVR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH2CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare 2 value
    #[inline(always)]
    pub fn ch2cvr(&self) -> u32 {
        (self.value.read_bits() & Self::CH2CVR_MASK) >> Self::CH2CVR_OFFSET
    }

    /// Write **CH2CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare 2 value
    #[inline(always)]
    pub fn write_ch2cvr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH2CVR_MASK) | ((value << Self::CH2CVR_OFFSET) & Self::CH2CVR_MASK)
        });
    }
}

/// CH2CVR value type for batch read/modify/write operations
///
/// Use `ch2cvr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2cvr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2cvrVal {
    bits: u32,
}

impl Ch2cvrVal {
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

    /// Read **CH2CVR** `[15:0]`
    ///
    /// Capture/Compare 2 value
    #[inline(always)]
    pub const fn ch2cvr(&self) -> u32 {
        (self.bits & Ch2cvr::CH2CVR_MASK) >> Ch2cvr::CH2CVR_OFFSET
    }

    /// Set **CH2CVR** `[15:0]`
    ///
    /// Capture/Compare 2 value
    #[inline(always)]
    pub const fn with_ch2cvr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2cvr::CH2CVR_MASK)
                | ((value << Ch2cvr::CH2CVR_OFFSET) & Ch2cvr::CH2CVR_MASK),
        }
    }
}

impl Default for Ch2cvrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2cvr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2cvrVal {
        Ch2cvrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2cvrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2cvrVal) -> Ch2cvrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPEC Register
///
/// SPEC
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TOGGLE** `[15]` (RO)
///
///   TOGGLE
///
/// - **PWM_OC2** `[5]` (RW)
///
///   PWM_OC2
///
/// - **PWM_OC1** `[4]` (RW)
///
///   PWM_OC1
///
/// - **PWM_EN** `[0]` (RW)
///
///   PWM_EN
#[repr(transparent)]
pub struct Spec {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Spec {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spec {
    // ======== Field Constants ========

    /// **TOGGLE**
    ///
    /// TOGGLE
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TOGGLE_OFFSET: u32 = 15;
    pub const TOGGLE_MASK: u32 = 0x00008000;
    pub const TOGGLE: u32 = 1 << 15;

    /// **PWM_OC2**
    ///
    /// PWM_OC2
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const PWM_OC2_OFFSET: u32 = 5;
    pub const PWM_OC2_MASK: u32 = 0x00000020;
    pub const PWM_OC2: u32 = 1 << 5;

    /// **PWM_OC1**
    ///
    /// PWM_OC1
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PWM_OC1_OFFSET: u32 = 4;
    pub const PWM_OC1_MASK: u32 = 0x00000010;
    pub const PWM_OC1: u32 = 1 << 4;

    /// **PWM_EN**
    ///
    /// PWM_EN
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PWM_EN_OFFSET: u32 = 0;
    pub const PWM_EN_MASK: u32 = 0x00000001;
    pub const PWM_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TOGGLE** `[15]` (RO)
    ///
    /// TOGGLE
    #[inline(always)]
    pub fn toggle(&self) -> bool {
        (self.value.read_bits() & Self::TOGGLE_MASK) != 0
    }

    /// **PWM_OC2** `[5]` (RW)
    ///
    /// PWM_OC2
    #[inline(always)]
    pub fn pwm_oc2(&self) -> bool {
        (self.value.read_bits() & Self::PWM_OC2_MASK) != 0
    }

    /// Set PWM_OC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwm_oc2(&self) {
        self.value.modify_bits(|r| r | Self::PWM_OC2_MASK);
    }

    /// Clear PWM_OC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwm_oc2(&self) {
        self.value.modify_bits(|r| r & !Self::PWM_OC2_MASK);
    }

    /// **PWM_OC1** `[4]` (RW)
    ///
    /// PWM_OC1
    #[inline(always)]
    pub fn pwm_oc1(&self) -> bool {
        (self.value.read_bits() & Self::PWM_OC1_MASK) != 0
    }

    /// Set PWM_OC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwm_oc1(&self) {
        self.value.modify_bits(|r| r | Self::PWM_OC1_MASK);
    }

    /// Clear PWM_OC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwm_oc1(&self) {
        self.value.modify_bits(|r| r & !Self::PWM_OC1_MASK);
    }

    /// **PWM_EN** `[0]` (RW)
    ///
    /// PWM_EN
    #[inline(always)]
    pub fn pwm_en(&self) -> bool {
        (self.value.read_bits() & Self::PWM_EN_MASK) != 0
    }

    /// Set PWM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwm_en(&self) {
        self.value.modify_bits(|r| r | Self::PWM_EN_MASK);
    }

    /// Clear PWM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwm_en(&self) {
        self.value.modify_bits(|r| r & !Self::PWM_EN_MASK);
    }
}

/// SPEC value type for batch read/modify/write operations
///
/// Use `spec.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spec.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpecVal {
    bits: u32,
}

impl SpecVal {
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

    /// Read **TOGGLE** `[15:15]`
    ///
    /// TOGGLE
    #[inline(always)]
    pub const fn toggle(&self) -> bool {
        (self.bits & Spec::TOGGLE_MASK) != 0
    }

    /// Read **PWM_OC2** `[5:5]`
    ///
    /// PWM_OC2
    #[inline(always)]
    pub const fn pwm_oc2(&self) -> bool {
        (self.bits & Spec::PWM_OC2_MASK) != 0
    }

    /// Read **PWM_OC1** `[4:4]`
    ///
    /// PWM_OC1
    #[inline(always)]
    pub const fn pwm_oc1(&self) -> bool {
        (self.bits & Spec::PWM_OC1_MASK) != 0
    }

    /// Read **PWM_EN** `[0:0]`
    ///
    /// PWM_EN
    #[inline(always)]
    pub const fn pwm_en(&self) -> bool {
        (self.bits & Spec::PWM_EN_MASK) != 0
    }

    /// Set **PWM_OC2** `[5:5]`
    ///
    /// PWM_OC2
    #[inline(always)]
    pub const fn with_pwm_oc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spec::PWM_OC2_MASK
            } else {
                self.bits & !Spec::PWM_OC2_MASK
            },
        }
    }

    /// Set **PWM_OC1** `[4:4]`
    ///
    /// PWM_OC1
    #[inline(always)]
    pub const fn with_pwm_oc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spec::PWM_OC1_MASK
            } else {
                self.bits & !Spec::PWM_OC1_MASK
            },
        }
    }

    /// Set **PWM_EN** `[0:0]`
    ///
    /// PWM_EN
    #[inline(always)]
    pub const fn with_pwm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spec::PWM_EN_MASK
            } else {
                self.bits & !Spec::PWM_EN_MASK
            },
        }
    }
}

impl Default for SpecVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spec {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpecVal {
        SpecVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpecVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpecVal) -> SpecVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// control register 1
    pub ctlr1: Ctlr1,
    _reserved0: [u8; 6],
    /// 0x0008
    /// slave mode control register
    pub smcfgr: Smcfgr,
    _reserved1: [u8; 2],
    /// 0x000c
    /// DMA/Interrupt enable register
    pub dmaintenr: Dmaintenr,
    _reserved2: [u8; 2],
    /// 0x0010
    /// status register
    pub intfr: Intfr,
    _reserved3: [u8; 2],
    /// 0x0014
    /// event generation register
    pub swevgr: Swevgr,
    _reserved4: [u8; 2],
    /// 0x0018
    /// capture/compare mode register 1 (output
    /// mode)
    pub chctlr1_output: Chctlr1Output,
    _reserved5: [u8; 6],
    /// 0x0020
    /// capture/compare enable
    /// register
    pub ccer: Ccer,
    _reserved6: [u8; 2],
    /// 0x0024
    /// counter
    pub cnt: Cnt,
    _reserved7: [u8; 2],
    /// 0x0028
    /// prescaler
    pub psc: Psc,
    _reserved8: [u8; 2],
    /// 0x002c
    /// auto-reload register
    pub atrlr: Atrlr,
    _reserved9: [u8; 6],
    /// 0x0034
    /// capture/compare register 1
    pub ch1cvr: Ch1cvr,
    _reserved10: [u8; 2],
    /// 0x0038
    /// capture/compare register 2
    pub ch2cvr: Ch2cvr,
    _reserved11: [u8; 22],
    /// 0x0050
    /// SPEC
    pub spec: Spec,
}

impl RegisterBlock {
    /// Alternate view at offset 0x0018
    /// capture/compare mode register 1 (input
    /// mode)
    #[inline(always)]
    pub fn chctlr1_input(&self) -> &Chctlr1Input {
        // SAFETY: the alias view has the same offset
        // and size as the primary field.
        unsafe { &*core::ptr::addr_of!(self.chctlr1_output).cast() }
    }
}
