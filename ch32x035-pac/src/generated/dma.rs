//! DMA
//! DMA controller

#![allow(dead_code)]

pub const BASE: u32 = 0x40020000;

/// INTFR Register
///
/// DMA interrupt status register
/// (DMA_INTFR)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TEIF8** `[31]` (RO)
///
///   Channel 8 Transfer Error
///   flag
///
/// - **HTIF8** `[30]` (RO)
///
///   Channel 8 Half Transfer Complete
///   flag
///
/// - **TCIF8** `[29]` (RO)
///
///   Channel 8 Transfer Complete
///   flag
///
/// - **GIF8** `[28]` (RO)
///
///   Channel 8 Global interrupt
///   flag
///
/// - **TEIF7** `[27]` (RO)
///
///   Channel 7 Transfer Error
///   flag
///
/// - **HTIF7** `[26]` (RO)
///
///   Channel 7 Half Transfer Complete
///   flag
///
/// - **TCIF7** `[25]` (RO)
///
///   Channel 7 Transfer Complete
///   flag
///
/// - **GIF7** `[24]` (RO)
///
///   Channel 7 Global interrupt
///   flag
///
/// - **TEIF6** `[23]` (RO)
///
///   Channel 6 Transfer Error
///   flag
///
/// - **HTIF6** `[22]` (RO)
///
///   Channel 6 Half Transfer Complete
///   flag
///
/// - **TCIF6** `[21]` (RO)
///
///   Channel 6 Transfer Complete
///   flag
///
/// - **GIF6** `[20]` (RO)
///
///   Channel 6 Global interrupt
///   flag
///
/// - **TEIF5** `[19]` (RO)
///
///   Channel 5 Transfer Error
///   flag
///
/// - **HTIF5** `[18]` (RO)
///
///   Channel 5 Half Transfer Complete
///   flag
///
/// - **TCIF5** `[17]` (RO)
///
///   Channel 5 Transfer Complete
///   flag
///
/// - **GIF5** `[16]` (RO)
///
///   Channel 5 Global interrupt
///   flag
///
/// - **TEIF4** `[15]` (RO)
///
///   Channel 4 Transfer Error
///   flag
///
/// - **HTIF4** `[14]` (RO)
///
///   Channel 4 Half Transfer Complete
///   flag
///
/// - **TCIF4** `[13]` (RO)
///
///   Channel 4 Transfer Complete
///   flag
///
/// - **GIF4** `[12]` (RO)
///
///   Channel 4 Global interrupt
///   flag
///
/// - **TEIF3** `[11]` (RO)
///
///   Channel 3 Transfer Error
///   flag
///
/// - **HTIF3** `[10]` (RO)
///
///   Channel 3 Half Transfer Complete
///   flag
///
/// - **TCIF3** `[9]` (RO)
///
///   Channel 3 Transfer Complete
///   flag
///
/// - **GIF3** `[8]` (RO)
///
///   Channel 3 Global interrupt
///   flag
///
/// - **TEIF2** `[7]` (RO)
///
///   Channel 2 Transfer Error
///   flag
///
/// - **HTIF2** `[6]` (RO)
///
///   Channel 2 Half Transfer Complete
///   flag
///
/// - **TCIF2** `[5]` (RO)
///
///   Channel 2 Transfer Complete
///   flag
///
/// - **GIF2** `[4]` (RO)
///
///   Channel 2 Global interrupt
///   flag
///
/// - **TEIF1** `[3]` (RO)
///
///   Channel 1 Transfer Error
///   flag
///
/// - **HTIF1** `[2]` (RO)
///
///   Channel 1 Half Transfer Complete
///   flag
///
/// - **TCIF1** `[1]` (RO)
///
///   Channel 1 Transfer Complete
///   flag
///
/// - **GIF1** `[0]` (RO)
///
///   Channel 1 Global interrupt
///   flag
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

    /// **TEIF8**
    ///
    /// Channel 8 Transfer Error
    /// flag
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TEIF8_OFFSET: u32 = 31;
    pub const TEIF8_MASK: u32 = 0x80000000;
    pub const TEIF8: u32 = 1 << 31;

    /// **HTIF8**
    ///
    /// Channel 8 Half Transfer Complete
    /// flag
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const HTIF8_OFFSET: u32 = 30;
    pub const HTIF8_MASK: u32 = 0x40000000;
    pub const HTIF8: u32 = 1 << 30;

    /// **TCIF8**
    ///
    /// Channel 8 Transfer Complete
    /// flag
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TCIF8_OFFSET: u32 = 29;
    pub const TCIF8_MASK: u32 = 0x20000000;
    pub const TCIF8: u32 = 1 << 29;

    /// **GIF8**
    ///
    /// Channel 8 Global interrupt
    /// flag
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const GIF8_OFFSET: u32 = 28;
    pub const GIF8_MASK: u32 = 0x10000000;
    pub const GIF8: u32 = 1 << 28;

    /// **TEIF7**
    ///
    /// Channel 7 Transfer Error
    /// flag
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TEIF7_OFFSET: u32 = 27;
    pub const TEIF7_MASK: u32 = 0x08000000;
    pub const TEIF7: u32 = 1 << 27;

    /// **HTIF7**
    ///
    /// Channel 7 Half Transfer Complete
    /// flag
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const HTIF7_OFFSET: u32 = 26;
    pub const HTIF7_MASK: u32 = 0x04000000;
    pub const HTIF7: u32 = 1 << 26;

    /// **TCIF7**
    ///
    /// Channel 7 Transfer Complete
    /// flag
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TCIF7_OFFSET: u32 = 25;
    pub const TCIF7_MASK: u32 = 0x02000000;
    pub const TCIF7: u32 = 1 << 25;

    /// **GIF7**
    ///
    /// Channel 7 Global interrupt
    /// flag
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const GIF7_OFFSET: u32 = 24;
    pub const GIF7_MASK: u32 = 0x01000000;
    pub const GIF7: u32 = 1 << 24;

    /// **TEIF6**
    ///
    /// Channel 6 Transfer Error
    /// flag
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TEIF6_OFFSET: u32 = 23;
    pub const TEIF6_MASK: u32 = 0x00800000;
    pub const TEIF6: u32 = 1 << 23;

    /// **HTIF6**
    ///
    /// Channel 6 Half Transfer Complete
    /// flag
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const HTIF6_OFFSET: u32 = 22;
    pub const HTIF6_MASK: u32 = 0x00400000;
    pub const HTIF6: u32 = 1 << 22;

    /// **TCIF6**
    ///
    /// Channel 6 Transfer Complete
    /// flag
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const TCIF6_OFFSET: u32 = 21;
    pub const TCIF6_MASK: u32 = 0x00200000;
    pub const TCIF6: u32 = 1 << 21;

    /// **GIF6**
    ///
    /// Channel 6 Global interrupt
    /// flag
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const GIF6_OFFSET: u32 = 20;
    pub const GIF6_MASK: u32 = 0x00100000;
    pub const GIF6: u32 = 1 << 20;

    /// **TEIF5**
    ///
    /// Channel 5 Transfer Error
    /// flag
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TEIF5_OFFSET: u32 = 19;
    pub const TEIF5_MASK: u32 = 0x00080000;
    pub const TEIF5: u32 = 1 << 19;

    /// **HTIF5**
    ///
    /// Channel 5 Half Transfer Complete
    /// flag
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const HTIF5_OFFSET: u32 = 18;
    pub const HTIF5_MASK: u32 = 0x00040000;
    pub const HTIF5: u32 = 1 << 18;

    /// **TCIF5**
    ///
    /// Channel 5 Transfer Complete
    /// flag
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const TCIF5_OFFSET: u32 = 17;
    pub const TCIF5_MASK: u32 = 0x00020000;
    pub const TCIF5: u32 = 1 << 17;

    /// **GIF5**
    ///
    /// Channel 5 Global interrupt
    /// flag
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const GIF5_OFFSET: u32 = 16;
    pub const GIF5_MASK: u32 = 0x00010000;
    pub const GIF5: u32 = 1 << 16;

    /// **TEIF4**
    ///
    /// Channel 4 Transfer Error
    /// flag
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TEIF4_OFFSET: u32 = 15;
    pub const TEIF4_MASK: u32 = 0x00008000;
    pub const TEIF4: u32 = 1 << 15;

    /// **HTIF4**
    ///
    /// Channel 4 Half Transfer Complete
    /// flag
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const HTIF4_OFFSET: u32 = 14;
    pub const HTIF4_MASK: u32 = 0x00004000;
    pub const HTIF4: u32 = 1 << 14;

    /// **TCIF4**
    ///
    /// Channel 4 Transfer Complete
    /// flag
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TCIF4_OFFSET: u32 = 13;
    pub const TCIF4_MASK: u32 = 0x00002000;
    pub const TCIF4: u32 = 1 << 13;

    /// **GIF4**
    ///
    /// Channel 4 Global interrupt
    /// flag
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const GIF4_OFFSET: u32 = 12;
    pub const GIF4_MASK: u32 = 0x00001000;
    pub const GIF4: u32 = 1 << 12;

    /// **TEIF3**
    ///
    /// Channel 3 Transfer Error
    /// flag
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TEIF3_OFFSET: u32 = 11;
    pub const TEIF3_MASK: u32 = 0x00000800;
    pub const TEIF3: u32 = 1 << 11;

    /// **HTIF3**
    ///
    /// Channel 3 Half Transfer Complete
    /// flag
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const HTIF3_OFFSET: u32 = 10;
    pub const HTIF3_MASK: u32 = 0x00000400;
    pub const HTIF3: u32 = 1 << 10;

    /// **TCIF3**
    ///
    /// Channel 3 Transfer Complete
    /// flag
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TCIF3_OFFSET: u32 = 9;
    pub const TCIF3_MASK: u32 = 0x00000200;
    pub const TCIF3: u32 = 1 << 9;

    /// **GIF3**
    ///
    /// Channel 3 Global interrupt
    /// flag
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const GIF3_OFFSET: u32 = 8;
    pub const GIF3_MASK: u32 = 0x00000100;
    pub const GIF3: u32 = 1 << 8;

    /// **TEIF2**
    ///
    /// Channel 2 Transfer Error
    /// flag
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TEIF2_OFFSET: u32 = 7;
    pub const TEIF2_MASK: u32 = 0x00000080;
    pub const TEIF2: u32 = 1 << 7;

    /// **HTIF2**
    ///
    /// Channel 2 Half Transfer Complete
    /// flag
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const HTIF2_OFFSET: u32 = 6;
    pub const HTIF2_MASK: u32 = 0x00000040;
    pub const HTIF2: u32 = 1 << 6;

    /// **TCIF2**
    ///
    /// Channel 2 Transfer Complete
    /// flag
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TCIF2_OFFSET: u32 = 5;
    pub const TCIF2_MASK: u32 = 0x00000020;
    pub const TCIF2: u32 = 1 << 5;

    /// **GIF2**
    ///
    /// Channel 2 Global interrupt
    /// flag
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const GIF2_OFFSET: u32 = 4;
    pub const GIF2_MASK: u32 = 0x00000010;
    pub const GIF2: u32 = 1 << 4;

    /// **TEIF1**
    ///
    /// Channel 1 Transfer Error
    /// flag
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIF1_OFFSET: u32 = 3;
    pub const TEIF1_MASK: u32 = 0x00000008;
    pub const TEIF1: u32 = 1 << 3;

    /// **HTIF1**
    ///
    /// Channel 1 Half Transfer Complete
    /// flag
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIF1_OFFSET: u32 = 2;
    pub const HTIF1_MASK: u32 = 0x00000004;
    pub const HTIF1: u32 = 1 << 2;

    /// **TCIF1**
    ///
    /// Channel 1 Transfer Complete
    /// flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIF1_OFFSET: u32 = 1;
    pub const TCIF1_MASK: u32 = 0x00000002;
    pub const TCIF1: u32 = 1 << 1;

    /// **GIF1**
    ///
    /// Channel 1 Global interrupt
    /// flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const GIF1_OFFSET: u32 = 0;
    pub const GIF1_MASK: u32 = 0x00000001;
    pub const GIF1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TEIF8** `[31]` (RO)
    ///
    /// Channel 8 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif8(&self) -> bool {
        (self.value.read_bits() & Self::TEIF8_MASK) != 0
    }

    /// **HTIF8** `[30]` (RO)
    ///
    /// Channel 8 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif8(&self) -> bool {
        (self.value.read_bits() & Self::HTIF8_MASK) != 0
    }

    /// **TCIF8** `[29]` (RO)
    ///
    /// Channel 8 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif8(&self) -> bool {
        (self.value.read_bits() & Self::TCIF8_MASK) != 0
    }

    /// **GIF8** `[28]` (RO)
    ///
    /// Channel 8 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif8(&self) -> bool {
        (self.value.read_bits() & Self::GIF8_MASK) != 0
    }

    /// **TEIF7** `[27]` (RO)
    ///
    /// Channel 7 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif7(&self) -> bool {
        (self.value.read_bits() & Self::TEIF7_MASK) != 0
    }

    /// **HTIF7** `[26]` (RO)
    ///
    /// Channel 7 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif7(&self) -> bool {
        (self.value.read_bits() & Self::HTIF7_MASK) != 0
    }

    /// **TCIF7** `[25]` (RO)
    ///
    /// Channel 7 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif7(&self) -> bool {
        (self.value.read_bits() & Self::TCIF7_MASK) != 0
    }

    /// **GIF7** `[24]` (RO)
    ///
    /// Channel 7 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif7(&self) -> bool {
        (self.value.read_bits() & Self::GIF7_MASK) != 0
    }

    /// **TEIF6** `[23]` (RO)
    ///
    /// Channel 6 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif6(&self) -> bool {
        (self.value.read_bits() & Self::TEIF6_MASK) != 0
    }

    /// **HTIF6** `[22]` (RO)
    ///
    /// Channel 6 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif6(&self) -> bool {
        (self.value.read_bits() & Self::HTIF6_MASK) != 0
    }

    /// **TCIF6** `[21]` (RO)
    ///
    /// Channel 6 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif6(&self) -> bool {
        (self.value.read_bits() & Self::TCIF6_MASK) != 0
    }

    /// **GIF6** `[20]` (RO)
    ///
    /// Channel 6 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif6(&self) -> bool {
        (self.value.read_bits() & Self::GIF6_MASK) != 0
    }

    /// **TEIF5** `[19]` (RO)
    ///
    /// Channel 5 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif5(&self) -> bool {
        (self.value.read_bits() & Self::TEIF5_MASK) != 0
    }

    /// **HTIF5** `[18]` (RO)
    ///
    /// Channel 5 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif5(&self) -> bool {
        (self.value.read_bits() & Self::HTIF5_MASK) != 0
    }

    /// **TCIF5** `[17]` (RO)
    ///
    /// Channel 5 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif5(&self) -> bool {
        (self.value.read_bits() & Self::TCIF5_MASK) != 0
    }

    /// **GIF5** `[16]` (RO)
    ///
    /// Channel 5 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif5(&self) -> bool {
        (self.value.read_bits() & Self::GIF5_MASK) != 0
    }

    /// **TEIF4** `[15]` (RO)
    ///
    /// Channel 4 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif4(&self) -> bool {
        (self.value.read_bits() & Self::TEIF4_MASK) != 0
    }

    /// **HTIF4** `[14]` (RO)
    ///
    /// Channel 4 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif4(&self) -> bool {
        (self.value.read_bits() & Self::HTIF4_MASK) != 0
    }

    /// **TCIF4** `[13]` (RO)
    ///
    /// Channel 4 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif4(&self) -> bool {
        (self.value.read_bits() & Self::TCIF4_MASK) != 0
    }

    /// **GIF4** `[12]` (RO)
    ///
    /// Channel 4 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif4(&self) -> bool {
        (self.value.read_bits() & Self::GIF4_MASK) != 0
    }

    /// **TEIF3** `[11]` (RO)
    ///
    /// Channel 3 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif3(&self) -> bool {
        (self.value.read_bits() & Self::TEIF3_MASK) != 0
    }

    /// **HTIF3** `[10]` (RO)
    ///
    /// Channel 3 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif3(&self) -> bool {
        (self.value.read_bits() & Self::HTIF3_MASK) != 0
    }

    /// **TCIF3** `[9]` (RO)
    ///
    /// Channel 3 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif3(&self) -> bool {
        (self.value.read_bits() & Self::TCIF3_MASK) != 0
    }

    /// **GIF3** `[8]` (RO)
    ///
    /// Channel 3 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif3(&self) -> bool {
        (self.value.read_bits() & Self::GIF3_MASK) != 0
    }

    /// **TEIF2** `[7]` (RO)
    ///
    /// Channel 2 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif2(&self) -> bool {
        (self.value.read_bits() & Self::TEIF2_MASK) != 0
    }

    /// **HTIF2** `[6]` (RO)
    ///
    /// Channel 2 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif2(&self) -> bool {
        (self.value.read_bits() & Self::HTIF2_MASK) != 0
    }

    /// **TCIF2** `[5]` (RO)
    ///
    /// Channel 2 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif2(&self) -> bool {
        (self.value.read_bits() & Self::TCIF2_MASK) != 0
    }

    /// **GIF2** `[4]` (RO)
    ///
    /// Channel 2 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif2(&self) -> bool {
        (self.value.read_bits() & Self::GIF2_MASK) != 0
    }

    /// **TEIF1** `[3]` (RO)
    ///
    /// Channel 1 Transfer Error
    /// flag
    #[inline(always)]
    pub fn teif1(&self) -> bool {
        (self.value.read_bits() & Self::TEIF1_MASK) != 0
    }

    /// **HTIF1** `[2]` (RO)
    ///
    /// Channel 1 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub fn htif1(&self) -> bool {
        (self.value.read_bits() & Self::HTIF1_MASK) != 0
    }

    /// **TCIF1** `[1]` (RO)
    ///
    /// Channel 1 Transfer Complete
    /// flag
    #[inline(always)]
    pub fn tcif1(&self) -> bool {
        (self.value.read_bits() & Self::TCIF1_MASK) != 0
    }

    /// **GIF1** `[0]` (RO)
    ///
    /// Channel 1 Global interrupt
    /// flag
    #[inline(always)]
    pub fn gif1(&self) -> bool {
        (self.value.read_bits() & Self::GIF1_MASK) != 0
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

    /// Read **TEIF8** `[31:31]`
    ///
    /// Channel 8 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif8(&self) -> bool {
        (self.bits & Intfr::TEIF8_MASK) != 0
    }

    /// Read **HTIF8** `[30:30]`
    ///
    /// Channel 8 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif8(&self) -> bool {
        (self.bits & Intfr::HTIF8_MASK) != 0
    }

    /// Read **TCIF8** `[29:29]`
    ///
    /// Channel 8 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif8(&self) -> bool {
        (self.bits & Intfr::TCIF8_MASK) != 0
    }

    /// Read **GIF8** `[28:28]`
    ///
    /// Channel 8 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif8(&self) -> bool {
        (self.bits & Intfr::GIF8_MASK) != 0
    }

    /// Read **TEIF7** `[27:27]`
    ///
    /// Channel 7 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif7(&self) -> bool {
        (self.bits & Intfr::TEIF7_MASK) != 0
    }

    /// Read **HTIF7** `[26:26]`
    ///
    /// Channel 7 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif7(&self) -> bool {
        (self.bits & Intfr::HTIF7_MASK) != 0
    }

    /// Read **TCIF7** `[25:25]`
    ///
    /// Channel 7 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif7(&self) -> bool {
        (self.bits & Intfr::TCIF7_MASK) != 0
    }

    /// Read **GIF7** `[24:24]`
    ///
    /// Channel 7 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif7(&self) -> bool {
        (self.bits & Intfr::GIF7_MASK) != 0
    }

    /// Read **TEIF6** `[23:23]`
    ///
    /// Channel 6 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif6(&self) -> bool {
        (self.bits & Intfr::TEIF6_MASK) != 0
    }

    /// Read **HTIF6** `[22:22]`
    ///
    /// Channel 6 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif6(&self) -> bool {
        (self.bits & Intfr::HTIF6_MASK) != 0
    }

    /// Read **TCIF6** `[21:21]`
    ///
    /// Channel 6 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif6(&self) -> bool {
        (self.bits & Intfr::TCIF6_MASK) != 0
    }

    /// Read **GIF6** `[20:20]`
    ///
    /// Channel 6 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif6(&self) -> bool {
        (self.bits & Intfr::GIF6_MASK) != 0
    }

    /// Read **TEIF5** `[19:19]`
    ///
    /// Channel 5 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif5(&self) -> bool {
        (self.bits & Intfr::TEIF5_MASK) != 0
    }

    /// Read **HTIF5** `[18:18]`
    ///
    /// Channel 5 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif5(&self) -> bool {
        (self.bits & Intfr::HTIF5_MASK) != 0
    }

    /// Read **TCIF5** `[17:17]`
    ///
    /// Channel 5 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif5(&self) -> bool {
        (self.bits & Intfr::TCIF5_MASK) != 0
    }

    /// Read **GIF5** `[16:16]`
    ///
    /// Channel 5 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif5(&self) -> bool {
        (self.bits & Intfr::GIF5_MASK) != 0
    }

    /// Read **TEIF4** `[15:15]`
    ///
    /// Channel 4 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif4(&self) -> bool {
        (self.bits & Intfr::TEIF4_MASK) != 0
    }

    /// Read **HTIF4** `[14:14]`
    ///
    /// Channel 4 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif4(&self) -> bool {
        (self.bits & Intfr::HTIF4_MASK) != 0
    }

    /// Read **TCIF4** `[13:13]`
    ///
    /// Channel 4 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif4(&self) -> bool {
        (self.bits & Intfr::TCIF4_MASK) != 0
    }

    /// Read **GIF4** `[12:12]`
    ///
    /// Channel 4 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif4(&self) -> bool {
        (self.bits & Intfr::GIF4_MASK) != 0
    }

    /// Read **TEIF3** `[11:11]`
    ///
    /// Channel 3 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif3(&self) -> bool {
        (self.bits & Intfr::TEIF3_MASK) != 0
    }

    /// Read **HTIF3** `[10:10]`
    ///
    /// Channel 3 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif3(&self) -> bool {
        (self.bits & Intfr::HTIF3_MASK) != 0
    }

    /// Read **TCIF3** `[9:9]`
    ///
    /// Channel 3 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif3(&self) -> bool {
        (self.bits & Intfr::TCIF3_MASK) != 0
    }

    /// Read **GIF3** `[8:8]`
    ///
    /// Channel 3 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif3(&self) -> bool {
        (self.bits & Intfr::GIF3_MASK) != 0
    }

    /// Read **TEIF2** `[7:7]`
    ///
    /// Channel 2 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif2(&self) -> bool {
        (self.bits & Intfr::TEIF2_MASK) != 0
    }

    /// Read **HTIF2** `[6:6]`
    ///
    /// Channel 2 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif2(&self) -> bool {
        (self.bits & Intfr::HTIF2_MASK) != 0
    }

    /// Read **TCIF2** `[5:5]`
    ///
    /// Channel 2 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif2(&self) -> bool {
        (self.bits & Intfr::TCIF2_MASK) != 0
    }

    /// Read **GIF2** `[4:4]`
    ///
    /// Channel 2 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif2(&self) -> bool {
        (self.bits & Intfr::GIF2_MASK) != 0
    }

    /// Read **TEIF1** `[3:3]`
    ///
    /// Channel 1 Transfer Error
    /// flag
    #[inline(always)]
    pub const fn teif1(&self) -> bool {
        (self.bits & Intfr::TEIF1_MASK) != 0
    }

    /// Read **HTIF1** `[2:2]`
    ///
    /// Channel 1 Half Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn htif1(&self) -> bool {
        (self.bits & Intfr::HTIF1_MASK) != 0
    }

    /// Read **TCIF1** `[1:1]`
    ///
    /// Channel 1 Transfer Complete
    /// flag
    #[inline(always)]
    pub const fn tcif1(&self) -> bool {
        (self.bits & Intfr::TCIF1_MASK) != 0
    }

    /// Read **GIF1** `[0:0]`
    ///
    /// Channel 1 Global interrupt
    /// flag
    #[inline(always)]
    pub const fn gif1(&self) -> bool {
        (self.bits & Intfr::GIF1_MASK) != 0
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
}

