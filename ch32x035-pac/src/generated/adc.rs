//! ADC
//! Analog to digital converter

#![allow(dead_code)]

pub const BASE: u32 = 0x40012400;

/// STATR Register
///
/// status register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **STRT** `[4]` (RW)
///
///   Regular channel start flag
///
/// - **JSTRT** `[3]` (RW)
///
///   Injected channel start
///   flag
///
/// - **JEOC** `[2]` (RW)
///
///   Injected channel end of
///   conversion
///
/// - **EOC** `[1]` (RW)
///
///   Regular channel end of
///   conversion
///
/// - **AWD** `[0]` (RW)
///
///   Analog watchdog flag
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

    /// **STRT**
    ///
    /// Regular channel start flag
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const STRT_OFFSET: u32 = 4;
    pub const STRT_MASK: u32 = 0x00000010;
    pub const STRT: u32 = 1 << 4;

    /// **JSTRT**
    ///
    /// Injected channel start
    /// flag
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const JSTRT_OFFSET: u32 = 3;
    pub const JSTRT_MASK: u32 = 0x00000008;
    pub const JSTRT: u32 = 1 << 3;

    /// **JEOC**
    ///
    /// Injected channel end of
    /// conversion
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const JEOC_OFFSET: u32 = 2;
    pub const JEOC_MASK: u32 = 0x00000004;
    pub const JEOC: u32 = 1 << 2;

    /// **EOC**
    ///
    /// Regular channel end of
    /// conversion
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EOC_OFFSET: u32 = 1;
    pub const EOC_MASK: u32 = 0x00000002;
    pub const EOC: u32 = 1 << 1;

    /// **AWD**
    ///
    /// Analog watchdog flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const AWD_OFFSET: u32 = 0;
    pub const AWD_MASK: u32 = 0x00000001;
    pub const AWD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **STRT** `[4]` (RW)
    ///
    /// Regular channel start flag
    #[inline(always)]
    pub fn strt(&self) -> bool {
        (self.value.read_bits() & Self::STRT_MASK) != 0
    }

    /// Set STRT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_strt(&self) {
        self.value.modify_bits(|r| r | Self::STRT_MASK);
    }

    /// Clear STRT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_strt(&self) {
        self.value.modify_bits(|r| r & !Self::STRT_MASK);
    }

    /// **JSTRT** `[3]` (RW)
    ///
    /// Injected channel start
    /// flag
    #[inline(always)]
    pub fn jstrt(&self) -> bool {
        (self.value.read_bits() & Self::JSTRT_MASK) != 0
    }

    /// Set JSTRT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jstrt(&self) {
        self.value.modify_bits(|r| r | Self::JSTRT_MASK);
    }

    /// Clear JSTRT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jstrt(&self) {
        self.value.modify_bits(|r| r & !Self::JSTRT_MASK);
    }

    /// **JEOC** `[2]` (RW)
    ///
    /// Injected channel end of
    /// conversion
    #[inline(always)]
    pub fn jeoc(&self) -> bool {
        (self.value.read_bits() & Self::JEOC_MASK) != 0
    }

    /// Set JEOC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jeoc(&self) {
        self.value.modify_bits(|r| r | Self::JEOC_MASK);
    }

    /// Clear JEOC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jeoc(&self) {
        self.value.modify_bits(|r| r & !Self::JEOC_MASK);
    }

    /// **EOC** `[1]` (RW)
    ///
    /// Regular channel end of
    /// conversion
    #[inline(always)]
    pub fn eoc(&self) -> bool {
        (self.value.read_bits() & Self::EOC_MASK) != 0
    }

    /// Set EOC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eoc(&self) {
        self.value.modify_bits(|r| r | Self::EOC_MASK);
    }

    /// Clear EOC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eoc(&self) {
        self.value.modify_bits(|r| r & !Self::EOC_MASK);
    }

    /// **AWD** `[0]` (RW)
    ///
    /// Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> bool {
        (self.value.read_bits() & Self::AWD_MASK) != 0
    }

    /// Set AWD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_awd(&self) {
        self.value.modify_bits(|r| r | Self::AWD_MASK);
    }

    /// Clear AWD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_awd(&self) {
        self.value.modify_bits(|r| r & !Self::AWD_MASK);
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

    /// Read **STRT** `[4:4]`
    ///
    /// Regular channel start flag
    #[inline(always)]
    pub const fn strt(&self) -> bool {
        (self.bits & Statr::STRT_MASK) != 0
    }

    /// Read **JSTRT** `[3:3]`
    ///
    /// Injected channel start
    /// flag
    #[inline(always)]
    pub const fn jstrt(&self) -> bool {
        (self.bits & Statr::JSTRT_MASK) != 0
    }

    /// Read **JEOC** `[2:2]`
    ///
    /// Injected channel end of
    /// conversion
    #[inline(always)]
    pub const fn jeoc(&self) -> bool {
        (self.bits & Statr::JEOC_MASK) != 0
    }

    /// Read **EOC** `[1:1]`
    ///
    /// Regular channel end of
    /// conversion
    #[inline(always)]
    pub const fn eoc(&self) -> bool {
        (self.bits & Statr::EOC_MASK) != 0
    }

    /// Read **AWD** `[0:0]`
    ///
    /// Analog watchdog flag
    #[inline(always)]
    pub const fn awd(&self) -> bool {
        (self.bits & Statr::AWD_MASK) != 0
    }

    /// Set **STRT** `[4:4]`
    ///
    /// Regular channel start flag
    #[inline(always)]
    pub const fn with_strt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::STRT_MASK
            } else {
                self.bits & !Statr::STRT_MASK
            },
        }
    }

    /// Set **JSTRT** `[3:3]`
    ///
    /// Injected channel start
    /// flag
    #[inline(always)]
    pub const fn with_jstrt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::JSTRT_MASK
            } else {
                self.bits & !Statr::JSTRT_MASK
            },
        }
    }

    /// Set **JEOC** `[2:2]`
    ///
    /// Injected channel end of
    /// conversion
    #[inline(always)]
    pub const fn with_jeoc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::JEOC_MASK
            } else {
                self.bits & !Statr::JEOC_MASK
            },
        }
    }

    /// Set **EOC** `[1:1]`
    ///
    /// Regular channel end of
    /// conversion
    #[inline(always)]
    pub const fn with_eoc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::EOC_MASK
            } else {
                self.bits & !Statr::EOC_MASK
            },
        }
    }

    /// Set **AWD** `[0:0]`
    ///
    /// Analog watchdog flag
    #[inline(always)]
    pub const fn with_awd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::AWD_MASK
            } else {
                self.bits & !Statr::AWD_MASK
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

/// CTLR1 Register
///
/// control register 1
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TKEYEN** `[24]` (RW)
///
///   TKEY enable, including TKEY_F and
///   TKEY_V
///
/// - **AWDEN** `[23]` (RW)
///
///   Analog watchdog enable on regular
///   channels
///
/// - **JAWDEN** `[22]` (RW)
///
///   Analog watchdog enable on injected
///   channels
///
/// - **DISCNUM** `[15:13]` (RW)
///
///   Discontinuous mode channel
///   count
///
/// - **JDISCEN** `[12]` (RW)
///
///   Discontinuous mode on injected
///   channels
///
/// - **DISCEN** `[11]` (RW)
///
///   Discontinuous mode on regular
///   channels
///
/// - **JAUTO** `[10]` (RW)
///
///   Automatic injected group
///   conversion
///
/// - **AWDSGL** `[9]` (RW)
///
///   Enable the watchdog on a single channel
///   in scan mode
///
/// - **SCAN** `[8]` (RW)
///
///   Scan mode enable
///
/// - **JEOCIE** `[7]` (RW)
///
///   Interrupt enable for injected
///   channels
///
/// - **AWDIE** `[6]` (RW)
///
///   Analog watchdog interrupt
///   enable
///
/// - **EOCIE** `[5]` (RW)
///
///   Interrupt enable for EOC
///
/// - **AWDCH** `[3:0]` (RW)
///
///   Analog watchdog channel select
///   bits
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

    /// **TKEYEN**
    ///
    /// TKEY enable, including TKEY_F and
    /// TKEY_V
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TKEYEN_OFFSET: u32 = 24;
    pub const TKEYEN_MASK: u32 = 0x01000000;
    pub const TKEYEN: u32 = 1 << 24;

    /// **AWDEN**
    ///
    /// Analog watchdog enable on regular
    /// channels
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const AWDEN_OFFSET: u32 = 23;
    pub const AWDEN_MASK: u32 = 0x00800000;
    pub const AWDEN: u32 = 1 << 23;

    /// **JAWDEN**
    ///
    /// Analog watchdog enable on injected
    /// channels
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const JAWDEN_OFFSET: u32 = 22;
    pub const JAWDEN_MASK: u32 = 0x00400000;
    pub const JAWDEN: u32 = 1 << 22;

    /// **DISCNUM**
    ///
    /// Discontinuous mode channel
    /// count
    ///
    /// Bits `[15:13]`, mask `0x0000e000`
    pub const DISCNUM_OFFSET: u32 = 13;
    pub const DISCNUM_MASK: u32 = 0x0000e000;

    /// **JDISCEN**
    ///
    /// Discontinuous mode on injected
    /// channels
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const JDISCEN_OFFSET: u32 = 12;
    pub const JDISCEN_MASK: u32 = 0x00001000;
    pub const JDISCEN: u32 = 1 << 12;

    /// **DISCEN**
    ///
    /// Discontinuous mode on regular
    /// channels
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DISCEN_OFFSET: u32 = 11;
    pub const DISCEN_MASK: u32 = 0x00000800;
    pub const DISCEN: u32 = 1 << 11;

    /// **JAUTO**
    ///
    /// Automatic injected group
    /// conversion
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const JAUTO_OFFSET: u32 = 10;
    pub const JAUTO_MASK: u32 = 0x00000400;
    pub const JAUTO: u32 = 1 << 10;

    /// **AWDSGL**
    ///
    /// Enable the watchdog on a single channel
    /// in scan mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const AWDSGL_OFFSET: u32 = 9;
    pub const AWDSGL_MASK: u32 = 0x00000200;
    pub const AWDSGL: u32 = 1 << 9;

    /// **SCAN**
    ///
    /// Scan mode enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SCAN_OFFSET: u32 = 8;
    pub const SCAN_MASK: u32 = 0x00000100;
    pub const SCAN: u32 = 1 << 8;

    /// **JEOCIE**
    ///
    /// Interrupt enable for injected
    /// channels
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const JEOCIE_OFFSET: u32 = 7;
    pub const JEOCIE_MASK: u32 = 0x00000080;
    pub const JEOCIE: u32 = 1 << 7;

    /// **AWDIE**
    ///
    /// Analog watchdog interrupt
    /// enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const AWDIE_OFFSET: u32 = 6;
    pub const AWDIE_MASK: u32 = 0x00000040;
    pub const AWDIE: u32 = 1 << 6;

    /// **EOCIE**
    ///
    /// Interrupt enable for EOC
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EOCIE_OFFSET: u32 = 5;
    pub const EOCIE_MASK: u32 = 0x00000020;
    pub const EOCIE: u32 = 1 << 5;

    /// **AWDCH**
    ///
    /// Analog watchdog channel select
    /// bits
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const AWDCH_OFFSET: u32 = 0;
    pub const AWDCH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TKEYEN** `[24]` (RW)
    ///
    /// TKEY enable, including TKEY_F and
    /// TKEY_V
    #[inline(always)]
    pub fn tkeyen(&self) -> bool {
        (self.value.read_bits() & Self::TKEYEN_MASK) != 0
    }

    /// Set TKEYEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tkeyen(&self) {
        self.value.modify_bits(|r| r | Self::TKEYEN_MASK);
    }

    /// Clear TKEYEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tkeyen(&self) {
        self.value.modify_bits(|r| r & !Self::TKEYEN_MASK);
    }

    /// **AWDEN** `[23]` (RW)
    ///
    /// Analog watchdog enable on regular
    /// channels
    #[inline(always)]
    pub fn awden(&self) -> bool {
        (self.value.read_bits() & Self::AWDEN_MASK) != 0
    }

    /// Set AWDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_awden(&self) {
        self.value.modify_bits(|r| r | Self::AWDEN_MASK);
    }

    /// Clear AWDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_awden(&self) {
        self.value.modify_bits(|r| r & !Self::AWDEN_MASK);
    }

    /// **JAWDEN** `[22]` (RW)
    ///
    /// Analog watchdog enable on injected
    /// channels
    #[inline(always)]
    pub fn jawden(&self) -> bool {
        (self.value.read_bits() & Self::JAWDEN_MASK) != 0
    }

    /// Set JAWDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jawden(&self) {
        self.value.modify_bits(|r| r | Self::JAWDEN_MASK);
    }

    /// Clear JAWDEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jawden(&self) {
        self.value.modify_bits(|r| r & !Self::JAWDEN_MASK);
    }

    /// **DISCNUM** `[15:13]` (RW)
    ///
    /// Discontinuous mode channel
    /// count
    #[inline(always)]
    pub fn discnum(&self) -> u32 {
        (self.value.read_bits() & Self::DISCNUM_MASK) >> Self::DISCNUM_OFFSET
    }

    /// Write **DISCNUM** `[15:13]` (RW)
    ///
    /// Discontinuous mode channel
    /// count
    #[inline(always)]
    pub fn write_discnum(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DISCNUM_MASK) | ((value << Self::DISCNUM_OFFSET) & Self::DISCNUM_MASK)
        });
    }

    /// **JDISCEN** `[12]` (RW)
    ///
    /// Discontinuous mode on injected
    /// channels
    #[inline(always)]
    pub fn jdiscen(&self) -> bool {
        (self.value.read_bits() & Self::JDISCEN_MASK) != 0
    }

    /// Set JDISCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jdiscen(&self) {
        self.value.modify_bits(|r| r | Self::JDISCEN_MASK);
    }

    /// Clear JDISCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jdiscen(&self) {
        self.value.modify_bits(|r| r & !Self::JDISCEN_MASK);
    }

    /// **DISCEN** `[11]` (RW)
    ///
    /// Discontinuous mode on regular
    /// channels
    #[inline(always)]
    pub fn discen(&self) -> bool {
        (self.value.read_bits() & Self::DISCEN_MASK) != 0
    }

    /// Set DISCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_discen(&self) {
        self.value.modify_bits(|r| r | Self::DISCEN_MASK);
    }

    /// Clear DISCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_discen(&self) {
        self.value.modify_bits(|r| r & !Self::DISCEN_MASK);
    }

    /// **JAUTO** `[10]` (RW)
    ///
    /// Automatic injected group
    /// conversion
    #[inline(always)]
    pub fn jauto(&self) -> bool {
        (self.value.read_bits() & Self::JAUTO_MASK) != 0
    }

    /// Set JAUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jauto(&self) {
        self.value.modify_bits(|r| r | Self::JAUTO_MASK);
    }

    /// Clear JAUTO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jauto(&self) {
        self.value.modify_bits(|r| r & !Self::JAUTO_MASK);
    }

    /// **AWDSGL** `[9]` (RW)
    ///
    /// Enable the watchdog on a single channel
    /// in scan mode
    #[inline(always)]
    pub fn awdsgl(&self) -> bool {
        (self.value.read_bits() & Self::AWDSGL_MASK) != 0
    }

    /// Set AWDSGL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_awdsgl(&self) {
        self.value.modify_bits(|r| r | Self::AWDSGL_MASK);
    }

    /// Clear AWDSGL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_awdsgl(&self) {
        self.value.modify_bits(|r| r & !Self::AWDSGL_MASK);
    }

    /// **SCAN** `[8]` (RW)
    ///
    /// Scan mode enable
    #[inline(always)]
    pub fn scan(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_MASK) != 0
    }

    /// Set SCAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan(&self) {
        self.value.modify_bits(|r| r | Self::SCAN_MASK);
    }

    /// Clear SCAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan(&self) {
        self.value.modify_bits(|r| r & !Self::SCAN_MASK);
    }

    /// **JEOCIE** `[7]` (RW)
    ///
    /// Interrupt enable for injected
    /// channels
    #[inline(always)]
    pub fn jeocie(&self) -> bool {
        (self.value.read_bits() & Self::JEOCIE_MASK) != 0
    }

    /// Set JEOCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jeocie(&self) {
        self.value.modify_bits(|r| r | Self::JEOCIE_MASK);
    }

    /// Clear JEOCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jeocie(&self) {
        self.value.modify_bits(|r| r & !Self::JEOCIE_MASK);
    }

    /// **AWDIE** `[6]` (RW)
    ///
    /// Analog watchdog interrupt
    /// enable
    #[inline(always)]
    pub fn awdie(&self) -> bool {
        (self.value.read_bits() & Self::AWDIE_MASK) != 0
    }

    /// Set AWDIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_awdie(&self) {
        self.value.modify_bits(|r| r | Self::AWDIE_MASK);
    }

    /// Clear AWDIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_awdie(&self) {
        self.value.modify_bits(|r| r & !Self::AWDIE_MASK);
    }

    /// **EOCIE** `[5]` (RW)
    ///
    /// Interrupt enable for EOC
    #[inline(always)]
    pub fn eocie(&self) -> bool {
        (self.value.read_bits() & Self::EOCIE_MASK) != 0
    }

    /// Set EOCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eocie(&self) {
        self.value.modify_bits(|r| r | Self::EOCIE_MASK);
    }

    /// Clear EOCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eocie(&self) {
        self.value.modify_bits(|r| r & !Self::EOCIE_MASK);
    }

    /// **AWDCH** `[3:0]` (RW)
    ///
    /// Analog watchdog channel select
    /// bits
    #[inline(always)]
    pub fn awdch(&self) -> u32 {
        (self.value.read_bits() & Self::AWDCH_MASK) >> Self::AWDCH_OFFSET
    }

    /// Write **AWDCH** `[3:0]` (RW)
    ///
    /// Analog watchdog channel select
    /// bits
    #[inline(always)]
    pub fn write_awdch(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AWDCH_MASK) | ((value << Self::AWDCH_OFFSET) & Self::AWDCH_MASK)
        });
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

    /// Read **TKEYEN** `[24:24]`
    ///
    /// TKEY enable, including TKEY_F and
    /// TKEY_V
    #[inline(always)]
    pub const fn tkeyen(&self) -> bool {
        (self.bits & Ctlr1::TKEYEN_MASK) != 0
    }

    /// Read **AWDEN** `[23:23]`
    ///
    /// Analog watchdog enable on regular
    /// channels
    #[inline(always)]
    pub const fn awden(&self) -> bool {
        (self.bits & Ctlr1::AWDEN_MASK) != 0
    }

    /// Read **JAWDEN** `[22:22]`
    ///
    /// Analog watchdog enable on injected
    /// channels
    #[inline(always)]
    pub const fn jawden(&self) -> bool {
        (self.bits & Ctlr1::JAWDEN_MASK) != 0
    }

    /// Read **DISCNUM** `[15:13]`
    ///
    /// Discontinuous mode channel
    /// count
    #[inline(always)]
    pub const fn discnum(&self) -> u32 {
        (self.bits & Ctlr1::DISCNUM_MASK) >> Ctlr1::DISCNUM_OFFSET
    }

    /// Read **JDISCEN** `[12:12]`
    ///
    /// Discontinuous mode on injected
    /// channels
    #[inline(always)]
    pub const fn jdiscen(&self) -> bool {
        (self.bits & Ctlr1::JDISCEN_MASK) != 0
    }

    /// Read **DISCEN** `[11:11]`
    ///
    /// Discontinuous mode on regular
    /// channels
    #[inline(always)]
    pub const fn discen(&self) -> bool {
        (self.bits & Ctlr1::DISCEN_MASK) != 0
    }

    /// Read **JAUTO** `[10:10]`
    ///
    /// Automatic injected group
    /// conversion
    #[inline(always)]
    pub const fn jauto(&self) -> bool {
        (self.bits & Ctlr1::JAUTO_MASK) != 0
    }

    /// Read **AWDSGL** `[9:9]`
    ///
    /// Enable the watchdog on a single channel
    /// in scan mode
    #[inline(always)]
    pub const fn awdsgl(&self) -> bool {
        (self.bits & Ctlr1::AWDSGL_MASK) != 0
    }

    /// Read **SCAN** `[8:8]`
    ///
    /// Scan mode enable
    #[inline(always)]
    pub const fn scan(&self) -> bool {
        (self.bits & Ctlr1::SCAN_MASK) != 0
    }

    /// Read **JEOCIE** `[7:7]`
    ///
    /// Interrupt enable for injected
    /// channels
    #[inline(always)]
    pub const fn jeocie(&self) -> bool {
        (self.bits & Ctlr1::JEOCIE_MASK) != 0
    }

    /// Read **AWDIE** `[6:6]`
    ///
    /// Analog watchdog interrupt
    /// enable
    #[inline(always)]
    pub const fn awdie(&self) -> bool {
        (self.bits & Ctlr1::AWDIE_MASK) != 0
    }

    /// Read **EOCIE** `[5:5]`
    ///
    /// Interrupt enable for EOC
    #[inline(always)]
    pub const fn eocie(&self) -> bool {
        (self.bits & Ctlr1::EOCIE_MASK) != 0
    }

    /// Read **AWDCH** `[3:0]`
    ///
    /// Analog watchdog channel select
    /// bits
    #[inline(always)]
    pub const fn awdch(&self) -> u32 {
        (self.bits & Ctlr1::AWDCH_MASK) >> Ctlr1::AWDCH_OFFSET
    }

    /// Set **TKEYEN** `[24:24]`
    ///
    /// TKEY enable, including TKEY_F and
    /// TKEY_V
    #[inline(always)]
    pub const fn with_tkeyen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::TKEYEN_MASK
            } else {
                self.bits & !Ctlr1::TKEYEN_MASK
            },
        }
    }

    /// Set **AWDEN** `[23:23]`
    ///
    /// Analog watchdog enable on regular
    /// channels
    #[inline(always)]
    pub const fn with_awden(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::AWDEN_MASK
            } else {
                self.bits & !Ctlr1::AWDEN_MASK
            },
        }
    }

    /// Set **JAWDEN** `[22:22]`
    ///
    /// Analog watchdog enable on injected
    /// channels
    #[inline(always)]
    pub const fn with_jawden(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::JAWDEN_MASK
            } else {
                self.bits & !Ctlr1::JAWDEN_MASK
            },
        }
    }

    /// Set **DISCNUM** `[15:13]`
    ///
    /// Discontinuous mode channel
    /// count
    #[inline(always)]
    pub const fn with_discnum(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::DISCNUM_MASK)
                | ((value << Ctlr1::DISCNUM_OFFSET) & Ctlr1::DISCNUM_MASK),
        }
    }

    /// Set **JDISCEN** `[12:12]`
    ///
    /// Discontinuous mode on injected
    /// channels
    #[inline(always)]
    pub const fn with_jdiscen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::JDISCEN_MASK
            } else {
                self.bits & !Ctlr1::JDISCEN_MASK
            },
        }
    }

    /// Set **DISCEN** `[11:11]`
    ///
    /// Discontinuous mode on regular
    /// channels
    #[inline(always)]
    pub const fn with_discen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::DISCEN_MASK
            } else {
                self.bits & !Ctlr1::DISCEN_MASK
            },
        }
    }

    /// Set **JAUTO** `[10:10]`
    ///
    /// Automatic injected group
    /// conversion
    #[inline(always)]
    pub const fn with_jauto(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::JAUTO_MASK
            } else {
                self.bits & !Ctlr1::JAUTO_MASK
            },
        }
    }

    /// Set **AWDSGL** `[9:9]`
    ///
    /// Enable the watchdog on a single channel
    /// in scan mode
    #[inline(always)]
    pub const fn with_awdsgl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::AWDSGL_MASK
            } else {
                self.bits & !Ctlr1::AWDSGL_MASK
            },
        }
    }

    /// Set **SCAN** `[8:8]`
    ///
    /// Scan mode enable
    #[inline(always)]
    pub const fn with_scan(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::SCAN_MASK
            } else {
                self.bits & !Ctlr1::SCAN_MASK
            },
        }
    }

    /// Set **JEOCIE** `[7:7]`
    ///
    /// Interrupt enable for injected
    /// channels
    #[inline(always)]
    pub const fn with_jeocie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::JEOCIE_MASK
            } else {
                self.bits & !Ctlr1::JEOCIE_MASK
            },
        }
    }

    /// Set **AWDIE** `[6:6]`
    ///
    /// Analog watchdog interrupt
    /// enable
    #[inline(always)]
    pub const fn with_awdie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::AWDIE_MASK
            } else {
                self.bits & !Ctlr1::AWDIE_MASK
            },
        }
    }

    /// Set **EOCIE** `[5:5]`
    ///
    /// Interrupt enable for EOC
    #[inline(always)]
    pub const fn with_eocie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::EOCIE_MASK
            } else {
                self.bits & !Ctlr1::EOCIE_MASK
            },
        }
    }

    /// Set **AWDCH** `[3:0]`
    ///
    /// Analog watchdog channel select
    /// bits
    #[inline(always)]
    pub const fn with_awdch(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::AWDCH_MASK)
                | ((value << Ctlr1::AWDCH_OFFSET) & Ctlr1::AWDCH_MASK),
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
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SWSTART** `[22]` (RW)
///
///   Start conversion of regular
///   channels
///
/// - **JSWSTART** `[21]` (RW)
///
///   Start conversion of injected
///   channels
///
/// - **EXTTRIG** `[20]` (RW)
///
///   External trigger conversion mode for
///   regular channels
///
/// - **EXTSEL** `[19:17]` (RW)
///
///   External event select for regular
///   group
///
/// - **JEXTTRIG** `[15]` (RW)
///
///   External trigger conversion mode for
///   injected channels
///
/// - **JEXTSEL** `[14:12]` (RW)
///
///   External event select for injected
///   group
///
/// - **ALIGN** `[11]` (RW)
///
///   Data alignment
///
/// - **DMA** `[8]` (RW)
///
///   Direct memory access mode
///
/// - **CONT** `[1]` (RW)
///
///   Continuous conversion
///
/// - **ADON** `[0]` (RW)
///
///   A/D converter ON / OFF
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

    /// **SWSTART**
    ///
    /// Start conversion of regular
    /// channels
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SWSTART_OFFSET: u32 = 22;
    pub const SWSTART_MASK: u32 = 0x00400000;
    pub const SWSTART: u32 = 1 << 22;

    /// **JSWSTART**
    ///
    /// Start conversion of injected
    /// channels
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const JSWSTART_OFFSET: u32 = 21;
    pub const JSWSTART_MASK: u32 = 0x00200000;
    pub const JSWSTART: u32 = 1 << 21;

    /// **EXTTRIG**
    ///
    /// External trigger conversion mode for
    /// regular channels
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EXTTRIG_OFFSET: u32 = 20;
    pub const EXTTRIG_MASK: u32 = 0x00100000;
    pub const EXTTRIG: u32 = 1 << 20;

    /// **EXTSEL**
    ///
    /// External event select for regular
    /// group
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const EXTSEL_OFFSET: u32 = 17;
    pub const EXTSEL_MASK: u32 = 0x000e0000;

    /// **JEXTTRIG**
    ///
    /// External trigger conversion mode for
    /// injected channels
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const JEXTTRIG_OFFSET: u32 = 15;
    pub const JEXTTRIG_MASK: u32 = 0x00008000;
    pub const JEXTTRIG: u32 = 1 << 15;

    /// **JEXTSEL**
    ///
    /// External event select for injected
    /// group
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const JEXTSEL_OFFSET: u32 = 12;
    pub const JEXTSEL_MASK: u32 = 0x00007000;

    /// **ALIGN**
    ///
    /// Data alignment
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ALIGN_OFFSET: u32 = 11;
    pub const ALIGN_MASK: u32 = 0x00000800;
    pub const ALIGN: u32 = 1 << 11;

    /// **DMA**
    ///
    /// Direct memory access mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DMA_OFFSET: u32 = 8;
    pub const DMA_MASK: u32 = 0x00000100;
    pub const DMA: u32 = 1 << 8;

    /// **CONT**
    ///
    /// Continuous conversion
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CONT_OFFSET: u32 = 1;
    pub const CONT_MASK: u32 = 0x00000002;
    pub const CONT: u32 = 1 << 1;

    /// **ADON**
    ///
    /// A/D converter ON / OFF
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADON_OFFSET: u32 = 0;
    pub const ADON_MASK: u32 = 0x00000001;
    pub const ADON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SWSTART** `[22]` (RW)
    ///
    /// Start conversion of regular
    /// channels
    #[inline(always)]
    pub fn swstart(&self) -> bool {
        (self.value.read_bits() & Self::SWSTART_MASK) != 0
    }

    /// Set SWSTART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swstart(&self) {
        self.value.modify_bits(|r| r | Self::SWSTART_MASK);
    }

    /// Clear SWSTART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swstart(&self) {
        self.value.modify_bits(|r| r & !Self::SWSTART_MASK);
    }

    /// **JSWSTART** `[21]` (RW)
    ///
    /// Start conversion of injected
    /// channels
    #[inline(always)]
    pub fn jswstart(&self) -> bool {
        (self.value.read_bits() & Self::JSWSTART_MASK) != 0
    }

    /// Set JSWSTART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jswstart(&self) {
        self.value.modify_bits(|r| r | Self::JSWSTART_MASK);
    }

    /// Clear JSWSTART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jswstart(&self) {
        self.value.modify_bits(|r| r & !Self::JSWSTART_MASK);
    }

    /// **EXTTRIG** `[20]` (RW)
    ///
    /// External trigger conversion mode for
    /// regular channels
    #[inline(always)]
    pub fn exttrig(&self) -> bool {
        (self.value.read_bits() & Self::EXTTRIG_MASK) != 0
    }

    /// Set EXTTRIG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_exttrig(&self) {
        self.value.modify_bits(|r| r | Self::EXTTRIG_MASK);
    }

    /// Clear EXTTRIG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_exttrig(&self) {
        self.value.modify_bits(|r| r & !Self::EXTTRIG_MASK);
    }

    /// **EXTSEL** `[19:17]` (RW)
    ///
    /// External event select for regular
    /// group
    #[inline(always)]
    pub fn extsel(&self) -> u32 {
        (self.value.read_bits() & Self::EXTSEL_MASK) >> Self::EXTSEL_OFFSET
    }

    /// Write **EXTSEL** `[19:17]` (RW)
    ///
    /// External event select for regular
    /// group
    #[inline(always)]
    pub fn write_extsel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EXTSEL_MASK) | ((value << Self::EXTSEL_OFFSET) & Self::EXTSEL_MASK)
        });
    }

    /// **JEXTTRIG** `[15]` (RW)
    ///
    /// External trigger conversion mode for
    /// injected channels
    #[inline(always)]
    pub fn jexttrig(&self) -> bool {
        (self.value.read_bits() & Self::JEXTTRIG_MASK) != 0
    }

    /// Set JEXTTRIG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jexttrig(&self) {
        self.value.modify_bits(|r| r | Self::JEXTTRIG_MASK);
    }

    /// Clear JEXTTRIG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jexttrig(&self) {
        self.value.modify_bits(|r| r & !Self::JEXTTRIG_MASK);
    }

    /// **JEXTSEL** `[14:12]` (RW)
    ///
    /// External event select for injected
    /// group
    #[inline(always)]
    pub fn jextsel(&self) -> u32 {
        (self.value.read_bits() & Self::JEXTSEL_MASK) >> Self::JEXTSEL_OFFSET
    }

    /// Write **JEXTSEL** `[14:12]` (RW)
    ///
    /// External event select for injected
    /// group
    #[inline(always)]
    pub fn write_jextsel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JEXTSEL_MASK) | ((value << Self::JEXTSEL_OFFSET) & Self::JEXTSEL_MASK)
        });
    }

    /// **ALIGN** `[11]` (RW)
    ///
    /// Data alignment
    #[inline(always)]
    pub fn align(&self) -> bool {
        (self.value.read_bits() & Self::ALIGN_MASK) != 0
    }

    /// Set ALIGN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_align(&self) {
        self.value.modify_bits(|r| r | Self::ALIGN_MASK);
    }

    /// Clear ALIGN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_align(&self) {
        self.value.modify_bits(|r| r & !Self::ALIGN_MASK);
    }

    /// **DMA** `[8]` (RW)
    ///
    /// Direct memory access mode
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma(&self) {
        self.value.modify_bits(|r| r | Self::DMA_MASK);
    }

    /// Clear DMA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_MASK);
    }

    /// **CONT** `[1]` (RW)
    ///
    /// Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> bool {
        (self.value.read_bits() & Self::CONT_MASK) != 0
    }

    /// Set CONT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cont(&self) {
        self.value.modify_bits(|r| r | Self::CONT_MASK);
    }

    /// Clear CONT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cont(&self) {
        self.value.modify_bits(|r| r & !Self::CONT_MASK);
    }

    /// **ADON** `[0]` (RW)
    ///
    /// A/D converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> bool {
        (self.value.read_bits() & Self::ADON_MASK) != 0
    }

    /// Set ADON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_adon(&self) {
        self.value.modify_bits(|r| r | Self::ADON_MASK);
    }

    /// Clear ADON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_adon(&self) {
        self.value.modify_bits(|r| r & !Self::ADON_MASK);
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

    /// Read **SWSTART** `[22:22]`
    ///
    /// Start conversion of regular
    /// channels
    #[inline(always)]
    pub const fn swstart(&self) -> bool {
        (self.bits & Ctlr2::SWSTART_MASK) != 0
    }

    /// Read **JSWSTART** `[21:21]`
    ///
    /// Start conversion of injected
    /// channels
    #[inline(always)]
    pub const fn jswstart(&self) -> bool {
        (self.bits & Ctlr2::JSWSTART_MASK) != 0
    }

    /// Read **EXTTRIG** `[20:20]`
    ///
    /// External trigger conversion mode for
    /// regular channels
    #[inline(always)]
    pub const fn exttrig(&self) -> bool {
        (self.bits & Ctlr2::EXTTRIG_MASK) != 0
    }

    /// Read **EXTSEL** `[19:17]`
    ///
    /// External event select for regular
    /// group
    #[inline(always)]
    pub const fn extsel(&self) -> u32 {
        (self.bits & Ctlr2::EXTSEL_MASK) >> Ctlr2::EXTSEL_OFFSET
    }

    /// Read **JEXTTRIG** `[15:15]`
    ///
    /// External trigger conversion mode for
    /// injected channels
    #[inline(always)]
    pub const fn jexttrig(&self) -> bool {
        (self.bits & Ctlr2::JEXTTRIG_MASK) != 0
    }

    /// Read **JEXTSEL** `[14:12]`
    ///
    /// External event select for injected
    /// group
    #[inline(always)]
    pub const fn jextsel(&self) -> u32 {
        (self.bits & Ctlr2::JEXTSEL_MASK) >> Ctlr2::JEXTSEL_OFFSET
    }

    /// Read **ALIGN** `[11:11]`
    ///
    /// Data alignment
    #[inline(always)]
    pub const fn align(&self) -> bool {
        (self.bits & Ctlr2::ALIGN_MASK) != 0
    }

    /// Read **DMA** `[8:8]`
    ///
    /// Direct memory access mode
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Ctlr2::DMA_MASK) != 0
    }

    /// Read **CONT** `[1:1]`
    ///
    /// Continuous conversion
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        (self.bits & Ctlr2::CONT_MASK) != 0
    }

    /// Read **ADON** `[0:0]`
    ///
    /// A/D converter ON / OFF
    #[inline(always)]
    pub const fn adon(&self) -> bool {
        (self.bits & Ctlr2::ADON_MASK) != 0
    }

    /// Set **SWSTART** `[22:22]`
    ///
    /// Start conversion of regular
    /// channels
    #[inline(always)]
    pub const fn with_swstart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::SWSTART_MASK
            } else {
                self.bits & !Ctlr2::SWSTART_MASK
            },
        }
    }

    /// Set **JSWSTART** `[21:21]`
    ///
    /// Start conversion of injected
    /// channels
    #[inline(always)]
    pub const fn with_jswstart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::JSWSTART_MASK
            } else {
                self.bits & !Ctlr2::JSWSTART_MASK
            },
        }
    }

    /// Set **EXTTRIG** `[20:20]`
    ///
    /// External trigger conversion mode for
    /// regular channels
    #[inline(always)]
    pub const fn with_exttrig(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::EXTTRIG_MASK
            } else {
                self.bits & !Ctlr2::EXTTRIG_MASK
            },
        }
    }

    /// Set **EXTSEL** `[19:17]`
    ///
    /// External event select for regular
    /// group
    #[inline(always)]
    pub const fn with_extsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr2::EXTSEL_MASK)
                | ((value << Ctlr2::EXTSEL_OFFSET) & Ctlr2::EXTSEL_MASK),
        }
    }

    /// Set **JEXTTRIG** `[15:15]`
    ///
    /// External trigger conversion mode for
    /// injected channels
    #[inline(always)]
    pub const fn with_jexttrig(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::JEXTTRIG_MASK
            } else {
                self.bits & !Ctlr2::JEXTTRIG_MASK
            },
        }
    }

    /// Set **JEXTSEL** `[14:12]`
    ///
    /// External event select for injected
    /// group
    #[inline(always)]
    pub const fn with_jextsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr2::JEXTSEL_MASK)
                | ((value << Ctlr2::JEXTSEL_OFFSET) & Ctlr2::JEXTSEL_MASK),
        }
    }

    /// Set **ALIGN** `[11:11]`
    ///
    /// Data alignment
    #[inline(always)]
    pub const fn with_align(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::ALIGN_MASK
            } else {
                self.bits & !Ctlr2::ALIGN_MASK
            },
        }
    }

    /// Set **DMA** `[8:8]`
    ///
    /// Direct memory access mode
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::DMA_MASK
            } else {
                self.bits & !Ctlr2::DMA_MASK
            },
        }
    }

    /// Set **CONT** `[1:1]`
    ///
    /// Continuous conversion
    #[inline(always)]
    pub const fn with_cont(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CONT_MASK
            } else {
                self.bits & !Ctlr2::CONT_MASK
            },
        }
    }

    /// Set **ADON** `[0:0]`
    ///
    /// A/D converter ON / OFF
    #[inline(always)]
    pub const fn with_adon(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::ADON_MASK
            } else {
                self.bits & !Ctlr2::ADON_MASK
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

/// SAMPTR1_CHARGE1 Register
///
/// sample time register 1/TKEYx_CHARGE1
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SMP17_TKCG17** `[23:21]` (RW)
///
///   Channel 17 sample time
///   selection
///
/// - **SMP16_TKCG16** `[20:18]` (RW)
///
///   Channel 16 sample time
///   selection
///
/// - **SMP15_TKCG15** `[17:15]` (RW)
///
///   Channel 15 sample time
///   selection
///
/// - **SMP14_TKCG14** `[14:12]` (RW)
///
///   Channel 14 sample time
///   selection
///
/// - **SMP13_TKCG13** `[11:9]` (RW)
///
///   Channel 13 sample time
///   selection
///
/// - **SMP12_TKCG12** `[8:6]` (RW)
///
///   Channel 12 sample time
///   selection
///
/// - **SMP11_TKCG11** `[5:3]` (RW)
///
///   Channel 11 sample time
///   selection
///
/// - **SMP10_TKCG10** `[2:0]` (RW)
///
///   Channel 10 sample time
///   selection
#[repr(transparent)]
pub struct Samptr1Charge1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Samptr1Charge1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Samptr1Charge1 {
    // ======== Field Constants ========

    /// **SMP17_TKCG17**
    ///
    /// Channel 17 sample time
    /// selection
    ///
    /// Bits `[23:21]`, mask `0x00e00000`
    pub const SMP17_TKCG17_OFFSET: u32 = 21;
    pub const SMP17_TKCG17_MASK: u32 = 0x00e00000;

    /// **SMP16_TKCG16**
    ///
    /// Channel 16 sample time
    /// selection
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SMP16_TKCG16_OFFSET: u32 = 18;
    pub const SMP16_TKCG16_MASK: u32 = 0x001c0000;

    /// **SMP15_TKCG15**
    ///
    /// Channel 15 sample time
    /// selection
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const SMP15_TKCG15_OFFSET: u32 = 15;
    pub const SMP15_TKCG15_MASK: u32 = 0x00038000;

    /// **SMP14_TKCG14**
    ///
    /// Channel 14 sample time
    /// selection
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const SMP14_TKCG14_OFFSET: u32 = 12;
    pub const SMP14_TKCG14_MASK: u32 = 0x00007000;

    /// **SMP13_TKCG13**
    ///
    /// Channel 13 sample time
    /// selection
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const SMP13_TKCG13_OFFSET: u32 = 9;
    pub const SMP13_TKCG13_MASK: u32 = 0x00000e00;

    /// **SMP12_TKCG12**
    ///
    /// Channel 12 sample time
    /// selection
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SMP12_TKCG12_OFFSET: u32 = 6;
    pub const SMP12_TKCG12_MASK: u32 = 0x000001c0;

    /// **SMP11_TKCG11**
    ///
    /// Channel 11 sample time
    /// selection
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const SMP11_TKCG11_OFFSET: u32 = 3;
    pub const SMP11_TKCG11_MASK: u32 = 0x00000038;

    /// **SMP10_TKCG10**
    ///
    /// Channel 10 sample time
    /// selection
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SMP10_TKCG10_OFFSET: u32 = 0;
    pub const SMP10_TKCG10_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SMP17_TKCG17** `[23:21]` (RW)
    ///
    /// Channel 17 sample time
    /// selection
    #[inline(always)]
    pub fn smp17_tkcg17(&self) -> u32 {
        (self.value.read_bits() & Self::SMP17_TKCG17_MASK) >> Self::SMP17_TKCG17_OFFSET
    }

    /// Write **SMP17_TKCG17** `[23:21]` (RW)
    ///
    /// Channel 17 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp17_tkcg17(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP17_TKCG17_MASK)
                | ((value << Self::SMP17_TKCG17_OFFSET) & Self::SMP17_TKCG17_MASK)
        });
    }

    /// **SMP16_TKCG16** `[20:18]` (RW)
    ///
    /// Channel 16 sample time
    /// selection
    #[inline(always)]
    pub fn smp16_tkcg16(&self) -> u32 {
        (self.value.read_bits() & Self::SMP16_TKCG16_MASK) >> Self::SMP16_TKCG16_OFFSET
    }

    /// Write **SMP16_TKCG16** `[20:18]` (RW)
    ///
    /// Channel 16 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp16_tkcg16(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP16_TKCG16_MASK)
                | ((value << Self::SMP16_TKCG16_OFFSET) & Self::SMP16_TKCG16_MASK)
        });
    }

    /// **SMP15_TKCG15** `[17:15]` (RW)
    ///
    /// Channel 15 sample time
    /// selection
    #[inline(always)]
    pub fn smp15_tkcg15(&self) -> u32 {
        (self.value.read_bits() & Self::SMP15_TKCG15_MASK) >> Self::SMP15_TKCG15_OFFSET
    }

    /// Write **SMP15_TKCG15** `[17:15]` (RW)
    ///
    /// Channel 15 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp15_tkcg15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP15_TKCG15_MASK)
                | ((value << Self::SMP15_TKCG15_OFFSET) & Self::SMP15_TKCG15_MASK)
        });
    }

    /// **SMP14_TKCG14** `[14:12]` (RW)
    ///
    /// Channel 14 sample time
    /// selection
    #[inline(always)]
    pub fn smp14_tkcg14(&self) -> u32 {
        (self.value.read_bits() & Self::SMP14_TKCG14_MASK) >> Self::SMP14_TKCG14_OFFSET
    }

    /// Write **SMP14_TKCG14** `[14:12]` (RW)
    ///
    /// Channel 14 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp14_tkcg14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP14_TKCG14_MASK)
                | ((value << Self::SMP14_TKCG14_OFFSET) & Self::SMP14_TKCG14_MASK)
        });
    }

    /// **SMP13_TKCG13** `[11:9]` (RW)
    ///
    /// Channel 13 sample time
    /// selection
    #[inline(always)]
    pub fn smp13_tkcg13(&self) -> u32 {
        (self.value.read_bits() & Self::SMP13_TKCG13_MASK) >> Self::SMP13_TKCG13_OFFSET
    }

    /// Write **SMP13_TKCG13** `[11:9]` (RW)
    ///
    /// Channel 13 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp13_tkcg13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP13_TKCG13_MASK)
                | ((value << Self::SMP13_TKCG13_OFFSET) & Self::SMP13_TKCG13_MASK)
        });
    }

    /// **SMP12_TKCG12** `[8:6]` (RW)
    ///
    /// Channel 12 sample time
    /// selection
    #[inline(always)]
    pub fn smp12_tkcg12(&self) -> u32 {
        (self.value.read_bits() & Self::SMP12_TKCG12_MASK) >> Self::SMP12_TKCG12_OFFSET
    }

    /// Write **SMP12_TKCG12** `[8:6]` (RW)
    ///
    /// Channel 12 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp12_tkcg12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP12_TKCG12_MASK)
                | ((value << Self::SMP12_TKCG12_OFFSET) & Self::SMP12_TKCG12_MASK)
        });
    }

    /// **SMP11_TKCG11** `[5:3]` (RW)
    ///
    /// Channel 11 sample time
    /// selection
    #[inline(always)]
    pub fn smp11_tkcg11(&self) -> u32 {
        (self.value.read_bits() & Self::SMP11_TKCG11_MASK) >> Self::SMP11_TKCG11_OFFSET
    }

    /// Write **SMP11_TKCG11** `[5:3]` (RW)
    ///
    /// Channel 11 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp11_tkcg11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP11_TKCG11_MASK)
                | ((value << Self::SMP11_TKCG11_OFFSET) & Self::SMP11_TKCG11_MASK)
        });
    }

    /// **SMP10_TKCG10** `[2:0]` (RW)
    ///
    /// Channel 10 sample time
    /// selection
    #[inline(always)]
    pub fn smp10_tkcg10(&self) -> u32 {
        (self.value.read_bits() & Self::SMP10_TKCG10_MASK) >> Self::SMP10_TKCG10_OFFSET
    }

    /// Write **SMP10_TKCG10** `[2:0]` (RW)
    ///
    /// Channel 10 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp10_tkcg10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP10_TKCG10_MASK)
                | ((value << Self::SMP10_TKCG10_OFFSET) & Self::SMP10_TKCG10_MASK)
        });
    }
}

