//! TIM1
//! Advanced timer

#![allow(dead_code)]

pub const BASE: u32 = 0x40012c00;

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
/// - **CMS** `[6:5]` (RW)
///
///   Center-aligned mode
///   selection
///
/// - **DIR** `[4]` (RW)
///
///   Direction
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctlr1 {
    type Target = crate::common::VolatileCell<u32>;
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

    /// **CMS**
    ///
    /// Center-aligned mode
    /// selection
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const CMS_OFFSET: u32 = 5;
    pub const CMS_MASK: u32 = 0x00000060;

    /// **DIR**
    ///
    /// Direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

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

    /// **CMS** `[6:5]` (RW)
    ///
    /// Center-aligned mode
    /// selection
    #[inline(always)]
    pub fn cms(&self) -> u32 {
        (self.value.read_bits() & Self::CMS_MASK) >> Self::CMS_OFFSET
    }

    /// Write **CMS** `[6:5]` (RW)
    ///
    /// Center-aligned mode
    /// selection
    #[inline(always)]
    pub fn write_cms(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CMS_MASK) | ((value << Self::CMS_OFFSET) & Self::CMS_MASK)
        });
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Direction
    #[inline(always)]
    pub fn dir(&self) -> bool {
        (self.value.read_bits() & Self::DIR_MASK) != 0
    }

    /// Set DIR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dir(&self) {
        self.value.modify_bits(|r| r | Self::DIR_MASK);
    }

    /// Clear DIR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dir(&self) {
        self.value.modify_bits(|r| r & !Self::DIR_MASK);
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

    /// Read **CMS** `[6:5]`
    ///
    /// Center-aligned mode
    /// selection
    #[inline(always)]
    pub const fn cms(&self) -> u32 {
        (self.bits & Ctlr1::CMS_MASK) >> Ctlr1::CMS_OFFSET
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Ctlr1::DIR_MASK) != 0
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

    /// Set **CMS** `[6:5]`
    ///
    /// Center-aligned mode
    /// selection
    #[inline(always)]
    pub const fn with_cms(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::CMS_MASK) | ((value << Ctlr1::CMS_OFFSET) & Ctlr1::CMS_MASK),
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::DIR_MASK
            } else {
                self.bits & !Ctlr1::DIR_MASK
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

/// CTLR2 Register
///
/// control register 2
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OIS4** `[14]` (RW)
///
///   Output Idle state 4
///
/// - **OIS3N** `[13]` (RW)
///
///   Output Idle state 3
///
/// - **OIS3** `[12]` (RW)
///
///   Output Idle state 3
///
/// - **OIS2N** `[11]` (RW)
///
///   Output Idle state 2
///
/// - **OIS2** `[10]` (RW)
///
///   Output Idle state 2
///
/// - **OIS1N** `[9]` (RW)
///
///   Output Idle state 1
///
/// - **OIS1** `[8]` (RW)
///
///   Output Idle state 1
///
/// - **TI1S** `[7]` (RW)
///
///   TI1 selection
///
/// - **MMS** `[6:4]` (RW)
///
///   Master mode selection
///
/// - **CCDS** `[3]` (RW)
///
///   Capture/compare DMA
///   selection
///
/// - **CCUS** `[2]` (RW)
///
///   Capture/compare control update
///   selection
///
/// - **CCPC** `[0]` (RW)
///
///   Capture/compare preloaded
///   control
#[repr(transparent)]
pub struct Ctlr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctlr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctlr2 {
    // ======== Field Constants ========

    /// **OIS4**
    ///
    /// Output Idle state 4
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const OIS4_OFFSET: u32 = 14;
    pub const OIS4_MASK: u32 = 0x00004000;
    pub const OIS4: u32 = 1 << 14;

    /// **OIS3N**
    ///
    /// Output Idle state 3
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const OIS3N_OFFSET: u32 = 13;
    pub const OIS3N_MASK: u32 = 0x00002000;
    pub const OIS3N: u32 = 1 << 13;

    /// **OIS3**
    ///
    /// Output Idle state 3
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const OIS3_OFFSET: u32 = 12;
    pub const OIS3_MASK: u32 = 0x00001000;
    pub const OIS3: u32 = 1 << 12;

    /// **OIS2N**
    ///
    /// Output Idle state 2
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OIS2N_OFFSET: u32 = 11;
    pub const OIS2N_MASK: u32 = 0x00000800;
    pub const OIS2N: u32 = 1 << 11;

    /// **OIS2**
    ///
    /// Output Idle state 2
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OIS2_OFFSET: u32 = 10;
    pub const OIS2_MASK: u32 = 0x00000400;
    pub const OIS2: u32 = 1 << 10;

    /// **OIS1N**
    ///
    /// Output Idle state 1
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OIS1N_OFFSET: u32 = 9;
    pub const OIS1N_MASK: u32 = 0x00000200;
    pub const OIS1N: u32 = 1 << 9;

    /// **OIS1**
    ///
    /// Output Idle state 1
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const OIS1_OFFSET: u32 = 8;
    pub const OIS1_MASK: u32 = 0x00000100;
    pub const OIS1: u32 = 1 << 8;

    /// **TI1S**
    ///
    /// TI1 selection
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TI1S_OFFSET: u32 = 7;
    pub const TI1S_MASK: u32 = 0x00000080;
    pub const TI1S: u32 = 1 << 7;

    /// **MMS**
    ///
    /// Master mode selection
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const MMS_OFFSET: u32 = 4;
    pub const MMS_MASK: u32 = 0x00000070;

    /// **CCDS**
    ///
    /// Capture/compare DMA
    /// selection
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CCDS_OFFSET: u32 = 3;
    pub const CCDS_MASK: u32 = 0x00000008;
    pub const CCDS: u32 = 1 << 3;

    /// **CCUS**
    ///
    /// Capture/compare control update
    /// selection
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CCUS_OFFSET: u32 = 2;
    pub const CCUS_MASK: u32 = 0x00000004;
    pub const CCUS: u32 = 1 << 2;

    /// **CCPC**
    ///
    /// Capture/compare preloaded
    /// control
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CCPC_OFFSET: u32 = 0;
    pub const CCPC_MASK: u32 = 0x00000001;
    pub const CCPC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OIS4** `[14]` (RW)
    ///
    /// Output Idle state 4
    #[inline(always)]
    pub fn ois4(&self) -> bool {
        (self.value.read_bits() & Self::OIS4_MASK) != 0
    }

    /// Set OIS4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois4(&self) {
        self.value.modify_bits(|r| r | Self::OIS4_MASK);
    }

    /// Clear OIS4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois4(&self) {
        self.value.modify_bits(|r| r & !Self::OIS4_MASK);
    }

    /// **OIS3N** `[13]` (RW)
    ///
    /// Output Idle state 3
    #[inline(always)]
    pub fn ois3n(&self) -> bool {
        (self.value.read_bits() & Self::OIS3N_MASK) != 0
    }

    /// Set OIS3N bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois3n(&self) {
        self.value.modify_bits(|r| r | Self::OIS3N_MASK);
    }

    /// Clear OIS3N bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois3n(&self) {
        self.value.modify_bits(|r| r & !Self::OIS3N_MASK);
    }

    /// **OIS3** `[12]` (RW)
    ///
    /// Output Idle state 3
    #[inline(always)]
    pub fn ois3(&self) -> bool {
        (self.value.read_bits() & Self::OIS3_MASK) != 0
    }

    /// Set OIS3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois3(&self) {
        self.value.modify_bits(|r| r | Self::OIS3_MASK);
    }

    /// Clear OIS3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois3(&self) {
        self.value.modify_bits(|r| r & !Self::OIS3_MASK);
    }

    /// **OIS2N** `[11]` (RW)
    ///
    /// Output Idle state 2
    #[inline(always)]
    pub fn ois2n(&self) -> bool {
        (self.value.read_bits() & Self::OIS2N_MASK) != 0
    }

    /// Set OIS2N bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois2n(&self) {
        self.value.modify_bits(|r| r | Self::OIS2N_MASK);
    }

    /// Clear OIS2N bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois2n(&self) {
        self.value.modify_bits(|r| r & !Self::OIS2N_MASK);
    }

    /// **OIS2** `[10]` (RW)
    ///
    /// Output Idle state 2
    #[inline(always)]
    pub fn ois2(&self) -> bool {
        (self.value.read_bits() & Self::OIS2_MASK) != 0
    }

    /// Set OIS2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois2(&self) {
        self.value.modify_bits(|r| r | Self::OIS2_MASK);
    }

    /// Clear OIS2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois2(&self) {
        self.value.modify_bits(|r| r & !Self::OIS2_MASK);
    }

    /// **OIS1N** `[9]` (RW)
    ///
    /// Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&self) -> bool {
        (self.value.read_bits() & Self::OIS1N_MASK) != 0
    }

    /// Set OIS1N bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois1n(&self) {
        self.value.modify_bits(|r| r | Self::OIS1N_MASK);
    }

    /// Clear OIS1N bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois1n(&self) {
        self.value.modify_bits(|r| r & !Self::OIS1N_MASK);
    }

    /// **OIS1** `[8]` (RW)
    ///
    /// Output Idle state 1
    #[inline(always)]
    pub fn ois1(&self) -> bool {
        (self.value.read_bits() & Self::OIS1_MASK) != 0
    }

    /// Set OIS1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ois1(&self) {
        self.value.modify_bits(|r| r | Self::OIS1_MASK);
    }

    /// Clear OIS1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ois1(&self) {
        self.value.modify_bits(|r| r & !Self::OIS1_MASK);
    }

    /// **TI1S** `[7]` (RW)
    ///
    /// TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> bool {
        (self.value.read_bits() & Self::TI1S_MASK) != 0
    }

    /// Set TI1S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ti1s(&self) {
        self.value.modify_bits(|r| r | Self::TI1S_MASK);
    }

    /// Clear TI1S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ti1s(&self) {
        self.value.modify_bits(|r| r & !Self::TI1S_MASK);
    }

    /// **MMS** `[6:4]` (RW)
    ///
    /// Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> u32 {
        (self.value.read_bits() & Self::MMS_MASK) >> Self::MMS_OFFSET
    }

    /// Write **MMS** `[6:4]` (RW)
    ///
    /// Master mode selection
    #[inline(always)]
    pub fn write_mms(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MMS_MASK) | ((value << Self::MMS_OFFSET) & Self::MMS_MASK)
        });
    }

    /// **CCDS** `[3]` (RW)
    ///
    /// Capture/compare DMA
    /// selection
    #[inline(always)]
    pub fn ccds(&self) -> bool {
        (self.value.read_bits() & Self::CCDS_MASK) != 0
    }

    /// Set CCDS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ccds(&self) {
        self.value.modify_bits(|r| r | Self::CCDS_MASK);
    }

    /// Clear CCDS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ccds(&self) {
        self.value.modify_bits(|r| r & !Self::CCDS_MASK);
    }

    /// **CCUS** `[2]` (RW)
    ///
    /// Capture/compare control update
    /// selection
    #[inline(always)]
    pub fn ccus(&self) -> bool {
        (self.value.read_bits() & Self::CCUS_MASK) != 0
    }

    /// Set CCUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ccus(&self) {
        self.value.modify_bits(|r| r | Self::CCUS_MASK);
    }

    /// Clear CCUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ccus(&self) {
        self.value.modify_bits(|r| r & !Self::CCUS_MASK);
    }

    /// **CCPC** `[0]` (RW)
    ///
    /// Capture/compare preloaded
    /// control
    #[inline(always)]
    pub fn ccpc(&self) -> bool {
        (self.value.read_bits() & Self::CCPC_MASK) != 0
    }

    /// Set CCPC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ccpc(&self) {
        self.value.modify_bits(|r| r | Self::CCPC_MASK);
    }

    /// Clear CCPC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ccpc(&self) {
        self.value.modify_bits(|r| r & !Self::CCPC_MASK);
    }
}

/// CTLR2 value type for batch read/modify/write operations
///
/// Use `ctlr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctlr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ctlr2Val {
    bits: u32,
}

impl Ctlr2Val {
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

    /// Read **OIS4** `[14:14]`
    ///
    /// Output Idle state 4
    #[inline(always)]
    pub const fn ois4(&self) -> bool {
        (self.bits & Ctlr2::OIS4_MASK) != 0
    }

    /// Read **OIS3N** `[13:13]`
    ///
    /// Output Idle state 3
    #[inline(always)]
    pub const fn ois3n(&self) -> bool {
        (self.bits & Ctlr2::OIS3N_MASK) != 0
    }

    /// Read **OIS3** `[12:12]`
    ///
    /// Output Idle state 3
    #[inline(always)]
    pub const fn ois3(&self) -> bool {
        (self.bits & Ctlr2::OIS3_MASK) != 0
    }

    /// Read **OIS2N** `[11:11]`
    ///
    /// Output Idle state 2
    #[inline(always)]
    pub const fn ois2n(&self) -> bool {
        (self.bits & Ctlr2::OIS2N_MASK) != 0
    }

    /// Read **OIS2** `[10:10]`
    ///
    /// Output Idle state 2
    #[inline(always)]
    pub const fn ois2(&self) -> bool {
        (self.bits & Ctlr2::OIS2_MASK) != 0
    }

    /// Read **OIS1N** `[9:9]`
    ///
    /// Output Idle state 1
    #[inline(always)]
    pub const fn ois1n(&self) -> bool {
        (self.bits & Ctlr2::OIS1N_MASK) != 0
    }

    /// Read **OIS1** `[8:8]`
    ///
    /// Output Idle state 1
    #[inline(always)]
    pub const fn ois1(&self) -> bool {
        (self.bits & Ctlr2::OIS1_MASK) != 0
    }

    /// Read **TI1S** `[7:7]`
    ///
    /// TI1 selection
    #[inline(always)]
    pub const fn ti1s(&self) -> bool {
        (self.bits & Ctlr2::TI1S_MASK) != 0
    }

    /// Read **MMS** `[6:4]`
    ///
    /// Master mode selection
    #[inline(always)]
    pub const fn mms(&self) -> u32 {
        (self.bits & Ctlr2::MMS_MASK) >> Ctlr2::MMS_OFFSET
    }

    /// Read **CCDS** `[3:3]`
    ///
    /// Capture/compare DMA
    /// selection
    #[inline(always)]
    pub const fn ccds(&self) -> bool {
        (self.bits & Ctlr2::CCDS_MASK) != 0
    }

    /// Read **CCUS** `[2:2]`
    ///
    /// Capture/compare control update
    /// selection
    #[inline(always)]
    pub const fn ccus(&self) -> bool {
        (self.bits & Ctlr2::CCUS_MASK) != 0
    }

    /// Read **CCPC** `[0:0]`
    ///
    /// Capture/compare preloaded
    /// control
    #[inline(always)]
    pub const fn ccpc(&self) -> bool {
        (self.bits & Ctlr2::CCPC_MASK) != 0
    }