/// INTFCR Register
///
/// DMA interrupt flag clear register
/// (DMA_INTFCR)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CTEIF8** `[31]` (WO)
///
///   Channel 8 Transfer Error
///   clear
///
/// - **CHTIF8** `[30]` (WO)
///
///   Channel 8 Half Transfer
///   clear
///
/// - **CTCIF8** `[29]` (WO)
///
///   Channel 8 Transfer Complete
///   clear
///
/// - **CGIF8** `[28]` (WO)
///
///   Channel 8 Global interrupt
///   clear
///
/// - **CTEIF7** `[27]` (WO)
///
///   Channel 7 Transfer Error
///   clear
///
/// - **CHTIF7** `[26]` (WO)
///
///   Channel 7 Half Transfer
///   clear
///
/// - **CTCIF7** `[25]` (WO)
///
///   Channel 7 Transfer Complete
///   clear
///
/// - **CGIF7** `[24]` (WO)
///
///   Channel 7 Global interrupt
///   clear
///
/// - **CTEIF6** `[23]` (WO)
///
///   Channel 6 Transfer Error
///   clear
///
/// - **CHTIF6** `[22]` (WO)
///
///   Channel 6 Half Transfer
///   clear
///
/// - **CTCIF6** `[21]` (WO)
///
///   Channel 6 Transfer Complete
///   clear
///
/// - **CGIF6** `[20]` (WO)
///
///   Channel 6 Global interrupt
///   clear
///
/// - **CTEIF5** `[19]` (WO)
///
///   Channel 5 Transfer Error
///   clear
///
/// - **CHTIF5** `[18]` (WO)
///
///   Channel 5 Half Transfer
///   clear
///
/// - **CTCIF5** `[17]` (WO)
///
///   Channel 5 Transfer Complete
///   clear
///
/// - **CGIF5** `[16]` (WO)
///
///   Channel 5 Global interrupt
///   clear
///
/// - **CTEIF4** `[15]` (WO)
///
///   Channel 4 Transfer Error
///   clear
///
/// - **CHTIF4** `[14]` (WO)
///
///   Channel 4 Half Transfer
///   clear
///
/// - **CTCIF4** `[13]` (WO)
///
///   Channel 4 Transfer Complete
///   clear
///
/// - **CGIF4** `[12]` (WO)
///
///   Channel 4 Global interrupt
///   clear
///
/// - **CTEIF3** `[11]` (WO)
///
///   Channel 3 Transfer Error
///   clear
///
/// - **CHTIF3** `[10]` (WO)
///
///   Channel 3 Half Transfer
///   clear
///
/// - **CTCIF3** `[9]` (WO)
///
///   Channel 3 Transfer Complete
///   clear
///
/// - **CGIF3** `[8]` (WO)
///
///   Channel 3 Global interrupt
///   clear
///
/// - **CTEIF2** `[7]` (WO)
///
///   Channel 2 Transfer Error
///   clear
///
/// - **CHTIF2** `[6]` (WO)
///
///   Channel 2 Half Transfer
///   clear
///
/// - **CTCIF2** `[5]` (WO)
///
///   Channel 2 Transfer Complete
///   clear
///
/// - **CGIF2** `[4]` (WO)
///
///   Channel 2 Global interrupt
///   clear
///
/// - **CTEIF1** `[3]` (WO)
///
///   Channel 1 Transfer Error
///   clear
///
/// - **CHTIF1** `[2]` (WO)
///
///   Channel 1 Half Transfer
///   clear
///
/// - **CTCIF1** `[1]` (WO)
///
///   Channel 1 Transfer Complete
///   clear
///
/// - **CGIF1** `[0]` (WO)
///
///   Channel 1 Global interrupt
///   clear
#[repr(transparent)]
pub struct Intfcr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intfcr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intfcr {
    // ======== Field Constants ========