/// SAMPTR1_CHARGE1 value type for batch read/modify/write operations
///
/// Use `samptr1charge1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `samptr1charge1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Samptr1Charge1Val {
    bits: u32,
}

impl Samptr1Charge1Val {
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

    /// Read **SMP17_TKCG17** `[23:21]`
    ///
    /// Channel 17 sample time
    /// selection
    #[inline(always)]
    pub const fn smp17_tkcg17(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP17_TKCG17_MASK) >> Samptr1Charge1::SMP17_TKCG17_OFFSET
    }

    /// Read **SMP16_TKCG16** `[20:18]`
    ///
    /// Channel 16 sample time
    /// selection
    #[inline(always)]
    pub const fn smp16_tkcg16(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP16_TKCG16_MASK) >> Samptr1Charge1::SMP16_TKCG16_OFFSET
    }

    /// Read **SMP15_TKCG15** `[17:15]`
    ///
    /// Channel 15 sample time
    /// selection
    #[inline(always)]
    pub const fn smp15_tkcg15(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP15_TKCG15_MASK) >> Samptr1Charge1::SMP15_TKCG15_OFFSET
    }

    /// Read **SMP14_TKCG14** `[14:12]`
    ///
    /// Channel 14 sample time
    /// selection
    #[inline(always)]
    pub const fn smp14_tkcg14(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP14_TKCG14_MASK) >> Samptr1Charge1::SMP14_TKCG14_OFFSET
    }

    /// Read **SMP13_TKCG13** `[11:9]`
    ///
    /// Channel 13 sample time
    /// selection
    #[inline(always)]
    pub const fn smp13_tkcg13(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP13_TKCG13_MASK) >> Samptr1Charge1::SMP13_TKCG13_OFFSET
    }

    /// Read **SMP12_TKCG12** `[8:6]`
    ///
    /// Channel 12 sample time
    /// selection
    #[inline(always)]
    pub const fn smp12_tkcg12(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP12_TKCG12_MASK) >> Samptr1Charge1::SMP12_TKCG12_OFFSET
    }

    /// Read **SMP11_TKCG11** `[5:3]`
    ///
    /// Channel 11 sample time
    /// selection
    #[inline(always)]
    pub const fn smp11_tkcg11(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP11_TKCG11_MASK) >> Samptr1Charge1::SMP11_TKCG11_OFFSET
    }

    /// Read **SMP10_TKCG10** `[2:0]`
    ///
    /// Channel 10 sample time
    /// selection
    #[inline(always)]
    pub const fn smp10_tkcg10(&self) -> u32 {
        (self.bits & Samptr1Charge1::SMP10_TKCG10_MASK) >> Samptr1Charge1::SMP10_TKCG10_OFFSET
    }

    /// Set **SMP17_TKCG17** `[23:21]`
    ///
    /// Channel 17 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp17_tkcg17(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP17_TKCG17_MASK)
                | ((value << Samptr1Charge1::SMP17_TKCG17_OFFSET)
                    & Samptr1Charge1::SMP17_TKCG17_MASK),
        }
    }

    /// Set **SMP16_TKCG16** `[20:18]`
    ///
    /// Channel 16 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp16_tkcg16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP16_TKCG16_MASK)
                | ((value << Samptr1Charge1::SMP16_TKCG16_OFFSET)
                    & Samptr1Charge1::SMP16_TKCG16_MASK),
        }
    }

    /// Set **SMP15_TKCG15** `[17:15]`
    ///
    /// Channel 15 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp15_tkcg15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP15_TKCG15_MASK)
                | ((value << Samptr1Charge1::SMP15_TKCG15_OFFSET)
                    & Samptr1Charge1::SMP15_TKCG15_MASK),
        }
    }

    /// Set **SMP14_TKCG14** `[14:12]`
    ///
    /// Channel 14 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp14_tkcg14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP14_TKCG14_MASK)
                | ((value << Samptr1Charge1::SMP14_TKCG14_OFFSET)
                    & Samptr1Charge1::SMP14_TKCG14_MASK),
        }
    }

    /// Set **SMP13_TKCG13** `[11:9]`
    ///
    /// Channel 13 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp13_tkcg13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP13_TKCG13_MASK)
                | ((value << Samptr1Charge1::SMP13_TKCG13_OFFSET)
                    & Samptr1Charge1::SMP13_TKCG13_MASK),
        }
    }

    /// Set **SMP12_TKCG12** `[8:6]`
    ///
    /// Channel 12 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp12_tkcg12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP12_TKCG12_MASK)
                | ((value << Samptr1Charge1::SMP12_TKCG12_OFFSET)
                    & Samptr1Charge1::SMP12_TKCG12_MASK),
        }
    }

    /// Set **SMP11_TKCG11** `[5:3]`
    ///
    /// Channel 11 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp11_tkcg11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP11_TKCG11_MASK)
                | ((value << Samptr1Charge1::SMP11_TKCG11_OFFSET)
                    & Samptr1Charge1::SMP11_TKCG11_MASK),
        }
    }

    /// Set **SMP10_TKCG10** `[2:0]`
    ///
    /// Channel 10 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp10_tkcg10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr1Charge1::SMP10_TKCG10_MASK)
                | ((value << Samptr1Charge1::SMP10_TKCG10_OFFSET)
                    & Samptr1Charge1::SMP10_TKCG10_MASK),
        }
    }
}

