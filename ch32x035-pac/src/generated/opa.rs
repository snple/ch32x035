//! OPA
//! OPA configuration

#![allow(dead_code)]

pub const BASE: u32 = 0x40026000;

/// CFGR1 Register
///
/// OPA Configuration register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IF_CNT** `[14]` (RW)
///
///   IF_CNT Configuration
///
/// - **IF_OUT2** `[13]` (RW)
///
///   IF_OUT2 Enable
///
/// - **IF_OUT1** `[12]` (RW)
///
///   IF_OUT1 Enable
///
/// - **NMI_EN** `[11]` (RW)
///
///   NMI_EN Enable
///
/// - **IE_CNT** `[10]` (RW)
///
///   IE_CNT Enable
///
/// - **IE_OUT2** `[9]` (RW)
///
///   IE_OUT2 Enable
///
/// - **IE_OUT1** `[8]` (RW)
///
///   IE_OUT1 Enable
///
/// - **POLL_LOCK** `[7]` (RW)
///
///   POLL_LOCK select
///
/// - **BKIN_SEL** `[6]` (RW)
///
///   BKIN_SEL select
///
/// - **RST_EN2** `[5]` (RW)
///
///   RST_EN2 Enable
///
/// - **RST_EN1** `[4]` (RW)
///
///   RST_EN1 Enable
///
/// - **BKIN_EN2** `[3]` (RW)
///
///   BKIN_EN2 Enable
///
/// - **BKIN_EN1** `[2]` (RW)
///
///   BKIN_EN1 Enable
///
/// - **POLL_EN2** `[1]` (RW)
///
///   POLL_EN2 Enable
///
/// - **POLL_EN1** `[0]` (RW)
///
///   POLL_EN1 Enable
#[repr(transparent)]
pub struct Cfgr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr1 {
    // ======== Field Constants ========

    /// **IF_CNT**
    ///
    /// IF_CNT Configuration
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IF_CNT_OFFSET: u32 = 14;
    pub const IF_CNT_MASK: u32 = 0x00004000;
    pub const IF_CNT: u32 = 1 << 14;

    /// **IF_OUT2**
    ///
    /// IF_OUT2 Enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IF_OUT2_OFFSET: u32 = 13;
    pub const IF_OUT2_MASK: u32 = 0x00002000;
    pub const IF_OUT2: u32 = 1 << 13;

    /// **IF_OUT1**
    ///
    /// IF_OUT1 Enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IF_OUT1_OFFSET: u32 = 12;
    pub const IF_OUT1_MASK: u32 = 0x00001000;
    pub const IF_OUT1: u32 = 1 << 12;

    /// **NMI_EN**
    ///
    /// NMI_EN Enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const NMI_EN_OFFSET: u32 = 11;
    pub const NMI_EN_MASK: u32 = 0x00000800;
    pub const NMI_EN: u32 = 1 << 11;

    /// **IE_CNT**
    ///
    /// IE_CNT Enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const IE_CNT_OFFSET: u32 = 10;
    pub const IE_CNT_MASK: u32 = 0x00000400;
    pub const IE_CNT: u32 = 1 << 10;

    /// **IE_OUT2**
    ///
    /// IE_OUT2 Enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const IE_OUT2_OFFSET: u32 = 9;
    pub const IE_OUT2_MASK: u32 = 0x00000200;
    pub const IE_OUT2: u32 = 1 << 9;

    /// **IE_OUT1**
    ///
    /// IE_OUT1 Enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IE_OUT1_OFFSET: u32 = 8;
    pub const IE_OUT1_MASK: u32 = 0x00000100;
    pub const IE_OUT1: u32 = 1 << 8;

    /// **POLL_LOCK**
    ///
    /// POLL_LOCK select
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const POLL_LOCK_OFFSET: u32 = 7;
    pub const POLL_LOCK_MASK: u32 = 0x00000080;
    pub const POLL_LOCK: u32 = 1 << 7;

    /// **BKIN_SEL**
    ///
    /// BKIN_SEL select
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const BKIN_SEL_OFFSET: u32 = 6;
    pub const BKIN_SEL_MASK: u32 = 0x00000040;
    pub const BKIN_SEL: u32 = 1 << 6;

    /// **RST_EN2**
    ///
    /// RST_EN2 Enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RST_EN2_OFFSET: u32 = 5;
    pub const RST_EN2_MASK: u32 = 0x00000020;
    pub const RST_EN2: u32 = 1 << 5;

    /// **RST_EN1**
    ///
    /// RST_EN1 Enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RST_EN1_OFFSET: u32 = 4;
    pub const RST_EN1_MASK: u32 = 0x00000010;
    pub const RST_EN1: u32 = 1 << 4;

    /// **BKIN_EN2**
    ///
    /// BKIN_EN2 Enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BKIN_EN2_OFFSET: u32 = 3;
    pub const BKIN_EN2_MASK: u32 = 0x00000008;
    pub const BKIN_EN2: u32 = 1 << 3;

    /// **BKIN_EN1**
    ///
    /// BKIN_EN1 Enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BKIN_EN1_OFFSET: u32 = 2;
    pub const BKIN_EN1_MASK: u32 = 0x00000004;
    pub const BKIN_EN1: u32 = 1 << 2;

    /// **POLL_EN2**
    ///
    /// POLL_EN2 Enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const POLL_EN2_OFFSET: u32 = 1;
    pub const POLL_EN2_MASK: u32 = 0x00000002;
    pub const POLL_EN2: u32 = 1 << 1;

    /// **POLL_EN1**
    ///
    /// POLL_EN1 Enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const POLL_EN1_OFFSET: u32 = 0;
    pub const POLL_EN1_MASK: u32 = 0x00000001;
    pub const POLL_EN1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IF_CNT** `[14]` (RW)
    ///
    /// IF_CNT Configuration
    #[inline(always)]
    pub fn if_cnt(&self) -> bool {
        (self.value.read_bits() & Self::IF_CNT_MASK) != 0
    }

