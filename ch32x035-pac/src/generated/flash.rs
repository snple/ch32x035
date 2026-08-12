//! FLASH
//! FLASH

#![allow(dead_code)]

pub const BASE: u32 = 0x40022000;

/// ACTLR Register
///
/// Flash ACTL register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LATENCY** `[1:0]` (RW)
///
///   LATENCY
#[repr(transparent)]
pub struct Actlr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Actlr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Actlr {
    // ======== Field Constants ========

    /// **LATENCY**
    ///
    /// LATENCY
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const LATENCY_OFFSET: u32 = 0;
    pub const LATENCY_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **LATENCY** `[1:0]` (RW)
    ///
    /// LATENCY
    #[inline(always)]
    pub fn latency(&self) -> u32 {
        (self.value.read_bits() & Self::LATENCY_MASK) >> Self::LATENCY_OFFSET
    }

    /// Write **LATENCY** `[1:0]` (RW)
    ///
    /// LATENCY
    #[inline(always)]
    pub fn write_latency(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LATENCY_MASK) | ((value << Self::LATENCY_OFFSET) & Self::LATENCY_MASK)
        });
    }
}

/// ACTLR value type for batch read/modify/write operations
///
/// Use `actlr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `actlr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ActlrVal {
    bits: u32,
}

impl ActlrVal {
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

    /// Read **LATENCY** `[1:0]`
    ///
    /// LATENCY
    #[inline(always)]
    pub const fn latency(&self) -> u32 {
        (self.bits & Actlr::LATENCY_MASK) >> Actlr::LATENCY_OFFSET
    }

    /// Set **LATENCY** `[1:0]`
    ///
    /// LATENCY
    #[inline(always)]
    pub const fn with_latency(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Actlr::LATENCY_MASK)
                | ((value << Actlr::LATENCY_OFFSET) & Actlr::LATENCY_MASK),
        }
    }
}

impl Default for ActlrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Actlr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ActlrVal {
        ActlrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ActlrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ActlrVal) -> ActlrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEYR Register
///
/// Flash key register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **KEYR** `[31:0]` (RW)
///
///   FPEC key
#[repr(transparent)]
pub struct Keyr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Keyr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Keyr {
    // ======== Field Constants ========

    /// **KEYR**
    ///
    /// FPEC key
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEYR_OFFSET: u32 = 0;
    pub const KEYR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEYR** `[31:0]` (RW)
    ///
    /// FPEC key
    #[inline(always)]
    pub fn keyr(&self) -> u32 {
        (self.value.read_bits() & Self::KEYR_MASK) >> Self::KEYR_OFFSET
    }
}

/// KEYR value type for batch read/modify/write operations
///
/// Use `keyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `keyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct KeyrVal {
    bits: u32,
}

impl KeyrVal {
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

    /// Read **KEYR** `[31:0]`
    ///
    /// FPEC key
    #[inline(always)]
    pub const fn keyr(&self) -> u32 {
        (self.bits & Keyr::KEYR_MASK) >> Keyr::KEYR_OFFSET
    }

    /// Set **KEYR** `[31:0]`
    ///
    /// FPEC key
    #[inline(always)]
    pub const fn with_keyr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Keyr::KEYR_MASK) | ((value << Keyr::KEYR_OFFSET) & Keyr::KEYR_MASK),
        }
    }
}

impl Default for KeyrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Keyr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: KeyrVal) {
        self.value.write_bits(val.bits())
    }
}

/// OBKEYR Register
///
/// Flash option key register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OPTKEY** `[31:0]` (RW)
///
///   Option byte key
#[repr(transparent)]
pub struct Obkeyr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Obkeyr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Obkeyr {
    // ======== Field Constants ========

    /// **OPTKEY**
    ///
    /// Option byte key
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const OPTKEY_OFFSET: u32 = 0;
    pub const OPTKEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **OPTKEY** `[31:0]` (RW)
    ///
    /// Option byte key
    #[inline(always)]
    pub fn optkey(&self) -> u32 {
        (self.value.read_bits() & Self::OPTKEY_MASK) >> Self::OPTKEY_OFFSET
    }
}

/// OBKEYR value type for batch read/modify/write operations
///
/// Use `obkeyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `obkeyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ObkeyrVal {
    bits: u32,
}

impl ObkeyrVal {
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

    /// Read **OPTKEY** `[31:0]`
    ///
    /// Option byte key
    #[inline(always)]
    pub const fn optkey(&self) -> u32 {
        (self.bits & Obkeyr::OPTKEY_MASK) >> Obkeyr::OPTKEY_OFFSET
    }

    /// Set **OPTKEY** `[31:0]`
    ///
    /// Option byte key
    #[inline(always)]
    pub const fn with_optkey(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Obkeyr::OPTKEY_MASK)
                | ((value << Obkeyr::OPTKEY_OFFSET) & Obkeyr::OPTKEY_MASK),
        }
    }
}