impl Default for Samptr1Charge1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Samptr1Charge1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Samptr1Charge1Val {
        Samptr1Charge1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Samptr1Charge1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Samptr1Charge1Val) -> Samptr1Charge1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAMPTR2_CHARGE2 Register
///
/// sample time register 2/TKEYx_CHARGE2
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SMP9_TKCG9** `[29:27]` (RW)
///
///   Channel 9 sample time
///   selection
///
/// - **SMP8_TKCG8** `[26:24]` (RW)
///
///   Channel 8 sample time
///   selection
///
/// - **SMP7_TKCG7** `[23:21]` (RW)
///
///   Channel 7 sample time
///   selection
///
/// - **SMP6_TKCG6** `[20:18]` (RW)
///
///   Channel 6 sample time
///   selection
///
/// - **SMP5_TKCG5** `[17:15]` (RW)
///
///   Channel 5 sample time
///   selection
///
/// - **SMP4_TKCG4** `[14:12]` (RW)
///
///   Channel 4 sample time
///   selection
///
/// - **SMP3_TKCG3** `[11:9]` (RW)
///
///   Channel 3 sample time
///   selection
///
/// - **SMP2_TKCG2** `[8:6]` (RW)
///
///   Channel 2 sample time
///   selection
///
/// - **SMP1_TKCG1** `[5:3]` (RW)
///
///   Channel 1 sample time
///   selection
///
/// - **SMP0_TKCG0** `[2:0]` (RW)
///
///   Channel 0 sample time
///   selection
#[repr(transparent)]
pub struct Samptr2Charge2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Samptr2Charge2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Samptr2Charge2 {
    // ======== Field Constants ========

