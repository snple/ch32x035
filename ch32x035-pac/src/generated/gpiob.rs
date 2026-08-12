//! GPIOB

#![allow(dead_code)]

pub const BASE: u32 = 0x40010c00;

/// CFGLR Register
///
/// Port configuration register low
/// (GPIOn_CFGLR)
///
/// Address offset: 0x0000
/// Reset value: 0x44444444
///
/// ## Fields
///
/// - **CNF7** `[31:30]` (RW)
///
///   Port n.7 configuration
///   bits
///
/// - **MODE7** `[29:28]` (RW)
///
///   Port n.7 mode bits
///
/// - **CNF6** `[27:26]` (RW)
///
///   Port n.6 configuration
///   bits
///
/// - **MODE6** `[25:24]` (RW)
///
///   Port n.6 mode bits
///
/// - **CNF5** `[23:22]` (RW)
///
///   Port n.5 configuration
///   bits
///
/// - **MODE5** `[21:20]` (RW)
///
///   Port n.5 mode bits
///
/// - **CNF4** `[19:18]` (RW)
///
///   Port n.4 configuration
///   bits
///
/// - **MODE4** `[17:16]` (RW)
///
///   Port n.4 mode bits
///
/// - **CNF3** `[15:14]` (RW)
///
///   Port n.3 configuration
///   bits
///
/// - **MODE3** `[13:12]` (RW)
///
///   Port n.3 mode bits
///
/// - **CNF2** `[11:10]` (RW)
///
///   Port n.2 configuration
///   bits
///
/// - **MODE2** `[9:8]` (RW)
///
///   Port n.2 mode bits
///
/// - **CNF1** `[7:6]` (RW)
///
///   Port n.1 configuration
///   bits
///
/// - **MODE1** `[5:4]` (RW)
///
///   Port n.1 mode bits
///
/// - **CNF0** `[3:2]` (RW)
///
///   Port n.0 configuration
///   bits
///
/// - **MODE0** `[1:0]` (RW)
///
///   Port n.0 mode bits
#[repr(transparent)]
pub struct Cfglr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfglr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfglr {
    // ======== Field Constants ========

    /// **CNF7**
    ///
    /// Port n.7 configuration
    /// bits
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CNF7_OFFSET: u32 = 30;
    pub const CNF7_MASK: u32 = 0xc0000000;

    /// **MODE7**
    ///
    /// Port n.7 mode bits
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const MODE7_OFFSET: u32 = 28;
    pub const MODE7_MASK: u32 = 0x30000000;

    /// **CNF6**
    ///
    /// Port n.6 configuration
    /// bits
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CNF6_OFFSET: u32 = 26;
    pub const CNF6_MASK: u32 = 0x0c000000;

    /// **MODE6**
    ///
    /// Port n.6 mode bits
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const MODE6_OFFSET: u32 = 24;
    pub const MODE6_MASK: u32 = 0x03000000;

    /// **CNF5**
    ///
    /// Port n.5 configuration
    /// bits
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CNF5_OFFSET: u32 = 22;
    pub const CNF5_MASK: u32 = 0x00c00000;

    /// **MODE5**
    ///
    /// Port n.5 mode bits
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const MODE5_OFFSET: u32 = 20;
    pub const MODE5_MASK: u32 = 0x00300000;

    /// **CNF4**
    ///
    /// Port n.4 configuration
    /// bits
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CNF4_OFFSET: u32 = 18;
    pub const CNF4_MASK: u32 = 0x000c0000;

    /// **MODE4**
    ///
    /// Port n.4 mode bits
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const MODE4_OFFSET: u32 = 16;
    pub const MODE4_MASK: u32 = 0x00030000;

    /// **CNF3**
    ///
    /// Port n.3 configuration
    /// bits
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const CNF3_OFFSET: u32 = 14;
    pub const CNF3_MASK: u32 = 0x0000c000;

    /// **MODE3**
    ///
    /// Port n.3 mode bits
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const MODE3_OFFSET: u32 = 12;
    pub const MODE3_MASK: u32 = 0x00003000;

    /// **CNF2**
    ///
    /// Port n.2 configuration
    /// bits
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const CNF2_OFFSET: u32 = 10;
    pub const CNF2_MASK: u32 = 0x00000c00;

    /// **MODE2**
    ///
    /// Port n.2 mode bits
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const MODE2_OFFSET: u32 = 8;
    pub const MODE2_MASK: u32 = 0x00000300;

    /// **CNF1**
    ///
    /// Port n.1 configuration
    /// bits
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const CNF1_OFFSET: u32 = 6;
    pub const CNF1_MASK: u32 = 0x000000c0;

    /// **MODE1**
    ///
    /// Port n.1 mode bits
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const MODE1_OFFSET: u32 = 4;
    pub const MODE1_MASK: u32 = 0x00000030;

    /// **CNF0**
    ///
    /// Port n.0 configuration
    /// bits
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const CNF0_OFFSET: u32 = 2;
    pub const CNF0_MASK: u32 = 0x0000000c;

    /// **MODE0**
    ///
    /// Port n.0 mode bits
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MODE0_OFFSET: u32 = 0;
    pub const MODE0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNF7** `[31:30]` (RW)
    ///
    /// Port n.7 configuration
    /// bits
    #[inline(always)]
    pub fn cnf7(&self) -> u32 {
        (self.value.read_bits() & Self::CNF7_MASK) >> Self::CNF7_OFFSET
    }

    /// Write **CNF7** `[31:30]` (RW)
    ///
    /// Port n.7 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF7_MASK) | ((value << Self::CNF7_OFFSET) & Self::CNF7_MASK)
        });
    }

    /// **MODE7** `[29:28]` (RW)
    ///
    /// Port n.7 mode bits
    #[inline(always)]
    pub fn mode7(&self) -> u32 {
        (self.value.read_bits() & Self::MODE7_MASK) >> Self::MODE7_OFFSET
    }

    /// Write **MODE7** `[29:28]` (RW)
    ///
    /// Port n.7 mode bits
    #[inline(always)]
    pub fn write_mode7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE7_MASK) | ((value << Self::MODE7_OFFSET) & Self::MODE7_MASK)
        });
    }

    /// **CNF6** `[27:26]` (RW)
    ///
    /// Port n.6 configuration
    /// bits
    #[inline(always)]
    pub fn cnf6(&self) -> u32 {
        (self.value.read_bits() & Self::CNF6_MASK) >> Self::CNF6_OFFSET
    }

    /// Write **CNF6** `[27:26]` (RW)
    ///
    /// Port n.6 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF6_MASK) | ((value << Self::CNF6_OFFSET) & Self::CNF6_MASK)
        });
    }

    /// **MODE6** `[25:24]` (RW)
    ///
    /// Port n.6 mode bits
    #[inline(always)]
    pub fn mode6(&self) -> u32 {
        (self.value.read_bits() & Self::MODE6_MASK) >> Self::MODE6_OFFSET
    }

    /// Write **MODE6** `[25:24]` (RW)
    ///
    /// Port n.6 mode bits
    #[inline(always)]
    pub fn write_mode6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE6_MASK) | ((value << Self::MODE6_OFFSET) & Self::MODE6_MASK)
        });
    }

    /// **CNF5** `[23:22]` (RW)
    ///
    /// Port n.5 configuration
    /// bits
    #[inline(always)]
    pub fn cnf5(&self) -> u32 {
        (self.value.read_bits() & Self::CNF5_MASK) >> Self::CNF5_OFFSET
    }

    /// Write **CNF5** `[23:22]` (RW)
    ///
    /// Port n.5 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF5_MASK) | ((value << Self::CNF5_OFFSET) & Self::CNF5_MASK)
        });
    }

    /// **MODE5** `[21:20]` (RW)
    ///
    /// Port n.5 mode bits
    #[inline(always)]
    pub fn mode5(&self) -> u32 {
        (self.value.read_bits() & Self::MODE5_MASK) >> Self::MODE5_OFFSET
    }

    /// Write **MODE5** `[21:20]` (RW)
    ///
    /// Port n.5 mode bits
    #[inline(always)]
    pub fn write_mode5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE5_MASK) | ((value << Self::MODE5_OFFSET) & Self::MODE5_MASK)
        });
    }

    /// **CNF4** `[19:18]` (RW)
    ///
    /// Port n.4 configuration
    /// bits
    #[inline(always)]
    pub fn cnf4(&self) -> u32 {
        (self.value.read_bits() & Self::CNF4_MASK) >> Self::CNF4_OFFSET
    }

    /// Write **CNF4** `[19:18]` (RW)
    ///
    /// Port n.4 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF4_MASK) | ((value << Self::CNF4_OFFSET) & Self::CNF4_MASK)
        });
    }

    /// **MODE4** `[17:16]` (RW)
    ///
    /// Port n.4 mode bits
    #[inline(always)]
    pub fn mode4(&self) -> u32 {
        (self.value.read_bits() & Self::MODE4_MASK) >> Self::MODE4_OFFSET
    }

    /// Write **MODE4** `[17:16]` (RW)
    ///
    /// Port n.4 mode bits
    #[inline(always)]
    pub fn write_mode4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE4_MASK) | ((value << Self::MODE4_OFFSET) & Self::MODE4_MASK)
        });
    }

    /// **CNF3** `[15:14]` (RW)
    ///
    /// Port n.3 configuration
    /// bits
    #[inline(always)]
    pub fn cnf3(&self) -> u32 {
        (self.value.read_bits() & Self::CNF3_MASK) >> Self::CNF3_OFFSET
    }

    /// Write **CNF3** `[15:14]` (RW)
    ///
    /// Port n.3 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF3_MASK) | ((value << Self::CNF3_OFFSET) & Self::CNF3_MASK)
        });
    }

    /// **MODE3** `[13:12]` (RW)
    ///
    /// Port n.3 mode bits
    #[inline(always)]
    pub fn mode3(&self) -> u32 {
        (self.value.read_bits() & Self::MODE3_MASK) >> Self::MODE3_OFFSET
    }

    /// Write **MODE3** `[13:12]` (RW)
    ///
    /// Port n.3 mode bits
    #[inline(always)]
    pub fn write_mode3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE3_MASK) | ((value << Self::MODE3_OFFSET) & Self::MODE3_MASK)
        });
    }

    /// **CNF2** `[11:10]` (RW)
    ///
    /// Port n.2 configuration
    /// bits
    #[inline(always)]
    pub fn cnf2(&self) -> u32 {
        (self.value.read_bits() & Self::CNF2_MASK) >> Self::CNF2_OFFSET
    }

    /// Write **CNF2** `[11:10]` (RW)
    ///
    /// Port n.2 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF2_MASK) | ((value << Self::CNF2_OFFSET) & Self::CNF2_MASK)
        });
    }

    /// **MODE2** `[9:8]` (RW)
    ///
    /// Port n.2 mode bits
    #[inline(always)]
    pub fn mode2(&self) -> u32 {
        (self.value.read_bits() & Self::MODE2_MASK) >> Self::MODE2_OFFSET
    }

    /// Write **MODE2** `[9:8]` (RW)
    ///
    /// Port n.2 mode bits
    #[inline(always)]
    pub fn write_mode2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE2_MASK) | ((value << Self::MODE2_OFFSET) & Self::MODE2_MASK)
        });
    }

    /// **CNF1** `[7:6]` (RW)
    ///
    /// Port n.1 configuration
    /// bits
    #[inline(always)]
    pub fn cnf1(&self) -> u32 {
        (self.value.read_bits() & Self::CNF1_MASK) >> Self::CNF1_OFFSET
    }

    /// Write **CNF1** `[7:6]` (RW)
    ///
    /// Port n.1 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF1_MASK) | ((value << Self::CNF1_OFFSET) & Self::CNF1_MASK)
        });
    }

    /// **MODE1** `[5:4]` (RW)
    ///
    /// Port n.1 mode bits
    #[inline(always)]
    pub fn mode1(&self) -> u32 {
        (self.value.read_bits() & Self::MODE1_MASK) >> Self::MODE1_OFFSET
    }

    /// Write **MODE1** `[5:4]` (RW)
    ///
    /// Port n.1 mode bits
    #[inline(always)]
    pub fn write_mode1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE1_MASK) | ((value << Self::MODE1_OFFSET) & Self::MODE1_MASK)
        });
    }

    /// **CNF0** `[3:2]` (RW)
    ///
    /// Port n.0 configuration
    /// bits
    #[inline(always)]
    pub fn cnf0(&self) -> u32 {
        (self.value.read_bits() & Self::CNF0_MASK) >> Self::CNF0_OFFSET
    }

    /// Write **CNF0** `[3:2]` (RW)
    ///
    /// Port n.0 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF0_MASK) | ((value << Self::CNF0_OFFSET) & Self::CNF0_MASK)
        });
    }

    /// **MODE0** `[1:0]` (RW)
    ///
    /// Port n.0 mode bits
    #[inline(always)]
    pub fn mode0(&self) -> u32 {
        (self.value.read_bits() & Self::MODE0_MASK) >> Self::MODE0_OFFSET
    }

    /// Write **MODE0** `[1:0]` (RW)
    ///
    /// Port n.0 mode bits
    #[inline(always)]
    pub fn write_mode0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE0_MASK) | ((value << Self::MODE0_OFFSET) & Self::MODE0_MASK)
        });
    }
}

/// CFGLR value type for batch read/modify/write operations
///
/// Use `cfglr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfglr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CfglrVal {
    bits: u32,
}

impl CfglrVal {
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
        Self { bits: 0x44444444 }
    }

    /// Read **CNF7** `[31:30]`
    ///
    /// Port n.7 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf7(&self) -> u32 {
        (self.bits & Cfglr::CNF7_MASK) >> Cfglr::CNF7_OFFSET
    }

    /// Read **MODE7** `[29:28]`
    ///
    /// Port n.7 mode bits
    #[inline(always)]
    pub const fn mode7(&self) -> u32 {
        (self.bits & Cfglr::MODE7_MASK) >> Cfglr::MODE7_OFFSET
    }

    /// Read **CNF6** `[27:26]`
    ///
    /// Port n.6 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf6(&self) -> u32 {
        (self.bits & Cfglr::CNF6_MASK) >> Cfglr::CNF6_OFFSET
    }

    /// Read **MODE6** `[25:24]`
    ///
    /// Port n.6 mode bits
    #[inline(always)]
    pub const fn mode6(&self) -> u32 {
        (self.bits & Cfglr::MODE6_MASK) >> Cfglr::MODE6_OFFSET
    }

    /// Read **CNF5** `[23:22]`
    ///
    /// Port n.5 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf5(&self) -> u32 {
        (self.bits & Cfglr::CNF5_MASK) >> Cfglr::CNF5_OFFSET
    }

    /// Read **MODE5** `[21:20]`
    ///
    /// Port n.5 mode bits
    #[inline(always)]
    pub const fn mode5(&self) -> u32 {
        (self.bits & Cfglr::MODE5_MASK) >> Cfglr::MODE5_OFFSET
    }

    /// Read **CNF4** `[19:18]`
    ///
    /// Port n.4 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf4(&self) -> u32 {
        (self.bits & Cfglr::CNF4_MASK) >> Cfglr::CNF4_OFFSET
    }

    /// Read **MODE4** `[17:16]`
    ///
    /// Port n.4 mode bits
    #[inline(always)]
    pub const fn mode4(&self) -> u32 {
        (self.bits & Cfglr::MODE4_MASK) >> Cfglr::MODE4_OFFSET
    }

    /// Read **CNF3** `[15:14]`
    ///
    /// Port n.3 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf3(&self) -> u32 {
        (self.bits & Cfglr::CNF3_MASK) >> Cfglr::CNF3_OFFSET
    }

    /// Read **MODE3** `[13:12]`
    ///
    /// Port n.3 mode bits
    #[inline(always)]
    pub const fn mode3(&self) -> u32 {
        (self.bits & Cfglr::MODE3_MASK) >> Cfglr::MODE3_OFFSET
    }

    /// Read **CNF2** `[11:10]`
    ///
    /// Port n.2 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf2(&self) -> u32 {
        (self.bits & Cfglr::CNF2_MASK) >> Cfglr::CNF2_OFFSET
    }

    /// Read **MODE2** `[9:8]`
    ///
    /// Port n.2 mode bits
    #[inline(always)]
    pub const fn mode2(&self) -> u32 {
        (self.bits & Cfglr::MODE2_MASK) >> Cfglr::MODE2_OFFSET
    }

    /// Read **CNF1** `[7:6]`
    ///
    /// Port n.1 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf1(&self) -> u32 {
        (self.bits & Cfglr::CNF1_MASK) >> Cfglr::CNF1_OFFSET
    }

    /// Read **MODE1** `[5:4]`
    ///
    /// Port n.1 mode bits
    #[inline(always)]
    pub const fn mode1(&self) -> u32 {
        (self.bits & Cfglr::MODE1_MASK) >> Cfglr::MODE1_OFFSET
    }

    /// Read **CNF0** `[3:2]`
    ///
    /// Port n.0 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf0(&self) -> u32 {
        (self.bits & Cfglr::CNF0_MASK) >> Cfglr::CNF0_OFFSET
    }

    /// Read **MODE0** `[1:0]`
    ///
    /// Port n.0 mode bits
    #[inline(always)]
    pub const fn mode0(&self) -> u32 {
        (self.bits & Cfglr::MODE0_MASK) >> Cfglr::MODE0_OFFSET
    }

    /// Set **CNF7** `[31:30]`
    ///
    /// Port n.7 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF7_MASK)
                | ((value << Cfglr::CNF7_OFFSET) & Cfglr::CNF7_MASK),
        }
    }

    /// Set **MODE7** `[29:28]`
    ///
    /// Port n.7 mode bits
    #[inline(always)]
    pub const fn with_mode7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE7_MASK)
                | ((value << Cfglr::MODE7_OFFSET) & Cfglr::MODE7_MASK),
        }
    }

    /// Set **CNF6** `[27:26]`
    ///
    /// Port n.6 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF6_MASK)
                | ((value << Cfglr::CNF6_OFFSET) & Cfglr::CNF6_MASK),
        }
    }

    /// Set **MODE6** `[25:24]`
    ///
    /// Port n.6 mode bits
    #[inline(always)]
    pub const fn with_mode6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE6_MASK)
                | ((value << Cfglr::MODE6_OFFSET) & Cfglr::MODE6_MASK),
        }
    }

    /// Set **CNF5** `[23:22]`
    ///
    /// Port n.5 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF5_MASK)
                | ((value << Cfglr::CNF5_OFFSET) & Cfglr::CNF5_MASK),
        }
    }

    /// Set **MODE5** `[21:20]`
    ///
    /// Port n.5 mode bits
    #[inline(always)]
    pub const fn with_mode5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE5_MASK)
                | ((value << Cfglr::MODE5_OFFSET) & Cfglr::MODE5_MASK),
        }
    }

    /// Set **CNF4** `[19:18]`
    ///
    /// Port n.4 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF4_MASK)
                | ((value << Cfglr::CNF4_OFFSET) & Cfglr::CNF4_MASK),
        }
    }

    /// Set **MODE4** `[17:16]`
    ///
    /// Port n.4 mode bits
    #[inline(always)]
    pub const fn with_mode4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE4_MASK)
                | ((value << Cfglr::MODE4_OFFSET) & Cfglr::MODE4_MASK),
        }
    }

    /// Set **CNF3** `[15:14]`
    ///
    /// Port n.3 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF3_MASK)
                | ((value << Cfglr::CNF3_OFFSET) & Cfglr::CNF3_MASK),
        }
    }

    /// Set **MODE3** `[13:12]`
    ///
    /// Port n.3 mode bits
    #[inline(always)]
    pub const fn with_mode3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE3_MASK)
                | ((value << Cfglr::MODE3_OFFSET) & Cfglr::MODE3_MASK),
        }
    }

    /// Set **CNF2** `[11:10]`
    ///
    /// Port n.2 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF2_MASK)
                | ((value << Cfglr::CNF2_OFFSET) & Cfglr::CNF2_MASK),
        }
    }

    /// Set **MODE2** `[9:8]`
    ///
    /// Port n.2 mode bits
    #[inline(always)]
    pub const fn with_mode2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE2_MASK)
                | ((value << Cfglr::MODE2_OFFSET) & Cfglr::MODE2_MASK),
        }
    }

    /// Set **CNF1** `[7:6]`
    ///
    /// Port n.1 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF1_MASK)
                | ((value << Cfglr::CNF1_OFFSET) & Cfglr::CNF1_MASK),
        }
    }

    /// Set **MODE1** `[5:4]`
    ///
    /// Port n.1 mode bits
    #[inline(always)]
    pub const fn with_mode1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE1_MASK)
                | ((value << Cfglr::MODE1_OFFSET) & Cfglr::MODE1_MASK),
        }
    }

    /// Set **CNF0** `[3:2]`
    ///
    /// Port n.0 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::CNF0_MASK)
                | ((value << Cfglr::CNF0_OFFSET) & Cfglr::CNF0_MASK),
        }
    }

    /// Set **MODE0** `[1:0]`
    ///
    /// Port n.0 mode bits
    #[inline(always)]
    pub const fn with_mode0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfglr::MODE0_MASK)
                | ((value << Cfglr::MODE0_OFFSET) & Cfglr::MODE0_MASK),
        }
    }
}

impl Default for CfglrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfglr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CfglrVal {
        CfglrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CfglrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CfglrVal) -> CfglrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGHR Register
///
/// Port configuration register high
/// (GPIOn_CFGHR)
///
/// Address offset: 0x0004
/// Reset value: 0x44444444
///
/// ## Fields
///
/// - **CNF15** `[31:30]` (RW)
///
///   Port n.15 configuration
///   bits
///
/// - **MODE15** `[29:28]` (RW)
///
///   Port n.15 mode bits
///
/// - **CNF14** `[27:26]` (RW)
///
///   Port n.14 configuration
///   bits
///
/// - **MODE14** `[25:24]` (RW)
///
///   Port n.14 mode bits
///
/// - **CNF13** `[23:22]` (RW)
///
///   Port n.13 configuration
///   bits
///
/// - **MODE13** `[21:20]` (RW)
///
///   Port n.13 mode bits
///
/// - **CNF12** `[19:18]` (RW)
///
///   Port n.12 configuration
///   bits
///
/// - **MODE12** `[17:16]` (RW)
///
///   Port n.12 mode bits
///
/// - **CNF11** `[15:14]` (RW)
///
///   Port n.11 configuration
///   bits
///
/// - **MODE11** `[13:12]` (RW)
///
///   Port n.11 mode bits
///
/// - **CNF10** `[11:10]` (RW)
///
///   Port n.10 configuration
///   bits
///
/// - **MODE10** `[9:8]` (RW)
///
///   Port n.10 mode bits
///
/// - **CNF9** `[7:6]` (RW)
///
///   Port n.9 configuration
///   bits
///
/// - **MODE9** `[5:4]` (RW)
///
///   Port n.9 mode bits
///
/// - **CNF8** `[3:2]` (RW)
///
///   Port n.8 configuration
///   bits
///
/// - **MODE8** `[1:0]` (RW)
///
///   Port n.8 mode bits
#[repr(transparent)]
pub struct Cfghr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfghr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfghr {
    // ======== Field Constants ========

    /// **CNF15**
    ///
    /// Port n.15 configuration
    /// bits
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CNF15_OFFSET: u32 = 30;
    pub const CNF15_MASK: u32 = 0xc0000000;

    /// **MODE15**
    ///
    /// Port n.15 mode bits
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const MODE15_OFFSET: u32 = 28;
    pub const MODE15_MASK: u32 = 0x30000000;