    /// Set **OIS4** `[14:14]`
    ///
    /// Output Idle state 4
    #[inline(always)]
    pub const fn with_ois4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS4_MASK
            } else {
                self.bits & !Ctlr2::OIS4_MASK
            },
        }
    }

    /// Set **OIS3N** `[13:13]`
    ///
    /// Output Idle state 3
    #[inline(always)]
    pub const fn with_ois3n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS3N_MASK
            } else {
                self.bits & !Ctlr2::OIS3N_MASK
            },
        }
    }

    /// Set **OIS3** `[12:12]`
    ///
    /// Output Idle state 3
    #[inline(always)]
    pub const fn with_ois3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS3_MASK
            } else {
                self.bits & !Ctlr2::OIS3_MASK
            },
        }
    }

    /// Set **OIS2N** `[11:11]`
    ///
    /// Output Idle state 2
    #[inline(always)]
    pub const fn with_ois2n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS2N_MASK
            } else {
                self.bits & !Ctlr2::OIS2N_MASK
            },
        }
    }

    /// Set **OIS2** `[10:10]`
    ///
    /// Output Idle state 2
    #[inline(always)]
    pub const fn with_ois2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS2_MASK
            } else {
                self.bits & !Ctlr2::OIS2_MASK
            },
        }
    }

    /// Set **OIS1N** `[9:9]`
    ///
    /// Output Idle state 1
    #[inline(always)]
    pub const fn with_ois1n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS1N_MASK
            } else {
                self.bits & !Ctlr2::OIS1N_MASK
            },
        }
    }

    /// Set **OIS1** `[8:8]`
    ///
    /// Output Idle state 1
    #[inline(always)]
    pub const fn with_ois1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::OIS1_MASK
            } else {
                self.bits & !Ctlr2::OIS1_MASK
            },
        }
    }

    /// Set **TI1S** `[7:7]`
    ///
    /// TI1 selection
    #[inline(always)]
    pub const fn with_ti1s(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::TI1S_MASK
            } else {
                self.bits & !Ctlr2::TI1S_MASK
            },
        }
    }

    /// Set **MMS** `[6:4]`
    ///
    /// Master mode selection
    #[inline(always)]
    pub const fn with_mms(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr2::MMS_MASK) | ((value << Ctlr2::MMS_OFFSET) & Ctlr2::MMS_MASK),
        }
    }

    /// Set **CCDS** `[3:3]`
    ///
    /// Capture/compare DMA
    /// selection
    #[inline(always)]
    pub const fn with_ccds(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CCDS_MASK
            } else {
                self.bits & !Ctlr2::CCDS_MASK
            },
        }
    }

    /// Set **CCUS** `[2:2]`
    ///
    /// Capture/compare control update
    /// selection
    #[inline(always)]
    pub const fn with_ccus(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CCUS_MASK
            } else {
                self.bits & !Ctlr2::CCUS_MASK
            },
        }
    }

    /// Set **CCPC** `[0:0]`
    ///
    /// Capture/compare preloaded
    /// control
    #[inline(always)]
    pub const fn with_ccpc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CCPC_MASK
            } else {
                self.bits & !Ctlr2::CCPC_MASK
            },
        }
    }
}