    /// **SMP9_TKCG9**
    ///
    /// Channel 9 sample time
    /// selection
    ///
    /// Bits `[29:27]`, mask `0x38000000`
    pub const SMP9_TKCG9_OFFSET: u32 = 27;
    pub const SMP9_TKCG9_MASK: u32 = 0x38000000;

    /// **SMP8_TKCG8**
    ///
    /// Channel 8 sample time
    /// selection
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const SMP8_TKCG8_OFFSET: u32 = 24;
    pub const SMP8_TKCG8_MASK: u32 = 0x07000000;

    /// **SMP7_TKCG7**
    ///
    /// Channel 7 sample time
    /// selection
    ///
    /// Bits `[23:21]`, mask `0x00e00000`
    pub const SMP7_TKCG7_OFFSET: u32 = 21;
    pub const SMP7_TKCG7_MASK: u32 = 0x00e00000;

    /// **SMP6_TKCG6**
    ///
    /// Channel 6 sample time
    /// selection
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SMP6_TKCG6_OFFSET: u32 = 18;
    pub const SMP6_TKCG6_MASK: u32 = 0x001c0000;

    /// **SMP5_TKCG5**
    ///
    /// Channel 5 sample time
    /// selection
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const SMP5_TKCG5_OFFSET: u32 = 15;
    pub const SMP5_TKCG5_MASK: u32 = 0x00038000;

    /// **SMP4_TKCG4**
    ///
    /// Channel 4 sample time
    /// selection
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const SMP4_TKCG4_OFFSET: u32 = 12;
    pub const SMP4_TKCG4_MASK: u32 = 0x00007000;

    /// **SMP3_TKCG3**
    ///
    /// Channel 3 sample time
    /// selection
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const SMP3_TKCG3_OFFSET: u32 = 9;
    pub const SMP3_TKCG3_MASK: u32 = 0x00000e00;

    /// **SMP2_TKCG2**
    ///
    /// Channel 2 sample time
    /// selection
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SMP2_TKCG2_OFFSET: u32 = 6;
    pub const SMP2_TKCG2_MASK: u32 = 0x000001c0;

    /// **SMP1_TKCG1**
    ///
    /// Channel 1 sample time
    /// selection
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const SMP1_TKCG1_OFFSET: u32 = 3;
    pub const SMP1_TKCG1_MASK: u32 = 0x00000038;

    /// **SMP0_TKCG0**
    ///
    /// Channel 0 sample time
    /// selection
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SMP0_TKCG0_OFFSET: u32 = 0;
    pub const SMP0_TKCG0_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SMP9_TKCG9** `[29:27]` (RW)
    ///
    /// Channel 9 sample time
    /// selection
    #[inline(always)]
    pub fn smp9_tkcg9(&self) -> u32 {
        (self.value.read_bits() & Self::SMP9_TKCG9_MASK) >> Self::SMP9_TKCG9_OFFSET
    }

    /// Write **SMP9_TKCG9** `[29:27]` (RW)
    ///
    /// Channel 9 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp9_tkcg9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP9_TKCG9_MASK)
                | ((value << Self::SMP9_TKCG9_OFFSET) & Self::SMP9_TKCG9_MASK)
        });
    }

    /// **SMP8_TKCG8** `[26:24]` (RW)
    ///
    /// Channel 8 sample time
    /// selection
    #[inline(always)]
    pub fn smp8_tkcg8(&self) -> u32 {
        (self.value.read_bits() & Self::SMP8_TKCG8_MASK) >> Self::SMP8_TKCG8_OFFSET
    }

    /// Write **SMP8_TKCG8** `[26:24]` (RW)
    ///
    /// Channel 8 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp8_tkcg8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP8_TKCG8_MASK)
                | ((value << Self::SMP8_TKCG8_OFFSET) & Self::SMP8_TKCG8_MASK)
        });
    }

    /// **SMP7_TKCG7** `[23:21]` (RW)
    ///
    /// Channel 7 sample time
    /// selection
    #[inline(always)]
    pub fn smp7_tkcg7(&self) -> u32 {
        (self.value.read_bits() & Self::SMP7_TKCG7_MASK) >> Self::SMP7_TKCG7_OFFSET
    }

    /// Write **SMP7_TKCG7** `[23:21]` (RW)
    ///
    /// Channel 7 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp7_tkcg7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP7_TKCG7_MASK)
                | ((value << Self::SMP7_TKCG7_OFFSET) & Self::SMP7_TKCG7_MASK)
        });
    }

    /// **SMP6_TKCG6** `[20:18]` (RW)
    ///
    /// Channel 6 sample time
    /// selection
    #[inline(always)]
    pub fn smp6_tkcg6(&self) -> u32 {
        (self.value.read_bits() & Self::SMP6_TKCG6_MASK) >> Self::SMP6_TKCG6_OFFSET
    }

    /// Write **SMP6_TKCG6** `[20:18]` (RW)
    ///
    /// Channel 6 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp6_tkcg6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP6_TKCG6_MASK)
                | ((value << Self::SMP6_TKCG6_OFFSET) & Self::SMP6_TKCG6_MASK)
        });
    }

    /// **SMP5_TKCG5** `[17:15]` (RW)
    ///
    /// Channel 5 sample time
    /// selection
    #[inline(always)]
    pub fn smp5_tkcg5(&self) -> u32 {
        (self.value.read_bits() & Self::SMP5_TKCG5_MASK) >> Self::SMP5_TKCG5_OFFSET
    }

    /// Write **SMP5_TKCG5** `[17:15]` (RW)
    ///
    /// Channel 5 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp5_tkcg5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP5_TKCG5_MASK)
                | ((value << Self::SMP5_TKCG5_OFFSET) & Self::SMP5_TKCG5_MASK)
        });
    }

    /// **SMP4_TKCG4** `[14:12]` (RW)
    ///
    /// Channel 4 sample time
    /// selection
    #[inline(always)]
    pub fn smp4_tkcg4(&self) -> u32 {
        (self.value.read_bits() & Self::SMP4_TKCG4_MASK) >> Self::SMP4_TKCG4_OFFSET
    }

    /// Write **SMP4_TKCG4** `[14:12]` (RW)
    ///
    /// Channel 4 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp4_tkcg4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP4_TKCG4_MASK)
                | ((value << Self::SMP4_TKCG4_OFFSET) & Self::SMP4_TKCG4_MASK)
        });
    }

    /// **SMP3_TKCG3** `[11:9]` (RW)
    ///
    /// Channel 3 sample time
    /// selection
    #[inline(always)]
    pub fn smp3_tkcg3(&self) -> u32 {
        (self.value.read_bits() & Self::SMP3_TKCG3_MASK) >> Self::SMP3_TKCG3_OFFSET
    }

    /// Write **SMP3_TKCG3** `[11:9]` (RW)
    ///
    /// Channel 3 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp3_tkcg3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP3_TKCG3_MASK)
                | ((value << Self::SMP3_TKCG3_OFFSET) & Self::SMP3_TKCG3_MASK)
        });
    }

    /// **SMP2_TKCG2** `[8:6]` (RW)
    ///
    /// Channel 2 sample time
    /// selection
    #[inline(always)]
    pub fn smp2_tkcg2(&self) -> u32 {
        (self.value.read_bits() & Self::SMP2_TKCG2_MASK) >> Self::SMP2_TKCG2_OFFSET
    }

    /// Write **SMP2_TKCG2** `[8:6]` (RW)
    ///
    /// Channel 2 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp2_tkcg2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP2_TKCG2_MASK)
                | ((value << Self::SMP2_TKCG2_OFFSET) & Self::SMP2_TKCG2_MASK)
        });
    }

    /// **SMP1_TKCG1** `[5:3]` (RW)
    ///
    /// Channel 1 sample time
    /// selection
    #[inline(always)]
    pub fn smp1_tkcg1(&self) -> u32 {
        (self.value.read_bits() & Self::SMP1_TKCG1_MASK) >> Self::SMP1_TKCG1_OFFSET
    }

    /// Write **SMP1_TKCG1** `[5:3]` (RW)
    ///
    /// Channel 1 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp1_tkcg1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP1_TKCG1_MASK)
                | ((value << Self::SMP1_TKCG1_OFFSET) & Self::SMP1_TKCG1_MASK)
        });
    }

    /// **SMP0_TKCG0** `[2:0]` (RW)
    ///
    /// Channel 0 sample time
    /// selection
    #[inline(always)]
    pub fn smp0_tkcg0(&self) -> u32 {
        (self.value.read_bits() & Self::SMP0_TKCG0_MASK) >> Self::SMP0_TKCG0_OFFSET
    }

    /// Write **SMP0_TKCG0** `[2:0]` (RW)
    ///
    /// Channel 0 sample time
    /// selection
    #[inline(always)]
    pub fn write_smp0_tkcg0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SMP0_TKCG0_MASK)
                | ((value << Self::SMP0_TKCG0_OFFSET) & Self::SMP0_TKCG0_MASK)
        });
    }
}