    /// **CTEIF8**
    ///
    /// Channel 8 Transfer Error
    /// clear
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CTEIF8_OFFSET: u32 = 31;
    pub const CTEIF8_MASK: u32 = 0x80000000;
    pub const CTEIF8: u32 = 1 << 31;

    /// **CHTIF8**
    ///
    /// Channel 8 Half Transfer
    /// clear
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CHTIF8_OFFSET: u32 = 30;
    pub const CHTIF8_MASK: u32 = 0x40000000;
    pub const CHTIF8: u32 = 1 << 30;

    /// **CTCIF8**
    ///
    /// Channel 8 Transfer Complete
    /// clear
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CTCIF8_OFFSET: u32 = 29;
    pub const CTCIF8_MASK: u32 = 0x20000000;
    pub const CTCIF8: u32 = 1 << 29;

    /// **CGIF8**
    ///
    /// Channel 8 Global interrupt
    /// clear
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CGIF8_OFFSET: u32 = 28;
    pub const CGIF8_MASK: u32 = 0x10000000;
    pub const CGIF8: u32 = 1 << 28;

    /// **CTEIF7**
    ///
    /// Channel 7 Transfer Error
    /// clear
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CTEIF7_OFFSET: u32 = 27;
    pub const CTEIF7_MASK: u32 = 0x08000000;
    pub const CTEIF7: u32 = 1 << 27;

    /// **CHTIF7**
    ///
    /// Channel 7 Half Transfer
    /// clear
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CHTIF7_OFFSET: u32 = 26;
    pub const CHTIF7_MASK: u32 = 0x04000000;
    pub const CHTIF7: u32 = 1 << 26;

    /// **CTCIF7**
    ///
    /// Channel 7 Transfer Complete
    /// clear
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CTCIF7_OFFSET: u32 = 25;
    pub const CTCIF7_MASK: u32 = 0x02000000;
    pub const CTCIF7: u32 = 1 << 25;

    /// **CGIF7**
    ///
    /// Channel 7 Global interrupt
    /// clear
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CGIF7_OFFSET: u32 = 24;
    pub const CGIF7_MASK: u32 = 0x01000000;
    pub const CGIF7: u32 = 1 << 24;

    /// **CTEIF6**
    ///
    /// Channel 6 Transfer Error
    /// clear
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CTEIF6_OFFSET: u32 = 23;
    pub const CTEIF6_MASK: u32 = 0x00800000;
    pub const CTEIF6: u32 = 1 << 23;

    /// **CHTIF6**
    ///
    /// Channel 6 Half Transfer
    /// clear
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CHTIF6_OFFSET: u32 = 22;
    pub const CHTIF6_MASK: u32 = 0x00400000;
    pub const CHTIF6: u32 = 1 << 22;

    /// **CTCIF6**
    ///
    /// Channel 6 Transfer Complete
    /// clear
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CTCIF6_OFFSET: u32 = 21;
    pub const CTCIF6_MASK: u32 = 0x00200000;
    pub const CTCIF6: u32 = 1 << 21;

    /// **CGIF6**
    ///
    /// Channel 6 Global interrupt
    /// clear
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CGIF6_OFFSET: u32 = 20;
    pub const CGIF6_MASK: u32 = 0x00100000;
    pub const CGIF6: u32 = 1 << 20;

    /// **CTEIF5**
    ///
    /// Channel 5 Transfer Error
    /// clear
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CTEIF5_OFFSET: u32 = 19;
    pub const CTEIF5_MASK: u32 = 0x00080000;
    pub const CTEIF5: u32 = 1 << 19;

    /// **CHTIF5**
    ///
    /// Channel 5 Half Transfer
    /// clear
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CHTIF5_OFFSET: u32 = 18;
    pub const CHTIF5_MASK: u32 = 0x00040000;
    pub const CHTIF5: u32 = 1 << 18;

    /// **CTCIF5**
    ///
    /// Channel 5 Transfer Complete
    /// clear
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CTCIF5_OFFSET: u32 = 17;
    pub const CTCIF5_MASK: u32 = 0x00020000;
    pub const CTCIF5: u32 = 1 << 17;

    /// **CGIF5**
    ///
    /// Channel 5 Global interrupt
    /// clear
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CGIF5_OFFSET: u32 = 16;
    pub const CGIF5_MASK: u32 = 0x00010000;
    pub const CGIF5: u32 = 1 << 16;

    /// **CTEIF4**
    ///
    /// Channel 4 Transfer Error
    /// clear
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CTEIF4_OFFSET: u32 = 15;
    pub const CTEIF4_MASK: u32 = 0x00008000;
    pub const CTEIF4: u32 = 1 << 15;

    /// **CHTIF4**
    ///
    /// Channel 4 Half Transfer
    /// clear
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CHTIF4_OFFSET: u32 = 14;
    pub const CHTIF4_MASK: u32 = 0x00004000;
    pub const CHTIF4: u32 = 1 << 14;

    /// **CTCIF4**
    ///
    /// Channel 4 Transfer Complete
    /// clear
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CTCIF4_OFFSET: u32 = 13;
    pub const CTCIF4_MASK: u32 = 0x00002000;
    pub const CTCIF4: u32 = 1 << 13;

    /// **CGIF4**
    ///
    /// Channel 4 Global interrupt
    /// clear
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CGIF4_OFFSET: u32 = 12;
    pub const CGIF4_MASK: u32 = 0x00001000;
    pub const CGIF4: u32 = 1 << 12;

    /// **CTEIF3**
    ///
    /// Channel 3 Transfer Error
    /// clear
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CTEIF3_OFFSET: u32 = 11;
    pub const CTEIF3_MASK: u32 = 0x00000800;
    pub const CTEIF3: u32 = 1 << 11;

    /// **CHTIF3**
    ///
    /// Channel 3 Half Transfer
    /// clear
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CHTIF3_OFFSET: u32 = 10;
    pub const CHTIF3_MASK: u32 = 0x00000400;
    pub const CHTIF3: u32 = 1 << 10;