    /// **CNF14**
    ///
    /// Port n.14 configuration
    /// bits
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CNF14_OFFSET: u32 = 26;
    pub const CNF14_MASK: u32 = 0x0c000000;

    /// **MODE14**
    ///
    /// Port n.14 mode bits
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const MODE14_OFFSET: u32 = 24;
    pub const MODE14_MASK: u32 = 0x03000000;

    /// **CNF13**
    ///
    /// Port n.13 configuration
    /// bits
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CNF13_OFFSET: u32 = 22;
    pub const CNF13_MASK: u32 = 0x00c00000;

    /// **MODE13**
    ///
    /// Port n.13 mode bits
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const MODE13_OFFSET: u32 = 20;
    pub const MODE13_MASK: u32 = 0x00300000;

    /// **CNF12**
    ///
    /// Port n.12 configuration
    /// bits
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CNF12_OFFSET: u32 = 18;
    pub const CNF12_MASK: u32 = 0x000c0000;

    /// **MODE12**
    ///
    /// Port n.12 mode bits
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const MODE12_OFFSET: u32 = 16;
    pub const MODE12_MASK: u32 = 0x00030000;

    /// **CNF11**
    ///
    /// Port n.11 configuration
    /// bits
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const CNF11_OFFSET: u32 = 14;
    pub const CNF11_MASK: u32 = 0x0000c000;

    /// **MODE11**
    ///
    /// Port n.11 mode bits
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const MODE11_OFFSET: u32 = 12;
    pub const MODE11_MASK: u32 = 0x00003000;

    /// **CNF10**
    ///
    /// Port n.10 configuration
    /// bits
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const CNF10_OFFSET: u32 = 10;
    pub const CNF10_MASK: u32 = 0x00000c00;

    /// **MODE10**
    ///
    /// Port n.10 mode bits
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const MODE10_OFFSET: u32 = 8;
    pub const MODE10_MASK: u32 = 0x00000300;

    /// **CNF9**
    ///
    /// Port n.9 configuration
    /// bits
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const CNF9_OFFSET: u32 = 6;
    pub const CNF9_MASK: u32 = 0x000000c0;

    /// **MODE9**
    ///
    /// Port n.9 mode bits
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const MODE9_OFFSET: u32 = 4;
    pub const MODE9_MASK: u32 = 0x00000030;

    /// **CNF8**
    ///
    /// Port n.8 configuration
    /// bits
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const CNF8_OFFSET: u32 = 2;
    pub const CNF8_MASK: u32 = 0x0000000c;

    /// **MODE8**
    ///
    /// Port n.8 mode bits
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MODE8_OFFSET: u32 = 0;
    pub const MODE8_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNF15** `[31:30]` (RW)
    ///
    /// Port n.15 configuration
    /// bits
    #[inline(always)]
    pub fn cnf15(&self) -> u32 {
        (self.value.read_bits() & Self::CNF15_MASK) >> Self::CNF15_OFFSET
    }

    /// Write **CNF15** `[31:30]` (RW)
    ///
    /// Port n.15 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF15_MASK) | ((value << Self::CNF15_OFFSET) & Self::CNF15_MASK)
        });
    }

    /// **MODE15** `[29:28]` (RW)
    ///
    /// Port n.15 mode bits
    #[inline(always)]
    pub fn mode15(&self) -> u32 {
        (self.value.read_bits() & Self::MODE15_MASK) >> Self::MODE15_OFFSET
    }

    /// Write **MODE15** `[29:28]` (RW)
    ///
    /// Port n.15 mode bits
    #[inline(always)]
    pub fn write_mode15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE15_MASK) | ((value << Self::MODE15_OFFSET) & Self::MODE15_MASK)
        });
    }

    /// **CNF14** `[27:26]` (RW)
    ///
    /// Port n.14 configuration
    /// bits
    #[inline(always)]
    pub fn cnf14(&self) -> u32 {
        (self.value.read_bits() & Self::CNF14_MASK) >> Self::CNF14_OFFSET
    }

    /// Write **CNF14** `[27:26]` (RW)
    ///
    /// Port n.14 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF14_MASK) | ((value << Self::CNF14_OFFSET) & Self::CNF14_MASK)
        });
    }

    /// **MODE14** `[25:24]` (RW)
    ///
    /// Port n.14 mode bits
    #[inline(always)]
    pub fn mode14(&self) -> u32 {
        (self.value.read_bits() & Self::MODE14_MASK) >> Self::MODE14_OFFSET
    }

    /// Write **MODE14** `[25:24]` (RW)
    ///
    /// Port n.14 mode bits
    #[inline(always)]
    pub fn write_mode14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE14_MASK) | ((value << Self::MODE14_OFFSET) & Self::MODE14_MASK)
        });
    }

    /// **CNF13** `[23:22]` (RW)
    ///
    /// Port n.13 configuration
    /// bits
    #[inline(always)]
    pub fn cnf13(&self) -> u32 {
        (self.value.read_bits() & Self::CNF13_MASK) >> Self::CNF13_OFFSET
    }

    /// Write **CNF13** `[23:22]` (RW)
    ///
    /// Port n.13 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF13_MASK) | ((value << Self::CNF13_OFFSET) & Self::CNF13_MASK)
        });
    }

    /// **MODE13** `[21:20]` (RW)
    ///
    /// Port n.13 mode bits
    #[inline(always)]
    pub fn mode13(&self) -> u32 {
        (self.value.read_bits() & Self::MODE13_MASK) >> Self::MODE13_OFFSET
    }

    /// Write **MODE13** `[21:20]` (RW)
    ///
    /// Port n.13 mode bits
    #[inline(always)]
    pub fn write_mode13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE13_MASK) | ((value << Self::MODE13_OFFSET) & Self::MODE13_MASK)
        });
    }

    /// **CNF12** `[19:18]` (RW)
    ///
    /// Port n.12 configuration
    /// bits
    #[inline(always)]
    pub fn cnf12(&self) -> u32 {
        (self.value.read_bits() & Self::CNF12_MASK) >> Self::CNF12_OFFSET
    }

    /// Write **CNF12** `[19:18]` (RW)
    ///
    /// Port n.12 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF12_MASK) | ((value << Self::CNF12_OFFSET) & Self::CNF12_MASK)
        });
    }

    /// **MODE12** `[17:16]` (RW)
    ///
    /// Port n.12 mode bits
    #[inline(always)]
    pub fn mode12(&self) -> u32 {
        (self.value.read_bits() & Self::MODE12_MASK) >> Self::MODE12_OFFSET
    }

    /// Write **MODE12** `[17:16]` (RW)
    ///
    /// Port n.12 mode bits
    #[inline(always)]
    pub fn write_mode12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE12_MASK) | ((value << Self::MODE12_OFFSET) & Self::MODE12_MASK)
        });
    }

    /// **CNF11** `[15:14]` (RW)
    ///
    /// Port n.11 configuration
    /// bits
    #[inline(always)]
    pub fn cnf11(&self) -> u32 {
        (self.value.read_bits() & Self::CNF11_MASK) >> Self::CNF11_OFFSET
    }

    /// Write **CNF11** `[15:14]` (RW)
    ///
    /// Port n.11 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF11_MASK) | ((value << Self::CNF11_OFFSET) & Self::CNF11_MASK)
        });
    }

    /// **MODE11** `[13:12]` (RW)
    ///
    /// Port n.11 mode bits
    #[inline(always)]
    pub fn mode11(&self) -> u32 {
        (self.value.read_bits() & Self::MODE11_MASK) >> Self::MODE11_OFFSET
    }

    /// Write **MODE11** `[13:12]` (RW)
    ///
    /// Port n.11 mode bits
    #[inline(always)]
    pub fn write_mode11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE11_MASK) | ((value << Self::MODE11_OFFSET) & Self::MODE11_MASK)
        });
    }

    /// **CNF10** `[11:10]` (RW)
    ///
    /// Port n.10 configuration
    /// bits
    #[inline(always)]
    pub fn cnf10(&self) -> u32 {
        (self.value.read_bits() & Self::CNF10_MASK) >> Self::CNF10_OFFSET
    }

    /// Write **CNF10** `[11:10]` (RW)
    ///
    /// Port n.10 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF10_MASK) | ((value << Self::CNF10_OFFSET) & Self::CNF10_MASK)
        });
    }

    /// **MODE10** `[9:8]` (RW)
    ///
    /// Port n.10 mode bits
    #[inline(always)]
    pub fn mode10(&self) -> u32 {
        (self.value.read_bits() & Self::MODE10_MASK) >> Self::MODE10_OFFSET
    }

    /// Write **MODE10** `[9:8]` (RW)
    ///
    /// Port n.10 mode bits
    #[inline(always)]
    pub fn write_mode10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE10_MASK) | ((value << Self::MODE10_OFFSET) & Self::MODE10_MASK)
        });
    }

    /// **CNF9** `[7:6]` (RW)
    ///
    /// Port n.9 configuration
    /// bits
    #[inline(always)]
    pub fn cnf9(&self) -> u32 {
        (self.value.read_bits() & Self::CNF9_MASK) >> Self::CNF9_OFFSET
    }

    /// Write **CNF9** `[7:6]` (RW)
    ///
    /// Port n.9 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF9_MASK) | ((value << Self::CNF9_OFFSET) & Self::CNF9_MASK)
        });
    }

    /// **MODE9** `[5:4]` (RW)
    ///
    /// Port n.9 mode bits
    #[inline(always)]
    pub fn mode9(&self) -> u32 {
        (self.value.read_bits() & Self::MODE9_MASK) >> Self::MODE9_OFFSET
    }

    /// Write **MODE9** `[5:4]` (RW)
    ///
    /// Port n.9 mode bits
    #[inline(always)]
    pub fn write_mode9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE9_MASK) | ((value << Self::MODE9_OFFSET) & Self::MODE9_MASK)
        });
    }

    /// **CNF8** `[3:2]` (RW)
    ///
    /// Port n.8 configuration
    /// bits
    #[inline(always)]
    pub fn cnf8(&self) -> u32 {
        (self.value.read_bits() & Self::CNF8_MASK) >> Self::CNF8_OFFSET
    }

    /// Write **CNF8** `[3:2]` (RW)
    ///
    /// Port n.8 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF8_MASK) | ((value << Self::CNF8_OFFSET) & Self::CNF8_MASK)
        });
    }

    /// **MODE8** `[1:0]` (RW)
    ///
    /// Port n.8 mode bits
    #[inline(always)]
    pub fn mode8(&self) -> u32 {
        (self.value.read_bits() & Self::MODE8_MASK) >> Self::MODE8_OFFSET
    }

    /// Write **MODE8** `[1:0]` (RW)
    ///
    /// Port n.8 mode bits
    #[inline(always)]
    pub fn write_mode8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE8_MASK) | ((value << Self::MODE8_OFFSET) & Self::MODE8_MASK)
        });
    }
}

/// CFGHR value type for batch read/modify/write operations
///
/// Use `cfghr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfghr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CfghrVal {
    bits: u32,
}

impl CfghrVal {
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
        Self { bits: 0x44444444 }
    }

    /// Read **CNF15** `[31:30]`
    ///
    /// Port n.15 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf15(&self) -> u32 {
        (self.bits & Cfghr::CNF15_MASK) >> Cfghr::CNF15_OFFSET
    }

    /// Read **MODE15** `[29:28]`
    ///
    /// Port n.15 mode bits
    #[inline(always)]
    pub const fn mode15(&self) -> u32 {
        (self.bits & Cfghr::MODE15_MASK) >> Cfghr::MODE15_OFFSET
    }

    /// Read **CNF14** `[27:26]`
    ///
    /// Port n.14 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf14(&self) -> u32 {
        (self.bits & Cfghr::CNF14_MASK) >> Cfghr::CNF14_OFFSET
    }

    /// Read **MODE14** `[25:24]`
    ///
    /// Port n.14 mode bits
    #[inline(always)]
    pub const fn mode14(&self) -> u32 {
        (self.bits & Cfghr::MODE14_MASK) >> Cfghr::MODE14_OFFSET
    }

    /// Read **CNF13** `[23:22]`
    ///
    /// Port n.13 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf13(&self) -> u32 {
        (self.bits & Cfghr::CNF13_MASK) >> Cfghr::CNF13_OFFSET
    }

    /// Read **MODE13** `[21:20]`
    ///
    /// Port n.13 mode bits
    #[inline(always)]
    pub const fn mode13(&self) -> u32 {
        (self.bits & Cfghr::MODE13_MASK) >> Cfghr::MODE13_OFFSET
    }

    /// Read **CNF12** `[19:18]`
    ///
    /// Port n.12 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf12(&self) -> u32 {
        (self.bits & Cfghr::CNF12_MASK) >> Cfghr::CNF12_OFFSET
    }

    /// Read **MODE12** `[17:16]`
    ///
    /// Port n.12 mode bits
    #[inline(always)]
    pub const fn mode12(&self) -> u32 {
        (self.bits & Cfghr::MODE12_MASK) >> Cfghr::MODE12_OFFSET
    }

    /// Read **CNF11** `[15:14]`
    ///
    /// Port n.11 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf11(&self) -> u32 {
        (self.bits & Cfghr::CNF11_MASK) >> Cfghr::CNF11_OFFSET
    }

    /// Read **MODE11** `[13:12]`
    ///
    /// Port n.11 mode bits
    #[inline(always)]
    pub const fn mode11(&self) -> u32 {
        (self.bits & Cfghr::MODE11_MASK) >> Cfghr::MODE11_OFFSET
    }

    /// Read **CNF10** `[11:10]`
    ///
    /// Port n.10 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf10(&self) -> u32 {
        (self.bits & Cfghr::CNF10_MASK) >> Cfghr::CNF10_OFFSET
    }

    /// Read **MODE10** `[9:8]`
    ///
    /// Port n.10 mode bits
    #[inline(always)]
    pub const fn mode10(&self) -> u32 {
        (self.bits & Cfghr::MODE10_MASK) >> Cfghr::MODE10_OFFSET
    }

    /// Read **CNF9** `[7:6]`
    ///
    /// Port n.9 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf9(&self) -> u32 {
        (self.bits & Cfghr::CNF9_MASK) >> Cfghr::CNF9_OFFSET
    }

    /// Read **MODE9** `[5:4]`
    ///
    /// Port n.9 mode bits
    #[inline(always)]
    pub const fn mode9(&self) -> u32 {
        (self.bits & Cfghr::MODE9_MASK) >> Cfghr::MODE9_OFFSET
    }

    /// Read **CNF8** `[3:2]`
    ///
    /// Port n.8 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf8(&self) -> u32 {
        (self.bits & Cfghr::CNF8_MASK) >> Cfghr::CNF8_OFFSET
    }

    /// Read **MODE8** `[1:0]`
    ///
    /// Port n.8 mode bits
    #[inline(always)]
    pub const fn mode8(&self) -> u32 {
        (self.bits & Cfghr::MODE8_MASK) >> Cfghr::MODE8_OFFSET
    }

    /// Set **CNF15** `[31:30]`
    ///
    /// Port n.15 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF15_MASK)
                | ((value << Cfghr::CNF15_OFFSET) & Cfghr::CNF15_MASK),
        }
    }

    /// Set **MODE15** `[29:28]`
    ///
    /// Port n.15 mode bits
    #[inline(always)]
    pub const fn with_mode15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE15_MASK)
                | ((value << Cfghr::MODE15_OFFSET) & Cfghr::MODE15_MASK),
        }
    }

    /// Set **CNF14** `[27:26]`
    ///
    /// Port n.14 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF14_MASK)
                | ((value << Cfghr::CNF14_OFFSET) & Cfghr::CNF14_MASK),
        }
    }

    /// Set **MODE14** `[25:24]`
    ///
    /// Port n.14 mode bits
    #[inline(always)]
    pub const fn with_mode14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE14_MASK)
                | ((value << Cfghr::MODE14_OFFSET) & Cfghr::MODE14_MASK),
        }
    }

    /// Set **CNF13** `[23:22]`
    ///
    /// Port n.13 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF13_MASK)
                | ((value << Cfghr::CNF13_OFFSET) & Cfghr::CNF13_MASK),
        }
    }

    /// Set **MODE13** `[21:20]`
    ///
    /// Port n.13 mode bits
    #[inline(always)]
    pub const fn with_mode13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE13_MASK)
                | ((value << Cfghr::MODE13_OFFSET) & Cfghr::MODE13_MASK),
        }
    }

    /// Set **CNF12** `[19:18]`
    ///
    /// Port n.12 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF12_MASK)
                | ((value << Cfghr::CNF12_OFFSET) & Cfghr::CNF12_MASK),
        }
    }

    /// Set **MODE12** `[17:16]`
    ///
    /// Port n.12 mode bits
    #[inline(always)]
    pub const fn with_mode12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE12_MASK)
                | ((value << Cfghr::MODE12_OFFSET) & Cfghr::MODE12_MASK),
        }
    }

    /// Set **CNF11** `[15:14]`
    ///
    /// Port n.11 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF11_MASK)
                | ((value << Cfghr::CNF11_OFFSET) & Cfghr::CNF11_MASK),
        }
    }

    /// Set **MODE11** `[13:12]`
    ///
    /// Port n.11 mode bits
    #[inline(always)]
    pub const fn with_mode11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE11_MASK)
                | ((value << Cfghr::MODE11_OFFSET) & Cfghr::MODE11_MASK),
        }
    }

    /// Set **CNF10** `[11:10]`
    ///
    /// Port n.10 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF10_MASK)
                | ((value << Cfghr::CNF10_OFFSET) & Cfghr::CNF10_MASK),
        }
    }

    /// Set **MODE10** `[9:8]`
    ///
    /// Port n.10 mode bits
    #[inline(always)]
    pub const fn with_mode10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE10_MASK)
                | ((value << Cfghr::MODE10_OFFSET) & Cfghr::MODE10_MASK),
        }
    }

    /// Set **CNF9** `[7:6]`
    ///
    /// Port n.9 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF9_MASK)
                | ((value << Cfghr::CNF9_OFFSET) & Cfghr::CNF9_MASK),
        }
    }

    /// Set **MODE9** `[5:4]`
    ///
    /// Port n.9 mode bits
    #[inline(always)]
    pub const fn with_mode9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE9_MASK)
                | ((value << Cfghr::MODE9_OFFSET) & Cfghr::MODE9_MASK),
        }
    }

    /// Set **CNF8** `[3:2]`
    ///
    /// Port n.8 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::CNF8_MASK)
                | ((value << Cfghr::CNF8_OFFSET) & Cfghr::CNF8_MASK),
        }
    }

    /// Set **MODE8** `[1:0]`
    ///
    /// Port n.8 mode bits
    #[inline(always)]
    pub const fn with_mode8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfghr::MODE8_MASK)
                | ((value << Cfghr::MODE8_OFFSET) & Cfghr::MODE8_MASK),
        }
    }
}

impl Default for CfghrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfghr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CfghrVal {
        CfghrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CfghrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CfghrVal) -> CfghrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INDR Register
///
/// Port input data register
/// (GPIOn_INDR)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IDR23** `[23]` (RO)
///
///   Port input data
///
/// - **IDR22** `[22]` (RO)
///
///   Port input data
///
/// - **IDR21** `[21]` (RO)
///
///   Port input data
///
/// - **IDR20** `[20]` (RO)
///
///   Port input data
///
/// - **IDR19** `[19]` (RO)
///
///   Port input data
///
/// - **IDR18** `[18]` (RO)
///
///   Port input data
///
/// - **IDR17** `[17]` (RO)
///
///   Port input data
///
/// - **IDR16** `[16]` (RO)
///
///   Port input data
///
/// - **IDR15** `[15]` (RO)
///
///   Port input data
///
/// - **IDR14** `[14]` (RO)
///
///   Port input data
///
/// - **IDR13** `[13]` (RO)
///
///   Port input data
///
/// - **IDR12** `[12]` (RO)
///
///   Port input data
///
/// - **IDR11** `[11]` (RO)
///
///   Port input data
///
/// - **IDR10** `[10]` (RO)
///
///   Port input data
///
/// - **IDR9** `[9]` (RO)
///
///   Port input data
///
/// - **IDR8** `[8]` (RO)
///
///   Port input data
///
/// - **IDR7** `[7]` (RO)
///
///   Port input data
///
/// - **IDR6** `[6]` (RO)
///
///   Port input data
///
/// - **IDR5** `[5]` (RO)
///
///   Port input data
///
/// - **IDR4** `[4]` (RO)
///
///   Port input data
///
/// - **IDR3** `[3]` (RO)
///
///   Port input data
///
/// - **IDR2** `[2]` (RO)
///
///   Port input data
///
/// - **IDR1** `[1]` (RO)
///
///   Port input data
///
/// - **IDR0** `[0]` (RO)
///
///   Port input data
#[repr(transparent)]
pub struct Indr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Indr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Indr {
    // ======== Field Constants ========

    /// **IDR23**
    ///
    /// Port input data
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const IDR23_OFFSET: u32 = 23;
    pub const IDR23_MASK: u32 = 0x00800000;
    pub const IDR23: u32 = 1 << 23;

    /// **IDR22**
    ///
    /// Port input data
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const IDR22_OFFSET: u32 = 22;
    pub const IDR22_MASK: u32 = 0x00400000;
    pub const IDR22: u32 = 1 << 22;

    /// **IDR21**
    ///
    /// Port input data
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const IDR21_OFFSET: u32 = 21;
    pub const IDR21_MASK: u32 = 0x00200000;
    pub const IDR21: u32 = 1 << 21;

    /// **IDR20**
    ///
    /// Port input data
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const IDR20_OFFSET: u32 = 20;
    pub const IDR20_MASK: u32 = 0x00100000;
    pub const IDR20: u32 = 1 << 20;

    /// **IDR19**
    ///
    /// Port input data
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const IDR19_OFFSET: u32 = 19;
    pub const IDR19_MASK: u32 = 0x00080000;
    pub const IDR19: u32 = 1 << 19;

    /// **IDR18**
    ///
    /// Port input data
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const IDR18_OFFSET: u32 = 18;
    pub const IDR18_MASK: u32 = 0x00040000;
    pub const IDR18: u32 = 1 << 18;

    /// **IDR17**
    ///
    /// Port input data
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const IDR17_OFFSET: u32 = 17;
    pub const IDR17_MASK: u32 = 0x00020000;
    pub const IDR17: u32 = 1 << 17;

    /// **IDR16**
    ///
    /// Port input data
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const IDR16_OFFSET: u32 = 16;
    pub const IDR16_MASK: u32 = 0x00010000;
    pub const IDR16: u32 = 1 << 16;

    /// **IDR15**
    ///
    /// Port input data
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const IDR15_OFFSET: u32 = 15;
    pub const IDR15_MASK: u32 = 0x00008000;
    pub const IDR15: u32 = 1 << 15;

    /// **IDR14**
    ///
    /// Port input data
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IDR14_OFFSET: u32 = 14;
    pub const IDR14_MASK: u32 = 0x00004000;
    pub const IDR14: u32 = 1 << 14;

    /// **IDR13**
    ///
    /// Port input data
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IDR13_OFFSET: u32 = 13;
    pub const IDR13_MASK: u32 = 0x00002000;
    pub const IDR13: u32 = 1 << 13;

    /// **IDR12**
    ///
    /// Port input data
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IDR12_OFFSET: u32 = 12;
    pub const IDR12_MASK: u32 = 0x00001000;
    pub const IDR12: u32 = 1 << 12;