/// SAMPTR2_CHARGE2 value type for batch read/modify/write operations
///
/// Use `samptr2charge2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `samptr2charge2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Samptr2Charge2Val {
    bits: u32,
}

impl Samptr2Charge2Val {
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

    /// Read **SMP9_TKCG9** `[29:27]`
    ///
    /// Channel 9 sample time
    /// selection
    #[inline(always)]
    pub const fn smp9_tkcg9(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP9_TKCG9_MASK) >> Samptr2Charge2::SMP9_TKCG9_OFFSET
    }

    /// Read **SMP8_TKCG8** `[26:24]`
    ///
    /// Channel 8 sample time
    /// selection
    #[inline(always)]
    pub const fn smp8_tkcg8(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP8_TKCG8_MASK) >> Samptr2Charge2::SMP8_TKCG8_OFFSET
    }

    /// Read **SMP7_TKCG7** `[23:21]`
    ///
    /// Channel 7 sample time
    /// selection
    #[inline(always)]
    pub const fn smp7_tkcg7(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP7_TKCG7_MASK) >> Samptr2Charge2::SMP7_TKCG7_OFFSET
    }

    /// Read **SMP6_TKCG6** `[20:18]`
    ///
    /// Channel 6 sample time
    /// selection
    #[inline(always)]
    pub const fn smp6_tkcg6(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP6_TKCG6_MASK) >> Samptr2Charge2::SMP6_TKCG6_OFFSET
    }

    /// Read **SMP5_TKCG5** `[17:15]`
    ///
    /// Channel 5 sample time
    /// selection
    #[inline(always)]
    pub const fn smp5_tkcg5(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP5_TKCG5_MASK) >> Samptr2Charge2::SMP5_TKCG5_OFFSET
    }

    /// Read **SMP4_TKCG4** `[14:12]`
    ///
    /// Channel 4 sample time
    /// selection
    #[inline(always)]
    pub const fn smp4_tkcg4(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP4_TKCG4_MASK) >> Samptr2Charge2::SMP4_TKCG4_OFFSET
    }

    /// Read **SMP3_TKCG3** `[11:9]`
    ///
    /// Channel 3 sample time
    /// selection
    #[inline(always)]
    pub const fn smp3_tkcg3(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP3_TKCG3_MASK) >> Samptr2Charge2::SMP3_TKCG3_OFFSET
    }

    /// Read **SMP2_TKCG2** `[8:6]`
    ///
    /// Channel 2 sample time
    /// selection
    #[inline(always)]
    pub const fn smp2_tkcg2(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP2_TKCG2_MASK) >> Samptr2Charge2::SMP2_TKCG2_OFFSET
    }

    /// Read **SMP1_TKCG1** `[5:3]`
    ///
    /// Channel 1 sample time
    /// selection
    #[inline(always)]
    pub const fn smp1_tkcg1(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP1_TKCG1_MASK) >> Samptr2Charge2::SMP1_TKCG1_OFFSET
    }

    /// Read **SMP0_TKCG0** `[2:0]`
    ///
    /// Channel 0 sample time
    /// selection
    #[inline(always)]
    pub const fn smp0_tkcg0(&self) -> u32 {
        (self.bits & Samptr2Charge2::SMP0_TKCG0_MASK) >> Samptr2Charge2::SMP0_TKCG0_OFFSET
    }

    /// Set **SMP9_TKCG9** `[29:27]`
    ///
    /// Channel 9 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp9_tkcg9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP9_TKCG9_MASK)
                | ((value << Samptr2Charge2::SMP9_TKCG9_OFFSET) & Samptr2Charge2::SMP9_TKCG9_MASK),
        }
    }

    /// Set **SMP8_TKCG8** `[26:24]`
    ///
    /// Channel 8 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp8_tkcg8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP8_TKCG8_MASK)
                | ((value << Samptr2Charge2::SMP8_TKCG8_OFFSET) & Samptr2Charge2::SMP8_TKCG8_MASK),
        }
    }

    /// Set **SMP7_TKCG7** `[23:21]`
    ///
    /// Channel 7 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp7_tkcg7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP7_TKCG7_MASK)
                | ((value << Samptr2Charge2::SMP7_TKCG7_OFFSET) & Samptr2Charge2::SMP7_TKCG7_MASK),
        }
    }

    /// Set **SMP6_TKCG6** `[20:18]`
    ///
    /// Channel 6 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp6_tkcg6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP6_TKCG6_MASK)
                | ((value << Samptr2Charge2::SMP6_TKCG6_OFFSET) & Samptr2Charge2::SMP6_TKCG6_MASK),
        }
    }

    /// Set **SMP5_TKCG5** `[17:15]`
    ///
    /// Channel 5 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp5_tkcg5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP5_TKCG5_MASK)
                | ((value << Samptr2Charge2::SMP5_TKCG5_OFFSET) & Samptr2Charge2::SMP5_TKCG5_MASK),
        }
    }

    /// Set **SMP4_TKCG4** `[14:12]`
    ///
    /// Channel 4 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp4_tkcg4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP4_TKCG4_MASK)
                | ((value << Samptr2Charge2::SMP4_TKCG4_OFFSET) & Samptr2Charge2::SMP4_TKCG4_MASK),
        }
    }

    /// Set **SMP3_TKCG3** `[11:9]`
    ///
    /// Channel 3 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp3_tkcg3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP3_TKCG3_MASK)
                | ((value << Samptr2Charge2::SMP3_TKCG3_OFFSET) & Samptr2Charge2::SMP3_TKCG3_MASK),
        }
    }

    /// Set **SMP2_TKCG2** `[8:6]`
    ///
    /// Channel 2 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp2_tkcg2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP2_TKCG2_MASK)
                | ((value << Samptr2Charge2::SMP2_TKCG2_OFFSET) & Samptr2Charge2::SMP2_TKCG2_MASK),
        }
    }

    /// Set **SMP1_TKCG1** `[5:3]`
    ///
    /// Channel 1 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp1_tkcg1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP1_TKCG1_MASK)
                | ((value << Samptr2Charge2::SMP1_TKCG1_OFFSET) & Samptr2Charge2::SMP1_TKCG1_MASK),
        }
    }

    /// Set **SMP0_TKCG0** `[2:0]`
    ///
    /// Channel 0 sample time
    /// selection
    #[inline(always)]
    pub const fn with_smp0_tkcg0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Samptr2Charge2::SMP0_TKCG0_MASK)
                | ((value << Samptr2Charge2::SMP0_TKCG0_OFFSET) & Samptr2Charge2::SMP0_TKCG0_MASK),
        }
    }
}

impl Default for Samptr2Charge2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Samptr2Charge2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Samptr2Charge2Val {
        Samptr2Charge2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Samptr2Charge2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Samptr2Charge2Val) -> Samptr2Charge2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IOFR1 Register
///
/// injected channel data offset register
/// x
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JOFFSET1** `[11:0]` (RW)
///
///   Data offset for injected channel
///   x
#[repr(transparent)]
pub struct Iofr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Iofr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Iofr1 {
    // ======== Field Constants ========

    /// **JOFFSET1**
    ///
    /// Data offset for injected channel
    /// x
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const JOFFSET1_OFFSET: u32 = 0;
    pub const JOFFSET1_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **JOFFSET1** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn joffset1(&self) -> u32 {
        (self.value.read_bits() & Self::JOFFSET1_MASK) >> Self::JOFFSET1_OFFSET
    }

    /// Write **JOFFSET1** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn write_joffset1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JOFFSET1_MASK) | ((value << Self::JOFFSET1_OFFSET) & Self::JOFFSET1_MASK)
        });
    }
}

/// IOFR1 value type for batch read/modify/write operations
///
/// Use `iofr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iofr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Iofr1Val {
    bits: u32,
}

impl Iofr1Val {
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

    /// Read **JOFFSET1** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn joffset1(&self) -> u32 {
        (self.bits & Iofr1::JOFFSET1_MASK) >> Iofr1::JOFFSET1_OFFSET
    }

    /// Set **JOFFSET1** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn with_joffset1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Iofr1::JOFFSET1_MASK)
                | ((value << Iofr1::JOFFSET1_OFFSET) & Iofr1::JOFFSET1_MASK),
        }
    }
}

impl Default for Iofr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Iofr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Iofr1Val {
        Iofr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Iofr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Iofr1Val) -> Iofr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IOFR2 Register
///
/// injected channel data offset register
/// x
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JOFFSET2** `[11:0]` (RW)
///
///   Data offset for injected channel
///   x
#[repr(transparent)]
pub struct Iofr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Iofr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Iofr2 {
    // ======== Field Constants ========

    /// **JOFFSET2**
    ///
    /// Data offset for injected channel
    /// x
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const JOFFSET2_OFFSET: u32 = 0;
    pub const JOFFSET2_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **JOFFSET2** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn joffset2(&self) -> u32 {
        (self.value.read_bits() & Self::JOFFSET2_MASK) >> Self::JOFFSET2_OFFSET
    }

    /// Write **JOFFSET2** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn write_joffset2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JOFFSET2_MASK) | ((value << Self::JOFFSET2_OFFSET) & Self::JOFFSET2_MASK)
        });
    }
}

/// IOFR2 value type for batch read/modify/write operations
///
/// Use `iofr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iofr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Iofr2Val {
    bits: u32,
}

impl Iofr2Val {
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

    /// Read **JOFFSET2** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn joffset2(&self) -> u32 {
        (self.bits & Iofr2::JOFFSET2_MASK) >> Iofr2::JOFFSET2_OFFSET
    }

    /// Set **JOFFSET2** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn with_joffset2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Iofr2::JOFFSET2_MASK)
                | ((value << Iofr2::JOFFSET2_OFFSET) & Iofr2::JOFFSET2_MASK),
        }
    }
}

impl Default for Iofr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Iofr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Iofr2Val {
        Iofr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Iofr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Iofr2Val) -> Iofr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IOFR3 Register
///
/// injected channel data offset register
/// x
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JOFFSET3** `[11:0]` (RW)
///
///   Data offset for injected channel
///   x
#[repr(transparent)]
pub struct Iofr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Iofr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Iofr3 {
    // ======== Field Constants ========

    /// **JOFFSET3**
    ///
    /// Data offset for injected channel
    /// x
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const JOFFSET3_OFFSET: u32 = 0;
    pub const JOFFSET3_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **JOFFSET3** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn joffset3(&self) -> u32 {
        (self.value.read_bits() & Self::JOFFSET3_MASK) >> Self::JOFFSET3_OFFSET
    }

    /// Write **JOFFSET3** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn write_joffset3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JOFFSET3_MASK) | ((value << Self::JOFFSET3_OFFSET) & Self::JOFFSET3_MASK)
        });
    }
}

/// IOFR3 value type for batch read/modify/write operations
///
/// Use `iofr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iofr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Iofr3Val {
    bits: u32,
}

impl Iofr3Val {
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

    /// Read **JOFFSET3** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn joffset3(&self) -> u32 {
        (self.bits & Iofr3::JOFFSET3_MASK) >> Iofr3::JOFFSET3_OFFSET
    }

    /// Set **JOFFSET3** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn with_joffset3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Iofr3::JOFFSET3_MASK)
                | ((value << Iofr3::JOFFSET3_OFFSET) & Iofr3::JOFFSET3_MASK),
        }
    }
}

impl Default for Iofr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Iofr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Iofr3Val {
        Iofr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Iofr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Iofr3Val) -> Iofr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IOFR4 Register
///
/// injected channel data offset register
/// x
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JOFFSET4** `[11:0]` (RW)
///
///   Data offset for injected channel
///   x
#[repr(transparent)]
pub struct Iofr4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Iofr4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Iofr4 {
    // ======== Field Constants ========

    /// **JOFFSET4**
    ///
    /// Data offset for injected channel
    /// x
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const JOFFSET4_OFFSET: u32 = 0;
    pub const JOFFSET4_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **JOFFSET4** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn joffset4(&self) -> u32 {
        (self.value.read_bits() & Self::JOFFSET4_MASK) >> Self::JOFFSET4_OFFSET
    }

    /// Write **JOFFSET4** `[11:0]` (RW)
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub fn write_joffset4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JOFFSET4_MASK) | ((value << Self::JOFFSET4_OFFSET) & Self::JOFFSET4_MASK)
        });
    }
}

/// IOFR4 value type for batch read/modify/write operations
///
/// Use `iofr4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iofr4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Iofr4Val {
    bits: u32,
}

impl Iofr4Val {
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

    /// Read **JOFFSET4** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn joffset4(&self) -> u32 {
        (self.bits & Iofr4::JOFFSET4_MASK) >> Iofr4::JOFFSET4_OFFSET
    }

    /// Set **JOFFSET4** `[11:0]`
    ///
    /// Data offset for injected channel
    /// x
    #[inline(always)]
    pub const fn with_joffset4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Iofr4::JOFFSET4_MASK)
                | ((value << Iofr4::JOFFSET4_OFFSET) & Iofr4::JOFFSET4_MASK),
        }
    }
}

impl Default for Iofr4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Iofr4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Iofr4Val {
        Iofr4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Iofr4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Iofr4Val) -> Iofr4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDHTR Register
///
/// watchdog higher threshold
/// register
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **HT** `[11:0]` (RW)
///
///   Analog watchdog higher
///   threshold
#[repr(transparent)]
pub struct Wdhtr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdhtr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdhtr {
    // ======== Field Constants ========

    /// **HT**
    ///
    /// Analog watchdog higher
    /// threshold
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const HT_OFFSET: u32 = 0;
    pub const HT_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **HT** `[11:0]` (RW)
    ///
    /// Analog watchdog higher
    /// threshold
    #[inline(always)]
    pub fn ht(&self) -> u32 {
        (self.value.read_bits() & Self::HT_MASK) >> Self::HT_OFFSET
    }