impl Default for ObkeyrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Obkeyr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ObkeyrVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATR Register
///
/// Status register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOT_LOCK** `[15]` (RW)
///
///   BOOT_LOCK
///
/// - **BOOT_MODE** `[14]` (RW)
///
///   BOOT_MODE
///
/// - **BOOT_STATUS** `[13]` (RO)
///
///   BOOT_STATUS
///
/// - **BOOT_AVA** `[12]` (RO)
///
///   BOOT_AVA
///
/// - **TURBO** `[7]` (RO)
///
///   TURBO mode start
///
/// - **FWAKE_FLAG** `[6]` (RW)
///
///   FWAKE_FLAG mode start
///
/// - **EOP** `[5]` (RW)
///
///   End of operation
///
/// - **WRPRTERR** `[4]` (RW)
///
///   Write protection error
///
/// - **BSY** `[0]` (RO)
///
///   Busy
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

    /// **BOOT_LOCK**
    ///
    /// BOOT_LOCK
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const BOOT_LOCK_OFFSET: u32 = 15;
    pub const BOOT_LOCK_MASK: u32 = 0x00008000;
    pub const BOOT_LOCK: u32 = 1 << 15;

    /// **BOOT_MODE**
    ///
    /// BOOT_MODE
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const BOOT_MODE_OFFSET: u32 = 14;
    pub const BOOT_MODE_MASK: u32 = 0x00004000;
    pub const BOOT_MODE: u32 = 1 << 14;

    /// **BOOT_STATUS**
    ///
    /// BOOT_STATUS
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const BOOT_STATUS_OFFSET: u32 = 13;
    pub const BOOT_STATUS_MASK: u32 = 0x00002000;
    pub const BOOT_STATUS: u32 = 1 << 13;

    /// **BOOT_AVA**
    ///
    /// BOOT_AVA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BOOT_AVA_OFFSET: u32 = 12;
    pub const BOOT_AVA_MASK: u32 = 0x00001000;
    pub const BOOT_AVA: u32 = 1 << 12;

    /// **TURBO**
    ///
    /// TURBO mode start
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TURBO_OFFSET: u32 = 7;
    pub const TURBO_MASK: u32 = 0x00000080;
    pub const TURBO: u32 = 1 << 7;

    /// **FWAKE_FLAG**
    ///
    /// FWAKE_FLAG mode start
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FWAKE_FLAG_OFFSET: u32 = 6;
    pub const FWAKE_FLAG_MASK: u32 = 0x00000040;
    pub const FWAKE_FLAG: u32 = 1 << 6;

    /// **EOP**
    ///
    /// End of operation
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EOP_OFFSET: u32 = 5;
    pub const EOP_MASK: u32 = 0x00000020;
    pub const EOP: u32 = 1 << 5;

    /// **WRPRTERR**
    ///
    /// Write protection error
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const WRPRTERR_OFFSET: u32 = 4;
    pub const WRPRTERR_MASK: u32 = 0x00000010;
    pub const WRPRTERR: u32 = 1 << 4;

    /// **BSY**
    ///
    /// Busy
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BSY_OFFSET: u32 = 0;
    pub const BSY_MASK: u32 = 0x00000001;
    pub const BSY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BOOT_LOCK** `[15]` (RW)
    ///
    /// BOOT_LOCK
    #[inline(always)]
    pub fn boot_lock(&self) -> bool {
        (self.value.read_bits() & Self::BOOT_LOCK_MASK) != 0
    }

    /// Set BOOT_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_boot_lock(&self) {
        self.value.modify_bits(|r| r | Self::BOOT_LOCK_MASK);
    }

    /// Clear BOOT_LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_boot_lock(&self) {
        self.value.modify_bits(|r| r & !Self::BOOT_LOCK_MASK);
    }

    /// **BOOT_MODE** `[14]` (RW)
    ///
    /// BOOT_MODE
    #[inline(always)]
    pub fn boot_mode(&self) -> bool {
        (self.value.read_bits() & Self::BOOT_MODE_MASK) != 0
    }

    /// Set BOOT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_boot_mode(&self) {
        self.value.modify_bits(|r| r | Self::BOOT_MODE_MASK);
    }

    /// Clear BOOT_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_boot_mode(&self) {
        self.value.modify_bits(|r| r & !Self::BOOT_MODE_MASK);
    }

    /// **BOOT_STATUS** `[13]` (RO)
    ///
    /// BOOT_STATUS
    #[inline(always)]
    pub fn boot_status(&self) -> bool {
        (self.value.read_bits() & Self::BOOT_STATUS_MASK) != 0
    }

    /// **BOOT_AVA** `[12]` (RO)
    ///
    /// BOOT_AVA
    #[inline(always)]
    pub fn boot_ava(&self) -> bool {
        (self.value.read_bits() & Self::BOOT_AVA_MASK) != 0
    }

    /// **TURBO** `[7]` (RO)
    ///
    /// TURBO mode start
    #[inline(always)]
    pub fn turbo(&self) -> bool {
        (self.value.read_bits() & Self::TURBO_MASK) != 0
    }

    /// **FWAKE_FLAG** `[6]` (RW)
    ///
    /// FWAKE_FLAG mode start
    #[inline(always)]
    pub fn fwake_flag(&self) -> bool {
        (self.value.read_bits() & Self::FWAKE_FLAG_MASK) != 0
    }

    /// Set FWAKE_FLAG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fwake_flag(&self) {
        self.value.modify_bits(|r| r | Self::FWAKE_FLAG_MASK);
    }

    /// Clear FWAKE_FLAG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fwake_flag(&self) {
        self.value.modify_bits(|r| r & !Self::FWAKE_FLAG_MASK);
    }

    /// **EOP** `[5]` (RW)
    ///
    /// End of operation
    #[inline(always)]
    pub fn eop(&self) -> bool {
        (self.value.read_bits() & Self::EOP_MASK) != 0
    }

    /// Set EOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eop(&self) {
        self.value.modify_bits(|r| r | Self::EOP_MASK);
    }

    /// Clear EOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eop(&self) {
        self.value.modify_bits(|r| r & !Self::EOP_MASK);
    }

    /// **WRPRTERR** `[4]` (RW)
    ///
    /// Write protection error
    #[inline(always)]
    pub fn wrprterr(&self) -> bool {
        (self.value.read_bits() & Self::WRPRTERR_MASK) != 0
    }

    /// Set WRPRTERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wrprterr(&self) {
        self.value.modify_bits(|r| r | Self::WRPRTERR_MASK);
    }

    /// Clear WRPRTERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wrprterr(&self) {
        self.value.modify_bits(|r| r & !Self::WRPRTERR_MASK);
    }

    /// **BSY** `[0]` (RO)
    ///
    /// Busy
    #[inline(always)]
    pub fn bsy(&self) -> bool {
        (self.value.read_bits() & Self::BSY_MASK) != 0
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

    /// Read **BOOT_LOCK** `[15:15]`
    ///
    /// BOOT_LOCK
    #[inline(always)]
    pub const fn boot_lock(&self) -> bool {
        (self.bits & Statr::BOOT_LOCK_MASK) != 0
    }

    /// Read **BOOT_MODE** `[14:14]`
    ///
    /// BOOT_MODE
    #[inline(always)]
    pub const fn boot_mode(&self) -> bool {
        (self.bits & Statr::BOOT_MODE_MASK) != 0
    }

    /// Read **BOOT_STATUS** `[13:13]`
    ///
    /// BOOT_STATUS
    #[inline(always)]
    pub const fn boot_status(&self) -> bool {
        (self.bits & Statr::BOOT_STATUS_MASK) != 0
    }

    /// Read **BOOT_AVA** `[12:12]`
    ///
    /// BOOT_AVA
    #[inline(always)]
    pub const fn boot_ava(&self) -> bool {
        (self.bits & Statr::BOOT_AVA_MASK) != 0
    }

    /// Read **TURBO** `[7:7]`
    ///
    /// TURBO mode start
    #[inline(always)]
    pub const fn turbo(&self) -> bool {
        (self.bits & Statr::TURBO_MASK) != 0
    }

    /// Read **FWAKE_FLAG** `[6:6]`
    ///
    /// FWAKE_FLAG mode start
    #[inline(always)]
    pub const fn fwake_flag(&self) -> bool {
        (self.bits & Statr::FWAKE_FLAG_MASK) != 0
    }

    /// Read **EOP** `[5:5]`
    ///
    /// End of operation
    #[inline(always)]
    pub const fn eop(&self) -> bool {
        (self.bits & Statr::EOP_MASK) != 0
    }

    /// Read **WRPRTERR** `[4:4]`
    ///
    /// Write protection error
    #[inline(always)]
    pub const fn wrprterr(&self) -> bool {
        (self.bits & Statr::WRPRTERR_MASK) != 0
    }

    /// Read **BSY** `[0:0]`
    ///
    /// Busy
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        (self.bits & Statr::BSY_MASK) != 0
    }

    /// Set **BOOT_LOCK** `[15:15]`
    ///
    /// BOOT_LOCK
    #[inline(always)]
    pub const fn with_boot_lock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::BOOT_LOCK_MASK
            } else {
                self.bits & !Statr::BOOT_LOCK_MASK
            },
        }
    }

    /// Set **BOOT_MODE** `[14:14]`
    ///
    /// BOOT_MODE
    #[inline(always)]
    pub const fn with_boot_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::BOOT_MODE_MASK
            } else {
                self.bits & !Statr::BOOT_MODE_MASK
            },
        }
    }

    /// Set **FWAKE_FLAG** `[6:6]`
    ///
    /// FWAKE_FLAG mode start
    #[inline(always)]
    pub const fn with_fwake_flag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::FWAKE_FLAG_MASK
            } else {
                self.bits & !Statr::FWAKE_FLAG_MASK
            },
        }
    }

    /// Set **EOP** `[5:5]`
    ///
    /// End of operation
    #[inline(always)]
    pub const fn with_eop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::EOP_MASK
            } else {
                self.bits & !Statr::EOP_MASK
            },
        }
    }

    /// Set **WRPRTERR** `[4:4]`
    ///
    /// Write protection error
    #[inline(always)]
    pub const fn with_wrprterr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::WRPRTERR_MASK
            } else {
                self.bits & !Statr::WRPRTERR_MASK
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

/// CTLR Register
///
/// Control register
///
/// Address offset: 0x0010
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **BER32** `[23]` (RW)
///
///   BER32
///
/// - **BUFRST** `[19]` (RW)
///
///   BUFRST
///
/// - **BUFLOAD** `[18]` (RW)
///
///   BUFLOAD
///
/// - **FTER** `[17]` (RW)
///
///   Fast erase
///
/// - **FTPG** `[16]` (RW)
///
///   Fast programming
///
/// - **FLOCK** `[15]` (RW)
///
///   Fast programmable lock
///
/// - **FWAKEIE** `[13]` (RW)
///
///   Wakeup enable
///
/// - **EOPIE** `[12]` (RW)
///
///   End of operation interrupt
///   enable
///
/// - **ERRIE** `[10]` (RW)
///
///   Error interrupt enable
///
/// - **OBWRE** `[9]` (RW)
///
///   Option bytes write enable
///
/// - **LOCK** `[7]` (RW)
///
///   Lock
///
/// - **STRT** `[6]` (RW)
///
///   Start
///
/// - **OBER** `[5]` (RW)
///
///   Option byte erase
///
/// - **OBPG** `[4]` (RW)
///
///   Option byte programming
///
/// - **MER** `[2]` (RW)
///
///   Mass Erase
///
/// - **PER** `[1]` (RW)
///
///   Page Erase
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

    /// **BER32**
    ///
    /// BER32
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const BER32_OFFSET: u32 = 23;
    pub const BER32_MASK: u32 = 0x00800000;
    pub const BER32: u32 = 1 << 23;

    /// **BUFRST**
    ///
    /// BUFRST
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const BUFRST_OFFSET: u32 = 19;
    pub const BUFRST_MASK: u32 = 0x00080000;
    pub const BUFRST: u32 = 1 << 19;

    /// **BUFLOAD**
    ///
    /// BUFLOAD
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const BUFLOAD_OFFSET: u32 = 18;
    pub const BUFLOAD_MASK: u32 = 0x00040000;
    pub const BUFLOAD: u32 = 1 << 18;

    /// **FTER**
    ///
    /// Fast erase
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const FTER_OFFSET: u32 = 17;
    pub const FTER_MASK: u32 = 0x00020000;
    pub const FTER: u32 = 1 << 17;

    /// **FTPG**
    ///
    /// Fast programming
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const FTPG_OFFSET: u32 = 16;
    pub const FTPG_MASK: u32 = 0x00010000;
    pub const FTPG: u32 = 1 << 16;

    /// **FLOCK**
    ///
    /// Fast programmable lock
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FLOCK_OFFSET: u32 = 15;
    pub const FLOCK_MASK: u32 = 0x00008000;
    pub const FLOCK: u32 = 1 << 15;

    /// **FWAKEIE**
    ///
    /// Wakeup enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FWAKEIE_OFFSET: u32 = 13;
    pub const FWAKEIE_MASK: u32 = 0x00002000;
    pub const FWAKEIE: u32 = 1 << 13;

    /// **EOPIE**
    ///
    /// End of operation interrupt
    /// enable
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EOPIE_OFFSET: u32 = 12;
    pub const EOPIE_MASK: u32 = 0x00001000;
    pub const EOPIE: u32 = 1 << 12;

    /// **ERRIE**
    ///
    /// Error interrupt enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ERRIE_OFFSET: u32 = 10;
    pub const ERRIE_MASK: u32 = 0x00000400;
    pub const ERRIE: u32 = 1 << 10;

    /// **OBWRE**
    ///
    /// Option bytes write enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OBWRE_OFFSET: u32 = 9;
    pub const OBWRE_MASK: u32 = 0x00000200;
    pub const OBWRE: u32 = 1 << 9;

    /// **LOCK**
    ///
    /// Lock
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LOCK_OFFSET: u32 = 7;
    pub const LOCK_MASK: u32 = 0x00000080;
    pub const LOCK: u32 = 1 << 7;

    /// **STRT**
    ///
    /// Start
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const STRT_OFFSET: u32 = 6;
    pub const STRT_MASK: u32 = 0x00000040;
    pub const STRT: u32 = 1 << 6;

    /// **OBER**
    ///
    /// Option byte erase
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OBER_OFFSET: u32 = 5;
    pub const OBER_MASK: u32 = 0x00000020;
    pub const OBER: u32 = 1 << 5;

    /// **OBPG**
    ///
    /// Option byte programming
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OBPG_OFFSET: u32 = 4;
    pub const OBPG_MASK: u32 = 0x00000010;
    pub const OBPG: u32 = 1 << 4;

    /// **MER**
    ///
    /// Mass Erase
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MER_OFFSET: u32 = 2;
    pub const MER_MASK: u32 = 0x00000004;
    pub const MER: u32 = 1 << 2;

    /// **PER**
    ///
    /// Page Erase
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PER_OFFSET: u32 = 1;
    pub const PER_MASK: u32 = 0x00000002;
    pub const PER: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **BER32** `[23]` (RW)
    ///
    /// BER32
    #[inline(always)]
    pub fn ber32(&self) -> bool {
        (self.value.read_bits() & Self::BER32_MASK) != 0
    }

    /// Set BER32 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ber32(&self) {
        self.value.modify_bits(|r| r | Self::BER32_MASK);
    }

    /// Clear BER32 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ber32(&self) {
        self.value.modify_bits(|r| r & !Self::BER32_MASK);
    }

    /// **BUFRST** `[19]` (RW)
    ///
    /// BUFRST
    #[inline(always)]
    pub fn bufrst(&self) -> bool {
        (self.value.read_bits() & Self::BUFRST_MASK) != 0
    }

    /// Set BUFRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bufrst(&self) {
        self.value.modify_bits(|r| r | Self::BUFRST_MASK);
    }

    /// Clear BUFRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bufrst(&self) {
        self.value.modify_bits(|r| r & !Self::BUFRST_MASK);
    }

    /// **BUFLOAD** `[18]` (RW)
    ///
    /// BUFLOAD
    #[inline(always)]
    pub fn bufload(&self) -> bool {
        (self.value.read_bits() & Self::BUFLOAD_MASK) != 0
    }

    /// Set BUFLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bufload(&self) {
        self.value.modify_bits(|r| r | Self::BUFLOAD_MASK);
    }

    /// Clear BUFLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bufload(&self) {
        self.value.modify_bits(|r| r & !Self::BUFLOAD_MASK);
    }

    /// **FTER** `[17]` (RW)
    ///
    /// Fast erase
    #[inline(always)]
    pub fn fter(&self) -> bool {
        (self.value.read_bits() & Self::FTER_MASK) != 0
    }

    /// Set FTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fter(&self) {
        self.value.modify_bits(|r| r | Self::FTER_MASK);
    }

    /// Clear FTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fter(&self) {
        self.value.modify_bits(|r| r & !Self::FTER_MASK);
    }

    /// **FTPG** `[16]` (RW)
    ///
    /// Fast programming
    #[inline(always)]
    pub fn ftpg(&self) -> bool {
        (self.value.read_bits() & Self::FTPG_MASK) != 0
    }

    /// Set FTPG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ftpg(&self) {
        self.value.modify_bits(|r| r | Self::FTPG_MASK);
    }

    /// Clear FTPG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ftpg(&self) {
        self.value.modify_bits(|r| r & !Self::FTPG_MASK);
    }

    /// **FLOCK** `[15]` (RW)
    ///
    /// Fast programmable lock
    #[inline(always)]
    pub fn flock(&self) -> bool {
        (self.value.read_bits() & Self::FLOCK_MASK) != 0
    }

    /// Set FLOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_flock(&self) {
        self.value.modify_bits(|r| r | Self::FLOCK_MASK);
    }

    /// Clear FLOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_flock(&self) {
        self.value.modify_bits(|r| r & !Self::FLOCK_MASK);
    }

    /// **FWAKEIE** `[13]` (RW)
    ///
    /// Wakeup enable
    #[inline(always)]
    pub fn fwakeie(&self) -> bool {
        (self.value.read_bits() & Self::FWAKEIE_MASK) != 0
    }

    /// Set FWAKEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fwakeie(&self) {
        self.value.modify_bits(|r| r | Self::FWAKEIE_MASK);
    }

    /// Clear FWAKEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fwakeie(&self) {
        self.value.modify_bits(|r| r & !Self::FWAKEIE_MASK);
    }

    /// **EOPIE** `[12]` (RW)
    ///
    /// End of operation interrupt
    /// enable
    #[inline(always)]
    pub fn eopie(&self) -> bool {
        (self.value.read_bits() & Self::EOPIE_MASK) != 0
    }

    /// Set EOPIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eopie(&self) {
        self.value.modify_bits(|r| r | Self::EOPIE_MASK);
    }

    /// Clear EOPIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eopie(&self) {
        self.value.modify_bits(|r| r & !Self::EOPIE_MASK);
    }

    /// **ERRIE** `[10]` (RW)
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> bool {
        (self.value.read_bits() & Self::ERRIE_MASK) != 0
    }

    /// Set ERRIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_errie(&self) {
        self.value.modify_bits(|r| r | Self::ERRIE_MASK);
    }

    /// Clear ERRIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_errie(&self) {
        self.value.modify_bits(|r| r & !Self::ERRIE_MASK);
    }

    /// **OBWRE** `[9]` (RW)
    ///
    /// Option bytes write enable
    #[inline(always)]
    pub fn obwre(&self) -> bool {
        (self.value.read_bits() & Self::OBWRE_MASK) != 0
    }

    /// Set OBWRE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_obwre(&self) {
        self.value.modify_bits(|r| r | Self::OBWRE_MASK);
    }

    /// Clear OBWRE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_obwre(&self) {
        self.value.modify_bits(|r| r & !Self::OBWRE_MASK);
    }

    /// **LOCK** `[7]` (RW)
    ///
    /// Lock
    #[inline(always)]
    pub fn lock(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_MASK) != 0
    }

    /// Set LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lock(&self) {
        self.value.modify_bits(|r| r | Self::LOCK_MASK);
    }

    /// Clear LOCK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lock(&self) {
        self.value.modify_bits(|r| r & !Self::LOCK_MASK);
    }

    /// **STRT** `[6]` (RW)
    ///
    /// Start
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

    /// **OBER** `[5]` (RW)
    ///
    /// Option byte erase
    #[inline(always)]
    pub fn ober(&self) -> bool {
        (self.value.read_bits() & Self::OBER_MASK) != 0
    }

    /// Set OBER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ober(&self) {
        self.value.modify_bits(|r| r | Self::OBER_MASK);
    }

    /// Clear OBER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ober(&self) {
        self.value.modify_bits(|r| r & !Self::OBER_MASK);
    }

    /// **OBPG** `[4]` (RW)
    ///
    /// Option byte programming
    #[inline(always)]
    pub fn obpg(&self) -> bool {
        (self.value.read_bits() & Self::OBPG_MASK) != 0
    }

    /// Set OBPG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_obpg(&self) {
        self.value.modify_bits(|r| r | Self::OBPG_MASK);
    }

    /// Clear OBPG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_obpg(&self) {
        self.value.modify_bits(|r| r & !Self::OBPG_MASK);
    }

    /// **MER** `[2]` (RW)
    ///
    /// Mass Erase
    #[inline(always)]
    pub fn mer(&self) -> bool {
        (self.value.read_bits() & Self::MER_MASK) != 0
    }

    /// Set MER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mer(&self) {
        self.value.modify_bits(|r| r | Self::MER_MASK);
    }

    /// Clear MER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mer(&self) {
        self.value.modify_bits(|r| r & !Self::MER_MASK);
    }

    /// **PER** `[1]` (RW)
    ///
    /// Page Erase
    #[inline(always)]
    pub fn per(&self) -> bool {
        (self.value.read_bits() & Self::PER_MASK) != 0
    }

    /// Set PER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_per(&self) {
        self.value.modify_bits(|r| r | Self::PER_MASK);
    }

    /// Clear PER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_per(&self) {
        self.value.modify_bits(|r| r & !Self::PER_MASK);
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
        Self { bits: 0x00000080 }
    }

    /// Read **BER32** `[23:23]`
    ///
    /// BER32
    #[inline(always)]
    pub const fn ber32(&self) -> bool {
        (self.bits & Ctlr::BER32_MASK) != 0
    }

    /// Read **BUFRST** `[19:19]`
    ///
    /// BUFRST
    #[inline(always)]
    pub const fn bufrst(&self) -> bool {
        (self.bits & Ctlr::BUFRST_MASK) != 0
    }

    /// Read **BUFLOAD** `[18:18]`
    ///
    /// BUFLOAD
    #[inline(always)]
    pub const fn bufload(&self) -> bool {
        (self.bits & Ctlr::BUFLOAD_MASK) != 0
    }

    /// Read **FTER** `[17:17]`
    ///
    /// Fast erase
    #[inline(always)]
    pub const fn fter(&self) -> bool {
        (self.bits & Ctlr::FTER_MASK) != 0
    }

    /// Read **FTPG** `[16:16]`
    ///
    /// Fast programming
    #[inline(always)]
    pub const fn ftpg(&self) -> bool {
        (self.bits & Ctlr::FTPG_MASK) != 0
    }

    /// Read **FLOCK** `[15:15]`
    ///
    /// Fast programmable lock
    #[inline(always)]
    pub const fn flock(&self) -> bool {
        (self.bits & Ctlr::FLOCK_MASK) != 0
    }

    /// Read **FWAKEIE** `[13:13]`
    ///
    /// Wakeup enable
    #[inline(always)]
    pub const fn fwakeie(&self) -> bool {
        (self.bits & Ctlr::FWAKEIE_MASK) != 0
    }

    /// Read **EOPIE** `[12:12]`
    ///
    /// End of operation interrupt
    /// enable
    #[inline(always)]
    pub const fn eopie(&self) -> bool {
        (self.bits & Ctlr::EOPIE_MASK) != 0
    }

    /// Read **ERRIE** `[10:10]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn errie(&self) -> bool {
        (self.bits & Ctlr::ERRIE_MASK) != 0
    }

    /// Read **OBWRE** `[9:9]`
    ///
    /// Option bytes write enable
    #[inline(always)]
    pub const fn obwre(&self) -> bool {
        (self.bits & Ctlr::OBWRE_MASK) != 0
    }

    /// Read **LOCK** `[7:7]`
    ///
    /// Lock
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        (self.bits & Ctlr::LOCK_MASK) != 0
    }

    /// Read **STRT** `[6:6]`
    ///
    /// Start
    #[inline(always)]
    pub const fn strt(&self) -> bool {
        (self.bits & Ctlr::STRT_MASK) != 0
    }

    /// Read **OBER** `[5:5]`
    ///
    /// Option byte erase
    #[inline(always)]
    pub const fn ober(&self) -> bool {
        (self.bits & Ctlr::OBER_MASK) != 0
    }

    /// Read **OBPG** `[4:4]`
    ///
    /// Option byte programming
    #[inline(always)]
    pub const fn obpg(&self) -> bool {
        (self.bits & Ctlr::OBPG_MASK) != 0
    }

    /// Read **MER** `[2:2]`
    ///
    /// Mass Erase
    #[inline(always)]
    pub const fn mer(&self) -> bool {
        (self.bits & Ctlr::MER_MASK) != 0
    }

    /// Read **PER** `[1:1]`
    ///
    /// Page Erase
    #[inline(always)]
    pub const fn per(&self) -> bool {
        (self.bits & Ctlr::PER_MASK) != 0
    }

    /// Set **BER32** `[23:23]`
    ///
    /// BER32
    #[inline(always)]
    pub const fn with_ber32(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::BER32_MASK
            } else {
                self.bits & !Ctlr::BER32_MASK
            },
        }
    }

    /// Set **BUFRST** `[19:19]`
    ///
    /// BUFRST
    #[inline(always)]
    pub const fn with_bufrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::BUFRST_MASK
            } else {
                self.bits & !Ctlr::BUFRST_MASK
            },
        }
    }

    /// Set **BUFLOAD** `[18:18]`
    ///
    /// BUFLOAD
    #[inline(always)]
    pub const fn with_bufload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::BUFLOAD_MASK
            } else {
                self.bits & !Ctlr::BUFLOAD_MASK
            },
        }
    }

    /// Set **FTER** `[17:17]`
    ///
    /// Fast erase
    #[inline(always)]
    pub const fn with_fter(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::FTER_MASK
            } else {
                self.bits & !Ctlr::FTER_MASK
            },
        }
    }

    /// Set **FTPG** `[16:16]`
    ///
    /// Fast programming
    #[inline(always)]
    pub const fn with_ftpg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::FTPG_MASK
            } else {
                self.bits & !Ctlr::FTPG_MASK
            },
        }
    }

    /// Set **FLOCK** `[15:15]`
    ///
    /// Fast programmable lock
    #[inline(always)]
    pub const fn with_flock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::FLOCK_MASK
            } else {
                self.bits & !Ctlr::FLOCK_MASK
            },
        }
    }

    /// Set **FWAKEIE** `[13:13]`
    ///
    /// Wakeup enable
    #[inline(always)]
    pub const fn with_fwakeie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::FWAKEIE_MASK
            } else {
                self.bits & !Ctlr::FWAKEIE_MASK
            },
        }
    }

    /// Set **EOPIE** `[12:12]`
    ///
    /// End of operation interrupt
    /// enable
    #[inline(always)]
    pub const fn with_eopie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::EOPIE_MASK
            } else {
                self.bits & !Ctlr::EOPIE_MASK
            },
        }
    }

    /// Set **ERRIE** `[10:10]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn with_errie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::ERRIE_MASK
            } else {
                self.bits & !Ctlr::ERRIE_MASK
            },
        }
    }

    /// Set **OBWRE** `[9:9]`
    ///
    /// Option bytes write enable
    #[inline(always)]
    pub const fn with_obwre(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::OBWRE_MASK
            } else {
                self.bits & !Ctlr::OBWRE_MASK
            },
        }
    }

    /// Set **LOCK** `[7:7]`
    ///
    /// Lock
    #[inline(always)]
    pub const fn with_lock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::LOCK_MASK
            } else {
                self.bits & !Ctlr::LOCK_MASK
            },
        }
    }

    /// Set **STRT** `[6:6]`
    ///
    /// Start
    #[inline(always)]
    pub const fn with_strt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::STRT_MASK
            } else {
                self.bits & !Ctlr::STRT_MASK
            },
        }
    }

    /// Set **OBER** `[5:5]`
    ///
    /// Option byte erase
    #[inline(always)]
    pub const fn with_ober(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::OBER_MASK
            } else {
                self.bits & !Ctlr::OBER_MASK
            },
        }
    }

    /// Set **OBPG** `[4:4]`
    ///
    /// Option byte programming
    #[inline(always)]
    pub const fn with_obpg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::OBPG_MASK
            } else {
                self.bits & !Ctlr::OBPG_MASK
            },
        }
    }

    /// Set **MER** `[2:2]`
    ///
    /// Mass Erase
    #[inline(always)]
    pub const fn with_mer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::MER_MASK
            } else {
                self.bits & !Ctlr::MER_MASK
            },
        }
    }

    /// Set **PER** `[1:1]`
    ///
    /// Page Erase
    #[inline(always)]
    pub const fn with_per(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr::PER_MASK
            } else {
                self.bits & !Ctlr::PER_MASK
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

/// ADDR Register
///
/// Flash address register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FAR** `[31:0]` (RW)
///
///   Flash Address
#[repr(transparent)]
pub struct Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Addr {
    // ======== Field Constants ========

    /// **FAR**
    ///
    /// Flash Address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const FAR_OFFSET: u32 = 0;
    pub const FAR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **FAR** `[31:0]` (RW)
    ///
    /// Flash Address
    #[inline(always)]
    pub fn far(&self) -> u32 {
        (self.value.read_bits() & Self::FAR_MASK) >> Self::FAR_OFFSET
    }
}

/// ADDR value type for batch read/modify/write operations
///
/// Use `addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrVal {
    bits: u32,
}