    /// **CTCIF3**
    ///
    /// Channel 3 Transfer Complete
    /// clear
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CTCIF3_OFFSET: u32 = 9;
    pub const CTCIF3_MASK: u32 = 0x00000200;
    pub const CTCIF3: u32 = 1 << 9;

    /// **CGIF3**
    ///
    /// Channel 3 Global interrupt
    /// clear
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CGIF3_OFFSET: u32 = 8;
    pub const CGIF3_MASK: u32 = 0x00000100;
    pub const CGIF3: u32 = 1 << 8;

    /// **CTEIF2**
    ///
    /// Channel 2 Transfer Error
    /// clear
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CTEIF2_OFFSET: u32 = 7;
    pub const CTEIF2_MASK: u32 = 0x00000080;
    pub const CTEIF2: u32 = 1 << 7;

    /// **CHTIF2**
    ///
    /// Channel 2 Half Transfer
    /// clear
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CHTIF2_OFFSET: u32 = 6;
    pub const CHTIF2_MASK: u32 = 0x00000040;
    pub const CHTIF2: u32 = 1 << 6;

    /// **CTCIF2**
    ///
    /// Channel 2 Transfer Complete
    /// clear
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CTCIF2_OFFSET: u32 = 5;
    pub const CTCIF2_MASK: u32 = 0x00000020;
    pub const CTCIF2: u32 = 1 << 5;

    /// **CGIF2**
    ///
    /// Channel 2 Global interrupt
    /// clear
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CGIF2_OFFSET: u32 = 4;
    pub const CGIF2_MASK: u32 = 0x00000010;
    pub const CGIF2: u32 = 1 << 4;

    /// **CTEIF1**
    ///
    /// Channel 1 Transfer Error
    /// clear
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CTEIF1_OFFSET: u32 = 3;
    pub const CTEIF1_MASK: u32 = 0x00000008;
    pub const CTEIF1: u32 = 1 << 3;

    /// **CHTIF1**
    ///
    /// Channel 1 Half Transfer
    /// clear
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CHTIF1_OFFSET: u32 = 2;
    pub const CHTIF1_MASK: u32 = 0x00000004;
    pub const CHTIF1: u32 = 1 << 2;

    /// **CTCIF1**
    ///
    /// Channel 1 Transfer Complete
    /// clear
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CTCIF1_OFFSET: u32 = 1;
    pub const CTCIF1_MASK: u32 = 0x00000002;
    pub const CTCIF1: u32 = 1 << 1;

    /// **CGIF1**
    ///
    /// Channel 1 Global interrupt
    /// clear
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CGIF1_OFFSET: u32 = 0;
    pub const CGIF1_MASK: u32 = 0x00000001;
    pub const CGIF1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CTEIF8** (WO)
    ///
    /// Channel 8 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif8(&self) {
        self.value.write_bits(Self::CTEIF8_MASK);
    }

    /// Trigger **CHTIF8** (WO)
    ///
    /// Channel 8 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif8(&self) {
        self.value.write_bits(Self::CHTIF8_MASK);
    }

    /// Trigger **CTCIF8** (WO)
    ///
    /// Channel 8 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif8(&self) {
        self.value.write_bits(Self::CTCIF8_MASK);
    }

    /// Trigger **CGIF8** (WO)
    ///
    /// Channel 8 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif8(&self) {
        self.value.write_bits(Self::CGIF8_MASK);
    }

    /// Trigger **CTEIF7** (WO)
    ///
    /// Channel 7 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif7(&self) {
        self.value.write_bits(Self::CTEIF7_MASK);
    }

    /// Trigger **CHTIF7** (WO)
    ///
    /// Channel 7 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif7(&self) {
        self.value.write_bits(Self::CHTIF7_MASK);
    }

    /// Trigger **CTCIF7** (WO)
    ///
    /// Channel 7 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif7(&self) {
        self.value.write_bits(Self::CTCIF7_MASK);
    }

    /// Trigger **CGIF7** (WO)
    ///
    /// Channel 7 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif7(&self) {
        self.value.write_bits(Self::CGIF7_MASK);
    }

    /// Trigger **CTEIF6** (WO)
    ///
    /// Channel 6 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif6(&self) {
        self.value.write_bits(Self::CTEIF6_MASK);
    }

    /// Trigger **CHTIF6** (WO)
    ///
    /// Channel 6 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif6(&self) {
        self.value.write_bits(Self::CHTIF6_MASK);
    }

    /// Trigger **CTCIF6** (WO)
    ///
    /// Channel 6 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif6(&self) {
        self.value.write_bits(Self::CTCIF6_MASK);
    }

    /// Trigger **CGIF6** (WO)
    ///
    /// Channel 6 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif6(&self) {
        self.value.write_bits(Self::CGIF6_MASK);
    }

    /// Trigger **CTEIF5** (WO)
    ///
    /// Channel 5 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif5(&self) {
        self.value.write_bits(Self::CTEIF5_MASK);
    }

    /// Trigger **CHTIF5** (WO)
    ///
    /// Channel 5 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif5(&self) {
        self.value.write_bits(Self::CHTIF5_MASK);
    }

    /// Trigger **CTCIF5** (WO)
    ///
    /// Channel 5 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif5(&self) {
        self.value.write_bits(Self::CTCIF5_MASK);
    }

    /// Trigger **CGIF5** (WO)
    ///
    /// Channel 5 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif5(&self) {
        self.value.write_bits(Self::CGIF5_MASK);
    }

    /// Trigger **CTEIF4** (WO)
    ///
    /// Channel 4 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif4(&self) {
        self.value.write_bits(Self::CTEIF4_MASK);
    }

    /// Trigger **CHTIF4** (WO)
    ///
    /// Channel 4 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif4(&self) {
        self.value.write_bits(Self::CHTIF4_MASK);
    }

    /// Trigger **CTCIF4** (WO)
    ///
    /// Channel 4 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif4(&self) {
        self.value.write_bits(Self::CTCIF4_MASK);
    }

    /// Trigger **CGIF4** (WO)
    ///
    /// Channel 4 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif4(&self) {
        self.value.write_bits(Self::CGIF4_MASK);
    }

    /// Trigger **CTEIF3** (WO)
    ///
    /// Channel 3 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif3(&self) {
        self.value.write_bits(Self::CTEIF3_MASK);
    }

    /// Trigger **CHTIF3** (WO)
    ///
    /// Channel 3 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif3(&self) {
        self.value.write_bits(Self::CHTIF3_MASK);
    }

    /// Trigger **CTCIF3** (WO)
    ///
    /// Channel 3 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif3(&self) {
        self.value.write_bits(Self::CTCIF3_MASK);
    }

    /// Trigger **CGIF3** (WO)
    ///
    /// Channel 3 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif3(&self) {
        self.value.write_bits(Self::CGIF3_MASK);
    }

    /// Trigger **CTEIF2** (WO)
    ///
    /// Channel 2 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif2(&self) {
        self.value.write_bits(Self::CTEIF2_MASK);
    }

    /// Trigger **CHTIF2** (WO)
    ///
    /// Channel 2 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif2(&self) {
        self.value.write_bits(Self::CHTIF2_MASK);
    }

    /// Trigger **CTCIF2** (WO)
    ///
    /// Channel 2 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif2(&self) {
        self.value.write_bits(Self::CTCIF2_MASK);
    }

    /// Trigger **CGIF2** (WO)
    ///
    /// Channel 2 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif2(&self) {
        self.value.write_bits(Self::CGIF2_MASK);
    }

    /// Trigger **CTEIF1** (WO)
    ///
    /// Channel 1 Transfer Error
    /// clear
    #[inline(always)]
    pub fn trigger_cteif1(&self) {
        self.value.write_bits(Self::CTEIF1_MASK);
    }

    /// Trigger **CHTIF1** (WO)
    ///
    /// Channel 1 Half Transfer
    /// clear
    #[inline(always)]
    pub fn trigger_chtif1(&self) {
        self.value.write_bits(Self::CHTIF1_MASK);
    }

    /// Trigger **CTCIF1** (WO)
    ///
    /// Channel 1 Transfer Complete
    /// clear
    #[inline(always)]
    pub fn trigger_ctcif1(&self) {
        self.value.write_bits(Self::CTCIF1_MASK);
    }

    /// Trigger **CGIF1** (WO)
    ///
    /// Channel 1 Global interrupt
    /// clear
    #[inline(always)]
    pub fn trigger_cgif1(&self) {
        self.value.write_bits(Self::CGIF1_MASK);
    }
}

/// INTFCR value type for batch read/modify/write operations
///
/// Use `intfcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intfcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntfcrVal {
    bits: u32,
}

impl IntfcrVal {
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