    /// **IDR11**
    ///
    /// Port input data
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IDR11_OFFSET: u32 = 11;
    pub const IDR11_MASK: u32 = 0x00000800;
    pub const IDR11: u32 = 1 << 11;

    /// **IDR10**
    ///
    /// Port input data
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const IDR10_OFFSET: u32 = 10;
    pub const IDR10_MASK: u32 = 0x00000400;
    pub const IDR10: u32 = 1 << 10;

    /// **IDR9**
    ///
    /// Port input data
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const IDR9_OFFSET: u32 = 9;
    pub const IDR9_MASK: u32 = 0x00000200;
    pub const IDR9: u32 = 1 << 9;

    /// **IDR8**
    ///
    /// Port input data
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IDR8_OFFSET: u32 = 8;
    pub const IDR8_MASK: u32 = 0x00000100;
    pub const IDR8: u32 = 1 << 8;

    /// **IDR7**
    ///
    /// Port input data
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const IDR7_OFFSET: u32 = 7;
    pub const IDR7_MASK: u32 = 0x00000080;
    pub const IDR7: u32 = 1 << 7;

    /// **IDR6**
    ///
    /// Port input data
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IDR6_OFFSET: u32 = 6;
    pub const IDR6_MASK: u32 = 0x00000040;
    pub const IDR6: u32 = 1 << 6;

    /// **IDR5**
    ///
    /// Port input data
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IDR5_OFFSET: u32 = 5;
    pub const IDR5_MASK: u32 = 0x00000020;
    pub const IDR5: u32 = 1 << 5;

    /// **IDR4**
    ///
    /// Port input data
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IDR4_OFFSET: u32 = 4;
    pub const IDR4_MASK: u32 = 0x00000010;
    pub const IDR4: u32 = 1 << 4;

    /// **IDR3**
    ///
    /// Port input data
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IDR3_OFFSET: u32 = 3;
    pub const IDR3_MASK: u32 = 0x00000008;
    pub const IDR3: u32 = 1 << 3;

    /// **IDR2**
    ///
    /// Port input data
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IDR2_OFFSET: u32 = 2;
    pub const IDR2_MASK: u32 = 0x00000004;
    pub const IDR2: u32 = 1 << 2;

    /// **IDR1**
    ///
    /// Port input data
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IDR1_OFFSET: u32 = 1;
    pub const IDR1_MASK: u32 = 0x00000002;
    pub const IDR1: u32 = 1 << 1;

    /// **IDR0**
    ///
    /// Port input data
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IDR0_OFFSET: u32 = 0;
    pub const IDR0_MASK: u32 = 0x00000001;
    pub const IDR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IDR23** `[23]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr23(&self) -> bool {
        (self.value.read_bits() & Self::IDR23_MASK) != 0
    }

    /// **IDR22** `[22]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr22(&self) -> bool {
        (self.value.read_bits() & Self::IDR22_MASK) != 0
    }

    /// **IDR21** `[21]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr21(&self) -> bool {
        (self.value.read_bits() & Self::IDR21_MASK) != 0
    }

    /// **IDR20** `[20]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr20(&self) -> bool {
        (self.value.read_bits() & Self::IDR20_MASK) != 0
    }

    /// **IDR19** `[19]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr19(&self) -> bool {
        (self.value.read_bits() & Self::IDR19_MASK) != 0
    }

    /// **IDR18** `[18]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr18(&self) -> bool {
        (self.value.read_bits() & Self::IDR18_MASK) != 0
    }

    /// **IDR17** `[17]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr17(&self) -> bool {
        (self.value.read_bits() & Self::IDR17_MASK) != 0
    }

    /// **IDR16** `[16]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr16(&self) -> bool {
        (self.value.read_bits() & Self::IDR16_MASK) != 0
    }

    /// **IDR15** `[15]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr15(&self) -> bool {
        (self.value.read_bits() & Self::IDR15_MASK) != 0
    }

    /// **IDR14** `[14]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr14(&self) -> bool {
        (self.value.read_bits() & Self::IDR14_MASK) != 0
    }

    /// **IDR13** `[13]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr13(&self) -> bool {
        (self.value.read_bits() & Self::IDR13_MASK) != 0
    }

    /// **IDR12** `[12]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr12(&self) -> bool {
        (self.value.read_bits() & Self::IDR12_MASK) != 0
    }

    /// **IDR11** `[11]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr11(&self) -> bool {
        (self.value.read_bits() & Self::IDR11_MASK) != 0
    }

    /// **IDR10** `[10]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr10(&self) -> bool {
        (self.value.read_bits() & Self::IDR10_MASK) != 0
    }

    /// **IDR9** `[9]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr9(&self) -> bool {
        (self.value.read_bits() & Self::IDR9_MASK) != 0
    }

    /// **IDR8** `[8]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr8(&self) -> bool {
        (self.value.read_bits() & Self::IDR8_MASK) != 0
    }

    /// **IDR7** `[7]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr7(&self) -> bool {
        (self.value.read_bits() & Self::IDR7_MASK) != 0
    }

    /// **IDR6** `[6]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr6(&self) -> bool {
        (self.value.read_bits() & Self::IDR6_MASK) != 0
    }

    /// **IDR5** `[5]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr5(&self) -> bool {
        (self.value.read_bits() & Self::IDR5_MASK) != 0
    }

    /// **IDR4** `[4]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr4(&self) -> bool {
        (self.value.read_bits() & Self::IDR4_MASK) != 0
    }

    /// **IDR3** `[3]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr3(&self) -> bool {
        (self.value.read_bits() & Self::IDR3_MASK) != 0
    }

    /// **IDR2** `[2]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr2(&self) -> bool {
        (self.value.read_bits() & Self::IDR2_MASK) != 0
    }

    /// **IDR1** `[1]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr1(&self) -> bool {
        (self.value.read_bits() & Self::IDR1_MASK) != 0
    }

    /// **IDR0** `[0]` (RO)
    ///
    /// Port input data
    #[inline(always)]
    pub fn idr0(&self) -> bool {
        (self.value.read_bits() & Self::IDR0_MASK) != 0
    }
}

/// INDR value type for batch read/modify/write operations
///
/// Use `indr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `indr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IndrVal {
    bits: u32,
}

impl IndrVal {
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

    /// Read **IDR23** `[23:23]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr23(&self) -> bool {
        (self.bits & Indr::IDR23_MASK) != 0
    }

    /// Read **IDR22** `[22:22]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr22(&self) -> bool {
        (self.bits & Indr::IDR22_MASK) != 0
    }

    /// Read **IDR21** `[21:21]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr21(&self) -> bool {
        (self.bits & Indr::IDR21_MASK) != 0
    }

    /// Read **IDR20** `[20:20]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr20(&self) -> bool {
        (self.bits & Indr::IDR20_MASK) != 0
    }

    /// Read **IDR19** `[19:19]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr19(&self) -> bool {
        (self.bits & Indr::IDR19_MASK) != 0
    }

    /// Read **IDR18** `[18:18]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr18(&self) -> bool {
        (self.bits & Indr::IDR18_MASK) != 0
    }

    /// Read **IDR17** `[17:17]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr17(&self) -> bool {
        (self.bits & Indr::IDR17_MASK) != 0
    }

    /// Read **IDR16** `[16:16]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr16(&self) -> bool {
        (self.bits & Indr::IDR16_MASK) != 0
    }

    /// Read **IDR15** `[15:15]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr15(&self) -> bool {
        (self.bits & Indr::IDR15_MASK) != 0
    }

    /// Read **IDR14** `[14:14]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr14(&self) -> bool {
        (self.bits & Indr::IDR14_MASK) != 0
    }

    /// Read **IDR13** `[13:13]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr13(&self) -> bool {
        (self.bits & Indr::IDR13_MASK) != 0
    }

    /// Read **IDR12** `[12:12]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr12(&self) -> bool {
        (self.bits & Indr::IDR12_MASK) != 0
    }

    /// Read **IDR11** `[11:11]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr11(&self) -> bool {
        (self.bits & Indr::IDR11_MASK) != 0
    }

    /// Read **IDR10** `[10:10]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr10(&self) -> bool {
        (self.bits & Indr::IDR10_MASK) != 0
    }

    /// Read **IDR9** `[9:9]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr9(&self) -> bool {
        (self.bits & Indr::IDR9_MASK) != 0
    }

    /// Read **IDR8** `[8:8]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr8(&self) -> bool {
        (self.bits & Indr::IDR8_MASK) != 0
    }

    /// Read **IDR7** `[7:7]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr7(&self) -> bool {
        (self.bits & Indr::IDR7_MASK) != 0
    }

    /// Read **IDR6** `[6:6]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr6(&self) -> bool {
        (self.bits & Indr::IDR6_MASK) != 0
    }

    /// Read **IDR5** `[5:5]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr5(&self) -> bool {
        (self.bits & Indr::IDR5_MASK) != 0
    }

    /// Read **IDR4** `[4:4]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr4(&self) -> bool {
        (self.bits & Indr::IDR4_MASK) != 0
    }

    /// Read **IDR3** `[3:3]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr3(&self) -> bool {
        (self.bits & Indr::IDR3_MASK) != 0
    }

    /// Read **IDR2** `[2:2]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr2(&self) -> bool {
        (self.bits & Indr::IDR2_MASK) != 0
    }

    /// Read **IDR1** `[1:1]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr1(&self) -> bool {
        (self.bits & Indr::IDR1_MASK) != 0
    }

    /// Read **IDR0** `[0:0]`
    ///
    /// Port input data
    #[inline(always)]
    pub const fn idr0(&self) -> bool {
        (self.bits & Indr::IDR0_MASK) != 0
    }
}

impl Default for IndrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Indr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IndrVal {
        IndrVal::from_bits(self.value.read_bits())
    }
}

/// OUTDR Register
///
/// Port output data register
/// (GPIOn_OUTDR)
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ODR23** `[23]` (RW)
///
///   Port output data
///
/// - **ODR22** `[22]` (RW)
///
///   Port output data
///
/// - **ODR21** `[21]` (RW)
///
///   Port output data
///
/// - **ODR20** `[20]` (RW)
///
///   Port output data
///
/// - **ODR19** `[19]` (RW)
///
///   Port output data
///
/// - **ODR18** `[18]` (RW)
///
///   Port output data
///
/// - **ODR17** `[17]` (RW)
///
///   Port output data
///
/// - **ODR16** `[16]` (RW)
///
///   Port output data
///
/// - **ODR15** `[15]` (RW)
///
///   Port output data
///
/// - **ODR14** `[14]` (RW)
///
///   Port output data
///
/// - **ODR13** `[13]` (RW)
///
///   Port output data
///
/// - **ODR12** `[12]` (RW)
///
///   Port output data
///
/// - **ODR11** `[11]` (RW)
///
///   Port output data
///
/// - **ODR10** `[10]` (RW)
///
///   Port output data
///
/// - **ODR9** `[9]` (RW)
///
///   Port output data
///
/// - **ODR8** `[8]` (RW)
///
///   Port output data
///
/// - **ODR7** `[7]` (RW)
///
///   Port output data
///
/// - **ODR6** `[6]` (RW)
///
///   Port output data
///
/// - **ODR5** `[5]` (RW)
///
///   Port output data
///
/// - **ODR4** `[4]` (RW)
///
///   Port output data
///
/// - **ODR3** `[3]` (RW)
///
///   Port output data
///
/// - **ODR2** `[2]` (RW)
///
///   Port output data
///
/// - **ODR1** `[1]` (RW)
///
///   Port output data
///
/// - **ODR0** `[0]` (RW)
///
///   Port output data
#[repr(transparent)]
pub struct Outdr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Outdr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Outdr {
    // ======== Field Constants ========

    /// **ODR23**
    ///
    /// Port output data
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const ODR23_OFFSET: u32 = 23;
    pub const ODR23_MASK: u32 = 0x00800000;
    pub const ODR23: u32 = 1 << 23;

    /// **ODR22**
    ///
    /// Port output data
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ODR22_OFFSET: u32 = 22;
    pub const ODR22_MASK: u32 = 0x00400000;
    pub const ODR22: u32 = 1 << 22;

    /// **ODR21**
    ///
    /// Port output data
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const ODR21_OFFSET: u32 = 21;
    pub const ODR21_MASK: u32 = 0x00200000;
    pub const ODR21: u32 = 1 << 21;

    /// **ODR20**
    ///
    /// Port output data
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const ODR20_OFFSET: u32 = 20;
    pub const ODR20_MASK: u32 = 0x00100000;
    pub const ODR20: u32 = 1 << 20;

    /// **ODR19**
    ///
    /// Port output data
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const ODR19_OFFSET: u32 = 19;
    pub const ODR19_MASK: u32 = 0x00080000;
    pub const ODR19: u32 = 1 << 19;

    /// **ODR18**
    ///
    /// Port output data
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ODR18_OFFSET: u32 = 18;
    pub const ODR18_MASK: u32 = 0x00040000;
    pub const ODR18: u32 = 1 << 18;

    /// **ODR17**
    ///
    /// Port output data
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const ODR17_OFFSET: u32 = 17;
    pub const ODR17_MASK: u32 = 0x00020000;
    pub const ODR17: u32 = 1 << 17;

    /// **ODR16**
    ///
    /// Port output data
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ODR16_OFFSET: u32 = 16;
    pub const ODR16_MASK: u32 = 0x00010000;
    pub const ODR16: u32 = 1 << 16;

    /// **ODR15**
    ///
    /// Port output data
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ODR15_OFFSET: u32 = 15;
    pub const ODR15_MASK: u32 = 0x00008000;
    pub const ODR15: u32 = 1 << 15;

    /// **ODR14**
    ///
    /// Port output data
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ODR14_OFFSET: u32 = 14;
    pub const ODR14_MASK: u32 = 0x00004000;
    pub const ODR14: u32 = 1 << 14;

    /// **ODR13**
    ///
    /// Port output data
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ODR13_OFFSET: u32 = 13;
    pub const ODR13_MASK: u32 = 0x00002000;
    pub const ODR13: u32 = 1 << 13;

    /// **ODR12**
    ///
    /// Port output data
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ODR12_OFFSET: u32 = 12;
    pub const ODR12_MASK: u32 = 0x00001000;
    pub const ODR12: u32 = 1 << 12;

    /// **ODR11**
    ///
    /// Port output data
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ODR11_OFFSET: u32 = 11;
    pub const ODR11_MASK: u32 = 0x00000800;
    pub const ODR11: u32 = 1 << 11;

    /// **ODR10**
    ///
    /// Port output data
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ODR10_OFFSET: u32 = 10;
    pub const ODR10_MASK: u32 = 0x00000400;
    pub const ODR10: u32 = 1 << 10;

    /// **ODR9**
    ///
    /// Port output data
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ODR9_OFFSET: u32 = 9;
    pub const ODR9_MASK: u32 = 0x00000200;
    pub const ODR9: u32 = 1 << 9;

    /// **ODR8**
    ///
    /// Port output data
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ODR8_OFFSET: u32 = 8;
    pub const ODR8_MASK: u32 = 0x00000100;
    pub const ODR8: u32 = 1 << 8;

    /// **ODR7**
    ///
    /// Port output data
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ODR7_OFFSET: u32 = 7;
    pub const ODR7_MASK: u32 = 0x00000080;
    pub const ODR7: u32 = 1 << 7;

    /// **ODR6**
    ///
    /// Port output data
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ODR6_OFFSET: u32 = 6;
    pub const ODR6_MASK: u32 = 0x00000040;
    pub const ODR6: u32 = 1 << 6;

    /// **ODR5**
    ///
    /// Port output data
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ODR5_OFFSET: u32 = 5;
    pub const ODR5_MASK: u32 = 0x00000020;
    pub const ODR5: u32 = 1 << 5;

    /// **ODR4**
    ///
    /// Port output data
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ODR4_OFFSET: u32 = 4;
    pub const ODR4_MASK: u32 = 0x00000010;
    pub const ODR4: u32 = 1 << 4;

    /// **ODR3**
    ///
    /// Port output data
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ODR3_OFFSET: u32 = 3;
    pub const ODR3_MASK: u32 = 0x00000008;
    pub const ODR3: u32 = 1 << 3;

    /// **ODR2**
    ///
    /// Port output data
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ODR2_OFFSET: u32 = 2;
    pub const ODR2_MASK: u32 = 0x00000004;
    pub const ODR2: u32 = 1 << 2;

    /// **ODR1**
    ///
    /// Port output data
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ODR1_OFFSET: u32 = 1;
    pub const ODR1_MASK: u32 = 0x00000002;
    pub const ODR1: u32 = 1 << 1;

    /// **ODR0**
    ///
    /// Port output data
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ODR0_OFFSET: u32 = 0;
    pub const ODR0_MASK: u32 = 0x00000001;
    pub const ODR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ODR23** `[23]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr23(&self) -> bool {
        (self.value.read_bits() & Self::ODR23_MASK) != 0
    }

    /// Set ODR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr23(&self) {
        self.value.modify_bits(|r| r | Self::ODR23_MASK);
    }

    /// Clear ODR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr23(&self) {
        self.value.modify_bits(|r| r & !Self::ODR23_MASK);
    }

    /// **ODR22** `[22]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr22(&self) -> bool {
        (self.value.read_bits() & Self::ODR22_MASK) != 0
    }

    /// Set ODR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr22(&self) {
        self.value.modify_bits(|r| r | Self::ODR22_MASK);
    }

    /// Clear ODR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr22(&self) {
        self.value.modify_bits(|r| r & !Self::ODR22_MASK);
    }

    /// **ODR21** `[21]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr21(&self) -> bool {
        (self.value.read_bits() & Self::ODR21_MASK) != 0
    }

    /// Set ODR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr21(&self) {
        self.value.modify_bits(|r| r | Self::ODR21_MASK);
    }

    /// Clear ODR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr21(&self) {
        self.value.modify_bits(|r| r & !Self::ODR21_MASK);
    }

    /// **ODR20** `[20]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr20(&self) -> bool {
        (self.value.read_bits() & Self::ODR20_MASK) != 0
    }

    /// Set ODR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr20(&self) {
        self.value.modify_bits(|r| r | Self::ODR20_MASK);
    }

    /// Clear ODR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr20(&self) {
        self.value.modify_bits(|r| r & !Self::ODR20_MASK);
    }

    /// **ODR19** `[19]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr19(&self) -> bool {
        (self.value.read_bits() & Self::ODR19_MASK) != 0
    }

    /// Set ODR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr19(&self) {
        self.value.modify_bits(|r| r | Self::ODR19_MASK);
    }

    /// Clear ODR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr19(&self) {
        self.value.modify_bits(|r| r & !Self::ODR19_MASK);
    }

    /// **ODR18** `[18]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr18(&self) -> bool {
        (self.value.read_bits() & Self::ODR18_MASK) != 0
    }

    /// Set ODR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr18(&self) {
        self.value.modify_bits(|r| r | Self::ODR18_MASK);
    }

    /// Clear ODR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr18(&self) {
        self.value.modify_bits(|r| r & !Self::ODR18_MASK);
    }

    /// **ODR17** `[17]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr17(&self) -> bool {
        (self.value.read_bits() & Self::ODR17_MASK) != 0
    }

    /// Set ODR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr17(&self) {
        self.value.modify_bits(|r| r | Self::ODR17_MASK);
    }

    /// Clear ODR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr17(&self) {
        self.value.modify_bits(|r| r & !Self::ODR17_MASK);
    }

    /// **ODR16** `[16]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr16(&self) -> bool {
        (self.value.read_bits() & Self::ODR16_MASK) != 0
    }

    /// Set ODR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr16(&self) {
        self.value.modify_bits(|r| r | Self::ODR16_MASK);
    }

    /// Clear ODR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr16(&self) {
        self.value.modify_bits(|r| r & !Self::ODR16_MASK);
    }

    /// **ODR15** `[15]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr15(&self) -> bool {
        (self.value.read_bits() & Self::ODR15_MASK) != 0
    }

    /// Set ODR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr15(&self) {
        self.value.modify_bits(|r| r | Self::ODR15_MASK);
    }

    /// Clear ODR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr15(&self) {
        self.value.modify_bits(|r| r & !Self::ODR15_MASK);
    }

    /// **ODR14** `[14]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr14(&self) -> bool {
        (self.value.read_bits() & Self::ODR14_MASK) != 0
    }

    /// Set ODR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr14(&self) {
        self.value.modify_bits(|r| r | Self::ODR14_MASK);
    }

    /// Clear ODR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr14(&self) {
        self.value.modify_bits(|r| r & !Self::ODR14_MASK);
    }

    /// **ODR13** `[13]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr13(&self) -> bool {
        (self.value.read_bits() & Self::ODR13_MASK) != 0
    }

    /// Set ODR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr13(&self) {
        self.value.modify_bits(|r| r | Self::ODR13_MASK);
    }

    /// Clear ODR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr13(&self) {
        self.value.modify_bits(|r| r & !Self::ODR13_MASK);
    }

    /// **ODR12** `[12]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr12(&self) -> bool {
        (self.value.read_bits() & Self::ODR12_MASK) != 0
    }

    /// Set ODR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr12(&self) {
        self.value.modify_bits(|r| r | Self::ODR12_MASK);
    }

    /// Clear ODR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr12(&self) {
        self.value.modify_bits(|r| r & !Self::ODR12_MASK);
    }

    /// **ODR11** `[11]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr11(&self) -> bool {
        (self.value.read_bits() & Self::ODR11_MASK) != 0
    }

    /// Set ODR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr11(&self) {
        self.value.modify_bits(|r| r | Self::ODR11_MASK);
    }

    /// Clear ODR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr11(&self) {
        self.value.modify_bits(|r| r & !Self::ODR11_MASK);
    }

    /// **ODR10** `[10]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr10(&self) -> bool {
        (self.value.read_bits() & Self::ODR10_MASK) != 0
    }

    /// Set ODR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr10(&self) {
        self.value.modify_bits(|r| r | Self::ODR10_MASK);
    }

    /// Clear ODR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr10(&self) {
        self.value.modify_bits(|r| r & !Self::ODR10_MASK);
    }

    /// **ODR9** `[9]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr9(&self) -> bool {
        (self.value.read_bits() & Self::ODR9_MASK) != 0
    }

    /// Set ODR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr9(&self) {
        self.value.modify_bits(|r| r | Self::ODR9_MASK);
    }

    /// Clear ODR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr9(&self) {
        self.value.modify_bits(|r| r & !Self::ODR9_MASK);
    }

    /// **ODR8** `[8]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr8(&self) -> bool {
        (self.value.read_bits() & Self::ODR8_MASK) != 0
    }

    /// Set ODR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr8(&self) {
        self.value.modify_bits(|r| r | Self::ODR8_MASK);
    }

    /// Clear ODR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr8(&self) {
        self.value.modify_bits(|r| r & !Self::ODR8_MASK);
    }

    /// **ODR7** `[7]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr7(&self) -> bool {
        (self.value.read_bits() & Self::ODR7_MASK) != 0
    }

    /// Set ODR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr7(&self) {
        self.value.modify_bits(|r| r | Self::ODR7_MASK);
    }

    /// Clear ODR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr7(&self) {
        self.value.modify_bits(|r| r & !Self::ODR7_MASK);
    }

    /// **ODR6** `[6]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr6(&self) -> bool {
        (self.value.read_bits() & Self::ODR6_MASK) != 0
    }

    /// Set ODR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr6(&self) {
        self.value.modify_bits(|r| r | Self::ODR6_MASK);
    }

    /// Clear ODR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr6(&self) {
        self.value.modify_bits(|r| r & !Self::ODR6_MASK);
    }

    /// **ODR5** `[5]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr5(&self) -> bool {
        (self.value.read_bits() & Self::ODR5_MASK) != 0
    }

    /// Set ODR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr5(&self) {
        self.value.modify_bits(|r| r | Self::ODR5_MASK);
    }

    /// Clear ODR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr5(&self) {
        self.value.modify_bits(|r| r & !Self::ODR5_MASK);
    }

    /// **ODR4** `[4]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr4(&self) -> bool {
        (self.value.read_bits() & Self::ODR4_MASK) != 0
    }

    /// Set ODR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr4(&self) {
        self.value.modify_bits(|r| r | Self::ODR4_MASK);
    }

    /// Clear ODR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr4(&self) {
        self.value.modify_bits(|r| r & !Self::ODR4_MASK);
    }

    /// **ODR3** `[3]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr3(&self) -> bool {
        (self.value.read_bits() & Self::ODR3_MASK) != 0
    }

    /// Set ODR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr3(&self) {
        self.value.modify_bits(|r| r | Self::ODR3_MASK);
    }

    /// Clear ODR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr3(&self) {
        self.value.modify_bits(|r| r & !Self::ODR3_MASK);
    }

    /// **ODR2** `[2]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr2(&self) -> bool {
        (self.value.read_bits() & Self::ODR2_MASK) != 0
    }

    /// Set ODR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr2(&self) {
        self.value.modify_bits(|r| r | Self::ODR2_MASK);
    }

    /// Clear ODR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr2(&self) {
        self.value.modify_bits(|r| r & !Self::ODR2_MASK);
    }

    /// **ODR1** `[1]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr1(&self) -> bool {
        (self.value.read_bits() & Self::ODR1_MASK) != 0
    }

    /// Set ODR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr1(&self) {
        self.value.modify_bits(|r| r | Self::ODR1_MASK);
    }

    /// Clear ODR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr1(&self) {
        self.value.modify_bits(|r| r & !Self::ODR1_MASK);
    }

    /// **ODR0** `[0]` (RW)
    ///
    /// Port output data
    #[inline(always)]
    pub fn odr0(&self) -> bool {
        (self.value.read_bits() & Self::ODR0_MASK) != 0
    }

    /// Set ODR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_odr0(&self) {
        self.value.modify_bits(|r| r | Self::ODR0_MASK);
    }

    /// Clear ODR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_odr0(&self) {
        self.value.modify_bits(|r| r & !Self::ODR0_MASK);
    }
}