impl Default for Ctlr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctlr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ctlr2Val {
        Ctlr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ctlr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ctlr2Val) -> Ctlr2Val>(&self, f: F) {
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
/// - **ETP** `[15]` (RO)
///
///   External trigger polarity
///
/// - **ECE** `[14]` (RW)
///
///   External clock enable
///
/// - **ETPS** `[13:12]` (RW)
///
///   External trigger prescaler
///
/// - **ETF** `[11:8]` (RW)
///
///   External trigger filter
///
/// - **MSM** `[7]` (RW)
///
///   Master/Slave mode
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Smcfgr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Smcfgr {
    // ======== Field Constants ========

    /// **ETP**
    ///
    /// External trigger polarity
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ETP_OFFSET: u32 = 15;
    pub const ETP_MASK: u32 = 0x00008000;
    pub const ETP: u32 = 1 << 15;

    /// **ECE**
    ///
    /// External clock enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ECE_OFFSET: u32 = 14;
    pub const ECE_MASK: u32 = 0x00004000;
    pub const ECE: u32 = 1 << 14;

    /// **ETPS**
    ///
    /// External trigger prescaler
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const ETPS_OFFSET: u32 = 12;
    pub const ETPS_MASK: u32 = 0x00003000;

    /// **ETF**
    ///
    /// External trigger filter
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const ETF_OFFSET: u32 = 8;
    pub const ETF_MASK: u32 = 0x00000f00;

    /// **MSM**
    ///
    /// Master/Slave mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MSM_OFFSET: u32 = 7;
    pub const MSM_MASK: u32 = 0x00000080;
    pub const MSM: u32 = 1 << 7;

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

    /// **ETP** `[15]` (RO)
    ///
    /// External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> bool {
        (self.value.read_bits() & Self::ETP_MASK) != 0
    }

    /// **ECE** `[14]` (RW)
    ///
    /// External clock enable
    #[inline(always)]
    pub fn ece(&self) -> bool {
        (self.value.read_bits() & Self::ECE_MASK) != 0
    }

    /// Set ECE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ece(&self) {
        self.value.modify_bits(|r| r | Self::ECE_MASK);
    }

    /// Clear ECE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ece(&self) {
        self.value.modify_bits(|r| r & !Self::ECE_MASK);
    }

    /// **ETPS** `[13:12]` (RW)
    ///
    /// External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> u32 {
        (self.value.read_bits() & Self::ETPS_MASK) >> Self::ETPS_OFFSET
    }

    /// Write **ETPS** `[13:12]` (RW)
    ///
    /// External trigger prescaler
    #[inline(always)]
    pub fn write_etps(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETPS_MASK) | ((value << Self::ETPS_OFFSET) & Self::ETPS_MASK)
        });
    }

    /// **ETF** `[11:8]` (RW)
    ///
    /// External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> u32 {
        (self.value.read_bits() & Self::ETF_MASK) >> Self::ETF_OFFSET
    }

    /// Write **ETF** `[11:8]` (RW)
    ///
    /// External trigger filter
    #[inline(always)]
    pub fn write_etf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETF_MASK) | ((value << Self::ETF_OFFSET) & Self::ETF_MASK)
        });
    }

    /// **MSM** `[7]` (RW)
    ///
    /// Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> bool {
        (self.value.read_bits() & Self::MSM_MASK) != 0
    }

    /// Set MSM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_msm(&self) {
        self.value.modify_bits(|r| r | Self::MSM_MASK);
    }

    /// Clear MSM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_msm(&self) {
        self.value.modify_bits(|r| r & !Self::MSM_MASK);
    }

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

    /// Read **ETP** `[15:15]`
    ///
    /// External trigger polarity
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        (self.bits & Smcfgr::ETP_MASK) != 0
    }

    /// Read **ECE** `[14:14]`
    ///
    /// External clock enable
    #[inline(always)]
    pub const fn ece(&self) -> bool {
        (self.bits & Smcfgr::ECE_MASK) != 0
    }

    /// Read **ETPS** `[13:12]`
    ///
    /// External trigger prescaler
    #[inline(always)]
    pub const fn etps(&self) -> u32 {
        (self.bits & Smcfgr::ETPS_MASK) >> Smcfgr::ETPS_OFFSET
    }

    /// Read **ETF** `[11:8]`
    ///
    /// External trigger filter
    #[inline(always)]
    pub const fn etf(&self) -> u32 {
        (self.bits & Smcfgr::ETF_MASK) >> Smcfgr::ETF_OFFSET
    }

    /// Read **MSM** `[7:7]`
    ///
    /// Master/Slave mode
    #[inline(always)]
    pub const fn msm(&self) -> bool {
        (self.bits & Smcfgr::MSM_MASK) != 0
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

    /// Set **ECE** `[14:14]`
    ///
    /// External clock enable
    #[inline(always)]
    pub const fn with_ece(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Smcfgr::ECE_MASK
            } else {
                self.bits & !Smcfgr::ECE_MASK
            },
        }
    }

    /// Set **ETPS** `[13:12]`
    ///
    /// External trigger prescaler
    #[inline(always)]
    pub const fn with_etps(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Smcfgr::ETPS_MASK)
                | ((value << Smcfgr::ETPS_OFFSET) & Smcfgr::ETPS_MASK),
        }
    }

    /// Set **ETF** `[11:8]`
    ///
    /// External trigger filter
    #[inline(always)]
    pub const fn with_etf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Smcfgr::ETF_MASK)
                | ((value << Smcfgr::ETF_OFFSET) & Smcfgr::ETF_MASK),
        }
    }

    /// Set **MSM** `[7:7]`
    ///
    /// Master/Slave mode
    #[inline(always)]
    pub const fn with_msm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Smcfgr::MSM_MASK
            } else {
                self.bits & !Smcfgr::MSM_MASK
            },
        }
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
/// - **TDE** `[14]` (RW)
///
///   Trigger DMA request enable
///
/// - **COMDE** `[13]` (RW)
///
///   COM DMA request enable
///
/// - **CC4DE** `[12]` (RW)
///
///   Capture/Compare 4 DMA request
///   enable
///
/// - **CC3DE** `[11]` (RW)
///
///   Capture/Compare 3 DMA request
///   enable
///
/// - **CC2DE** `[10]` (RW)
///
///   Capture/Compare 2 DMA request
///   enable
///
/// - **CC1DE** `[9]` (RW)
///
///   Capture/Compare 1 DMA request
///   enable
///
/// - **UDE** `[8]` (RW)
///
///   Update DMA request enable
///
/// - **BIE** `[7]` (RW)
///
///   Break interrupt enable
///
/// - **TIE** `[6]` (RW)
///
///   Trigger interrupt enable
///
/// - **COMIE** `[5]` (RW)
///
///   COM interrupt enable
///
/// - **CC4IE** `[4]` (RW)
///
///   Capture/Compare 4 interrupt
///   enable
///
/// - **CC3IE** `[3]` (RW)
///
///   Capture/Compare 3 interrupt
///   enable
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmaintenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmaintenr {
    // ======== Field Constants ========

    /// **TDE**
    ///
    /// Trigger DMA request enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TDE_OFFSET: u32 = 14;
    pub const TDE_MASK: u32 = 0x00004000;
    pub const TDE: u32 = 1 << 14;

    /// **COMDE**
    ///
    /// COM DMA request enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const COMDE_OFFSET: u32 = 13;
    pub const COMDE_MASK: u32 = 0x00002000;
    pub const COMDE: u32 = 1 << 13;

    /// **CC4DE**
    ///
    /// Capture/Compare 4 DMA request
    /// enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CC4DE_OFFSET: u32 = 12;
    pub const CC4DE_MASK: u32 = 0x00001000;
    pub const CC4DE: u32 = 1 << 12;

    /// **CC3DE**
    ///
    /// Capture/Compare 3 DMA request
    /// enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CC3DE_OFFSET: u32 = 11;
    pub const CC3DE_MASK: u32 = 0x00000800;
    pub const CC3DE: u32 = 1 << 11;

    /// **CC2DE**
    ///
    /// Capture/Compare 2 DMA request
    /// enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CC2DE_OFFSET: u32 = 10;
    pub const CC2DE_MASK: u32 = 0x00000400;
    pub const CC2DE: u32 = 1 << 10;

    /// **CC1DE**
    ///
    /// Capture/Compare 1 DMA request
    /// enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CC1DE_OFFSET: u32 = 9;
    pub const CC1DE_MASK: u32 = 0x00000200;
    pub const CC1DE: u32 = 1 << 9;

    /// **UDE**
    ///
    /// Update DMA request enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const UDE_OFFSET: u32 = 8;
    pub const UDE_MASK: u32 = 0x00000100;
    pub const UDE: u32 = 1 << 8;

    /// **BIE**
    ///
    /// Break interrupt enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BIE_OFFSET: u32 = 7;
    pub const BIE_MASK: u32 = 0x00000080;
    pub const BIE: u32 = 1 << 7;

    /// **TIE**
    ///
    /// Trigger interrupt enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TIE_OFFSET: u32 = 6;
    pub const TIE_MASK: u32 = 0x00000040;
    pub const TIE: u32 = 1 << 6;

    /// **COMIE**
    ///
    /// COM interrupt enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COMIE_OFFSET: u32 = 5;
    pub const COMIE_MASK: u32 = 0x00000020;
    pub const COMIE: u32 = 1 << 5;

    /// **CC4IE**
    ///
    /// Capture/Compare 4 interrupt
    /// enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CC4IE_OFFSET: u32 = 4;
    pub const CC4IE_MASK: u32 = 0x00000010;
    pub const CC4IE: u32 = 1 << 4;

    /// **CC3IE**
    ///
    /// Capture/Compare 3 interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CC3IE_OFFSET: u32 = 3;
    pub const CC3IE_MASK: u32 = 0x00000008;
    pub const CC3IE: u32 = 1 << 3;

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

    /// **TDE** `[14]` (RW)
    ///
    /// Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> bool {
        (self.value.read_bits() & Self::TDE_MASK) != 0
    }

    /// Set TDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tde(&self) {
        self.value.modify_bits(|r| r | Self::TDE_MASK);
    }

    /// Clear TDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tde(&self) {
        self.value.modify_bits(|r| r & !Self::TDE_MASK);
    }

    /// **COMDE** `[13]` (RW)
    ///
    /// COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> bool {
        (self.value.read_bits() & Self::COMDE_MASK) != 0
    }

    /// Set COMDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_comde(&self) {
        self.value.modify_bits(|r| r | Self::COMDE_MASK);
    }

    /// Clear COMDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_comde(&self) {
        self.value.modify_bits(|r| r & !Self::COMDE_MASK);
    }

    /// **CC4DE** `[12]` (RW)
    ///
    /// Capture/Compare 4 DMA request
    /// enable
    #[inline(always)]
    pub fn cc4de(&self) -> bool {
        (self.value.read_bits() & Self::CC4DE_MASK) != 0
    }

    /// Set CC4DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc4de(&self) {
        self.value.modify_bits(|r| r | Self::CC4DE_MASK);
    }

    /// Clear CC4DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc4de(&self) {
        self.value.modify_bits(|r| r & !Self::CC4DE_MASK);
    }

    /// **CC3DE** `[11]` (RW)
    ///
    /// Capture/Compare 3 DMA request
    /// enable
    #[inline(always)]
    pub fn cc3de(&self) -> bool {
        (self.value.read_bits() & Self::CC3DE_MASK) != 0
    }

    /// Set CC3DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3de(&self) {
        self.value.modify_bits(|r| r | Self::CC3DE_MASK);
    }

    /// Clear CC3DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3de(&self) {
        self.value.modify_bits(|r| r & !Self::CC3DE_MASK);
    }

    /// **CC2DE** `[10]` (RW)
    ///
    /// Capture/Compare 2 DMA request
    /// enable
    #[inline(always)]
    pub fn cc2de(&self) -> bool {
        (self.value.read_bits() & Self::CC2DE_MASK) != 0
    }

    /// Set CC2DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2de(&self) {
        self.value.modify_bits(|r| r | Self::CC2DE_MASK);
    }

    /// Clear CC2DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2de(&self) {
        self.value.modify_bits(|r| r & !Self::CC2DE_MASK);
    }

    /// **CC1DE** `[9]` (RW)
    ///
    /// Capture/Compare 1 DMA request
    /// enable
    #[inline(always)]
    pub fn cc1de(&self) -> bool {
        (self.value.read_bits() & Self::CC1DE_MASK) != 0
    }

    /// Set CC1DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1de(&self) {
        self.value.modify_bits(|r| r | Self::CC1DE_MASK);
    }

    /// Clear CC1DE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1de(&self) {
        self.value.modify_bits(|r| r & !Self::CC1DE_MASK);
    }

    /// **UDE** `[8]` (RW)
    ///
    /// Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> bool {
        (self.value.read_bits() & Self::UDE_MASK) != 0
    }

    /// Set UDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ude(&self) {
        self.value.modify_bits(|r| r | Self::UDE_MASK);
    }

    /// Clear UDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ude(&self) {
        self.value.modify_bits(|r| r & !Self::UDE_MASK);
    }

    /// **BIE** `[7]` (RW)
    ///
    /// Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> bool {
        (self.value.read_bits() & Self::BIE_MASK) != 0
    }

    /// Set BIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bie(&self) {
        self.value.modify_bits(|r| r | Self::BIE_MASK);
    }

    /// Clear BIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bie(&self) {
        self.value.modify_bits(|r| r & !Self::BIE_MASK);
    }

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

    /// **COMIE** `[5]` (RW)
    ///
    /// COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> bool {
        (self.value.read_bits() & Self::COMIE_MASK) != 0
    }

    /// Set COMIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_comie(&self) {
        self.value.modify_bits(|r| r | Self::COMIE_MASK);
    }

    /// Clear COMIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_comie(&self) {
        self.value.modify_bits(|r| r & !Self::COMIE_MASK);
    }

    /// **CC4IE** `[4]` (RW)
    ///
    /// Capture/Compare 4 interrupt
    /// enable
    #[inline(always)]
    pub fn cc4ie(&self) -> bool {
        (self.value.read_bits() & Self::CC4IE_MASK) != 0
    }

    /// Set CC4IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc4ie(&self) {
        self.value.modify_bits(|r| r | Self::CC4IE_MASK);
    }

    /// Clear CC4IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc4ie(&self) {
        self.value.modify_bits(|r| r & !Self::CC4IE_MASK);
    }

    /// **CC3IE** `[3]` (RW)
    ///
    /// Capture/Compare 3 interrupt
    /// enable
    #[inline(always)]
    pub fn cc3ie(&self) -> bool {
        (self.value.read_bits() & Self::CC3IE_MASK) != 0
    }

    /// Set CC3IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3ie(&self) {
        self.value.modify_bits(|r| r | Self::CC3IE_MASK);
    }

    /// Clear CC3IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3ie(&self) {
        self.value.modify_bits(|r| r & !Self::CC3IE_MASK);
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

    /// Read **TDE** `[14:14]`
    ///
    /// Trigger DMA request enable
    #[inline(always)]
    pub const fn tde(&self) -> bool {
        (self.bits & Dmaintenr::TDE_MASK) != 0
    }

    /// Read **COMDE** `[13:13]`
    ///
    /// COM DMA request enable
    #[inline(always)]
    pub const fn comde(&self) -> bool {
        (self.bits & Dmaintenr::COMDE_MASK) != 0
    }

    /// Read **CC4DE** `[12:12]`
    ///
    /// Capture/Compare 4 DMA request
    /// enable
    #[inline(always)]
    pub const fn cc4de(&self) -> bool {
        (self.bits & Dmaintenr::CC4DE_MASK) != 0
    }

    /// Read **CC3DE** `[11:11]`
    ///
    /// Capture/Compare 3 DMA request
    /// enable
    #[inline(always)]
    pub const fn cc3de(&self) -> bool {
        (self.bits & Dmaintenr::CC3DE_MASK) != 0
    }

    /// Read **CC2DE** `[10:10]`
    ///
    /// Capture/Compare 2 DMA request
    /// enable
    #[inline(always)]
    pub const fn cc2de(&self) -> bool {
        (self.bits & Dmaintenr::CC2DE_MASK) != 0
    }

    /// Read **CC1DE** `[9:9]`
    ///
    /// Capture/Compare 1 DMA request
    /// enable
    #[inline(always)]
    pub const fn cc1de(&self) -> bool {
        (self.bits & Dmaintenr::CC1DE_MASK) != 0
    }

    /// Read **UDE** `[8:8]`
    ///
    /// Update DMA request enable
    #[inline(always)]
    pub const fn ude(&self) -> bool {
        (self.bits & Dmaintenr::UDE_MASK) != 0
    }

    /// Read **BIE** `[7:7]`
    ///
    /// Break interrupt enable
    #[inline(always)]
    pub const fn bie(&self) -> bool {
        (self.bits & Dmaintenr::BIE_MASK) != 0
    }

    /// Read **TIE** `[6:6]`
    ///
    /// Trigger interrupt enable
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        (self.bits & Dmaintenr::TIE_MASK) != 0
    }

    /// Read **COMIE** `[5:5]`
    ///
    /// COM interrupt enable
    #[inline(always)]
    pub const fn comie(&self) -> bool {
        (self.bits & Dmaintenr::COMIE_MASK) != 0
    }

    /// Read **CC4IE** `[4:4]`
    ///
    /// Capture/Compare 4 interrupt
    /// enable
    #[inline(always)]
    pub const fn cc4ie(&self) -> bool {
        (self.bits & Dmaintenr::CC4IE_MASK) != 0
    }

    /// Read **CC3IE** `[3:3]`
    ///
    /// Capture/Compare 3 interrupt
    /// enable
    #[inline(always)]
    pub const fn cc3ie(&self) -> bool {
        (self.bits & Dmaintenr::CC3IE_MASK) != 0
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

    /// Set **TDE** `[14:14]`
    ///
    /// Trigger DMA request enable
    #[inline(always)]
    pub const fn with_tde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::TDE_MASK
            } else {
                self.bits & !Dmaintenr::TDE_MASK
            },
        }
    }

    /// Set **COMDE** `[13:13]`
    ///
    /// COM DMA request enable
    #[inline(always)]
    pub const fn with_comde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::COMDE_MASK
            } else {
                self.bits & !Dmaintenr::COMDE_MASK
            },
        }
    }

    /// Set **CC4DE** `[12:12]`
    ///
    /// Capture/Compare 4 DMA request
    /// enable
    #[inline(always)]
    pub const fn with_cc4de(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC4DE_MASK
            } else {
                self.bits & !Dmaintenr::CC4DE_MASK
            },
        }
    }

    /// Set **CC3DE** `[11:11]`
    ///
    /// Capture/Compare 3 DMA request
    /// enable
    #[inline(always)]
    pub const fn with_cc3de(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC3DE_MASK
            } else {
                self.bits & !Dmaintenr::CC3DE_MASK
            },
        }
    }

    /// Set **CC2DE** `[10:10]`
    ///
    /// Capture/Compare 2 DMA request
    /// enable
    #[inline(always)]
    pub const fn with_cc2de(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC2DE_MASK
            } else {
                self.bits & !Dmaintenr::CC2DE_MASK
            },
        }
    }

    /// Set **CC1DE** `[9:9]`
    ///
    /// Capture/Compare 1 DMA request
    /// enable
    #[inline(always)]
    pub const fn with_cc1de(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC1DE_MASK
            } else {
                self.bits & !Dmaintenr::CC1DE_MASK
            },
        }
    }

    /// Set **UDE** `[8:8]`
    ///
    /// Update DMA request enable
    #[inline(always)]
    pub const fn with_ude(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::UDE_MASK
            } else {
                self.bits & !Dmaintenr::UDE_MASK
            },
        }
    }

    /// Set **BIE** `[7:7]`
    ///
    /// Break interrupt enable
    #[inline(always)]
    pub const fn with_bie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::BIE_MASK
            } else {
                self.bits & !Dmaintenr::BIE_MASK
            },
        }
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

    /// Set **COMIE** `[5:5]`
    ///
    /// COM interrupt enable
    #[inline(always)]
    pub const fn with_comie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::COMIE_MASK
            } else {
                self.bits & !Dmaintenr::COMIE_MASK
            },
        }
    }

    /// Set **CC4IE** `[4:4]`
    ///
    /// Capture/Compare 4 interrupt
    /// enable
    #[inline(always)]
    pub const fn with_cc4ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC4IE_MASK
            } else {
                self.bits & !Dmaintenr::CC4IE_MASK
            },
        }
    }

    /// Set **CC3IE** `[3:3]`
    ///
    /// Capture/Compare 3 interrupt
    /// enable
    #[inline(always)]
    pub const fn with_cc3ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaintenr::CC3IE_MASK
            } else {
                self.bits & !Dmaintenr::CC3IE_MASK
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
/// - **CC4OF** `[12]` (RW)
///
///   Capture/Compare 4 overcapture
///   flag
///
/// - **CC3OF** `[11]` (RW)
///
///   Capture/Compare 3 overcapture
///   flag
///
/// - **CC2OF** `[10]` (RW)
///
///   Capture/compare 2 overcapture
///   flag
///
/// - **CC1OF** `[9]` (RW)
///
///   Capture/Compare 1 overcapture
///   flag
///
/// - **BIF** `[7]` (RW)
///
///   Break interrupt flag
///
/// - **TIF** `[6]` (RW)
///
///   Trigger interrupt flag
///
/// - **COMIF** `[5]` (RW)
///
///   COM interrupt flag
///
/// - **CC4IF** `[4]` (RW)
///
///   Capture/Compare 4 interrupt
///   flag
///
/// - **CC3IF** `[3]` (RW)
///
///   Capture/Compare 3 interrupt
///   flag
///
/// - **CC2IF** `[2]` (RW)
///
///   Capture/Compare 2 interrupt
///   flag
///
/// - **CC1IF** `[1]` (RW)
///
///   Capture/compare 1 interrupt
///   flag
///
/// - **UIF** `[0]` (RW)
///
///   Update interrupt flag
#[repr(transparent)]
pub struct Intfr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intfr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intfr {
    // ======== Field Constants ========

    /// **CC4OF**
    ///
    /// Capture/Compare 4 overcapture
    /// flag
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CC4OF_OFFSET: u32 = 12;
    pub const CC4OF_MASK: u32 = 0x00001000;
    pub const CC4OF: u32 = 1 << 12;

    /// **CC3OF**
    ///
    /// Capture/Compare 3 overcapture
    /// flag
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CC3OF_OFFSET: u32 = 11;
    pub const CC3OF_MASK: u32 = 0x00000800;
    pub const CC3OF: u32 = 1 << 11;

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

    /// **BIF**
    ///
    /// Break interrupt flag
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BIF_OFFSET: u32 = 7;
    pub const BIF_MASK: u32 = 0x00000080;
    pub const BIF: u32 = 1 << 7;

    /// **TIF**
    ///
    /// Trigger interrupt flag
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TIF_OFFSET: u32 = 6;
    pub const TIF_MASK: u32 = 0x00000040;
    pub const TIF: u32 = 1 << 6;

    /// **COMIF**
    ///
    /// COM interrupt flag
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COMIF_OFFSET: u32 = 5;
    pub const COMIF_MASK: u32 = 0x00000020;
    pub const COMIF: u32 = 1 << 5;

    /// **CC4IF**
    ///
    /// Capture/Compare 4 interrupt
    /// flag
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CC4IF_OFFSET: u32 = 4;
    pub const CC4IF_MASK: u32 = 0x00000010;
    pub const CC4IF: u32 = 1 << 4;

    /// **CC3IF**
    ///
    /// Capture/Compare 3 interrupt
    /// flag
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CC3IF_OFFSET: u32 = 3;
    pub const CC3IF_MASK: u32 = 0x00000008;
    pub const CC3IF: u32 = 1 << 3;

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

    /// **CC4OF** `[12]` (RW)
    ///
    /// Capture/Compare 4 overcapture
    /// flag
    #[inline(always)]
    pub fn cc4of(&self) -> bool {
        (self.value.read_bits() & Self::CC4OF_MASK) != 0
    }

    /// Set CC4OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc4of(&self) {
        self.value.modify_bits(|r| r | Self::CC4OF_MASK);
    }

    /// Clear CC4OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc4of(&self) {
        self.value.modify_bits(|r| r & !Self::CC4OF_MASK);
    }

    /// **CC3OF** `[11]` (RW)
    ///
    /// Capture/Compare 3 overcapture
    /// flag
    #[inline(always)]
    pub fn cc3of(&self) -> bool {
        (self.value.read_bits() & Self::CC3OF_MASK) != 0
    }

    /// Set CC3OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3of(&self) {
        self.value.modify_bits(|r| r | Self::CC3OF_MASK);
    }

    /// Clear CC3OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3of(&self) {
        self.value.modify_bits(|r| r & !Self::CC3OF_MASK);
    }

    /// **CC2OF** `[10]` (RW)
    ///
    /// Capture/compare 2 overcapture
    /// flag
    #[inline(always)]
    pub fn cc2of(&self) -> bool {
        (self.value.read_bits() & Self::CC2OF_MASK) != 0
    }

    /// Set CC2OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2of(&self) {
        self.value.modify_bits(|r| r | Self::CC2OF_MASK);
    }

    /// Clear CC2OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2of(&self) {
        self.value.modify_bits(|r| r & !Self::CC2OF_MASK);
    }

    /// **CC1OF** `[9]` (RW)
    ///
    /// Capture/Compare 1 overcapture
    /// flag
    #[inline(always)]
    pub fn cc1of(&self) -> bool {
        (self.value.read_bits() & Self::CC1OF_MASK) != 0
    }

    /// Set CC1OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1of(&self) {
        self.value.modify_bits(|r| r | Self::CC1OF_MASK);
    }

    /// Clear CC1OF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1of(&self) {
        self.value.modify_bits(|r| r & !Self::CC1OF_MASK);
    }

    /// **BIF** `[7]` (RW)
    ///
    /// Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> bool {
        (self.value.read_bits() & Self::BIF_MASK) != 0
    }

    /// Set BIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bif(&self) {
        self.value.modify_bits(|r| r | Self::BIF_MASK);
    }

    /// Clear BIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bif(&self) {
        self.value.modify_bits(|r| r & !Self::BIF_MASK);
    }

    /// **TIF** `[6]` (RW)
    ///
    /// Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&self) -> bool {
        (self.value.read_bits() & Self::TIF_MASK) != 0
    }

    /// Set TIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tif(&self) {
        self.value.modify_bits(|r| r | Self::TIF_MASK);
    }

    /// Clear TIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tif(&self) {
        self.value.modify_bits(|r| r & !Self::TIF_MASK);
    }

    /// **COMIF** `[5]` (RW)
    ///
    /// COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> bool {
        (self.value.read_bits() & Self::COMIF_MASK) != 0
    }

    /// Set COMIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_comif(&self) {
        self.value.modify_bits(|r| r | Self::COMIF_MASK);
    }

    /// Clear COMIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_comif(&self) {
        self.value.modify_bits(|r| r & !Self::COMIF_MASK);
    }

    /// **CC4IF** `[4]` (RW)
    ///
    /// Capture/Compare 4 interrupt
    /// flag
    #[inline(always)]
    pub fn cc4if(&self) -> bool {
        (self.value.read_bits() & Self::CC4IF_MASK) != 0
    }

    /// Set CC4IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc4if(&self) {
        self.value.modify_bits(|r| r | Self::CC4IF_MASK);
    }

    /// Clear CC4IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc4if(&self) {
        self.value.modify_bits(|r| r & !Self::CC4IF_MASK);
    }

    /// **CC3IF** `[3]` (RW)
    ///
    /// Capture/Compare 3 interrupt
    /// flag
    #[inline(always)]
    pub fn cc3if(&self) -> bool {
        (self.value.read_bits() & Self::CC3IF_MASK) != 0
    }

    /// Set CC3IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3if(&self) {
        self.value.modify_bits(|r| r | Self::CC3IF_MASK);
    }

    /// Clear CC3IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3if(&self) {
        self.value.modify_bits(|r| r & !Self::CC3IF_MASK);
    }

    /// **CC2IF** `[2]` (RW)
    ///
    /// Capture/Compare 2 interrupt
    /// flag
    #[inline(always)]
    pub fn cc2if(&self) -> bool {
        (self.value.read_bits() & Self::CC2IF_MASK) != 0
    }

    /// Set CC2IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2if(&self) {
        self.value.modify_bits(|r| r | Self::CC2IF_MASK);
    }

    /// Clear CC2IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2if(&self) {
        self.value.modify_bits(|r| r & !Self::CC2IF_MASK);
    }

    /// **CC1IF** `[1]` (RW)
    ///
    /// Capture/compare 1 interrupt
    /// flag
    #[inline(always)]
    pub fn cc1if(&self) -> bool {
        (self.value.read_bits() & Self::CC1IF_MASK) != 0
    }

    /// Set CC1IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1if(&self) {
        self.value.modify_bits(|r| r | Self::CC1IF_MASK);
    }

    /// Clear CC1IF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1if(&self) {
        self.value.modify_bits(|r| r & !Self::CC1IF_MASK);
    }

    /// **UIF** `[0]` (RW)
    ///
    /// Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> bool {
        (self.value.read_bits() & Self::UIF_MASK) != 0
    }

    /// Set UIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uif(&self) {
        self.value.modify_bits(|r| r | Self::UIF_MASK);
    }

    /// Clear UIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uif(&self) {
        self.value.modify_bits(|r| r & !Self::UIF_MASK);
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

    /// Read **CC4OF** `[12:12]`
    ///
    /// Capture/Compare 4 overcapture
    /// flag
    #[inline(always)]
    pub const fn cc4of(&self) -> bool {
        (self.bits & Intfr::CC4OF_MASK) != 0
    }

    /// Read **CC3OF** `[11:11]`
    ///
    /// Capture/Compare 3 overcapture
    /// flag
    #[inline(always)]
    pub const fn cc3of(&self) -> bool {
        (self.bits & Intfr::CC3OF_MASK) != 0
    }

    /// Read **CC2OF** `[10:10]`
    ///
    /// Capture/compare 2 overcapture
    /// flag
    #[inline(always)]
    pub const fn cc2of(&self) -> bool {
        (self.bits & Intfr::CC2OF_MASK) != 0
    }

    /// Read **CC1OF** `[9:9]`
    ///
    /// Capture/Compare 1 overcapture
    /// flag
    #[inline(always)]
    pub const fn cc1of(&self) -> bool {
        (self.bits & Intfr::CC1OF_MASK) != 0
    }

    /// Read **BIF** `[7:7]`
    ///
    /// Break interrupt flag
    #[inline(always)]
    pub const fn bif(&self) -> bool {
        (self.bits & Intfr::BIF_MASK) != 0
    }

    /// Read **TIF** `[6:6]`
    ///
    /// Trigger interrupt flag
    #[inline(always)]
    pub const fn tif(&self) -> bool {
        (self.bits & Intfr::TIF_MASK) != 0
    }

    /// Read **COMIF** `[5:5]`
    ///
    /// COM interrupt flag
    #[inline(always)]
    pub const fn comif(&self) -> bool {
        (self.bits & Intfr::COMIF_MASK) != 0
    }

    /// Read **CC4IF** `[4:4]`
    ///
    /// Capture/Compare 4 interrupt
    /// flag
    #[inline(always)]
    pub const fn cc4if(&self) -> bool {
        (self.bits & Intfr::CC4IF_MASK) != 0
    }

    /// Read **CC3IF** `[3:3]`
    ///
    /// Capture/Compare 3 interrupt
    /// flag
    #[inline(always)]
    pub const fn cc3if(&self) -> bool {
        (self.bits & Intfr::CC3IF_MASK) != 0
    }

    /// Read **CC2IF** `[2:2]`
    ///
    /// Capture/Compare 2 interrupt
    /// flag
    #[inline(always)]
    pub const fn cc2if(&self) -> bool {
        (self.bits & Intfr::CC2IF_MASK) != 0
    }

    /// Read **CC1IF** `[1:1]`
    ///
    /// Capture/compare 1 interrupt
    /// flag
    #[inline(always)]
    pub const fn cc1if(&self) -> bool {
        (self.bits & Intfr::CC1IF_MASK) != 0
    }

    /// Read **UIF** `[0:0]`
    ///
    /// Update interrupt flag
    #[inline(always)]
    pub const fn uif(&self) -> bool {
        (self.bits & Intfr::UIF_MASK) != 0
    }

    /// Set **CC4OF** `[12:12]`
    ///
    /// Capture/Compare 4 overcapture
    /// flag
    #[inline(always)]
    pub const fn with_cc4of(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC4OF_MASK
            } else {
                self.bits & !Intfr::CC4OF_MASK
            },
        }
    }

    /// Set **CC3OF** `[11:11]`
    ///
    /// Capture/Compare 3 overcapture
    /// flag
    #[inline(always)]
    pub const fn with_cc3of(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC3OF_MASK
            } else {
                self.bits & !Intfr::CC3OF_MASK
            },
        }
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

    /// Set **BIF** `[7:7]`
    ///
    /// Break interrupt flag
    #[inline(always)]
    pub const fn with_bif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::BIF_MASK
            } else {
                self.bits & !Intfr::BIF_MASK
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

    /// Set **COMIF** `[5:5]`
    ///
    /// COM interrupt flag
    #[inline(always)]
    pub const fn with_comif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::COMIF_MASK
            } else {
                self.bits & !Intfr::COMIF_MASK
            },
        }
    }

    /// Set **CC4IF** `[4:4]`
    ///
    /// Capture/Compare 4 interrupt
    /// flag
    #[inline(always)]
    pub const fn with_cc4if(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC4IF_MASK
            } else {
                self.bits & !Intfr::CC4IF_MASK
            },
        }
    }

    /// Set **CC3IF** `[3:3]`
    ///
    /// Capture/Compare 3 interrupt
    /// flag
    #[inline(always)]
    pub const fn with_cc3if(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::CC3IF_MASK
            } else {
                self.bits & !Intfr::CC3IF_MASK
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
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntfrVal {
        IntfrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntfrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntfrVal) -> IntfrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
/// - **BG** `[7]` (WO)
///
///   Break generation
///
/// - **TG** `[6]` (WO)
///
///   Trigger generation
///
/// - **COMG** `[5]` (WO)
///
///   Capture/Compare control update
///   generation
///
/// - **CC4G** `[4]` (WO)
///
///   Capture/compare 4
///   generation
///
/// - **CC3G** `[3]` (WO)
///
///   Capture/compare 3
///   generation
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Swevgr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Swevgr {
    // ======== Field Constants ========

    /// **BG**
    ///
    /// Break generation
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BG_OFFSET: u32 = 7;
    pub const BG_MASK: u32 = 0x00000080;
    pub const BG: u32 = 1 << 7;

    /// **TG**
    ///
    /// Trigger generation
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TG_OFFSET: u32 = 6;
    pub const TG_MASK: u32 = 0x00000040;
    pub const TG: u32 = 1 << 6;

    /// **COMG**
    ///
    /// Capture/Compare control update
    /// generation
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COMG_OFFSET: u32 = 5;
    pub const COMG_MASK: u32 = 0x00000020;
    pub const COMG: u32 = 1 << 5;

    /// **CC4G**
    ///
    /// Capture/compare 4
    /// generation
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CC4G_OFFSET: u32 = 4;
    pub const CC4G_MASK: u32 = 0x00000010;
    pub const CC4G: u32 = 1 << 4;

    /// **CC3G**
    ///
    /// Capture/compare 3
    /// generation
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CC3G_OFFSET: u32 = 3;
    pub const CC3G_MASK: u32 = 0x00000008;
    pub const CC3G: u32 = 1 << 3;

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

    /// Trigger **BG** (WO)
    ///
    /// Break generation
    #[inline(always)]
    pub fn trigger_bg(&self) {
        self.value.write_bits(Self::BG_MASK);
    }

    /// Trigger **TG** (WO)
    ///
    /// Trigger generation
    #[inline(always)]
    pub fn trigger_tg(&self) {
        self.value.write_bits(Self::TG_MASK);
    }

    /// Trigger **COMG** (WO)
    ///
    /// Capture/Compare control update
    /// generation
    #[inline(always)]
    pub fn trigger_comg(&self) {
        self.value.write_bits(Self::COMG_MASK);
    }

    /// Trigger **CC4G** (WO)
    ///
    /// Capture/compare 4
    /// generation
    #[inline(always)]
    pub fn trigger_cc4g(&self) {
        self.value.write_bits(Self::CC4G_MASK);
    }

    /// Trigger **CC3G** (WO)
    ///
    /// Capture/compare 3
    /// generation
    #[inline(always)]
    pub fn trigger_cc3g(&self) {
        self.value.write_bits(Self::CC3G_MASK);
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

    /// Set **BG** `[7:7]`
    ///
    /// Break generation
    #[inline(always)]
    pub const fn with_bg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::BG_MASK
            } else {
                self.bits & !Swevgr::BG_MASK
            },
        }
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

    /// Set **COMG** `[5:5]`
    ///
    /// Capture/Compare control update
    /// generation
    #[inline(always)]
    pub const fn with_comg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::COMG_MASK
            } else {
                self.bits & !Swevgr::COMG_MASK
            },
        }
    }

    /// Set **CC4G** `[4:4]`
    ///
    /// Capture/compare 4
    /// generation
    #[inline(always)]
    pub const fn with_cc4g(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::CC4G_MASK
            } else {
                self.bits & !Swevgr::CC4G_MASK
            },
        }
    }

    /// Set **CC3G** `[3:3]`
    ///
    /// Capture/compare 3
    /// generation
    #[inline(always)]
    pub const fn with_cc3g(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swevgr::CC3G_MASK
            } else {
                self.bits & !Swevgr::CC3G_MASK
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
/// capture/compare mode register (output
/// mode)
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OC2CE** `[15]` (RW)
///
///   Output Compare 2 clear
///   enable
///
/// - **OC2M** `[14:12]` (RW)
///
///   Output Compare 2 mode
///
/// - **OC2PE** `[11]` (RW)
///
///   Output Compare 2 preload
///   enable
///
/// - **OC2FE** `[10]` (RW)
///
///   Output Compare 2 fast
///   enable
///
/// - **CC2S** `[9:8]` (RW)
///
///   Capture/Compare 2
///   selection
///
/// - **OC1CE** `[7]` (RW)
///
///   Output Compare 1 clear
///   enable
///
/// - **OC1M** `[6:4]` (RW)
///
///   Output Compare 1 mode
///
/// - **OC1PE** `[3]` (RW)
///
///   Output Compare 1 preload
///   enable
///
/// - **OC1FE** `[2]` (RW)
///
///   Output Compare 1 fast
///   enable
///
/// - **CC1S** `[1:0]` (RW)
///
///   Capture/Compare 1
///   selection
#[repr(transparent)]
pub struct Chctlr1Output {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Chctlr1Output {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chctlr1Output {
    // ======== Field Constants ========

    /// **OC2CE**
    ///
    /// Output Compare 2 clear
    /// enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OC2CE_OFFSET: u32 = 15;
    pub const OC2CE_MASK: u32 = 0x00008000;
    pub const OC2CE: u32 = 1 << 15;

    /// **OC2M**
    ///
    /// Output Compare 2 mode
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const OC2M_OFFSET: u32 = 12;
    pub const OC2M_MASK: u32 = 0x00007000;

    /// **OC2PE**
    ///
    /// Output Compare 2 preload
    /// enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OC2PE_OFFSET: u32 = 11;
    pub const OC2PE_MASK: u32 = 0x00000800;
    pub const OC2PE: u32 = 1 << 11;

    /// **OC2FE**
    ///
    /// Output Compare 2 fast
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
    /// Output Compare 1 clear
    /// enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OC1CE_OFFSET: u32 = 7;
    pub const OC1CE_MASK: u32 = 0x00000080;
    pub const OC1CE: u32 = 1 << 7;

    /// **OC1M**
    ///
    /// Output Compare 1 mode
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const OC1M_OFFSET: u32 = 4;
    pub const OC1M_MASK: u32 = 0x00000070;

    /// **OC1PE**
    ///
    /// Output Compare 1 preload
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OC1PE_OFFSET: u32 = 3;
    pub const OC1PE_MASK: u32 = 0x00000008;
    pub const OC1PE: u32 = 1 << 3;

    /// **OC1FE**
    ///
    /// Output Compare 1 fast
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
    /// Output Compare 2 clear
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
    /// Output Compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> u32 {
        (self.value.read_bits() & Self::OC2M_MASK) >> Self::OC2M_OFFSET
    }

    /// Write **OC2M** `[14:12]` (RW)
    ///
    /// Output Compare 2 mode
    #[inline(always)]
    pub fn write_oc2m(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OC2M_MASK) | ((value << Self::OC2M_OFFSET) & Self::OC2M_MASK)
        });
    }

    /// **OC2PE** `[11]` (RW)
    ///
    /// Output Compare 2 preload
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
    /// Output Compare 2 fast
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
    /// Output Compare 1 clear
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
    /// Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> u32 {
        (self.value.read_bits() & Self::OC1M_MASK) >> Self::OC1M_OFFSET
    }

    /// Write **OC1M** `[6:4]` (RW)
    ///
    /// Output Compare 1 mode
    #[inline(always)]
    pub fn write_oc1m(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OC1M_MASK) | ((value << Self::OC1M_OFFSET) & Self::OC1M_MASK)
        });
    }

    /// **OC1PE** `[3]` (RW)
    ///
    /// Output Compare 1 preload
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
    /// Output Compare 1 fast
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
    /// Output Compare 2 clear
    /// enable
    #[inline(always)]
    pub const fn oc2ce(&self) -> bool {
        (self.bits & Chctlr1Output::OC2CE_MASK) != 0
    }

    /// Read **OC2M** `[14:12]`
    ///
    /// Output Compare 2 mode
    #[inline(always)]
    pub const fn oc2m(&self) -> u32 {
        (self.bits & Chctlr1Output::OC2M_MASK) >> Chctlr1Output::OC2M_OFFSET
    }

    /// Read **OC2PE** `[11:11]`
    ///
    /// Output Compare 2 preload
    /// enable
    #[inline(always)]
    pub const fn oc2pe(&self) -> bool {
        (self.bits & Chctlr1Output::OC2PE_MASK) != 0
    }

    /// Read **OC2FE** `[10:10]`
    ///
    /// Output Compare 2 fast
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
    /// Output Compare 1 clear
    /// enable
    #[inline(always)]
    pub const fn oc1ce(&self) -> bool {
        (self.bits & Chctlr1Output::OC1CE_MASK) != 0
    }

    /// Read **OC1M** `[6:4]`
    ///
    /// Output Compare 1 mode
    #[inline(always)]
    pub const fn oc1m(&self) -> u32 {
        (self.bits & Chctlr1Output::OC1M_MASK) >> Chctlr1Output::OC1M_OFFSET
    }

    /// Read **OC1PE** `[3:3]`
    ///
    /// Output Compare 1 preload
    /// enable
    #[inline(always)]
    pub const fn oc1pe(&self) -> bool {
        (self.bits & Chctlr1Output::OC1PE_MASK) != 0
    }

    /// Read **OC1FE** `[2:2]`
    ///
    /// Output Compare 1 fast
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
    /// Output Compare 2 clear
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
    /// Output Compare 2 mode
    #[inline(always)]
    pub const fn with_oc2m(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Output::OC2M_MASK)
                | ((value << Chctlr1Output::OC2M_OFFSET) & Chctlr1Output::OC2M_MASK),
        }
    }

    /// Set **OC2PE** `[11:11]`
    ///
    /// Output Compare 2 preload
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
    /// Output Compare 2 fast
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
    /// Output Compare 1 clear
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
    /// Output Compare 1 mode
    #[inline(always)]
    pub const fn with_oc1m(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Output::OC1M_MASK)
                | ((value << Chctlr1Output::OC1M_OFFSET) & Chctlr1Output::OC1M_MASK),
        }
    }

    /// Set **OC1PE** `[3:3]`
    ///
    /// Output Compare 1 preload
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
    /// Output Compare 1 fast
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
/// - **IC2PCS** `[11:10]` (RW)
///
///   Input capture 2 prescaler
///
/// - **CC2S** `[9:8]` (RW)
///
///   Capture/Compare 2
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Chctlr1Input {
    type Target = crate::common::VolatileCell<u32>;
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

    /// **IC2PCS**
    ///
    /// Input capture 2 prescaler
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const IC2PCS_OFFSET: u32 = 10;
    pub const IC2PCS_MASK: u32 = 0x00000c00;

    /// **CC2S**
    ///
    /// Capture/Compare 2
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

    /// **IC2PCS** `[11:10]` (RW)
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2pcs(&self) -> u32 {
        (self.value.read_bits() & Self::IC2PCS_MASK) >> Self::IC2PCS_OFFSET
    }

    /// Write **IC2PCS** `[11:10]` (RW)
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub fn write_ic2pcs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC2PCS_MASK) | ((value << Self::IC2PCS_OFFSET) & Self::IC2PCS_MASK)
        });
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

    /// Read **IC2PCS** `[11:10]`
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub const fn ic2pcs(&self) -> u32 {
        (self.bits & Chctlr1Input::IC2PCS_MASK) >> Chctlr1Input::IC2PCS_OFFSET
    }

    /// Read **CC2S** `[9:8]`
    ///
    /// Capture/Compare 2
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

    /// Set **IC2PCS** `[11:10]`
    ///
    /// Input capture 2 prescaler
    #[inline(always)]
    pub const fn with_ic2pcs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr1Input::IC2PCS_MASK)
                | ((value << Chctlr1Input::IC2PCS_OFFSET) & Chctlr1Input::IC2PCS_MASK),
        }
    }

    /// Set **CC2S** `[9:8]`
    ///
    /// Capture/Compare 2
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