    /// Set **CTEIF8** `[31:31]`
    ///
    /// Channel 8 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF8_MASK
            } else {
                self.bits & !Intfcr::CTEIF8_MASK
            },
        }
    }

    /// Set **CHTIF8** `[30:30]`
    ///
    /// Channel 8 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF8_MASK
            } else {
                self.bits & !Intfcr::CHTIF8_MASK
            },
        }
    }

    /// Set **CTCIF8** `[29:29]`
    ///
    /// Channel 8 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF8_MASK
            } else {
                self.bits & !Intfcr::CTCIF8_MASK
            },
        }
    }

    /// Set **CGIF8** `[28:28]`
    ///
    /// Channel 8 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF8_MASK
            } else {
                self.bits & !Intfcr::CGIF8_MASK
            },
        }
    }

    /// Set **CTEIF7** `[27:27]`
    ///
    /// Channel 7 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF7_MASK
            } else {
                self.bits & !Intfcr::CTEIF7_MASK
            },
        }
    }

    /// Set **CHTIF7** `[26:26]`
    ///
    /// Channel 7 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF7_MASK
            } else {
                self.bits & !Intfcr::CHTIF7_MASK
            },
        }
    }

    /// Set **CTCIF7** `[25:25]`
    ///
    /// Channel 7 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF7_MASK
            } else {
                self.bits & !Intfcr::CTCIF7_MASK
            },
        }
    }

    /// Set **CGIF7** `[24:24]`
    ///
    /// Channel 7 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF7_MASK
            } else {
                self.bits & !Intfcr::CGIF7_MASK
            },
        }
    }

    /// Set **CTEIF6** `[23:23]`
    ///
    /// Channel 6 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF6_MASK
            } else {
                self.bits & !Intfcr::CTEIF6_MASK
            },
        }
    }

    /// Set **CHTIF6** `[22:22]`
    ///
    /// Channel 6 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF6_MASK
            } else {
                self.bits & !Intfcr::CHTIF6_MASK
            },
        }
    }

    /// Set **CTCIF6** `[21:21]`
    ///
    /// Channel 6 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF6_MASK
            } else {
                self.bits & !Intfcr::CTCIF6_MASK
            },
        }
    }

    /// Set **CGIF6** `[20:20]`
    ///
    /// Channel 6 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF6_MASK
            } else {
                self.bits & !Intfcr::CGIF6_MASK
            },
        }
    }

    /// Set **CTEIF5** `[19:19]`
    ///
    /// Channel 5 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF5_MASK
            } else {
                self.bits & !Intfcr::CTEIF5_MASK
            },
        }
    }

    /// Set **CHTIF5** `[18:18]`
    ///
    /// Channel 5 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF5_MASK
            } else {
                self.bits & !Intfcr::CHTIF5_MASK
            },
        }
    }

    /// Set **CTCIF5** `[17:17]`
    ///
    /// Channel 5 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF5_MASK
            } else {
                self.bits & !Intfcr::CTCIF5_MASK
            },
        }
    }

    /// Set **CGIF5** `[16:16]`
    ///
    /// Channel 5 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF5_MASK
            } else {
                self.bits & !Intfcr::CGIF5_MASK
            },
        }
    }

    /// Set **CTEIF4** `[15:15]`
    ///
    /// Channel 4 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF4_MASK
            } else {
                self.bits & !Intfcr::CTEIF4_MASK
            },
        }
    }

    /// Set **CHTIF4** `[14:14]`
    ///
    /// Channel 4 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF4_MASK
            } else {
                self.bits & !Intfcr::CHTIF4_MASK
            },
        }
    }

    /// Set **CTCIF4** `[13:13]`
    ///
    /// Channel 4 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF4_MASK
            } else {
                self.bits & !Intfcr::CTCIF4_MASK
            },
        }
    }

    /// Set **CGIF4** `[12:12]`
    ///
    /// Channel 4 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF4_MASK
            } else {
                self.bits & !Intfcr::CGIF4_MASK
            },
        }
    }

    /// Set **CTEIF3** `[11:11]`
    ///
    /// Channel 3 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF3_MASK
            } else {
                self.bits & !Intfcr::CTEIF3_MASK
            },
        }
    }

    /// Set **CHTIF3** `[10:10]`
    ///
    /// Channel 3 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF3_MASK
            } else {
                self.bits & !Intfcr::CHTIF3_MASK
            },
        }
    }

    /// Set **CTCIF3** `[9:9]`
    ///
    /// Channel 3 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF3_MASK
            } else {
                self.bits & !Intfcr::CTCIF3_MASK
            },
        }
    }

    /// Set **CGIF3** `[8:8]`
    ///
    /// Channel 3 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF3_MASK
            } else {
                self.bits & !Intfcr::CGIF3_MASK
            },
        }
    }

    /// Set **CTEIF2** `[7:7]`
    ///
    /// Channel 2 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF2_MASK
            } else {
                self.bits & !Intfcr::CTEIF2_MASK
            },
        }
    }

    /// Set **CHTIF2** `[6:6]`
    ///
    /// Channel 2 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF2_MASK
            } else {
                self.bits & !Intfcr::CHTIF2_MASK
            },
        }
    }

    /// Set **CTCIF2** `[5:5]`
    ///
    /// Channel 2 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF2_MASK
            } else {
                self.bits & !Intfcr::CTCIF2_MASK
            },
        }
    }

    /// Set **CGIF2** `[4:4]`
    ///
    /// Channel 2 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF2_MASK
            } else {
                self.bits & !Intfcr::CGIF2_MASK
            },
        }
    }

    /// Set **CTEIF1** `[3:3]`
    ///
    /// Channel 1 Transfer Error
    /// clear
    #[inline(always)]
    pub const fn with_cteif1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTEIF1_MASK
            } else {
                self.bits & !Intfcr::CTEIF1_MASK
            },
        }
    }

    /// Set **CHTIF1** `[2:2]`
    ///
    /// Channel 1 Half Transfer
    /// clear
    #[inline(always)]
    pub const fn with_chtif1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CHTIF1_MASK
            } else {
                self.bits & !Intfcr::CHTIF1_MASK
            },
        }
    }

    /// Set **CTCIF1** `[1:1]`
    ///
    /// Channel 1 Transfer Complete
    /// clear
    #[inline(always)]
    pub const fn with_ctcif1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CTCIF1_MASK
            } else {
                self.bits & !Intfcr::CTCIF1_MASK
            },
        }
    }

    /// Set **CGIF1** `[0:0]`
    ///
    /// Channel 1 Global interrupt
    /// clear
    #[inline(always)]
    pub const fn with_cgif1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfcr::CGIF1_MASK
            } else {
                self.bits & !Intfcr::CGIF1_MASK
            },
        }
    }
}

impl Default for IntfcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intfcr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntfcrVal) {
        self.value.write_bits(val.bits())
    }
}

/// CFGR1 Register
///
/// DMA channel 1 configuration register
/// (DMA_CFGR1)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
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

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr1::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr1::PL_MASK) >> Cfgr1::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr1::MSIZE_MASK) >> Cfgr1::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr1::PSIZE_MASK) >> Cfgr1::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr1::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr1::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr1::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr1::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr1::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr1::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr1::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr1::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr1::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr1::PL_MASK) | ((value << Cfgr1::PL_OFFSET) & Cfgr1::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr1::MSIZE_MASK)
                | ((value << Cfgr1::MSIZE_OFFSET) & Cfgr1::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr1::PSIZE_MASK)
                | ((value << Cfgr1::PSIZE_OFFSET) & Cfgr1::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::MINC_MASK
            } else {
                self.bits & !Cfgr1::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::PINC_MASK
            } else {
                self.bits & !Cfgr1::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::CIRC_MASK
            } else {
                self.bits & !Cfgr1::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::DIR_MASK
            } else {
                self.bits & !Cfgr1::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::TEIE_MASK
            } else {
                self.bits & !Cfgr1::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::HTIE_MASK
            } else {
                self.bits & !Cfgr1::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::TCIE_MASK
            } else {
                self.bits & !Cfgr1::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr1::EN_MASK
            } else {
                self.bits & !Cfgr1::EN_MASK
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

/// CNTR1 Register
///
/// DMA channel 1 number of data
/// register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr1 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR1 value type for batch read/modify/write operations
///
/// Use `cntr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr1Val {
    bits: u32,
}

impl Cntr1Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr1::NDT_MASK) >> Cntr1::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr1::NDT_MASK) | ((value << Cntr1::NDT_OFFSET) & Cntr1::NDT_MASK),
        }
    }
}

impl Default for Cntr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr1Val {
        Cntr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr1Val) -> Cntr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR1 Register
///
/// DMA channel 1 peripheral address
/// register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr1 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR1 value type for batch read/modify/write operations
///
/// Use `paddr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr1Val {
    bits: u32,
}

impl Paddr1Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr1::PA_MASK) >> Paddr1::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr1::PA_MASK) | ((value << Paddr1::PA_OFFSET) & Paddr1::PA_MASK),
        }
    }
}

impl Default for Paddr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr1Val {
        Paddr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr1Val) -> Paddr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR1 Register
///
/// DMA channel 1 memory address
/// register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr1 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR1 value type for batch read/modify/write operations
///
/// Use `maddr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr1Val {
    bits: u32,
}

impl Maddr1Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr1::MA_MASK) >> Maddr1::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr1::MA_MASK) | ((value << Maddr1::MA_OFFSET) & Maddr1::MA_MASK),
        }
    }
}

impl Default for Maddr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr1Val {
        Maddr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr1Val) -> Maddr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR2 Register
///
/// DMA channel 2 configuration register
/// (DMA_CFGR2)
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
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

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr2::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr2::PL_MASK) >> Cfgr2::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr2::MSIZE_MASK) >> Cfgr2::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr2::PSIZE_MASK) >> Cfgr2::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr2::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr2::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr2::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr2::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr2::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr2::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr2::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr2::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr2::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr2::PL_MASK) | ((value << Cfgr2::PL_OFFSET) & Cfgr2::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr2::MSIZE_MASK)
                | ((value << Cfgr2::MSIZE_OFFSET) & Cfgr2::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr2::PSIZE_MASK)
                | ((value << Cfgr2::PSIZE_OFFSET) & Cfgr2::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::MINC_MASK
            } else {
                self.bits & !Cfgr2::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::PINC_MASK
            } else {
                self.bits & !Cfgr2::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::CIRC_MASK
            } else {
                self.bits & !Cfgr2::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::DIR_MASK
            } else {
                self.bits & !Cfgr2::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::TEIE_MASK
            } else {
                self.bits & !Cfgr2::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::HTIE_MASK
            } else {
                self.bits & !Cfgr2::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::TCIE_MASK
            } else {
                self.bits & !Cfgr2::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr2::EN_MASK
            } else {
                self.bits & !Cfgr2::EN_MASK
            },
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

/// CNTR2 Register
///
/// DMA channel 2 number of data
/// register
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr2 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR2 value type for batch read/modify/write operations
///
/// Use `cntr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr2Val {
    bits: u32,
}

impl Cntr2Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr2::NDT_MASK) >> Cntr2::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr2::NDT_MASK) | ((value << Cntr2::NDT_OFFSET) & Cntr2::NDT_MASK),
        }
    }
}

impl Default for Cntr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr2Val {
        Cntr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr2Val) -> Cntr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR2 Register
///
/// DMA channel 2 peripheral address
/// register
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr2 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR2 value type for batch read/modify/write operations
///
/// Use `paddr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr2Val {
    bits: u32,
}

impl Paddr2Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr2::PA_MASK) >> Paddr2::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr2::PA_MASK) | ((value << Paddr2::PA_OFFSET) & Paddr2::PA_MASK),
        }
    }
}

impl Default for Paddr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr2Val {
        Paddr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr2Val) -> Paddr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR2 Register
///
/// DMA channel 2 memory address
/// register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr2 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR2 value type for batch read/modify/write operations
///
/// Use `maddr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr2Val {
    bits: u32,
}

impl Maddr2Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr2::MA_MASK) >> Maddr2::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr2::MA_MASK) | ((value << Maddr2::MA_OFFSET) & Maddr2::MA_MASK),
        }
    }
}

impl Default for Maddr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr2Val {
        Maddr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr2Val) -> Maddr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR3 Register
///
/// DMA channel 3 configuration register
/// (DMA_CFGR3)
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
#[repr(transparent)]
pub struct Cfgr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr3 {
    // ======== Field Constants ========

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CFGR3 value type for batch read/modify/write operations
///
/// Use `cfgr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr3Val {
    bits: u32,
}

impl Cfgr3Val {
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr3::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr3::PL_MASK) >> Cfgr3::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr3::MSIZE_MASK) >> Cfgr3::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr3::PSIZE_MASK) >> Cfgr3::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr3::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr3::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr3::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr3::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr3::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr3::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr3::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr3::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr3::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr3::PL_MASK) | ((value << Cfgr3::PL_OFFSET) & Cfgr3::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr3::MSIZE_MASK)
                | ((value << Cfgr3::MSIZE_OFFSET) & Cfgr3::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr3::PSIZE_MASK)
                | ((value << Cfgr3::PSIZE_OFFSET) & Cfgr3::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::MINC_MASK
            } else {
                self.bits & !Cfgr3::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::PINC_MASK
            } else {
                self.bits & !Cfgr3::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::CIRC_MASK
            } else {
                self.bits & !Cfgr3::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::DIR_MASK
            } else {
                self.bits & !Cfgr3::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::TEIE_MASK
            } else {
                self.bits & !Cfgr3::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::HTIE_MASK
            } else {
                self.bits & !Cfgr3::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::TCIE_MASK
            } else {
                self.bits & !Cfgr3::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr3::EN_MASK
            } else {
                self.bits & !Cfgr3::EN_MASK
            },
        }
    }
}

impl Default for Cfgr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr3Val {
        Cfgr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr3Val) -> Cfgr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNTR3 Register
///
/// DMA channel 3 number of data
/// register
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr3 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR3 value type for batch read/modify/write operations
///
/// Use `cntr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr3Val {
    bits: u32,
}

impl Cntr3Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr3::NDT_MASK) >> Cntr3::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr3::NDT_MASK) | ((value << Cntr3::NDT_OFFSET) & Cntr3::NDT_MASK),
        }
    }
}

impl Default for Cntr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr3Val {
        Cntr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr3Val) -> Cntr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR3 Register