/// OUTDR value type for batch read/modify/write operations
///
/// Use `outdr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outdr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutdrVal {
    bits: u32,
}

impl OutdrVal {
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

    /// Read **ODR23** `[23:23]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr23(&self) -> bool {
        (self.bits & Outdr::ODR23_MASK) != 0
    }

    /// Read **ODR22** `[22:22]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr22(&self) -> bool {
        (self.bits & Outdr::ODR22_MASK) != 0
    }

    /// Read **ODR21** `[21:21]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr21(&self) -> bool {
        (self.bits & Outdr::ODR21_MASK) != 0
    }

    /// Read **ODR20** `[20:20]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr20(&self) -> bool {
        (self.bits & Outdr::ODR20_MASK) != 0
    }

    /// Read **ODR19** `[19:19]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr19(&self) -> bool {
        (self.bits & Outdr::ODR19_MASK) != 0
    }

    /// Read **ODR18** `[18:18]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr18(&self) -> bool {
        (self.bits & Outdr::ODR18_MASK) != 0
    }

    /// Read **ODR17** `[17:17]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr17(&self) -> bool {
        (self.bits & Outdr::ODR17_MASK) != 0
    }

    /// Read **ODR16** `[16:16]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr16(&self) -> bool {
        (self.bits & Outdr::ODR16_MASK) != 0
    }

    /// Read **ODR15** `[15:15]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr15(&self) -> bool {
        (self.bits & Outdr::ODR15_MASK) != 0
    }

    /// Read **ODR14** `[14:14]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr14(&self) -> bool {
        (self.bits & Outdr::ODR14_MASK) != 0
    }

    /// Read **ODR13** `[13:13]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr13(&self) -> bool {
        (self.bits & Outdr::ODR13_MASK) != 0
    }

    /// Read **ODR12** `[12:12]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr12(&self) -> bool {
        (self.bits & Outdr::ODR12_MASK) != 0
    }

    /// Read **ODR11** `[11:11]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr11(&self) -> bool {
        (self.bits & Outdr::ODR11_MASK) != 0
    }

    /// Read **ODR10** `[10:10]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr10(&self) -> bool {
        (self.bits & Outdr::ODR10_MASK) != 0
    }

    /// Read **ODR9** `[9:9]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr9(&self) -> bool {
        (self.bits & Outdr::ODR9_MASK) != 0
    }

    /// Read **ODR8** `[8:8]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr8(&self) -> bool {
        (self.bits & Outdr::ODR8_MASK) != 0
    }

    /// Read **ODR7** `[7:7]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr7(&self) -> bool {
        (self.bits & Outdr::ODR7_MASK) != 0
    }

    /// Read **ODR6** `[6:6]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr6(&self) -> bool {
        (self.bits & Outdr::ODR6_MASK) != 0
    }

    /// Read **ODR5** `[5:5]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr5(&self) -> bool {
        (self.bits & Outdr::ODR5_MASK) != 0
    }

    /// Read **ODR4** `[4:4]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr4(&self) -> bool {
        (self.bits & Outdr::ODR4_MASK) != 0
    }

    /// Read **ODR3** `[3:3]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr3(&self) -> bool {
        (self.bits & Outdr::ODR3_MASK) != 0
    }

    /// Read **ODR2** `[2:2]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr2(&self) -> bool {
        (self.bits & Outdr::ODR2_MASK) != 0
    }

    /// Read **ODR1** `[1:1]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr1(&self) -> bool {
        (self.bits & Outdr::ODR1_MASK) != 0
    }

    /// Read **ODR0** `[0:0]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn odr0(&self) -> bool {
        (self.bits & Outdr::ODR0_MASK) != 0
    }

    /// Set **ODR23** `[23:23]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR23_MASK
            } else {
                self.bits & !Outdr::ODR23_MASK
            },
        }
    }

    /// Set **ODR22** `[22:22]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR22_MASK
            } else {
                self.bits & !Outdr::ODR22_MASK
            },
        }
    }

    /// Set **ODR21** `[21:21]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR21_MASK
            } else {
                self.bits & !Outdr::ODR21_MASK
            },
        }
    }

    /// Set **ODR20** `[20:20]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR20_MASK
            } else {
                self.bits & !Outdr::ODR20_MASK
            },
        }
    }

    /// Set **ODR19** `[19:19]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR19_MASK
            } else {
                self.bits & !Outdr::ODR19_MASK
            },
        }
    }

    /// Set **ODR18** `[18:18]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR18_MASK
            } else {
                self.bits & !Outdr::ODR18_MASK
            },
        }
    }

    /// Set **ODR17** `[17:17]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR17_MASK
            } else {
                self.bits & !Outdr::ODR17_MASK
            },
        }
    }

    /// Set **ODR16** `[16:16]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR16_MASK
            } else {
                self.bits & !Outdr::ODR16_MASK
            },
        }
    }

    /// Set **ODR15** `[15:15]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR15_MASK
            } else {
                self.bits & !Outdr::ODR15_MASK
            },
        }
    }

    /// Set **ODR14** `[14:14]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR14_MASK
            } else {
                self.bits & !Outdr::ODR14_MASK
            },
        }
    }

    /// Set **ODR13** `[13:13]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR13_MASK
            } else {
                self.bits & !Outdr::ODR13_MASK
            },
        }
    }

    /// Set **ODR12** `[12:12]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR12_MASK
            } else {
                self.bits & !Outdr::ODR12_MASK
            },
        }
    }

    /// Set **ODR11** `[11:11]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR11_MASK
            } else {
                self.bits & !Outdr::ODR11_MASK
            },
        }
    }

    /// Set **ODR10** `[10:10]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR10_MASK
            } else {
                self.bits & !Outdr::ODR10_MASK
            },
        }
    }

    /// Set **ODR9** `[9:9]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR9_MASK
            } else {
                self.bits & !Outdr::ODR9_MASK
            },
        }
    }

    /// Set **ODR8** `[8:8]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR8_MASK
            } else {
                self.bits & !Outdr::ODR8_MASK
            },
        }
    }

    /// Set **ODR7** `[7:7]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR7_MASK
            } else {
                self.bits & !Outdr::ODR7_MASK
            },
        }
    }

    /// Set **ODR6** `[6:6]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR6_MASK
            } else {
                self.bits & !Outdr::ODR6_MASK
            },
        }
    }

    /// Set **ODR5** `[5:5]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR5_MASK
            } else {
                self.bits & !Outdr::ODR5_MASK
            },
        }
    }

    /// Set **ODR4** `[4:4]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR4_MASK
            } else {
                self.bits & !Outdr::ODR4_MASK
            },
        }
    }

    /// Set **ODR3** `[3:3]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR3_MASK
            } else {
                self.bits & !Outdr::ODR3_MASK
            },
        }
    }

    /// Set **ODR2** `[2:2]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR2_MASK
            } else {
                self.bits & !Outdr::ODR2_MASK
            },
        }
    }

    /// Set **ODR1** `[1:1]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR1_MASK
            } else {
                self.bits & !Outdr::ODR1_MASK
            },
        }
    }

    /// Set **ODR0** `[0:0]`
    ///
    /// Port output data
    #[inline(always)]
    pub const fn with_odr0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Outdr::ODR0_MASK
            } else {
                self.bits & !Outdr::ODR0_MASK
            },
        }
    }
}

impl Default for OutdrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Outdr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutdrVal {
        OutdrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutdrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutdrVal) -> OutdrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BSHR Register
///
/// Port bit set/reset register
/// (GPIOn_BSHR)
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BR15** `[31]` (WO)
///
///   Reset bit 15
///
/// - **BR14** `[30]` (WO)
///
///   Reset bit 14
///
/// - **BR13** `[29]` (WO)
///
///   Reset bit 13
///
/// - **BR12** `[28]` (WO)
///
///   Reset bit 12
///
/// - **BR11** `[27]` (WO)
///
///   Reset bit 11
///
/// - **BR10** `[26]` (WO)
///
///   Reset bit 10
///
/// - **BR9** `[25]` (WO)
///
///   Reset bit 9
///
/// - **BR8** `[24]` (WO)
///
///   Reset bit 8
///
/// - **BR7** `[23]` (WO)
///
///   Reset bit 7
///
/// - **BR6** `[22]` (WO)
///
///   Reset bit 6
///
/// - **BR5** `[21]` (WO)
///
///   Reset bit 5
///
/// - **BR4** `[20]` (WO)
///
///   Reset bit 4
///
/// - **BR3** `[19]` (WO)
///
///   Reset bit 3
///
/// - **BR2** `[18]` (WO)
///
///   Reset bit 2
///
/// - **BR1** `[17]` (WO)
///
///   Reset bit 1
///
/// - **BR0** `[16]` (WO)
///
///   Reset bit 0
///
/// - **BS15** `[15]` (WO)
///
///   Set bit 15
///
/// - **BS14** `[14]` (WO)
///
///   Set bit 14
///
/// - **BS13** `[13]` (WO)
///
///   Set bit 13
///
/// - **BS12** `[12]` (WO)
///
///   Set bit 12
///
/// - **BS11** `[11]` (WO)
///
///   Set bit 11
///
/// - **BS10** `[10]` (WO)
///
///   Set bit 10
///
/// - **BS9** `[9]` (WO)
///
///   Set bit 9
///
/// - **BS8** `[8]` (WO)
///
///   Set bit 8
///
/// - **BS7** `[7]` (WO)
///
///   Set bit 7
///
/// - **BS6** `[6]` (WO)
///
///   Set bit 6
///
/// - **BS5** `[5]` (WO)
///
///   Set bit 5
///
/// - **BS4** `[4]` (WO)
///
///   Set bit 4
///
/// - **BS3** `[3]` (WO)
///
///   Set bit 3
///
/// - **BS2** `[2]` (WO)
///
///   Set bit 2
///
/// - **BS1** `[1]` (WO)
///
///   Set bit 1
///
/// - **BS0** `[0]` (WO)
///
///   Set bit 0
#[repr(transparent)]
pub struct Bshr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bshr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bshr {
    // ======== Field Constants ========

    /// **BR15**
    ///
    /// Reset bit 15
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const BR15_OFFSET: u32 = 31;
    pub const BR15_MASK: u32 = 0x80000000;
    pub const BR15: u32 = 1 << 31;

    /// **BR14**
    ///
    /// Reset bit 14
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const BR14_OFFSET: u32 = 30;
    pub const BR14_MASK: u32 = 0x40000000;
    pub const BR14: u32 = 1 << 30;

    /// **BR13**
    ///
    /// Reset bit 13
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const BR13_OFFSET: u32 = 29;
    pub const BR13_MASK: u32 = 0x20000000;
    pub const BR13: u32 = 1 << 29;

    /// **BR12**
    ///
    /// Reset bit 12
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const BR12_OFFSET: u32 = 28;
    pub const BR12_MASK: u32 = 0x10000000;
    pub const BR12: u32 = 1 << 28;

    /// **BR11**
    ///
    /// Reset bit 11
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const BR11_OFFSET: u32 = 27;
    pub const BR11_MASK: u32 = 0x08000000;
    pub const BR11: u32 = 1 << 27;

    /// **BR10**
    ///
    /// Reset bit 10
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const BR10_OFFSET: u32 = 26;
    pub const BR10_MASK: u32 = 0x04000000;
    pub const BR10: u32 = 1 << 26;

    /// **BR9**
    ///
    /// Reset bit 9
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const BR9_OFFSET: u32 = 25;
    pub const BR9_MASK: u32 = 0x02000000;
    pub const BR9: u32 = 1 << 25;

    /// **BR8**
    ///
    /// Reset bit 8
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const BR8_OFFSET: u32 = 24;
    pub const BR8_MASK: u32 = 0x01000000;
    pub const BR8: u32 = 1 << 24;

    /// **BR7**
    ///
    /// Reset bit 7
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const BR7_OFFSET: u32 = 23;
    pub const BR7_MASK: u32 = 0x00800000;
    pub const BR7: u32 = 1 << 23;

    /// **BR6**
    ///
    /// Reset bit 6
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const BR6_OFFSET: u32 = 22;
    pub const BR6_MASK: u32 = 0x00400000;
    pub const BR6: u32 = 1 << 22;

    /// **BR5**
    ///
    /// Reset bit 5
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const BR5_OFFSET: u32 = 21;
    pub const BR5_MASK: u32 = 0x00200000;
    pub const BR5: u32 = 1 << 21;

    /// **BR4**
    ///
    /// Reset bit 4
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const BR4_OFFSET: u32 = 20;
    pub const BR4_MASK: u32 = 0x00100000;
    pub const BR4: u32 = 1 << 20;

    /// **BR3**
    ///
    /// Reset bit 3
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const BR3_OFFSET: u32 = 19;
    pub const BR3_MASK: u32 = 0x00080000;
    pub const BR3: u32 = 1 << 19;

    /// **BR2**
    ///
    /// Reset bit 2
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const BR2_OFFSET: u32 = 18;
    pub const BR2_MASK: u32 = 0x00040000;
    pub const BR2: u32 = 1 << 18;

    /// **BR1**
    ///
    /// Reset bit 1
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const BR1_OFFSET: u32 = 17;
    pub const BR1_MASK: u32 = 0x00020000;
    pub const BR1: u32 = 1 << 17;

    /// **BR0**
    ///
    /// Reset bit 0
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const BR0_OFFSET: u32 = 16;
    pub const BR0_MASK: u32 = 0x00010000;
    pub const BR0: u32 = 1 << 16;

    /// **BS15**
    ///
    /// Set bit 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const BS15_OFFSET: u32 = 15;
    pub const BS15_MASK: u32 = 0x00008000;
    pub const BS15: u32 = 1 << 15;

    /// **BS14**
    ///
    /// Set bit 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const BS14_OFFSET: u32 = 14;
    pub const BS14_MASK: u32 = 0x00004000;
    pub const BS14: u32 = 1 << 14;

    /// **BS13**
    ///
    /// Set bit 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const BS13_OFFSET: u32 = 13;
    pub const BS13_MASK: u32 = 0x00002000;
    pub const BS13: u32 = 1 << 13;

    /// **BS12**
    ///
    /// Set bit 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BS12_OFFSET: u32 = 12;
    pub const BS12_MASK: u32 = 0x00001000;
    pub const BS12: u32 = 1 << 12;

    /// **BS11**
    ///
    /// Set bit 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const BS11_OFFSET: u32 = 11;
    pub const BS11_MASK: u32 = 0x00000800;
    pub const BS11: u32 = 1 << 11;

    /// **BS10**
    ///
    /// Set bit 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const BS10_OFFSET: u32 = 10;
    pub const BS10_MASK: u32 = 0x00000400;
    pub const BS10: u32 = 1 << 10;

    /// **BS9**
    ///
    /// Set bit 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BS9_OFFSET: u32 = 9;
    pub const BS9_MASK: u32 = 0x00000200;
    pub const BS9: u32 = 1 << 9;

    /// **BS8**
    ///
    /// Set bit 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const BS8_OFFSET: u32 = 8;
    pub const BS8_MASK: u32 = 0x00000100;
    pub const BS8: u32 = 1 << 8;

    /// **BS7**
    ///
    /// Set bit 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BS7_OFFSET: u32 = 7;
    pub const BS7_MASK: u32 = 0x00000080;
    pub const BS7: u32 = 1 << 7;

    /// **BS6**
    ///
    /// Set bit 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const BS6_OFFSET: u32 = 6;
    pub const BS6_MASK: u32 = 0x00000040;
    pub const BS6: u32 = 1 << 6;

    /// **BS5**
    ///
    /// Set bit 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const BS5_OFFSET: u32 = 5;
    pub const BS5_MASK: u32 = 0x00000020;
    pub const BS5: u32 = 1 << 5;

    /// **BS4**
    ///
    /// Set bit 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BS4_OFFSET: u32 = 4;
    pub const BS4_MASK: u32 = 0x00000010;
    pub const BS4: u32 = 1 << 4;

    /// **BS3**
    ///
    /// Set bit 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BS3_OFFSET: u32 = 3;
    pub const BS3_MASK: u32 = 0x00000008;
    pub const BS3: u32 = 1 << 3;

    /// **BS2**
    ///
    /// Set bit 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BS2_OFFSET: u32 = 2;
    pub const BS2_MASK: u32 = 0x00000004;
    pub const BS2: u32 = 1 << 2;

    /// **BS1**
    ///
    /// Set bit 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BS1_OFFSET: u32 = 1;
    pub const BS1_MASK: u32 = 0x00000002;
    pub const BS1: u32 = 1 << 1;

    /// **BS0**
    ///
    /// Set bit 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BS0_OFFSET: u32 = 0;
    pub const BS0_MASK: u32 = 0x00000001;
    pub const BS0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **BR15** (WO)
    ///
    /// Reset bit 15
    #[inline(always)]
    pub fn trigger_br15(&self) {
        self.value.write_bits(Self::BR15_MASK);
    }

    /// Trigger **BR14** (WO)
    ///
    /// Reset bit 14
    #[inline(always)]
    pub fn trigger_br14(&self) {
        self.value.write_bits(Self::BR14_MASK);
    }

    /// Trigger **BR13** (WO)
    ///
    /// Reset bit 13
    #[inline(always)]
    pub fn trigger_br13(&self) {
        self.value.write_bits(Self::BR13_MASK);
    }

    /// Trigger **BR12** (WO)
    ///
    /// Reset bit 12
    #[inline(always)]
    pub fn trigger_br12(&self) {
        self.value.write_bits(Self::BR12_MASK);
    }

    /// Trigger **BR11** (WO)
    ///
    /// Reset bit 11
    #[inline(always)]
    pub fn trigger_br11(&self) {
        self.value.write_bits(Self::BR11_MASK);
    }

    /// Trigger **BR10** (WO)
    ///
    /// Reset bit 10
    #[inline(always)]
    pub fn trigger_br10(&self) {
        self.value.write_bits(Self::BR10_MASK);
    }

    /// Trigger **BR9** (WO)
    ///
    /// Reset bit 9
    #[inline(always)]
    pub fn trigger_br9(&self) {
        self.value.write_bits(Self::BR9_MASK);
    }

    /// Trigger **BR8** (WO)
    ///
    /// Reset bit 8
    #[inline(always)]
    pub fn trigger_br8(&self) {
        self.value.write_bits(Self::BR8_MASK);
    }

    /// Trigger **BR7** (WO)
    ///
    /// Reset bit 7
    #[inline(always)]
    pub fn trigger_br7(&self) {
        self.value.write_bits(Self::BR7_MASK);
    }

    /// Trigger **BR6** (WO)
    ///
    /// Reset bit 6
    #[inline(always)]
    pub fn trigger_br6(&self) {
        self.value.write_bits(Self::BR6_MASK);
    }

    /// Trigger **BR5** (WO)
    ///
    /// Reset bit 5
    #[inline(always)]
    pub fn trigger_br5(&self) {
        self.value.write_bits(Self::BR5_MASK);
    }

    /// Trigger **BR4** (WO)
    ///
    /// Reset bit 4
    #[inline(always)]
    pub fn trigger_br4(&self) {
        self.value.write_bits(Self::BR4_MASK);
    }

    /// Trigger **BR3** (WO)
    ///
    /// Reset bit 3
    #[inline(always)]
    pub fn trigger_br3(&self) {
        self.value.write_bits(Self::BR3_MASK);
    }

    /// Trigger **BR2** (WO)
    ///
    /// Reset bit 2
    #[inline(always)]
    pub fn trigger_br2(&self) {
        self.value.write_bits(Self::BR2_MASK);
    }

    /// Trigger **BR1** (WO)
    ///
    /// Reset bit 1
    #[inline(always)]
    pub fn trigger_br1(&self) {
        self.value.write_bits(Self::BR1_MASK);
    }

    /// Trigger **BR0** (WO)
    ///
    /// Reset bit 0
    #[inline(always)]
    pub fn trigger_br0(&self) {
        self.value.write_bits(Self::BR0_MASK);
    }

    /// Trigger **BS15** (WO)
    ///
    /// Set bit 15
    #[inline(always)]
    pub fn trigger_bs15(&self) {
        self.value.write_bits(Self::BS15_MASK);
    }

    /// Trigger **BS14** (WO)
    ///
    /// Set bit 14
    #[inline(always)]
    pub fn trigger_bs14(&self) {
        self.value.write_bits(Self::BS14_MASK);
    }

    /// Trigger **BS13** (WO)
    ///
    /// Set bit 13
    #[inline(always)]
    pub fn trigger_bs13(&self) {
        self.value.write_bits(Self::BS13_MASK);
    }

    /// Trigger **BS12** (WO)
    ///
    /// Set bit 12
    #[inline(always)]
    pub fn trigger_bs12(&self) {
        self.value.write_bits(Self::BS12_MASK);
    }

    /// Trigger **BS11** (WO)
    ///
    /// Set bit 11
    #[inline(always)]
    pub fn trigger_bs11(&self) {
        self.value.write_bits(Self::BS11_MASK);
    }

    /// Trigger **BS10** (WO)
    ///
    /// Set bit 10
    #[inline(always)]
    pub fn trigger_bs10(&self) {
        self.value.write_bits(Self::BS10_MASK);
    }