/// CHCTLR2_Output Register
///
/// capture/compare mode register (output
/// mode)
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OC4CE** `[15]` (RW)
///
///   Output compare 4 clear
///   enable
///
/// - **OC4M** `[14:12]` (RW)
///
///   Output compare 4 mode
///
/// - **OC4PE** `[11]` (RW)
///
///   Output compare 4 preload
///   enable
///
/// - **OC4FE** `[10]` (RW)
///
///   Output compare 4 fast
///   enable
///
/// - **CC4S** `[9:8]` (RW)
///
///   Capture/Compare 4
///   selection
///
/// - **OC3CE** `[7]` (RW)
///
///   Output compare 3 clear
///   enable
///
/// - **OC3M** `[6:4]` (RW)
///
///   Output compare 3 mode
///
/// - **OC3PE** `[3]` (RW)
///
///   Output compare 3 preload
///   enable
///
/// - **OC3FE** `[2]` (RW)
///
///   Output compare 3 fast
///   enable
///
/// - **CC3S** `[1:0]` (RW)
///
///   Capture/Compare 3
///   selection
#[repr(transparent)]
pub struct Chctlr2Output {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Chctlr2Output {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chctlr2Output {
    // ======== Field Constants ========

    /// **OC4CE**
    ///
    /// Output compare 4 clear
    /// enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const OC4CE_OFFSET: u32 = 15;
    pub const OC4CE_MASK: u32 = 0x00008000;
    pub const OC4CE: u32 = 1 << 15;