    /// Write **HT** `[11:0]` (RW)
    ///
    /// Analog watchdog higher
    /// threshold
    #[inline(always)]
    pub fn write_ht(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::HT_MASK) | ((value << Self::HT_OFFSET) & Self::HT_MASK));
    }
}

/// WDHTR value type for batch read/modify/write operations
///
/// Use `wdhtr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdhtr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdhtrVal {
    bits: u32,
}

impl WdhtrVal {
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

    /// Read **HT** `[11:0]`
    ///
    /// Analog watchdog higher
    /// threshold
    #[inline(always)]
    pub const fn ht(&self) -> u32 {
        (self.bits & Wdhtr::HT_MASK) >> Wdhtr::HT_OFFSET
    }

    /// Set **HT** `[11:0]`
    ///
    /// Analog watchdog higher
    /// threshold
    #[inline(always)]
    pub const fn with_ht(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdhtr::HT_MASK) | ((value << Wdhtr::HT_OFFSET) & Wdhtr::HT_MASK),
        }
    }
}

impl Default for WdhtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdhtr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WdhtrVal {
        WdhtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdhtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WdhtrVal) -> WdhtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDLTR Register
///
/// watchdog lower threshold
/// register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LT** `[11:0]` (RW)
///
///   Analog watchdog lower
///   threshold
#[repr(transparent)]
pub struct Wdltr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdltr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdltr {
    // ======== Field Constants ========

    /// **LT**
    ///
    /// Analog watchdog lower
    /// threshold
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const LT_OFFSET: u32 = 0;
    pub const LT_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **LT** `[11:0]` (RW)
    ///
    /// Analog watchdog lower
    /// threshold
    #[inline(always)]
    pub fn lt(&self) -> u32 {
        (self.value.read_bits() & Self::LT_MASK) >> Self::LT_OFFSET
    }

    /// Write **LT** `[11:0]` (RW)
    ///
    /// Analog watchdog lower
    /// threshold
    #[inline(always)]
    pub fn write_lt(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::LT_MASK) | ((value << Self::LT_OFFSET) & Self::LT_MASK));
    }
}

/// WDLTR value type for batch read/modify/write operations
///
/// Use `wdltr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdltr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdltrVal {
    bits: u32,
}

impl WdltrVal {
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

    /// Read **LT** `[11:0]`
    ///
    /// Analog watchdog lower
    /// threshold
    #[inline(always)]
    pub const fn lt(&self) -> u32 {
        (self.bits & Wdltr::LT_MASK) >> Wdltr::LT_OFFSET
    }

    /// Set **LT** `[11:0]`
    ///
    /// Analog watchdog lower
    /// threshold
    #[inline(always)]
    pub const fn with_lt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdltr::LT_MASK) | ((value << Wdltr::LT_OFFSET) & Wdltr::LT_MASK),
        }
    }
}

impl Default for WdltrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdltr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WdltrVal {
        WdltrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdltrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WdltrVal) -> WdltrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSQR1 Register
///
/// regular sequence register 1
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **L** `[23:20]` (RW)
///
///   Regular channel sequence
///   length
///
/// - **SQ16** `[19:15]` (RW)
///
///   16th conversion in regular
///   sequence
///
/// - **SQ15** `[14:10]` (RW)
///
///   15th conversion in regular
///   sequence
///
/// - **SQ14** `[9:5]` (RW)
///
///   14th conversion in regular
///   sequence
///
/// - **SQ13** `[4:0]` (RW)
///
///   13th conversion in regular
///   sequence
#[repr(transparent)]
pub struct Rsqr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rsqr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rsqr1 {
    // ======== Field Constants ========

    /// **L**
    ///
    /// Regular channel sequence
    /// length
    ///
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const L_OFFSET: u32 = 20;
    pub const L_MASK: u32 = 0x00f00000;

    /// **SQ16**
    ///
    /// 16th conversion in regular
    /// sequence
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const SQ16_OFFSET: u32 = 15;
    pub const SQ16_MASK: u32 = 0x000f8000;

    /// **SQ15**
    ///
    /// 15th conversion in regular
    /// sequence
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SQ15_OFFSET: u32 = 10;
    pub const SQ15_MASK: u32 = 0x00007c00;

    /// **SQ14**
    ///
    /// 14th conversion in regular
    /// sequence
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SQ14_OFFSET: u32 = 5;
    pub const SQ14_MASK: u32 = 0x000003e0;

    /// **SQ13**
    ///
    /// 13th conversion in regular
    /// sequence
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SQ13_OFFSET: u32 = 0;
    pub const SQ13_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **L** `[23:20]` (RW)
    ///
    /// Regular channel sequence
    /// length
    #[inline(always)]
    pub fn l(&self) -> u32 {
        (self.value.read_bits() & Self::L_MASK) >> Self::L_OFFSET
    }

    /// Write **L** `[23:20]` (RW)
    ///
    /// Regular channel sequence
    /// length
    #[inline(always)]
    pub fn write_l(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::L_MASK) | ((value << Self::L_OFFSET) & Self::L_MASK));
    }

    /// **SQ16** `[19:15]` (RW)
    ///
    /// 16th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq16(&self) -> u32 {
        (self.value.read_bits() & Self::SQ16_MASK) >> Self::SQ16_OFFSET
    }

    /// Write **SQ16** `[19:15]` (RW)
    ///
    /// 16th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq16(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ16_MASK) | ((value << Self::SQ16_OFFSET) & Self::SQ16_MASK)
        });
    }

    /// **SQ15** `[14:10]` (RW)
    ///
    /// 15th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq15(&self) -> u32 {
        (self.value.read_bits() & Self::SQ15_MASK) >> Self::SQ15_OFFSET
    }

    /// Write **SQ15** `[14:10]` (RW)
    ///
    /// 15th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ15_MASK) | ((value << Self::SQ15_OFFSET) & Self::SQ15_MASK)
        });
    }

    /// **SQ14** `[9:5]` (RW)
    ///
    /// 14th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq14(&self) -> u32 {
        (self.value.read_bits() & Self::SQ14_MASK) >> Self::SQ14_OFFSET
    }

    /// Write **SQ14** `[9:5]` (RW)
    ///
    /// 14th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ14_MASK) | ((value << Self::SQ14_OFFSET) & Self::SQ14_MASK)
        });
    }

    /// **SQ13** `[4:0]` (RW)
    ///
    /// 13th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq13(&self) -> u32 {
        (self.value.read_bits() & Self::SQ13_MASK) >> Self::SQ13_OFFSET
    }

    /// Write **SQ13** `[4:0]` (RW)
    ///
    /// 13th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ13_MASK) | ((value << Self::SQ13_OFFSET) & Self::SQ13_MASK)
        });
    }
}

/// RSQR1 value type for batch read/modify/write operations
///
/// Use `rsqr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rsqr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rsqr1Val {
    bits: u32,
}

impl Rsqr1Val {
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

    /// Read **L** `[23:20]`
    ///
    /// Regular channel sequence
    /// length
    #[inline(always)]
    pub const fn l(&self) -> u32 {
        (self.bits & Rsqr1::L_MASK) >> Rsqr1::L_OFFSET
    }

    /// Read **SQ16** `[19:15]`
    ///
    /// 16th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq16(&self) -> u32 {
        (self.bits & Rsqr1::SQ16_MASK) >> Rsqr1::SQ16_OFFSET
    }

    /// Read **SQ15** `[14:10]`
    ///
    /// 15th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq15(&self) -> u32 {
        (self.bits & Rsqr1::SQ15_MASK) >> Rsqr1::SQ15_OFFSET
    }

    /// Read **SQ14** `[9:5]`
    ///
    /// 14th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq14(&self) -> u32 {
        (self.bits & Rsqr1::SQ14_MASK) >> Rsqr1::SQ14_OFFSET
    }

    /// Read **SQ13** `[4:0]`
    ///
    /// 13th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq13(&self) -> u32 {
        (self.bits & Rsqr1::SQ13_MASK) >> Rsqr1::SQ13_OFFSET
    }

    /// Set **L** `[23:20]`
    ///
    /// Regular channel sequence
    /// length
    #[inline(always)]
    pub const fn with_l(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr1::L_MASK) | ((value << Rsqr1::L_OFFSET) & Rsqr1::L_MASK),
        }
    }

    /// Set **SQ16** `[19:15]`
    ///
    /// 16th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr1::SQ16_MASK)
                | ((value << Rsqr1::SQ16_OFFSET) & Rsqr1::SQ16_MASK),
        }
    }

    /// Set **SQ15** `[14:10]`
    ///
    /// 15th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr1::SQ15_MASK)
                | ((value << Rsqr1::SQ15_OFFSET) & Rsqr1::SQ15_MASK),
        }
    }

    /// Set **SQ14** `[9:5]`
    ///
    /// 14th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr1::SQ14_MASK)
                | ((value << Rsqr1::SQ14_OFFSET) & Rsqr1::SQ14_MASK),
        }
    }

    /// Set **SQ13** `[4:0]`
    ///
    /// 13th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr1::SQ13_MASK)
                | ((value << Rsqr1::SQ13_OFFSET) & Rsqr1::SQ13_MASK),
        }
    }
}

impl Default for Rsqr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rsqr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rsqr1Val {
        Rsqr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rsqr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rsqr1Val) -> Rsqr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSQR2 Register
///
/// regular sequence register 2
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SQ12** `[29:25]` (RW)
///
///   12th conversion in regular
///   sequence
///
/// - **SQ11** `[24:20]` (RW)
///
///   11th conversion in regular
///   sequence
///
/// - **SQ10** `[19:15]` (RW)
///
///   10th conversion in regular
///   sequence
///
/// - **SQ9** `[14:10]` (RW)
///
///   9th conversion in regular
///   sequence
///
/// - **SQ8** `[9:5]` (RW)
///
///   8th conversion in regular
///   sequence
///
/// - **SQ7** `[4:0]` (RW)
///
///   7th conversion in regular
///   sequence
#[repr(transparent)]
pub struct Rsqr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rsqr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rsqr2 {
    // ======== Field Constants ========

    /// **SQ12**
    ///
    /// 12th conversion in regular
    /// sequence
    ///
    /// Bits `[29:25]`, mask `0x3e000000`
    pub const SQ12_OFFSET: u32 = 25;
    pub const SQ12_MASK: u32 = 0x3e000000;

    /// **SQ11**
    ///
    /// 11th conversion in regular
    /// sequence
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const SQ11_OFFSET: u32 = 20;
    pub const SQ11_MASK: u32 = 0x01f00000;

    /// **SQ10**
    ///
    /// 10th conversion in regular
    /// sequence
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const SQ10_OFFSET: u32 = 15;
    pub const SQ10_MASK: u32 = 0x000f8000;

    /// **SQ9**
    ///
    /// 9th conversion in regular
    /// sequence
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SQ9_OFFSET: u32 = 10;
    pub const SQ9_MASK: u32 = 0x00007c00;

    /// **SQ8**
    ///
    /// 8th conversion in regular
    /// sequence
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SQ8_OFFSET: u32 = 5;
    pub const SQ8_MASK: u32 = 0x000003e0;

    /// **SQ7**
    ///
    /// 7th conversion in regular
    /// sequence
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SQ7_OFFSET: u32 = 0;
    pub const SQ7_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SQ12** `[29:25]` (RW)
    ///
    /// 12th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq12(&self) -> u32 {
        (self.value.read_bits() & Self::SQ12_MASK) >> Self::SQ12_OFFSET
    }

    /// Write **SQ12** `[29:25]` (RW)
    ///
    /// 12th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ12_MASK) | ((value << Self::SQ12_OFFSET) & Self::SQ12_MASK)
        });
    }

    /// **SQ11** `[24:20]` (RW)
    ///
    /// 11th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq11(&self) -> u32 {
        (self.value.read_bits() & Self::SQ11_MASK) >> Self::SQ11_OFFSET
    }

    /// Write **SQ11** `[24:20]` (RW)
    ///
    /// 11th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ11_MASK) | ((value << Self::SQ11_OFFSET) & Self::SQ11_MASK)
        });
    }

    /// **SQ10** `[19:15]` (RW)
    ///
    /// 10th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq10(&self) -> u32 {
        (self.value.read_bits() & Self::SQ10_MASK) >> Self::SQ10_OFFSET
    }

    /// Write **SQ10** `[19:15]` (RW)
    ///
    /// 10th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ10_MASK) | ((value << Self::SQ10_OFFSET) & Self::SQ10_MASK)
        });
    }

    /// **SQ9** `[14:10]` (RW)
    ///
    /// 9th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq9(&self) -> u32 {
        (self.value.read_bits() & Self::SQ9_MASK) >> Self::SQ9_OFFSET
    }

    /// Write **SQ9** `[14:10]` (RW)
    ///
    /// 9th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ9_MASK) | ((value << Self::SQ9_OFFSET) & Self::SQ9_MASK)
        });
    }

    /// **SQ8** `[9:5]` (RW)
    ///
    /// 8th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq8(&self) -> u32 {
        (self.value.read_bits() & Self::SQ8_MASK) >> Self::SQ8_OFFSET
    }

    /// Write **SQ8** `[9:5]` (RW)
    ///
    /// 8th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ8_MASK) | ((value << Self::SQ8_OFFSET) & Self::SQ8_MASK)
        });
    }

    /// **SQ7** `[4:0]` (RW)
    ///
    /// 7th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq7(&self) -> u32 {
        (self.value.read_bits() & Self::SQ7_MASK) >> Self::SQ7_OFFSET
    }

    /// Write **SQ7** `[4:0]` (RW)
    ///
    /// 7th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ7_MASK) | ((value << Self::SQ7_OFFSET) & Self::SQ7_MASK)
        });
    }
}

/// RSQR2 value type for batch read/modify/write operations
///
/// Use `rsqr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rsqr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rsqr2Val {
    bits: u32,
}

impl Rsqr2Val {
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