impl AddrVal {
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

    /// Read **FAR** `[31:0]`
    ///
    /// Flash Address
    #[inline(always)]
    pub const fn far(&self) -> u32 {
        (self.bits & Addr::FAR_MASK) >> Addr::FAR_OFFSET
    }

    /// Set **FAR** `[31:0]`
    ///
    /// Flash Address
    #[inline(always)]
    pub const fn with_far(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Addr::FAR_MASK) | ((value << Addr::FAR_OFFSET) & Addr::FAR_MASK),
        }
    }
}

impl Default for AddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Addr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrVal) {
        self.value.write_bits(val.bits())
    }
}

/// OBR Register
///
/// Option byte register
///
/// Address offset: 0x001c
/// Reset value: 0x03fffffc
///
/// ## Fields
///
/// - **DATA1** `[25:18]` (RW)
///
///   DATA1
///
/// - **DATA0** `[17:10]` (RW)
///
///   DATA0
///
/// - **CFGRSTT** `[6:5]` (RW)
///
///   Config reset delay time
///
/// - **STANDY_RST** `[4]` (RW)
///
///   STANDY_RST
///
/// - **STOP_RST** `[3]` (RW)
///
///   STOP_RST
///
/// - **IWDG_SW** `[2]` (RW)
///
///   IWDG_SW
///
/// - **RDPRT** `[1]` (RW)
///
///   Read protection
///
/// - **OBERR** `[0]` (RW)
///
///   Option byte error
#[repr(transparent)]
pub struct Obr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Obr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Obr {
    // ======== Field Constants ========