    /// **OC4M**
    ///
    /// Output compare 4 mode
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const OC4M_OFFSET: u32 = 12;
    pub const OC4M_MASK: u32 = 0x00007000;

    /// **OC4PE**
    ///
    /// Output compare 4 preload
    /// enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OC4PE_OFFSET: u32 = 11;
    pub const OC4PE_MASK: u32 = 0x00000800;
    pub const OC4PE: u32 = 1 << 11;

    /// **OC4FE**
    ///
    /// Output compare 4 fast
    /// enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OC4FE_OFFSET: u32 = 10;
    pub const OC4FE_MASK: u32 = 0x00000400;
    pub const OC4FE: u32 = 1 << 10;

    /// **CC4S**
    ///
    /// Capture/Compare 4
    /// selection
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const CC4S_OFFSET: u32 = 8;
    pub const CC4S_MASK: u32 = 0x00000300;

    /// **OC3CE**
    ///
    /// Output compare 3 clear
    /// enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OC3CE_OFFSET: u32 = 7;
    pub const OC3CE_MASK: u32 = 0x00000080;
    pub const OC3CE: u32 = 1 << 7;

    /// **OC3M**
    ///
    /// Output compare 3 mode
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const OC3M_OFFSET: u32 = 4;
    pub const OC3M_MASK: u32 = 0x00000070;

    /// **OC3PE**
    ///
    /// Output compare 3 preload
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const OC3PE_OFFSET: u32 = 3;
    pub const OC3PE_MASK: u32 = 0x00000008;
    pub const OC3PE: u32 = 1 << 3;

    /// **OC3FE**
    ///
    /// Output compare 3 fast
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OC3FE_OFFSET: u32 = 2;
    pub const OC3FE_MASK: u32 = 0x00000004;
    pub const OC3FE: u32 = 1 << 2;

    /// **CC3S**
    ///
    /// Capture/Compare 3
    /// selection
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CC3S_OFFSET: u32 = 0;
    pub const CC3S_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **OC4CE** `[15]` (RW)
    ///
    /// Output compare 4 clear
    /// enable
    #[inline(always)]
    pub fn oc4ce(&self) -> bool {
        (self.value.read_bits() & Self::OC4CE_MASK) != 0
    }

    /// Set OC4CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc4ce(&self) {
        self.value.modify_bits(|r| r | Self::OC4CE_MASK);
    }

    /// Clear OC4CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc4ce(&self) {
        self.value.modify_bits(|r| r & !Self::OC4CE_MASK);
    }

    /// **OC4M** `[14:12]` (RW)
    ///
    /// Output compare 4 mode
    #[inline(always)]
    pub fn oc4m(&self) -> u32 {
        (self.value.read_bits() & Self::OC4M_MASK) >> Self::OC4M_OFFSET
    }

    /// Write **OC4M** `[14:12]` (RW)
    ///
    /// Output compare 4 mode
    #[inline(always)]
    pub fn write_oc4m(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OC4M_MASK) | ((value << Self::OC4M_OFFSET) & Self::OC4M_MASK)
        });
    }

    /// **OC4PE** `[11]` (RW)
    ///
    /// Output compare 4 preload
    /// enable
    #[inline(always)]
    pub fn oc4pe(&self) -> bool {
        (self.value.read_bits() & Self::OC4PE_MASK) != 0
    }

    /// Set OC4PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc4pe(&self) {
        self.value.modify_bits(|r| r | Self::OC4PE_MASK);
    }

    /// Clear OC4PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc4pe(&self) {
        self.value.modify_bits(|r| r & !Self::OC4PE_MASK);
    }

    /// **OC4FE** `[10]` (RW)
    ///
    /// Output compare 4 fast
    /// enable
    #[inline(always)]
    pub fn oc4fe(&self) -> bool {
        (self.value.read_bits() & Self::OC4FE_MASK) != 0
    }

    /// Set OC4FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc4fe(&self) {
        self.value.modify_bits(|r| r | Self::OC4FE_MASK);
    }

    /// Clear OC4FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc4fe(&self) {
        self.value.modify_bits(|r| r & !Self::OC4FE_MASK);
    }

    /// **CC4S** `[9:8]` (RW)
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub fn cc4s(&self) -> u32 {
        (self.value.read_bits() & Self::CC4S_MASK) >> Self::CC4S_OFFSET
    }

    /// Write **CC4S** `[9:8]` (RW)
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub fn write_cc4s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC4S_MASK) | ((value << Self::CC4S_OFFSET) & Self::CC4S_MASK)
        });
    }

    /// **OC3CE** `[7]` (RW)
    ///
    /// Output compare 3 clear
    /// enable
    #[inline(always)]
    pub fn oc3ce(&self) -> bool {
        (self.value.read_bits() & Self::OC3CE_MASK) != 0
    }

    /// Set OC3CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc3ce(&self) {
        self.value.modify_bits(|r| r | Self::OC3CE_MASK);
    }

    /// Clear OC3CE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc3ce(&self) {
        self.value.modify_bits(|r| r & !Self::OC3CE_MASK);
    }

    /// **OC3M** `[6:4]` (RW)
    ///
    /// Output compare 3 mode
    #[inline(always)]
    pub fn oc3m(&self) -> u32 {
        (self.value.read_bits() & Self::OC3M_MASK) >> Self::OC3M_OFFSET
    }

    /// Write **OC3M** `[6:4]` (RW)
    ///
    /// Output compare 3 mode
    #[inline(always)]
    pub fn write_oc3m(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OC3M_MASK) | ((value << Self::OC3M_OFFSET) & Self::OC3M_MASK)
        });
    }

    /// **OC3PE** `[3]` (RW)
    ///
    /// Output compare 3 preload
    /// enable
    #[inline(always)]
    pub fn oc3pe(&self) -> bool {
        (self.value.read_bits() & Self::OC3PE_MASK) != 0
    }

    /// Set OC3PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc3pe(&self) {
        self.value.modify_bits(|r| r | Self::OC3PE_MASK);
    }

    /// Clear OC3PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc3pe(&self) {
        self.value.modify_bits(|r| r & !Self::OC3PE_MASK);
    }

    /// **OC3FE** `[2]` (RW)
    ///
    /// Output compare 3 fast
    /// enable
    #[inline(always)]
    pub fn oc3fe(&self) -> bool {
        (self.value.read_bits() & Self::OC3FE_MASK) != 0
    }

    /// Set OC3FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oc3fe(&self) {
        self.value.modify_bits(|r| r | Self::OC3FE_MASK);
    }

    /// Clear OC3FE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oc3fe(&self) {
        self.value.modify_bits(|r| r & !Self::OC3FE_MASK);
    }

    /// **CC3S** `[1:0]` (RW)
    ///
    /// Capture/Compare 3
    /// selection
    #[inline(always)]
    pub fn cc3s(&self) -> u32 {
        (self.value.read_bits() & Self::CC3S_MASK) >> Self::CC3S_OFFSET
    }

    /// Write **CC3S** `[1:0]` (RW)
    ///
    /// Capture/Compare 3
    /// selection
    #[inline(always)]
    pub fn write_cc3s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC3S_MASK) | ((value << Self::CC3S_OFFSET) & Self::CC3S_MASK)
        });
    }
}

/// CHCTLR2_Output value type for batch read/modify/write operations
///
/// Use `chctlr2output.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chctlr2output.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chctlr2OutputVal {
    bits: u32,
}

impl Chctlr2OutputVal {
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

    /// Read **OC4CE** `[15:15]`
    ///
    /// Output compare 4 clear
    /// enable
    #[inline(always)]
    pub const fn oc4ce(&self) -> bool {
        (self.bits & Chctlr2Output::OC4CE_MASK) != 0
    }

    /// Read **OC4M** `[14:12]`
    ///
    /// Output compare 4 mode
    #[inline(always)]
    pub const fn oc4m(&self) -> u32 {
        (self.bits & Chctlr2Output::OC4M_MASK) >> Chctlr2Output::OC4M_OFFSET
    }

    /// Read **OC4PE** `[11:11]`
    ///
    /// Output compare 4 preload
    /// enable
    #[inline(always)]
    pub const fn oc4pe(&self) -> bool {
        (self.bits & Chctlr2Output::OC4PE_MASK) != 0
    }

    /// Read **OC4FE** `[10:10]`
    ///
    /// Output compare 4 fast
    /// enable
    #[inline(always)]
    pub const fn oc4fe(&self) -> bool {
        (self.bits & Chctlr2Output::OC4FE_MASK) != 0
    }

    /// Read **CC4S** `[9:8]`
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub const fn cc4s(&self) -> u32 {
        (self.bits & Chctlr2Output::CC4S_MASK) >> Chctlr2Output::CC4S_OFFSET
    }

    /// Read **OC3CE** `[7:7]`
    ///
    /// Output compare 3 clear
    /// enable
    #[inline(always)]
    pub const fn oc3ce(&self) -> bool {
        (self.bits & Chctlr2Output::OC3CE_MASK) != 0
    }

    /// Read **OC3M** `[6:4]`
    ///
    /// Output compare 3 mode
    #[inline(always)]
    pub const fn oc3m(&self) -> u32 {
        (self.bits & Chctlr2Output::OC3M_MASK) >> Chctlr2Output::OC3M_OFFSET
    }

    /// Read **OC3PE** `[3:3]`
    ///
    /// Output compare 3 preload
    /// enable
    #[inline(always)]
    pub const fn oc3pe(&self) -> bool {
        (self.bits & Chctlr2Output::OC3PE_MASK) != 0
    }

    /// Read **OC3FE** `[2:2]`
    ///
    /// Output compare 3 fast
    /// enable
    #[inline(always)]
    pub const fn oc3fe(&self) -> bool {
        (self.bits & Chctlr2Output::OC3FE_MASK) != 0
    }

    /// Read **CC3S** `[1:0]`
    ///
    /// Capture/Compare 3
    /// selection
    #[inline(always)]
    pub const fn cc3s(&self) -> u32 {
        (self.bits & Chctlr2Output::CC3S_MASK) >> Chctlr2Output::CC3S_OFFSET
    }

    /// Set **OC4CE** `[15:15]`
    ///
    /// Output compare 4 clear
    /// enable
    #[inline(always)]
    pub const fn with_oc4ce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr2Output::OC4CE_MASK
            } else {
                self.bits & !Chctlr2Output::OC4CE_MASK
            },
        }
    }

    /// Set **OC4M** `[14:12]`
    ///
    /// Output compare 4 mode
    #[inline(always)]
    pub const fn with_oc4m(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Output::OC4M_MASK)
                | ((value << Chctlr2Output::OC4M_OFFSET) & Chctlr2Output::OC4M_MASK),
        }
    }

    /// Set **OC4PE** `[11:11]`
    ///
    /// Output compare 4 preload
    /// enable
    #[inline(always)]
    pub const fn with_oc4pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr2Output::OC4PE_MASK
            } else {
                self.bits & !Chctlr2Output::OC4PE_MASK
            },
        }
    }

    /// Set **OC4FE** `[10:10]`
    ///
    /// Output compare 4 fast
    /// enable
    #[inline(always)]
    pub const fn with_oc4fe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr2Output::OC4FE_MASK
            } else {
                self.bits & !Chctlr2Output::OC4FE_MASK
            },
        }
    }

    /// Set **CC4S** `[9:8]`
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub const fn with_cc4s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Output::CC4S_MASK)
                | ((value << Chctlr2Output::CC4S_OFFSET) & Chctlr2Output::CC4S_MASK),
        }
    }

    /// Set **OC3CE** `[7:7]`
    ///
    /// Output compare 3 clear
    /// enable
    #[inline(always)]
    pub const fn with_oc3ce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr2Output::OC3CE_MASK
            } else {
                self.bits & !Chctlr2Output::OC3CE_MASK
            },
        }
    }

    /// Set **OC3M** `[6:4]`
    ///
    /// Output compare 3 mode
    #[inline(always)]
    pub const fn with_oc3m(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Output::OC3M_MASK)
                | ((value << Chctlr2Output::OC3M_OFFSET) & Chctlr2Output::OC3M_MASK),
        }
    }

    /// Set **OC3PE** `[3:3]`
    ///
    /// Output compare 3 preload
    /// enable
    #[inline(always)]
    pub const fn with_oc3pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr2Output::OC3PE_MASK
            } else {
                self.bits & !Chctlr2Output::OC3PE_MASK
            },
        }
    }

    /// Set **OC3FE** `[2:2]`
    ///
    /// Output compare 3 fast
    /// enable
    #[inline(always)]
    pub const fn with_oc3fe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Chctlr2Output::OC3FE_MASK
            } else {
                self.bits & !Chctlr2Output::OC3FE_MASK
            },
        }
    }

    /// Set **CC3S** `[1:0]`
    ///
    /// Capture/Compare 3
    /// selection
    #[inline(always)]
    pub const fn with_cc3s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Output::CC3S_MASK)
                | ((value << Chctlr2Output::CC3S_OFFSET) & Chctlr2Output::CC3S_MASK),
        }
    }
}

impl Default for Chctlr2OutputVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Chctlr2Output {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chctlr2OutputVal {
        Chctlr2OutputVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chctlr2OutputVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chctlr2OutputVal) -> Chctlr2OutputVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CHCTLR2_Input Register
///
/// capture/compare mode register 2 (input
/// mode)
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IC4F** `[15:12]` (RW)
///
///   Input capture 4 filter
///
/// - **IC4PSC** `[11:10]` (RW)
///
///   Input capture 4 prescaler
///
/// - **CC4S** `[9:8]` (RW)
///
///   Capture/Compare 4
///   selection
///
/// - **IC3F** `[7:4]` (RW)
///
///   Input capture 3 filter
///
/// - **IC3PSC** `[3:2]` (RW)
///
///   Input capture 3 prescaler
///
/// - **CC3S** `[1:0]` (RW)
///
///   Capture/compare 3
///   selection
#[repr(transparent)]
pub struct Chctlr2Input {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Chctlr2Input {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chctlr2Input {
    // ======== Field Constants ========

    /// **IC4F**
    ///
    /// Input capture 4 filter
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const IC4F_OFFSET: u32 = 12;
    pub const IC4F_MASK: u32 = 0x0000f000;

    /// **IC4PSC**
    ///
    /// Input capture 4 prescaler
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const IC4PSC_OFFSET: u32 = 10;
    pub const IC4PSC_MASK: u32 = 0x00000c00;

    /// **CC4S**
    ///
    /// Capture/Compare 4
    /// selection
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const CC4S_OFFSET: u32 = 8;
    pub const CC4S_MASK: u32 = 0x00000300;

    /// **IC3F**
    ///
    /// Input capture 3 filter
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const IC3F_OFFSET: u32 = 4;
    pub const IC3F_MASK: u32 = 0x000000f0;

    /// **IC3PSC**
    ///
    /// Input capture 3 prescaler
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const IC3PSC_OFFSET: u32 = 2;
    pub const IC3PSC_MASK: u32 = 0x0000000c;

    /// **CC3S**
    ///
    /// Capture/compare 3
    /// selection
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CC3S_OFFSET: u32 = 0;
    pub const CC3S_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **IC4F** `[15:12]` (RW)
    ///
    /// Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> u32 {
        (self.value.read_bits() & Self::IC4F_MASK) >> Self::IC4F_OFFSET
    }