    /// Trigger **BS9** (WO)
    ///
    /// Set bit 9
    #[inline(always)]
    pub fn trigger_bs9(&self) {
        self.value.write_bits(Self::BS9_MASK);
    }

    /// Trigger **BS8** (WO)
    ///
    /// Set bit 8
    #[inline(always)]
    pub fn trigger_bs8(&self) {
        self.value.write_bits(Self::BS8_MASK);
    }

    /// Trigger **BS7** (WO)
    ///
    /// Set bit 7
    #[inline(always)]
    pub fn trigger_bs7(&self) {
        self.value.write_bits(Self::BS7_MASK);
    }

    /// Trigger **BS6** (WO)
    ///
    /// Set bit 6
    #[inline(always)]
    pub fn trigger_bs6(&self) {
        self.value.write_bits(Self::BS6_MASK);
    }

    /// Trigger **BS5** (WO)
    ///
    /// Set bit 5
    #[inline(always)]
    pub fn trigger_bs5(&self) {
        self.value.write_bits(Self::BS5_MASK);
    }

    /// Trigger **BS4** (WO)
    ///
    /// Set bit 4
    #[inline(always)]
    pub fn trigger_bs4(&self) {
        self.value.write_bits(Self::BS4_MASK);
    }

    /// Trigger **BS3** (WO)
    ///
    /// Set bit 3
    #[inline(always)]
    pub fn trigger_bs3(&self) {
        self.value.write_bits(Self::BS3_MASK);
    }

    /// Trigger **BS2** (WO)
    ///
    /// Set bit 2
    #[inline(always)]
    pub fn trigger_bs2(&self) {
        self.value.write_bits(Self::BS2_MASK);
    }

    /// Trigger **BS1** (WO)
    ///
    /// Set bit 1
    #[inline(always)]
    pub fn trigger_bs1(&self) {
        self.value.write_bits(Self::BS1_MASK);
    }

    /// Trigger **BS0** (WO)
    ///
    /// Set bit 0
    #[inline(always)]
    pub fn trigger_bs0(&self) {
        self.value.write_bits(Self::BS0_MASK);
    }
}

/// BSHR value type for batch read/modify/write operations
///
/// Use `bshr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bshr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BshrVal {
    bits: u32,
}

impl BshrVal {
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

    /// Set **BR15** `[31:31]`
    ///
    /// Reset bit 15
    #[inline(always)]
    pub const fn with_br15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR15_MASK
            } else {
                self.bits & !Bshr::BR15_MASK
            },
        }
    }

    /// Set **BR14** `[30:30]`
    ///
    /// Reset bit 14
    #[inline(always)]
    pub const fn with_br14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR14_MASK
            } else {
                self.bits & !Bshr::BR14_MASK
            },
        }
    }

    /// Set **BR13** `[29:29]`
    ///
    /// Reset bit 13
    #[inline(always)]
    pub const fn with_br13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR13_MASK
            } else {
                self.bits & !Bshr::BR13_MASK
            },
        }
    }

    /// Set **BR12** `[28:28]`
    ///
    /// Reset bit 12
    #[inline(always)]
    pub const fn with_br12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR12_MASK
            } else {
                self.bits & !Bshr::BR12_MASK
            },
        }
    }

    /// Set **BR11** `[27:27]`
    ///
    /// Reset bit 11
    #[inline(always)]
    pub const fn with_br11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR11_MASK
            } else {
                self.bits & !Bshr::BR11_MASK
            },
        }
    }

    /// Set **BR10** `[26:26]`
    ///
    /// Reset bit 10
    #[inline(always)]
    pub const fn with_br10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR10_MASK
            } else {
                self.bits & !Bshr::BR10_MASK
            },
        }
    }

    /// Set **BR9** `[25:25]`
    ///
    /// Reset bit 9
    #[inline(always)]
    pub const fn with_br9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR9_MASK
            } else {
                self.bits & !Bshr::BR9_MASK
            },
        }
    }

    /// Set **BR8** `[24:24]`
    ///
    /// Reset bit 8
    #[inline(always)]
    pub const fn with_br8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR8_MASK
            } else {
                self.bits & !Bshr::BR8_MASK
            },
        }
    }

    /// Set **BR7** `[23:23]`
    ///
    /// Reset bit 7
    #[inline(always)]
    pub const fn with_br7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR7_MASK
            } else {
                self.bits & !Bshr::BR7_MASK
            },
        }
    }

    /// Set **BR6** `[22:22]`
    ///
    /// Reset bit 6
    #[inline(always)]
    pub const fn with_br6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR6_MASK
            } else {
                self.bits & !Bshr::BR6_MASK
            },
        }
    }

    /// Set **BR5** `[21:21]`
    ///
    /// Reset bit 5
    #[inline(always)]
    pub const fn with_br5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR5_MASK
            } else {
                self.bits & !Bshr::BR5_MASK
            },
        }
    }

    /// Set **BR4** `[20:20]`
    ///
    /// Reset bit 4
    #[inline(always)]
    pub const fn with_br4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR4_MASK
            } else {
                self.bits & !Bshr::BR4_MASK
            },
        }
    }

    /// Set **BR3** `[19:19]`
    ///
    /// Reset bit 3
    #[inline(always)]
    pub const fn with_br3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR3_MASK
            } else {
                self.bits & !Bshr::BR3_MASK
            },
        }
    }

    /// Set **BR2** `[18:18]`
    ///
    /// Reset bit 2
    #[inline(always)]
    pub const fn with_br2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR2_MASK
            } else {
                self.bits & !Bshr::BR2_MASK
            },
        }
    }

    /// Set **BR1** `[17:17]`
    ///
    /// Reset bit 1
    #[inline(always)]
    pub const fn with_br1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR1_MASK
            } else {
                self.bits & !Bshr::BR1_MASK
            },
        }
    }

    /// Set **BR0** `[16:16]`
    ///
    /// Reset bit 0
    #[inline(always)]
    pub const fn with_br0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BR0_MASK
            } else {
                self.bits & !Bshr::BR0_MASK
            },
        }
    }

    /// Set **BS15** `[15:15]`
    ///
    /// Set bit 15
    #[inline(always)]
    pub const fn with_bs15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS15_MASK
            } else {
                self.bits & !Bshr::BS15_MASK
            },
        }
    }

    /// Set **BS14** `[14:14]`
    ///
    /// Set bit 14
    #[inline(always)]
    pub const fn with_bs14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS14_MASK
            } else {
                self.bits & !Bshr::BS14_MASK
            },
        }
    }

    /// Set **BS13** `[13:13]`
    ///
    /// Set bit 13
    #[inline(always)]
    pub const fn with_bs13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS13_MASK
            } else {
                self.bits & !Bshr::BS13_MASK
            },
        }
    }

    /// Set **BS12** `[12:12]`
    ///
    /// Set bit 12
    #[inline(always)]
    pub const fn with_bs12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS12_MASK
            } else {
                self.bits & !Bshr::BS12_MASK
            },
        }
    }

    /// Set **BS11** `[11:11]`
    ///
    /// Set bit 11
    #[inline(always)]
    pub const fn with_bs11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS11_MASK
            } else {
                self.bits & !Bshr::BS11_MASK
            },
        }
    }

    /// Set **BS10** `[10:10]`
    ///
    /// Set bit 10
    #[inline(always)]
    pub const fn with_bs10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS10_MASK
            } else {
                self.bits & !Bshr::BS10_MASK
            },
        }
    }

    /// Set **BS9** `[9:9]`
    ///
    /// Set bit 9
    #[inline(always)]
    pub const fn with_bs9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS9_MASK
            } else {
                self.bits & !Bshr::BS9_MASK
            },
        }
    }

    /// Set **BS8** `[8:8]`
    ///
    /// Set bit 8
    #[inline(always)]
    pub const fn with_bs8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS8_MASK
            } else {
                self.bits & !Bshr::BS8_MASK
            },
        }
    }

    /// Set **BS7** `[7:7]`
    ///
    /// Set bit 7
    #[inline(always)]
    pub const fn with_bs7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS7_MASK
            } else {
                self.bits & !Bshr::BS7_MASK
            },
        }
    }

    /// Set **BS6** `[6:6]`
    ///
    /// Set bit 6
    #[inline(always)]
    pub const fn with_bs6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS6_MASK
            } else {
                self.bits & !Bshr::BS6_MASK
            },
        }
    }

    /// Set **BS5** `[5:5]`
    ///
    /// Set bit 5
    #[inline(always)]
    pub const fn with_bs5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS5_MASK
            } else {
                self.bits & !Bshr::BS5_MASK
            },
        }
    }

    /// Set **BS4** `[4:4]`
    ///
    /// Set bit 4
    #[inline(always)]
    pub const fn with_bs4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS4_MASK
            } else {
                self.bits & !Bshr::BS4_MASK
            },
        }
    }

    /// Set **BS3** `[3:3]`
    ///
    /// Set bit 3
    #[inline(always)]
    pub const fn with_bs3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS3_MASK
            } else {
                self.bits & !Bshr::BS3_MASK
            },
        }
    }

    /// Set **BS2** `[2:2]`
    ///
    /// Set bit 2
    #[inline(always)]
    pub const fn with_bs2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS2_MASK
            } else {
                self.bits & !Bshr::BS2_MASK
            },
        }
    }

    /// Set **BS1** `[1:1]`
    ///
    /// Set bit 1
    #[inline(always)]
    pub const fn with_bs1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS1_MASK
            } else {
                self.bits & !Bshr::BS1_MASK
            },
        }
    }

    /// Set **BS0** `[0:0]`
    ///
    /// Set bit 0
    #[inline(always)]
    pub const fn with_bs0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bshr::BS0_MASK
            } else {
                self.bits & !Bshr::BS0_MASK
            },
        }
    }
}

impl Default for BshrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bshr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BshrVal) {
        self.value.write_bits(val.bits())
    }
}

/// BCR Register
///
/// Port bit reset register
/// (GPIOn_BCR)
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BR23** `[23]` (WO)
///
///   Reset bit 23
///
/// - **BR22** `[22]` (WO)
///
///   Reset bit 22
///
/// - **BR21** `[21]` (WO)
///
///   Reset bit 21
///
/// - **BR20** `[20]` (WO)
///
///   Reset bit 20
///
/// - **BR19** `[19]` (WO)
///
///   Reset bit 19
///
/// - **BR18** `[18]` (WO)
///
///   Reset bit 18
///
/// - **BR17** `[17]` (WO)
///
///   Reset bit 17
///
/// - **BR16** `[16]` (WO)
///
///   Reset bit 16
///
/// - **BR15** `[15]` (WO)
///
///   Reset bit 15
///
/// - **BR14** `[14]` (WO)
///
///   Reset bit 14
///
/// - **BR13** `[13]` (WO)
///
///   Reset bit 13
///
/// - **BR12** `[12]` (WO)
///
///   Reset bit 12
///
/// - **BR11** `[11]` (WO)
///
///   Reset bit 11
///
/// - **BR10** `[10]` (WO)
///
///   Reset bit 10
///
/// - **BR9** `[9]` (WO)
///
///   Reset bit 9
///
/// - **BR8** `[8]` (WO)
///
///   Reset bit 8
///
/// - **BR7** `[7]` (WO)
///
///   Reset bit 7
///
/// - **BR6** `[6]` (WO)
///
///   Reset bit 6
///
/// - **BR5** `[5]` (WO)
///
///   Reset bit 5
///
/// - **BR4** `[4]` (WO)
///
///   Reset bit 4
///
/// - **BR3** `[3]` (WO)
///
///   Reset bit 3
///
/// - **BR2** `[2]` (WO)
///
///   Reset bit 1
///
/// - **BR1** `[1]` (WO)
///
///   Reset bit 1
///
/// - **BR0** `[0]` (WO)
///
///   Reset bit 0
#[repr(transparent)]
pub struct Bcr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bcr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bcr {
    // ======== Field Constants ========

    /// **BR23**
    ///
    /// Reset bit 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const BR23_OFFSET: u32 = 23;
    pub const BR23_MASK: u32 = 0x00800000;
    pub const BR23: u32 = 1 << 23;

    /// **BR22**
    ///
    /// Reset bit 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const BR22_OFFSET: u32 = 22;
    pub const BR22_MASK: u32 = 0x00400000;
    pub const BR22: u32 = 1 << 22;

    /// **BR21**
    ///
    /// Reset bit 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const BR21_OFFSET: u32 = 21;
    pub const BR21_MASK: u32 = 0x00200000;
    pub const BR21: u32 = 1 << 21;

    /// **BR20**
    ///
    /// Reset bit 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const BR20_OFFSET: u32 = 20;
    pub const BR20_MASK: u32 = 0x00100000;
    pub const BR20: u32 = 1 << 20;

    /// **BR19**
    ///
    /// Reset bit 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const BR19_OFFSET: u32 = 19;
    pub const BR19_MASK: u32 = 0x00080000;
    pub const BR19: u32 = 1 << 19;

    /// **BR18**
    ///
    /// Reset bit 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const BR18_OFFSET: u32 = 18;
    pub const BR18_MASK: u32 = 0x00040000;
    pub const BR18: u32 = 1 << 18;

    /// **BR17**
    ///
    /// Reset bit 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const BR17_OFFSET: u32 = 17;
    pub const BR17_MASK: u32 = 0x00020000;
    pub const BR17: u32 = 1 << 17;

    /// **BR16**
    ///
    /// Reset bit 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const BR16_OFFSET: u32 = 16;
    pub const BR16_MASK: u32 = 0x00010000;
    pub const BR16: u32 = 1 << 16;

    /// **BR15**
    ///
    /// Reset bit 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const BR15_OFFSET: u32 = 15;
    pub const BR15_MASK: u32 = 0x00008000;
    pub const BR15: u32 = 1 << 15;

    /// **BR14**
    ///
    /// Reset bit 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const BR14_OFFSET: u32 = 14;
    pub const BR14_MASK: u32 = 0x00004000;
    pub const BR14: u32 = 1 << 14;

    /// **BR13**
    ///
    /// Reset bit 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const BR13_OFFSET: u32 = 13;
    pub const BR13_MASK: u32 = 0x00002000;
    pub const BR13: u32 = 1 << 13;

    /// **BR12**
    ///
    /// Reset bit 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BR12_OFFSET: u32 = 12;
    pub const BR12_MASK: u32 = 0x00001000;
    pub const BR12: u32 = 1 << 12;

    /// **BR11**
    ///
    /// Reset bit 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const BR11_OFFSET: u32 = 11;
    pub const BR11_MASK: u32 = 0x00000800;
    pub const BR11: u32 = 1 << 11;

    /// **BR10**
    ///
    /// Reset bit 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const BR10_OFFSET: u32 = 10;
    pub const BR10_MASK: u32 = 0x00000400;
    pub const BR10: u32 = 1 << 10;

    /// **BR9**
    ///
    /// Reset bit 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const BR9_OFFSET: u32 = 9;
    pub const BR9_MASK: u32 = 0x00000200;
    pub const BR9: u32 = 1 << 9;

    /// **BR8**
    ///
    /// Reset bit 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const BR8_OFFSET: u32 = 8;
    pub const BR8_MASK: u32 = 0x00000100;
    pub const BR8: u32 = 1 << 8;

    /// **BR7**
    ///
    /// Reset bit 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BR7_OFFSET: u32 = 7;
    pub const BR7_MASK: u32 = 0x00000080;
    pub const BR7: u32 = 1 << 7;

    /// **BR6**
    ///
    /// Reset bit 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const BR6_OFFSET: u32 = 6;
    pub const BR6_MASK: u32 = 0x00000040;
    pub const BR6: u32 = 1 << 6;

    /// **BR5**
    ///
    /// Reset bit 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const BR5_OFFSET: u32 = 5;
    pub const BR5_MASK: u32 = 0x00000020;
    pub const BR5: u32 = 1 << 5;

    /// **BR4**
    ///
    /// Reset bit 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BR4_OFFSET: u32 = 4;
    pub const BR4_MASK: u32 = 0x00000010;
    pub const BR4: u32 = 1 << 4;

    /// **BR3**
    ///
    /// Reset bit 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BR3_OFFSET: u32 = 3;
    pub const BR3_MASK: u32 = 0x00000008;
    pub const BR3: u32 = 1 << 3;

    /// **BR2**
    ///
    /// Reset bit 1
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BR2_OFFSET: u32 = 2;
    pub const BR2_MASK: u32 = 0x00000004;
    pub const BR2: u32 = 1 << 2;

    /// **BR1**
    ///
    /// Reset bit 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BR1_OFFSET: u32 = 1;
    pub const BR1_MASK: u32 = 0x00000002;
    pub const BR1: u32 = 1 << 1;

    /// **BR0**
    ///
    /// Reset bit 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BR0_OFFSET: u32 = 0;
    pub const BR0_MASK: u32 = 0x00000001;
    pub const BR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **BR23** (WO)
    ///
    /// Reset bit 23
    #[inline(always)]
    pub fn trigger_br23(&self) {
        self.value.write_bits(Self::BR23_MASK);
    }

    /// Trigger **BR22** (WO)
    ///
    /// Reset bit 22
    #[inline(always)]
    pub fn trigger_br22(&self) {
        self.value.write_bits(Self::BR22_MASK);
    }

    /// Trigger **BR21** (WO)
    ///
    /// Reset bit 21
    #[inline(always)]
    pub fn trigger_br21(&self) {
        self.value.write_bits(Self::BR21_MASK);
    }

    /// Trigger **BR20** (WO)
    ///
    /// Reset bit 20
    #[inline(always)]
    pub fn trigger_br20(&self) {
        self.value.write_bits(Self::BR20_MASK);
    }

    /// Trigger **BR19** (WO)
    ///
    /// Reset bit 19
    #[inline(always)]
    pub fn trigger_br19(&self) {
        self.value.write_bits(Self::BR19_MASK);
    }

    /// Trigger **BR18** (WO)
    ///
    /// Reset bit 18
    #[inline(always)]
    pub fn trigger_br18(&self) {
        self.value.write_bits(Self::BR18_MASK);
    }

    /// Trigger **BR17** (WO)
    ///
    /// Reset bit 17
    #[inline(always)]
    pub fn trigger_br17(&self) {
        self.value.write_bits(Self::BR17_MASK);
    }

    /// Trigger **BR16** (WO)
    ///
    /// Reset bit 16
    #[inline(always)]
    pub fn trigger_br16(&self) {
        self.value.write_bits(Self::BR16_MASK);
    }

    /// Trigger **BR15** (WO)
    ///
    /// Reset bit 15
    #[inline(always)]
    pub fn trigger_br15(&self) {
        self.value.write_bits(Self::BR15_MASK);
    }

    /// Trigger **BR14** (WO)
    ///
    /// Reset bit 14
    #[inline(always)]
    pub fn trigger_br14(&self) {
        self.value.write_bits(Self::BR14_MASK);
    }

    /// Trigger **BR13** (WO)
    ///
    /// Reset bit 13
    #[inline(always)]
    pub fn trigger_br13(&self) {
        self.value.write_bits(Self::BR13_MASK);
    }

    /// Trigger **BR12** (WO)
    ///
    /// Reset bit 12
    #[inline(always)]
    pub fn trigger_br12(&self) {
        self.value.write_bits(Self::BR12_MASK);
    }

    /// Trigger **BR11** (WO)
    ///
    /// Reset bit 11
    #[inline(always)]
    pub fn trigger_br11(&self) {
        self.value.write_bits(Self::BR11_MASK);
    }

    /// Trigger **BR10** (WO)
    ///
    /// Reset bit 10
    #[inline(always)]
    pub fn trigger_br10(&self) {
        self.value.write_bits(Self::BR10_MASK);
    }

    /// Trigger **BR9** (WO)
    ///
    /// Reset bit 9
    #[inline(always)]
    pub fn trigger_br9(&self) {
        self.value.write_bits(Self::BR9_MASK);
    }

    /// Trigger **BR8** (WO)
    ///
    /// Reset bit 8
    #[inline(always)]
    pub fn trigger_br8(&self) {
        self.value.write_bits(Self::BR8_MASK);
    }

    /// Trigger **BR7** (WO)
    ///
    /// Reset bit 7
    #[inline(always)]
    pub fn trigger_br7(&self) {
        self.value.write_bits(Self::BR7_MASK);
    }

    /// Trigger **BR6** (WO)
    ///
    /// Reset bit 6
    #[inline(always)]
    pub fn trigger_br6(&self) {
        self.value.write_bits(Self::BR6_MASK);
    }

    /// Trigger **BR5** (WO)
    ///
    /// Reset bit 5
    #[inline(always)]
    pub fn trigger_br5(&self) {
        self.value.write_bits(Self::BR5_MASK);
    }

    /// Trigger **BR4** (WO)
    ///
    /// Reset bit 4
    #[inline(always)]
    pub fn trigger_br4(&self) {
        self.value.write_bits(Self::BR4_MASK);
    }

    /// Trigger **BR3** (WO)
    ///
    /// Reset bit 3
    #[inline(always)]
    pub fn trigger_br3(&self) {
        self.value.write_bits(Self::BR3_MASK);
    }

    /// Trigger **BR2** (WO)
    ///
    /// Reset bit 1
    #[inline(always)]
    pub fn trigger_br2(&self) {
        self.value.write_bits(Self::BR2_MASK);
    }

    /// Trigger **BR1** (WO)
    ///
    /// Reset bit 1
    #[inline(always)]
    pub fn trigger_br1(&self) {
        self.value.write_bits(Self::BR1_MASK);
    }

    /// Trigger **BR0** (WO)
    ///
    /// Reset bit 0
    #[inline(always)]
    pub fn trigger_br0(&self) {
        self.value.write_bits(Self::BR0_MASK);
    }
}

/// BCR value type for batch read/modify/write operations
///
/// Use `bcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BcrVal {
    bits: u32,
}

impl BcrVal {
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