    /// **DATA1**
    ///
    /// DATA1
    ///
    /// Bits `[25:18]`, mask `0x03fc0000`
    pub const DATA1_OFFSET: u32 = 18;
    pub const DATA1_MASK: u32 = 0x03fc0000;

    /// **DATA0**
    ///
    /// DATA0
    ///
    /// Bits `[17:10]`, mask `0x0003fc00`
    pub const DATA0_OFFSET: u32 = 10;
    pub const DATA0_MASK: u32 = 0x0003fc00;

    /// **CFGRSTT**
    ///
    /// Config reset delay time
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const CFGRSTT_OFFSET: u32 = 5;
    pub const CFGRSTT_MASK: u32 = 0x00000060;

    /// **STANDY_RST**
    ///
    /// STANDY_RST
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const STANDY_RST_OFFSET: u32 = 4;
    pub const STANDY_RST_MASK: u32 = 0x00000010;
    pub const STANDY_RST: u32 = 1 << 4;

    /// **STOP_RST**
    ///
    /// STOP_RST
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const STOP_RST_OFFSET: u32 = 3;
    pub const STOP_RST_MASK: u32 = 0x00000008;
    pub const STOP_RST: u32 = 1 << 3;

    /// **IWDG_SW**
    ///
    /// IWDG_SW
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IWDG_SW_OFFSET: u32 = 2;
    pub const IWDG_SW_MASK: u32 = 0x00000004;
    pub const IWDG_SW: u32 = 1 << 2;