    /// Write **IC4F** `[15:12]` (RW)
    ///
    /// Input capture 4 filter
    #[inline(always)]
    pub fn write_ic4f(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC4F_MASK) | ((value << Self::IC4F_OFFSET) & Self::IC4F_MASK)
        });
    }

    /// **IC4PSC** `[11:10]` (RW)
    ///
    /// Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> u32 {
        (self.value.read_bits() & Self::IC4PSC_MASK) >> Self::IC4PSC_OFFSET
    }

    /// Write **IC4PSC** `[11:10]` (RW)
    ///
    /// Input capture 4 prescaler
    #[inline(always)]
    pub fn write_ic4psc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC4PSC_MASK) | ((value << Self::IC4PSC_OFFSET) & Self::IC4PSC_MASK)
        });
    }

    /// **CC4S** `[9:8]` (RW)
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub fn cc4s(&self) -> u32 {
        (self.value.read_bits() & Self::CC4S_MASK) >> Self::CC4S_OFFSET
    }

    /// Write **CC4S** `[9:8]` (RW)
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub fn write_cc4s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC4S_MASK) | ((value << Self::CC4S_OFFSET) & Self::CC4S_MASK)
        });
    }

    /// **IC3F** `[7:4]` (RW)
    ///
    /// Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> u32 {
        (self.value.read_bits() & Self::IC3F_MASK) >> Self::IC3F_OFFSET
    }

    /// Write **IC3F** `[7:4]` (RW)
    ///
    /// Input capture 3 filter
    #[inline(always)]
    pub fn write_ic3f(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC3F_MASK) | ((value << Self::IC3F_OFFSET) & Self::IC3F_MASK)
        });
    }

    /// **IC3PSC** `[3:2]` (RW)
    ///
    /// Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> u32 {
        (self.value.read_bits() & Self::IC3PSC_MASK) >> Self::IC3PSC_OFFSET
    }

    /// Write **IC3PSC** `[3:2]` (RW)
    ///
    /// Input capture 3 prescaler
    #[inline(always)]
    pub fn write_ic3psc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IC3PSC_MASK) | ((value << Self::IC3PSC_OFFSET) & Self::IC3PSC_MASK)
        });
    }

    /// **CC3S** `[1:0]` (RW)
    ///
    /// Capture/compare 3
    /// selection
    #[inline(always)]
    pub fn cc3s(&self) -> u32 {
        (self.value.read_bits() & Self::CC3S_MASK) >> Self::CC3S_OFFSET
    }

    /// Write **CC3S** `[1:0]` (RW)
    ///
    /// Capture/compare 3
    /// selection
    #[inline(always)]
    pub fn write_cc3s(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC3S_MASK) | ((value << Self::CC3S_OFFSET) & Self::CC3S_MASK)
        });
    }
}

/// CHCTLR2_Input value type for batch read/modify/write operations
///
/// Use `chctlr2input.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chctlr2input.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chctlr2InputVal {
    bits: u32,
}

impl Chctlr2InputVal {
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

    /// Read **IC4F** `[15:12]`
    ///
    /// Input capture 4 filter
    #[inline(always)]
    pub const fn ic4f(&self) -> u32 {
        (self.bits & Chctlr2Input::IC4F_MASK) >> Chctlr2Input::IC4F_OFFSET
    }

    /// Read **IC4PSC** `[11:10]`
    ///
    /// Input capture 4 prescaler
    #[inline(always)]
    pub const fn ic4psc(&self) -> u32 {
        (self.bits & Chctlr2Input::IC4PSC_MASK) >> Chctlr2Input::IC4PSC_OFFSET
    }

    /// Read **CC4S** `[9:8]`
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub const fn cc4s(&self) -> u32 {
        (self.bits & Chctlr2Input::CC4S_MASK) >> Chctlr2Input::CC4S_OFFSET
    }

    /// Read **IC3F** `[7:4]`
    ///
    /// Input capture 3 filter
    #[inline(always)]
    pub const fn ic3f(&self) -> u32 {
        (self.bits & Chctlr2Input::IC3F_MASK) >> Chctlr2Input::IC3F_OFFSET
    }

    /// Read **IC3PSC** `[3:2]`
    ///
    /// Input capture 3 prescaler
    #[inline(always)]
    pub const fn ic3psc(&self) -> u32 {
        (self.bits & Chctlr2Input::IC3PSC_MASK) >> Chctlr2Input::IC3PSC_OFFSET
    }

    /// Read **CC3S** `[1:0]`
    ///
    /// Capture/compare 3
    /// selection
    #[inline(always)]
    pub const fn cc3s(&self) -> u32 {
        (self.bits & Chctlr2Input::CC3S_MASK) >> Chctlr2Input::CC3S_OFFSET
    }

    /// Set **IC4F** `[15:12]`
    ///
    /// Input capture 4 filter
    #[inline(always)]
    pub const fn with_ic4f(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Input::IC4F_MASK)
                | ((value << Chctlr2Input::IC4F_OFFSET) & Chctlr2Input::IC4F_MASK),
        }
    }

    /// Set **IC4PSC** `[11:10]`
    ///
    /// Input capture 4 prescaler
    #[inline(always)]
    pub const fn with_ic4psc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Input::IC4PSC_MASK)
                | ((value << Chctlr2Input::IC4PSC_OFFSET) & Chctlr2Input::IC4PSC_MASK),
        }
    }

    /// Set **CC4S** `[9:8]`
    ///
    /// Capture/Compare 4
    /// selection
    #[inline(always)]
    pub const fn with_cc4s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Input::CC4S_MASK)
                | ((value << Chctlr2Input::CC4S_OFFSET) & Chctlr2Input::CC4S_MASK),
        }
    }

    /// Set **IC3F** `[7:4]`
    ///
    /// Input capture 3 filter
    #[inline(always)]
    pub const fn with_ic3f(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Input::IC3F_MASK)
                | ((value << Chctlr2Input::IC3F_OFFSET) & Chctlr2Input::IC3F_MASK),
        }
    }

    /// Set **IC3PSC** `[3:2]`
    ///
    /// Input capture 3 prescaler
    #[inline(always)]
    pub const fn with_ic3psc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Input::IC3PSC_MASK)
                | ((value << Chctlr2Input::IC3PSC_OFFSET) & Chctlr2Input::IC3PSC_MASK),
        }
    }

    /// Set **CC3S** `[1:0]`
    ///
    /// Capture/compare 3
    /// selection
    #[inline(always)]
    pub const fn with_cc3s(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Chctlr2Input::CC3S_MASK)
                | ((value << Chctlr2Input::CC3S_OFFSET) & Chctlr2Input::CC3S_MASK),
        }
    }
}

impl Default for Chctlr2InputVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Chctlr2Input {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chctlr2InputVal {
        Chctlr2InputVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chctlr2InputVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chctlr2InputVal) -> Chctlr2InputVal>(&self, f: F) {
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
/// - **CC4P** `[13]` (RW)
///
///   Capture/Compare 3 output
///   Polarity
///
/// - **CC4E** `[12]` (RW)
///
///   Capture/Compare 4 output
///   enable
///
/// - **CC3NP** `[11]` (RW)
///
///   Capture/Compare 3 output
///   Polarity
///
/// - **CC3NE** `[10]` (RW)
///
///   Capture/Compare 3 complementary output
///   enable
///
/// - **CC3P** `[9]` (RW)
///
///   Capture/Compare 3 output
///   Polarity
///
/// - **CC3E** `[8]` (RW)
///
///   Capture/Compare 3 output
///   enable
///
/// - **CC2NP** `[7]` (RW)
///
///   Capture/Compare 2 output
///   Polarity
///
/// - **CC2NE** `[6]` (RW)
///
///   Capture/Compare 2 complementary output
///   enable
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
/// - **CC1NP** `[3]` (RW)
///
///   Capture/Compare 1 output
///   Polarity
///
/// - **CC1NE** `[2]` (RW)
///
///   Capture/Compare 1 complementary output
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ccer {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ccer {
    // ======== Field Constants ========

    /// **CC4P**
    ///
    /// Capture/Compare 3 output
    /// Polarity
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CC4P_OFFSET: u32 = 13;
    pub const CC4P_MASK: u32 = 0x00002000;
    pub const CC4P: u32 = 1 << 13;

    /// **CC4E**
    ///
    /// Capture/Compare 4 output
    /// enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CC4E_OFFSET: u32 = 12;
    pub const CC4E_MASK: u32 = 0x00001000;
    pub const CC4E: u32 = 1 << 12;

    /// **CC3NP**
    ///
    /// Capture/Compare 3 output
    /// Polarity
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CC3NP_OFFSET: u32 = 11;
    pub const CC3NP_MASK: u32 = 0x00000800;
    pub const CC3NP: u32 = 1 << 11;

    /// **CC3NE**
    ///
    /// Capture/Compare 3 complementary output
    /// enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CC3NE_OFFSET: u32 = 10;
    pub const CC3NE_MASK: u32 = 0x00000400;
    pub const CC3NE: u32 = 1 << 10;

    /// **CC3P**
    ///
    /// Capture/Compare 3 output
    /// Polarity
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CC3P_OFFSET: u32 = 9;
    pub const CC3P_MASK: u32 = 0x00000200;
    pub const CC3P: u32 = 1 << 9;

    /// **CC3E**
    ///
    /// Capture/Compare 3 output
    /// enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CC3E_OFFSET: u32 = 8;
    pub const CC3E_MASK: u32 = 0x00000100;
    pub const CC3E: u32 = 1 << 8;

    /// **CC2NP**
    ///
    /// Capture/Compare 2 output
    /// Polarity
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CC2NP_OFFSET: u32 = 7;
    pub const CC2NP_MASK: u32 = 0x00000080;
    pub const CC2NP: u32 = 1 << 7;

    /// **CC2NE**
    ///
    /// Capture/Compare 2 complementary output
    /// enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CC2NE_OFFSET: u32 = 6;
    pub const CC2NE_MASK: u32 = 0x00000040;
    pub const CC2NE: u32 = 1 << 6;

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

    /// **CC1NP**
    ///
    /// Capture/Compare 1 output
    /// Polarity
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CC1NP_OFFSET: u32 = 3;
    pub const CC1NP_MASK: u32 = 0x00000008;
    pub const CC1NP: u32 = 1 << 3;

    /// **CC1NE**
    ///
    /// Capture/Compare 1 complementary output
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CC1NE_OFFSET: u32 = 2;
    pub const CC1NE_MASK: u32 = 0x00000004;
    pub const CC1NE: u32 = 1 << 2;

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

    /// **CC4P** `[13]` (RW)
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub fn cc4p(&self) -> bool {
        (self.value.read_bits() & Self::CC4P_MASK) != 0
    }

    /// Set CC4P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc4p(&self) {
        self.value.modify_bits(|r| r | Self::CC4P_MASK);
    }

    /// Clear CC4P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc4p(&self) {
        self.value.modify_bits(|r| r & !Self::CC4P_MASK);
    }

    /// **CC4E** `[12]` (RW)
    ///
    /// Capture/Compare 4 output
    /// enable
    #[inline(always)]
    pub fn cc4e(&self) -> bool {
        (self.value.read_bits() & Self::CC4E_MASK) != 0
    }

    /// Set CC4E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc4e(&self) {
        self.value.modify_bits(|r| r | Self::CC4E_MASK);
    }

    /// Clear CC4E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc4e(&self) {
        self.value.modify_bits(|r| r & !Self::CC4E_MASK);
    }

    /// **CC3NP** `[11]` (RW)
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub fn cc3np(&self) -> bool {
        (self.value.read_bits() & Self::CC3NP_MASK) != 0
    }

    /// Set CC3NP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3np(&self) {
        self.value.modify_bits(|r| r | Self::CC3NP_MASK);
    }

    /// Clear CC3NP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3np(&self) {
        self.value.modify_bits(|r| r & !Self::CC3NP_MASK);
    }

    /// **CC3NE** `[10]` (RW)
    ///
    /// Capture/Compare 3 complementary output
    /// enable
    #[inline(always)]
    pub fn cc3ne(&self) -> bool {
        (self.value.read_bits() & Self::CC3NE_MASK) != 0
    }

    /// Set CC3NE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3ne(&self) {
        self.value.modify_bits(|r| r | Self::CC3NE_MASK);
    }

    /// Clear CC3NE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3ne(&self) {
        self.value.modify_bits(|r| r & !Self::CC3NE_MASK);
    }

    /// **CC3P** `[9]` (RW)
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub fn cc3p(&self) -> bool {
        (self.value.read_bits() & Self::CC3P_MASK) != 0
    }

    /// Set CC3P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3p(&self) {
        self.value.modify_bits(|r| r | Self::CC3P_MASK);
    }

    /// Clear CC3P bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3p(&self) {
        self.value.modify_bits(|r| r & !Self::CC3P_MASK);
    }

    /// **CC3E** `[8]` (RW)
    ///
    /// Capture/Compare 3 output
    /// enable
    #[inline(always)]
    pub fn cc3e(&self) -> bool {
        (self.value.read_bits() & Self::CC3E_MASK) != 0
    }

    /// Set CC3E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc3e(&self) {
        self.value.modify_bits(|r| r | Self::CC3E_MASK);
    }

    /// Clear CC3E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc3e(&self) {
        self.value.modify_bits(|r| r & !Self::CC3E_MASK);
    }

    /// **CC2NP** `[7]` (RW)
    ///
    /// Capture/Compare 2 output
    /// Polarity
    #[inline(always)]
    pub fn cc2np(&self) -> bool {
        (self.value.read_bits() & Self::CC2NP_MASK) != 0
    }

    /// Set CC2NP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2np(&self) {
        self.value.modify_bits(|r| r | Self::CC2NP_MASK);
    }

    /// Clear CC2NP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2np(&self) {
        self.value.modify_bits(|r| r & !Self::CC2NP_MASK);
    }

    /// **CC2NE** `[6]` (RW)
    ///
    /// Capture/Compare 2 complementary output
    /// enable
    #[inline(always)]
    pub fn cc2ne(&self) -> bool {
        (self.value.read_bits() & Self::CC2NE_MASK) != 0
    }

    /// Set CC2NE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2ne(&self) {
        self.value.modify_bits(|r| r | Self::CC2NE_MASK);
    }

    /// Clear CC2NE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2ne(&self) {
        self.value.modify_bits(|r| r & !Self::CC2NE_MASK);
    }

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

    /// **CC1NP** `[3]` (RW)
    ///
    /// Capture/Compare 1 output
    /// Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> bool {
        (self.value.read_bits() & Self::CC1NP_MASK) != 0
    }

    /// Set CC1NP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1np(&self) {
        self.value.modify_bits(|r| r | Self::CC1NP_MASK);
    }

    /// Clear CC1NP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1np(&self) {
        self.value.modify_bits(|r| r & !Self::CC1NP_MASK);
    }

    /// **CC1NE** `[2]` (RW)
    ///
    /// Capture/Compare 1 complementary output
    /// enable
    #[inline(always)]
    pub fn cc1ne(&self) -> bool {
        (self.value.read_bits() & Self::CC1NE_MASK) != 0
    }

    /// Set CC1NE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1ne(&self) {
        self.value.modify_bits(|r| r | Self::CC1NE_MASK);
    }

    /// Clear CC1NE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1ne(&self) {
        self.value.modify_bits(|r| r & !Self::CC1NE_MASK);
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

    /// Read **CC4P** `[13:13]`
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub const fn cc4p(&self) -> bool {
        (self.bits & Ccer::CC4P_MASK) != 0
    }

    /// Read **CC4E** `[12:12]`
    ///
    /// Capture/Compare 4 output
    /// enable
    #[inline(always)]
    pub const fn cc4e(&self) -> bool {
        (self.bits & Ccer::CC4E_MASK) != 0
    }

    /// Read **CC3NP** `[11:11]`
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub const fn cc3np(&self) -> bool {
        (self.bits & Ccer::CC3NP_MASK) != 0
    }

    /// Read **CC3NE** `[10:10]`
    ///
    /// Capture/Compare 3 complementary output
    /// enable
    #[inline(always)]
    pub const fn cc3ne(&self) -> bool {
        (self.bits & Ccer::CC3NE_MASK) != 0
    }

    /// Read **CC3P** `[9:9]`
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub const fn cc3p(&self) -> bool {
        (self.bits & Ccer::CC3P_MASK) != 0
    }

    /// Read **CC3E** `[8:8]`
    ///
    /// Capture/Compare 3 output
    /// enable
    #[inline(always)]
    pub const fn cc3e(&self) -> bool {
        (self.bits & Ccer::CC3E_MASK) != 0
    }

    /// Read **CC2NP** `[7:7]`
    ///
    /// Capture/Compare 2 output
    /// Polarity
    #[inline(always)]
    pub const fn cc2np(&self) -> bool {
        (self.bits & Ccer::CC2NP_MASK) != 0
    }

    /// Read **CC2NE** `[6:6]`
    ///
    /// Capture/Compare 2 complementary output
    /// enable
    #[inline(always)]
    pub const fn cc2ne(&self) -> bool {
        (self.bits & Ccer::CC2NE_MASK) != 0
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

    /// Read **CC1NP** `[3:3]`
    ///
    /// Capture/Compare 1 output
    /// Polarity
    #[inline(always)]
    pub const fn cc1np(&self) -> bool {
        (self.bits & Ccer::CC1NP_MASK) != 0
    }

    /// Read **CC1NE** `[2:2]`
    ///
    /// Capture/Compare 1 complementary output
    /// enable
    #[inline(always)]
    pub const fn cc1ne(&self) -> bool {
        (self.bits & Ccer::CC1NE_MASK) != 0
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

    /// Set **CC4P** `[13:13]`
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc4p(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC4P_MASK
            } else {
                self.bits & !Ccer::CC4P_MASK
            },
        }
    }

    /// Set **CC4E** `[12:12]`
    ///
    /// Capture/Compare 4 output
    /// enable
    #[inline(always)]
    pub const fn with_cc4e(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC4E_MASK
            } else {
                self.bits & !Ccer::CC4E_MASK
            },
        }
    }

    /// Set **CC3NP** `[11:11]`
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc3np(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC3NP_MASK
            } else {
                self.bits & !Ccer::CC3NP_MASK
            },
        }
    }

    /// Set **CC3NE** `[10:10]`
    ///
    /// Capture/Compare 3 complementary output
    /// enable
    #[inline(always)]
    pub const fn with_cc3ne(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC3NE_MASK
            } else {
                self.bits & !Ccer::CC3NE_MASK
            },
        }
    }

    /// Set **CC3P** `[9:9]`
    ///
    /// Capture/Compare 3 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc3p(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC3P_MASK
            } else {
                self.bits & !Ccer::CC3P_MASK
            },
        }
    }

    /// Set **CC3E** `[8:8]`
    ///
    /// Capture/Compare 3 output
    /// enable
    #[inline(always)]
    pub const fn with_cc3e(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC3E_MASK
            } else {
                self.bits & !Ccer::CC3E_MASK
            },
        }
    }

    /// Set **CC2NP** `[7:7]`
    ///
    /// Capture/Compare 2 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc2np(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC2NP_MASK
            } else {
                self.bits & !Ccer::CC2NP_MASK
            },
        }
    }

    /// Set **CC2NE** `[6:6]`
    ///
    /// Capture/Compare 2 complementary output
    /// enable
    #[inline(always)]
    pub const fn with_cc2ne(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC2NE_MASK
            } else {
                self.bits & !Ccer::CC2NE_MASK
            },
        }
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

    /// Set **CC1NP** `[3:3]`
    ///
    /// Capture/Compare 1 output
    /// Polarity
    #[inline(always)]
    pub const fn with_cc1np(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC1NP_MASK
            } else {
                self.bits & !Ccer::CC1NP_MASK
            },
        }
    }

    /// Set **CC1NE** `[2:2]`
    ///
    /// Capture/Compare 1 complementary output
    /// enable
    #[inline(always)]
    pub const fn with_cc1ne(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ccer::CC1NE_MASK
            } else {
                self.bits & !Ccer::CC1NE_MASK
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cnt {
    type Target = crate::common::VolatileCell<u32>;
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Psc {
    type Target = crate::common::VolatileCell<u32>;
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrlr {
    type Target = crate::common::VolatileCell<u32>;
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

/// RPTCR Register
///
/// repetition counter register
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RPTCR** `[7:0]` (RW)
///
///   Repetition counter value
#[repr(transparent)]
pub struct Rptcr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rptcr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rptcr {
    // ======== Field Constants ========

    /// **RPTCR**
    ///
    /// Repetition counter value
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RPTCR_OFFSET: u32 = 0;
    pub const RPTCR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RPTCR** `[7:0]` (RW)
    ///
    /// Repetition counter value
    #[inline(always)]
    pub fn rptcr(&self) -> u32 {
        (self.value.read_bits() & Self::RPTCR_MASK) >> Self::RPTCR_OFFSET
    }

    /// Write **RPTCR** `[7:0]` (RW)
    ///
    /// Repetition counter value
    #[inline(always)]
    pub fn write_rptcr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RPTCR_MASK) | ((value << Self::RPTCR_OFFSET) & Self::RPTCR_MASK)
        });
    }
}