///
/// DMA channel 3 peripheral address
/// register
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr3 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR3 value type for batch read/modify/write operations
///
/// Use `paddr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr3Val {
    bits: u32,
}

impl Paddr3Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr3::PA_MASK) >> Paddr3::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr3::PA_MASK) | ((value << Paddr3::PA_OFFSET) & Paddr3::PA_MASK),
        }
    }
}

impl Default for Paddr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr3Val {
        Paddr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr3Val) -> Paddr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR3 Register
///
/// DMA channel 3 memory address
/// register
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr3 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR3 value type for batch read/modify/write operations
///
/// Use `maddr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr3Val {
    bits: u32,
}

impl Maddr3Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr3::MA_MASK) >> Maddr3::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr3::MA_MASK) | ((value << Maddr3::MA_OFFSET) & Maddr3::MA_MASK),
        }
    }
}

impl Default for Maddr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr3Val {
        Maddr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr3Val) -> Maddr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR4 Register
///
/// DMA channel 4 configuration register
/// (DMA_CFGR4)
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
#[repr(transparent)]
pub struct Cfgr4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr4 {
    // ======== Field Constants ========

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CFGR4 value type for batch read/modify/write operations
///
/// Use `cfgr4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr4Val {
    bits: u32,
}

impl Cfgr4Val {
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr4::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr4::PL_MASK) >> Cfgr4::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr4::MSIZE_MASK) >> Cfgr4::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr4::PSIZE_MASK) >> Cfgr4::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr4::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr4::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr4::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr4::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr4::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr4::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr4::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr4::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr4::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr4::PL_MASK) | ((value << Cfgr4::PL_OFFSET) & Cfgr4::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr4::MSIZE_MASK)
                | ((value << Cfgr4::MSIZE_OFFSET) & Cfgr4::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr4::PSIZE_MASK)
                | ((value << Cfgr4::PSIZE_OFFSET) & Cfgr4::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::MINC_MASK
            } else {
                self.bits & !Cfgr4::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::PINC_MASK
            } else {
                self.bits & !Cfgr4::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::CIRC_MASK
            } else {
                self.bits & !Cfgr4::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::DIR_MASK
            } else {
                self.bits & !Cfgr4::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::TEIE_MASK
            } else {
                self.bits & !Cfgr4::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::HTIE_MASK
            } else {
                self.bits & !Cfgr4::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::TCIE_MASK
            } else {
                self.bits & !Cfgr4::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr4::EN_MASK
            } else {
                self.bits & !Cfgr4::EN_MASK
            },
        }
    }
}

impl Default for Cfgr4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr4Val {
        Cfgr4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr4Val) -> Cfgr4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNTR4 Register
///
/// DMA channel 4 number of data
/// register
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr4 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR4 value type for batch read/modify/write operations
///
/// Use `cntr4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr4Val {
    bits: u32,
}

impl Cntr4Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr4::NDT_MASK) >> Cntr4::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr4::NDT_MASK) | ((value << Cntr4::NDT_OFFSET) & Cntr4::NDT_MASK),
        }
    }
}

impl Default for Cntr4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr4Val {
        Cntr4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr4Val) -> Cntr4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR4 Register
///
/// DMA channel 4 peripheral address
/// register
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr4 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR4 value type for batch read/modify/write operations
///
/// Use `paddr4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr4Val {
    bits: u32,
}

impl Paddr4Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr4::PA_MASK) >> Paddr4::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr4::PA_MASK) | ((value << Paddr4::PA_OFFSET) & Paddr4::PA_MASK),
        }
    }
}

impl Default for Paddr4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr4Val {
        Paddr4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr4Val) -> Paddr4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR4 Register
///
/// DMA channel 4 memory address
/// register
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr4 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR4 value type for batch read/modify/write operations
///
/// Use `maddr4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr4Val {
    bits: u32,
}

impl Maddr4Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr4::MA_MASK) >> Maddr4::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr4::MA_MASK) | ((value << Maddr4::MA_OFFSET) & Maddr4::MA_MASK),
        }
    }
}

impl Default for Maddr4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr4Val {
        Maddr4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr4Val) -> Maddr4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR5 Register
///
/// DMA channel 5 configuration register
/// (DMA_CFGR1)
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
#[repr(transparent)]
pub struct Cfgr5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr5 {
    // ======== Field Constants ========

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CFGR5 value type for batch read/modify/write operations
///
/// Use `cfgr5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr5Val {
    bits: u32,
}

impl Cfgr5Val {
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr5::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr5::PL_MASK) >> Cfgr5::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr5::MSIZE_MASK) >> Cfgr5::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr5::PSIZE_MASK) >> Cfgr5::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr5::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr5::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr5::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr5::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr5::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr5::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr5::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr5::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr5::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr5::PL_MASK) | ((value << Cfgr5::PL_OFFSET) & Cfgr5::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr5::MSIZE_MASK)
                | ((value << Cfgr5::MSIZE_OFFSET) & Cfgr5::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr5::PSIZE_MASK)
                | ((value << Cfgr5::PSIZE_OFFSET) & Cfgr5::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::MINC_MASK
            } else {
                self.bits & !Cfgr5::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::PINC_MASK
            } else {
                self.bits & !Cfgr5::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::CIRC_MASK
            } else {
                self.bits & !Cfgr5::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::DIR_MASK
            } else {
                self.bits & !Cfgr5::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::TEIE_MASK
            } else {
                self.bits & !Cfgr5::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::HTIE_MASK
            } else {
                self.bits & !Cfgr5::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::TCIE_MASK
            } else {
                self.bits & !Cfgr5::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr5::EN_MASK
            } else {
                self.bits & !Cfgr5::EN_MASK
            },
        }
    }
}

impl Default for Cfgr5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr5Val {
        Cfgr5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr5Val) -> Cfgr5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNTR5 Register
///
/// DMA channel 5 number of data
/// register
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr5 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR5 value type for batch read/modify/write operations
///
/// Use `cntr5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr5Val {
    bits: u32,
}

impl Cntr5Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr5::NDT_MASK) >> Cntr5::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr5::NDT_MASK) | ((value << Cntr5::NDT_OFFSET) & Cntr5::NDT_MASK),
        }
    }
}

impl Default for Cntr5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr5Val {
        Cntr5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr5Val) -> Cntr5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR5 Register
///
/// DMA channel 5 peripheral address
/// register
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr5 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR5 value type for batch read/modify/write operations
///
/// Use `paddr5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr5Val {
    bits: u32,
}

impl Paddr5Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr5::PA_MASK) >> Paddr5::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr5::PA_MASK) | ((value << Paddr5::PA_OFFSET) & Paddr5::PA_MASK),
        }
    }
}

impl Default for Paddr5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr5Val {
        Paddr5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr5Val) -> Paddr5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR5 Register
///
/// DMA channel 5 memory address
/// register
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr5 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR5 value type for batch read/modify/write operations
///
/// Use `maddr5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr5Val {
    bits: u32,
}

impl Maddr5Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr5::MA_MASK) >> Maddr5::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr5::MA_MASK) | ((value << Maddr5::MA_OFFSET) & Maddr5::MA_MASK),
        }
    }
}

impl Default for Maddr5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr5Val {
        Maddr5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr5Val) -> Maddr5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR6 Register
///
/// DMA channel 6 configuration register
/// (DMA_CFGR1)
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
#[repr(transparent)]
pub struct Cfgr6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr6 {
    // ======== Field Constants ========

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CFGR6 value type for batch read/modify/write operations
///
/// Use `cfgr6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr6Val {
    bits: u32,
}

impl Cfgr6Val {
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr6::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr6::PL_MASK) >> Cfgr6::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr6::MSIZE_MASK) >> Cfgr6::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr6::PSIZE_MASK) >> Cfgr6::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr6::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr6::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr6::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr6::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr6::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr6::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr6::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr6::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr6::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr6::PL_MASK) | ((value << Cfgr6::PL_OFFSET) & Cfgr6::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr6::MSIZE_MASK)
                | ((value << Cfgr6::MSIZE_OFFSET) & Cfgr6::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr6::PSIZE_MASK)
                | ((value << Cfgr6::PSIZE_OFFSET) & Cfgr6::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::MINC_MASK
            } else {
                self.bits & !Cfgr6::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::PINC_MASK
            } else {
                self.bits & !Cfgr6::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::CIRC_MASK
            } else {
                self.bits & !Cfgr6::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::DIR_MASK
            } else {
                self.bits & !Cfgr6::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::TEIE_MASK
            } else {
                self.bits & !Cfgr6::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::HTIE_MASK
            } else {
                self.bits & !Cfgr6::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::TCIE_MASK
            } else {
                self.bits & !Cfgr6::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr6::EN_MASK
            } else {
                self.bits & !Cfgr6::EN_MASK
            },
        }
    }
}

impl Default for Cfgr6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr6Val {
        Cfgr6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr6Val) -> Cfgr6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNTR6 Register
///
/// DMA channel 6 number of data
/// register
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr6 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR6 value type for batch read/modify/write operations
///
/// Use `cntr6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr6Val {
    bits: u32,
}

impl Cntr6Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr6::NDT_MASK) >> Cntr6::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr6::NDT_MASK) | ((value << Cntr6::NDT_OFFSET) & Cntr6::NDT_MASK),
        }
    }
}

impl Default for Cntr6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr6Val {
        Cntr6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr6Val) -> Cntr6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR6 Register
///
/// DMA channel 6 peripheral address
/// register
///
/// Address offset: 0x0074
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr6 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR6 value type for batch read/modify/write operations
///
/// Use `paddr6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr6Val {
    bits: u32,
}

impl Paddr6Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr6::PA_MASK) >> Paddr6::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr6::PA_MASK) | ((value << Paddr6::PA_OFFSET) & Paddr6::PA_MASK),
        }
    }
}

impl Default for Paddr6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr6Val {
        Paddr6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr6Val) -> Paddr6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR6 Register
///
/// DMA channel 6 memory address
/// register
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr6 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR6 value type for batch read/modify/write operations
///
/// Use `maddr6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr6Val {
    bits: u32,
}

impl Maddr6Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr6::MA_MASK) >> Maddr6::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr6::MA_MASK) | ((value << Maddr6::MA_OFFSET) & Maddr6::MA_MASK),
        }
    }
}

impl Default for Maddr6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr6Val {
        Maddr6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr6Val) -> Maddr6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR7 Register
///
/// DMA channel 7 configuration register
/// (DMA_CFGR1)
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
#[repr(transparent)]
pub struct Cfgr7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr7 {
    // ======== Field Constants ========

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CFGR7 value type for batch read/modify/write operations
///
/// Use `cfgr7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr7Val {
    bits: u32,
}

impl Cfgr7Val {
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr7::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr7::PL_MASK) >> Cfgr7::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr7::MSIZE_MASK) >> Cfgr7::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr7::PSIZE_MASK) >> Cfgr7::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr7::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr7::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr7::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr7::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr7::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr7::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr7::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr7::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr7::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr7::PL_MASK) | ((value << Cfgr7::PL_OFFSET) & Cfgr7::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr7::MSIZE_MASK)
                | ((value << Cfgr7::MSIZE_OFFSET) & Cfgr7::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr7::PSIZE_MASK)
                | ((value << Cfgr7::PSIZE_OFFSET) & Cfgr7::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::MINC_MASK
            } else {
                self.bits & !Cfgr7::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::PINC_MASK
            } else {
                self.bits & !Cfgr7::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::CIRC_MASK
            } else {
                self.bits & !Cfgr7::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::DIR_MASK
            } else {
                self.bits & !Cfgr7::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::TEIE_MASK
            } else {
                self.bits & !Cfgr7::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::HTIE_MASK
            } else {
                self.bits & !Cfgr7::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::TCIE_MASK
            } else {
                self.bits & !Cfgr7::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr7::EN_MASK
            } else {
                self.bits & !Cfgr7::EN_MASK
            },
        }
    }
}