    /// Set **BR23** `[23:23]`
    ///
    /// Reset bit 23
    #[inline(always)]
    pub const fn with_br23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR23_MASK
            } else {
                self.bits & !Bcr::BR23_MASK
            },
        }
    }

    /// Set **BR22** `[22:22]`
    ///
    /// Reset bit 22
    #[inline(always)]
    pub const fn with_br22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR22_MASK
            } else {
                self.bits & !Bcr::BR22_MASK
            },
        }
    }

    /// Set **BR21** `[21:21]`
    ///
    /// Reset bit 21
    #[inline(always)]
    pub const fn with_br21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR21_MASK
            } else {
                self.bits & !Bcr::BR21_MASK
            },
        }
    }

    /// Set **BR20** `[20:20]`
    ///
    /// Reset bit 20
    #[inline(always)]
    pub const fn with_br20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR20_MASK
            } else {
                self.bits & !Bcr::BR20_MASK
            },
        }
    }

    /// Set **BR19** `[19:19]`
    ///
    /// Reset bit 19
    #[inline(always)]
    pub const fn with_br19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR19_MASK
            } else {
                self.bits & !Bcr::BR19_MASK
            },
        }
    }

    /// Set **BR18** `[18:18]`
    ///
    /// Reset bit 18
    #[inline(always)]
    pub const fn with_br18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR18_MASK
            } else {
                self.bits & !Bcr::BR18_MASK
            },
        }
    }

    /// Set **BR17** `[17:17]`
    ///
    /// Reset bit 17
    #[inline(always)]
    pub const fn with_br17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR17_MASK
            } else {
                self.bits & !Bcr::BR17_MASK
            },
        }
    }

    /// Set **BR16** `[16:16]`
    ///
    /// Reset bit 16
    #[inline(always)]
    pub const fn with_br16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR16_MASK
            } else {
                self.bits & !Bcr::BR16_MASK
            },
        }
    }

    /// Set **BR15** `[15:15]`
    ///
    /// Reset bit 15
    #[inline(always)]
    pub const fn with_br15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR15_MASK
            } else {
                self.bits & !Bcr::BR15_MASK
            },
        }
    }

    /// Set **BR14** `[14:14]`
    ///
    /// Reset bit 14
    #[inline(always)]
    pub const fn with_br14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR14_MASK
            } else {
                self.bits & !Bcr::BR14_MASK
            },
        }
    }

    /// Set **BR13** `[13:13]`
    ///
    /// Reset bit 13
    #[inline(always)]
    pub const fn with_br13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR13_MASK
            } else {
                self.bits & !Bcr::BR13_MASK
            },
        }
    }

    /// Set **BR12** `[12:12]`
    ///
    /// Reset bit 12
    #[inline(always)]
    pub const fn with_br12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR12_MASK
            } else {
                self.bits & !Bcr::BR12_MASK
            },
        }
    }

    /// Set **BR11** `[11:11]`
    ///
    /// Reset bit 11
    #[inline(always)]
    pub const fn with_br11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR11_MASK
            } else {
                self.bits & !Bcr::BR11_MASK
            },
        }
    }

    /// Set **BR10** `[10:10]`
    ///
    /// Reset bit 10
    #[inline(always)]
    pub const fn with_br10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR10_MASK
            } else {
                self.bits & !Bcr::BR10_MASK
            },
        }
    }

    /// Set **BR9** `[9:9]`
    ///
    /// Reset bit 9
    #[inline(always)]
    pub const fn with_br9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR9_MASK
            } else {
                self.bits & !Bcr::BR9_MASK
            },
        }
    }

    /// Set **BR8** `[8:8]`
    ///
    /// Reset bit 8
    #[inline(always)]
    pub const fn with_br8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR8_MASK
            } else {
                self.bits & !Bcr::BR8_MASK
            },
        }
    }

    /// Set **BR7** `[7:7]`
    ///
    /// Reset bit 7
    #[inline(always)]
    pub const fn with_br7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR7_MASK
            } else {
                self.bits & !Bcr::BR7_MASK
            },
        }
    }

    /// Set **BR6** `[6:6]`
    ///
    /// Reset bit 6
    #[inline(always)]
    pub const fn with_br6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR6_MASK
            } else {
                self.bits & !Bcr::BR6_MASK
            },
        }
    }

    /// Set **BR5** `[5:5]`
    ///
    /// Reset bit 5
    #[inline(always)]
    pub const fn with_br5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR5_MASK
            } else {
                self.bits & !Bcr::BR5_MASK
            },
        }
    }

    /// Set **BR4** `[4:4]`
    ///
    /// Reset bit 4
    #[inline(always)]
    pub const fn with_br4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR4_MASK
            } else {
                self.bits & !Bcr::BR4_MASK
            },
        }
    }

    /// Set **BR3** `[3:3]`
    ///
    /// Reset bit 3
    #[inline(always)]
    pub const fn with_br3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR3_MASK
            } else {
                self.bits & !Bcr::BR3_MASK
            },
        }
    }

    /// Set **BR2** `[2:2]`
    ///
    /// Reset bit 1
    #[inline(always)]
    pub const fn with_br2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR2_MASK
            } else {
                self.bits & !Bcr::BR2_MASK
            },
        }
    }

    /// Set **BR1** `[1:1]`
    ///
    /// Reset bit 1
    #[inline(always)]
    pub const fn with_br1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR1_MASK
            } else {
                self.bits & !Bcr::BR1_MASK
            },
        }
    }

    /// Set **BR0** `[0:0]`
    ///
    /// Reset bit 0
    #[inline(always)]
    pub const fn with_br0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bcr::BR0_MASK
            } else {
                self.bits & !Bcr::BR0_MASK
            },
        }
    }
}

impl Default for BcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bcr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BcrVal) {
        self.value.write_bits(val.bits())
    }
}

/// LCKR Register
///
/// Port configuration lock
/// register
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LCKK** `[24]` (RO)
///
///   Lock key
///
/// - **LCK23** `[23]` (RW)
///
///   Port A Lock bit 23
///
/// - **LCK22** `[22]` (RW)
///
///   Port A Lock bit 22
///
/// - **LCK21** `[21]` (RW)
///
///   Port A Lock bit 21
///
/// - **LCK20** `[20]` (RW)
///
///   Port A Lock bit 20
///
/// - **LCK19** `[19]` (RW)
///
///   Port A Lock bit 19
///
/// - **LCK18** `[18]` (RW)
///
///   Port A Lock bit 18
///
/// - **LCK17** `[17]` (RW)
///
///   Port A Lock bit 17
///
/// - **LCK16** `[16]` (RW)
///
///   Port A Lock bit 16
///
/// - **LCK15** `[15]` (RW)
///
///   Port A Lock bit 15
///
/// - **LCK14** `[14]` (RW)
///
///   Port A Lock bit 14
///
/// - **LCK13** `[13]` (RW)
///
///   Port A Lock bit 13
///
/// - **LCK12** `[12]` (RW)
///
///   Port A Lock bit 12
///
/// - **LCK11** `[11]` (RW)
///
///   Port A Lock bit 11
///
/// - **LCK10** `[10]` (RW)
///
///   Port A Lock bit 10
///
/// - **LCK9** `[9]` (RW)
///
///   Port A Lock bit 9
///
/// - **LCK8** `[8]` (RW)
///
///   Port A Lock bit 8
///
/// - **LCK7** `[7]` (RW)
///
///   Port A Lock bit 7
///
/// - **LCK6** `[6]` (RW)
///
///   Port A Lock bit 6
///
/// - **LCK5** `[5]` (RW)
///
///   Port A Lock bit 5
///
/// - **LCK4** `[4]` (RW)
///
///   Port A Lock bit 4
///
/// - **LCK3** `[3]` (RW)
///
///   Port A Lock bit 3
///
/// - **LCK2** `[2]` (RW)
///
///   Port A Lock bit 2
///
/// - **LCK1** `[1]` (RW)
///
///   Port A Lock bit 1
///
/// - **LCK0** `[0]` (RW)
///
///   Port A Lock bit 0
#[repr(transparent)]
pub struct Lckr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Lckr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Lckr {
    // ======== Field Constants ========

    /// **LCKK**
    ///
    /// Lock key
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const LCKK_OFFSET: u32 = 24;
    pub const LCKK_MASK: u32 = 0x01000000;
    pub const LCKK: u32 = 1 << 24;

    /// **LCK23**
    ///
    /// Port A Lock bit 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const LCK23_OFFSET: u32 = 23;
    pub const LCK23_MASK: u32 = 0x00800000;
    pub const LCK23: u32 = 1 << 23;

    /// **LCK22**
    ///
    /// Port A Lock bit 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const LCK22_OFFSET: u32 = 22;
    pub const LCK22_MASK: u32 = 0x00400000;
    pub const LCK22: u32 = 1 << 22;

    /// **LCK21**
    ///
    /// Port A Lock bit 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LCK21_OFFSET: u32 = 21;
    pub const LCK21_MASK: u32 = 0x00200000;
    pub const LCK21: u32 = 1 << 21;

    /// **LCK20**
    ///
    /// Port A Lock bit 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LCK20_OFFSET: u32 = 20;
    pub const LCK20_MASK: u32 = 0x00100000;
    pub const LCK20: u32 = 1 << 20;

    /// **LCK19**
    ///
    /// Port A Lock bit 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const LCK19_OFFSET: u32 = 19;
    pub const LCK19_MASK: u32 = 0x00080000;
    pub const LCK19: u32 = 1 << 19;

    /// **LCK18**
    ///
    /// Port A Lock bit 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const LCK18_OFFSET: u32 = 18;
    pub const LCK18_MASK: u32 = 0x00040000;
    pub const LCK18: u32 = 1 << 18;

    /// **LCK17**
    ///
    /// Port A Lock bit 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const LCK17_OFFSET: u32 = 17;
    pub const LCK17_MASK: u32 = 0x00020000;
    pub const LCK17: u32 = 1 << 17;

    /// **LCK16**
    ///
    /// Port A Lock bit 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LCK16_OFFSET: u32 = 16;
    pub const LCK16_MASK: u32 = 0x00010000;
    pub const LCK16: u32 = 1 << 16;

    /// **LCK15**
    ///
    /// Port A Lock bit 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const LCK15_OFFSET: u32 = 15;
    pub const LCK15_MASK: u32 = 0x00008000;
    pub const LCK15: u32 = 1 << 15;

    /// **LCK14**
    ///
    /// Port A Lock bit 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LCK14_OFFSET: u32 = 14;
    pub const LCK14_MASK: u32 = 0x00004000;
    pub const LCK14: u32 = 1 << 14;

    /// **LCK13**
    ///
    /// Port A Lock bit 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LCK13_OFFSET: u32 = 13;
    pub const LCK13_MASK: u32 = 0x00002000;
    pub const LCK13: u32 = 1 << 13;

    /// **LCK12**
    ///
    /// Port A Lock bit 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LCK12_OFFSET: u32 = 12;
    pub const LCK12_MASK: u32 = 0x00001000;
    pub const LCK12: u32 = 1 << 12;

    /// **LCK11**
    ///
    /// Port A Lock bit 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const LCK11_OFFSET: u32 = 11;
    pub const LCK11_MASK: u32 = 0x00000800;
    pub const LCK11: u32 = 1 << 11;

    /// **LCK10**
    ///
    /// Port A Lock bit 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LCK10_OFFSET: u32 = 10;
    pub const LCK10_MASK: u32 = 0x00000400;
    pub const LCK10: u32 = 1 << 10;

    /// **LCK9**
    ///
    /// Port A Lock bit 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LCK9_OFFSET: u32 = 9;
    pub const LCK9_MASK: u32 = 0x00000200;
    pub const LCK9: u32 = 1 << 9;

    /// **LCK8**
    ///
    /// Port A Lock bit 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LCK8_OFFSET: u32 = 8;
    pub const LCK8_MASK: u32 = 0x00000100;
    pub const LCK8: u32 = 1 << 8;

    /// **LCK7**
    ///
    /// Port A Lock bit 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LCK7_OFFSET: u32 = 7;
    pub const LCK7_MASK: u32 = 0x00000080;
    pub const LCK7: u32 = 1 << 7;

    /// **LCK6**
    ///
    /// Port A Lock bit 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LCK6_OFFSET: u32 = 6;
    pub const LCK6_MASK: u32 = 0x00000040;
    pub const LCK6: u32 = 1 << 6;

    /// **LCK5**
    ///
    /// Port A Lock bit 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LCK5_OFFSET: u32 = 5;
    pub const LCK5_MASK: u32 = 0x00000020;
    pub const LCK5: u32 = 1 << 5;

    /// **LCK4**
    ///
    /// Port A Lock bit 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LCK4_OFFSET: u32 = 4;
    pub const LCK4_MASK: u32 = 0x00000010;
    pub const LCK4: u32 = 1 << 4;

    /// **LCK3**
    ///
    /// Port A Lock bit 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LCK3_OFFSET: u32 = 3;
    pub const LCK3_MASK: u32 = 0x00000008;
    pub const LCK3: u32 = 1 << 3;

    /// **LCK2**
    ///
    /// Port A Lock bit 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LCK2_OFFSET: u32 = 2;
    pub const LCK2_MASK: u32 = 0x00000004;
    pub const LCK2: u32 = 1 << 2;

    /// **LCK1**
    ///
    /// Port A Lock bit 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LCK1_OFFSET: u32 = 1;
    pub const LCK1_MASK: u32 = 0x00000002;
    pub const LCK1: u32 = 1 << 1;

    /// **LCK0**
    ///
    /// Port A Lock bit 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LCK0_OFFSET: u32 = 0;
    pub const LCK0_MASK: u32 = 0x00000001;
    pub const LCK0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LCKK** `[24]` (RO)
    ///
    /// Lock key
    #[inline(always)]
    pub fn lckk(&self) -> bool {
        (self.value.read_bits() & Self::LCKK_MASK) != 0
    }

    /// **LCK23** `[23]` (RW)
    ///
    /// Port A Lock bit 23
    #[inline(always)]
    pub fn lck23(&self) -> bool {
        (self.value.read_bits() & Self::LCK23_MASK) != 0
    }

    /// Set LCK23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck23(&self) {
        self.value.modify_bits(|r| r | Self::LCK23_MASK);
    }

    /// Clear LCK23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck23(&self) {
        self.value.modify_bits(|r| r & !Self::LCK23_MASK);
    }

    /// **LCK22** `[22]` (RW)
    ///
    /// Port A Lock bit 22
    #[inline(always)]
    pub fn lck22(&self) -> bool {
        (self.value.read_bits() & Self::LCK22_MASK) != 0
    }

    /// Set LCK22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck22(&self) {
        self.value.modify_bits(|r| r | Self::LCK22_MASK);
    }

    /// Clear LCK22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck22(&self) {
        self.value.modify_bits(|r| r & !Self::LCK22_MASK);
    }

    /// **LCK21** `[21]` (RW)
    ///
    /// Port A Lock bit 21
    #[inline(always)]
    pub fn lck21(&self) -> bool {
        (self.value.read_bits() & Self::LCK21_MASK) != 0
    }

    /// Set LCK21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck21(&self) {
        self.value.modify_bits(|r| r | Self::LCK21_MASK);
    }

    /// Clear LCK21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck21(&self) {
        self.value.modify_bits(|r| r & !Self::LCK21_MASK);
    }

    /// **LCK20** `[20]` (RW)
    ///
    /// Port A Lock bit 20
    #[inline(always)]
    pub fn lck20(&self) -> bool {
        (self.value.read_bits() & Self::LCK20_MASK) != 0
    }

    /// Set LCK20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck20(&self) {
        self.value.modify_bits(|r| r | Self::LCK20_MASK);
    }

    /// Clear LCK20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck20(&self) {
        self.value.modify_bits(|r| r & !Self::LCK20_MASK);
    }

    /// **LCK19** `[19]` (RW)
    ///
    /// Port A Lock bit 19
    #[inline(always)]
    pub fn lck19(&self) -> bool {
        (self.value.read_bits() & Self::LCK19_MASK) != 0
    }

    /// Set LCK19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck19(&self) {
        self.value.modify_bits(|r| r | Self::LCK19_MASK);
    }

    /// Clear LCK19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck19(&self) {
        self.value.modify_bits(|r| r & !Self::LCK19_MASK);
    }

    /// **LCK18** `[18]` (RW)
    ///
    /// Port A Lock bit 18
    #[inline(always)]
    pub fn lck18(&self) -> bool {
        (self.value.read_bits() & Self::LCK18_MASK) != 0
    }

    /// Set LCK18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck18(&self) {
        self.value.modify_bits(|r| r | Self::LCK18_MASK);
    }

    /// Clear LCK18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck18(&self) {
        self.value.modify_bits(|r| r & !Self::LCK18_MASK);
    }

    /// **LCK17** `[17]` (RW)
    ///
    /// Port A Lock bit 17
    #[inline(always)]
    pub fn lck17(&self) -> bool {
        (self.value.read_bits() & Self::LCK17_MASK) != 0
    }

    /// Set LCK17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck17(&self) {
        self.value.modify_bits(|r| r | Self::LCK17_MASK);
    }

    /// Clear LCK17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck17(&self) {
        self.value.modify_bits(|r| r & !Self::LCK17_MASK);
    }

    /// **LCK16** `[16]` (RW)
    ///
    /// Port A Lock bit 16
    #[inline(always)]
    pub fn lck16(&self) -> bool {
        (self.value.read_bits() & Self::LCK16_MASK) != 0
    }

    /// Set LCK16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck16(&self) {
        self.value.modify_bits(|r| r | Self::LCK16_MASK);
    }

    /// Clear LCK16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck16(&self) {
        self.value.modify_bits(|r| r & !Self::LCK16_MASK);
    }

    /// **LCK15** `[15]` (RW)
    ///
    /// Port A Lock bit 15
    #[inline(always)]
    pub fn lck15(&self) -> bool {
        (self.value.read_bits() & Self::LCK15_MASK) != 0
    }

    /// Set LCK15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck15(&self) {
        self.value.modify_bits(|r| r | Self::LCK15_MASK);
    }

    /// Clear LCK15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck15(&self) {
        self.value.modify_bits(|r| r & !Self::LCK15_MASK);
    }

    /// **LCK14** `[14]` (RW)
    ///
    /// Port A Lock bit 14
    #[inline(always)]
    pub fn lck14(&self) -> bool {
        (self.value.read_bits() & Self::LCK14_MASK) != 0
    }

    /// Set LCK14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck14(&self) {
        self.value.modify_bits(|r| r | Self::LCK14_MASK);
    }

    /// Clear LCK14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck14(&self) {
        self.value.modify_bits(|r| r & !Self::LCK14_MASK);
    }

    /// **LCK13** `[13]` (RW)
    ///
    /// Port A Lock bit 13
    #[inline(always)]
    pub fn lck13(&self) -> bool {
        (self.value.read_bits() & Self::LCK13_MASK) != 0
    }

    /// Set LCK13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck13(&self) {
        self.value.modify_bits(|r| r | Self::LCK13_MASK);
    }

    /// Clear LCK13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck13(&self) {
        self.value.modify_bits(|r| r & !Self::LCK13_MASK);
    }

    /// **LCK12** `[12]` (RW)
    ///
    /// Port A Lock bit 12
    #[inline(always)]
    pub fn lck12(&self) -> bool {
        (self.value.read_bits() & Self::LCK12_MASK) != 0
    }

    /// Set LCK12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck12(&self) {
        self.value.modify_bits(|r| r | Self::LCK12_MASK);
    }

    /// Clear LCK12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck12(&self) {
        self.value.modify_bits(|r| r & !Self::LCK12_MASK);
    }

    /// **LCK11** `[11]` (RW)
    ///
    /// Port A Lock bit 11
    #[inline(always)]
    pub fn lck11(&self) -> bool {
        (self.value.read_bits() & Self::LCK11_MASK) != 0
    }

    /// Set LCK11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck11(&self) {
        self.value.modify_bits(|r| r | Self::LCK11_MASK);
    }

    /// Clear LCK11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck11(&self) {
        self.value.modify_bits(|r| r & !Self::LCK11_MASK);
    }

    /// **LCK10** `[10]` (RW)
    ///
    /// Port A Lock bit 10
    #[inline(always)]
    pub fn lck10(&self) -> bool {
        (self.value.read_bits() & Self::LCK10_MASK) != 0
    }

    /// Set LCK10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck10(&self) {
        self.value.modify_bits(|r| r | Self::LCK10_MASK);
    }

    /// Clear LCK10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck10(&self) {
        self.value.modify_bits(|r| r & !Self::LCK10_MASK);
    }

    /// **LCK9** `[9]` (RW)
    ///
    /// Port A Lock bit 9
    #[inline(always)]
    pub fn lck9(&self) -> bool {
        (self.value.read_bits() & Self::LCK9_MASK) != 0
    }

    /// Set LCK9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck9(&self) {
        self.value.modify_bits(|r| r | Self::LCK9_MASK);
    }

    /// Clear LCK9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck9(&self) {
        self.value.modify_bits(|r| r & !Self::LCK9_MASK);
    }

    /// **LCK8** `[8]` (RW)
    ///
    /// Port A Lock bit 8
    #[inline(always)]
    pub fn lck8(&self) -> bool {
        (self.value.read_bits() & Self::LCK8_MASK) != 0
    }

    /// Set LCK8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck8(&self) {
        self.value.modify_bits(|r| r | Self::LCK8_MASK);
    }

    /// Clear LCK8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck8(&self) {
        self.value.modify_bits(|r| r & !Self::LCK8_MASK);
    }

    /// **LCK7** `[7]` (RW)
    ///
    /// Port A Lock bit 7
    #[inline(always)]
    pub fn lck7(&self) -> bool {
        (self.value.read_bits() & Self::LCK7_MASK) != 0
    }

    /// Set LCK7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck7(&self) {
        self.value.modify_bits(|r| r | Self::LCK7_MASK);
    }

    /// Clear LCK7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck7(&self) {
        self.value.modify_bits(|r| r & !Self::LCK7_MASK);
    }

    /// **LCK6** `[6]` (RW)
    ///
    /// Port A Lock bit 6
    #[inline(always)]
    pub fn lck6(&self) -> bool {
        (self.value.read_bits() & Self::LCK6_MASK) != 0
    }

    /// Set LCK6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck6(&self) {
        self.value.modify_bits(|r| r | Self::LCK6_MASK);
    }

    /// Clear LCK6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck6(&self) {
        self.value.modify_bits(|r| r & !Self::LCK6_MASK);
    }

    /// **LCK5** `[5]` (RW)
    ///
    /// Port A Lock bit 5
    #[inline(always)]
    pub fn lck5(&self) -> bool {
        (self.value.read_bits() & Self::LCK5_MASK) != 0
    }

    /// Set LCK5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck5(&self) {
        self.value.modify_bits(|r| r | Self::LCK5_MASK);
    }

    /// Clear LCK5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck5(&self) {
        self.value.modify_bits(|r| r & !Self::LCK5_MASK);
    }

    /// **LCK4** `[4]` (RW)
    ///
    /// Port A Lock bit 4
    #[inline(always)]
    pub fn lck4(&self) -> bool {
        (self.value.read_bits() & Self::LCK4_MASK) != 0
    }

    /// Set LCK4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck4(&self) {
        self.value.modify_bits(|r| r | Self::LCK4_MASK);
    }

    /// Clear LCK4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck4(&self) {
        self.value.modify_bits(|r| r & !Self::LCK4_MASK);
    }

    /// **LCK3** `[3]` (RW)
    ///
    /// Port A Lock bit 3
    #[inline(always)]
    pub fn lck3(&self) -> bool {
        (self.value.read_bits() & Self::LCK3_MASK) != 0
    }

    /// Set LCK3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck3(&self) {
        self.value.modify_bits(|r| r | Self::LCK3_MASK);
    }

    /// Clear LCK3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck3(&self) {
        self.value.modify_bits(|r| r & !Self::LCK3_MASK);
    }

    /// **LCK2** `[2]` (RW)
    ///
    /// Port A Lock bit 2
    #[inline(always)]
    pub fn lck2(&self) -> bool {
        (self.value.read_bits() & Self::LCK2_MASK) != 0
    }

    /// Set LCK2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck2(&self) {
        self.value.modify_bits(|r| r | Self::LCK2_MASK);
    }

    /// Clear LCK2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck2(&self) {
        self.value.modify_bits(|r| r & !Self::LCK2_MASK);
    }

    /// **LCK1** `[1]` (RW)
    ///
    /// Port A Lock bit 1
    #[inline(always)]
    pub fn lck1(&self) -> bool {
        (self.value.read_bits() & Self::LCK1_MASK) != 0
    }

    /// Set LCK1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck1(&self) {
        self.value.modify_bits(|r| r | Self::LCK1_MASK);
    }

    /// Clear LCK1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck1(&self) {
        self.value.modify_bits(|r| r & !Self::LCK1_MASK);
    }

    /// **LCK0** `[0]` (RW)
    ///
    /// Port A Lock bit 0
    #[inline(always)]
    pub fn lck0(&self) -> bool {
        (self.value.read_bits() & Self::LCK0_MASK) != 0
    }

    /// Set LCK0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lck0(&self) {
        self.value.modify_bits(|r| r | Self::LCK0_MASK);
    }

    /// Clear LCK0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lck0(&self) {
        self.value.modify_bits(|r| r & !Self::LCK0_MASK);
    }
}