/// RPTCR value type for batch read/modify/write operations
///
/// Use `rptcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rptcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RptcrVal {
    bits: u32,
}

impl RptcrVal {
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

    /// Read **RPTCR** `[7:0]`
    ///
    /// Repetition counter value
    #[inline(always)]
    pub const fn rptcr(&self) -> u32 {
        (self.bits & Rptcr::RPTCR_MASK) >> Rptcr::RPTCR_OFFSET
    }

    /// Set **RPTCR** `[7:0]`
    ///
    /// Repetition counter value
    #[inline(always)]
    pub const fn with_rptcr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rptcr::RPTCR_MASK)
                | ((value << Rptcr::RPTCR_OFFSET) & Rptcr::RPTCR_MASK),
        }
    }
}

impl Default for RptcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rptcr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RptcrVal {
        RptcrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RptcrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RptcrVal) -> RptcrVal>(&self, f: F) {
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1cvr {
    type Target = crate::common::VolatileCell<u32>;
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
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2cvr {
    type Target = crate::common::VolatileCell<u32>;
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

/// CH3CVR Register
///
/// capture/compare register 3
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH3CVR** `[15:0]` (RW)
///
///   Capture/Compare value
#[repr(transparent)]
pub struct Ch3cvr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3cvr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3cvr {
    // ======== Field Constants ========

    /// **CH3CVR**
    ///
    /// Capture/Compare value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH3CVR_OFFSET: u32 = 0;
    pub const CH3CVR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH3CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub fn ch3cvr(&self) -> u32 {
        (self.value.read_bits() & Self::CH3CVR_MASK) >> Self::CH3CVR_OFFSET
    }

    /// Write **CH3CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub fn write_ch3cvr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH3CVR_MASK) | ((value << Self::CH3CVR_OFFSET) & Self::CH3CVR_MASK)
        });
    }
}

/// CH3CVR value type for batch read/modify/write operations
///
/// Use `ch3cvr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3cvr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3cvrVal {
    bits: u32,
}

impl Ch3cvrVal {
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

    /// Read **CH3CVR** `[15:0]`
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub const fn ch3cvr(&self) -> u32 {
        (self.bits & Ch3cvr::CH3CVR_MASK) >> Ch3cvr::CH3CVR_OFFSET
    }

    /// Set **CH3CVR** `[15:0]`
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub const fn with_ch3cvr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3cvr::CH3CVR_MASK)
                | ((value << Ch3cvr::CH3CVR_OFFSET) & Ch3cvr::CH3CVR_MASK),
        }
    }
}

impl Default for Ch3cvrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3cvr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3cvrVal {
        Ch3cvrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3cvrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3cvrVal) -> Ch3cvrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CH4CVR Register
///
/// capture/compare register 4
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH4CVR** `[15:0]` (RW)
///
///   Capture/Compare value
#[repr(transparent)]
pub struct Ch4cvr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4cvr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4cvr {
    // ======== Field Constants ========

    /// **CH4CVR**
    ///
    /// Capture/Compare value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH4CVR_OFFSET: u32 = 0;
    pub const CH4CVR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH4CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub fn ch4cvr(&self) -> u32 {
        (self.value.read_bits() & Self::CH4CVR_MASK) >> Self::CH4CVR_OFFSET
    }

    /// Write **CH4CVR** `[15:0]` (RW)
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub fn write_ch4cvr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CH4CVR_MASK) | ((value << Self::CH4CVR_OFFSET) & Self::CH4CVR_MASK)
        });
    }
}

/// CH4CVR value type for batch read/modify/write operations
///
/// Use `ch4cvr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4cvr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4cvrVal {
    bits: u32,
}

impl Ch4cvrVal {
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

    /// Read **CH4CVR** `[15:0]`
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub const fn ch4cvr(&self) -> u32 {
        (self.bits & Ch4cvr::CH4CVR_MASK) >> Ch4cvr::CH4CVR_OFFSET
    }

    /// Set **CH4CVR** `[15:0]`
    ///
    /// Capture/Compare value
    #[inline(always)]
    pub const fn with_ch4cvr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4cvr::CH4CVR_MASK)
                | ((value << Ch4cvr::CH4CVR_OFFSET) & Ch4cvr::CH4CVR_MASK),
        }
    }
}

impl Default for Ch4cvrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch4cvr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4cvrVal {
        Ch4cvrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4cvrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4cvrVal) -> Ch4cvrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BDTR Register
///
/// break and dead-time register
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MOE** `[15]` (RW)
///
///   Main output enable
///
/// - **AOE** `[14]` (RW)
///
///   Automatic output enable
///
/// - **BKP** `[13]` (RW)
///
///   Break polarity
///
/// - **BKE** `[12]` (RW)
///
///   Break enable
///
/// - **OSSR** `[11]` (RW)
///
///   Off-state selection for Run
///   mode
///
/// - **OSSI** `[10]` (RW)
///
///   Off-state selection for Idle
///   mode
///
/// - **LOCK** `[9:8]` (RW)
///
///   Lock configuration
///
/// - **DTG** `[7:0]` (RW)
///
///   Dead-time generator setup
#[repr(transparent)]
pub struct Bdtr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bdtr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bdtr {
    // ======== Field Constants ========

    /// **MOE**
    ///
    /// Main output enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MOE_OFFSET: u32 = 15;
    pub const MOE_MASK: u32 = 0x00008000;
    pub const MOE: u32 = 1 << 15;

    /// **AOE**
    ///
    /// Automatic output enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const AOE_OFFSET: u32 = 14;
    pub const AOE_MASK: u32 = 0x00004000;
    pub const AOE: u32 = 1 << 14;

    /// **BKP**
    ///
    /// Break polarity
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const BKP_OFFSET: u32 = 13;
    pub const BKP_MASK: u32 = 0x00002000;
    pub const BKP: u32 = 1 << 13;

    /// **BKE**
    ///
    /// Break enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BKE_OFFSET: u32 = 12;
    pub const BKE_MASK: u32 = 0x00001000;
    pub const BKE: u32 = 1 << 12;

    /// **OSSR**
    ///
    /// Off-state selection for Run
    /// mode
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OSSR_OFFSET: u32 = 11;
    pub const OSSR_MASK: u32 = 0x00000800;
    pub const OSSR: u32 = 1 << 11;

    /// **OSSI**
    ///
    /// Off-state selection for Idle
    /// mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const OSSI_OFFSET: u32 = 10;
    pub const OSSI_MASK: u32 = 0x00000400;
    pub const OSSI: u32 = 1 << 10;

    /// **LOCK**
    ///
    /// Lock configuration
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const LOCK_OFFSET: u32 = 8;
    pub const LOCK_MASK: u32 = 0x00000300;

    /// **DTG**
    ///
    /// Dead-time generator setup
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DTG_OFFSET: u32 = 0;
    pub const DTG_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **MOE** `[15]` (RW)
    ///
    /// Main output enable
    #[inline(always)]
    pub fn moe(&self) -> bool {
        (self.value.read_bits() & Self::MOE_MASK) != 0
    }

    /// Set MOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_moe(&self) {
        self.value.modify_bits(|r| r | Self::MOE_MASK);
    }

    /// Clear MOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_moe(&self) {
        self.value.modify_bits(|r| r & !Self::MOE_MASK);
    }

    /// **AOE** `[14]` (RW)
    ///
    /// Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> bool {
        (self.value.read_bits() & Self::AOE_MASK) != 0
    }

    /// Set AOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_aoe(&self) {
        self.value.modify_bits(|r| r | Self::AOE_MASK);
    }

    /// Clear AOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_aoe(&self) {
        self.value.modify_bits(|r| r & !Self::AOE_MASK);
    }

    /// **BKP** `[13]` (RW)
    ///
    /// Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> bool {
        (self.value.read_bits() & Self::BKP_MASK) != 0
    }

    /// Set BKP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bkp(&self) {
        self.value.modify_bits(|r| r | Self::BKP_MASK);
    }

    /// Clear BKP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bkp(&self) {
        self.value.modify_bits(|r| r & !Self::BKP_MASK);
    }

    /// **BKE** `[12]` (RW)
    ///
    /// Break enable
    #[inline(always)]
    pub fn bke(&self) -> bool {
        (self.value.read_bits() & Self::BKE_MASK) != 0
    }

    /// Set BKE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bke(&self) {
        self.value.modify_bits(|r| r | Self::BKE_MASK);
    }

    /// Clear BKE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bke(&self) {
        self.value.modify_bits(|r| r & !Self::BKE_MASK);
    }

    /// **OSSR** `[11]` (RW)
    ///
    /// Off-state selection for Run
    /// mode
    #[inline(always)]
    pub fn ossr(&self) -> bool {
        (self.value.read_bits() & Self::OSSR_MASK) != 0
    }

    /// Set OSSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ossr(&self) {
        self.value.modify_bits(|r| r | Self::OSSR_MASK);
    }

    /// Clear OSSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ossr(&self) {
        self.value.modify_bits(|r| r & !Self::OSSR_MASK);
    }

    /// **OSSI** `[10]` (RW)
    ///
    /// Off-state selection for Idle
    /// mode
    #[inline(always)]
    pub fn ossi(&self) -> bool {
        (self.value.read_bits() & Self::OSSI_MASK) != 0
    }

    /// Set OSSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ossi(&self) {
        self.value.modify_bits(|r| r | Self::OSSI_MASK);
    }

    /// Clear OSSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ossi(&self) {
        self.value.modify_bits(|r| r & !Self::OSSI_MASK);
    }

    /// **LOCK** `[9:8]` (RW)
    ///
    /// Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> u32 {
        (self.value.read_bits() & Self::LOCK_MASK) >> Self::LOCK_OFFSET
    }

    /// Write **LOCK** `[9:8]` (RW)
    ///
    /// Lock configuration
    #[inline(always)]
    pub fn write_lock(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LOCK_MASK) | ((value << Self::LOCK_OFFSET) & Self::LOCK_MASK)
        });
    }

    /// **DTG** `[7:0]` (RW)
    ///
    /// Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> u32 {
        (self.value.read_bits() & Self::DTG_MASK) >> Self::DTG_OFFSET
    }

    /// Write **DTG** `[7:0]` (RW)
    ///
    /// Dead-time generator setup
    #[inline(always)]
    pub fn write_dtg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DTG_MASK) | ((value << Self::DTG_OFFSET) & Self::DTG_MASK)
        });
    }
}

/// BDTR value type for batch read/modify/write operations
///
/// Use `bdtr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bdtr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BdtrVal {
    bits: u32,
}

impl BdtrVal {
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

    /// Read **MOE** `[15:15]`
    ///
    /// Main output enable
    #[inline(always)]
    pub const fn moe(&self) -> bool {
        (self.bits & Bdtr::MOE_MASK) != 0
    }

    /// Read **AOE** `[14:14]`
    ///
    /// Automatic output enable
    #[inline(always)]
    pub const fn aoe(&self) -> bool {
        (self.bits & Bdtr::AOE_MASK) != 0
    }

    /// Read **BKP** `[13:13]`
    ///
    /// Break polarity
    #[inline(always)]
    pub const fn bkp(&self) -> bool {
        (self.bits & Bdtr::BKP_MASK) != 0
    }

    /// Read **BKE** `[12:12]`
    ///
    /// Break enable
    #[inline(always)]
    pub const fn bke(&self) -> bool {
        (self.bits & Bdtr::BKE_MASK) != 0
    }

    /// Read **OSSR** `[11:11]`
    ///
    /// Off-state selection for Run
    /// mode
    #[inline(always)]
    pub const fn ossr(&self) -> bool {
        (self.bits & Bdtr::OSSR_MASK) != 0
    }