    /// Set IF_CNT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_cnt(&self) {
        self.value.modify_bits(|r| r | Self::IF_CNT_MASK);
    }

    /// Clear IF_CNT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_cnt(&self) {
        self.value.modify_bits(|r| r & !Self::IF_CNT_MASK);
    }

    /// **IF_OUT2** `[13]` (RW)
    ///
    /// IF_OUT2 Enable
    #[inline(always)]
    pub fn if_out2(&self) -> bool {
        (self.value.read_bits() & Self::IF_OUT2_MASK) != 0
    }

    /// Set IF_OUT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_out2(&self) {
        self.value.modify_bits(|r| r | Self::IF_OUT2_MASK);
    }

    /// Clear IF_OUT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_out2(&self) {
        self.value.modify_bits(|r| r & !Self::IF_OUT2_MASK);
    }

    /// **IF_OUT1** `[12]` (RW)
    ///
    /// IF_OUT1 Enable
    #[inline(always)]
    pub fn if_out1(&self) -> bool {
        (self.value.read_bits() & Self::IF_OUT1_MASK) != 0
    }

    /// Set IF_OUT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_out1(&self) {
        self.value.modify_bits(|r| r | Self::IF_OUT1_MASK);
    }

    /// Clear IF_OUT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_out1(&self) {
        self.value.modify_bits(|r| r & !Self::IF_OUT1_MASK);
    }

    /// **NMI_EN** `[11]` (RW)
    ///
    /// NMI_EN Enable
    #[inline(always)]
    pub fn nmi_en(&self) -> bool {
        (self.value.read_bits() & Self::NMI_EN_MASK) != 0
    }

    /// Set NMI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nmi_en(&self) {
        self.value.modify_bits(|r| r | Self::NMI_EN_MASK);
    }

    /// Clear NMI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nmi_en(&self) {
        self.value.modify_bits(|r| r & !Self::NMI_EN_MASK);
    }

    /// **IE_CNT** `[10]` (RW)
    ///
    /// IE_CNT Enable
    #[inline(always)]
    pub fn ie_cnt(&self) -> bool {
        (self.value.read_bits() & Self::IE_CNT_MASK) != 0
    }

    /// Set IE_CNT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_cnt(&self) {
        self.value.modify_bits(|r| r | Self::IE_CNT_MASK);
    }

    /// Clear IE_CNT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_cnt(&self) {
        self.value.modify_bits(|r| r & !Self::IE_CNT_MASK);
    }

    /// **IE_OUT2** `[9]` (RW)
    ///
    /// IE_OUT2 Enable
    #[inline(always)]
    pub fn ie_out2(&self) -> bool {
        (self.value.read_bits() & Self::IE_OUT2_MASK) != 0
    }

    /// Set IE_OUT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_out2(&self) {
        self.value.modify_bits(|r| r | Self::IE_OUT2_MASK);
    }

    /// Clear IE_OUT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_out2(&self) {
        self.value.modify_bits(|r| r & !Self::IE_OUT2_MASK);
    }

    /// **IE_OUT1** `[8]` (RW)
    ///
    /// IE_OUT1 Enable
    #[inline(always)]
    pub fn ie_out1(&self) -> bool {
        (self.value.read_bits() & Self::IE_OUT1_MASK) != 0
    }

    /// Set IE_OUT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_out1(&self) {
        self.value.modify_bits(|r| r | Self::IE_OUT1_MASK);
    }

    /// Clear IE_OUT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_out1(&self) {
        self.value.modify_bits(|r| r & !Self::IE_OUT1_MASK);
    }

    /// **POLL_LOCK** `[7]` (RW)
    ///
    /// POLL_LOCK select
    #[inline(always)]
    pub fn poll_lock(&self) -> bool {
        (self.value.read_bits() & Self::POLL_LOCK_MASK) != 0
    }

    /// Set POLL_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_poll_lock(&self) {
        self.value.modify_bits(|r| r | Self::POLL_LOCK_MASK);
    }

    /// Clear POLL_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_poll_lock(&self) {
        self.value.modify_bits(|r| r & !Self::POLL_LOCK_MASK);
    }

    /// **BKIN_SEL** `[6]` (RW)
    ///
    /// BKIN_SEL select
    #[inline(always)]
    pub fn bkin_sel(&self) -> bool {
        (self.value.read_bits() & Self::BKIN_SEL_MASK) != 0
    }

    /// Set BKIN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bkin_sel(&self) {
        self.value.modify_bits(|r| r | Self::BKIN_SEL_MASK);
    }

    /// Clear BKIN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bkin_sel(&self) {
        self.value.modify_bits(|r| r & !Self::BKIN_SEL_MASK);
    }

    /// **RST_EN2** `[5]` (RW)
    ///
    /// RST_EN2 Enable
    #[inline(always)]
    pub fn rst_en2(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN2_MASK) != 0
    }

    /// Set RST_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en2(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN2_MASK);
    }

    /// Clear RST_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en2(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN2_MASK);
    }

    /// **RST_EN1** `[4]` (RW)
    ///
    /// RST_EN1 Enable
    #[inline(always)]
    pub fn rst_en1(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN1_MASK) != 0
    }

    /// Set RST_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en1(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN1_MASK);
    }

    /// Clear RST_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en1(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN1_MASK);
    }

    /// **BKIN_EN2** `[3]` (RW)
    ///
    /// BKIN_EN2 Enable
    #[inline(always)]
    pub fn bkin_en2(&self) -> bool {
        (self.value.read_bits() & Self::BKIN_EN2_MASK) != 0
    }

    /// Set BKIN_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bkin_en2(&self) {
        self.value.modify_bits(|r| r | Self::BKIN_EN2_MASK);
    }

    /// Clear BKIN_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bkin_en2(&self) {
        self.value.modify_bits(|r| r & !Self::BKIN_EN2_MASK);
    }

    /// **BKIN_EN1** `[2]` (RW)
    ///
    /// BKIN_EN1 Enable
    #[inline(always)]
    pub fn bkin_en1(&self) -> bool {
        (self.value.read_bits() & Self::BKIN_EN1_MASK) != 0
    }

    /// Set BKIN_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bkin_en1(&self) {
        self.value.modify_bits(|r| r | Self::BKIN_EN1_MASK);
    }

    /// Clear BKIN_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bkin_en1(&self) {
        self.value.modify_bits(|r| r & !Self::BKIN_EN1_MASK);
    }

    /// **POLL_EN2** `[1]` (RW)
    ///
    /// POLL_EN2 Enable
    #[inline(always)]
    pub fn poll_en2(&self) -> bool {
        (self.value.read_bits() & Self::POLL_EN2_MASK) != 0
    }

    /// Set POLL_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_poll_en2(&self) {
        self.value.modify_bits(|r| r | Self::POLL_EN2_MASK);
    }

    /// Clear POLL_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_poll_en2(&self) {
        self.value.modify_bits(|r| r & !Self::POLL_EN2_MASK);
    }

    /// **POLL_EN1** `[0]` (RW)
    ///
    /// POLL_EN1 Enable
    #[inline(always)]
    pub fn poll_en1(&self) -> bool {
        (self.value.read_bits() & Self::POLL_EN1_MASK) != 0
    }

    /// Set POLL_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_poll_en1(&self) {
        self.value.modify_bits(|r| r | Self::POLL_EN1_MASK);
    }

    /// Clear POLL_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_poll_en1(&self) {
        self.value.modify_bits(|r| r & !Self::POLL_EN1_MASK);
    }
}

/// CFGR1 value type for batch read/modify/write operations
///
/// Use `cfgr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr1Val {
    bits: u32,
}

impl Cfgr1Val {
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

    /// Read **IF_CNT** `[14:14]`
    ///
    /// IF_CNT Configuration
    #[inline(always)]
    pub const fn if_cnt(&self) -> bool {
        (self.bits & Cfgr1::IF_CNT_MASK) != 0
    }

    /// Read **IF_OUT2** `[13:13]`
    ///
    /// IF_OUT2 Enable
    #[inline(always)]
    pub const fn if_out2(&self) -> bool {
        (self.bits & Cfgr1::IF_OUT2_MASK) != 0
    }

    /// Read **IF_OUT1** `[12:12]`
    ///
    /// IF_OUT1 Enable
    #[inline(always)]
    pub const fn if_out1(&self) -> bool {
        (self.bits & Cfgr1::IF_OUT1_MASK) != 0
    }

    /// Read **NMI_EN** `[11:11]`
    ///
    /// NMI_EN Enable
    #[inline(always)]
    pub const fn nmi_en(&self) -> bool {
        (self.bits & Cfgr1::NMI_EN_MASK) != 0
    }

    /// Read **IE_CNT** `[10:10]`
    ///
    /// IE_CNT Enable
    #[inline(always)]
    pub const fn ie_cnt(&self) -> bool {
        (self.bits & Cfgr1::IE_CNT_MASK) != 0
    }

    /// Read **IE_OUT2** `[9:9]`
    ///
    /// IE_OUT2 Enable
    #[inline(always)]
    pub const fn ie_out2(&self) -> bool {
        (self.bits & Cfgr1::IE_OUT2_MASK) != 0
    }

    /// Read **IE_OUT1** `[8:8]`
    ///
    /// IE_OUT1 Enable
    #[inline(always)]
    pub const fn ie_out1(&self) -> bool {
        (self.bits & Cfgr1::IE_OUT1_MASK) != 0
    }

    /// Read **POLL_LOCK** `[7:7]`
    ///
    /// POLL_LOCK select
    #[inline(always)]
    pub const fn poll_lock(&self) -> bool {
        (self.bits & Cfgr1::POLL_LOCK_MASK) != 0
    }

    /// Read **BKIN_SEL** `[6:6]`
    ///
    /// BKIN_SEL select
    #[inline(always)]
    pub const fn bkin_sel(&self) -> bool {
        (self.bits & Cfgr1::BKIN_SEL_MASK) != 0
    }

    /// Read **RST_EN2** `[5:5]`
    ///
    /// RST_EN2 Enable
    #[inline(always)]
    pub const fn rst_en2(&self) -> bool {
        (self.bits & Cfgr1::RST_EN2_MASK) != 0
    }

    /// Read **RST_EN1** `[4:4]`
    ///
    /// RST_EN1 Enable
    #[inline(always)]
    pub const fn rst_en1(&self) -> bool {
        (self.bits & Cfgr1::RST_EN1_MASK) != 0
    }

    /// Read **BKIN_EN2** `[3:3]`
    ///
    /// BKIN_EN2 Enable
    #[inline(always)]
    pub const fn bkin_en2(&self) -> bool {
        (self.bits & Cfgr1::BKIN_EN2_MASK) != 0
    }

    /// Read **BKIN_EN1** `[2:2]`
    ///
    /// BKIN_EN1 Enable
    #[inline(always)]
    pub const fn bkin_en1(&self) -> bool {
        (self.bits & Cfgr1::BKIN_EN1_MASK) != 0
    }

    /// Read **POLL_EN2** `[1:1]`
    ///
    /// POLL_EN2 Enable
    #[inline(always)]
    pub const fn poll_en2(&self) -> bool {
        (self.bits & Cfgr1::POLL_EN2_MASK) != 0
    }