/// LCKR value type for batch read/modify/write operations
///
/// Use `lckr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lckr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LckrVal {
    bits: u32,
}

impl LckrVal {
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

    /// Read **LCKK** `[24:24]`
    ///
    /// Lock key
    #[inline(always)]
    pub const fn lckk(&self) -> bool {
        (self.bits & Lckr::LCKK_MASK) != 0
    }

    /// Read **LCK23** `[23:23]`
    ///
    /// Port A Lock bit 23
    #[inline(always)]
    pub const fn lck23(&self) -> bool {
        (self.bits & Lckr::LCK23_MASK) != 0
    }

    /// Read **LCK22** `[22:22]`
    ///
    /// Port A Lock bit 22
    #[inline(always)]
    pub const fn lck22(&self) -> bool {
        (self.bits & Lckr::LCK22_MASK) != 0
    }

    /// Read **LCK21** `[21:21]`
    ///
    /// Port A Lock bit 21
    #[inline(always)]
    pub const fn lck21(&self) -> bool {
        (self.bits & Lckr::LCK21_MASK) != 0
    }

    /// Read **LCK20** `[20:20]`
    ///
    /// Port A Lock bit 20
    #[inline(always)]
    pub const fn lck20(&self) -> bool {
        (self.bits & Lckr::LCK20_MASK) != 0
    }

    /// Read **LCK19** `[19:19]`
    ///
    /// Port A Lock bit 19
    #[inline(always)]
    pub const fn lck19(&self) -> bool {
        (self.bits & Lckr::LCK19_MASK) != 0
    }

    /// Read **LCK18** `[18:18]`
    ///
    /// Port A Lock bit 18
    #[inline(always)]
    pub const fn lck18(&self) -> bool {
        (self.bits & Lckr::LCK18_MASK) != 0
    }

    /// Read **LCK17** `[17:17]`
    ///
    /// Port A Lock bit 17
    #[inline(always)]
    pub const fn lck17(&self) -> bool {
        (self.bits & Lckr::LCK17_MASK) != 0
    }

    /// Read **LCK16** `[16:16]`
    ///
    /// Port A Lock bit 16
    #[inline(always)]
    pub const fn lck16(&self) -> bool {
        (self.bits & Lckr::LCK16_MASK) != 0
    }

    /// Read **LCK15** `[15:15]`
    ///
    /// Port A Lock bit 15
    #[inline(always)]
    pub const fn lck15(&self) -> bool {
        (self.bits & Lckr::LCK15_MASK) != 0
    }

    /// Read **LCK14** `[14:14]`
    ///
    /// Port A Lock bit 14
    #[inline(always)]
    pub const fn lck14(&self) -> bool {
        (self.bits & Lckr::LCK14_MASK) != 0
    }

    /// Read **LCK13** `[13:13]`
    ///
    /// Port A Lock bit 13
    #[inline(always)]
    pub const fn lck13(&self) -> bool {
        (self.bits & Lckr::LCK13_MASK) != 0
    }

    /// Read **LCK12** `[12:12]`
    ///
    /// Port A Lock bit 12
    #[inline(always)]
    pub const fn lck12(&self) -> bool {
        (self.bits & Lckr::LCK12_MASK) != 0
    }

    /// Read **LCK11** `[11:11]`
    ///
    /// Port A Lock bit 11
    #[inline(always)]
    pub const fn lck11(&self) -> bool {
        (self.bits & Lckr::LCK11_MASK) != 0
    }

    /// Read **LCK10** `[10:10]`
    ///
    /// Port A Lock bit 10
    #[inline(always)]
    pub const fn lck10(&self) -> bool {
        (self.bits & Lckr::LCK10_MASK) != 0
    }

    /// Read **LCK9** `[9:9]`
    ///
    /// Port A Lock bit 9
    #[inline(always)]
    pub const fn lck9(&self) -> bool {
        (self.bits & Lckr::LCK9_MASK) != 0
    }

    /// Read **LCK8** `[8:8]`
    ///
    /// Port A Lock bit 8
    #[inline(always)]
    pub const fn lck8(&self) -> bool {
        (self.bits & Lckr::LCK8_MASK) != 0
    }

    /// Read **LCK7** `[7:7]`
    ///
    /// Port A Lock bit 7
    #[inline(always)]
    pub const fn lck7(&self) -> bool {
        (self.bits & Lckr::LCK7_MASK) != 0
    }

    /// Read **LCK6** `[6:6]`
    ///
    /// Port A Lock bit 6
    #[inline(always)]
    pub const fn lck6(&self) -> bool {
        (self.bits & Lckr::LCK6_MASK) != 0
    }

    /// Read **LCK5** `[5:5]`
    ///
    /// Port A Lock bit 5
    #[inline(always)]
    pub const fn lck5(&self) -> bool {
        (self.bits & Lckr::LCK5_MASK) != 0
    }

    /// Read **LCK4** `[4:4]`
    ///
    /// Port A Lock bit 4
    #[inline(always)]
    pub const fn lck4(&self) -> bool {
        (self.bits & Lckr::LCK4_MASK) != 0
    }

    /// Read **LCK3** `[3:3]`
    ///
    /// Port A Lock bit 3
    #[inline(always)]
    pub const fn lck3(&self) -> bool {
        (self.bits & Lckr::LCK3_MASK) != 0
    }

    /// Read **LCK2** `[2:2]`
    ///
    /// Port A Lock bit 2
    #[inline(always)]
    pub const fn lck2(&self) -> bool {
        (self.bits & Lckr::LCK2_MASK) != 0
    }

    /// Read **LCK1** `[1:1]`
    ///
    /// Port A Lock bit 1
    #[inline(always)]
    pub const fn lck1(&self) -> bool {
        (self.bits & Lckr::LCK1_MASK) != 0
    }

    /// Read **LCK0** `[0:0]`
    ///
    /// Port A Lock bit 0
    #[inline(always)]
    pub const fn lck0(&self) -> bool {
        (self.bits & Lckr::LCK0_MASK) != 0
    }

    /// Set **LCK23** `[23:23]`
    ///
    /// Port A Lock bit 23
    #[inline(always)]
    pub const fn with_lck23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK23_MASK
            } else {
                self.bits & !Lckr::LCK23_MASK
            },
        }
    }

    /// Set **LCK22** `[22:22]`
    ///
    /// Port A Lock bit 22
    #[inline(always)]
    pub const fn with_lck22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK22_MASK
            } else {
                self.bits & !Lckr::LCK22_MASK
            },
        }
    }

    /// Set **LCK21** `[21:21]`
    ///
    /// Port A Lock bit 21
    #[inline(always)]
    pub const fn with_lck21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK21_MASK
            } else {
                self.bits & !Lckr::LCK21_MASK
            },
        }
    }

    /// Set **LCK20** `[20:20]`
    ///
    /// Port A Lock bit 20
    #[inline(always)]
    pub const fn with_lck20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK20_MASK
            } else {
                self.bits & !Lckr::LCK20_MASK
            },
        }
    }

    /// Set **LCK19** `[19:19]`
    ///
    /// Port A Lock bit 19
    #[inline(always)]
    pub const fn with_lck19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK19_MASK
            } else {
                self.bits & !Lckr::LCK19_MASK
            },
        }
    }

    /// Set **LCK18** `[18:18]`
    ///
    /// Port A Lock bit 18
    #[inline(always)]
    pub const fn with_lck18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK18_MASK
            } else {
                self.bits & !Lckr::LCK18_MASK
            },
        }
    }

    /// Set **LCK17** `[17:17]`
    ///
    /// Port A Lock bit 17
    #[inline(always)]
    pub const fn with_lck17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK17_MASK
            } else {
                self.bits & !Lckr::LCK17_MASK
            },
        }
    }

    /// Set **LCK16** `[16:16]`
    ///
    /// Port A Lock bit 16
    #[inline(always)]
    pub const fn with_lck16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK16_MASK
            } else {
                self.bits & !Lckr::LCK16_MASK
            },
        }
    }

    /// Set **LCK15** `[15:15]`
    ///
    /// Port A Lock bit 15
    #[inline(always)]
    pub const fn with_lck15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK15_MASK
            } else {
                self.bits & !Lckr::LCK15_MASK
            },
        }
    }

    /// Set **LCK14** `[14:14]`
    ///
    /// Port A Lock bit 14
    #[inline(always)]
    pub const fn with_lck14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK14_MASK
            } else {
                self.bits & !Lckr::LCK14_MASK
            },
        }
    }

    /// Set **LCK13** `[13:13]`
    ///
    /// Port A Lock bit 13
    #[inline(always)]
    pub const fn with_lck13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK13_MASK
            } else {
                self.bits & !Lckr::LCK13_MASK
            },
        }
    }

    /// Set **LCK12** `[12:12]`
    ///
    /// Port A Lock bit 12
    #[inline(always)]
    pub const fn with_lck12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK12_MASK
            } else {
                self.bits & !Lckr::LCK12_MASK
            },
        }
    }

    /// Set **LCK11** `[11:11]`
    ///
    /// Port A Lock bit 11
    #[inline(always)]
    pub const fn with_lck11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK11_MASK
            } else {
                self.bits & !Lckr::LCK11_MASK
            },
        }
    }

    /// Set **LCK10** `[10:10]`
    ///
    /// Port A Lock bit 10
    #[inline(always)]
    pub const fn with_lck10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK10_MASK
            } else {
                self.bits & !Lckr::LCK10_MASK
            },
        }
    }

    /// Set **LCK9** `[9:9]`
    ///
    /// Port A Lock bit 9
    #[inline(always)]
    pub const fn with_lck9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK9_MASK
            } else {
                self.bits & !Lckr::LCK9_MASK
            },
        }
    }

    /// Set **LCK8** `[8:8]`
    ///
    /// Port A Lock bit 8
    #[inline(always)]
    pub const fn with_lck8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK8_MASK
            } else {
                self.bits & !Lckr::LCK8_MASK
            },
        }
    }

    /// Set **LCK7** `[7:7]`
    ///
    /// Port A Lock bit 7
    #[inline(always)]
    pub const fn with_lck7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK7_MASK
            } else {
                self.bits & !Lckr::LCK7_MASK
            },
        }
    }

    /// Set **LCK6** `[6:6]`
    ///
    /// Port A Lock bit 6
    #[inline(always)]
    pub const fn with_lck6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK6_MASK
            } else {
                self.bits & !Lckr::LCK6_MASK
            },
        }
    }

    /// Set **LCK5** `[5:5]`
    ///
    /// Port A Lock bit 5
    #[inline(always)]
    pub const fn with_lck5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK5_MASK
            } else {
                self.bits & !Lckr::LCK5_MASK
            },
        }
    }

    /// Set **LCK4** `[4:4]`
    ///
    /// Port A Lock bit 4
    #[inline(always)]
    pub const fn with_lck4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK4_MASK
            } else {
                self.bits & !Lckr::LCK4_MASK
            },
        }
    }

    /// Set **LCK3** `[3:3]`
    ///
    /// Port A Lock bit 3
    #[inline(always)]
    pub const fn with_lck3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK3_MASK
            } else {
                self.bits & !Lckr::LCK3_MASK
            },
        }
    }

    /// Set **LCK2** `[2:2]`
    ///
    /// Port A Lock bit 2
    #[inline(always)]
    pub const fn with_lck2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK2_MASK
            } else {
                self.bits & !Lckr::LCK2_MASK
            },
        }
    }

    /// Set **LCK1** `[1:1]`
    ///
    /// Port A Lock bit 1
    #[inline(always)]
    pub const fn with_lck1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK1_MASK
            } else {
                self.bits & !Lckr::LCK1_MASK
            },
        }
    }

    /// Set **LCK0** `[0:0]`
    ///
    /// Port A Lock bit 0
    #[inline(always)]
    pub const fn with_lck0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lckr::LCK0_MASK
            } else {
                self.bits & !Lckr::LCK0_MASK
            },
        }
    }
}

impl Default for LckrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Lckr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LckrVal {
        LckrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LckrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LckrVal) -> LckrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGXR Register
///
/// Port configuration register extension
/// (GPIOn_CFGXR)
///
/// Address offset: 0x001c
/// Reset value: 0x44444444
///
/// ## Fields
///
/// - **CNF23** `[31:30]` (RW)
///
///   Port n.23 configuration
///   bits
///
/// - **MODE23** `[29:28]` (RW)
///
///   Port n.23 mode bits
///
/// - **CNF22** `[27:26]` (RW)
///
///   Port n.22 configuration
///   bits
///
/// - **MODE22** `[25:24]` (RW)
///
///   Port n.22 mode bits
///
/// - **CNF21** `[23:22]` (RW)
///
///   Port n.21 configuration
///   bits
///
/// - **MODE21** `[21:20]` (RW)
///
///   Port n.21 mode bits
///
/// - **CNF20** `[19:18]` (RW)
///
///   Port n.20 configuration
///   bits
///
/// - **MODE20** `[17:16]` (RW)
///
///   Port n.20 mode bits
///
/// - **CNF19** `[15:14]` (RW)
///
///   Port n.19 configuration
///   bits
///
/// - **MODE19** `[13:12]` (RW)
///
///   Port n.19 mode bits
///
/// - **CNF18** `[11:10]` (RW)
///
///   Port n.18 configuration
///   bits
///
/// - **MODE18** `[9:8]` (RW)
///
///   Port n.18 mode bits
///
/// - **CNF17** `[7:6]` (RW)
///
///   Port n.17 configuration
///   bits
///
/// - **MODE17** `[5:4]` (RW)
///
///   Port n.17 mode bits
///
/// - **CNF16** `[3:2]` (RW)
///
///   Port n.16 configuration
///   bits
///
/// - **MODE16** `[1:0]` (RW)
///
///   Port n.16 mode bits
#[repr(transparent)]
pub struct Cfgxr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cfgxr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgxr {
    // ======== Field Constants ========

    /// **CNF23**
    ///
    /// Port n.23 configuration
    /// bits
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const CNF23_OFFSET: u32 = 30;
    pub const CNF23_MASK: u32 = 0xc0000000;

    /// **MODE23**
    ///
    /// Port n.23 mode bits
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const MODE23_OFFSET: u32 = 28;
    pub const MODE23_MASK: u32 = 0x30000000;

    /// **CNF22**
    ///
    /// Port n.22 configuration
    /// bits
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const CNF22_OFFSET: u32 = 26;
    pub const CNF22_MASK: u32 = 0x0c000000;

    /// **MODE22**
    ///
    /// Port n.22 mode bits
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const MODE22_OFFSET: u32 = 24;
    pub const MODE22_MASK: u32 = 0x03000000;

    /// **CNF21**
    ///
    /// Port n.21 configuration
    /// bits
    ///
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const CNF21_OFFSET: u32 = 22;
    pub const CNF21_MASK: u32 = 0x00c00000;

    /// **MODE21**
    ///
    /// Port n.21 mode bits
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const MODE21_OFFSET: u32 = 20;
    pub const MODE21_MASK: u32 = 0x00300000;

    /// **CNF20**
    ///
    /// Port n.20 configuration
    /// bits
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const CNF20_OFFSET: u32 = 18;
    pub const CNF20_MASK: u32 = 0x000c0000;

    /// **MODE20**
    ///
    /// Port n.20 mode bits
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const MODE20_OFFSET: u32 = 16;
    pub const MODE20_MASK: u32 = 0x00030000;

    /// **CNF19**
    ///
    /// Port n.19 configuration
    /// bits
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const CNF19_OFFSET: u32 = 14;
    pub const CNF19_MASK: u32 = 0x0000c000;

    /// **MODE19**
    ///
    /// Port n.19 mode bits
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const MODE19_OFFSET: u32 = 12;
    pub const MODE19_MASK: u32 = 0x00003000;

    /// **CNF18**
    ///
    /// Port n.18 configuration
    /// bits
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const CNF18_OFFSET: u32 = 10;
    pub const CNF18_MASK: u32 = 0x00000c00;

    /// **MODE18**
    ///
    /// Port n.18 mode bits
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const MODE18_OFFSET: u32 = 8;
    pub const MODE18_MASK: u32 = 0x00000300;

    /// **CNF17**
    ///
    /// Port n.17 configuration
    /// bits
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const CNF17_OFFSET: u32 = 6;
    pub const CNF17_MASK: u32 = 0x000000c0;

    /// **MODE17**
    ///
    /// Port n.17 mode bits
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const MODE17_OFFSET: u32 = 4;
    pub const MODE17_MASK: u32 = 0x00000030;

    /// **CNF16**
    ///
    /// Port n.16 configuration
    /// bits
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const CNF16_OFFSET: u32 = 2;
    pub const CNF16_MASK: u32 = 0x0000000c;

    /// **MODE16**
    ///
    /// Port n.16 mode bits
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MODE16_OFFSET: u32 = 0;
    pub const MODE16_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CNF23** `[31:30]` (RW)
    ///
    /// Port n.23 configuration
    /// bits
    #[inline(always)]
    pub fn cnf23(&self) -> u32 {
        (self.value.read_bits() & Self::CNF23_MASK) >> Self::CNF23_OFFSET
    }

    /// Write **CNF23** `[31:30]` (RW)
    ///
    /// Port n.23 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf23(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF23_MASK) | ((value << Self::CNF23_OFFSET) & Self::CNF23_MASK)
        });
    }

    /// **MODE23** `[29:28]` (RW)
    ///
    /// Port n.23 mode bits
    #[inline(always)]
    pub fn mode23(&self) -> u32 {
        (self.value.read_bits() & Self::MODE23_MASK) >> Self::MODE23_OFFSET
    }

    /// Write **MODE23** `[29:28]` (RW)
    ///
    /// Port n.23 mode bits
    #[inline(always)]
    pub fn write_mode23(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE23_MASK) | ((value << Self::MODE23_OFFSET) & Self::MODE23_MASK)
        });
    }

    /// **CNF22** `[27:26]` (RW)
    ///
    /// Port n.22 configuration
    /// bits
    #[inline(always)]
    pub fn cnf22(&self) -> u32 {
        (self.value.read_bits() & Self::CNF22_MASK) >> Self::CNF22_OFFSET
    }

    /// Write **CNF22** `[27:26]` (RW)
    ///
    /// Port n.22 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf22(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF22_MASK) | ((value << Self::CNF22_OFFSET) & Self::CNF22_MASK)
        });
    }

    /// **MODE22** `[25:24]` (RW)
    ///
    /// Port n.22 mode bits
    #[inline(always)]
    pub fn mode22(&self) -> u32 {
        (self.value.read_bits() & Self::MODE22_MASK) >> Self::MODE22_OFFSET
    }

    /// Write **MODE22** `[25:24]` (RW)
    ///
    /// Port n.22 mode bits
    #[inline(always)]
    pub fn write_mode22(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE22_MASK) | ((value << Self::MODE22_OFFSET) & Self::MODE22_MASK)
        });
    }

    /// **CNF21** `[23:22]` (RW)
    ///
    /// Port n.21 configuration
    /// bits
    #[inline(always)]
    pub fn cnf21(&self) -> u32 {
        (self.value.read_bits() & Self::CNF21_MASK) >> Self::CNF21_OFFSET
    }

    /// Write **CNF21** `[23:22]` (RW)
    ///
    /// Port n.21 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf21(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF21_MASK) | ((value << Self::CNF21_OFFSET) & Self::CNF21_MASK)
        });
    }

    /// **MODE21** `[21:20]` (RW)
    ///
    /// Port n.21 mode bits
    #[inline(always)]
    pub fn mode21(&self) -> u32 {
        (self.value.read_bits() & Self::MODE21_MASK) >> Self::MODE21_OFFSET
    }

    /// Write **MODE21** `[21:20]` (RW)
    ///
    /// Port n.21 mode bits
    #[inline(always)]
    pub fn write_mode21(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE21_MASK) | ((value << Self::MODE21_OFFSET) & Self::MODE21_MASK)
        });
    }

    /// **CNF20** `[19:18]` (RW)
    ///
    /// Port n.20 configuration
    /// bits
    #[inline(always)]
    pub fn cnf20(&self) -> u32 {
        (self.value.read_bits() & Self::CNF20_MASK) >> Self::CNF20_OFFSET
    }

    /// Write **CNF20** `[19:18]` (RW)
    ///
    /// Port n.20 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf20(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF20_MASK) | ((value << Self::CNF20_OFFSET) & Self::CNF20_MASK)
        });
    }

    /// **MODE20** `[17:16]` (RW)
    ///
    /// Port n.20 mode bits
    #[inline(always)]
    pub fn mode20(&self) -> u32 {
        (self.value.read_bits() & Self::MODE20_MASK) >> Self::MODE20_OFFSET
    }

    /// Write **MODE20** `[17:16]` (RW)
    ///
    /// Port n.20 mode bits
    #[inline(always)]
    pub fn write_mode20(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE20_MASK) | ((value << Self::MODE20_OFFSET) & Self::MODE20_MASK)
        });
    }

    /// **CNF19** `[15:14]` (RW)
    ///
    /// Port n.19 configuration
    /// bits
    #[inline(always)]
    pub fn cnf19(&self) -> u32 {
        (self.value.read_bits() & Self::CNF19_MASK) >> Self::CNF19_OFFSET
    }

    /// Write **CNF19** `[15:14]` (RW)
    ///
    /// Port n.19 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf19(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF19_MASK) | ((value << Self::CNF19_OFFSET) & Self::CNF19_MASK)
        });
    }

    /// **MODE19** `[13:12]` (RW)
    ///
    /// Port n.19 mode bits
    #[inline(always)]
    pub fn mode19(&self) -> u32 {
        (self.value.read_bits() & Self::MODE19_MASK) >> Self::MODE19_OFFSET
    }

    /// Write **MODE19** `[13:12]` (RW)
    ///
    /// Port n.19 mode bits
    #[inline(always)]
    pub fn write_mode19(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE19_MASK) | ((value << Self::MODE19_OFFSET) & Self::MODE19_MASK)
        });
    }

    /// **CNF18** `[11:10]` (RW)
    ///
    /// Port n.18 configuration
    /// bits
    #[inline(always)]
    pub fn cnf18(&self) -> u32 {
        (self.value.read_bits() & Self::CNF18_MASK) >> Self::CNF18_OFFSET
    }

    /// Write **CNF18** `[11:10]` (RW)
    ///
    /// Port n.18 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf18(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF18_MASK) | ((value << Self::CNF18_OFFSET) & Self::CNF18_MASK)
        });
    }

    /// **MODE18** `[9:8]` (RW)
    ///
    /// Port n.18 mode bits
    #[inline(always)]
    pub fn mode18(&self) -> u32 {
        (self.value.read_bits() & Self::MODE18_MASK) >> Self::MODE18_OFFSET
    }

    /// Write **MODE18** `[9:8]` (RW)
    ///
    /// Port n.18 mode bits
    #[inline(always)]
    pub fn write_mode18(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE18_MASK) | ((value << Self::MODE18_OFFSET) & Self::MODE18_MASK)
        });
    }

    /// **CNF17** `[7:6]` (RW)
    ///
    /// Port n.17 configuration
    /// bits
    #[inline(always)]
    pub fn cnf17(&self) -> u32 {
        (self.value.read_bits() & Self::CNF17_MASK) >> Self::CNF17_OFFSET
    }

    /// Write **CNF17** `[7:6]` (RW)
    ///
    /// Port n.17 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf17(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF17_MASK) | ((value << Self::CNF17_OFFSET) & Self::CNF17_MASK)
        });
    }

    /// **MODE17** `[5:4]` (RW)
    ///
    /// Port n.17 mode bits
    #[inline(always)]
    pub fn mode17(&self) -> u32 {
        (self.value.read_bits() & Self::MODE17_MASK) >> Self::MODE17_OFFSET
    }

    /// Write **MODE17** `[5:4]` (RW)
    ///
    /// Port n.17 mode bits
    #[inline(always)]
    pub fn write_mode17(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE17_MASK) | ((value << Self::MODE17_OFFSET) & Self::MODE17_MASK)
        });
    }

    /// **CNF16** `[3:2]` (RW)
    ///
    /// Port n.16 configuration
    /// bits
    #[inline(always)]
    pub fn cnf16(&self) -> u32 {
        (self.value.read_bits() & Self::CNF16_MASK) >> Self::CNF16_OFFSET
    }

    /// Write **CNF16** `[3:2]` (RW)
    ///
    /// Port n.16 configuration
    /// bits
    #[inline(always)]
    pub fn write_cnf16(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CNF16_MASK) | ((value << Self::CNF16_OFFSET) & Self::CNF16_MASK)
        });
    }

    /// **MODE16** `[1:0]` (RW)
    ///
    /// Port n.16 mode bits
    #[inline(always)]
    pub fn mode16(&self) -> u32 {
        (self.value.read_bits() & Self::MODE16_MASK) >> Self::MODE16_OFFSET
    }

    /// Write **MODE16** `[1:0]` (RW)
    ///
    /// Port n.16 mode bits
    #[inline(always)]
    pub fn write_mode16(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE16_MASK) | ((value << Self::MODE16_OFFSET) & Self::MODE16_MASK)
        });
    }
}