    /// Read **SQ12** `[29:25]`
    ///
    /// 12th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq12(&self) -> u32 {
        (self.bits & Rsqr2::SQ12_MASK) >> Rsqr2::SQ12_OFFSET
    }

    /// Read **SQ11** `[24:20]`
    ///
    /// 11th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq11(&self) -> u32 {
        (self.bits & Rsqr2::SQ11_MASK) >> Rsqr2::SQ11_OFFSET
    }

    /// Read **SQ10** `[19:15]`
    ///
    /// 10th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq10(&self) -> u32 {
        (self.bits & Rsqr2::SQ10_MASK) >> Rsqr2::SQ10_OFFSET
    }

    /// Read **SQ9** `[14:10]`
    ///
    /// 9th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq9(&self) -> u32 {
        (self.bits & Rsqr2::SQ9_MASK) >> Rsqr2::SQ9_OFFSET
    }

    /// Read **SQ8** `[9:5]`
    ///
    /// 8th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq8(&self) -> u32 {
        (self.bits & Rsqr2::SQ8_MASK) >> Rsqr2::SQ8_OFFSET
    }

    /// Read **SQ7** `[4:0]`
    ///
    /// 7th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq7(&self) -> u32 {
        (self.bits & Rsqr2::SQ7_MASK) >> Rsqr2::SQ7_OFFSET
    }

    /// Set **SQ12** `[29:25]`
    ///
    /// 12th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr2::SQ12_MASK)
                | ((value << Rsqr2::SQ12_OFFSET) & Rsqr2::SQ12_MASK),
        }
    }

    /// Set **SQ11** `[24:20]`
    ///
    /// 11th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr2::SQ11_MASK)
                | ((value << Rsqr2::SQ11_OFFSET) & Rsqr2::SQ11_MASK),
        }
    }

    /// Set **SQ10** `[19:15]`
    ///
    /// 10th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr2::SQ10_MASK)
                | ((value << Rsqr2::SQ10_OFFSET) & Rsqr2::SQ10_MASK),
        }
    }

    /// Set **SQ9** `[14:10]`
    ///
    /// 9th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr2::SQ9_MASK) | ((value << Rsqr2::SQ9_OFFSET) & Rsqr2::SQ9_MASK),
        }
    }

    /// Set **SQ8** `[9:5]`
    ///
    /// 8th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr2::SQ8_MASK) | ((value << Rsqr2::SQ8_OFFSET) & Rsqr2::SQ8_MASK),
        }
    }

    /// Set **SQ7** `[4:0]`
    ///
    /// 7th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr2::SQ7_MASK) | ((value << Rsqr2::SQ7_OFFSET) & Rsqr2::SQ7_MASK),
        }
    }
}

impl Default for Rsqr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rsqr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rsqr2Val {
        Rsqr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rsqr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rsqr2Val) -> Rsqr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSQR3 Register
///
/// regular sequence register 3
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SQ6** `[29:25]` (RW)
///
///   6th conversion in regular
///   sequence
///
/// - **SQ5** `[24:20]` (RW)
///
///   5th conversion in regular
///   sequence
///
/// - **SQ4** `[19:15]` (RW)
///
///   4th conversion in regular
///   sequence
///
/// - **SQ3** `[14:10]` (RW)
///
///   3rd conversion in regular
///   sequence
///
/// - **SQ2** `[9:5]` (RW)
///
///   2nd conversion in regular
///   sequence
///
/// - **SQ1** `[4:0]` (RW)
///
///   1st conversion in regular
///   sequence
#[repr(transparent)]
pub struct Rsqr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rsqr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rsqr3 {
    // ======== Field Constants ========

    /// **SQ6**
    ///
    /// 6th conversion in regular
    /// sequence
    ///
    /// Bits `[29:25]`, mask `0x3e000000`
    pub const SQ6_OFFSET: u32 = 25;
    pub const SQ6_MASK: u32 = 0x3e000000;

    /// **SQ5**
    ///
    /// 5th conversion in regular
    /// sequence
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const SQ5_OFFSET: u32 = 20;
    pub const SQ5_MASK: u32 = 0x01f00000;

    /// **SQ4**
    ///
    /// 4th conversion in regular
    /// sequence
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const SQ4_OFFSET: u32 = 15;
    pub const SQ4_MASK: u32 = 0x000f8000;

    /// **SQ3**
    ///
    /// 3rd conversion in regular
    /// sequence
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SQ3_OFFSET: u32 = 10;
    pub const SQ3_MASK: u32 = 0x00007c00;

    /// **SQ2**
    ///
    /// 2nd conversion in regular
    /// sequence
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SQ2_OFFSET: u32 = 5;
    pub const SQ2_MASK: u32 = 0x000003e0;

    /// **SQ1**
    ///
    /// 1st conversion in regular
    /// sequence
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SQ1_OFFSET: u32 = 0;
    pub const SQ1_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SQ6** `[29:25]` (RW)
    ///
    /// 6th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq6(&self) -> u32 {
        (self.value.read_bits() & Self::SQ6_MASK) >> Self::SQ6_OFFSET
    }

    /// Write **SQ6** `[29:25]` (RW)
    ///
    /// 6th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ6_MASK) | ((value << Self::SQ6_OFFSET) & Self::SQ6_MASK)
        });
    }

    /// **SQ5** `[24:20]` (RW)
    ///
    /// 5th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq5(&self) -> u32 {
        (self.value.read_bits() & Self::SQ5_MASK) >> Self::SQ5_OFFSET
    }

    /// Write **SQ5** `[24:20]` (RW)
    ///
    /// 5th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ5_MASK) | ((value << Self::SQ5_OFFSET) & Self::SQ5_MASK)
        });
    }

    /// **SQ4** `[19:15]` (RW)
    ///
    /// 4th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq4(&self) -> u32 {
        (self.value.read_bits() & Self::SQ4_MASK) >> Self::SQ4_OFFSET
    }

    /// Write **SQ4** `[19:15]` (RW)
    ///
    /// 4th conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ4_MASK) | ((value << Self::SQ4_OFFSET) & Self::SQ4_MASK)
        });
    }

    /// **SQ3** `[14:10]` (RW)
    ///
    /// 3rd conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq3(&self) -> u32 {
        (self.value.read_bits() & Self::SQ3_MASK) >> Self::SQ3_OFFSET
    }

    /// Write **SQ3** `[14:10]` (RW)
    ///
    /// 3rd conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ3_MASK) | ((value << Self::SQ3_OFFSET) & Self::SQ3_MASK)
        });
    }

    /// **SQ2** `[9:5]` (RW)
    ///
    /// 2nd conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq2(&self) -> u32 {
        (self.value.read_bits() & Self::SQ2_MASK) >> Self::SQ2_OFFSET
    }

    /// Write **SQ2** `[9:5]` (RW)
    ///
    /// 2nd conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ2_MASK) | ((value << Self::SQ2_OFFSET) & Self::SQ2_MASK)
        });
    }

    /// **SQ1** `[4:0]` (RW)
    ///
    /// 1st conversion in regular
    /// sequence
    #[inline(always)]
    pub fn sq1(&self) -> u32 {
        (self.value.read_bits() & Self::SQ1_MASK) >> Self::SQ1_OFFSET
    }

    /// Write **SQ1** `[4:0]` (RW)
    ///
    /// 1st conversion in regular
    /// sequence
    #[inline(always)]
    pub fn write_sq1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SQ1_MASK) | ((value << Self::SQ1_OFFSET) & Self::SQ1_MASK)
        });
    }
}

/// RSQR3 value type for batch read/modify/write operations
///
/// Use `rsqr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rsqr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rsqr3Val {
    bits: u32,
}

impl Rsqr3Val {
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

    /// Read **SQ6** `[29:25]`
    ///
    /// 6th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq6(&self) -> u32 {
        (self.bits & Rsqr3::SQ6_MASK) >> Rsqr3::SQ6_OFFSET
    }

    /// Read **SQ5** `[24:20]`
    ///
    /// 5th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq5(&self) -> u32 {
        (self.bits & Rsqr3::SQ5_MASK) >> Rsqr3::SQ5_OFFSET
    }

    /// Read **SQ4** `[19:15]`
    ///
    /// 4th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq4(&self) -> u32 {
        (self.bits & Rsqr3::SQ4_MASK) >> Rsqr3::SQ4_OFFSET
    }

    /// Read **SQ3** `[14:10]`
    ///
    /// 3rd conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq3(&self) -> u32 {
        (self.bits & Rsqr3::SQ3_MASK) >> Rsqr3::SQ3_OFFSET
    }

    /// Read **SQ2** `[9:5]`
    ///
    /// 2nd conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq2(&self) -> u32 {
        (self.bits & Rsqr3::SQ2_MASK) >> Rsqr3::SQ2_OFFSET
    }

    /// Read **SQ1** `[4:0]`
    ///
    /// 1st conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn sq1(&self) -> u32 {
        (self.bits & Rsqr3::SQ1_MASK) >> Rsqr3::SQ1_OFFSET
    }

    /// Set **SQ6** `[29:25]`
    ///
    /// 6th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr3::SQ6_MASK) | ((value << Rsqr3::SQ6_OFFSET) & Rsqr3::SQ6_MASK),
        }
    }

    /// Set **SQ5** `[24:20]`
    ///
    /// 5th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr3::SQ5_MASK) | ((value << Rsqr3::SQ5_OFFSET) & Rsqr3::SQ5_MASK),
        }
    }

    /// Set **SQ4** `[19:15]`
    ///
    /// 4th conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr3::SQ4_MASK) | ((value << Rsqr3::SQ4_OFFSET) & Rsqr3::SQ4_MASK),
        }
    }

    /// Set **SQ3** `[14:10]`
    ///
    /// 3rd conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr3::SQ3_MASK) | ((value << Rsqr3::SQ3_OFFSET) & Rsqr3::SQ3_MASK),
        }
    }

    /// Set **SQ2** `[9:5]`
    ///
    /// 2nd conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr3::SQ2_MASK) | ((value << Rsqr3::SQ2_OFFSET) & Rsqr3::SQ2_MASK),
        }
    }

    /// Set **SQ1** `[4:0]`
    ///
    /// 1st conversion in regular
    /// sequence
    #[inline(always)]
    pub const fn with_sq1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rsqr3::SQ1_MASK) | ((value << Rsqr3::SQ1_OFFSET) & Rsqr3::SQ1_MASK),
        }
    }
}

impl Default for Rsqr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rsqr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rsqr3Val {
        Rsqr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rsqr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rsqr3Val) -> Rsqr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ISQR Register
///
/// injected sequence register
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JL** `[21:20]` (RW)
///
///   Injected sequence length
///
/// - **JSQ4** `[19:15]` (RW)
///
///   4th conversion in injected
///   sequence
///
/// - **JSQ3** `[14:10]` (RW)
///
///   3rd conversion in injected
///   sequence
///
/// - **JSQ2** `[9:5]` (RW)
///
///   2nd conversion in injected
///   sequence
///
/// - **JSQ1** `[4:0]` (RW)
///
///   1st conversion in injected
///   sequence
#[repr(transparent)]
pub struct Isqr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Isqr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Isqr {
    // ======== Field Constants ========

    /// **JL**
    ///
    /// Injected sequence length
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const JL_OFFSET: u32 = 20;
    pub const JL_MASK: u32 = 0x00300000;

    /// **JSQ4**
    ///
    /// 4th conversion in injected
    /// sequence
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const JSQ4_OFFSET: u32 = 15;
    pub const JSQ4_MASK: u32 = 0x000f8000;

    /// **JSQ3**
    ///
    /// 3rd conversion in injected
    /// sequence
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const JSQ3_OFFSET: u32 = 10;
    pub const JSQ3_MASK: u32 = 0x00007c00;

    /// **JSQ2**
    ///
    /// 2nd conversion in injected
    /// sequence
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const JSQ2_OFFSET: u32 = 5;
    pub const JSQ2_MASK: u32 = 0x000003e0;

    /// **JSQ1**
    ///
    /// 1st conversion in injected
    /// sequence
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const JSQ1_OFFSET: u32 = 0;
    pub const JSQ1_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **JL** `[21:20]` (RW)
    ///
    /// Injected sequence length
    #[inline(always)]
    pub fn jl(&self) -> u32 {
        (self.value.read_bits() & Self::JL_MASK) >> Self::JL_OFFSET
    }

    /// Write **JL** `[21:20]` (RW)
    ///
    /// Injected sequence length
    #[inline(always)]
    pub fn write_jl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::JL_MASK) | ((value << Self::JL_OFFSET) & Self::JL_MASK));
    }

    /// **JSQ4** `[19:15]` (RW)
    ///
    /// 4th conversion in injected
    /// sequence
    #[inline(always)]
    pub fn jsq4(&self) -> u32 {
        (self.value.read_bits() & Self::JSQ4_MASK) >> Self::JSQ4_OFFSET
    }

    /// Write **JSQ4** `[19:15]` (RW)
    ///
    /// 4th conversion in injected
    /// sequence
    #[inline(always)]
    pub fn write_jsq4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JSQ4_MASK) | ((value << Self::JSQ4_OFFSET) & Self::JSQ4_MASK)
        });
    }

    /// **JSQ3** `[14:10]` (RW)
    ///
    /// 3rd conversion in injected
    /// sequence
    #[inline(always)]
    pub fn jsq3(&self) -> u32 {
        (self.value.read_bits() & Self::JSQ3_MASK) >> Self::JSQ3_OFFSET
    }

    /// Write **JSQ3** `[14:10]` (RW)
    ///
    /// 3rd conversion in injected
    /// sequence
    #[inline(always)]
    pub fn write_jsq3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JSQ3_MASK) | ((value << Self::JSQ3_OFFSET) & Self::JSQ3_MASK)
        });
    }

    /// **JSQ2** `[9:5]` (RW)
    ///
    /// 2nd conversion in injected
    /// sequence
    #[inline(always)]
    pub fn jsq2(&self) -> u32 {
        (self.value.read_bits() & Self::JSQ2_MASK) >> Self::JSQ2_OFFSET
    }

    /// Write **JSQ2** `[9:5]` (RW)
    ///
    /// 2nd conversion in injected
    /// sequence
    #[inline(always)]
    pub fn write_jsq2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JSQ2_MASK) | ((value << Self::JSQ2_OFFSET) & Self::JSQ2_MASK)
        });
    }

    /// **JSQ1** `[4:0]` (RW)
    ///
    /// 1st conversion in injected
    /// sequence
    #[inline(always)]
    pub fn jsq1(&self) -> u32 {
        (self.value.read_bits() & Self::JSQ1_MASK) >> Self::JSQ1_OFFSET
    }

    /// Write **JSQ1** `[4:0]` (RW)
    ///
    /// 1st conversion in injected
    /// sequence
    #[inline(always)]
    pub fn write_jsq1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JSQ1_MASK) | ((value << Self::JSQ1_OFFSET) & Self::JSQ1_MASK)
        });
    }
}

/// ISQR value type for batch read/modify/write operations
///
/// Use `isqr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `isqr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IsqrVal {
    bits: u32,
}

impl IsqrVal {
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

    /// Read **JL** `[21:20]`
    ///
    /// Injected sequence length
    #[inline(always)]
    pub const fn jl(&self) -> u32 {
        (self.bits & Isqr::JL_MASK) >> Isqr::JL_OFFSET
    }

    /// Read **JSQ4** `[19:15]`
    ///
    /// 4th conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn jsq4(&self) -> u32 {
        (self.bits & Isqr::JSQ4_MASK) >> Isqr::JSQ4_OFFSET
    }

    /// Read **JSQ3** `[14:10]`
    ///
    /// 3rd conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn jsq3(&self) -> u32 {
        (self.bits & Isqr::JSQ3_MASK) >> Isqr::JSQ3_OFFSET
    }

    /// Read **JSQ2** `[9:5]`
    ///
    /// 2nd conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn jsq2(&self) -> u32 {
        (self.bits & Isqr::JSQ2_MASK) >> Isqr::JSQ2_OFFSET
    }

    /// Read **JSQ1** `[4:0]`
    ///
    /// 1st conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn jsq1(&self) -> u32 {
        (self.bits & Isqr::JSQ1_MASK) >> Isqr::JSQ1_OFFSET
    }

    /// Set **JL** `[21:20]`
    ///
    /// Injected sequence length
    #[inline(always)]
    pub const fn with_jl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Isqr::JL_MASK) | ((value << Isqr::JL_OFFSET) & Isqr::JL_MASK),
        }
    }

    /// Set **JSQ4** `[19:15]`
    ///
    /// 4th conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn with_jsq4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Isqr::JSQ4_MASK) | ((value << Isqr::JSQ4_OFFSET) & Isqr::JSQ4_MASK),
        }
    }

    /// Set **JSQ3** `[14:10]`
    ///
    /// 3rd conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn with_jsq3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Isqr::JSQ3_MASK) | ((value << Isqr::JSQ3_OFFSET) & Isqr::JSQ3_MASK),
        }
    }

    /// Set **JSQ2** `[9:5]`
    ///
    /// 2nd conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn with_jsq2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Isqr::JSQ2_MASK) | ((value << Isqr::JSQ2_OFFSET) & Isqr::JSQ2_MASK),
        }
    }

    /// Set **JSQ1** `[4:0]`
    ///
    /// 1st conversion in injected
    /// sequence
    #[inline(always)]
    pub const fn with_jsq1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Isqr::JSQ1_MASK) | ((value << Isqr::JSQ1_OFFSET) & Isqr::JSQ1_MASK),
        }
    }
}