    /// Read **POLL_EN1** `[0:0]`
    ///
    /// POLL_EN1 Enable
    #[inline(always)]
    pub const fn poll_en1(&self) -> bool {
        (self.bits & Cfgr1::POLL_EN1_MASK) != 0
    }

    /// Set **IF_CNT** `[14:14]`
    ///
    /// IF_CNT Configuration
    #[inline(always)]
    pub const fn with_if_cnt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::IF_CNT_MASK
            } else {
                self.bits & !Cfgr1::IF_CNT_MASK
            },
        }
    }

    /// Set **IF_OUT2** `[13:13]`
    ///
    /// IF_OUT2 Enable
    #[inline(always)]
    pub const fn with_if_out2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::IF_OUT2_MASK
            } else {
                self.bits & !Cfgr1::IF_OUT2_MASK
            },
        }
    }

    /// Set **IF_OUT1** `[12:12]`
    ///
    /// IF_OUT1 Enable
    #[inline(always)]
    pub const fn with_if_out1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::IF_OUT1_MASK
            } else {
                self.bits & !Cfgr1::IF_OUT1_MASK
            },
        }
    }

    /// Set **NMI_EN** `[11:11]`
    ///
    /// NMI_EN Enable
    #[inline(always)]
    pub const fn with_nmi_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::NMI_EN_MASK
            } else {
                self.bits & !Cfgr1::NMI_EN_MASK
            },
        }
    }

    /// Set **IE_CNT** `[10:10]`
    ///
    /// IE_CNT Enable
    #[inline(always)]
    pub const fn with_ie_cnt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::IE_CNT_MASK
            } else {
                self.bits & !Cfgr1::IE_CNT_MASK
            },
        }
    }

    /// Set **IE_OUT2** `[9:9]`
    ///
    /// IE_OUT2 Enable
    #[inline(always)]
    pub const fn with_ie_out2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::IE_OUT2_MASK
            } else {
                self.bits & !Cfgr1::IE_OUT2_MASK
            },
        }
    }

    /// Set **IE_OUT1** `[8:8]`
    ///
    /// IE_OUT1 Enable
    #[inline(always)]
    pub const fn with_ie_out1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::IE_OUT1_MASK
            } else {
                self.bits & !Cfgr1::IE_OUT1_MASK
            },
        }
    }

    /// Set **POLL_LOCK** `[7:7]`
    ///
    /// POLL_LOCK select
    #[inline(always)]
    pub const fn with_poll_lock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::POLL_LOCK_MASK
            } else {
                self.bits & !Cfgr1::POLL_LOCK_MASK
            },
        }
    }

    /// Set **BKIN_SEL** `[6:6]`
    ///
    /// BKIN_SEL select
    #[inline(always)]
    pub const fn with_bkin_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::BKIN_SEL_MASK
            } else {
                self.bits & !Cfgr1::BKIN_SEL_MASK
            },
        }
    }

    /// Set **RST_EN2** `[5:5]`
    ///
    /// RST_EN2 Enable
    #[inline(always)]
    pub const fn with_rst_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::RST_EN2_MASK
            } else {
                self.bits & !Cfgr1::RST_EN2_MASK
            },
        }
    }

    /// Set **RST_EN1** `[4:4]`
    ///
    /// RST_EN1 Enable
    #[inline(always)]
    pub const fn with_rst_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::RST_EN1_MASK
            } else {
                self.bits & !Cfgr1::RST_EN1_MASK
            },
        }
    }

    /// Set **BKIN_EN2** `[3:3]`
    ///
    /// BKIN_EN2 Enable
    #[inline(always)]
    pub const fn with_bkin_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::BKIN_EN2_MASK
            } else {
                self.bits & !Cfgr1::BKIN_EN2_MASK
            },
        }
    }

    /// Set **BKIN_EN1** `[2:2]`
    ///
    /// BKIN_EN1 Enable
    #[inline(always)]
    pub const fn with_bkin_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::BKIN_EN1_MASK
            } else {
                self.bits & !Cfgr1::BKIN_EN1_MASK
            },
        }
    }

    /// Set **POLL_EN2** `[1:1]`
    ///
    /// POLL_EN2 Enable
    #[inline(always)]
    pub const fn with_poll_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::POLL_EN2_MASK
            } else {
                self.bits & !Cfgr1::POLL_EN2_MASK
            },
        }
    }

    /// Set **POLL_EN1** `[0:0]`
    ///
    /// POLL_EN1 Enable
    #[inline(always)]
    pub const fn with_poll_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::POLL_EN1_MASK
            } else {
                self.bits & !Cfgr1::POLL_EN1_MASK
            },
        }
    }
}

impl Default for Cfgr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr1Val {
        Cfgr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr1Val) -> Cfgr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR2 Register
///
/// OPA Configuration register
///
/// Address offset: 0x0002
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **POLL2_NUM** `[12:11]` (WO)
///
///   POLL2_NUM Configuration
///
/// - **POLL1_NUM** `[10:9]` (WO)
///
///   POLL1_NUM Configuration
///
/// - **POLL_VLU** `[8:0]` (RW)
///
///   POLL_VLU Configuration
#[repr(transparent)]
pub struct Cfgr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr2 {
    // ======== Field Constants ========

    /// **POLL2_NUM**
    ///
    /// POLL2_NUM Configuration
    ///
    /// Bits `[12:11]`, mask `0x00001800`
    pub const POLL2_NUM_OFFSET: u32 = 11;
    pub const POLL2_NUM_MASK: u32 = 0x00001800;

    /// **POLL1_NUM**
    ///
    /// POLL1_NUM Configuration
    ///
    /// Bits `[10:9]`, mask `0x00000600`
    pub const POLL1_NUM_OFFSET: u32 = 9;
    pub const POLL1_NUM_MASK: u32 = 0x00000600;

    /// **POLL_VLU**
    ///
    /// POLL_VLU Configuration
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const POLL_VLU_OFFSET: u32 = 0;
    pub const POLL_VLU_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// Write **POLL2_NUM** `[12:11]` (WO)
    ///
    /// POLL2_NUM Configuration
    #[inline(always)]
    pub fn write_poll2_num(&self, value: u32) {
        self.value
            .write_bits((value << Self::POLL2_NUM_OFFSET) & Self::POLL2_NUM_MASK);
    }

    /// Write **POLL1_NUM** `[10:9]` (WO)
    ///
    /// POLL1_NUM Configuration
    #[inline(always)]
    pub fn write_poll1_num(&self, value: u32) {
        self.value
            .write_bits((value << Self::POLL1_NUM_OFFSET) & Self::POLL1_NUM_MASK);
    }

    /// **POLL_VLU** `[8:0]` (RW)
    ///
    /// POLL_VLU Configuration
    #[inline(always)]
    pub fn poll_vlu(&self) -> u32 {
        (self.value.read_bits() & Self::POLL_VLU_MASK) >> Self::POLL_VLU_OFFSET
    }

    /// Write **POLL_VLU** `[8:0]` (RW)
    ///
    /// POLL_VLU Configuration
    #[inline(always)]
    pub fn write_poll_vlu(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::POLL_VLU_MASK) | ((value << Self::POLL_VLU_OFFSET) & Self::POLL_VLU_MASK)
        });
    }
}

/// CFGR2 value type for batch read/modify/write operations
///
/// Use `cfgr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr2Val {
    bits: u32,
}

impl Cfgr2Val {
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

    /// Read **POLL_VLU** `[8:0]`
    ///
    /// POLL_VLU Configuration
    #[inline(always)]
    pub const fn poll_vlu(&self) -> u32 {
        (self.bits & Cfgr2::POLL_VLU_MASK) >> Cfgr2::POLL_VLU_OFFSET
    }

    /// Set **POLL2_NUM** `[12:11]`
    ///
    /// POLL2_NUM Configuration
    #[inline(always)]
    pub const fn with_poll2_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr2::POLL2_NUM_MASK)
                | ((value << Cfgr2::POLL2_NUM_OFFSET) & Cfgr2::POLL2_NUM_MASK),
        }
    }

    /// Set **POLL1_NUM** `[10:9]`
    ///
    /// POLL1_NUM Configuration
    #[inline(always)]
    pub const fn with_poll1_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr2::POLL1_NUM_MASK)
                | ((value << Cfgr2::POLL1_NUM_OFFSET) & Cfgr2::POLL1_NUM_MASK),
        }
    }

    /// Set **POLL_VLU** `[8:0]`
    ///
    /// POLL_VLU Configuration
    #[inline(always)]
    pub const fn with_poll_vlu(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr2::POLL_VLU_MASK)
                | ((value << Cfgr2::POLL_VLU_OFFSET) & Cfgr2::POLL_VLU_MASK),
        }
    }
}