    /// **RDPRT**
    ///
    /// Read protection
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDPRT_OFFSET: u32 = 1;
    pub const RDPRT_MASK: u32 = 0x00000002;
    pub const RDPRT: u32 = 1 << 1;

    /// **OBERR**
    ///
    /// Option byte error
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OBERR_OFFSET: u32 = 0;
    pub const OBERR_MASK: u32 = 0x00000001;
    pub const OBERR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DATA1** `[25:18]` (RW)
    ///
    /// DATA1
    #[inline(always)]
    pub fn data1(&self) -> u32 {
        (self.value.read_bits() & Self::DATA1_MASK) >> Self::DATA1_OFFSET
    }

    /// **DATA0** `[17:10]` (RW)
    ///
    /// DATA0
    #[inline(always)]
    pub fn data0(&self) -> u32 {
        (self.value.read_bits() & Self::DATA0_MASK) >> Self::DATA0_OFFSET
    }

    /// **CFGRSTT** `[6:5]` (RW)
    ///
    /// Config reset delay time
    #[inline(always)]
    pub fn cfgrstt(&self) -> u32 {
        (self.value.read_bits() & Self::CFGRSTT_MASK) >> Self::CFGRSTT_OFFSET
    }

    /// **STANDY_RST** `[4]` (RW)
    ///
    /// STANDY_RST
    #[inline(always)]
    pub fn standy_rst(&self) -> bool {
        (self.value.read_bits() & Self::STANDY_RST_MASK) != 0
    }