impl Default for Cfgr7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr7Val {
        Cfgr7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr7Val) -> Cfgr7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNTR7 Register
///
/// DMA channel 7 number of data
/// register
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr7 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR7 value type for batch read/modify/write operations
///
/// Use `cntr7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr7Val {
    bits: u32,
}

impl Cntr7Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr7::NDT_MASK) >> Cntr7::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr7::NDT_MASK) | ((value << Cntr7::NDT_OFFSET) & Cntr7::NDT_MASK),
        }
    }
}

impl Default for Cntr7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr7Val {
        Cntr7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr7Val) -> Cntr7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR7 Register
///
/// DMA channel 7 peripheral address
/// register
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr7 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR7 value type for batch read/modify/write operations
///
/// Use `paddr7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr7Val {
    bits: u32,
}

impl Paddr7Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr7::PA_MASK) >> Paddr7::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr7::PA_MASK) | ((value << Paddr7::PA_OFFSET) & Paddr7::PA_MASK),
        }
    }
}

impl Default for Paddr7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr7Val {
        Paddr7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr7Val) -> Paddr7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR7 Register
///
/// DMA channel 7 memory address
/// register
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr7 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR7 value type for batch read/modify/write operations
///
/// Use `maddr7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr7Val {
    bits: u32,
}

impl Maddr7Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr7::MA_MASK) >> Maddr7::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr7::MA_MASK) | ((value << Maddr7::MA_OFFSET) & Maddr7::MA_MASK),
        }
    }
}

impl Default for Maddr7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr7Val {
        Maddr7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr7Val) -> Maddr7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGR8 Register
///
/// DMA channel 8 configuration register
/// (DMA_CFGR1)
///
/// Address offset: 0x0094
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MEM2MEM** `[14]` (RW)
///
///   Memory to memory mode
///
/// - **PL** `[13:12]` (RW)
///
///   Channel Priority level
///
/// - **MSIZE** `[11:10]` (RW)
///
///   Memory size
///
/// - **PSIZE** `[9:8]` (RW)
///
///   Peripheral size
///
/// - **MINC** `[7]` (RW)
///
///   Memory increment mode
///
/// - **PINC** `[6]` (RW)
///
///   Peripheral increment mode
///
/// - **CIRC** `[5]` (RW)
///
///   Circular mode
///
/// - **DIR** `[4]` (RW)
///
///   Data transfer direction
///
/// - **TEIE** `[3]` (RW)
///
///   Transfer error interrupt
///   enable
///
/// - **HTIE** `[2]` (RW)
///
///   Half Transfer interrupt
///   enable
///
/// - **TCIE** `[1]` (RW)
///
///   Transfer complete interrupt
///   enable
///
/// - **EN** `[0]` (RW)
///
///   Channel enable
#[repr(transparent)]
pub struct Cfgr8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgr8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgr8 {
    // ======== Field Constants ========

    /// **MEM2MEM**
    ///
    /// Memory to memory mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MEM2MEM_OFFSET: u32 = 14;
    pub const MEM2MEM_MASK: u32 = 0x00004000;
    pub const MEM2MEM: u32 = 1 << 14;

    /// **PL**
    ///
    /// Channel Priority level
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const PL_OFFSET: u32 = 12;
    pub const PL_MASK: u32 = 0x00003000;

    /// **MSIZE**
    ///
    /// Memory size
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const MSIZE_OFFSET: u32 = 10;
    pub const MSIZE_MASK: u32 = 0x00000c00;

    /// **PSIZE**
    ///
    /// Peripheral size
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const PSIZE_OFFSET: u32 = 8;
    pub const PSIZE_MASK: u32 = 0x00000300;

    /// **MINC**
    ///
    /// Memory increment mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MINC_OFFSET: u32 = 7;
    pub const MINC_MASK: u32 = 0x00000080;
    pub const MINC: u32 = 1 << 7;

    /// **PINC**
    ///
    /// Peripheral increment mode
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PINC_OFFSET: u32 = 6;
    pub const PINC_MASK: u32 = 0x00000040;
    pub const PINC: u32 = 1 << 6;

    /// **CIRC**
    ///
    /// Circular mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CIRC_OFFSET: u32 = 5;
    pub const CIRC_MASK: u32 = 0x00000020;
    pub const CIRC: u32 = 1 << 5;

    /// **DIR**
    ///
    /// Data transfer direction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIR_OFFSET: u32 = 4;
    pub const DIR_MASK: u32 = 0x00000010;
    pub const DIR: u32 = 1 << 4;

    /// **TEIE**
    ///
    /// Transfer error interrupt
    /// enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEIE_OFFSET: u32 = 3;
    pub const TEIE_MASK: u32 = 0x00000008;
    pub const TEIE: u32 = 1 << 3;

    /// **HTIE**
    ///
    /// Half Transfer interrupt
    /// enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HTIE_OFFSET: u32 = 2;
    pub const HTIE_MASK: u32 = 0x00000004;
    pub const HTIE: u32 = 1 << 2;

    /// **TCIE**
    ///
    /// Transfer complete interrupt
    /// enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TCIE_OFFSET: u32 = 1;
    pub const TCIE_MASK: u32 = 0x00000002;
    pub const TCIE: u32 = 1 << 1;

    /// **EN**
    ///
    /// Channel enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM2MEM** `[14]` (RW)
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> bool {
        (self.value.read_bits() & Self::MEM2MEM_MASK) != 0
    }

    /// Set MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem2mem(&self) {
        self.value.modify_bits(|r| r | Self::MEM2MEM_MASK);
    }

    /// Clear MEM2MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem2mem(&self) {
        self.value.modify_bits(|r| r & !Self::MEM2MEM_MASK);
    }

    /// **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn pl(&self) -> u32 {
        (self.value.read_bits() & Self::PL_MASK) >> Self::PL_OFFSET
    }

    /// Write **PL** `[13:12]` (RW)
    ///
    /// Channel Priority level
    #[inline(always)]
    pub fn write_pl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PL_MASK) | ((value << Self::PL_OFFSET) & Self::PL_MASK));
    }

    /// **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn msize(&self) -> u32 {
        (self.value.read_bits() & Self::MSIZE_MASK) >> Self::MSIZE_OFFSET
    }

    /// Write **MSIZE** `[11:10]` (RW)
    ///
    /// Memory size
    #[inline(always)]
    pub fn write_msize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MSIZE_MASK) | ((value << Self::MSIZE_OFFSET) & Self::MSIZE_MASK)
        });
    }

    /// **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn psize(&self) -> u32 {
        (self.value.read_bits() & Self::PSIZE_MASK) >> Self::PSIZE_OFFSET
    }

    /// Write **PSIZE** `[9:8]` (RW)
    ///
    /// Peripheral size
    #[inline(always)]
    pub fn write_psize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSIZE_MASK) | ((value << Self::PSIZE_OFFSET) & Self::PSIZE_MASK)
        });
    }

    /// **MINC** `[7]` (RW)
    ///
    /// Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> bool {
        (self.value.read_bits() & Self::MINC_MASK) != 0
    }

    /// Set MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_minc(&self) {
        self.value.modify_bits(|r| r | Self::MINC_MASK);
    }

    /// Clear MINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_minc(&self) {
        self.value.modify_bits(|r| r & !Self::MINC_MASK);
    }

    /// **PINC** `[6]` (RW)
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> bool {
        (self.value.read_bits() & Self::PINC_MASK) != 0
    }

    /// Set PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pinc(&self) {
        self.value.modify_bits(|r| r | Self::PINC_MASK);
    }

    /// Clear PINC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pinc(&self) {
        self.value.modify_bits(|r| r & !Self::PINC_MASK);
    }

    /// **CIRC** `[5]` (RW)
    ///
    /// Circular mode
    #[inline(always)]
    pub fn circ(&self) -> bool {
        (self.value.read_bits() & Self::CIRC_MASK) != 0
    }

    /// Set CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_circ(&self) {
        self.value.modify_bits(|r| r | Self::CIRC_MASK);
    }

    /// Clear CIRC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_circ(&self) {
        self.value.modify_bits(|r| r & !Self::CIRC_MASK);
    }

    /// **DIR** `[4]` (RW)
    ///
    /// Data transfer direction
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

    /// **TEIE** `[3]` (RW)
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub fn teie(&self) -> bool {
        (self.value.read_bits() & Self::TEIE_MASK) != 0
    }

    /// Set TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_teie(&self) {
        self.value.modify_bits(|r| r | Self::TEIE_MASK);
    }

    /// Clear TEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_teie(&self) {
        self.value.modify_bits(|r| r & !Self::TEIE_MASK);
    }

    /// **HTIE** `[2]` (RW)
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub fn htie(&self) -> bool {
        (self.value.read_bits() & Self::HTIE_MASK) != 0
    }

    /// Set HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_htie(&self) {
        self.value.modify_bits(|r| r | Self::HTIE_MASK);
    }

    /// Clear HTIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_htie(&self) {
        self.value.modify_bits(|r| r & !Self::HTIE_MASK);
    }

    /// **TCIE** `[1]` (RW)
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub fn tcie(&self) -> bool {
        (self.value.read_bits() & Self::TCIE_MASK) != 0
    }

    /// Set TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcie(&self) {
        self.value.modify_bits(|r| r | Self::TCIE_MASK);
    }

    /// Clear TCIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcie(&self) {
        self.value.modify_bits(|r| r & !Self::TCIE_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Channel enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CFGR8 value type for batch read/modify/write operations
///
/// Use `cfgr8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgr8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cfgr8Val {
    bits: u32,
}

impl Cfgr8Val {
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