impl Default for Cfgr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr2Val {
        Cfgr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr2Val) -> Cfgr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTLR1 Register
///
/// OPA Control1 register
///
/// Address offset: 0x0004
/// Reset value: 0x81d801d8
///
/// ## Fields
///
/// - **OPA_LOCK** `[31]` (RW)
///
///   OPA_LOCK
///
/// - **NSEL2** `[24:22]` (RW)
///
///   output select
///
/// - **FB_EN2** `[21]` (RW)
///
///   res inside enable
///
/// - **PSEL2** `[20:19]` (RW)
///
///   Input select
///
/// - **MODE2** `[17]` (RW)
///
///   OPA2 mode
///
/// - **EN2** `[16]` (RW)
///
///   OPA2 enable
///
/// - **NSEL1** `[8:6]` (RW)
///
///   output select
///
/// - **FB_EN1** `[5]` (RW)
///
///   res inside enable
///
/// - **PSEL1** `[4:3]` (RW)
///
///   Input select
///
/// - **MODE1** `[1]` (RW)
///
///   OPA1 mode
///
/// - **EN1** `[0]` (RW)
///
///   OPA1 enable
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

    /// **OPA_LOCK**
    ///
    /// OPA_LOCK
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const OPA_LOCK_OFFSET: u32 = 31;
    pub const OPA_LOCK_MASK: u32 = 0x80000000;
    pub const OPA_LOCK: u32 = 1 << 31;

    /// **NSEL2**
    ///
    /// output select
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const NSEL2_OFFSET: u32 = 22;
    pub const NSEL2_MASK: u32 = 0x01c00000;

    /// **FB_EN2**
    ///
    /// res inside enable
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const FB_EN2_OFFSET: u32 = 21;
    pub const FB_EN2_MASK: u32 = 0x00200000;
    pub const FB_EN2: u32 = 1 << 21;

    /// **PSEL2**
    ///
    /// Input select
    ///
    /// Bits `[20:19]`, mask `0x00180000`
    pub const PSEL2_OFFSET: u32 = 19;
    pub const PSEL2_MASK: u32 = 0x00180000;

    /// **MODE2**
    ///
    /// OPA2 mode
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MODE2_OFFSET: u32 = 17;
    pub const MODE2_MASK: u32 = 0x00020000;
    pub const MODE2: u32 = 1 << 17;

    /// **EN2**
    ///
    /// OPA2 enable
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EN2_OFFSET: u32 = 16;
    pub const EN2_MASK: u32 = 0x00010000;
    pub const EN2: u32 = 1 << 16;

    /// **NSEL1**
    ///
    /// output select
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const NSEL1_OFFSET: u32 = 6;
    pub const NSEL1_MASK: u32 = 0x000001c0;

    /// **FB_EN1**
    ///
    /// res inside enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FB_EN1_OFFSET: u32 = 5;
    pub const FB_EN1_MASK: u32 = 0x00000020;
    pub const FB_EN1: u32 = 1 << 5;

    /// **PSEL1**
    ///
    /// Input select
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const PSEL1_OFFSET: u32 = 3;
    pub const PSEL1_MASK: u32 = 0x00000018;

    /// **MODE1**
    ///
    /// OPA1 mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MODE1_OFFSET: u32 = 1;
    pub const MODE1_MASK: u32 = 0x00000002;
    pub const MODE1: u32 = 1 << 1;

    /// **EN1**
    ///
    /// OPA1 enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN1_OFFSET: u32 = 0;
    pub const EN1_MASK: u32 = 0x00000001;
    pub const EN1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OPA_LOCK** `[31]` (RW)
    ///
    /// OPA_LOCK
    #[inline(always)]
    pub fn opa_lock(&self) -> bool {
        (self.value.read_bits() & Self::OPA_LOCK_MASK) != 0
    }

    /// Set OPA_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_opa_lock(&self) {
        self.value.modify_bits(|r| r | Self::OPA_LOCK_MASK);
    }

    /// Clear OPA_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_opa_lock(&self) {
        self.value.modify_bits(|r| r & !Self::OPA_LOCK_MASK);
    }

    /// **NSEL2** `[24:22]` (RW)
    ///
    /// output select
    #[inline(always)]
    pub fn nsel2(&self) -> u32 {
        (self.value.read_bits() & Self::NSEL2_MASK) >> Self::NSEL2_OFFSET
    }

    /// Write **NSEL2** `[24:22]` (RW)
    ///
    /// output select
    #[inline(always)]
    pub fn write_nsel2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NSEL2_MASK) | ((value << Self::NSEL2_OFFSET) & Self::NSEL2_MASK)
        });
    }

    /// **FB_EN2** `[21]` (RW)
    ///
    /// res inside enable
    #[inline(always)]
    pub fn fb_en2(&self) -> bool {
        (self.value.read_bits() & Self::FB_EN2_MASK) != 0
    }

    /// Set FB_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fb_en2(&self) {
        self.value.modify_bits(|r| r | Self::FB_EN2_MASK);
    }

    /// Clear FB_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fb_en2(&self) {
        self.value.modify_bits(|r| r & !Self::FB_EN2_MASK);
    }

    /// **PSEL2** `[20:19]` (RW)
    ///
    /// Input select
    #[inline(always)]
    pub fn psel2(&self) -> u32 {
        (self.value.read_bits() & Self::PSEL2_MASK) >> Self::PSEL2_OFFSET
    }

    /// Write **PSEL2** `[20:19]` (RW)
    ///
    /// Input select
    #[inline(always)]
    pub fn write_psel2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSEL2_MASK) | ((value << Self::PSEL2_OFFSET) & Self::PSEL2_MASK)
        });
    }

    /// **MODE2** `[17]` (RW)
    ///
    /// OPA2 mode
    #[inline(always)]
    pub fn mode2(&self) -> bool {
        (self.value.read_bits() & Self::MODE2_MASK) != 0
    }

    /// Set MODE2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode2(&self) {
        self.value.modify_bits(|r| r | Self::MODE2_MASK);
    }

    /// Clear MODE2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode2(&self) {
        self.value.modify_bits(|r| r & !Self::MODE2_MASK);
    }

    /// **EN2** `[16]` (RW)
    ///
    /// OPA2 enable
    #[inline(always)]
    pub fn en2(&self) -> bool {
        (self.value.read_bits() & Self::EN2_MASK) != 0
    }

    /// Set EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en2(&self) {
        self.value.modify_bits(|r| r | Self::EN2_MASK);
    }

    /// Clear EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en2(&self) {
        self.value.modify_bits(|r| r & !Self::EN2_MASK);
    }

    /// **NSEL1** `[8:6]` (RW)
    ///
    /// output select
    #[inline(always)]
    pub fn nsel1(&self) -> u32 {
        (self.value.read_bits() & Self::NSEL1_MASK) >> Self::NSEL1_OFFSET
    }

    /// Write **NSEL1** `[8:6]` (RW)
    ///
    /// output select
    #[inline(always)]
    pub fn write_nsel1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NSEL1_MASK) | ((value << Self::NSEL1_OFFSET) & Self::NSEL1_MASK)
        });
    }

    /// **FB_EN1** `[5]` (RW)
    ///
    /// res inside enable
    #[inline(always)]
    pub fn fb_en1(&self) -> bool {
        (self.value.read_bits() & Self::FB_EN1_MASK) != 0
    }

    /// Set FB_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fb_en1(&self) {
        self.value.modify_bits(|r| r | Self::FB_EN1_MASK);
    }

    /// Clear FB_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fb_en1(&self) {
        self.value.modify_bits(|r| r & !Self::FB_EN1_MASK);
    }

    /// **PSEL1** `[4:3]` (RW)
    ///
    /// Input select
    #[inline(always)]
    pub fn psel1(&self) -> u32 {
        (self.value.read_bits() & Self::PSEL1_MASK) >> Self::PSEL1_OFFSET
    }

    /// Write **PSEL1** `[4:3]` (RW)
    ///
    /// Input select
    #[inline(always)]
    pub fn write_psel1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSEL1_MASK) | ((value << Self::PSEL1_OFFSET) & Self::PSEL1_MASK)
        });
    }

    /// **MODE1** `[1]` (RW)
    ///
    /// OPA1 mode
    #[inline(always)]
    pub fn mode1(&self) -> bool {
        (self.value.read_bits() & Self::MODE1_MASK) != 0
    }

    /// Set MODE1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode1(&self) {
        self.value.modify_bits(|r| r | Self::MODE1_MASK);
    }

    /// Clear MODE1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode1(&self) {
        self.value.modify_bits(|r| r & !Self::MODE1_MASK);
    }

    /// **EN1** `[0]` (RW)
    ///
    /// OPA1 enable
    #[inline(always)]
    pub fn en1(&self) -> bool {
        (self.value.read_bits() & Self::EN1_MASK) != 0
    }

    /// Set EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en1(&self) {
        self.value.modify_bits(|r| r | Self::EN1_MASK);
    }

    /// Clear EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en1(&self) {
        self.value.modify_bits(|r| r & !Self::EN1_MASK);
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
        Self { bits: 0x81d801d8 }
    }

    /// Read **OPA_LOCK** `[31:31]`
    ///
    /// OPA_LOCK
    #[inline(always)]
    pub const fn opa_lock(&self) -> bool {
        (self.bits & Ctlr1::OPA_LOCK_MASK) != 0
    }

    /// Read **NSEL2** `[24:22]`
    ///
    /// output select
    #[inline(always)]
    pub const fn nsel2(&self) -> u32 {
        (self.bits & Ctlr1::NSEL2_MASK) >> Ctlr1::NSEL2_OFFSET
    }

    /// Read **FB_EN2** `[21:21]`
    ///
    /// res inside enable
    #[inline(always)]
    pub const fn fb_en2(&self) -> bool {
        (self.bits & Ctlr1::FB_EN2_MASK) != 0
    }

    /// Read **PSEL2** `[20:19]`
    ///
    /// Input select
    #[inline(always)]
    pub const fn psel2(&self) -> u32 {
        (self.bits & Ctlr1::PSEL2_MASK) >> Ctlr1::PSEL2_OFFSET
    }

    /// Read **MODE2** `[17:17]`
    ///
    /// OPA2 mode
    #[inline(always)]
    pub const fn mode2(&self) -> bool {
        (self.bits & Ctlr1::MODE2_MASK) != 0
    }

    /// Read **EN2** `[16:16]`
    ///
    /// OPA2 enable
    #[inline(always)]
    pub const fn en2(&self) -> bool {
        (self.bits & Ctlr1::EN2_MASK) != 0
    }

    /// Read **NSEL1** `[8:6]`
    ///
    /// output select
    #[inline(always)]
    pub const fn nsel1(&self) -> u32 {
        (self.bits & Ctlr1::NSEL1_MASK) >> Ctlr1::NSEL1_OFFSET
    }

    /// Read **FB_EN1** `[5:5]`
    ///
    /// res inside enable
    #[inline(always)]
    pub const fn fb_en1(&self) -> bool {
        (self.bits & Ctlr1::FB_EN1_MASK) != 0
    }

    /// Read **PSEL1** `[4:3]`
    ///
    /// Input select
    #[inline(always)]
    pub const fn psel1(&self) -> u32 {
        (self.bits & Ctlr1::PSEL1_MASK) >> Ctlr1::PSEL1_OFFSET
    }

    /// Read **MODE1** `[1:1]`
    ///
    /// OPA1 mode
    #[inline(always)]
    pub const fn mode1(&self) -> bool {
        (self.bits & Ctlr1::MODE1_MASK) != 0
    }

    /// Read **EN1** `[0:0]`
    ///
    /// OPA1 enable
    #[inline(always)]
    pub const fn en1(&self) -> bool {
        (self.bits & Ctlr1::EN1_MASK) != 0
    }

    /// Set **OPA_LOCK** `[31:31]`
    ///
    /// OPA_LOCK
    #[inline(always)]
    pub const fn with_opa_lock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::OPA_LOCK_MASK
            } else {
                self.bits & !Ctlr1::OPA_LOCK_MASK
            },
        }
    }

    /// Set **NSEL2** `[24:22]`
    ///
    /// output select
    #[inline(always)]
    pub const fn with_nsel2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::NSEL2_MASK)
                | ((value << Ctlr1::NSEL2_OFFSET) & Ctlr1::NSEL2_MASK),
        }
    }

    /// Set **FB_EN2** `[21:21]`
    ///
    /// res inside enable
    #[inline(always)]
    pub const fn with_fb_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::FB_EN2_MASK
            } else {
                self.bits & !Ctlr1::FB_EN2_MASK
            },
        }
    }

    /// Set **PSEL2** `[20:19]`
    ///
    /// Input select
    #[inline(always)]
    pub const fn with_psel2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::PSEL2_MASK)
                | ((value << Ctlr1::PSEL2_OFFSET) & Ctlr1::PSEL2_MASK),
        }
    }

    /// Set **MODE2** `[17:17]`
    ///
    /// OPA2 mode
    #[inline(always)]
    pub const fn with_mode2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::MODE2_MASK
            } else {
                self.bits & !Ctlr1::MODE2_MASK
            },
        }
    }

    /// Set **EN2** `[16:16]`
    ///
    /// OPA2 enable
    #[inline(always)]
    pub const fn with_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::EN2_MASK
            } else {
                self.bits & !Ctlr1::EN2_MASK
            },
        }
    }

    /// Set **NSEL1** `[8:6]`
    ///
    /// output select
    #[inline(always)]
    pub const fn with_nsel1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::NSEL1_MASK)
                | ((value << Ctlr1::NSEL1_OFFSET) & Ctlr1::NSEL1_MASK),
        }
    }

    /// Set **FB_EN1** `[5:5]`
    ///
    /// res inside enable
    #[inline(always)]
    pub const fn with_fb_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::FB_EN1_MASK
            } else {
                self.bits & !Ctlr1::FB_EN1_MASK
            },
        }
    }

    /// Set **PSEL1** `[4:3]`
    ///
    /// Input select
    #[inline(always)]
    pub const fn with_psel1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::PSEL1_MASK)
                | ((value << Ctlr1::PSEL1_OFFSET) & Ctlr1::PSEL1_MASK),
        }
    }

    /// Set **MODE1** `[1:1]`
    ///
    /// OPA1 mode
    #[inline(always)]
    pub const fn with_mode1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::MODE1_MASK
            } else {
                self.bits & !Ctlr1::MODE1_MASK
            },
        }
    }

    /// Set **EN1** `[0:0]`
    ///
    /// OPA1 enable
    #[inline(always)]
    pub const fn with_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::EN1_MASK
            } else {
                self.bits & !Ctlr1::EN1_MASK
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
/// OPA Control2 register
///
/// Address offset: 0x0008
/// Reset value: 0x80000000
///
/// ## Fields
///
/// - **CMP_LOCK** `[31]` (RW)
///
///   CMP_LOCK
///
/// - **HYEN3** `[14]` (RW)
///
///   CMP3 HYEN Enable
///
/// - **PSEL3** `[13]` (RW)
///
///   CMP3 IN_P Selection
///
/// - **NSEL3** `[12]` (RW)
///
///   CMP3 IN_N Selection
///
/// - **MODE3** `[11]` (RW)
///
///   CMP3 OUT Selection
///
/// - **EN3** `[10]` (RW)
///
///   CMP3 Enable
///
/// - **HYEN2** `[9]` (RW)
///
///   CMP2 HYEN Enable
///
/// - **PSEL2** `[8]` (RW)
///
///   CMP2 IN_P Selection
///
/// - **NSEL2** `[7]` (RW)
///
///   CMP2 IN_N Selection
///
/// - **MODE2** `[6]` (RW)
///
///   CMP2 OUT Selection
///
/// - **EN2** `[5]` (RW)
///
///   CMP2 Enable
///
/// - **HYEN1** `[4]` (RW)
///
///   CMP1 HYEN Enable
///
/// - **PSEL1** `[3]` (RW)
///
///   CMP1 IN_P Selection
///
/// - **NSEL1** `[2]` (RW)
///
///   CMP1 IN_N Selection
///
/// - **MODE1** `[1]` (RW)
///
///   CMP1 OUT Selection
///
/// - **EN1** `[0]` (RW)
///
///   CMP1 Enable
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

    /// **CMP_LOCK**
    ///
    /// CMP_LOCK
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CMP_LOCK_OFFSET: u32 = 31;
    pub const CMP_LOCK_MASK: u32 = 0x80000000;
    pub const CMP_LOCK: u32 = 1 << 31;

    /// **HYEN3**
    ///
    /// CMP3 HYEN Enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const HYEN3_OFFSET: u32 = 14;
    pub const HYEN3_MASK: u32 = 0x00004000;
    pub const HYEN3: u32 = 1 << 14;

    /// **PSEL3**
    ///
    /// CMP3 IN_P Selection
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const PSEL3_OFFSET: u32 = 13;
    pub const PSEL3_MASK: u32 = 0x00002000;
    pub const PSEL3: u32 = 1 << 13;

    /// **NSEL3**
    ///
    /// CMP3 IN_N Selection
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const NSEL3_OFFSET: u32 = 12;
    pub const NSEL3_MASK: u32 = 0x00001000;
    pub const NSEL3: u32 = 1 << 12;

    /// **MODE3**
    ///
    /// CMP3 OUT Selection
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MODE3_OFFSET: u32 = 11;
    pub const MODE3_MASK: u32 = 0x00000800;
    pub const MODE3: u32 = 1 << 11;

    /// **EN3**
    ///
    /// CMP3 Enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EN3_OFFSET: u32 = 10;
    pub const EN3_MASK: u32 = 0x00000400;
    pub const EN3: u32 = 1 << 10;

    /// **HYEN2**
    ///
    /// CMP2 HYEN Enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const HYEN2_OFFSET: u32 = 9;
    pub const HYEN2_MASK: u32 = 0x00000200;
    pub const HYEN2: u32 = 1 << 9;

    /// **PSEL2**
    ///
    /// CMP2 IN_P Selection
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PSEL2_OFFSET: u32 = 8;
    pub const PSEL2_MASK: u32 = 0x00000100;
    pub const PSEL2: u32 = 1 << 8;

    /// **NSEL2**
    ///
    /// CMP2 IN_N Selection
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const NSEL2_OFFSET: u32 = 7;
    pub const NSEL2_MASK: u32 = 0x00000080;
    pub const NSEL2: u32 = 1 << 7;

    /// **MODE2**
    ///
    /// CMP2 OUT Selection
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MODE2_OFFSET: u32 = 6;
    pub const MODE2_MASK: u32 = 0x00000040;
    pub const MODE2: u32 = 1 << 6;

    /// **EN2**
    ///
    /// CMP2 Enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EN2_OFFSET: u32 = 5;
    pub const EN2_MASK: u32 = 0x00000020;
    pub const EN2: u32 = 1 << 5;

    /// **HYEN1**
    ///
    /// CMP1 HYEN Enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const HYEN1_OFFSET: u32 = 4;
    pub const HYEN1_MASK: u32 = 0x00000010;
    pub const HYEN1: u32 = 1 << 4;

    /// **PSEL1**
    ///
    /// CMP1 IN_P Selection
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PSEL1_OFFSET: u32 = 3;
    pub const PSEL1_MASK: u32 = 0x00000008;
    pub const PSEL1: u32 = 1 << 3;

    /// **NSEL1**
    ///
    /// CMP1 IN_N Selection
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const NSEL1_OFFSET: u32 = 2;
    pub const NSEL1_MASK: u32 = 0x00000004;
    pub const NSEL1: u32 = 1 << 2;

    /// **MODE1**
    ///
    /// CMP1 OUT Selection
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MODE1_OFFSET: u32 = 1;
    pub const MODE1_MASK: u32 = 0x00000002;
    pub const MODE1: u32 = 1 << 1;

    /// **EN1**
    ///
    /// CMP1 Enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN1_OFFSET: u32 = 0;
    pub const EN1_MASK: u32 = 0x00000001;
    pub const EN1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CMP_LOCK** `[31]` (RW)
    ///
    /// CMP_LOCK
    #[inline(always)]
    pub fn cmp_lock(&self) -> bool {
        (self.value.read_bits() & Self::CMP_LOCK_MASK) != 0
    }

    /// Set CMP_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmp_lock(&self) {
        self.value.modify_bits(|r| r | Self::CMP_LOCK_MASK);
    }

    /// Clear CMP_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmp_lock(&self) {
        self.value.modify_bits(|r| r & !Self::CMP_LOCK_MASK);
    }

    /// **HYEN3** `[14]` (RW)
    ///
    /// CMP3 HYEN Enable
    #[inline(always)]
    pub fn hyen3(&self) -> bool {
        (self.value.read_bits() & Self::HYEN3_MASK) != 0
    }

    /// Set HYEN3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hyen3(&self) {
        self.value.modify_bits(|r| r | Self::HYEN3_MASK);
    }

    /// Clear HYEN3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hyen3(&self) {
        self.value.modify_bits(|r| r & !Self::HYEN3_MASK);
    }

    /// **PSEL3** `[13]` (RW)
    ///
    /// CMP3 IN_P Selection
    #[inline(always)]
    pub fn psel3(&self) -> bool {
        (self.value.read_bits() & Self::PSEL3_MASK) != 0
    }

    /// Set PSEL3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_psel3(&self) {
        self.value.modify_bits(|r| r | Self::PSEL3_MASK);
    }

    /// Clear PSEL3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_psel3(&self) {
        self.value.modify_bits(|r| r & !Self::PSEL3_MASK);
    }

    /// **NSEL3** `[12]` (RW)
    ///
    /// CMP3 IN_N Selection
    #[inline(always)]
    pub fn nsel3(&self) -> bool {
        (self.value.read_bits() & Self::NSEL3_MASK) != 0
    }

    /// Set NSEL3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nsel3(&self) {
        self.value.modify_bits(|r| r | Self::NSEL3_MASK);
    }

    /// Clear NSEL3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nsel3(&self) {
        self.value.modify_bits(|r| r & !Self::NSEL3_MASK);
    }

    /// **MODE3** `[11]` (RW)
    ///
    /// CMP3 OUT Selection
    #[inline(always)]
    pub fn mode3(&self) -> bool {
        (self.value.read_bits() & Self::MODE3_MASK) != 0
    }

    /// Set MODE3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode3(&self) {
        self.value.modify_bits(|r| r | Self::MODE3_MASK);
    }

    /// Clear MODE3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode3(&self) {
        self.value.modify_bits(|r| r & !Self::MODE3_MASK);
    }

    /// **EN3** `[10]` (RW)
    ///
    /// CMP3 Enable
    #[inline(always)]
    pub fn en3(&self) -> bool {
        (self.value.read_bits() & Self::EN3_MASK) != 0
    }

    /// Set EN3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en3(&self) {
        self.value.modify_bits(|r| r | Self::EN3_MASK);
    }

    /// Clear EN3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en3(&self) {
        self.value.modify_bits(|r| r & !Self::EN3_MASK);
    }

    /// **HYEN2** `[9]` (RW)
    ///
    /// CMP2 HYEN Enable
    #[inline(always)]
    pub fn hyen2(&self) -> bool {
        (self.value.read_bits() & Self::HYEN2_MASK) != 0
    }

    /// Set HYEN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hyen2(&self) {
        self.value.modify_bits(|r| r | Self::HYEN2_MASK);
    }

    /// Clear HYEN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hyen2(&self) {
        self.value.modify_bits(|r| r & !Self::HYEN2_MASK);
    }

    /// **PSEL2** `[8]` (RW)
    ///
    /// CMP2 IN_P Selection
    #[inline(always)]
    pub fn psel2(&self) -> bool {
        (self.value.read_bits() & Self::PSEL2_MASK) != 0
    }

    /// Set PSEL2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_psel2(&self) {
        self.value.modify_bits(|r| r | Self::PSEL2_MASK);
    }

    /// Clear PSEL2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_psel2(&self) {
        self.value.modify_bits(|r| r & !Self::PSEL2_MASK);
    }

    /// **NSEL2** `[7]` (RW)
    ///
    /// CMP2 IN_N Selection
    #[inline(always)]
    pub fn nsel2(&self) -> bool {
        (self.value.read_bits() & Self::NSEL2_MASK) != 0
    }

    /// Set NSEL2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nsel2(&self) {
        self.value.modify_bits(|r| r | Self::NSEL2_MASK);
    }

    /// Clear NSEL2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nsel2(&self) {
        self.value.modify_bits(|r| r & !Self::NSEL2_MASK);
    }

    /// **MODE2** `[6]` (RW)
    ///
    /// CMP2 OUT Selection
    #[inline(always)]
    pub fn mode2(&self) -> bool {
        (self.value.read_bits() & Self::MODE2_MASK) != 0
    }

    /// Set MODE2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode2(&self) {
        self.value.modify_bits(|r| r | Self::MODE2_MASK);
    }

    /// Clear MODE2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode2(&self) {
        self.value.modify_bits(|r| r & !Self::MODE2_MASK);
    }

    /// **EN2** `[5]` (RW)
    ///
    /// CMP2 Enable
    #[inline(always)]
    pub fn en2(&self) -> bool {
        (self.value.read_bits() & Self::EN2_MASK) != 0
    }

    /// Set EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en2(&self) {
        self.value.modify_bits(|r| r | Self::EN2_MASK);
    }

    /// Clear EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en2(&self) {
        self.value.modify_bits(|r| r & !Self::EN2_MASK);
    }

    /// **HYEN1** `[4]` (RW)
    ///
    /// CMP1 HYEN Enable
    #[inline(always)]
    pub fn hyen1(&self) -> bool {
        (self.value.read_bits() & Self::HYEN1_MASK) != 0
    }

    /// Set HYEN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hyen1(&self) {
        self.value.modify_bits(|r| r | Self::HYEN1_MASK);
    }

    /// Clear HYEN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hyen1(&self) {
        self.value.modify_bits(|r| r & !Self::HYEN1_MASK);
    }

    /// **PSEL1** `[3]` (RW)
    ///
    /// CMP1 IN_P Selection
    #[inline(always)]
    pub fn psel1(&self) -> bool {
        (self.value.read_bits() & Self::PSEL1_MASK) != 0
    }

    /// Set PSEL1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_psel1(&self) {
        self.value.modify_bits(|r| r | Self::PSEL1_MASK);
    }

    /// Clear PSEL1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_psel1(&self) {
        self.value.modify_bits(|r| r & !Self::PSEL1_MASK);
    }

    /// **NSEL1** `[2]` (RW)
    ///
    /// CMP1 IN_N Selection
    #[inline(always)]
    pub fn nsel1(&self) -> bool {
        (self.value.read_bits() & Self::NSEL1_MASK) != 0
    }

    /// Set NSEL1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nsel1(&self) {
        self.value.modify_bits(|r| r | Self::NSEL1_MASK);
    }

    /// Clear NSEL1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nsel1(&self) {
        self.value.modify_bits(|r| r & !Self::NSEL1_MASK);
    }

    /// **MODE1** `[1]` (RW)
    ///
    /// CMP1 OUT Selection
    #[inline(always)]
    pub fn mode1(&self) -> bool {
        (self.value.read_bits() & Self::MODE1_MASK) != 0
    }

    /// Set MODE1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode1(&self) {
        self.value.modify_bits(|r| r | Self::MODE1_MASK);
    }

    /// Clear MODE1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode1(&self) {
        self.value.modify_bits(|r| r & !Self::MODE1_MASK);
    }

    /// **EN1** `[0]` (RW)
    ///
    /// CMP1 Enable
    #[inline(always)]
    pub fn en1(&self) -> bool {
        (self.value.read_bits() & Self::EN1_MASK) != 0
    }

    /// Set EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en1(&self) {
        self.value.modify_bits(|r| r | Self::EN1_MASK);
    }

    /// Clear EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en1(&self) {
        self.value.modify_bits(|r| r & !Self::EN1_MASK);
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
        Self { bits: 0x80000000 }
    }

    /// Read **CMP_LOCK** `[31:31]`
    ///
    /// CMP_LOCK
    #[inline(always)]
    pub const fn cmp_lock(&self) -> bool {
        (self.bits & Ctlr2::CMP_LOCK_MASK) != 0
    }

    /// Read **HYEN3** `[14:14]`
    ///
    /// CMP3 HYEN Enable
    #[inline(always)]
    pub const fn hyen3(&self) -> bool {
        (self.bits & Ctlr2::HYEN3_MASK) != 0
    }

    /// Read **PSEL3** `[13:13]`
    ///
    /// CMP3 IN_P Selection
    #[inline(always)]
    pub const fn psel3(&self) -> bool {
        (self.bits & Ctlr2::PSEL3_MASK) != 0
    }

    /// Read **NSEL3** `[12:12]`
    ///
    /// CMP3 IN_N Selection
    #[inline(always)]
    pub const fn nsel3(&self) -> bool {
        (self.bits & Ctlr2::NSEL3_MASK) != 0
    }

    /// Read **MODE3** `[11:11]`
    ///
    /// CMP3 OUT Selection
    #[inline(always)]
    pub const fn mode3(&self) -> bool {
        (self.bits & Ctlr2::MODE3_MASK) != 0
    }

    /// Read **EN3** `[10:10]`
    ///
    /// CMP3 Enable
    #[inline(always)]
    pub const fn en3(&self) -> bool {
        (self.bits & Ctlr2::EN3_MASK) != 0
    }

    /// Read **HYEN2** `[9:9]`
    ///
    /// CMP2 HYEN Enable
    #[inline(always)]
    pub const fn hyen2(&self) -> bool {
        (self.bits & Ctlr2::HYEN2_MASK) != 0
    }

    /// Read **PSEL2** `[8:8]`
    ///
    /// CMP2 IN_P Selection
    #[inline(always)]
    pub const fn psel2(&self) -> bool {
        (self.bits & Ctlr2::PSEL2_MASK) != 0
    }

    /// Read **NSEL2** `[7:7]`
    ///
    /// CMP2 IN_N Selection
    #[inline(always)]
    pub const fn nsel2(&self) -> bool {
        (self.bits & Ctlr2::NSEL2_MASK) != 0
    }

    /// Read **MODE2** `[6:6]`
    ///
    /// CMP2 OUT Selection
    #[inline(always)]
    pub const fn mode2(&self) -> bool {
        (self.bits & Ctlr2::MODE2_MASK) != 0
    }

    /// Read **EN2** `[5:5]`
    ///
    /// CMP2 Enable
    #[inline(always)]
    pub const fn en2(&self) -> bool {
        (self.bits & Ctlr2::EN2_MASK) != 0
    }

    /// Read **HYEN1** `[4:4]`
    ///
    /// CMP1 HYEN Enable
    #[inline(always)]
    pub const fn hyen1(&self) -> bool {
        (self.bits & Ctlr2::HYEN1_MASK) != 0
    }

    /// Read **PSEL1** `[3:3]`
    ///
    /// CMP1 IN_P Selection
    #[inline(always)]
    pub const fn psel1(&self) -> bool {
        (self.bits & Ctlr2::PSEL1_MASK) != 0
    }

    /// Read **NSEL1** `[2:2]`
    ///
    /// CMP1 IN_N Selection
    #[inline(always)]
    pub const fn nsel1(&self) -> bool {
        (self.bits & Ctlr2::NSEL1_MASK) != 0
    }

    /// Read **MODE1** `[1:1]`
    ///
    /// CMP1 OUT Selection
    #[inline(always)]
    pub const fn mode1(&self) -> bool {
        (self.bits & Ctlr2::MODE1_MASK) != 0
    }

    /// Read **EN1** `[0:0]`
    ///
    /// CMP1 Enable
    #[inline(always)]
    pub const fn en1(&self) -> bool {
        (self.bits & Ctlr2::EN1_MASK) != 0
    }

    /// Set **CMP_LOCK** `[31:31]`
    ///
    /// CMP_LOCK
    #[inline(always)]
    pub const fn with_cmp_lock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CMP_LOCK_MASK
            } else {
                self.bits & !Ctlr2::CMP_LOCK_MASK
            },
        }
    }

    /// Set **HYEN3** `[14:14]`
    ///
    /// CMP3 HYEN Enable
    #[inline(always)]
    pub const fn with_hyen3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::HYEN3_MASK
            } else {
                self.bits & !Ctlr2::HYEN3_MASK
            },
        }
    }

    /// Set **PSEL3** `[13:13]`
    ///
    /// CMP3 IN_P Selection
    #[inline(always)]
    pub const fn with_psel3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::PSEL3_MASK
            } else {
                self.bits & !Ctlr2::PSEL3_MASK
            },
        }
    }

    /// Set **NSEL3** `[12:12]`
    ///
    /// CMP3 IN_N Selection
    #[inline(always)]
    pub const fn with_nsel3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::NSEL3_MASK
            } else {
                self.bits & !Ctlr2::NSEL3_MASK
            },
        }
    }

    /// Set **MODE3** `[11:11]`
    ///
    /// CMP3 OUT Selection
    #[inline(always)]
    pub const fn with_mode3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::MODE3_MASK
            } else {
                self.bits & !Ctlr2::MODE3_MASK
            },
        }
    }

    /// Set **EN3** `[10:10]`
    ///
    /// CMP3 Enable
    #[inline(always)]
    pub const fn with_en3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::EN3_MASK
            } else {
                self.bits & !Ctlr2::EN3_MASK
            },
        }
    }

    /// Set **HYEN2** `[9:9]`
    ///
    /// CMP2 HYEN Enable
    #[inline(always)]
    pub const fn with_hyen2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::HYEN2_MASK
            } else {
                self.bits & !Ctlr2::HYEN2_MASK
            },
        }
    }

    /// Set **PSEL2** `[8:8]`
    ///
    /// CMP2 IN_P Selection
    #[inline(always)]
    pub const fn with_psel2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::PSEL2_MASK
            } else {
                self.bits & !Ctlr2::PSEL2_MASK
            },
        }
    }

    /// Set **NSEL2** `[7:7]`
    ///
    /// CMP2 IN_N Selection
    #[inline(always)]
    pub const fn with_nsel2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::NSEL2_MASK
            } else {
                self.bits & !Ctlr2::NSEL2_MASK
            },
        }
    }

    /// Set **MODE2** `[6:6]`
    ///
    /// CMP2 OUT Selection
    #[inline(always)]
    pub const fn with_mode2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::MODE2_MASK
            } else {
                self.bits & !Ctlr2::MODE2_MASK
            },
        }
    }

    /// Set **EN2** `[5:5]`
    ///
    /// CMP2 Enable
    #[inline(always)]
    pub const fn with_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::EN2_MASK
            } else {
                self.bits & !Ctlr2::EN2_MASK
            },
        }
    }

    /// Set **HYEN1** `[4:4]`
    ///
    /// CMP1 HYEN Enable
    #[inline(always)]
    pub const fn with_hyen1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::HYEN1_MASK
            } else {
                self.bits & !Ctlr2::HYEN1_MASK
            },
        }
    }

    /// Set **PSEL1** `[3:3]`
    ///
    /// CMP1 IN_P Selection
    #[inline(always)]
    pub const fn with_psel1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::PSEL1_MASK
            } else {
                self.bits & !Ctlr2::PSEL1_MASK
            },
        }
    }

    /// Set **NSEL1** `[2:2]`
    ///
    /// CMP1 IN_N Selection
    #[inline(always)]
    pub const fn with_nsel1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::NSEL1_MASK
            } else {
                self.bits & !Ctlr2::NSEL1_MASK
            },
        }
    }

    /// Set **MODE1** `[1:1]`
    ///
    /// CMP1 OUT Selection
    #[inline(always)]
    pub const fn with_mode1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::MODE1_MASK
            } else {
                self.bits & !Ctlr2::MODE1_MASK
            },
        }
    }

    /// Set **EN1** `[0:0]`
    ///
    /// CMP1 Enable
    #[inline(always)]
    pub const fn with_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::EN1_MASK
            } else {
                self.bits & !Ctlr2::EN1_MASK
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

/// OPA_KEY Register
///
/// OPA unlock key register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OPA_KEY** `[31:0]` (RW)
///
///   OPA_KEY value
#[repr(transparent)]
pub struct OpaKey {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OpaKey {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OpaKey {
    // ======== Field Constants ========

    /// **OPA_KEY**
    ///
    /// OPA_KEY value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OPA_KEY_OFFSET: u32 = 0;
    pub const OPA_KEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OPA_KEY** `[31:0]` (RW)
    ///
    /// OPA_KEY value
    #[inline(always)]
    pub fn opa_key(&self) -> u32 {
        (self.value.read_bits() & Self::OPA_KEY_MASK) >> Self::OPA_KEY_OFFSET
    }

    /// Write **OPA_KEY** `[31:0]` (RW)
    ///
    /// OPA_KEY value
    #[inline(always)]
    pub fn write_opa_key(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OPA_KEY_MASK) | ((value << Self::OPA_KEY_OFFSET) & Self::OPA_KEY_MASK)
        });
    }
}