    /// Read **OSSI** `[10:10]`
    ///
    /// Off-state selection for Idle
    /// mode
    #[inline(always)]
    pub const fn ossi(&self) -> bool {
        (self.bits & Bdtr::OSSI_MASK) != 0
    }

    /// Read **LOCK** `[9:8]`
    ///
    /// Lock configuration
    #[inline(always)]
    pub const fn lock(&self) -> u32 {
        (self.bits & Bdtr::LOCK_MASK) >> Bdtr::LOCK_OFFSET
    }

    /// Read **DTG** `[7:0]`
    ///
    /// Dead-time generator setup
    #[inline(always)]
    pub const fn dtg(&self) -> u32 {
        (self.bits & Bdtr::DTG_MASK) >> Bdtr::DTG_OFFSET
    }

    /// Set **MOE** `[15:15]`
    ///
    /// Main output enable
    #[inline(always)]
    pub const fn with_moe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bdtr::MOE_MASK
            } else {
                self.bits & !Bdtr::MOE_MASK
            },
        }
    }

    /// Set **AOE** `[14:14]`
    ///
    /// Automatic output enable
    #[inline(always)]
    pub const fn with_aoe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bdtr::AOE_MASK
            } else {
                self.bits & !Bdtr::AOE_MASK
            },
        }
    }

    /// Set **BKP** `[13:13]`
    ///
    /// Break polarity
    #[inline(always)]
    pub const fn with_bkp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bdtr::BKP_MASK
            } else {
                self.bits & !Bdtr::BKP_MASK
            },
        }
    }

    /// Set **BKE** `[12:12]`
    ///
    /// Break enable
    #[inline(always)]
    pub const fn with_bke(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bdtr::BKE_MASK
            } else {
                self.bits & !Bdtr::BKE_MASK
            },
        }
    }

    /// Set **OSSR** `[11:11]`
    ///
    /// Off-state selection for Run
    /// mode
    #[inline(always)]
    pub const fn with_ossr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bdtr::OSSR_MASK
            } else {
                self.bits & !Bdtr::OSSR_MASK
            },
        }
    }

    /// Set **OSSI** `[10:10]`
    ///
    /// Off-state selection for Idle
    /// mode
    #[inline(always)]
    pub const fn with_ossi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bdtr::OSSI_MASK
            } else {
                self.bits & !Bdtr::OSSI_MASK
            },
        }
    }

    /// Set **LOCK** `[9:8]`
    ///
    /// Lock configuration
    #[inline(always)]
    pub const fn with_lock(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bdtr::LOCK_MASK) | ((value << Bdtr::LOCK_OFFSET) & Bdtr::LOCK_MASK),
        }
    }

    /// Set **DTG** `[7:0]`
    ///
    /// Dead-time generator setup
    #[inline(always)]
    pub const fn with_dtg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bdtr::DTG_MASK) | ((value << Bdtr::DTG_OFFSET) & Bdtr::DTG_MASK),
        }
    }
}

impl Default for BdtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bdtr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BdtrVal {
        BdtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BdtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BdtrVal) -> BdtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMACFGR Register
///
/// DMA control register
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DBL** `[12:8]` (RW)
///
///   DMA burst length
///
/// - **DBA** `[4:0]` (RW)
///
///   DMA base address
#[repr(transparent)]
pub struct Dmacfgr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmacfgr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmacfgr {
    // ======== Field Constants ========

    /// **DBL**
    ///
    /// DMA burst length
    ///
    /// Bits `[12:8]`, mask `0x00001f00`
    pub const DBL_OFFSET: u32 = 8;
    pub const DBL_MASK: u32 = 0x00001f00;

    /// **DBA**
    ///
    /// DMA base address
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const DBA_OFFSET: u32 = 0;
    pub const DBA_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **DBL** `[12:8]` (RW)
    ///
    /// DMA burst length
    #[inline(always)]
    pub fn dbl(&self) -> u32 {
        (self.value.read_bits() & Self::DBL_MASK) >> Self::DBL_OFFSET
    }

    /// Write **DBL** `[12:8]` (RW)
    ///
    /// DMA burst length
    #[inline(always)]
    pub fn write_dbl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBL_MASK) | ((value << Self::DBL_OFFSET) & Self::DBL_MASK)
        });
    }

    /// **DBA** `[4:0]` (RW)
    ///
    /// DMA base address
    #[inline(always)]
    pub fn dba(&self) -> u32 {
        (self.value.read_bits() & Self::DBA_MASK) >> Self::DBA_OFFSET
    }

    /// Write **DBA** `[4:0]` (RW)
    ///
    /// DMA base address
    #[inline(always)]
    pub fn write_dba(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBA_MASK) | ((value << Self::DBA_OFFSET) & Self::DBA_MASK)
        });
    }
}

/// DMACFGR value type for batch read/modify/write operations
///
/// Use `dmacfgr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmacfgr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmacfgrVal {
    bits: u32,
}

impl DmacfgrVal {
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

    /// Read **DBL** `[12:8]`
    ///
    /// DMA burst length
    #[inline(always)]
    pub const fn dbl(&self) -> u32 {
        (self.bits & Dmacfgr::DBL_MASK) >> Dmacfgr::DBL_OFFSET
    }

    /// Read **DBA** `[4:0]`
    ///
    /// DMA base address
    #[inline(always)]
    pub const fn dba(&self) -> u32 {
        (self.bits & Dmacfgr::DBA_MASK) >> Dmacfgr::DBA_OFFSET
    }

    /// Set **DBL** `[12:8]`
    ///
    /// DMA burst length
    #[inline(always)]
    pub const fn with_dbl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmacfgr::DBL_MASK)
                | ((value << Dmacfgr::DBL_OFFSET) & Dmacfgr::DBL_MASK),
        }
    }

    /// Set **DBA** `[4:0]`
    ///
    /// DMA base address
    #[inline(always)]
    pub const fn with_dba(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmacfgr::DBA_MASK)
                | ((value << Dmacfgr::DBA_OFFSET) & Dmacfgr::DBA_MASK),
        }
    }
}

impl Default for DmacfgrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmacfgr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmacfgrVal {
        DmacfgrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmacfgrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmacfgrVal) -> DmacfgrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMAADR Register
///
/// DMA address for full transfer
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DMAADR** `[15:0]` (RW)
///
///   DMA register for burst
///   accesses
#[repr(transparent)]
pub struct Dmaadr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmaadr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmaadr {
    // ======== Field Constants ========

    /// **DMAADR**
    ///
    /// DMA register for burst
    /// accesses
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DMAADR_OFFSET: u32 = 0;
    pub const DMAADR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DMAADR** `[15:0]` (RW)
    ///
    /// DMA register for burst
    /// accesses
    #[inline(always)]
    pub fn dmaadr(&self) -> u32 {
        (self.value.read_bits() & Self::DMAADR_MASK) >> Self::DMAADR_OFFSET
    }

    /// Write **DMAADR** `[15:0]` (RW)
    ///
    /// DMA register for burst
    /// accesses
    #[inline(always)]
    pub fn write_dmaadr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMAADR_MASK) | ((value << Self::DMAADR_OFFSET) & Self::DMAADR_MASK)
        });
    }
}

/// DMAADR value type for batch read/modify/write operations
///
/// Use `dmaadr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaadr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaadrVal {
    bits: u32,
}

impl DmaadrVal {
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

    /// Read **DMAADR** `[15:0]`
    ///
    /// DMA register for burst
    /// accesses
    #[inline(always)]
    pub const fn dmaadr(&self) -> u32 {
        (self.bits & Dmaadr::DMAADR_MASK) >> Dmaadr::DMAADR_OFFSET
    }

    /// Set **DMAADR** `[15:0]`
    ///
    /// DMA register for burst
    /// accesses
    #[inline(always)]
    pub const fn with_dmaadr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmaadr::DMAADR_MASK)
                | ((value << Dmaadr::DMAADR_OFFSET) & Dmaadr::DMAADR_MASK),
        }
    }
}

impl Default for DmaadrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmaadr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaadrVal {
        DmaadrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaadrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaadrVal) -> DmaadrVal>(&self, f: F) {
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
/// - **PWM_OC4** `[7]` (RW)
///
///   PWM_OC4
///
/// - **PWM_OC3** `[6]` (RW)
///
///   PWM_OC3
///
/// - **PWM_OC5** `[5]` (RW)
///
///   PWM_OC5
///
/// - **PWM_OC1** `[4]` (RW)
///
///   PWM_OC1
///
/// - **PWM_EN** `[1:0]` (RW)
///
///   PWM_EN
#[repr(transparent)]
pub struct Spec {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spec {
    type Target = crate::common::VolatileCell<u32>;
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

    /// **PWM_OC4**
    ///
    /// PWM_OC4
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PWM_OC4_OFFSET: u32 = 7;
    pub const PWM_OC4_MASK: u32 = 0x00000080;
    pub const PWM_OC4: u32 = 1 << 7;

    /// **PWM_OC3**
    ///
    /// PWM_OC3
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PWM_OC3_OFFSET: u32 = 6;
    pub const PWM_OC3_MASK: u32 = 0x00000040;
    pub const PWM_OC3: u32 = 1 << 6;

    /// **PWM_OC5**
    ///
    /// PWM_OC5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const PWM_OC5_OFFSET: u32 = 5;
    pub const PWM_OC5_MASK: u32 = 0x00000020;
    pub const PWM_OC5: u32 = 1 << 5;

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
    /// Bits `[1:0]`, mask `0x00000003`
    pub const PWM_EN_OFFSET: u32 = 0;
    pub const PWM_EN_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TOGGLE** `[15]` (RO)
    ///
    /// TOGGLE
    #[inline(always)]
    pub fn toggle(&self) -> bool {
        (self.value.read_bits() & Self::TOGGLE_MASK) != 0
    }

    /// **PWM_OC4** `[7]` (RW)
    ///
    /// PWM_OC4
    #[inline(always)]
    pub fn pwm_oc4(&self) -> bool {
        (self.value.read_bits() & Self::PWM_OC4_MASK) != 0
    }

    /// Set PWM_OC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwm_oc4(&self) {
        self.value.modify_bits(|r| r | Self::PWM_OC4_MASK);
    }

    /// Clear PWM_OC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwm_oc4(&self) {
        self.value.modify_bits(|r| r & !Self::PWM_OC4_MASK);
    }

    /// **PWM_OC3** `[6]` (RW)
    ///
    /// PWM_OC3
    #[inline(always)]
    pub fn pwm_oc3(&self) -> bool {
        (self.value.read_bits() & Self::PWM_OC3_MASK) != 0
    }

    /// Set PWM_OC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwm_oc3(&self) {
        self.value.modify_bits(|r| r | Self::PWM_OC3_MASK);
    }

    /// Clear PWM_OC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwm_oc3(&self) {
        self.value.modify_bits(|r| r & !Self::PWM_OC3_MASK);
    }

    /// **PWM_OC5** `[5]` (RW)
    ///
    /// PWM_OC5
    #[inline(always)]
    pub fn pwm_oc5(&self) -> bool {
        (self.value.read_bits() & Self::PWM_OC5_MASK) != 0
    }

    /// Set PWM_OC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwm_oc5(&self) {
        self.value.modify_bits(|r| r | Self::PWM_OC5_MASK);
    }

    /// Clear PWM_OC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwm_oc5(&self) {
        self.value.modify_bits(|r| r & !Self::PWM_OC5_MASK);
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

    /// **PWM_EN** `[1:0]` (RW)
    ///
    /// PWM_EN
    #[inline(always)]
    pub fn pwm_en(&self) -> u32 {
        (self.value.read_bits() & Self::PWM_EN_MASK) >> Self::PWM_EN_OFFSET
    }

    /// Write **PWM_EN** `[1:0]` (RW)
    ///
    /// PWM_EN
    #[inline(always)]
    pub fn write_pwm_en(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PWM_EN_MASK) | ((value << Self::PWM_EN_OFFSET) & Self::PWM_EN_MASK)
        });
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

    /// Read **PWM_OC4** `[7:7]`
    ///
    /// PWM_OC4
    #[inline(always)]
    pub const fn pwm_oc4(&self) -> bool {
        (self.bits & Spec::PWM_OC4_MASK) != 0
    }

    /// Read **PWM_OC3** `[6:6]`
    ///
    /// PWM_OC3
    #[inline(always)]
    pub const fn pwm_oc3(&self) -> bool {
        (self.bits & Spec::PWM_OC3_MASK) != 0
    }

    /// Read **PWM_OC5** `[5:5]`
    ///
    /// PWM_OC5
    #[inline(always)]
    pub const fn pwm_oc5(&self) -> bool {
        (self.bits & Spec::PWM_OC5_MASK) != 0
    }

    /// Read **PWM_OC1** `[4:4]`
    ///
    /// PWM_OC1
    #[inline(always)]
    pub const fn pwm_oc1(&self) -> bool {
        (self.bits & Spec::PWM_OC1_MASK) != 0
    }

    /// Read **PWM_EN** `[1:0]`
    ///
    /// PWM_EN
    #[inline(always)]
    pub const fn pwm_en(&self) -> u32 {
        (self.bits & Spec::PWM_EN_MASK) >> Spec::PWM_EN_OFFSET
    }

    /// Set **PWM_OC4** `[7:7]`
    ///
    /// PWM_OC4
    #[inline(always)]
    pub const fn with_pwm_oc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spec::PWM_OC4_MASK
            } else {
                self.bits & !Spec::PWM_OC4_MASK
            },
        }
    }

    /// Set **PWM_OC3** `[6:6]`
    ///
    /// PWM_OC3
    #[inline(always)]
    pub const fn with_pwm_oc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spec::PWM_OC3_MASK
            } else {
                self.bits & !Spec::PWM_OC3_MASK
            },
        }
    }

    /// Set **PWM_OC5** `[5:5]`
    ///
    /// PWM_OC5
    #[inline(always)]
    pub const fn with_pwm_oc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spec::PWM_OC5_MASK
            } else {
                self.bits & !Spec::PWM_OC5_MASK
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

    /// Set **PWM_EN** `[1:0]`
    ///
    /// PWM_EN
    #[inline(always)]
    pub const fn with_pwm_en(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spec::PWM_EN_MASK)
                | ((value << Spec::PWM_EN_OFFSET) & Spec::PWM_EN_MASK),
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
    /// 0x0004
    /// control register 2
    pub ctlr2: Ctlr2,
    /// 0x0008
    /// slave mode control register
    pub smcfgr: Smcfgr,
    /// 0x000c
    /// DMA/Interrupt enable register
    pub dmaintenr: Dmaintenr,
    /// 0x0010
    /// status register
    pub intfr: Intfr,
    /// 0x0014
    /// event generation register
    pub swevgr: Swevgr,
    /// 0x0018
    /// capture/compare mode register (output
    /// mode)
    pub chctlr1_output: Chctlr1Output,
    /// 0x0018
    /// capture/compare mode register 1 (input
    /// mode)
    pub chctlr1_input: Chctlr1Input,
    /// 0x001c
    /// capture/compare mode register (output
    /// mode)
    pub chctlr2_output: Chctlr2Output,
    /// 0x001c
    /// capture/compare mode register 2 (input
    /// mode)
    pub chctlr2_input: Chctlr2Input,
    /// 0x0020
    /// capture/compare enable
    /// register
    pub ccer: Ccer,
    /// 0x0024
    /// counter
    pub cnt: Cnt,
    /// 0x0028
    /// prescaler
    pub psc: Psc,
    /// 0x002c
    /// auto-reload register
    pub atrlr: Atrlr,
    /// 0x0030
    /// repetition counter register
    pub rptcr: Rptcr,
    /// 0x0034
    /// capture/compare register 1
    pub ch1cvr: Ch1cvr,
    /// 0x0038
    /// capture/compare register 2
    pub ch2cvr: Ch2cvr,
    /// 0x003c
    /// capture/compare register 3
    pub ch3cvr: Ch3cvr,
    /// 0x0040
    /// capture/compare register 4
    pub ch4cvr: Ch4cvr,
    /// 0x0044
    /// break and dead-time register
    pub bdtr: Bdtr,
    /// 0x0048
    /// DMA control register
    pub dmacfgr: Dmacfgr,
    /// 0x004c
    /// DMA address for full transfer
    pub dmaadr: Dmaadr,
    /// 0x0050
    /// SPEC
    pub spec: Spec,
}