impl Default for IsqrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Isqr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IsqrVal {
        IsqrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IsqrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IsqrVal) -> IsqrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IDATAR1_CHGOFFSET Register
///
/// injected data register x_Charge data offset for injected channel x/TKEYx_CHGOFFSET
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JDATA8_15** `[15:8]` (RW)
///
///   Injected data
///
/// - **JDATA0_7_TKCGOFFSET** `[7:0]` (RW)
///
///   Injected data_Touch key charge data offset for injected channel x
#[repr(transparent)]
pub struct Idatar1Chgoffset {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idatar1Chgoffset {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idatar1Chgoffset {
    // ======== Field Constants ========

    /// **JDATA8_15**
    ///
    /// Injected data
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const JDATA8_15_OFFSET: u32 = 8;
    pub const JDATA8_15_MASK: u32 = 0x0000ff00;

    /// **JDATA0_7_TKCGOFFSET**
    ///
    /// Injected data_Touch key charge data offset for injected channel x
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const JDATA0_7_TKCGOFFSET_OFFSET: u32 = 0;
    pub const JDATA0_7_TKCGOFFSET_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **JDATA8_15** `[15:8]` (RW)
    ///
    /// Injected data
    #[inline(always)]
    pub fn jdata8_15(&self) -> u32 {
        (self.value.read_bits() & Self::JDATA8_15_MASK) >> Self::JDATA8_15_OFFSET
    }

    /// **JDATA0_7_TKCGOFFSET** `[7:0]` (RW)
    ///
    /// Injected data_Touch key charge data offset for injected channel x
    #[inline(always)]
    pub fn jdata0_7_tkcgoffset(&self) -> u32 {
        (self.value.read_bits() & Self::JDATA0_7_TKCGOFFSET_MASK)
            >> Self::JDATA0_7_TKCGOFFSET_OFFSET
    }
}

/// IDATAR1_CHGOFFSET value type for batch read/modify/write operations
///
/// Use `idatar1chgoffset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idatar1chgoffset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Idatar1ChgoffsetVal {
    bits: u32,
}

impl Idatar1ChgoffsetVal {
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

    /// Read **JDATA8_15** `[15:8]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn jdata8_15(&self) -> u32 {
        (self.bits & Idatar1Chgoffset::JDATA8_15_MASK) >> Idatar1Chgoffset::JDATA8_15_OFFSET
    }

    /// Read **JDATA0_7_TKCGOFFSET** `[7:0]`
    ///
    /// Injected data_Touch key charge data offset for injected channel x
    #[inline(always)]
    pub const fn jdata0_7_tkcgoffset(&self) -> u32 {
        (self.bits & Idatar1Chgoffset::JDATA0_7_TKCGOFFSET_MASK)
            >> Idatar1Chgoffset::JDATA0_7_TKCGOFFSET_OFFSET
    }

    /// Set **JDATA8_15** `[15:8]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn with_jdata8_15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idatar1Chgoffset::JDATA8_15_MASK)
                | ((value << Idatar1Chgoffset::JDATA8_15_OFFSET)
                    & Idatar1Chgoffset::JDATA8_15_MASK),
        }
    }

    /// Set **JDATA0_7_TKCGOFFSET** `[7:0]`
    ///
    /// Injected data_Touch key charge data offset for injected channel x
    #[inline(always)]
    pub const fn with_jdata0_7_tkcgoffset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idatar1Chgoffset::JDATA0_7_TKCGOFFSET_MASK)
                | ((value << Idatar1Chgoffset::JDATA0_7_TKCGOFFSET_OFFSET)
                    & Idatar1Chgoffset::JDATA0_7_TKCGOFFSET_MASK),
        }
    }
}

impl Default for Idatar1ChgoffsetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Idatar1Chgoffset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Idatar1ChgoffsetVal {
        Idatar1ChgoffsetVal::from_bits(self.value.read_bits())
    }
}

/// IDATAR2 Register
///
/// injected data register x
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JDATA** `[15:0]` (RW)
///
///   Injected data
#[repr(transparent)]
pub struct Idatar2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idatar2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idatar2 {
    // ======== Field Constants ========

    /// **JDATA**
    ///
    /// Injected data
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const JDATA_OFFSET: u32 = 0;
    pub const JDATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **JDATA** `[15:0]` (RW)
    ///
    /// Injected data
    #[inline(always)]
    pub fn jdata(&self) -> u32 {
        (self.value.read_bits() & Self::JDATA_MASK) >> Self::JDATA_OFFSET
    }
}

/// IDATAR2 value type for batch read/modify/write operations
///
/// Use `idatar2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idatar2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Idatar2Val {
    bits: u32,
}

impl Idatar2Val {
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

    /// Read **JDATA** `[15:0]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn jdata(&self) -> u32 {
        (self.bits & Idatar2::JDATA_MASK) >> Idatar2::JDATA_OFFSET
    }

    /// Set **JDATA** `[15:0]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn with_jdata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idatar2::JDATA_MASK)
                | ((value << Idatar2::JDATA_OFFSET) & Idatar2::JDATA_MASK),
        }
    }
}

impl Default for Idatar2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Idatar2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Idatar2Val {
        Idatar2Val::from_bits(self.value.read_bits())
    }
}

/// IDATAR3 Register
///
/// injected data register x
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JDATA** `[15:0]` (RW)
///
///   Injected data
#[repr(transparent)]
pub struct Idatar3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idatar3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idatar3 {
    // ======== Field Constants ========

    /// **JDATA**
    ///
    /// Injected data
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const JDATA_OFFSET: u32 = 0;
    pub const JDATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **JDATA** `[15:0]` (RW)
    ///
    /// Injected data
    #[inline(always)]
    pub fn jdata(&self) -> u32 {
        (self.value.read_bits() & Self::JDATA_MASK) >> Self::JDATA_OFFSET
    }
}

/// IDATAR3 value type for batch read/modify/write operations
///
/// Use `idatar3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idatar3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Idatar3Val {
    bits: u32,
}

impl Idatar3Val {
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

    /// Read **JDATA** `[15:0]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn jdata(&self) -> u32 {
        (self.bits & Idatar3::JDATA_MASK) >> Idatar3::JDATA_OFFSET
    }

    /// Set **JDATA** `[15:0]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn with_jdata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idatar3::JDATA_MASK)
                | ((value << Idatar3::JDATA_OFFSET) & Idatar3::JDATA_MASK),
        }
    }
}

impl Default for Idatar3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Idatar3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Idatar3Val {
        Idatar3Val::from_bits(self.value.read_bits())
    }
}

/// IDATAR4 Register
///
/// injected data register x
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JDATA** `[15:0]` (RW)
///
///   Injected data
#[repr(transparent)]
pub struct Idatar4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Idatar4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Idatar4 {
    // ======== Field Constants ========

    /// **JDATA**
    ///
    /// Injected data
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const JDATA_OFFSET: u32 = 0;
    pub const JDATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **JDATA** `[15:0]` (RW)
    ///
    /// Injected data
    #[inline(always)]
    pub fn jdata(&self) -> u32 {
        (self.value.read_bits() & Self::JDATA_MASK) >> Self::JDATA_OFFSET
    }
}

/// IDATAR4 value type for batch read/modify/write operations
///
/// Use `idatar4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idatar4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Idatar4Val {
    bits: u32,
}

impl Idatar4Val {
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

    /// Read **JDATA** `[15:0]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn jdata(&self) -> u32 {
        (self.bits & Idatar4::JDATA_MASK) >> Idatar4::JDATA_OFFSET
    }

    /// Set **JDATA** `[15:0]`
    ///
    /// Injected data
    #[inline(always)]
    pub const fn with_jdata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Idatar4::JDATA_MASK)
                | ((value << Idatar4::JDATA_OFFSET) & Idatar4::JDATA_MASK),
        }
    }
}

impl Default for Idatar4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Idatar4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Idatar4Val {
        Idatar4Val::from_bits(self.value.read_bits())
    }
}

/// RDATAR_DR_ACT_DCG Register
///
/// regular data register_start and discharge time register/TKEYx_DR/TKEYx_ACT_DCG
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DATA8_15** `[15:8]` (RW)
///
///   Regular data
///
/// - **DATA0_7_TKACT_DCG** `[7:0]` (RW)
///
///   Regular data_Touch key start and discharge time register
#[repr(transparent)]
pub struct RdatarDrActDcg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdatarDrActDcg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdatarDrActDcg {
    // ======== Field Constants ========

    /// **DATA8_15**
    ///
    /// Regular data
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const DATA8_15_OFFSET: u32 = 8;
    pub const DATA8_15_MASK: u32 = 0x0000ff00;

    /// **DATA0_7_TKACT_DCG**
    ///
    /// Regular data_Touch key start and discharge time register
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA0_7_TKACT_DCG_OFFSET: u32 = 0;
    pub const DATA0_7_TKACT_DCG_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA8_15** `[15:8]` (RW)
    ///
    /// Regular data
    #[inline(always)]
    pub fn data8_15(&self) -> u32 {
        (self.value.read_bits() & Self::DATA8_15_MASK) >> Self::DATA8_15_OFFSET
    }

    /// Write **DATA8_15** `[15:8]` (RW)
    ///
    /// Regular data
    #[inline(always)]
    pub fn write_data8_15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA8_15_MASK) | ((value << Self::DATA8_15_OFFSET) & Self::DATA8_15_MASK)
        });
    }

    /// **DATA0_7_TKACT_DCG** `[7:0]` (RW)
    ///
    /// Regular data_Touch key start and discharge time register
    #[inline(always)]
    pub fn data0_7_tkact_dcg(&self) -> u32 {
        (self.value.read_bits() & Self::DATA0_7_TKACT_DCG_MASK) >> Self::DATA0_7_TKACT_DCG_OFFSET
    }

    /// Write **DATA0_7_TKACT_DCG** `[7:0]` (RW)
    ///
    /// Regular data_Touch key start and discharge time register
    #[inline(always)]
    pub fn write_data0_7_tkact_dcg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA0_7_TKACT_DCG_MASK)
                | ((value << Self::DATA0_7_TKACT_DCG_OFFSET) & Self::DATA0_7_TKACT_DCG_MASK)
        });
    }
}

/// RDATAR_DR_ACT_DCG value type for batch read/modify/write operations
///
/// Use `rdatardractdcg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdatardractdcg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdatarDrActDcgVal {
    bits: u32,
}

impl RdatarDrActDcgVal {
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

    /// Read **DATA8_15** `[15:8]`
    ///
    /// Regular data
    #[inline(always)]
    pub const fn data8_15(&self) -> u32 {
        (self.bits & RdatarDrActDcg::DATA8_15_MASK) >> RdatarDrActDcg::DATA8_15_OFFSET
    }

    /// Read **DATA0_7_TKACT_DCG** `[7:0]`
    ///
    /// Regular data_Touch key start and discharge time register
    #[inline(always)]
    pub const fn data0_7_tkact_dcg(&self) -> u32 {
        (self.bits & RdatarDrActDcg::DATA0_7_TKACT_DCG_MASK)
            >> RdatarDrActDcg::DATA0_7_TKACT_DCG_OFFSET
    }

    /// Set **DATA8_15** `[15:8]`
    ///
    /// Regular data
    #[inline(always)]
    pub const fn with_data8_15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdatarDrActDcg::DATA8_15_MASK)
                | ((value << RdatarDrActDcg::DATA8_15_OFFSET) & RdatarDrActDcg::DATA8_15_MASK),
        }
    }

    /// Set **DATA0_7_TKACT_DCG** `[7:0]`
    ///
    /// Regular data_Touch key start and discharge time register
    #[inline(always)]
    pub const fn with_data0_7_tkact_dcg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdatarDrActDcg::DATA0_7_TKACT_DCG_MASK)
                | ((value << RdatarDrActDcg::DATA0_7_TKACT_DCG_OFFSET)
                    & RdatarDrActDcg::DATA0_7_TKACT_DCG_MASK),
        }
    }
}

impl Default for RdatarDrActDcgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RdatarDrActDcg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdatarDrActDcgVal {
        RdatarDrActDcgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RdatarDrActDcgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RdatarDrActDcgVal) -> RdatarDrActDcgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// status register
    pub statr: Statr,
    /// 0x0004
    /// control register 1
    pub ctlr1: Ctlr1,
    /// 0x0008
    /// control register 2
    pub ctlr2: Ctlr2,
    /// 0x000c
    /// sample time register 1/TKEYx_CHARGE1
    pub samptr1_charge1: Samptr1Charge1,
    /// 0x0010
    /// sample time register 2/TKEYx_CHARGE2
    pub samptr2_charge2: Samptr2Charge2,
    /// 0x0014
    /// injected channel data offset register
    /// x
    pub iofr1: Iofr1,
    /// 0x0018
    /// injected channel data offset register
    /// x
    pub iofr2: Iofr2,
    /// 0x001c
    /// injected channel data offset register
    /// x
    pub iofr3: Iofr3,
    /// 0x0020
    /// injected channel data offset register
    /// x
    pub iofr4: Iofr4,
    /// 0x0024
    /// watchdog higher threshold
    /// register
    pub wdhtr: Wdhtr,
    /// 0x0028
    /// watchdog lower threshold
    /// register
    pub wdltr: Wdltr,
    /// 0x002c
    /// regular sequence register 1
    pub rsqr1: Rsqr1,
    /// 0x0030
    /// regular sequence register 2
    pub rsqr2: Rsqr2,
    /// 0x0034
    /// regular sequence register 3
    pub rsqr3: Rsqr3,
    /// 0x0038
    /// injected sequence register
    pub isqr: Isqr,
    /// 0x003c
    /// injected data register x_Charge data offset for injected channel x/TKEYx_CHGOFFSET
    pub idatar1_chgoffset: Idatar1Chgoffset,
    /// 0x0040
    /// injected data register x
    pub idatar2: Idatar2,
    /// 0x0044
    /// injected data register x
    pub idatar3: Idatar3,
    /// 0x0048
    /// injected data register x
    pub idatar4: Idatar4,
    /// 0x004c
    /// regular data register_start and discharge time register/TKEYx_DR/TKEYx_ACT_DCG
    pub rdatar_dr_act_dcg: RdatarDrActDcg,
}