    /// Read **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        (self.bits & Cfgr8::MEM2MEM_MASK) != 0
    }

    /// Read **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn pl(&self) -> u32 {
        (self.bits & Cfgr8::PL_MASK) >> Cfgr8::PL_OFFSET
    }

    /// Read **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn msize(&self) -> u32 {
        (self.bits & Cfgr8::MSIZE_MASK) >> Cfgr8::MSIZE_OFFSET
    }

    /// Read **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn psize(&self) -> u32 {
        (self.bits & Cfgr8::PSIZE_MASK) >> Cfgr8::PSIZE_OFFSET
    }

    /// Read **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        (self.bits & Cfgr8::MINC_MASK) != 0
    }

    /// Read **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        (self.bits & Cfgr8::PINC_MASK) != 0
    }

    /// Read **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        (self.bits & Cfgr8::CIRC_MASK) != 0
    }

    /// Read **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        (self.bits & Cfgr8::DIR_MASK) != 0
    }

    /// Read **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        (self.bits & Cfgr8::TEIE_MASK) != 0
    }

    /// Read **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        (self.bits & Cfgr8::HTIE_MASK) != 0
    }

    /// Read **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Cfgr8::TCIE_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cfgr8::EN_MASK) != 0
    }

    /// Set **MEM2MEM** `[14:14]`
    ///
    /// Memory to memory mode
    #[inline(always)]
    pub const fn with_mem2mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::MEM2MEM_MASK
            } else {
                self.bits & !Cfgr8::MEM2MEM_MASK
            },
        }
    }

    /// Set **PL** `[13:12]`
    ///
    /// Channel Priority level
    #[inline(always)]
    pub const fn with_pl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr8::PL_MASK) | ((value << Cfgr8::PL_OFFSET) & Cfgr8::PL_MASK),
        }
    }

    /// Set **MSIZE** `[11:10]`
    ///
    /// Memory size
    #[inline(always)]
    pub const fn with_msize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr8::MSIZE_MASK)
                | ((value << Cfgr8::MSIZE_OFFSET) & Cfgr8::MSIZE_MASK),
        }
    }

    /// Set **PSIZE** `[9:8]`
    ///
    /// Peripheral size
    #[inline(always)]
    pub const fn with_psize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgr8::PSIZE_MASK)
                | ((value << Cfgr8::PSIZE_OFFSET) & Cfgr8::PSIZE_MASK),
        }
    }

    /// Set **MINC** `[7:7]`
    ///
    /// Memory increment mode
    #[inline(always)]
    pub const fn with_minc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::MINC_MASK
            } else {
                self.bits & !Cfgr8::MINC_MASK
            },
        }
    }

    /// Set **PINC** `[6:6]`
    ///
    /// Peripheral increment mode
    #[inline(always)]
    pub const fn with_pinc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::PINC_MASK
            } else {
                self.bits & !Cfgr8::PINC_MASK
            },
        }
    }

    /// Set **CIRC** `[5:5]`
    ///
    /// Circular mode
    #[inline(always)]
    pub const fn with_circ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::CIRC_MASK
            } else {
                self.bits & !Cfgr8::CIRC_MASK
            },
        }
    }

    /// Set **DIR** `[4:4]`
    ///
    /// Data transfer direction
    #[inline(always)]
    pub const fn with_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::DIR_MASK
            } else {
                self.bits & !Cfgr8::DIR_MASK
            },
        }
    }

    /// Set **TEIE** `[3:3]`
    ///
    /// Transfer error interrupt
    /// enable
    #[inline(always)]
    pub const fn with_teie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::TEIE_MASK
            } else {
                self.bits & !Cfgr8::TEIE_MASK
            },
        }
    }

    /// Set **HTIE** `[2:2]`
    ///
    /// Half Transfer interrupt
    /// enable
    #[inline(always)]
    pub const fn with_htie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::HTIE_MASK
            } else {
                self.bits & !Cfgr8::HTIE_MASK
            },
        }
    }

    /// Set **TCIE** `[1:1]`
    ///
    /// Transfer complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::TCIE_MASK
            } else {
                self.bits & !Cfgr8::TCIE_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Channel enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgr8::EN_MASK
            } else {
                self.bits & !Cfgr8::EN_MASK
            },
        }
    }
}

impl Default for Cfgr8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgr8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cfgr8Val {
        Cfgr8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cfgr8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cfgr8Val) -> Cfgr8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CNTR8 Register
///
/// DMA channel 8 number of data
/// register
///
/// Address offset: 0x0098
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NDT** `[15:0]` (RW)
///
///   Number of data to transfer
#[repr(transparent)]
pub struct Cntr8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cntr8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cntr8 {
    // ======== Field Constants ========

    /// **NDT**
    ///
    /// Number of data to transfer
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const NDT_OFFSET: u32 = 0;
    pub const NDT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn ndt(&self) -> u32 {
        (self.value.read_bits() & Self::NDT_MASK) >> Self::NDT_OFFSET
    }

    /// Write **NDT** `[15:0]` (RW)
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub fn write_ndt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NDT_MASK) | ((value << Self::NDT_OFFSET) & Self::NDT_MASK)
        });
    }
}

/// CNTR8 value type for batch read/modify/write operations
///
/// Use `cntr8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cntr8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Cntr8Val {
    bits: u32,
}

impl Cntr8Val {
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

    /// Read **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn ndt(&self) -> u32 {
        (self.bits & Cntr8::NDT_MASK) >> Cntr8::NDT_OFFSET
    }

    /// Set **NDT** `[15:0]`
    ///
    /// Number of data to transfer
    #[inline(always)]
    pub const fn with_ndt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cntr8::NDT_MASK) | ((value << Cntr8::NDT_OFFSET) & Cntr8::NDT_MASK),
        }
    }
}

impl Default for Cntr8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cntr8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Cntr8Val {
        Cntr8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Cntr8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Cntr8Val) -> Cntr8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADDR8 Register
///
/// DMA channel 8 peripheral address
/// register
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PA** `[31:0]` (RW)
///
///   Peripheral address
#[repr(transparent)]
pub struct Paddr8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Paddr8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Paddr8 {
    // ======== Field Constants ========

    /// **PA**
    ///
    /// Peripheral address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PA_OFFSET: u32 = 0;
    pub const PA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn pa(&self) -> u32 {
        (self.value.read_bits() & Self::PA_MASK) >> Self::PA_OFFSET
    }

    /// Write **PA** `[31:0]` (RW)
    ///
    /// Peripheral address
    #[inline(always)]
    pub fn write_pa(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::PA_MASK) | ((value << Self::PA_OFFSET) & Self::PA_MASK));
    }
}

/// PADDR8 value type for batch read/modify/write operations
///
/// Use `paddr8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `paddr8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Paddr8Val {
    bits: u32,
}

impl Paddr8Val {
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

    /// Read **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        (self.bits & Paddr8::PA_MASK) >> Paddr8::PA_OFFSET
    }

    /// Set **PA** `[31:0]`
    ///
    /// Peripheral address
    #[inline(always)]
    pub const fn with_pa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Paddr8::PA_MASK) | ((value << Paddr8::PA_OFFSET) & Paddr8::PA_MASK),
        }
    }
}

impl Default for Paddr8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Paddr8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Paddr8Val {
        Paddr8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Paddr8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Paddr8Val) -> Paddr8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MADDR8 Register
///
/// DMA channel 8 memory address
/// register
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MA** `[31:0]` (RW)
///
///   Memory address
#[repr(transparent)]
pub struct Maddr8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Maddr8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Maddr8 {
    // ======== Field Constants ========

    /// **MA**
    ///
    /// Memory address
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MA_OFFSET: u32 = 0;
    pub const MA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn ma(&self) -> u32 {
        (self.value.read_bits() & Self::MA_MASK) >> Self::MA_OFFSET
    }

    /// Write **MA** `[31:0]` (RW)
    ///
    /// Memory address
    #[inline(always)]
    pub fn write_ma(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::MA_MASK) | ((value << Self::MA_OFFSET) & Self::MA_MASK));
    }
}

/// MADDR8 value type for batch read/modify/write operations
///
/// Use `maddr8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `maddr8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Maddr8Val {
    bits: u32,
}

impl Maddr8Val {
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

    /// Read **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        (self.bits & Maddr8::MA_MASK) >> Maddr8::MA_OFFSET
    }

    /// Set **MA** `[31:0]`
    ///
    /// Memory address
    #[inline(always)]
    pub const fn with_ma(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Maddr8::MA_MASK) | ((value << Maddr8::MA_OFFSET) & Maddr8::MA_MASK),
        }
    }
}

impl Default for Maddr8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Maddr8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Maddr8Val {
        Maddr8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Maddr8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Maddr8Val) -> Maddr8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// DMA interrupt status register
    /// (DMA_INTFR)
    pub intfr: Intfr,
    /// 0x0004
    /// DMA interrupt flag clear register
    /// (DMA_INTFCR)
    pub intfcr: Intfcr,
    /// 0x0008
    /// DMA channel 1 configuration register
    /// (DMA_CFGR1)
    pub cfgr1: Cfgr1,
    /// 0x000c
    /// DMA channel 1 number of data
    /// register
    pub cntr1: Cntr1,
    /// 0x0010
    /// DMA channel 1 peripheral address
    /// register
    pub paddr1: Paddr1,
    /// 0x0014
    /// DMA channel 1 memory address
    /// register
    pub maddr1: Maddr1,
    _reserved0: [u8; 4],
    /// 0x001c
    /// DMA channel 2 configuration register
    /// (DMA_CFGR2)
    pub cfgr2: Cfgr2,
    /// 0x0020
    /// DMA channel 2 number of data
    /// register
    pub cntr2: Cntr2,
    /// 0x0024
    /// DMA channel 2 peripheral address
    /// register
    pub paddr2: Paddr2,
    /// 0x0028
    /// DMA channel 2 memory address
    /// register
    pub maddr2: Maddr2,
    _reserved1: [u8; 4],
    /// 0x0030
    /// DMA channel 3 configuration register
    /// (DMA_CFGR3)
    pub cfgr3: Cfgr3,
    /// 0x0034
    /// DMA channel 3 number of data
    /// register
    pub cntr3: Cntr3,
    /// 0x0038
    /// DMA channel 3 peripheral address
    /// register
    pub paddr3: Paddr3,
    /// 0x003c
    /// DMA channel 3 memory address
    /// register
    pub maddr3: Maddr3,
    _reserved2: [u8; 4],
    /// 0x0044
    /// DMA channel 4 configuration register
    /// (DMA_CFGR4)
    pub cfgr4: Cfgr4,
    /// 0x0048
    /// DMA channel 4 number of data
    /// register
    pub cntr4: Cntr4,
    /// 0x004c
    /// DMA channel 4 peripheral address
    /// register
    pub paddr4: Paddr4,
    /// 0x0050
    /// DMA channel 4 memory address
    /// register
    pub maddr4: Maddr4,
    _reserved3: [u8; 4],
    /// 0x0058
    /// DMA channel 5 configuration register
    /// (DMA_CFGR1)
    pub cfgr5: Cfgr5,
    /// 0x005c
    /// DMA channel 5 number of data
    /// register
    pub cntr5: Cntr5,
    /// 0x0060
    /// DMA channel 5 peripheral address
    /// register
    pub paddr5: Paddr5,
    /// 0x0064
    /// DMA channel 5 memory address
    /// register
    pub maddr5: Maddr5,
    _reserved4: [u8; 4],
    /// 0x006c
    /// DMA channel 6 configuration register
    /// (DMA_CFGR1)
    pub cfgr6: Cfgr6,
    /// 0x0070
    /// DMA channel 6 number of data
    /// register
    pub cntr6: Cntr6,
    /// 0x0074
    /// DMA channel 6 peripheral address
    /// register
    pub paddr6: Paddr6,
    /// 0x0078
    /// DMA channel 6 memory address
    /// register
    pub maddr6: Maddr6,
    _reserved5: [u8; 4],
    /// 0x0080
    /// DMA channel 7 configuration register
    /// (DMA_CFGR1)
    pub cfgr7: Cfgr7,
    /// 0x0084
    /// DMA channel 7 number of data
    /// register
    pub cntr7: Cntr7,
    /// 0x0088
    /// DMA channel 7 peripheral address
    /// register
    pub paddr7: Paddr7,
    /// 0x008c
    /// DMA channel 7 memory address
    /// register
    pub maddr7: Maddr7,
    _reserved6: [u8; 4],
    /// 0x0094
    /// DMA channel 8 configuration register
    /// (DMA_CFGR1)
    pub cfgr8: Cfgr8,
    /// 0x0098
    /// DMA channel 8 number of data
    /// register
    pub cntr8: Cntr8,
    /// 0x009c
    /// DMA channel 8 peripheral address
    /// register
    pub paddr8: Paddr8,
    /// 0x00a0
    /// DMA channel 8 memory address
    /// register
    pub maddr8: Maddr8,
}
