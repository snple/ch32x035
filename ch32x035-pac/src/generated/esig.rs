//! ESIG
//! ESIG configuration

#![allow(dead_code)]

pub const BASE: u32 = 0x1ffff7e0;

/// FLACAP Register
///
/// Flash capecity register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **F_SIZE** `[15:0]` (RO)
///
///   F_SIZE/kByte
#[repr(transparent)]
pub struct Flacap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Flacap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Flacap {
    // ======== Field Constants ========

    /// **F_SIZE**
    ///
    /// F_SIZE/kByte
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const F_SIZE_OFFSET: u32 = 0;
    pub const F_SIZE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **F_SIZE** `[15:0]` (RO)
    ///
    /// F_SIZE/kByte
    #[inline(always)]
    pub fn f_size(&self) -> u32 {
        (self.value.read_bits() & Self::F_SIZE_MASK) >> Self::F_SIZE_OFFSET
    }
}

/// FLACAP value type for batch read/modify/write operations
///
/// Use `flacap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `flacap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FlacapVal {
    bits: u32,
}

impl FlacapVal {
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

    /// Read **F_SIZE** `[15:0]`
    ///
    /// F_SIZE/kByte
    #[inline(always)]
    pub const fn f_size(&self) -> u32 {
        (self.bits & Flacap::F_SIZE_MASK) >> Flacap::F_SIZE_OFFSET
    }
}

impl Default for FlacapVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Flacap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FlacapVal {
        FlacapVal::from_bits(self.value.read_bits())
    }
}

/// UNIID1 Register
///
/// UID register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **U_ID** `[31:0]` (RO)
///
///   U_ID value
#[repr(transparent)]
pub struct Uniid1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uniid1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uniid1 {
    // ======== Field Constants ========

    /// **U_ID**
    ///
    /// U_ID value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const U_ID_OFFSET: u32 = 0;
    pub const U_ID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **U_ID** `[31:0]` (RO)
    ///
    /// U_ID value
    #[inline(always)]
    pub fn u_id(&self) -> u32 {
        (self.value.read_bits() & Self::U_ID_MASK) >> Self::U_ID_OFFSET
    }
}

/// UNIID1 value type for batch read/modify/write operations
///
/// Use `uniid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uniid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uniid1Val {
    bits: u32,
}

impl Uniid1Val {
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

    /// Read **U_ID** `[31:0]`
    ///
    /// U_ID value
    #[inline(always)]
    pub const fn u_id(&self) -> u32 {
        (self.bits & Uniid1::U_ID_MASK) >> Uniid1::U_ID_OFFSET
    }
}

impl Default for Uniid1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uniid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uniid1Val {
        Uniid1Val::from_bits(self.value.read_bits())
    }
}

/// UNIID2 Register
///
/// UID register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **U_ID** `[31:0]` (RO)
///
///   U_ID value
#[repr(transparent)]
pub struct Uniid2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uniid2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uniid2 {
    // ======== Field Constants ========

    /// **U_ID**
    ///
    /// U_ID value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const U_ID_OFFSET: u32 = 0;
    pub const U_ID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **U_ID** `[31:0]` (RO)
    ///
    /// U_ID value
    #[inline(always)]
    pub fn u_id(&self) -> u32 {
        (self.value.read_bits() & Self::U_ID_MASK) >> Self::U_ID_OFFSET
    }
}

/// UNIID2 value type for batch read/modify/write operations
///
/// Use `uniid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uniid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uniid2Val {
    bits: u32,
}

impl Uniid2Val {
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

    /// Read **U_ID** `[31:0]`
    ///
    /// U_ID value
    #[inline(always)]
    pub const fn u_id(&self) -> u32 {
        (self.bits & Uniid2::U_ID_MASK) >> Uniid2::U_ID_OFFSET
    }
}

impl Default for Uniid2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uniid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uniid2Val {
        Uniid2Val::from_bits(self.value.read_bits())
    }
}

/// UNIID3 Register
///
/// Div register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AWU_APR** `[31:0]` (RO)
///
///   AWU_APR value
#[repr(transparent)]
pub struct Uniid3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uniid3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uniid3 {
    // ======== Field Constants ========

    /// **AWU_APR**
    ///
    /// AWU_APR value
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const AWU_APR_OFFSET: u32 = 0;
    pub const AWU_APR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **AWU_APR** `[31:0]` (RO)
    ///
    /// AWU_APR value
    #[inline(always)]
    pub fn awu_apr(&self) -> u32 {
        (self.value.read_bits() & Self::AWU_APR_MASK) >> Self::AWU_APR_OFFSET
    }
}

/// UNIID3 value type for batch read/modify/write operations
///
/// Use `uniid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uniid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uniid3Val {
    bits: u32,
}

impl Uniid3Val {
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

    /// Read **AWU_APR** `[31:0]`
    ///
    /// AWU_APR value
    #[inline(always)]
    pub const fn awu_apr(&self) -> u32 {
        (self.bits & Uniid3::AWU_APR_MASK) >> Uniid3::AWU_APR_OFFSET
    }
}

impl Default for Uniid3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uniid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uniid3Val {
        Uniid3Val::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Flash capecity register
    pub flacap: Flacap,
    _reserved0: [u8; 4],
    /// 0x0008
    /// UID register
    pub uniid1: Uniid1,
    /// 0x000c
    /// UID register
    pub uniid2: Uniid2,
    /// 0x0010
    /// Div register
    pub uniid3: Uniid3,
}