/// OPA_KEY value type for batch read/modify/write operations
///
/// Use `opakey.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `opakey.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OpaKeyVal {
    bits: u32,
}

impl OpaKeyVal {
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

    /// Read **OPA_KEY** `[31:0]`
    ///
    /// OPA_KEY value
    #[inline(always)]
    pub const fn opa_key(&self) -> u32 {
        (self.bits & OpaKey::OPA_KEY_MASK) >> OpaKey::OPA_KEY_OFFSET
    }

    /// Set **OPA_KEY** `[31:0]`
    ///
    /// OPA_KEY value
    #[inline(always)]
    pub const fn with_opa_key(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OpaKey::OPA_KEY_MASK)
                | ((value << OpaKey::OPA_KEY_OFFSET) & OpaKey::OPA_KEY_MASK),
        }
    }
}

impl Default for OpaKeyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OpaKey {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OpaKeyVal {
        OpaKeyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OpaKeyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OpaKeyVal) -> OpaKeyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CMP_KEY Register
///
/// CMP unlock key register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CMP_KEY** `[31:0]` (RW)
///
///   CMP_KEY value
#[repr(transparent)]
pub struct CmpKey {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CmpKey {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CmpKey {
    // ======== Field Constants ========

    /// **CMP_KEY**
    ///
    /// CMP_KEY value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CMP_KEY_OFFSET: u32 = 0;
    pub const CMP_KEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CMP_KEY** `[31:0]` (RW)
    ///
    /// CMP_KEY value
    #[inline(always)]
    pub fn cmp_key(&self) -> u32 {
        (self.value.read_bits() & Self::CMP_KEY_MASK) >> Self::CMP_KEY_OFFSET
    }