    /// **STOP_RST** `[3]` (RW)
    ///
    /// STOP_RST
    #[inline(always)]
    pub fn stop_rst(&self) -> bool {
        (self.value.read_bits() & Self::STOP_RST_MASK) != 0
    }

    /// **IWDG_SW** `[2]` (RW)
    ///
    /// IWDG_SW
    #[inline(always)]
    pub fn iwdg_sw(&self) -> bool {
        (self.value.read_bits() & Self::IWDG_SW_MASK) != 0
    }

    /// **RDPRT** `[1]` (RW)
    ///
    /// Read protection
    #[inline(always)]
    pub fn rdprt(&self) -> bool {
        (self.value.read_bits() & Self::RDPRT_MASK) != 0
    }

    /// **OBERR** `[0]` (RW)
    ///
    /// Option byte error
    #[inline(always)]
    pub fn oberr(&self) -> bool {
        (self.value.read_bits() & Self::OBERR_MASK) != 0
    }
}

/// OBR value type for batch read/modify/write operations
///
/// Use `obr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `obr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ObrVal {
    bits: u32,
}

impl ObrVal {
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
        Self { bits: 0x03fffffc }
    }

    /// Read **DATA1** `[25:18]`
    ///
    /// DATA1
    #[inline(always)]
    pub const fn data1(&self) -> u32 {
        (self.bits & Obr::DATA1_MASK) >> Obr::DATA1_OFFSET
    }

    /// Read **DATA0** `[17:10]`
    ///
    /// DATA0
    #[inline(always)]
    pub const fn data0(&self) -> u32 {
        (self.bits & Obr::DATA0_MASK) >> Obr::DATA0_OFFSET
    }

    /// Read **CFGRSTT** `[6:5]`
    ///
    /// Config reset delay time
    #[inline(always)]
    pub const fn cfgrstt(&self) -> u32 {
        (self.bits & Obr::CFGRSTT_MASK) >> Obr::CFGRSTT_OFFSET
    }

    /// Read **STANDY_RST** `[4:4]`
    ///
    /// STANDY_RST
    #[inline(always)]
    pub const fn standy_rst(&self) -> bool {
        (self.bits & Obr::STANDY_RST_MASK) != 0
    }

    /// Read **STOP_RST** `[3:3]`
    ///
    /// STOP_RST
    #[inline(always)]
    pub const fn stop_rst(&self) -> bool {
        (self.bits & Obr::STOP_RST_MASK) != 0
    }

    /// Read **IWDG_SW** `[2:2]`
    ///
    /// IWDG_SW
    #[inline(always)]
    pub const fn iwdg_sw(&self) -> bool {
        (self.bits & Obr::IWDG_SW_MASK) != 0
    }

    /// Read **RDPRT** `[1:1]`
    ///
    /// Read protection
    #[inline(always)]
    pub const fn rdprt(&self) -> bool {
        (self.bits & Obr::RDPRT_MASK) != 0
    }

    /// Read **OBERR** `[0:0]`
    ///
    /// Option byte error
    #[inline(always)]
    pub const fn oberr(&self) -> bool {
        (self.bits & Obr::OBERR_MASK) != 0
    }

    /// Set **DATA1** `[25:18]`
    ///
    /// DATA1
    #[inline(always)]
    pub const fn with_data1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Obr::DATA1_MASK) | ((value << Obr::DATA1_OFFSET) & Obr::DATA1_MASK),
        }
    }

    /// Set **DATA0** `[17:10]`
    ///
    /// DATA0
    #[inline(always)]
    pub const fn with_data0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Obr::DATA0_MASK) | ((value << Obr::DATA0_OFFSET) & Obr::DATA0_MASK),
        }
    }

    /// Set **CFGRSTT** `[6:5]`
    ///
    /// Config reset delay time
    #[inline(always)]
    pub const fn with_cfgrstt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Obr::CFGRSTT_MASK)
                | ((value << Obr::CFGRSTT_OFFSET) & Obr::CFGRSTT_MASK),
        }
    }

    /// Set **STANDY_RST** `[4:4]`
    ///
    /// STANDY_RST
    #[inline(always)]
    pub const fn with_standy_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Obr::STANDY_RST_MASK
            } else {
                self.bits & !Obr::STANDY_RST_MASK
            },
        }
    }

    /// Set **STOP_RST** `[3:3]`
    ///
    /// STOP_RST
    #[inline(always)]
    pub const fn with_stop_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Obr::STOP_RST_MASK
            } else {
                self.bits & !Obr::STOP_RST_MASK
            },
        }
    }

    /// Set **IWDG_SW** `[2:2]`
    ///
    /// IWDG_SW
    #[inline(always)]
    pub const fn with_iwdg_sw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Obr::IWDG_SW_MASK
            } else {
                self.bits & !Obr::IWDG_SW_MASK
            },
        }
    }

    /// Set **RDPRT** `[1:1]`
    ///
    /// Read protection
    #[inline(always)]
    pub const fn with_rdprt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Obr::RDPRT_MASK
            } else {
                self.bits & !Obr::RDPRT_MASK
            },
        }
    }

    /// Set **OBERR** `[0:0]`
    ///
    /// Option byte error
    #[inline(always)]
    pub const fn with_oberr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Obr::OBERR_MASK
            } else {
                self.bits & !Obr::OBERR_MASK
            },
        }
    }
}