/// CFGXR value type for batch read/modify/write operations
///
/// Use `cfgxr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgxr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CfgxrVal {
    bits: u32,
}

impl CfgxrVal {
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
        Self { bits: 0x44444444 }
    }

    /// Read **CNF23** `[31:30]`
    ///
    /// Port n.23 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf23(&self) -> u32 {
        (self.bits & Cfgxr::CNF23_MASK) >> Cfgxr::CNF23_OFFSET
    }

    /// Read **MODE23** `[29:28]`
    ///
    /// Port n.23 mode bits
    #[inline(always)]
    pub const fn mode23(&self) -> u32 {
        (self.bits & Cfgxr::MODE23_MASK) >> Cfgxr::MODE23_OFFSET
    }

    /// Read **CNF22** `[27:26]`
    ///
    /// Port n.22 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf22(&self) -> u32 {
        (self.bits & Cfgxr::CNF22_MASK) >> Cfgxr::CNF22_OFFSET
    }

    /// Read **MODE22** `[25:24]`
    ///
    /// Port n.22 mode bits
    #[inline(always)]
    pub const fn mode22(&self) -> u32 {
        (self.bits & Cfgxr::MODE22_MASK) >> Cfgxr::MODE22_OFFSET
    }

    /// Read **CNF21** `[23:22]`
    ///
    /// Port n.21 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf21(&self) -> u32 {
        (self.bits & Cfgxr::CNF21_MASK) >> Cfgxr::CNF21_OFFSET
    }

    /// Read **MODE21** `[21:20]`
    ///
    /// Port n.21 mode bits
    #[inline(always)]
    pub const fn mode21(&self) -> u32 {
        (self.bits & Cfgxr::MODE21_MASK) >> Cfgxr::MODE21_OFFSET
    }

    /// Read **CNF20** `[19:18]`
    ///
    /// Port n.20 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf20(&self) -> u32 {
        (self.bits & Cfgxr::CNF20_MASK) >> Cfgxr::CNF20_OFFSET
    }

    /// Read **MODE20** `[17:16]`
    ///
    /// Port n.20 mode bits
    #[inline(always)]
    pub const fn mode20(&self) -> u32 {
        (self.bits & Cfgxr::MODE20_MASK) >> Cfgxr::MODE20_OFFSET
    }

    /// Read **CNF19** `[15:14]`
    ///
    /// Port n.19 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf19(&self) -> u32 {
        (self.bits & Cfgxr::CNF19_MASK) >> Cfgxr::CNF19_OFFSET
    }

    /// Read **MODE19** `[13:12]`
    ///
    /// Port n.19 mode bits
    #[inline(always)]
    pub const fn mode19(&self) -> u32 {
        (self.bits & Cfgxr::MODE19_MASK) >> Cfgxr::MODE19_OFFSET
    }

    /// Read **CNF18** `[11:10]`
    ///
    /// Port n.18 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf18(&self) -> u32 {
        (self.bits & Cfgxr::CNF18_MASK) >> Cfgxr::CNF18_OFFSET
    }

    /// Read **MODE18** `[9:8]`
    ///
    /// Port n.18 mode bits
    #[inline(always)]
    pub const fn mode18(&self) -> u32 {
        (self.bits & Cfgxr::MODE18_MASK) >> Cfgxr::MODE18_OFFSET
    }

    /// Read **CNF17** `[7:6]`
    ///
    /// Port n.17 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf17(&self) -> u32 {
        (self.bits & Cfgxr::CNF17_MASK) >> Cfgxr::CNF17_OFFSET
    }

    /// Read **MODE17** `[5:4]`
    ///
    /// Port n.17 mode bits
    #[inline(always)]
    pub const fn mode17(&self) -> u32 {
        (self.bits & Cfgxr::MODE17_MASK) >> Cfgxr::MODE17_OFFSET
    }

    /// Read **CNF16** `[3:2]`
    ///
    /// Port n.16 configuration
    /// bits
    #[inline(always)]
    pub const fn cnf16(&self) -> u32 {
        (self.bits & Cfgxr::CNF16_MASK) >> Cfgxr::CNF16_OFFSET
    }

    /// Read **MODE16** `[1:0]`
    ///
    /// Port n.16 mode bits
    #[inline(always)]
    pub const fn mode16(&self) -> u32 {
        (self.bits & Cfgxr::MODE16_MASK) >> Cfgxr::MODE16_OFFSET
    }

    /// Set **CNF23** `[31:30]`
    ///
    /// Port n.23 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf23(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF23_MASK)
                | ((value << Cfgxr::CNF23_OFFSET) & Cfgxr::CNF23_MASK),
        }
    }

    /// Set **MODE23** `[29:28]`
    ///
    /// Port n.23 mode bits
    #[inline(always)]
    pub const fn with_mode23(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE23_MASK)
                | ((value << Cfgxr::MODE23_OFFSET) & Cfgxr::MODE23_MASK),
        }
    }

    /// Set **CNF22** `[27:26]`
    ///
    /// Port n.22 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf22(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF22_MASK)
                | ((value << Cfgxr::CNF22_OFFSET) & Cfgxr::CNF22_MASK),
        }
    }

    /// Set **MODE22** `[25:24]`
    ///
    /// Port n.22 mode bits
    #[inline(always)]
    pub const fn with_mode22(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE22_MASK)
                | ((value << Cfgxr::MODE22_OFFSET) & Cfgxr::MODE22_MASK),
        }
    }

    /// Set **CNF21** `[23:22]`
    ///
    /// Port n.21 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf21(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF21_MASK)
                | ((value << Cfgxr::CNF21_OFFSET) & Cfgxr::CNF21_MASK),
        }
    }

    /// Set **MODE21** `[21:20]`
    ///
    /// Port n.21 mode bits
    #[inline(always)]
    pub const fn with_mode21(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE21_MASK)
                | ((value << Cfgxr::MODE21_OFFSET) & Cfgxr::MODE21_MASK),
        }
    }

    /// Set **CNF20** `[19:18]`
    ///
    /// Port n.20 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf20(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF20_MASK)
                | ((value << Cfgxr::CNF20_OFFSET) & Cfgxr::CNF20_MASK),
        }
    }

    /// Set **MODE20** `[17:16]`
    ///
    /// Port n.20 mode bits
    #[inline(always)]
    pub const fn with_mode20(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE20_MASK)
                | ((value << Cfgxr::MODE20_OFFSET) & Cfgxr::MODE20_MASK),
        }
    }

    /// Set **CNF19** `[15:14]`
    ///
    /// Port n.19 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf19(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF19_MASK)
                | ((value << Cfgxr::CNF19_OFFSET) & Cfgxr::CNF19_MASK),
        }
    }

    /// Set **MODE19** `[13:12]`
    ///
    /// Port n.19 mode bits
    #[inline(always)]
    pub const fn with_mode19(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE19_MASK)
                | ((value << Cfgxr::MODE19_OFFSET) & Cfgxr::MODE19_MASK),
        }
    }

    /// Set **CNF18** `[11:10]`
    ///
    /// Port n.18 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf18(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF18_MASK)
                | ((value << Cfgxr::CNF18_OFFSET) & Cfgxr::CNF18_MASK),
        }
    }

    /// Set **MODE18** `[9:8]`
    ///
    /// Port n.18 mode bits
    #[inline(always)]
    pub const fn with_mode18(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE18_MASK)
                | ((value << Cfgxr::MODE18_OFFSET) & Cfgxr::MODE18_MASK),
        }
    }

    /// Set **CNF17** `[7:6]`
    ///
    /// Port n.17 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf17(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF17_MASK)
                | ((value << Cfgxr::CNF17_OFFSET) & Cfgxr::CNF17_MASK),
        }
    }

    /// Set **MODE17** `[5:4]`
    ///
    /// Port n.17 mode bits
    #[inline(always)]
    pub const fn with_mode17(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE17_MASK)
                | ((value << Cfgxr::MODE17_OFFSET) & Cfgxr::MODE17_MASK),
        }
    }

    /// Set **CNF16** `[3:2]`
    ///
    /// Port n.16 configuration
    /// bits
    #[inline(always)]
    pub const fn with_cnf16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::CNF16_MASK)
                | ((value << Cfgxr::CNF16_OFFSET) & Cfgxr::CNF16_MASK),
        }
    }

    /// Set **MODE16** `[1:0]`
    ///
    /// Port n.16 mode bits
    #[inline(always)]
    pub const fn with_mode16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cfgxr::MODE16_MASK)
                | ((value << Cfgxr::MODE16_OFFSET) & Cfgxr::MODE16_MASK),
        }
    }
}

impl Default for CfgxrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgxr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CfgxrVal {
        CfgxrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CfgxrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CfgxrVal) -> CfgxrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BSXR Register
///
/// Port bit set/reset register
/// (GPIOn_BSXR)
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BR23** `[23]` (WO)
///
///   Reset bit 23
///
/// - **BR22** `[22]` (WO)
///
///   Reset bit 22
///
/// - **BR21** `[21]` (WO)
///
///   Reset bit 21
///
/// - **BR20** `[20]` (WO)
///
///   Reset bit 20
///
/// - **BR19** `[19]` (WO)
///
///   Reset bit 19
///
/// - **BR18** `[18]` (WO)
///
///   Reset bit 18
///
/// - **BR17** `[17]` (WO)
///
///   Reset bit 17
///
/// - **BR16** `[16]` (WO)
///
///   Reset bit 16
///
/// - **BS23** `[7]` (WO)
///
///   Set bit 23
///
/// - **BS22** `[6]` (WO)
///
///   Set bit 22
///
/// - **BS21** `[5]` (WO)
///
///   Set bit 21
///
/// - **BS20** `[4]` (WO)
///
///   Set bit 20
///
/// - **BS19** `[3]` (WO)
///
///   Set bit 19
///
/// - **BS18** `[2]` (WO)
///
///   Set bit 18
///
/// - **BS17** `[1]` (WO)
///
///   Set bit 17
///
/// - **BS16** `[0]` (WO)
///
///   Set bit 16
#[repr(transparent)]
pub struct Bsxr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Bsxr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bsxr {
    // ======== Field Constants ========

    /// **BR23**
    ///
    /// Reset bit 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const BR23_OFFSET: u32 = 23;
    pub const BR23_MASK: u32 = 0x00800000;
    pub const BR23: u32 = 1 << 23;

    /// **BR22**
    ///
    /// Reset bit 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const BR22_OFFSET: u32 = 22;
    pub const BR22_MASK: u32 = 0x00400000;
    pub const BR22: u32 = 1 << 22;

    /// **BR21**
    ///
    /// Reset bit 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const BR21_OFFSET: u32 = 21;
    pub const BR21_MASK: u32 = 0x00200000;
    pub const BR21: u32 = 1 << 21;

    /// **BR20**
    ///
    /// Reset bit 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const BR20_OFFSET: u32 = 20;
    pub const BR20_MASK: u32 = 0x00100000;
    pub const BR20: u32 = 1 << 20;

    /// **BR19**
    ///
    /// Reset bit 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const BR19_OFFSET: u32 = 19;
    pub const BR19_MASK: u32 = 0x00080000;
    pub const BR19: u32 = 1 << 19;

    /// **BR18**
    ///
    /// Reset bit 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const BR18_OFFSET: u32 = 18;
    pub const BR18_MASK: u32 = 0x00040000;
    pub const BR18: u32 = 1 << 18;

    /// **BR17**
    ///
    /// Reset bit 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const BR17_OFFSET: u32 = 17;
    pub const BR17_MASK: u32 = 0x00020000;
    pub const BR17: u32 = 1 << 17;

    /// **BR16**
    ///
    /// Reset bit 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const BR16_OFFSET: u32 = 16;
    pub const BR16_MASK: u32 = 0x00010000;
    pub const BR16: u32 = 1 << 16;

    /// **BS23**
    ///
    /// Set bit 23
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BS23_OFFSET: u32 = 7;
    pub const BS23_MASK: u32 = 0x00000080;
    pub const BS23: u32 = 1 << 7;

    /// **BS22**
    ///
    /// Set bit 22
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const BS22_OFFSET: u32 = 6;
    pub const BS22_MASK: u32 = 0x00000040;
    pub const BS22: u32 = 1 << 6;

    /// **BS21**
    ///
    /// Set bit 21
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const BS21_OFFSET: u32 = 5;
    pub const BS21_MASK: u32 = 0x00000020;
    pub const BS21: u32 = 1 << 5;

    /// **BS20**
    ///
    /// Set bit 20
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BS20_OFFSET: u32 = 4;
    pub const BS20_MASK: u32 = 0x00000010;
    pub const BS20: u32 = 1 << 4;

    /// **BS19**
    ///
    /// Set bit 19
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BS19_OFFSET: u32 = 3;
    pub const BS19_MASK: u32 = 0x00000008;
    pub const BS19: u32 = 1 << 3;

    /// **BS18**
    ///
    /// Set bit 18
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BS18_OFFSET: u32 = 2;
    pub const BS18_MASK: u32 = 0x00000004;
    pub const BS18: u32 = 1 << 2;

    /// **BS17**
    ///
    /// Set bit 17
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BS17_OFFSET: u32 = 1;
    pub const BS17_MASK: u32 = 0x00000002;
    pub const BS17: u32 = 1 << 1;

    /// **BS16**
    ///
    /// Set bit 16
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BS16_OFFSET: u32 = 0;
    pub const BS16_MASK: u32 = 0x00000001;
    pub const BS16: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **BR23** (WO)
    ///
    /// Reset bit 23
    #[inline(always)]
    pub fn trigger_br23(&self) {
        self.value.write_bits(Self::BR23_MASK);
    }

    /// Trigger **BR22** (WO)
    ///
    /// Reset bit 22
    #[inline(always)]
    pub fn trigger_br22(&self) {
        self.value.write_bits(Self::BR22_MASK);
    }

    /// Trigger **BR21** (WO)
    ///
    /// Reset bit 21
    #[inline(always)]
    pub fn trigger_br21(&self) {
        self.value.write_bits(Self::BR21_MASK);
    }

    /// Trigger **BR20** (WO)
    ///
    /// Reset bit 20
    #[inline(always)]
    pub fn trigger_br20(&self) {
        self.value.write_bits(Self::BR20_MASK);
    }

    /// Trigger **BR19** (WO)
    ///
    /// Reset bit 19
    #[inline(always)]
    pub fn trigger_br19(&self) {
        self.value.write_bits(Self::BR19_MASK);
    }

    /// Trigger **BR18** (WO)
    ///
    /// Reset bit 18
    #[inline(always)]
    pub fn trigger_br18(&self) {
        self.value.write_bits(Self::BR18_MASK);
    }

    /// Trigger **BR17** (WO)
    ///
    /// Reset bit 17
    #[inline(always)]
    pub fn trigger_br17(&self) {
        self.value.write_bits(Self::BR17_MASK);
    }

    /// Trigger **BR16** (WO)
    ///
    /// Reset bit 16
    #[inline(always)]
    pub fn trigger_br16(&self) {
        self.value.write_bits(Self::BR16_MASK);
    }

    /// Trigger **BS23** (WO)
    ///
    /// Set bit 23
    #[inline(always)]
    pub fn trigger_bs23(&self) {
        self.value.write_bits(Self::BS23_MASK);
    }

    /// Trigger **BS22** (WO)
    ///
    /// Set bit 22
    #[inline(always)]
    pub fn trigger_bs22(&self) {
        self.value.write_bits(Self::BS22_MASK);
    }

    /// Trigger **BS21** (WO)
    ///
    /// Set bit 21
    #[inline(always)]
    pub fn trigger_bs21(&self) {
        self.value.write_bits(Self::BS21_MASK);
    }

    /// Trigger **BS20** (WO)
    ///
    /// Set bit 20
    #[inline(always)]
    pub fn trigger_bs20(&self) {
        self.value.write_bits(Self::BS20_MASK);
    }

    /// Trigger **BS19** (WO)
    ///
    /// Set bit 19
    #[inline(always)]
    pub fn trigger_bs19(&self) {
        self.value.write_bits(Self::BS19_MASK);
    }

    /// Trigger **BS18** (WO)
    ///
    /// Set bit 18
    #[inline(always)]
    pub fn trigger_bs18(&self) {
        self.value.write_bits(Self::BS18_MASK);
    }

    /// Trigger **BS17** (WO)
    ///
    /// Set bit 17
    #[inline(always)]
    pub fn trigger_bs17(&self) {
        self.value.write_bits(Self::BS17_MASK);
    }

    /// Trigger **BS16** (WO)
    ///
    /// Set bit 16
    #[inline(always)]
    pub fn trigger_bs16(&self) {
        self.value.write_bits(Self::BS16_MASK);
    }
}

/// BSXR value type for batch read/modify/write operations
///
/// Use `bsxr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bsxr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BsxrVal {
    bits: u32,
}

impl BsxrVal {
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

    /// Set **BR23** `[23:23]`
    ///
    /// Reset bit 23
    #[inline(always)]
    pub const fn with_br23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR23_MASK
            } else {
                self.bits & !Bsxr::BR23_MASK
            },
        }
    }

    /// Set **BR22** `[22:22]`
    ///
    /// Reset bit 22
    #[inline(always)]
    pub const fn with_br22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR22_MASK
            } else {
                self.bits & !Bsxr::BR22_MASK
            },
        }
    }

    /// Set **BR21** `[21:21]`
    ///
    /// Reset bit 21
    #[inline(always)]
    pub const fn with_br21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR21_MASK
            } else {
                self.bits & !Bsxr::BR21_MASK
            },
        }
    }

    /// Set **BR20** `[20:20]`
    ///
    /// Reset bit 20
    #[inline(always)]
    pub const fn with_br20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR20_MASK
            } else {
                self.bits & !Bsxr::BR20_MASK
            },
        }
    }

    /// Set **BR19** `[19:19]`
    ///
    /// Reset bit 19
    #[inline(always)]
    pub const fn with_br19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR19_MASK
            } else {
                self.bits & !Bsxr::BR19_MASK
            },
        }
    }

    /// Set **BR18** `[18:18]`
    ///
    /// Reset bit 18
    #[inline(always)]
    pub const fn with_br18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR18_MASK
            } else {
                self.bits & !Bsxr::BR18_MASK
            },
        }
    }

    /// Set **BR17** `[17:17]`
    ///
    /// Reset bit 17
    #[inline(always)]
    pub const fn with_br17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR17_MASK
            } else {
                self.bits & !Bsxr::BR17_MASK
            },
        }
    }

    /// Set **BR16** `[16:16]`
    ///
    /// Reset bit 16
    #[inline(always)]
    pub const fn with_br16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BR16_MASK
            } else {
                self.bits & !Bsxr::BR16_MASK
            },
        }
    }

    /// Set **BS23** `[7:7]`
    ///
    /// Set bit 23
    #[inline(always)]
    pub const fn with_bs23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS23_MASK
            } else {
                self.bits & !Bsxr::BS23_MASK
            },
        }
    }

    /// Set **BS22** `[6:6]`
    ///
    /// Set bit 22
    #[inline(always)]
    pub const fn with_bs22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS22_MASK
            } else {
                self.bits & !Bsxr::BS22_MASK
            },
        }
    }

    /// Set **BS21** `[5:5]`
    ///
    /// Set bit 21
    #[inline(always)]
    pub const fn with_bs21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS21_MASK
            } else {
                self.bits & !Bsxr::BS21_MASK
            },
        }
    }

    /// Set **BS20** `[4:4]`
    ///
    /// Set bit 20
    #[inline(always)]
    pub const fn with_bs20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS20_MASK
            } else {
                self.bits & !Bsxr::BS20_MASK
            },
        }
    }

    /// Set **BS19** `[3:3]`
    ///
    /// Set bit 19
    #[inline(always)]
    pub const fn with_bs19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS19_MASK
            } else {
                self.bits & !Bsxr::BS19_MASK
            },
        }
    }

    /// Set **BS18** `[2:2]`
    ///
    /// Set bit 18
    #[inline(always)]
    pub const fn with_bs18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS18_MASK
            } else {
                self.bits & !Bsxr::BS18_MASK
            },
        }
    }

    /// Set **BS17** `[1:1]`
    ///
    /// Set bit 17
    #[inline(always)]
    pub const fn with_bs17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS17_MASK
            } else {
                self.bits & !Bsxr::BS17_MASK
            },
        }
    }

    /// Set **BS16** `[0:0]`
    ///
    /// Set bit 16
    #[inline(always)]
    pub const fn with_bs16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bsxr::BS16_MASK
            } else {
                self.bits & !Bsxr::BS16_MASK
            },
        }
    }
}

impl Default for BsxrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bsxr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BsxrVal) {
        self.value.write_bits(val.bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Port configuration register low
    /// (GPIOn_CFGLR)
    pub cfglr: Cfglr,
    /// 0x0004
    /// Port configuration register high
    /// (GPIOn_CFGHR)
    pub cfghr: Cfghr,
    /// 0x0008
    /// Port input data register
    /// (GPIOn_INDR)
    pub indr: Indr,
    /// 0x000c
    /// Port output data register
    /// (GPIOn_OUTDR)
    pub outdr: Outdr,
    /// 0x0010
    /// Port bit set/reset register
    /// (GPIOn_BSHR)
    pub bshr: Bshr,
    /// 0x0014
    /// Port bit reset register
    /// (GPIOn_BCR)
    pub bcr: Bcr,
    /// 0x0018
    /// Port configuration lock
    /// register
    pub lckr: Lckr,
    /// 0x001c
    /// Port configuration register extension
    /// (GPIOn_CFGXR)
    pub cfgxr: Cfgxr,
    /// 0x0020
    /// Port bit set/reset register
    /// (GPIOn_BSXR)
    pub bsxr: Bsxr,
}