    /// Write **CMP_KEY** `[31:0]` (RW)
    ///
    /// CMP_KEY value
    #[inline(always)]
    pub fn write_cmp_key(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CMP_KEY_MASK) | ((value << Self::CMP_KEY_OFFSET) & Self::CMP_KEY_MASK)
        });
    }
}

/// CMP_KEY value type for batch read/modify/write operations
///
/// Use `cmpkey.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmpkey.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmpKeyVal {
    bits: u32,
}

impl CmpKeyVal {
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

    /// Read **CMP_KEY** `[31:0]`
    ///
    /// CMP_KEY value
    #[inline(always)]
    pub const fn cmp_key(&self) -> u32 {
        (self.bits & CmpKey::CMP_KEY_MASK) >> CmpKey::CMP_KEY_OFFSET
    }

    /// Set **CMP_KEY** `[31:0]`
    ///
    /// CMP_KEY value
    #[inline(always)]
    pub const fn with_cmp_key(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CmpKey::CMP_KEY_MASK)
                | ((value << CmpKey::CMP_KEY_OFFSET) & CmpKey::CMP_KEY_MASK),
        }
    }
}

impl Default for CmpKeyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CmpKey {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmpKeyVal {
        CmpKeyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmpKeyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CmpKeyVal) -> CmpKeyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// POLL_KEY Register
///
/// POLL unlock key register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **POLL_KEY** `[31:0]` (RW)
///
///   POLL_KEY value
#[repr(transparent)]
pub struct PollKey {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PollKey {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PollKey {
    // ======== Field Constants ========