impl Default for ObrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Obr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ObrVal {
        ObrVal::from_bits(self.value.read_bits())
    }
}

/// WPR Register
///
/// Write protection register
///
/// Address offset: 0x0020
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **WRP** `[31:0]` (RW)
///
///   Write protect
#[repr(transparent)]
pub struct Wpr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wpr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wpr {
    // ======== Field Constants ========

    /// **WRP**
    ///
    /// Write protect
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WRP_OFFSET: u32 = 0;
    pub const WRP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WRP** `[31:0]` (RW)
    ///
    /// Write protect
    #[inline(always)]
    pub fn wrp(&self) -> u32 {
        (self.value.read_bits() & Self::WRP_MASK) >> Self::WRP_OFFSET
    }
}

/// WPR value type for batch read/modify/write operations
///
/// Use `wpr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wpr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WprVal {
    bits: u32,
}

impl WprVal {
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
        Self { bits: 0xffffffff }
    }

    /// Read **WRP** `[31:0]`
    ///
    /// Write protect
    #[inline(always)]
    pub const fn wrp(&self) -> u32 {
        (self.bits & Wpr::WRP_MASK) >> Wpr::WRP_OFFSET
    }

    /// Set **WRP** `[31:0]`
    ///
    /// Write protect
    #[inline(always)]
    pub const fn with_wrp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wpr::WRP_MASK) | ((value << Wpr::WRP_OFFSET) & Wpr::WRP_MASK),
        }
    }
}

impl Default for WprVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wpr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WprVal {
        WprVal::from_bits(self.value.read_bits())
    }
}

/// MODEKEYR Register
///
/// Mode select register
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MODEKEYR** `[31:0]` (RW)
///
///   Mode select
#[repr(transparent)]
pub struct Modekeyr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Modekeyr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Modekeyr {
    // ======== Field Constants ========

    /// **MODEKEYR**
    ///
    /// Mode select
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MODEKEYR_OFFSET: u32 = 0;
    pub const MODEKEYR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MODEKEYR** `[31:0]` (RW)
    ///
    /// Mode select
    #[inline(always)]
    pub fn modekeyr(&self) -> u32 {
        (self.value.read_bits() & Self::MODEKEYR_MASK) >> Self::MODEKEYR_OFFSET
    }
}

/// MODEKEYR value type for batch read/modify/write operations
///
/// Use `modekeyr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `modekeyr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ModekeyrVal {
    bits: u32,
}

impl ModekeyrVal {
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

    /// Read **MODEKEYR** `[31:0]`
    ///
    /// Mode select
    #[inline(always)]
    pub const fn modekeyr(&self) -> u32 {
        (self.bits & Modekeyr::MODEKEYR_MASK) >> Modekeyr::MODEKEYR_OFFSET
    }

    /// Set **MODEKEYR** `[31:0]`
    ///
    /// Mode select
    #[inline(always)]
    pub const fn with_modekeyr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Modekeyr::MODEKEYR_MASK)
                | ((value << Modekeyr::MODEKEYR_OFFSET) & Modekeyr::MODEKEYR_MASK),
        }
    }
}

impl Default for ModekeyrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Modekeyr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ModekeyrVal) {
        self.value.write_bits(val.bits())
    }
}

/// BOOT_MODEKEYP Register
///
/// BOOT Mode select register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MODEKEYR** `[31:0]` (RW)
///
///   Mode select
#[repr(transparent)]
pub struct BootModekeyp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BootModekeyp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BootModekeyp {
    // ======== Field Constants ========

    /// **MODEKEYR**
    ///
    /// Mode select
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MODEKEYR_OFFSET: u32 = 0;
    pub const MODEKEYR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MODEKEYR** `[31:0]` (RW)
    ///
    /// Mode select
    #[inline(always)]
    pub fn modekeyr(&self) -> u32 {
        (self.value.read_bits() & Self::MODEKEYR_MASK) >> Self::MODEKEYR_OFFSET
    }
}

/// BOOT_MODEKEYP value type for batch read/modify/write operations
///
/// Use `bootmodekeyp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootmodekeyp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootModekeypVal {
    bits: u32,
}

impl BootModekeypVal {
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

    /// Read **MODEKEYR** `[31:0]`
    ///
    /// Mode select
    #[inline(always)]
    pub const fn modekeyr(&self) -> u32 {
        (self.bits & BootModekeyp::MODEKEYR_MASK) >> BootModekeyp::MODEKEYR_OFFSET
    }

    /// Set **MODEKEYR** `[31:0]`
    ///
    /// Mode select
    #[inline(always)]
    pub const fn with_modekeyr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BootModekeyp::MODEKEYR_MASK)
                | ((value << BootModekeyp::MODEKEYR_OFFSET) & BootModekeyp::MODEKEYR_MASK),
        }
    }
}

impl Default for BootModekeypVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BootModekeyp {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootModekeypVal) {
        self.value.write_bits(val.bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Flash ACTL register
    pub actlr: Actlr,
    /// 0x0004
    /// Flash key register
    pub keyr: Keyr,
    /// 0x0008
    /// Flash option key register
    pub obkeyr: Obkeyr,
    /// 0x000c
    /// Status register
    pub statr: Statr,
    /// 0x0010
    /// Control register
    pub ctlr: Ctlr,
    /// 0x0014
    /// Flash address register
    pub addr: Addr,
    _reserved0: [u8; 4],
    /// 0x001c
    /// Option byte register
    pub obr: Obr,
    /// 0x0020
    /// Write protection register
    pub wpr: Wpr,
    /// 0x0024
    /// Mode select register
    pub modekeyr: Modekeyr,
    /// 0x0028
    /// BOOT Mode select register
    pub boot_modekeyp: BootModekeyp,
}