    /// **POLL_KEY**
    ///
    /// POLL_KEY value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const POLL_KEY_OFFSET: u32 = 0;
    pub const POLL_KEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **POLL_KEY** `[31:0]` (RW)
    ///
    /// POLL_KEY value
    #[inline(always)]
    pub fn poll_key(&self) -> u32 {
        (self.value.read_bits() & Self::POLL_KEY_MASK) >> Self::POLL_KEY_OFFSET
    }

    /// Write **POLL_KEY** `[31:0]` (RW)
    ///
    /// POLL_KEY value
    #[inline(always)]
    pub fn write_poll_key(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::POLL_KEY_MASK) | ((value << Self::POLL_KEY_OFFSET) & Self::POLL_KEY_MASK)
        });
    }
}

/// POLL_KEY value type for batch read/modify/write operations
///
/// Use `pollkey.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pollkey.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PollKeyVal {
    bits: u32,
}

impl PollKeyVal {
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

    /// Read **POLL_KEY** `[31:0]`
    ///
    /// POLL_KEY value
    #[inline(always)]
    pub const fn poll_key(&self) -> u32 {
        (self.bits & PollKey::POLL_KEY_MASK) >> PollKey::POLL_KEY_OFFSET
    }

    /// Set **POLL_KEY** `[31:0]`
    ///
    /// POLL_KEY value
    #[inline(always)]
    pub const fn with_poll_key(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PollKey::POLL_KEY_MASK)
                | ((value << PollKey::POLL_KEY_OFFSET) & PollKey::POLL_KEY_MASK),
        }
    }
}

impl Default for PollKeyVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PollKey {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PollKeyVal {
        PollKeyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PollKeyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PollKeyVal) -> PollKeyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// OPA Configuration register
    pub cfgr1: Cfgr1,
    /// 0x0002
    /// OPA Configuration register
    pub cfgr2: Cfgr2,
    /// 0x0004
    /// OPA Control1 register
    pub ctlr1: Ctlr1,
    /// 0x0008
    /// OPA Control2 register
    pub ctlr2: Ctlr2,
    /// 0x000c
    /// OPA unlock key register
    pub opa_key: OpaKey,
    /// 0x0010
    /// CMP unlock key register
    pub cmp_key: CmpKey,
    /// 0x0014
    /// POLL unlock key register
    pub poll_key: PollKey,
}
