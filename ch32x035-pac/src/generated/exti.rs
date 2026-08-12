//! EXTI
//! EXTI

#![allow(dead_code)]

pub const BASE: u32 = 0x40010400;

/// INTENR Register
///
/// Interrupt mask register
/// (EXTI_INTENR)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MR29** `[29]` (RW)
///
///   Interrupt Mask on line 29
///
/// - **MR28** `[28]` (RW)
///
///   Interrupt Mask on line 28
///
/// - **MR27** `[27]` (RW)
///
///   Interrupt Mask on line 27
///
/// - **MR26** `[26]` (RW)
///
///   Interrupt Mask on line 26
///
/// - **MR25** `[25]` (RW)
///
///   Interrupt Mask on line 25
///
/// - **MR24** `[24]` (RW)
///
///   Interrupt Mask on line 24
///
/// - **MR23** `[23]` (RW)
///
///   Interrupt Mask on line 23
///
/// - **MR22** `[22]` (RW)
///
///   Interrupt Mask on line 22
///
/// - **MR21** `[21]` (RW)
///
///   Interrupt Mask on line 21
///
/// - **MR20** `[20]` (RW)
///
///   Interrupt Mask on line 20
///
/// - **MR19** `[19]` (RW)
///
///   Interrupt Mask on line 19
///
/// - **MR18** `[18]` (RW)
///
///   Interrupt Mask on line 18
///
/// - **MR17** `[17]` (RW)
///
///   Interrupt Mask on line 17
///
/// - **MR16** `[16]` (RW)
///
///   Interrupt Mask on line 16
///
/// - **MR15** `[15]` (RW)
///
///   Interrupt Mask on line 15
///
/// - **MR14** `[14]` (RW)
///
///   Interrupt Mask on line 14
///
/// - **MR13** `[13]` (RW)
///
///   Interrupt Mask on line 13
///
/// - **MR12** `[12]` (RW)
///
///   Interrupt Mask on line 12
///
/// - **MR11** `[11]` (RW)
///
///   Interrupt Mask on line 11
///
/// - **MR10** `[10]` (RW)
///
///   Interrupt Mask on line 10
///
/// - **MR9** `[9]` (RW)
///
///   Interrupt Mask on line 9
///
/// - **MR8** `[8]` (RW)
///
///   Interrupt Mask on line 8
///
/// - **MR7** `[7]` (RW)
///
///   Interrupt Mask on line 7
///
/// - **MR6** `[6]` (RW)
///
///   Interrupt Mask on line 6
///
/// - **MR5** `[5]` (RW)
///
///   Interrupt Mask on line 5
///
/// - **MR4** `[4]` (RW)
///
///   Interrupt Mask on line 4
///
/// - **MR3** `[3]` (RW)
///
///   Interrupt Mask on line 3
///
/// - **MR2** `[2]` (RW)
///
///   Interrupt Mask on line 2
///
/// - **MR1** `[1]` (RW)
///
///   Interrupt Mask on line 1
///
/// - **MR0** `[0]` (RW)
///
///   Interrupt Mask on line 0
#[repr(transparent)]
pub struct Intenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intenr {
    // ======== Field Constants ========

    /// **MR29**
    ///
    /// Interrupt Mask on line 29
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const MR29_OFFSET: u32 = 29;
    pub const MR29_MASK: u32 = 0x20000000;
    pub const MR29: u32 = 1 << 29;

    /// **MR28**
    ///
    /// Interrupt Mask on line 28
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const MR28_OFFSET: u32 = 28;
    pub const MR28_MASK: u32 = 0x10000000;
    pub const MR28: u32 = 1 << 28;

    /// **MR27**
    ///
    /// Interrupt Mask on line 27
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const MR27_OFFSET: u32 = 27;
    pub const MR27_MASK: u32 = 0x08000000;
    pub const MR27: u32 = 1 << 27;

    /// **MR26**
    ///
    /// Interrupt Mask on line 26
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MR26_OFFSET: u32 = 26;
    pub const MR26_MASK: u32 = 0x04000000;
    pub const MR26: u32 = 1 << 26;

    /// **MR25**
    ///
    /// Interrupt Mask on line 25
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MR25_OFFSET: u32 = 25;
    pub const MR25_MASK: u32 = 0x02000000;
    pub const MR25: u32 = 1 << 25;

    /// **MR24**
    ///
    /// Interrupt Mask on line 24
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const MR24_OFFSET: u32 = 24;
    pub const MR24_MASK: u32 = 0x01000000;
    pub const MR24: u32 = 1 << 24;

    /// **MR23**
    ///
    /// Interrupt Mask on line 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const MR23_OFFSET: u32 = 23;
    pub const MR23_MASK: u32 = 0x00800000;
    pub const MR23: u32 = 1 << 23;

    /// **MR22**
    ///
    /// Interrupt Mask on line 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MR22_OFFSET: u32 = 22;
    pub const MR22_MASK: u32 = 0x00400000;
    pub const MR22: u32 = 1 << 22;

    /// **MR21**
    ///
    /// Interrupt Mask on line 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const MR21_OFFSET: u32 = 21;
    pub const MR21_MASK: u32 = 0x00200000;
    pub const MR21: u32 = 1 << 21;

    /// **MR20**
    ///
    /// Interrupt Mask on line 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const MR20_OFFSET: u32 = 20;
    pub const MR20_MASK: u32 = 0x00100000;
    pub const MR20: u32 = 1 << 20;

    /// **MR19**
    ///
    /// Interrupt Mask on line 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const MR19_OFFSET: u32 = 19;
    pub const MR19_MASK: u32 = 0x00080000;
    pub const MR19: u32 = 1 << 19;

    /// **MR18**
    ///
    /// Interrupt Mask on line 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const MR18_OFFSET: u32 = 18;
    pub const MR18_MASK: u32 = 0x00040000;
    pub const MR18: u32 = 1 << 18;

    /// **MR17**
    ///
    /// Interrupt Mask on line 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MR17_OFFSET: u32 = 17;
    pub const MR17_MASK: u32 = 0x00020000;
    pub const MR17: u32 = 1 << 17;

    /// **MR16**
    ///
    /// Interrupt Mask on line 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MR16_OFFSET: u32 = 16;
    pub const MR16_MASK: u32 = 0x00010000;
    pub const MR16: u32 = 1 << 16;

    /// **MR15**
    ///
    /// Interrupt Mask on line 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MR15_OFFSET: u32 = 15;
    pub const MR15_MASK: u32 = 0x00008000;
    pub const MR15: u32 = 1 << 15;

    /// **MR14**
    ///
    /// Interrupt Mask on line 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MR14_OFFSET: u32 = 14;
    pub const MR14_MASK: u32 = 0x00004000;
    pub const MR14: u32 = 1 << 14;

    /// **MR13**
    ///
    /// Interrupt Mask on line 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MR13_OFFSET: u32 = 13;
    pub const MR13_MASK: u32 = 0x00002000;
    pub const MR13: u32 = 1 << 13;

    /// **MR12**
    ///
    /// Interrupt Mask on line 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MR12_OFFSET: u32 = 12;
    pub const MR12_MASK: u32 = 0x00001000;
    pub const MR12: u32 = 1 << 12;

    /// **MR11**
    ///
    /// Interrupt Mask on line 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MR11_OFFSET: u32 = 11;
    pub const MR11_MASK: u32 = 0x00000800;
    pub const MR11: u32 = 1 << 11;

    /// **MR10**
    ///
    /// Interrupt Mask on line 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MR10_OFFSET: u32 = 10;
    pub const MR10_MASK: u32 = 0x00000400;
    pub const MR10: u32 = 1 << 10;

    /// **MR9**
    ///
    /// Interrupt Mask on line 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MR9_OFFSET: u32 = 9;
    pub const MR9_MASK: u32 = 0x00000200;
    pub const MR9: u32 = 1 << 9;

    /// **MR8**
    ///
    /// Interrupt Mask on line 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MR8_OFFSET: u32 = 8;
    pub const MR8_MASK: u32 = 0x00000100;
    pub const MR8: u32 = 1 << 8;

    /// **MR7**
    ///
    /// Interrupt Mask on line 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MR7_OFFSET: u32 = 7;
    pub const MR7_MASK: u32 = 0x00000080;
    pub const MR7: u32 = 1 << 7;

    /// **MR6**
    ///
    /// Interrupt Mask on line 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MR6_OFFSET: u32 = 6;
    pub const MR6_MASK: u32 = 0x00000040;
    pub const MR6: u32 = 1 << 6;

    /// **MR5**
    ///
    /// Interrupt Mask on line 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MR5_OFFSET: u32 = 5;
    pub const MR5_MASK: u32 = 0x00000020;
    pub const MR5: u32 = 1 << 5;

    /// **MR4**
    ///
    /// Interrupt Mask on line 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MR4_OFFSET: u32 = 4;
    pub const MR4_MASK: u32 = 0x00000010;
    pub const MR4: u32 = 1 << 4;

    /// **MR3**
    ///
    /// Interrupt Mask on line 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MR3_OFFSET: u32 = 3;
    pub const MR3_MASK: u32 = 0x00000008;
    pub const MR3: u32 = 1 << 3;

    /// **MR2**
    ///
    /// Interrupt Mask on line 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MR2_OFFSET: u32 = 2;
    pub const MR2_MASK: u32 = 0x00000004;
    pub const MR2: u32 = 1 << 2;

    /// **MR1**
    ///
    /// Interrupt Mask on line 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MR1_OFFSET: u32 = 1;
    pub const MR1_MASK: u32 = 0x00000002;
    pub const MR1: u32 = 1 << 1;

    /// **MR0**
    ///
    /// Interrupt Mask on line 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MR0_OFFSET: u32 = 0;
    pub const MR0_MASK: u32 = 0x00000001;
    pub const MR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MR29** `[29]` (RW)
    ///
    /// Interrupt Mask on line 29
    #[inline(always)]
    pub fn mr29(&self) -> bool {
        (self.value.read_bits() & Self::MR29_MASK) != 0
    }

    /// Set MR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr29(&self) {
        self.value.modify_bits(|r| r | Self::MR29_MASK);
    }

    /// Clear MR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr29(&self) {
        self.value.modify_bits(|r| r & !Self::MR29_MASK);
    }

    /// **MR28** `[28]` (RW)
    ///
    /// Interrupt Mask on line 28
    #[inline(always)]
    pub fn mr28(&self) -> bool {
        (self.value.read_bits() & Self::MR28_MASK) != 0
    }

    /// Set MR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr28(&self) {
        self.value.modify_bits(|r| r | Self::MR28_MASK);
    }

    /// Clear MR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr28(&self) {
        self.value.modify_bits(|r| r & !Self::MR28_MASK);
    }

    /// **MR27** `[27]` (RW)
    ///
    /// Interrupt Mask on line 27
    #[inline(always)]
    pub fn mr27(&self) -> bool {
        (self.value.read_bits() & Self::MR27_MASK) != 0
    }

    /// Set MR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr27(&self) {
        self.value.modify_bits(|r| r | Self::MR27_MASK);
    }

    /// Clear MR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr27(&self) {
        self.value.modify_bits(|r| r & !Self::MR27_MASK);
    }

    /// **MR26** `[26]` (RW)
    ///
    /// Interrupt Mask on line 26
    #[inline(always)]
    pub fn mr26(&self) -> bool {
        (self.value.read_bits() & Self::MR26_MASK) != 0
    }

    /// Set MR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr26(&self) {
        self.value.modify_bits(|r| r | Self::MR26_MASK);
    }

    /// Clear MR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr26(&self) {
        self.value.modify_bits(|r| r & !Self::MR26_MASK);
    }

    /// **MR25** `[25]` (RW)
    ///
    /// Interrupt Mask on line 25
    #[inline(always)]
    pub fn mr25(&self) -> bool {
        (self.value.read_bits() & Self::MR25_MASK) != 0
    }

    /// Set MR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr25(&self) {
        self.value.modify_bits(|r| r | Self::MR25_MASK);
    }

    /// Clear MR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr25(&self) {
        self.value.modify_bits(|r| r & !Self::MR25_MASK);
    }

    /// **MR24** `[24]` (RW)
    ///
    /// Interrupt Mask on line 24
    #[inline(always)]
    pub fn mr24(&self) -> bool {
        (self.value.read_bits() & Self::MR24_MASK) != 0
    }

    /// Set MR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr24(&self) {
        self.value.modify_bits(|r| r | Self::MR24_MASK);
    }

    /// Clear MR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr24(&self) {
        self.value.modify_bits(|r| r & !Self::MR24_MASK);
    }

    /// **MR23** `[23]` (RW)
    ///
    /// Interrupt Mask on line 23
    #[inline(always)]
    pub fn mr23(&self) -> bool {
        (self.value.read_bits() & Self::MR23_MASK) != 0
    }

    /// Set MR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr23(&self) {
        self.value.modify_bits(|r| r | Self::MR23_MASK);
    }

    /// Clear MR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr23(&self) {
        self.value.modify_bits(|r| r & !Self::MR23_MASK);
    }

    /// **MR22** `[22]` (RW)
    ///
    /// Interrupt Mask on line 22
    #[inline(always)]
    pub fn mr22(&self) -> bool {
        (self.value.read_bits() & Self::MR22_MASK) != 0
    }

    /// Set MR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr22(&self) {
        self.value.modify_bits(|r| r | Self::MR22_MASK);
    }

    /// Clear MR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr22(&self) {
        self.value.modify_bits(|r| r & !Self::MR22_MASK);
    }

    /// **MR21** `[21]` (RW)
    ///
    /// Interrupt Mask on line 21
    #[inline(always)]
    pub fn mr21(&self) -> bool {
        (self.value.read_bits() & Self::MR21_MASK) != 0
    }

    /// Set MR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr21(&self) {
        self.value.modify_bits(|r| r | Self::MR21_MASK);
    }

    /// Clear MR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr21(&self) {
        self.value.modify_bits(|r| r & !Self::MR21_MASK);
    }

    /// **MR20** `[20]` (RW)
    ///
    /// Interrupt Mask on line 20
    #[inline(always)]
    pub fn mr20(&self) -> bool {
        (self.value.read_bits() & Self::MR20_MASK) != 0
    }

    /// Set MR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr20(&self) {
        self.value.modify_bits(|r| r | Self::MR20_MASK);
    }

    /// Clear MR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr20(&self) {
        self.value.modify_bits(|r| r & !Self::MR20_MASK);
    }

    /// **MR19** `[19]` (RW)
    ///
    /// Interrupt Mask on line 19
    #[inline(always)]
    pub fn mr19(&self) -> bool {
        (self.value.read_bits() & Self::MR19_MASK) != 0
    }

    /// Set MR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr19(&self) {
        self.value.modify_bits(|r| r | Self::MR19_MASK);
    }

    /// Clear MR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr19(&self) {
        self.value.modify_bits(|r| r & !Self::MR19_MASK);
    }

    /// **MR18** `[18]` (RW)
    ///
    /// Interrupt Mask on line 18
    #[inline(always)]
    pub fn mr18(&self) -> bool {
        (self.value.read_bits() & Self::MR18_MASK) != 0
    }

    /// Set MR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr18(&self) {
        self.value.modify_bits(|r| r | Self::MR18_MASK);
    }

    /// Clear MR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr18(&self) {
        self.value.modify_bits(|r| r & !Self::MR18_MASK);
    }

    /// **MR17** `[17]` (RW)
    ///
    /// Interrupt Mask on line 17
    #[inline(always)]
    pub fn mr17(&self) -> bool {
        (self.value.read_bits() & Self::MR17_MASK) != 0
    }

    /// Set MR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr17(&self) {
        self.value.modify_bits(|r| r | Self::MR17_MASK);
    }

    /// Clear MR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr17(&self) {
        self.value.modify_bits(|r| r & !Self::MR17_MASK);
    }

    /// **MR16** `[16]` (RW)
    ///
    /// Interrupt Mask on line 16
    #[inline(always)]
    pub fn mr16(&self) -> bool {
        (self.value.read_bits() & Self::MR16_MASK) != 0
    }

    /// Set MR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr16(&self) {
        self.value.modify_bits(|r| r | Self::MR16_MASK);
    }

    /// Clear MR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr16(&self) {
        self.value.modify_bits(|r| r & !Self::MR16_MASK);
    }

    /// **MR15** `[15]` (RW)
    ///
    /// Interrupt Mask on line 15
    #[inline(always)]
    pub fn mr15(&self) -> bool {
        (self.value.read_bits() & Self::MR15_MASK) != 0
    }

    /// Set MR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr15(&self) {
        self.value.modify_bits(|r| r | Self::MR15_MASK);
    }

    /// Clear MR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr15(&self) {
        self.value.modify_bits(|r| r & !Self::MR15_MASK);
    }

    /// **MR14** `[14]` (RW)
    ///
    /// Interrupt Mask on line 14
    #[inline(always)]
    pub fn mr14(&self) -> bool {
        (self.value.read_bits() & Self::MR14_MASK) != 0
    }

    /// Set MR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr14(&self) {
        self.value.modify_bits(|r| r | Self::MR14_MASK);
    }

    /// Clear MR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr14(&self) {
        self.value.modify_bits(|r| r & !Self::MR14_MASK);
    }

    /// **MR13** `[13]` (RW)
    ///
    /// Interrupt Mask on line 13
    #[inline(always)]
    pub fn mr13(&self) -> bool {
        (self.value.read_bits() & Self::MR13_MASK) != 0
    }

    /// Set MR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr13(&self) {
        self.value.modify_bits(|r| r | Self::MR13_MASK);
    }

    /// Clear MR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr13(&self) {
        self.value.modify_bits(|r| r & !Self::MR13_MASK);
    }

    /// **MR12** `[12]` (RW)
    ///
    /// Interrupt Mask on line 12
    #[inline(always)]
    pub fn mr12(&self) -> bool {
        (self.value.read_bits() & Self::MR12_MASK) != 0
    }

    /// Set MR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr12(&self) {
        self.value.modify_bits(|r| r | Self::MR12_MASK);
    }

    /// Clear MR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr12(&self) {
        self.value.modify_bits(|r| r & !Self::MR12_MASK);
    }

    /// **MR11** `[11]` (RW)
    ///
    /// Interrupt Mask on line 11
    #[inline(always)]
    pub fn mr11(&self) -> bool {
        (self.value.read_bits() & Self::MR11_MASK) != 0
    }

    /// Set MR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr11(&self) {
        self.value.modify_bits(|r| r | Self::MR11_MASK);
    }

    /// Clear MR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr11(&self) {
        self.value.modify_bits(|r| r & !Self::MR11_MASK);
    }

    /// **MR10** `[10]` (RW)
    ///
    /// Interrupt Mask on line 10
    #[inline(always)]
    pub fn mr10(&self) -> bool {
        (self.value.read_bits() & Self::MR10_MASK) != 0
    }

    /// Set MR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr10(&self) {
        self.value.modify_bits(|r| r | Self::MR10_MASK);
    }

    /// Clear MR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr10(&self) {
        self.value.modify_bits(|r| r & !Self::MR10_MASK);
    }

    /// **MR9** `[9]` (RW)
    ///
    /// Interrupt Mask on line 9
    #[inline(always)]
    pub fn mr9(&self) -> bool {
        (self.value.read_bits() & Self::MR9_MASK) != 0
    }

    /// Set MR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr9(&self) {
        self.value.modify_bits(|r| r | Self::MR9_MASK);
    }

    /// Clear MR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr9(&self) {
        self.value.modify_bits(|r| r & !Self::MR9_MASK);
    }

    /// **MR8** `[8]` (RW)
    ///
    /// Interrupt Mask on line 8
    #[inline(always)]
    pub fn mr8(&self) -> bool {
        (self.value.read_bits() & Self::MR8_MASK) != 0
    }

    /// Set MR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr8(&self) {
        self.value.modify_bits(|r| r | Self::MR8_MASK);
    }

    /// Clear MR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr8(&self) {
        self.value.modify_bits(|r| r & !Self::MR8_MASK);
    }

    /// **MR7** `[7]` (RW)
    ///
    /// Interrupt Mask on line 7
    #[inline(always)]
    pub fn mr7(&self) -> bool {
        (self.value.read_bits() & Self::MR7_MASK) != 0
    }

    /// Set MR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr7(&self) {
        self.value.modify_bits(|r| r | Self::MR7_MASK);
    }

    /// Clear MR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr7(&self) {
        self.value.modify_bits(|r| r & !Self::MR7_MASK);
    }

    /// **MR6** `[6]` (RW)
    ///
    /// Interrupt Mask on line 6
    #[inline(always)]
    pub fn mr6(&self) -> bool {
        (self.value.read_bits() & Self::MR6_MASK) != 0
    }

    /// Set MR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr6(&self) {
        self.value.modify_bits(|r| r | Self::MR6_MASK);
    }

    /// Clear MR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr6(&self) {
        self.value.modify_bits(|r| r & !Self::MR6_MASK);
    }

    /// **MR5** `[5]` (RW)
    ///
    /// Interrupt Mask on line 5
    #[inline(always)]
    pub fn mr5(&self) -> bool {
        (self.value.read_bits() & Self::MR5_MASK) != 0
    }

    /// Set MR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr5(&self) {
        self.value.modify_bits(|r| r | Self::MR5_MASK);
    }

    /// Clear MR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr5(&self) {
        self.value.modify_bits(|r| r & !Self::MR5_MASK);
    }

    /// **MR4** `[4]` (RW)
    ///
    /// Interrupt Mask on line 4
    #[inline(always)]
    pub fn mr4(&self) -> bool {
        (self.value.read_bits() & Self::MR4_MASK) != 0
    }

    /// Set MR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr4(&self) {
        self.value.modify_bits(|r| r | Self::MR4_MASK);
    }

    /// Clear MR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr4(&self) {
        self.value.modify_bits(|r| r & !Self::MR4_MASK);
    }

    /// **MR3** `[3]` (RW)
    ///
    /// Interrupt Mask on line 3
    #[inline(always)]
    pub fn mr3(&self) -> bool {
        (self.value.read_bits() & Self::MR3_MASK) != 0
    }

    /// Set MR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr3(&self) {
        self.value.modify_bits(|r| r | Self::MR3_MASK);
    }

    /// Clear MR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr3(&self) {
        self.value.modify_bits(|r| r & !Self::MR3_MASK);
    }

    /// **MR2** `[2]` (RW)
    ///
    /// Interrupt Mask on line 2
    #[inline(always)]
    pub fn mr2(&self) -> bool {
        (self.value.read_bits() & Self::MR2_MASK) != 0
    }

    /// Set MR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr2(&self) {
        self.value.modify_bits(|r| r | Self::MR2_MASK);
    }

    /// Clear MR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr2(&self) {
        self.value.modify_bits(|r| r & !Self::MR2_MASK);
    }

    /// **MR1** `[1]` (RW)
    ///
    /// Interrupt Mask on line 1
    #[inline(always)]
    pub fn mr1(&self) -> bool {
        (self.value.read_bits() & Self::MR1_MASK) != 0
    }

    /// Set MR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr1(&self) {
        self.value.modify_bits(|r| r | Self::MR1_MASK);
    }

    /// Clear MR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr1(&self) {
        self.value.modify_bits(|r| r & !Self::MR1_MASK);
    }

    /// **MR0** `[0]` (RW)
    ///
    /// Interrupt Mask on line 0
    #[inline(always)]
    pub fn mr0(&self) -> bool {
        (self.value.read_bits() & Self::MR0_MASK) != 0
    }

    /// Set MR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr0(&self) {
        self.value.modify_bits(|r| r | Self::MR0_MASK);
    }

    /// Clear MR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr0(&self) {
        self.value.modify_bits(|r| r & !Self::MR0_MASK);
    }
}

/// INTENR value type for batch read/modify/write operations
///
/// Use `intenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntenrVal {
    bits: u32,
}

impl IntenrVal {
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

    /// Read **MR29** `[29:29]`
    ///
    /// Interrupt Mask on line 29
    #[inline(always)]
    pub const fn mr29(&self) -> bool {
        (self.bits & Intenr::MR29_MASK) != 0
    }

    /// Read **MR28** `[28:28]`
    ///
    /// Interrupt Mask on line 28
    #[inline(always)]
    pub const fn mr28(&self) -> bool {
        (self.bits & Intenr::MR28_MASK) != 0
    }

    /// Read **MR27** `[27:27]`
    ///
    /// Interrupt Mask on line 27
    #[inline(always)]
    pub const fn mr27(&self) -> bool {
        (self.bits & Intenr::MR27_MASK) != 0
    }

    /// Read **MR26** `[26:26]`
    ///
    /// Interrupt Mask on line 26
    #[inline(always)]
    pub const fn mr26(&self) -> bool {
        (self.bits & Intenr::MR26_MASK) != 0
    }

    /// Read **MR25** `[25:25]`
    ///
    /// Interrupt Mask on line 25
    #[inline(always)]
    pub const fn mr25(&self) -> bool {
        (self.bits & Intenr::MR25_MASK) != 0
    }

    /// Read **MR24** `[24:24]`
    ///
    /// Interrupt Mask on line 24
    #[inline(always)]
    pub const fn mr24(&self) -> bool {
        (self.bits & Intenr::MR24_MASK) != 0
    }

    /// Read **MR23** `[23:23]`
    ///
    /// Interrupt Mask on line 23
    #[inline(always)]
    pub const fn mr23(&self) -> bool {
        (self.bits & Intenr::MR23_MASK) != 0
    }

    /// Read **MR22** `[22:22]`
    ///
    /// Interrupt Mask on line 22
    #[inline(always)]
    pub const fn mr22(&self) -> bool {
        (self.bits & Intenr::MR22_MASK) != 0
    }

    /// Read **MR21** `[21:21]`
    ///
    /// Interrupt Mask on line 21
    #[inline(always)]
    pub const fn mr21(&self) -> bool {
        (self.bits & Intenr::MR21_MASK) != 0
    }

    /// Read **MR20** `[20:20]`
    ///
    /// Interrupt Mask on line 20
    #[inline(always)]
    pub const fn mr20(&self) -> bool {
        (self.bits & Intenr::MR20_MASK) != 0
    }

    /// Read **MR19** `[19:19]`
    ///
    /// Interrupt Mask on line 19
    #[inline(always)]
    pub const fn mr19(&self) -> bool {
        (self.bits & Intenr::MR19_MASK) != 0
    }

    /// Read **MR18** `[18:18]`
    ///
    /// Interrupt Mask on line 18
    #[inline(always)]
    pub const fn mr18(&self) -> bool {
        (self.bits & Intenr::MR18_MASK) != 0
    }

    /// Read **MR17** `[17:17]`
    ///
    /// Interrupt Mask on line 17
    #[inline(always)]
    pub const fn mr17(&self) -> bool {
        (self.bits & Intenr::MR17_MASK) != 0
    }

    /// Read **MR16** `[16:16]`
    ///
    /// Interrupt Mask on line 16
    #[inline(always)]
    pub const fn mr16(&self) -> bool {
        (self.bits & Intenr::MR16_MASK) != 0
    }

    /// Read **MR15** `[15:15]`
    ///
    /// Interrupt Mask on line 15
    #[inline(always)]
    pub const fn mr15(&self) -> bool {
        (self.bits & Intenr::MR15_MASK) != 0
    }

    /// Read **MR14** `[14:14]`
    ///
    /// Interrupt Mask on line 14
    #[inline(always)]
    pub const fn mr14(&self) -> bool {
        (self.bits & Intenr::MR14_MASK) != 0
    }

    /// Read **MR13** `[13:13]`
    ///
    /// Interrupt Mask on line 13
    #[inline(always)]
    pub const fn mr13(&self) -> bool {
        (self.bits & Intenr::MR13_MASK) != 0
    }

    /// Read **MR12** `[12:12]`
    ///
    /// Interrupt Mask on line 12
    #[inline(always)]
    pub const fn mr12(&self) -> bool {
        (self.bits & Intenr::MR12_MASK) != 0
    }

    /// Read **MR11** `[11:11]`
    ///
    /// Interrupt Mask on line 11
    #[inline(always)]
    pub const fn mr11(&self) -> bool {
        (self.bits & Intenr::MR11_MASK) != 0
    }

    /// Read **MR10** `[10:10]`
    ///
    /// Interrupt Mask on line 10
    #[inline(always)]
    pub const fn mr10(&self) -> bool {
        (self.bits & Intenr::MR10_MASK) != 0
    }

    /// Read **MR9** `[9:9]`
    ///
    /// Interrupt Mask on line 9
    #[inline(always)]
    pub const fn mr9(&self) -> bool {
        (self.bits & Intenr::MR9_MASK) != 0
    }

    /// Read **MR8** `[8:8]`
    ///
    /// Interrupt Mask on line 8
    #[inline(always)]
    pub const fn mr8(&self) -> bool {
        (self.bits & Intenr::MR8_MASK) != 0
    }

    /// Read **MR7** `[7:7]`
    ///
    /// Interrupt Mask on line 7
    #[inline(always)]
    pub const fn mr7(&self) -> bool {
        (self.bits & Intenr::MR7_MASK) != 0
    }

    /// Read **MR6** `[6:6]`
    ///
    /// Interrupt Mask on line 6
    #[inline(always)]
    pub const fn mr6(&self) -> bool {
        (self.bits & Intenr::MR6_MASK) != 0
    }

    /// Read **MR5** `[5:5]`
    ///
    /// Interrupt Mask on line 5
    #[inline(always)]
    pub const fn mr5(&self) -> bool {
        (self.bits & Intenr::MR5_MASK) != 0
    }

    /// Read **MR4** `[4:4]`
    ///
    /// Interrupt Mask on line 4
    #[inline(always)]
    pub const fn mr4(&self) -> bool {
        (self.bits & Intenr::MR4_MASK) != 0
    }

    /// Read **MR3** `[3:3]`
    ///
    /// Interrupt Mask on line 3
    #[inline(always)]
    pub const fn mr3(&self) -> bool {
        (self.bits & Intenr::MR3_MASK) != 0
    }

    /// Read **MR2** `[2:2]`
    ///
    /// Interrupt Mask on line 2
    #[inline(always)]
    pub const fn mr2(&self) -> bool {
        (self.bits & Intenr::MR2_MASK) != 0
    }

    /// Read **MR1** `[1:1]`
    ///
    /// Interrupt Mask on line 1
    #[inline(always)]
    pub const fn mr1(&self) -> bool {
        (self.bits & Intenr::MR1_MASK) != 0
    }

    /// Read **MR0** `[0:0]`
    ///
    /// Interrupt Mask on line 0
    #[inline(always)]
    pub const fn mr0(&self) -> bool {
        (self.bits & Intenr::MR0_MASK) != 0
    }

    /// Set **MR29** `[29:29]`
    ///
    /// Interrupt Mask on line 29
    #[inline(always)]
    pub const fn with_mr29(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR29_MASK
            } else {
                self.bits & !Intenr::MR29_MASK
            },
        }
    }

    /// Set **MR28** `[28:28]`
    ///
    /// Interrupt Mask on line 28
    #[inline(always)]
    pub const fn with_mr28(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR28_MASK
            } else {
                self.bits & !Intenr::MR28_MASK
            },
        }
    }

    /// Set **MR27** `[27:27]`
    ///
    /// Interrupt Mask on line 27
    #[inline(always)]
    pub const fn with_mr27(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR27_MASK
            } else {
                self.bits & !Intenr::MR27_MASK
            },
        }
    }

    /// Set **MR26** `[26:26]`
    ///
    /// Interrupt Mask on line 26
    #[inline(always)]
    pub const fn with_mr26(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR26_MASK
            } else {
                self.bits & !Intenr::MR26_MASK
            },
        }
    }

    /// Set **MR25** `[25:25]`
    ///
    /// Interrupt Mask on line 25
    #[inline(always)]
    pub const fn with_mr25(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR25_MASK
            } else {
                self.bits & !Intenr::MR25_MASK
            },
        }
    }

    /// Set **MR24** `[24:24]`
    ///
    /// Interrupt Mask on line 24
    #[inline(always)]
    pub const fn with_mr24(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR24_MASK
            } else {
                self.bits & !Intenr::MR24_MASK
            },
        }
    }

    /// Set **MR23** `[23:23]`
    ///
    /// Interrupt Mask on line 23
    #[inline(always)]
    pub const fn with_mr23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR23_MASK
            } else {
                self.bits & !Intenr::MR23_MASK
            },
        }
    }

    /// Set **MR22** `[22:22]`
    ///
    /// Interrupt Mask on line 22
    #[inline(always)]
    pub const fn with_mr22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR22_MASK
            } else {
                self.bits & !Intenr::MR22_MASK
            },
        }
    }

    /// Set **MR21** `[21:21]`
    ///
    /// Interrupt Mask on line 21
    #[inline(always)]
    pub const fn with_mr21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR21_MASK
            } else {
                self.bits & !Intenr::MR21_MASK
            },
        }
    }

    /// Set **MR20** `[20:20]`
    ///
    /// Interrupt Mask on line 20
    #[inline(always)]
    pub const fn with_mr20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR20_MASK
            } else {
                self.bits & !Intenr::MR20_MASK
            },
        }
    }

    /// Set **MR19** `[19:19]`
    ///
    /// Interrupt Mask on line 19
    #[inline(always)]
    pub const fn with_mr19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR19_MASK
            } else {
                self.bits & !Intenr::MR19_MASK
            },
        }
    }

    /// Set **MR18** `[18:18]`
    ///
    /// Interrupt Mask on line 18
    #[inline(always)]
    pub const fn with_mr18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR18_MASK
            } else {
                self.bits & !Intenr::MR18_MASK
            },
        }
    }

    /// Set **MR17** `[17:17]`
    ///
    /// Interrupt Mask on line 17
    #[inline(always)]
    pub const fn with_mr17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR17_MASK
            } else {
                self.bits & !Intenr::MR17_MASK
            },
        }
    }

    /// Set **MR16** `[16:16]`
    ///
    /// Interrupt Mask on line 16
    #[inline(always)]
    pub const fn with_mr16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR16_MASK
            } else {
                self.bits & !Intenr::MR16_MASK
            },
        }
    }

    /// Set **MR15** `[15:15]`
    ///
    /// Interrupt Mask on line 15
    #[inline(always)]
    pub const fn with_mr15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR15_MASK
            } else {
                self.bits & !Intenr::MR15_MASK
            },
        }
    }

    /// Set **MR14** `[14:14]`
    ///
    /// Interrupt Mask on line 14
    #[inline(always)]
    pub const fn with_mr14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR14_MASK
            } else {
                self.bits & !Intenr::MR14_MASK
            },
        }
    }

    /// Set **MR13** `[13:13]`
    ///
    /// Interrupt Mask on line 13
    #[inline(always)]
    pub const fn with_mr13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR13_MASK
            } else {
                self.bits & !Intenr::MR13_MASK
            },
        }
    }

    /// Set **MR12** `[12:12]`
    ///
    /// Interrupt Mask on line 12
    #[inline(always)]
    pub const fn with_mr12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR12_MASK
            } else {
                self.bits & !Intenr::MR12_MASK
            },
        }
    }

    /// Set **MR11** `[11:11]`
    ///
    /// Interrupt Mask on line 11
    #[inline(always)]
    pub const fn with_mr11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR11_MASK
            } else {
                self.bits & !Intenr::MR11_MASK
            },
        }
    }

    /// Set **MR10** `[10:10]`
    ///
    /// Interrupt Mask on line 10
    #[inline(always)]
    pub const fn with_mr10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR10_MASK
            } else {
                self.bits & !Intenr::MR10_MASK
            },
        }
    }

    /// Set **MR9** `[9:9]`
    ///
    /// Interrupt Mask on line 9
    #[inline(always)]
    pub const fn with_mr9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR9_MASK
            } else {
                self.bits & !Intenr::MR9_MASK
            },
        }
    }

    /// Set **MR8** `[8:8]`
    ///
    /// Interrupt Mask on line 8
    #[inline(always)]
    pub const fn with_mr8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR8_MASK
            } else {
                self.bits & !Intenr::MR8_MASK
            },
        }
    }

    /// Set **MR7** `[7:7]`
    ///
    /// Interrupt Mask on line 7
    #[inline(always)]
    pub const fn with_mr7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR7_MASK
            } else {
                self.bits & !Intenr::MR7_MASK
            },
        }
    }

    /// Set **MR6** `[6:6]`
    ///
    /// Interrupt Mask on line 6
    #[inline(always)]
    pub const fn with_mr6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR6_MASK
            } else {
                self.bits & !Intenr::MR6_MASK
            },
        }
    }

    /// Set **MR5** `[5:5]`
    ///
    /// Interrupt Mask on line 5
    #[inline(always)]
    pub const fn with_mr5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR5_MASK
            } else {
                self.bits & !Intenr::MR5_MASK
            },
        }
    }

    /// Set **MR4** `[4:4]`
    ///
    /// Interrupt Mask on line 4
    #[inline(always)]
    pub const fn with_mr4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR4_MASK
            } else {
                self.bits & !Intenr::MR4_MASK
            },
        }
    }

    /// Set **MR3** `[3:3]`
    ///
    /// Interrupt Mask on line 3
    #[inline(always)]
    pub const fn with_mr3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR3_MASK
            } else {
                self.bits & !Intenr::MR3_MASK
            },
        }
    }

    /// Set **MR2** `[2:2]`
    ///
    /// Interrupt Mask on line 2
    #[inline(always)]
    pub const fn with_mr2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR2_MASK
            } else {
                self.bits & !Intenr::MR2_MASK
            },
        }
    }

    /// Set **MR1** `[1:1]`
    ///
    /// Interrupt Mask on line 1
    #[inline(always)]
    pub const fn with_mr1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR1_MASK
            } else {
                self.bits & !Intenr::MR1_MASK
            },
        }
    }

    /// Set **MR0** `[0:0]`
    ///
    /// Interrupt Mask on line 0
    #[inline(always)]
    pub const fn with_mr0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intenr::MR0_MASK
            } else {
                self.bits & !Intenr::MR0_MASK
            },
        }
    }
}

impl Default for IntenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntenrVal {
        IntenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntenrVal) -> IntenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EVENR Register
///
/// Event mask register (EXTI_EVENR)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MR29** `[29]` (RW)
///
///   Event Mask on line 29
///
/// - **MR28** `[28]` (RW)
///
///   Event Mask on line 28
///
/// - **MR27** `[27]` (RW)
///
///   Event Mask on line 27
///
/// - **MR26** `[26]` (RW)
///
///   Event Mask on line 26
///
/// - **MR25** `[25]` (RW)
///
///   Event Mask on line 25
///
/// - **MR24** `[24]` (RW)
///
///   Event Mask on line 24
///
/// - **MR23** `[23]` (RW)
///
///   Event Mask on line 23
///
/// - **MR22** `[22]` (RW)
///
///   Event Mask on line 22
///
/// - **MR21** `[21]` (RW)
///
///   Event Mask on line 21
///
/// - **MR20** `[20]` (RW)
///
///   Event Mask on line 20
///
/// - **MR19** `[19]` (RW)
///
///   Event Mask on line 19
///
/// - **MR18** `[18]` (RW)
///
///   Event Mask on line 18
///
/// - **MR17** `[17]` (RW)
///
///   Event Mask on line 17
///
/// - **MR16** `[16]` (RW)
///
///   Event Mask on line 16
///
/// - **MR15** `[15]` (RW)
///
///   Event Mask on line 15
///
/// - **MR14** `[14]` (RW)
///
///   Event Mask on line 14
///
/// - **MR13** `[13]` (RW)
///
///   Event Mask on line 13
///
/// - **MR12** `[12]` (RW)
///
///   Event Mask on line 12
///
/// - **MR11** `[11]` (RW)
///
///   Event Mask on line 11
///
/// - **MR10** `[10]` (RW)
///
///   Event Mask on line 10
///
/// - **MR9** `[9]` (RW)
///
///   Event Mask on line 9
///
/// - **MR8** `[8]` (RW)
///
///   Event Mask on line 8
///
/// - **MR7** `[7]` (RW)
///
///   Event Mask on line 7
///
/// - **MR6** `[6]` (RW)
///
///   Event Mask on line 6
///
/// - **MR5** `[5]` (RW)
///
///   Event Mask on line 5
///
/// - **MR4** `[4]` (RW)
///
///   Event Mask on line 4
///
/// - **MR3** `[3]` (RW)
///
///   Event Mask on line 3
///
/// - **MR2** `[2]` (RW)
///
///   Event Mask on line 2
///
/// - **MR1** `[1]` (RW)
///
///   Event Mask on line 1
///
/// - **MR0** `[0]` (RW)
///
///   Event Mask on line 0
#[repr(transparent)]
pub struct Evenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Evenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Evenr {
    // ======== Field Constants ========

    /// **MR29**
    ///
    /// Event Mask on line 29
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const MR29_OFFSET: u32 = 29;
    pub const MR29_MASK: u32 = 0x20000000;
    pub const MR29: u32 = 1 << 29;

    /// **MR28**
    ///
    /// Event Mask on line 28
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const MR28_OFFSET: u32 = 28;
    pub const MR28_MASK: u32 = 0x10000000;
    pub const MR28: u32 = 1 << 28;

    /// **MR27**
    ///
    /// Event Mask on line 27
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const MR27_OFFSET: u32 = 27;
    pub const MR27_MASK: u32 = 0x08000000;
    pub const MR27: u32 = 1 << 27;

    /// **MR26**
    ///
    /// Event Mask on line 26
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MR26_OFFSET: u32 = 26;
    pub const MR26_MASK: u32 = 0x04000000;
    pub const MR26: u32 = 1 << 26;

    /// **MR25**
    ///
    /// Event Mask on line 25
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MR25_OFFSET: u32 = 25;
    pub const MR25_MASK: u32 = 0x02000000;
    pub const MR25: u32 = 1 << 25;

    /// **MR24**
    ///
    /// Event Mask on line 24
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const MR24_OFFSET: u32 = 24;
    pub const MR24_MASK: u32 = 0x01000000;
    pub const MR24: u32 = 1 << 24;

    /// **MR23**
    ///
    /// Event Mask on line 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const MR23_OFFSET: u32 = 23;
    pub const MR23_MASK: u32 = 0x00800000;
    pub const MR23: u32 = 1 << 23;

    /// **MR22**
    ///
    /// Event Mask on line 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MR22_OFFSET: u32 = 22;
    pub const MR22_MASK: u32 = 0x00400000;
    pub const MR22: u32 = 1 << 22;

    /// **MR21**
    ///
    /// Event Mask on line 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const MR21_OFFSET: u32 = 21;
    pub const MR21_MASK: u32 = 0x00200000;
    pub const MR21: u32 = 1 << 21;

    /// **MR20**
    ///
    /// Event Mask on line 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const MR20_OFFSET: u32 = 20;
    pub const MR20_MASK: u32 = 0x00100000;
    pub const MR20: u32 = 1 << 20;

    /// **MR19**
    ///
    /// Event Mask on line 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const MR19_OFFSET: u32 = 19;
    pub const MR19_MASK: u32 = 0x00080000;
    pub const MR19: u32 = 1 << 19;

    /// **MR18**
    ///
    /// Event Mask on line 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const MR18_OFFSET: u32 = 18;
    pub const MR18_MASK: u32 = 0x00040000;
    pub const MR18: u32 = 1 << 18;

    /// **MR17**
    ///
    /// Event Mask on line 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MR17_OFFSET: u32 = 17;
    pub const MR17_MASK: u32 = 0x00020000;
    pub const MR17: u32 = 1 << 17;

    /// **MR16**
    ///
    /// Event Mask on line 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MR16_OFFSET: u32 = 16;
    pub const MR16_MASK: u32 = 0x00010000;
    pub const MR16: u32 = 1 << 16;

    /// **MR15**
    ///
    /// Event Mask on line 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MR15_OFFSET: u32 = 15;
    pub const MR15_MASK: u32 = 0x00008000;
    pub const MR15: u32 = 1 << 15;

    /// **MR14**
    ///
    /// Event Mask on line 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MR14_OFFSET: u32 = 14;
    pub const MR14_MASK: u32 = 0x00004000;
    pub const MR14: u32 = 1 << 14;

    /// **MR13**
    ///
    /// Event Mask on line 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MR13_OFFSET: u32 = 13;
    pub const MR13_MASK: u32 = 0x00002000;
    pub const MR13: u32 = 1 << 13;

    /// **MR12**
    ///
    /// Event Mask on line 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MR12_OFFSET: u32 = 12;
    pub const MR12_MASK: u32 = 0x00001000;
    pub const MR12: u32 = 1 << 12;

    /// **MR11**
    ///
    /// Event Mask on line 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MR11_OFFSET: u32 = 11;
    pub const MR11_MASK: u32 = 0x00000800;
    pub const MR11: u32 = 1 << 11;

    /// **MR10**
    ///
    /// Event Mask on line 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MR10_OFFSET: u32 = 10;
    pub const MR10_MASK: u32 = 0x00000400;
    pub const MR10: u32 = 1 << 10;

    /// **MR9**
    ///
    /// Event Mask on line 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MR9_OFFSET: u32 = 9;
    pub const MR9_MASK: u32 = 0x00000200;
    pub const MR9: u32 = 1 << 9;

    /// **MR8**
    ///
    /// Event Mask on line 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MR8_OFFSET: u32 = 8;
    pub const MR8_MASK: u32 = 0x00000100;
    pub const MR8: u32 = 1 << 8;

    /// **MR7**
    ///
    /// Event Mask on line 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MR7_OFFSET: u32 = 7;
    pub const MR7_MASK: u32 = 0x00000080;
    pub const MR7: u32 = 1 << 7;

    /// **MR6**
    ///
    /// Event Mask on line 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MR6_OFFSET: u32 = 6;
    pub const MR6_MASK: u32 = 0x00000040;
    pub const MR6: u32 = 1 << 6;

    /// **MR5**
    ///
    /// Event Mask on line 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MR5_OFFSET: u32 = 5;
    pub const MR5_MASK: u32 = 0x00000020;
    pub const MR5: u32 = 1 << 5;

    /// **MR4**
    ///
    /// Event Mask on line 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MR4_OFFSET: u32 = 4;
    pub const MR4_MASK: u32 = 0x00000010;
    pub const MR4: u32 = 1 << 4;

    /// **MR3**
    ///
    /// Event Mask on line 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MR3_OFFSET: u32 = 3;
    pub const MR3_MASK: u32 = 0x00000008;
    pub const MR3: u32 = 1 << 3;

    /// **MR2**
    ///
    /// Event Mask on line 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MR2_OFFSET: u32 = 2;
    pub const MR2_MASK: u32 = 0x00000004;
    pub const MR2: u32 = 1 << 2;

    /// **MR1**
    ///
    /// Event Mask on line 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MR1_OFFSET: u32 = 1;
    pub const MR1_MASK: u32 = 0x00000002;
    pub const MR1: u32 = 1 << 1;

    /// **MR0**
    ///
    /// Event Mask on line 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MR0_OFFSET: u32 = 0;
    pub const MR0_MASK: u32 = 0x00000001;
    pub const MR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MR29** `[29]` (RW)
    ///
    /// Event Mask on line 29
    #[inline(always)]
    pub fn mr29(&self) -> bool {
        (self.value.read_bits() & Self::MR29_MASK) != 0
    }

    /// Set MR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr29(&self) {
        self.value.modify_bits(|r| r | Self::MR29_MASK);
    }

    /// Clear MR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr29(&self) {
        self.value.modify_bits(|r| r & !Self::MR29_MASK);
    }

    /// **MR28** `[28]` (RW)
    ///
    /// Event Mask on line 28
    #[inline(always)]
    pub fn mr28(&self) -> bool {
        (self.value.read_bits() & Self::MR28_MASK) != 0
    }

    /// Set MR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr28(&self) {
        self.value.modify_bits(|r| r | Self::MR28_MASK);
    }

    /// Clear MR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr28(&self) {
        self.value.modify_bits(|r| r & !Self::MR28_MASK);
    }

    /// **MR27** `[27]` (RW)
    ///
    /// Event Mask on line 27
    #[inline(always)]
    pub fn mr27(&self) -> bool {
        (self.value.read_bits() & Self::MR27_MASK) != 0
    }

    /// Set MR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr27(&self) {
        self.value.modify_bits(|r| r | Self::MR27_MASK);
    }

    /// Clear MR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr27(&self) {
        self.value.modify_bits(|r| r & !Self::MR27_MASK);
    }

    /// **MR26** `[26]` (RW)
    ///
    /// Event Mask on line 26
    #[inline(always)]
    pub fn mr26(&self) -> bool {
        (self.value.read_bits() & Self::MR26_MASK) != 0
    }

    /// Set MR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr26(&self) {
        self.value.modify_bits(|r| r | Self::MR26_MASK);
    }

    /// Clear MR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr26(&self) {
        self.value.modify_bits(|r| r & !Self::MR26_MASK);
    }

    /// **MR25** `[25]` (RW)
    ///
    /// Event Mask on line 25
    #[inline(always)]
    pub fn mr25(&self) -> bool {
        (self.value.read_bits() & Self::MR25_MASK) != 0
    }

    /// Set MR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr25(&self) {
        self.value.modify_bits(|r| r | Self::MR25_MASK);
    }

    /// Clear MR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr25(&self) {
        self.value.modify_bits(|r| r & !Self::MR25_MASK);
    }

    /// **MR24** `[24]` (RW)
    ///
    /// Event Mask on line 24
    #[inline(always)]
    pub fn mr24(&self) -> bool {
        (self.value.read_bits() & Self::MR24_MASK) != 0
    }

    /// Set MR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr24(&self) {
        self.value.modify_bits(|r| r | Self::MR24_MASK);
    }

    /// Clear MR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr24(&self) {
        self.value.modify_bits(|r| r & !Self::MR24_MASK);
    }

    /// **MR23** `[23]` (RW)
    ///
    /// Event Mask on line 23
    #[inline(always)]
    pub fn mr23(&self) -> bool {
        (self.value.read_bits() & Self::MR23_MASK) != 0
    }

    /// Set MR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr23(&self) {
        self.value.modify_bits(|r| r | Self::MR23_MASK);
    }

    /// Clear MR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr23(&self) {
        self.value.modify_bits(|r| r & !Self::MR23_MASK);
    }

    /// **MR22** `[22]` (RW)
    ///
    /// Event Mask on line 22
    #[inline(always)]
    pub fn mr22(&self) -> bool {
        (self.value.read_bits() & Self::MR22_MASK) != 0
    }

    /// Set MR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr22(&self) {
        self.value.modify_bits(|r| r | Self::MR22_MASK);
    }

    /// Clear MR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr22(&self) {
        self.value.modify_bits(|r| r & !Self::MR22_MASK);
    }

    /// **MR21** `[21]` (RW)
    ///
    /// Event Mask on line 21
    #[inline(always)]
    pub fn mr21(&self) -> bool {
        (self.value.read_bits() & Self::MR21_MASK) != 0
    }

    /// Set MR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr21(&self) {
        self.value.modify_bits(|r| r | Self::MR21_MASK);
    }

    /// Clear MR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr21(&self) {
        self.value.modify_bits(|r| r & !Self::MR21_MASK);
    }

    /// **MR20** `[20]` (RW)
    ///
    /// Event Mask on line 20
    #[inline(always)]
    pub fn mr20(&self) -> bool {
        (self.value.read_bits() & Self::MR20_MASK) != 0
    }

    /// Set MR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr20(&self) {
        self.value.modify_bits(|r| r | Self::MR20_MASK);
    }

    /// Clear MR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr20(&self) {
        self.value.modify_bits(|r| r & !Self::MR20_MASK);
    }

    /// **MR19** `[19]` (RW)
    ///
    /// Event Mask on line 19
    #[inline(always)]
    pub fn mr19(&self) -> bool {
        (self.value.read_bits() & Self::MR19_MASK) != 0
    }

    /// Set MR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr19(&self) {
        self.value.modify_bits(|r| r | Self::MR19_MASK);
    }

    /// Clear MR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr19(&self) {
        self.value.modify_bits(|r| r & !Self::MR19_MASK);
    }

    /// **MR18** `[18]` (RW)
    ///
    /// Event Mask on line 18
    #[inline(always)]
    pub fn mr18(&self) -> bool {
        (self.value.read_bits() & Self::MR18_MASK) != 0
    }

    /// Set MR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr18(&self) {
        self.value.modify_bits(|r| r | Self::MR18_MASK);
    }

    /// Clear MR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr18(&self) {
        self.value.modify_bits(|r| r & !Self::MR18_MASK);
    }

    /// **MR17** `[17]` (RW)
    ///
    /// Event Mask on line 17
    #[inline(always)]
    pub fn mr17(&self) -> bool {
        (self.value.read_bits() & Self::MR17_MASK) != 0
    }

    /// Set MR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr17(&self) {
        self.value.modify_bits(|r| r | Self::MR17_MASK);
    }

    /// Clear MR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr17(&self) {
        self.value.modify_bits(|r| r & !Self::MR17_MASK);
    }

    /// **MR16** `[16]` (RW)
    ///
    /// Event Mask on line 16
    #[inline(always)]
    pub fn mr16(&self) -> bool {
        (self.value.read_bits() & Self::MR16_MASK) != 0
    }

    /// Set MR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr16(&self) {
        self.value.modify_bits(|r| r | Self::MR16_MASK);
    }

    /// Clear MR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr16(&self) {
        self.value.modify_bits(|r| r & !Self::MR16_MASK);
    }

    /// **MR15** `[15]` (RW)
    ///
    /// Event Mask on line 15
    #[inline(always)]
    pub fn mr15(&self) -> bool {
        (self.value.read_bits() & Self::MR15_MASK) != 0
    }

    /// Set MR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr15(&self) {
        self.value.modify_bits(|r| r | Self::MR15_MASK);
    }

    /// Clear MR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr15(&self) {
        self.value.modify_bits(|r| r & !Self::MR15_MASK);
    }

    /// **MR14** `[14]` (RW)
    ///
    /// Event Mask on line 14
    #[inline(always)]
    pub fn mr14(&self) -> bool {
        (self.value.read_bits() & Self::MR14_MASK) != 0
    }

    /// Set MR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr14(&self) {
        self.value.modify_bits(|r| r | Self::MR14_MASK);
    }

    /// Clear MR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr14(&self) {
        self.value.modify_bits(|r| r & !Self::MR14_MASK);
    }

    /// **MR13** `[13]` (RW)
    ///
    /// Event Mask on line 13
    #[inline(always)]
    pub fn mr13(&self) -> bool {
        (self.value.read_bits() & Self::MR13_MASK) != 0
    }

    /// Set MR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr13(&self) {
        self.value.modify_bits(|r| r | Self::MR13_MASK);
    }

    /// Clear MR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr13(&self) {
        self.value.modify_bits(|r| r & !Self::MR13_MASK);
    }

    /// **MR12** `[12]` (RW)
    ///
    /// Event Mask on line 12
    #[inline(always)]
    pub fn mr12(&self) -> bool {
        (self.value.read_bits() & Self::MR12_MASK) != 0
    }

    /// Set MR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr12(&self) {
        self.value.modify_bits(|r| r | Self::MR12_MASK);
    }

    /// Clear MR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr12(&self) {
        self.value.modify_bits(|r| r & !Self::MR12_MASK);
    }

    /// **MR11** `[11]` (RW)
    ///
    /// Event Mask on line 11
    #[inline(always)]
    pub fn mr11(&self) -> bool {
        (self.value.read_bits() & Self::MR11_MASK) != 0
    }

    /// Set MR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr11(&self) {
        self.value.modify_bits(|r| r | Self::MR11_MASK);
    }

    /// Clear MR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr11(&self) {
        self.value.modify_bits(|r| r & !Self::MR11_MASK);
    }

    /// **MR10** `[10]` (RW)
    ///
    /// Event Mask on line 10
    #[inline(always)]
    pub fn mr10(&self) -> bool {
        (self.value.read_bits() & Self::MR10_MASK) != 0
    }

    /// Set MR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr10(&self) {
        self.value.modify_bits(|r| r | Self::MR10_MASK);
    }

    /// Clear MR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr10(&self) {
        self.value.modify_bits(|r| r & !Self::MR10_MASK);
    }

    /// **MR9** `[9]` (RW)
    ///
    /// Event Mask on line 9
    #[inline(always)]
    pub fn mr9(&self) -> bool {
        (self.value.read_bits() & Self::MR9_MASK) != 0
    }

    /// Set MR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr9(&self) {
        self.value.modify_bits(|r| r | Self::MR9_MASK);
    }

    /// Clear MR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr9(&self) {
        self.value.modify_bits(|r| r & !Self::MR9_MASK);
    }

    /// **MR8** `[8]` (RW)
    ///
    /// Event Mask on line 8
    #[inline(always)]
    pub fn mr8(&self) -> bool {
        (self.value.read_bits() & Self::MR8_MASK) != 0
    }

    /// Set MR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr8(&self) {
        self.value.modify_bits(|r| r | Self::MR8_MASK);
    }

    /// Clear MR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr8(&self) {
        self.value.modify_bits(|r| r & !Self::MR8_MASK);
    }

    /// **MR7** `[7]` (RW)
    ///
    /// Event Mask on line 7
    #[inline(always)]
    pub fn mr7(&self) -> bool {
        (self.value.read_bits() & Self::MR7_MASK) != 0
    }

    /// Set MR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr7(&self) {
        self.value.modify_bits(|r| r | Self::MR7_MASK);
    }

    /// Clear MR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr7(&self) {
        self.value.modify_bits(|r| r & !Self::MR7_MASK);
    }

    /// **MR6** `[6]` (RW)
    ///
    /// Event Mask on line 6
    #[inline(always)]
    pub fn mr6(&self) -> bool {
        (self.value.read_bits() & Self::MR6_MASK) != 0
    }

    /// Set MR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr6(&self) {
        self.value.modify_bits(|r| r | Self::MR6_MASK);
    }

    /// Clear MR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr6(&self) {
        self.value.modify_bits(|r| r & !Self::MR6_MASK);
    }

    /// **MR5** `[5]` (RW)
    ///
    /// Event Mask on line 5
    #[inline(always)]
    pub fn mr5(&self) -> bool {
        (self.value.read_bits() & Self::MR5_MASK) != 0
    }

    /// Set MR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr5(&self) {
        self.value.modify_bits(|r| r | Self::MR5_MASK);
    }

    /// Clear MR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr5(&self) {
        self.value.modify_bits(|r| r & !Self::MR5_MASK);
    }

    /// **MR4** `[4]` (RW)
    ///
    /// Event Mask on line 4
    #[inline(always)]
    pub fn mr4(&self) -> bool {
        (self.value.read_bits() & Self::MR4_MASK) != 0
    }

    /// Set MR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr4(&self) {
        self.value.modify_bits(|r| r | Self::MR4_MASK);
    }

    /// Clear MR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr4(&self) {
        self.value.modify_bits(|r| r & !Self::MR4_MASK);
    }

    /// **MR3** `[3]` (RW)
    ///
    /// Event Mask on line 3
    #[inline(always)]
    pub fn mr3(&self) -> bool {
        (self.value.read_bits() & Self::MR3_MASK) != 0
    }

    /// Set MR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr3(&self) {
        self.value.modify_bits(|r| r | Self::MR3_MASK);
    }

    /// Clear MR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr3(&self) {
        self.value.modify_bits(|r| r & !Self::MR3_MASK);
    }

    /// **MR2** `[2]` (RW)
    ///
    /// Event Mask on line 2
    #[inline(always)]
    pub fn mr2(&self) -> bool {
        (self.value.read_bits() & Self::MR2_MASK) != 0
    }

    /// Set MR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr2(&self) {
        self.value.modify_bits(|r| r | Self::MR2_MASK);
    }

    /// Clear MR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr2(&self) {
        self.value.modify_bits(|r| r & !Self::MR2_MASK);
    }

    /// **MR1** `[1]` (RW)
    ///
    /// Event Mask on line 1
    #[inline(always)]
    pub fn mr1(&self) -> bool {
        (self.value.read_bits() & Self::MR1_MASK) != 0
    }

    /// Set MR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr1(&self) {
        self.value.modify_bits(|r| r | Self::MR1_MASK);
    }

    /// Clear MR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr1(&self) {
        self.value.modify_bits(|r| r & !Self::MR1_MASK);
    }

    /// **MR0** `[0]` (RW)
    ///
    /// Event Mask on line 0
    #[inline(always)]
    pub fn mr0(&self) -> bool {
        (self.value.read_bits() & Self::MR0_MASK) != 0
    }

    /// Set MR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mr0(&self) {
        self.value.modify_bits(|r| r | Self::MR0_MASK);
    }

    /// Clear MR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mr0(&self) {
        self.value.modify_bits(|r| r & !Self::MR0_MASK);
    }
}

/// EVENR value type for batch read/modify/write operations
///
/// Use `evenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `evenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EvenrVal {
    bits: u32,
}

impl EvenrVal {
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

    /// Read **MR29** `[29:29]`
    ///
    /// Event Mask on line 29
    #[inline(always)]
    pub const fn mr29(&self) -> bool {
        (self.bits & Evenr::MR29_MASK) != 0
    }

    /// Read **MR28** `[28:28]`
    ///
    /// Event Mask on line 28
    #[inline(always)]
    pub const fn mr28(&self) -> bool {
        (self.bits & Evenr::MR28_MASK) != 0
    }

    /// Read **MR27** `[27:27]`
    ///
    /// Event Mask on line 27
    #[inline(always)]
    pub const fn mr27(&self) -> bool {
        (self.bits & Evenr::MR27_MASK) != 0
    }

    /// Read **MR26** `[26:26]`
    ///
    /// Event Mask on line 26
    #[inline(always)]
    pub const fn mr26(&self) -> bool {
        (self.bits & Evenr::MR26_MASK) != 0
    }

    /// Read **MR25** `[25:25]`
    ///
    /// Event Mask on line 25
    #[inline(always)]
    pub const fn mr25(&self) -> bool {
        (self.bits & Evenr::MR25_MASK) != 0
    }

    /// Read **MR24** `[24:24]`
    ///
    /// Event Mask on line 24
    #[inline(always)]
    pub const fn mr24(&self) -> bool {
        (self.bits & Evenr::MR24_MASK) != 0
    }

    /// Read **MR23** `[23:23]`
    ///
    /// Event Mask on line 23
    #[inline(always)]
    pub const fn mr23(&self) -> bool {
        (self.bits & Evenr::MR23_MASK) != 0
    }

    /// Read **MR22** `[22:22]`
    ///
    /// Event Mask on line 22
    #[inline(always)]
    pub const fn mr22(&self) -> bool {
        (self.bits & Evenr::MR22_MASK) != 0
    }

    /// Read **MR21** `[21:21]`
    ///
    /// Event Mask on line 21
    #[inline(always)]
    pub const fn mr21(&self) -> bool {
        (self.bits & Evenr::MR21_MASK) != 0
    }

    /// Read **MR20** `[20:20]`
    ///
    /// Event Mask on line 20
    #[inline(always)]
    pub const fn mr20(&self) -> bool {
        (self.bits & Evenr::MR20_MASK) != 0
    }

    /// Read **MR19** `[19:19]`
    ///
    /// Event Mask on line 19
    #[inline(always)]
    pub const fn mr19(&self) -> bool {
        (self.bits & Evenr::MR19_MASK) != 0
    }

    /// Read **MR18** `[18:18]`
    ///
    /// Event Mask on line 18
    #[inline(always)]
    pub const fn mr18(&self) -> bool {
        (self.bits & Evenr::MR18_MASK) != 0
    }

    /// Read **MR17** `[17:17]`
    ///
    /// Event Mask on line 17
    #[inline(always)]
    pub const fn mr17(&self) -> bool {
        (self.bits & Evenr::MR17_MASK) != 0
    }

    /// Read **MR16** `[16:16]`
    ///
    /// Event Mask on line 16
    #[inline(always)]
    pub const fn mr16(&self) -> bool {
        (self.bits & Evenr::MR16_MASK) != 0
    }

    /// Read **MR15** `[15:15]`
    ///
    /// Event Mask on line 15
    #[inline(always)]
    pub const fn mr15(&self) -> bool {
        (self.bits & Evenr::MR15_MASK) != 0
    }

    /// Read **MR14** `[14:14]`
    ///
    /// Event Mask on line 14
    #[inline(always)]
    pub const fn mr14(&self) -> bool {
        (self.bits & Evenr::MR14_MASK) != 0
    }

    /// Read **MR13** `[13:13]`
    ///
    /// Event Mask on line 13
    #[inline(always)]
    pub const fn mr13(&self) -> bool {
        (self.bits & Evenr::MR13_MASK) != 0
    }

    /// Read **MR12** `[12:12]`
    ///
    /// Event Mask on line 12
    #[inline(always)]
    pub const fn mr12(&self) -> bool {
        (self.bits & Evenr::MR12_MASK) != 0
    }

    /// Read **MR11** `[11:11]`
    ///
    /// Event Mask on line 11
    #[inline(always)]
    pub const fn mr11(&self) -> bool {
        (self.bits & Evenr::MR11_MASK) != 0
    }

    /// Read **MR10** `[10:10]`
    ///
    /// Event Mask on line 10
    #[inline(always)]
    pub const fn mr10(&self) -> bool {
        (self.bits & Evenr::MR10_MASK) != 0
    }

    /// Read **MR9** `[9:9]`
    ///
    /// Event Mask on line 9
    #[inline(always)]
    pub const fn mr9(&self) -> bool {
        (self.bits & Evenr::MR9_MASK) != 0
    }

    /// Read **MR8** `[8:8]`
    ///
    /// Event Mask on line 8
    #[inline(always)]
    pub const fn mr8(&self) -> bool {
        (self.bits & Evenr::MR8_MASK) != 0
    }

    /// Read **MR7** `[7:7]`
    ///
    /// Event Mask on line 7
    #[inline(always)]
    pub const fn mr7(&self) -> bool {
        (self.bits & Evenr::MR7_MASK) != 0
    }

    /// Read **MR6** `[6:6]`
    ///
    /// Event Mask on line 6
    #[inline(always)]
    pub const fn mr6(&self) -> bool {
        (self.bits & Evenr::MR6_MASK) != 0
    }

    /// Read **MR5** `[5:5]`
    ///
    /// Event Mask on line 5
    #[inline(always)]
    pub const fn mr5(&self) -> bool {
        (self.bits & Evenr::MR5_MASK) != 0
    }

    /// Read **MR4** `[4:4]`
    ///
    /// Event Mask on line 4
    #[inline(always)]
    pub const fn mr4(&self) -> bool {
        (self.bits & Evenr::MR4_MASK) != 0
    }

    /// Read **MR3** `[3:3]`
    ///
    /// Event Mask on line 3
    #[inline(always)]
    pub const fn mr3(&self) -> bool {
        (self.bits & Evenr::MR3_MASK) != 0
    }

    /// Read **MR2** `[2:2]`
    ///
    /// Event Mask on line 2
    #[inline(always)]
    pub const fn mr2(&self) -> bool {
        (self.bits & Evenr::MR2_MASK) != 0
    }

    /// Read **MR1** `[1:1]`
    ///
    /// Event Mask on line 1
    #[inline(always)]
    pub const fn mr1(&self) -> bool {
        (self.bits & Evenr::MR1_MASK) != 0
    }

    /// Read **MR0** `[0:0]`
    ///
    /// Event Mask on line 0
    #[inline(always)]
    pub const fn mr0(&self) -> bool {
        (self.bits & Evenr::MR0_MASK) != 0
    }

    /// Set **MR29** `[29:29]`
    ///
    /// Event Mask on line 29
    #[inline(always)]
    pub const fn with_mr29(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR29_MASK
            } else {
                self.bits & !Evenr::MR29_MASK
            },
        }
    }

    /// Set **MR28** `[28:28]`
    ///
    /// Event Mask on line 28
    #[inline(always)]
    pub const fn with_mr28(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR28_MASK
            } else {
                self.bits & !Evenr::MR28_MASK
            },
        }
    }

    /// Set **MR27** `[27:27]`
    ///
    /// Event Mask on line 27
    #[inline(always)]
    pub const fn with_mr27(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR27_MASK
            } else {
                self.bits & !Evenr::MR27_MASK
            },
        }
    }

    /// Set **MR26** `[26:26]`
    ///
    /// Event Mask on line 26
    #[inline(always)]
    pub const fn with_mr26(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR26_MASK
            } else {
                self.bits & !Evenr::MR26_MASK
            },
        }
    }

    /// Set **MR25** `[25:25]`
    ///
    /// Event Mask on line 25
    #[inline(always)]
    pub const fn with_mr25(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR25_MASK
            } else {
                self.bits & !Evenr::MR25_MASK
            },
        }
    }

    /// Set **MR24** `[24:24]`
    ///
    /// Event Mask on line 24
    #[inline(always)]
    pub const fn with_mr24(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR24_MASK
            } else {
                self.bits & !Evenr::MR24_MASK
            },
        }
    }

    /// Set **MR23** `[23:23]`
    ///
    /// Event Mask on line 23
    #[inline(always)]
    pub const fn with_mr23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR23_MASK
            } else {
                self.bits & !Evenr::MR23_MASK
            },
        }
    }

    /// Set **MR22** `[22:22]`
    ///
    /// Event Mask on line 22
    #[inline(always)]
    pub const fn with_mr22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR22_MASK
            } else {
                self.bits & !Evenr::MR22_MASK
            },
        }
    }

    /// Set **MR21** `[21:21]`
    ///
    /// Event Mask on line 21
    #[inline(always)]
    pub const fn with_mr21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR21_MASK
            } else {
                self.bits & !Evenr::MR21_MASK
            },
        }
    }

    /// Set **MR20** `[20:20]`
    ///
    /// Event Mask on line 20
    #[inline(always)]
    pub const fn with_mr20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR20_MASK
            } else {
                self.bits & !Evenr::MR20_MASK
            },
        }
    }

    /// Set **MR19** `[19:19]`
    ///
    /// Event Mask on line 19
    #[inline(always)]
    pub const fn with_mr19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR19_MASK
            } else {
                self.bits & !Evenr::MR19_MASK
            },
        }
    }

    /// Set **MR18** `[18:18]`
    ///
    /// Event Mask on line 18
    #[inline(always)]
    pub const fn with_mr18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR18_MASK
            } else {
                self.bits & !Evenr::MR18_MASK
            },
        }
    }

    /// Set **MR17** `[17:17]`
    ///
    /// Event Mask on line 17
    #[inline(always)]
    pub const fn with_mr17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR17_MASK
            } else {
                self.bits & !Evenr::MR17_MASK
            },
        }
    }

    /// Set **MR16** `[16:16]`
    ///
    /// Event Mask on line 16
    #[inline(always)]
    pub const fn with_mr16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR16_MASK
            } else {
                self.bits & !Evenr::MR16_MASK
            },
        }
    }

    /// Set **MR15** `[15:15]`
    ///
    /// Event Mask on line 15
    #[inline(always)]
    pub const fn with_mr15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR15_MASK
            } else {
                self.bits & !Evenr::MR15_MASK
            },
        }
    }

    /// Set **MR14** `[14:14]`
    ///
    /// Event Mask on line 14
    #[inline(always)]
    pub const fn with_mr14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR14_MASK
            } else {
                self.bits & !Evenr::MR14_MASK
            },
        }
    }

    /// Set **MR13** `[13:13]`
    ///
    /// Event Mask on line 13
    #[inline(always)]
    pub const fn with_mr13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR13_MASK
            } else {
                self.bits & !Evenr::MR13_MASK
            },
        }
    }

    /// Set **MR12** `[12:12]`
    ///
    /// Event Mask on line 12
    #[inline(always)]
    pub const fn with_mr12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR12_MASK
            } else {
                self.bits & !Evenr::MR12_MASK
            },
        }
    }

    /// Set **MR11** `[11:11]`
    ///
    /// Event Mask on line 11
    #[inline(always)]
    pub const fn with_mr11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR11_MASK
            } else {
                self.bits & !Evenr::MR11_MASK
            },
        }
    }

    /// Set **MR10** `[10:10]`
    ///
    /// Event Mask on line 10
    #[inline(always)]
    pub const fn with_mr10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR10_MASK
            } else {
                self.bits & !Evenr::MR10_MASK
            },
        }
    }

    /// Set **MR9** `[9:9]`
    ///
    /// Event Mask on line 9
    #[inline(always)]
    pub const fn with_mr9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR9_MASK
            } else {
                self.bits & !Evenr::MR9_MASK
            },
        }
    }

    /// Set **MR8** `[8:8]`
    ///
    /// Event Mask on line 8
    #[inline(always)]
    pub const fn with_mr8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR8_MASK
            } else {
                self.bits & !Evenr::MR8_MASK
            },
        }
    }

    /// Set **MR7** `[7:7]`
    ///
    /// Event Mask on line 7
    #[inline(always)]
    pub const fn with_mr7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR7_MASK
            } else {
                self.bits & !Evenr::MR7_MASK
            },
        }
    }

    /// Set **MR6** `[6:6]`
    ///
    /// Event Mask on line 6
    #[inline(always)]
    pub const fn with_mr6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR6_MASK
            } else {
                self.bits & !Evenr::MR6_MASK
            },
        }
    }

    /// Set **MR5** `[5:5]`
    ///
    /// Event Mask on line 5
    #[inline(always)]
    pub const fn with_mr5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR5_MASK
            } else {
                self.bits & !Evenr::MR5_MASK
            },
        }
    }

    /// Set **MR4** `[4:4]`
    ///
    /// Event Mask on line 4
    #[inline(always)]
    pub const fn with_mr4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR4_MASK
            } else {
                self.bits & !Evenr::MR4_MASK
            },
        }
    }

    /// Set **MR3** `[3:3]`
    ///
    /// Event Mask on line 3
    #[inline(always)]
    pub const fn with_mr3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR3_MASK
            } else {
                self.bits & !Evenr::MR3_MASK
            },
        }
    }

    /// Set **MR2** `[2:2]`
    ///
    /// Event Mask on line 2
    #[inline(always)]
    pub const fn with_mr2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR2_MASK
            } else {
                self.bits & !Evenr::MR2_MASK
            },
        }
    }

    /// Set **MR1** `[1:1]`
    ///
    /// Event Mask on line 1
    #[inline(always)]
    pub const fn with_mr1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR1_MASK
            } else {
                self.bits & !Evenr::MR1_MASK
            },
        }
    }

    /// Set **MR0** `[0:0]`
    ///
    /// Event Mask on line 0
    #[inline(always)]
    pub const fn with_mr0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Evenr::MR0_MASK
            } else {
                self.bits & !Evenr::MR0_MASK
            },
        }
    }
}

impl Default for EvenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Evenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EvenrVal {
        EvenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EvenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EvenrVal) -> EvenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RTENR Register
///
/// Rising Trigger selection register
/// (EXTI_RTENR)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TR29** `[29]` (RW)
///
///   Rising trigger event configuration of
///   line 29
///
/// - **TR28** `[28]` (RW)
///
///   Rising trigger event configuration of
///   line 28
///
/// - **TR27** `[27]` (RW)
///
///   Rising trigger event configuration of
///   line 27
///
/// - **TR26** `[26]` (RW)
///
///   Rising trigger event configuration of
///   line 26
///
/// - **TR25** `[25]` (RW)
///
///   Rising trigger event configuration of
///   line 25
///
/// - **TR24** `[24]` (RW)
///
///   Rising trigger event configuration of
///   line 24
///
/// - **TR23** `[23]` (RW)
///
///   Rising trigger event configuration of
///   line 23
///
/// - **TR22** `[22]` (RW)
///
///   Rising trigger event configuration of
///   line 22
///
/// - **TR21** `[21]` (RW)
///
///   Rising trigger event configuration of
///   line 21
///
/// - **TR20** `[20]` (RW)
///
///   Rising trigger event configuration of
///   line 10
///
/// - **TR19** `[19]` (RW)
///
///   Rising trigger event configuration of
///   line 19
///
/// - **TR18** `[18]` (RW)
///
///   Rising trigger event configuration of
///   line 18
///
/// - **TR17** `[17]` (RW)
///
///   Rising trigger event configuration of
///   line 17
///
/// - **TR16** `[16]` (RW)
///
///   Rising trigger event configuration of
///   line 16
///
/// - **TR15** `[15]` (RW)
///
///   Rising trigger event configuration of
///   line 15
///
/// - **TR14** `[14]` (RW)
///
///   Rising trigger event configuration of
///   line 14
///
/// - **TR13** `[13]` (RW)
///
///   Rising trigger event configuration of
///   line 13
///
/// - **TR12** `[12]` (RW)
///
///   Rising trigger event configuration of
///   line 12
///
/// - **TR11** `[11]` (RW)
///
///   Rising trigger event configuration of
///   line 11
///
/// - **TR10** `[10]` (RW)
///
///   Rising trigger event configuration of
///   line 10
///
/// - **TR9** `[9]` (RW)
///
///   Rising trigger event configuration of
///   line 9
///
/// - **TR8** `[8]` (RW)
///
///   Rising trigger event configuration of
///   line 8
///
/// - **TR7** `[7]` (RW)
///
///   Rising trigger event configuration of
///   line 7
///
/// - **TR6** `[6]` (RW)
///
///   Rising trigger event configuration of
///   line 6
///
/// - **TR5** `[5]` (RW)
///
///   Rising trigger event configuration of
///   line 5
///
/// - **TR4** `[4]` (RW)
///
///   Rising trigger event configuration of
///   line 4
///
/// - **TR3** `[3]` (RW)
///
///   Rising trigger event configuration of
///   line 3
///
/// - **TR2** `[2]` (RW)
///
///   Rising trigger event configuration of
///   line 2
///
/// - **TR1** `[1]` (RW)
///
///   Rising trigger event configuration of
///   line 1
///
/// - **TR0** `[0]` (RW)
///
///   Rising trigger event configuration of
///   line 0
#[repr(transparent)]
pub struct Rtenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rtenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rtenr {
    // ======== Field Constants ========

    /// **TR29**
    ///
    /// Rising trigger event configuration of
    /// line 29
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TR29_OFFSET: u32 = 29;
    pub const TR29_MASK: u32 = 0x20000000;
    pub const TR29: u32 = 1 << 29;

    /// **TR28**
    ///
    /// Rising trigger event configuration of
    /// line 28
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TR28_OFFSET: u32 = 28;
    pub const TR28_MASK: u32 = 0x10000000;
    pub const TR28: u32 = 1 << 28;

    /// **TR27**
    ///
    /// Rising trigger event configuration of
    /// line 27
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TR27_OFFSET: u32 = 27;
    pub const TR27_MASK: u32 = 0x08000000;
    pub const TR27: u32 = 1 << 27;

    /// **TR26**
    ///
    /// Rising trigger event configuration of
    /// line 26
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TR26_OFFSET: u32 = 26;
    pub const TR26_MASK: u32 = 0x04000000;
    pub const TR26: u32 = 1 << 26;

    /// **TR25**
    ///
    /// Rising trigger event configuration of
    /// line 25
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TR25_OFFSET: u32 = 25;
    pub const TR25_MASK: u32 = 0x02000000;
    pub const TR25: u32 = 1 << 25;

    /// **TR24**
    ///
    /// Rising trigger event configuration of
    /// line 24
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TR24_OFFSET: u32 = 24;
    pub const TR24_MASK: u32 = 0x01000000;
    pub const TR24: u32 = 1 << 24;

    /// **TR23**
    ///
    /// Rising trigger event configuration of
    /// line 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TR23_OFFSET: u32 = 23;
    pub const TR23_MASK: u32 = 0x00800000;
    pub const TR23: u32 = 1 << 23;

    /// **TR22**
    ///
    /// Rising trigger event configuration of
    /// line 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TR22_OFFSET: u32 = 22;
    pub const TR22_MASK: u32 = 0x00400000;
    pub const TR22: u32 = 1 << 22;

    /// **TR21**
    ///
    /// Rising trigger event configuration of
    /// line 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const TR21_OFFSET: u32 = 21;
    pub const TR21_MASK: u32 = 0x00200000;
    pub const TR21: u32 = 1 << 21;

    /// **TR20**
    ///
    /// Rising trigger event configuration of
    /// line 10
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const TR20_OFFSET: u32 = 20;
    pub const TR20_MASK: u32 = 0x00100000;
    pub const TR20: u32 = 1 << 20;

    /// **TR19**
    ///
    /// Rising trigger event configuration of
    /// line 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TR19_OFFSET: u32 = 19;
    pub const TR19_MASK: u32 = 0x00080000;
    pub const TR19: u32 = 1 << 19;

    /// **TR18**
    ///
    /// Rising trigger event configuration of
    /// line 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TR18_OFFSET: u32 = 18;
    pub const TR18_MASK: u32 = 0x00040000;
    pub const TR18: u32 = 1 << 18;

    /// **TR17**
    ///
    /// Rising trigger event configuration of
    /// line 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const TR17_OFFSET: u32 = 17;
    pub const TR17_MASK: u32 = 0x00020000;
    pub const TR17: u32 = 1 << 17;

    /// **TR16**
    ///
    /// Rising trigger event configuration of
    /// line 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const TR16_OFFSET: u32 = 16;
    pub const TR16_MASK: u32 = 0x00010000;
    pub const TR16: u32 = 1 << 16;

    /// **TR15**
    ///
    /// Rising trigger event configuration of
    /// line 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TR15_OFFSET: u32 = 15;
    pub const TR15_MASK: u32 = 0x00008000;
    pub const TR15: u32 = 1 << 15;

    /// **TR14**
    ///
    /// Rising trigger event configuration of
    /// line 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TR14_OFFSET: u32 = 14;
    pub const TR14_MASK: u32 = 0x00004000;
    pub const TR14: u32 = 1 << 14;

    /// **TR13**
    ///
    /// Rising trigger event configuration of
    /// line 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TR13_OFFSET: u32 = 13;
    pub const TR13_MASK: u32 = 0x00002000;
    pub const TR13: u32 = 1 << 13;

    /// **TR12**
    ///
    /// Rising trigger event configuration of
    /// line 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TR12_OFFSET: u32 = 12;
    pub const TR12_MASK: u32 = 0x00001000;
    pub const TR12: u32 = 1 << 12;

    /// **TR11**
    ///
    /// Rising trigger event configuration of
    /// line 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TR11_OFFSET: u32 = 11;
    pub const TR11_MASK: u32 = 0x00000800;
    pub const TR11: u32 = 1 << 11;

    /// **TR10**
    ///
    /// Rising trigger event configuration of
    /// line 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TR10_OFFSET: u32 = 10;
    pub const TR10_MASK: u32 = 0x00000400;
    pub const TR10: u32 = 1 << 10;

    /// **TR9**
    ///
    /// Rising trigger event configuration of
    /// line 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TR9_OFFSET: u32 = 9;
    pub const TR9_MASK: u32 = 0x00000200;
    pub const TR9: u32 = 1 << 9;

    /// **TR8**
    ///
    /// Rising trigger event configuration of
    /// line 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TR8_OFFSET: u32 = 8;
    pub const TR8_MASK: u32 = 0x00000100;
    pub const TR8: u32 = 1 << 8;

    /// **TR7**
    ///
    /// Rising trigger event configuration of
    /// line 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TR7_OFFSET: u32 = 7;
    pub const TR7_MASK: u32 = 0x00000080;
    pub const TR7: u32 = 1 << 7;

    /// **TR6**
    ///
    /// Rising trigger event configuration of
    /// line 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TR6_OFFSET: u32 = 6;
    pub const TR6_MASK: u32 = 0x00000040;
    pub const TR6: u32 = 1 << 6;

    /// **TR5**
    ///
    /// Rising trigger event configuration of
    /// line 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TR5_OFFSET: u32 = 5;
    pub const TR5_MASK: u32 = 0x00000020;
    pub const TR5: u32 = 1 << 5;

    /// **TR4**
    ///
    /// Rising trigger event configuration of
    /// line 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TR4_OFFSET: u32 = 4;
    pub const TR4_MASK: u32 = 0x00000010;
    pub const TR4: u32 = 1 << 4;

    /// **TR3**
    ///
    /// Rising trigger event configuration of
    /// line 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TR3_OFFSET: u32 = 3;
    pub const TR3_MASK: u32 = 0x00000008;
    pub const TR3: u32 = 1 << 3;

    /// **TR2**
    ///
    /// Rising trigger event configuration of
    /// line 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TR2_OFFSET: u32 = 2;
    pub const TR2_MASK: u32 = 0x00000004;
    pub const TR2: u32 = 1 << 2;

    /// **TR1**
    ///
    /// Rising trigger event configuration of
    /// line 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TR1_OFFSET: u32 = 1;
    pub const TR1_MASK: u32 = 0x00000002;
    pub const TR1: u32 = 1 << 1;

    /// **TR0**
    ///
    /// Rising trigger event configuration of
    /// line 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TR0_OFFSET: u32 = 0;
    pub const TR0_MASK: u32 = 0x00000001;
    pub const TR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TR29** `[29]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 29
    #[inline(always)]
    pub fn tr29(&self) -> bool {
        (self.value.read_bits() & Self::TR29_MASK) != 0
    }

    /// Set TR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr29(&self) {
        self.value.modify_bits(|r| r | Self::TR29_MASK);
    }

    /// Clear TR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr29(&self) {
        self.value.modify_bits(|r| r & !Self::TR29_MASK);
    }

    /// **TR28** `[28]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 28
    #[inline(always)]
    pub fn tr28(&self) -> bool {
        (self.value.read_bits() & Self::TR28_MASK) != 0
    }

    /// Set TR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr28(&self) {
        self.value.modify_bits(|r| r | Self::TR28_MASK);
    }

    /// Clear TR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr28(&self) {
        self.value.modify_bits(|r| r & !Self::TR28_MASK);
    }

    /// **TR27** `[27]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 27
    #[inline(always)]
    pub fn tr27(&self) -> bool {
        (self.value.read_bits() & Self::TR27_MASK) != 0
    }

    /// Set TR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr27(&self) {
        self.value.modify_bits(|r| r | Self::TR27_MASK);
    }

    /// Clear TR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr27(&self) {
        self.value.modify_bits(|r| r & !Self::TR27_MASK);
    }

    /// **TR26** `[26]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 26
    #[inline(always)]
    pub fn tr26(&self) -> bool {
        (self.value.read_bits() & Self::TR26_MASK) != 0
    }

    /// Set TR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr26(&self) {
        self.value.modify_bits(|r| r | Self::TR26_MASK);
    }

    /// Clear TR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr26(&self) {
        self.value.modify_bits(|r| r & !Self::TR26_MASK);
    }

    /// **TR25** `[25]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 25
    #[inline(always)]
    pub fn tr25(&self) -> bool {
        (self.value.read_bits() & Self::TR25_MASK) != 0
    }

    /// Set TR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr25(&self) {
        self.value.modify_bits(|r| r | Self::TR25_MASK);
    }

    /// Clear TR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr25(&self) {
        self.value.modify_bits(|r| r & !Self::TR25_MASK);
    }

    /// **TR24** `[24]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 24
    #[inline(always)]
    pub fn tr24(&self) -> bool {
        (self.value.read_bits() & Self::TR24_MASK) != 0
    }

    /// Set TR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr24(&self) {
        self.value.modify_bits(|r| r | Self::TR24_MASK);
    }

    /// Clear TR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr24(&self) {
        self.value.modify_bits(|r| r & !Self::TR24_MASK);
    }

    /// **TR23** `[23]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 23
    #[inline(always)]
    pub fn tr23(&self) -> bool {
        (self.value.read_bits() & Self::TR23_MASK) != 0
    }

    /// Set TR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr23(&self) {
        self.value.modify_bits(|r| r | Self::TR23_MASK);
    }

    /// Clear TR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr23(&self) {
        self.value.modify_bits(|r| r & !Self::TR23_MASK);
    }

    /// **TR22** `[22]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 22
    #[inline(always)]
    pub fn tr22(&self) -> bool {
        (self.value.read_bits() & Self::TR22_MASK) != 0
    }

    /// Set TR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr22(&self) {
        self.value.modify_bits(|r| r | Self::TR22_MASK);
    }

    /// Clear TR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr22(&self) {
        self.value.modify_bits(|r| r & !Self::TR22_MASK);
    }

    /// **TR21** `[21]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 21
    #[inline(always)]
    pub fn tr21(&self) -> bool {
        (self.value.read_bits() & Self::TR21_MASK) != 0
    }

    /// Set TR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr21(&self) {
        self.value.modify_bits(|r| r | Self::TR21_MASK);
    }

    /// Clear TR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr21(&self) {
        self.value.modify_bits(|r| r & !Self::TR21_MASK);
    }

    /// **TR20** `[20]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 10
    #[inline(always)]
    pub fn tr20(&self) -> bool {
        (self.value.read_bits() & Self::TR20_MASK) != 0
    }

    /// Set TR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr20(&self) {
        self.value.modify_bits(|r| r | Self::TR20_MASK);
    }

    /// Clear TR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr20(&self) {
        self.value.modify_bits(|r| r & !Self::TR20_MASK);
    }

    /// **TR19** `[19]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 19
    #[inline(always)]
    pub fn tr19(&self) -> bool {
        (self.value.read_bits() & Self::TR19_MASK) != 0
    }

    /// Set TR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr19(&self) {
        self.value.modify_bits(|r| r | Self::TR19_MASK);
    }

    /// Clear TR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr19(&self) {
        self.value.modify_bits(|r| r & !Self::TR19_MASK);
    }

    /// **TR18** `[18]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 18
    #[inline(always)]
    pub fn tr18(&self) -> bool {
        (self.value.read_bits() & Self::TR18_MASK) != 0
    }

    /// Set TR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr18(&self) {
        self.value.modify_bits(|r| r | Self::TR18_MASK);
    }

    /// Clear TR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr18(&self) {
        self.value.modify_bits(|r| r & !Self::TR18_MASK);
    }

    /// **TR17** `[17]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 17
    #[inline(always)]
    pub fn tr17(&self) -> bool {
        (self.value.read_bits() & Self::TR17_MASK) != 0
    }

    /// Set TR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr17(&self) {
        self.value.modify_bits(|r| r | Self::TR17_MASK);
    }

    /// Clear TR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr17(&self) {
        self.value.modify_bits(|r| r & !Self::TR17_MASK);
    }

    /// **TR16** `[16]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 16
    #[inline(always)]
    pub fn tr16(&self) -> bool {
        (self.value.read_bits() & Self::TR16_MASK) != 0
    }

    /// Set TR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr16(&self) {
        self.value.modify_bits(|r| r | Self::TR16_MASK);
    }

    /// Clear TR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr16(&self) {
        self.value.modify_bits(|r| r & !Self::TR16_MASK);
    }

    /// **TR15** `[15]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 15
    #[inline(always)]
    pub fn tr15(&self) -> bool {
        (self.value.read_bits() & Self::TR15_MASK) != 0
    }

    /// Set TR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr15(&self) {
        self.value.modify_bits(|r| r | Self::TR15_MASK);
    }

    /// Clear TR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr15(&self) {
        self.value.modify_bits(|r| r & !Self::TR15_MASK);
    }

    /// **TR14** `[14]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 14
    #[inline(always)]
    pub fn tr14(&self) -> bool {
        (self.value.read_bits() & Self::TR14_MASK) != 0
    }

    /// Set TR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr14(&self) {
        self.value.modify_bits(|r| r | Self::TR14_MASK);
    }

    /// Clear TR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr14(&self) {
        self.value.modify_bits(|r| r & !Self::TR14_MASK);
    }

    /// **TR13** `[13]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 13
    #[inline(always)]
    pub fn tr13(&self) -> bool {
        (self.value.read_bits() & Self::TR13_MASK) != 0
    }

    /// Set TR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr13(&self) {
        self.value.modify_bits(|r| r | Self::TR13_MASK);
    }

    /// Clear TR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr13(&self) {
        self.value.modify_bits(|r| r & !Self::TR13_MASK);
    }

    /// **TR12** `[12]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 12
    #[inline(always)]
    pub fn tr12(&self) -> bool {
        (self.value.read_bits() & Self::TR12_MASK) != 0
    }

    /// Set TR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr12(&self) {
        self.value.modify_bits(|r| r | Self::TR12_MASK);
    }

    /// Clear TR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr12(&self) {
        self.value.modify_bits(|r| r & !Self::TR12_MASK);
    }

    /// **TR11** `[11]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 11
    #[inline(always)]
    pub fn tr11(&self) -> bool {
        (self.value.read_bits() & Self::TR11_MASK) != 0
    }

    /// Set TR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr11(&self) {
        self.value.modify_bits(|r| r | Self::TR11_MASK);
    }

    /// Clear TR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr11(&self) {
        self.value.modify_bits(|r| r & !Self::TR11_MASK);
    }

    /// **TR10** `[10]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 10
    #[inline(always)]
    pub fn tr10(&self) -> bool {
        (self.value.read_bits() & Self::TR10_MASK) != 0
    }

    /// Set TR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr10(&self) {
        self.value.modify_bits(|r| r | Self::TR10_MASK);
    }

    /// Clear TR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr10(&self) {
        self.value.modify_bits(|r| r & !Self::TR10_MASK);
    }

    /// **TR9** `[9]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 9
    #[inline(always)]
    pub fn tr9(&self) -> bool {
        (self.value.read_bits() & Self::TR9_MASK) != 0
    }

    /// Set TR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr9(&self) {
        self.value.modify_bits(|r| r | Self::TR9_MASK);
    }

    /// Clear TR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr9(&self) {
        self.value.modify_bits(|r| r & !Self::TR9_MASK);
    }

    /// **TR8** `[8]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 8
    #[inline(always)]
    pub fn tr8(&self) -> bool {
        (self.value.read_bits() & Self::TR8_MASK) != 0
    }

    /// Set TR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr8(&self) {
        self.value.modify_bits(|r| r | Self::TR8_MASK);
    }

    /// Clear TR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr8(&self) {
        self.value.modify_bits(|r| r & !Self::TR8_MASK);
    }

    /// **TR7** `[7]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 7
    #[inline(always)]
    pub fn tr7(&self) -> bool {
        (self.value.read_bits() & Self::TR7_MASK) != 0
    }

    /// Set TR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr7(&self) {
        self.value.modify_bits(|r| r | Self::TR7_MASK);
    }

    /// Clear TR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr7(&self) {
        self.value.modify_bits(|r| r & !Self::TR7_MASK);
    }

    /// **TR6** `[6]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 6
    #[inline(always)]
    pub fn tr6(&self) -> bool {
        (self.value.read_bits() & Self::TR6_MASK) != 0
    }

    /// Set TR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr6(&self) {
        self.value.modify_bits(|r| r | Self::TR6_MASK);
    }

    /// Clear TR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr6(&self) {
        self.value.modify_bits(|r| r & !Self::TR6_MASK);
    }

    /// **TR5** `[5]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 5
    #[inline(always)]
    pub fn tr5(&self) -> bool {
        (self.value.read_bits() & Self::TR5_MASK) != 0
    }

    /// Set TR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr5(&self) {
        self.value.modify_bits(|r| r | Self::TR5_MASK);
    }

    /// Clear TR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr5(&self) {
        self.value.modify_bits(|r| r & !Self::TR5_MASK);
    }

    /// **TR4** `[4]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 4
    #[inline(always)]
    pub fn tr4(&self) -> bool {
        (self.value.read_bits() & Self::TR4_MASK) != 0
    }

    /// Set TR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr4(&self) {
        self.value.modify_bits(|r| r | Self::TR4_MASK);
    }

    /// Clear TR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr4(&self) {
        self.value.modify_bits(|r| r & !Self::TR4_MASK);
    }

    /// **TR3** `[3]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 3
    #[inline(always)]
    pub fn tr3(&self) -> bool {
        (self.value.read_bits() & Self::TR3_MASK) != 0
    }

    /// Set TR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr3(&self) {
        self.value.modify_bits(|r| r | Self::TR3_MASK);
    }

    /// Clear TR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr3(&self) {
        self.value.modify_bits(|r| r & !Self::TR3_MASK);
    }

    /// **TR2** `[2]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 2
    #[inline(always)]
    pub fn tr2(&self) -> bool {
        (self.value.read_bits() & Self::TR2_MASK) != 0
    }

    /// Set TR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr2(&self) {
        self.value.modify_bits(|r| r | Self::TR2_MASK);
    }

    /// Clear TR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr2(&self) {
        self.value.modify_bits(|r| r & !Self::TR2_MASK);
    }

    /// **TR1** `[1]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 1
    #[inline(always)]
    pub fn tr1(&self) -> bool {
        (self.value.read_bits() & Self::TR1_MASK) != 0
    }

    /// Set TR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr1(&self) {
        self.value.modify_bits(|r| r | Self::TR1_MASK);
    }

    /// Clear TR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr1(&self) {
        self.value.modify_bits(|r| r & !Self::TR1_MASK);
    }

    /// **TR0** `[0]` (RW)
    ///
    /// Rising trigger event configuration of
    /// line 0
    #[inline(always)]
    pub fn tr0(&self) -> bool {
        (self.value.read_bits() & Self::TR0_MASK) != 0
    }

    /// Set TR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr0(&self) {
        self.value.modify_bits(|r| r | Self::TR0_MASK);
    }

    /// Clear TR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr0(&self) {
        self.value.modify_bits(|r| r & !Self::TR0_MASK);
    }
}

/// RTENR value type for batch read/modify/write operations
///
/// Use `rtenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rtenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RtenrVal {
    bits: u32,
}

impl RtenrVal {
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

    /// Read **TR29** `[29:29]`
    ///
    /// Rising trigger event configuration of
    /// line 29
    #[inline(always)]
    pub const fn tr29(&self) -> bool {
        (self.bits & Rtenr::TR29_MASK) != 0
    }

    /// Read **TR28** `[28:28]`
    ///
    /// Rising trigger event configuration of
    /// line 28
    #[inline(always)]
    pub const fn tr28(&self) -> bool {
        (self.bits & Rtenr::TR28_MASK) != 0
    }

    /// Read **TR27** `[27:27]`
    ///
    /// Rising trigger event configuration of
    /// line 27
    #[inline(always)]
    pub const fn tr27(&self) -> bool {
        (self.bits & Rtenr::TR27_MASK) != 0
    }

    /// Read **TR26** `[26:26]`
    ///
    /// Rising trigger event configuration of
    /// line 26
    #[inline(always)]
    pub const fn tr26(&self) -> bool {
        (self.bits & Rtenr::TR26_MASK) != 0
    }

    /// Read **TR25** `[25:25]`
    ///
    /// Rising trigger event configuration of
    /// line 25
    #[inline(always)]
    pub const fn tr25(&self) -> bool {
        (self.bits & Rtenr::TR25_MASK) != 0
    }

    /// Read **TR24** `[24:24]`
    ///
    /// Rising trigger event configuration of
    /// line 24
    #[inline(always)]
    pub const fn tr24(&self) -> bool {
        (self.bits & Rtenr::TR24_MASK) != 0
    }

    /// Read **TR23** `[23:23]`
    ///
    /// Rising trigger event configuration of
    /// line 23
    #[inline(always)]
    pub const fn tr23(&self) -> bool {
        (self.bits & Rtenr::TR23_MASK) != 0
    }

    /// Read **TR22** `[22:22]`
    ///
    /// Rising trigger event configuration of
    /// line 22
    #[inline(always)]
    pub const fn tr22(&self) -> bool {
        (self.bits & Rtenr::TR22_MASK) != 0
    }

    /// Read **TR21** `[21:21]`
    ///
    /// Rising trigger event configuration of
    /// line 21
    #[inline(always)]
    pub const fn tr21(&self) -> bool {
        (self.bits & Rtenr::TR21_MASK) != 0
    }

    /// Read **TR20** `[20:20]`
    ///
    /// Rising trigger event configuration of
    /// line 10
    #[inline(always)]
    pub const fn tr20(&self) -> bool {
        (self.bits & Rtenr::TR20_MASK) != 0
    }

    /// Read **TR19** `[19:19]`
    ///
    /// Rising trigger event configuration of
    /// line 19
    #[inline(always)]
    pub const fn tr19(&self) -> bool {
        (self.bits & Rtenr::TR19_MASK) != 0
    }

    /// Read **TR18** `[18:18]`
    ///
    /// Rising trigger event configuration of
    /// line 18
    #[inline(always)]
    pub const fn tr18(&self) -> bool {
        (self.bits & Rtenr::TR18_MASK) != 0
    }

    /// Read **TR17** `[17:17]`
    ///
    /// Rising trigger event configuration of
    /// line 17
    #[inline(always)]
    pub const fn tr17(&self) -> bool {
        (self.bits & Rtenr::TR17_MASK) != 0
    }

    /// Read **TR16** `[16:16]`
    ///
    /// Rising trigger event configuration of
    /// line 16
    #[inline(always)]
    pub const fn tr16(&self) -> bool {
        (self.bits & Rtenr::TR16_MASK) != 0
    }

    /// Read **TR15** `[15:15]`
    ///
    /// Rising trigger event configuration of
    /// line 15
    #[inline(always)]
    pub const fn tr15(&self) -> bool {
        (self.bits & Rtenr::TR15_MASK) != 0
    }

    /// Read **TR14** `[14:14]`
    ///
    /// Rising trigger event configuration of
    /// line 14
    #[inline(always)]
    pub const fn tr14(&self) -> bool {
        (self.bits & Rtenr::TR14_MASK) != 0
    }

    /// Read **TR13** `[13:13]`
    ///
    /// Rising trigger event configuration of
    /// line 13
    #[inline(always)]
    pub const fn tr13(&self) -> bool {
        (self.bits & Rtenr::TR13_MASK) != 0
    }

    /// Read **TR12** `[12:12]`
    ///
    /// Rising trigger event configuration of
    /// line 12
    #[inline(always)]
    pub const fn tr12(&self) -> bool {
        (self.bits & Rtenr::TR12_MASK) != 0
    }

    /// Read **TR11** `[11:11]`
    ///
    /// Rising trigger event configuration of
    /// line 11
    #[inline(always)]
    pub const fn tr11(&self) -> bool {
        (self.bits & Rtenr::TR11_MASK) != 0
    }

    /// Read **TR10** `[10:10]`
    ///
    /// Rising trigger event configuration of
    /// line 10
    #[inline(always)]
    pub const fn tr10(&self) -> bool {
        (self.bits & Rtenr::TR10_MASK) != 0
    }

    /// Read **TR9** `[9:9]`
    ///
    /// Rising trigger event configuration of
    /// line 9
    #[inline(always)]
    pub const fn tr9(&self) -> bool {
        (self.bits & Rtenr::TR9_MASK) != 0
    }

    /// Read **TR8** `[8:8]`
    ///
    /// Rising trigger event configuration of
    /// line 8
    #[inline(always)]
    pub const fn tr8(&self) -> bool {
        (self.bits & Rtenr::TR8_MASK) != 0
    }

    /// Read **TR7** `[7:7]`
    ///
    /// Rising trigger event configuration of
    /// line 7
    #[inline(always)]
    pub const fn tr7(&self) -> bool {
        (self.bits & Rtenr::TR7_MASK) != 0
    }

    /// Read **TR6** `[6:6]`
    ///
    /// Rising trigger event configuration of
    /// line 6
    #[inline(always)]
    pub const fn tr6(&self) -> bool {
        (self.bits & Rtenr::TR6_MASK) != 0
    }

    /// Read **TR5** `[5:5]`
    ///
    /// Rising trigger event configuration of
    /// line 5
    #[inline(always)]
    pub const fn tr5(&self) -> bool {
        (self.bits & Rtenr::TR5_MASK) != 0
    }

    /// Read **TR4** `[4:4]`
    ///
    /// Rising trigger event configuration of
    /// line 4
    #[inline(always)]
    pub const fn tr4(&self) -> bool {
        (self.bits & Rtenr::TR4_MASK) != 0
    }

    /// Read **TR3** `[3:3]`
    ///
    /// Rising trigger event configuration of
    /// line 3
    #[inline(always)]
    pub const fn tr3(&self) -> bool {
        (self.bits & Rtenr::TR3_MASK) != 0
    }

    /// Read **TR2** `[2:2]`
    ///
    /// Rising trigger event configuration of
    /// line 2
    #[inline(always)]
    pub const fn tr2(&self) -> bool {
        (self.bits & Rtenr::TR2_MASK) != 0
    }

    /// Read **TR1** `[1:1]`
    ///
    /// Rising trigger event configuration of
    /// line 1
    #[inline(always)]
    pub const fn tr1(&self) -> bool {
        (self.bits & Rtenr::TR1_MASK) != 0
    }

    /// Read **TR0** `[0:0]`
    ///
    /// Rising trigger event configuration of
    /// line 0
    #[inline(always)]
    pub const fn tr0(&self) -> bool {
        (self.bits & Rtenr::TR0_MASK) != 0
    }

    /// Set **TR29** `[29:29]`
    ///
    /// Rising trigger event configuration of
    /// line 29
    #[inline(always)]
    pub const fn with_tr29(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR29_MASK
            } else {
                self.bits & !Rtenr::TR29_MASK
            },
        }
    }

    /// Set **TR28** `[28:28]`
    ///
    /// Rising trigger event configuration of
    /// line 28
    #[inline(always)]
    pub const fn with_tr28(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR28_MASK
            } else {
                self.bits & !Rtenr::TR28_MASK
            },
        }
    }

    /// Set **TR27** `[27:27]`
    ///
    /// Rising trigger event configuration of
    /// line 27
    #[inline(always)]
    pub const fn with_tr27(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR27_MASK
            } else {
                self.bits & !Rtenr::TR27_MASK
            },
        }
    }

    /// Set **TR26** `[26:26]`
    ///
    /// Rising trigger event configuration of
    /// line 26
    #[inline(always)]
    pub const fn with_tr26(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR26_MASK
            } else {
                self.bits & !Rtenr::TR26_MASK
            },
        }
    }

    /// Set **TR25** `[25:25]`
    ///
    /// Rising trigger event configuration of
    /// line 25
    #[inline(always)]
    pub const fn with_tr25(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR25_MASK
            } else {
                self.bits & !Rtenr::TR25_MASK
            },
        }
    }

    /// Set **TR24** `[24:24]`
    ///
    /// Rising trigger event configuration of
    /// line 24
    #[inline(always)]
    pub const fn with_tr24(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR24_MASK
            } else {
                self.bits & !Rtenr::TR24_MASK
            },
        }
    }

    /// Set **TR23** `[23:23]`
    ///
    /// Rising trigger event configuration of
    /// line 23
    #[inline(always)]
    pub const fn with_tr23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR23_MASK
            } else {
                self.bits & !Rtenr::TR23_MASK
            },
        }
    }

    /// Set **TR22** `[22:22]`
    ///
    /// Rising trigger event configuration of
    /// line 22
    #[inline(always)]
    pub const fn with_tr22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR22_MASK
            } else {
                self.bits & !Rtenr::TR22_MASK
            },
        }
    }

    /// Set **TR21** `[21:21]`
    ///
    /// Rising trigger event configuration of
    /// line 21
    #[inline(always)]
    pub const fn with_tr21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR21_MASK
            } else {
                self.bits & !Rtenr::TR21_MASK
            },
        }
    }

    /// Set **TR20** `[20:20]`
    ///
    /// Rising trigger event configuration of
    /// line 10
    #[inline(always)]
    pub const fn with_tr20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR20_MASK
            } else {
                self.bits & !Rtenr::TR20_MASK
            },
        }
    }

    /// Set **TR19** `[19:19]`
    ///
    /// Rising trigger event configuration of
    /// line 19
    #[inline(always)]
    pub const fn with_tr19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR19_MASK
            } else {
                self.bits & !Rtenr::TR19_MASK
            },
        }
    }

    /// Set **TR18** `[18:18]`
    ///
    /// Rising trigger event configuration of
    /// line 18
    #[inline(always)]
    pub const fn with_tr18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR18_MASK
            } else {
                self.bits & !Rtenr::TR18_MASK
            },
        }
    }

    /// Set **TR17** `[17:17]`
    ///
    /// Rising trigger event configuration of
    /// line 17
    #[inline(always)]
    pub const fn with_tr17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR17_MASK
            } else {
                self.bits & !Rtenr::TR17_MASK
            },
        }
    }

    /// Set **TR16** `[16:16]`
    ///
    /// Rising trigger event configuration of
    /// line 16
    #[inline(always)]
    pub const fn with_tr16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR16_MASK
            } else {
                self.bits & !Rtenr::TR16_MASK
            },
        }
    }

    /// Set **TR15** `[15:15]`
    ///
    /// Rising trigger event configuration of
    /// line 15
    #[inline(always)]
    pub const fn with_tr15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR15_MASK
            } else {
                self.bits & !Rtenr::TR15_MASK
            },
        }
    }

    /// Set **TR14** `[14:14]`
    ///
    /// Rising trigger event configuration of
    /// line 14
    #[inline(always)]
    pub const fn with_tr14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR14_MASK
            } else {
                self.bits & !Rtenr::TR14_MASK
            },
        }
    }

    /// Set **TR13** `[13:13]`
    ///
    /// Rising trigger event configuration of
    /// line 13
    #[inline(always)]
    pub const fn with_tr13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR13_MASK
            } else {
                self.bits & !Rtenr::TR13_MASK
            },
        }
    }

    /// Set **TR12** `[12:12]`
    ///
    /// Rising trigger event configuration of
    /// line 12
    #[inline(always)]
    pub const fn with_tr12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR12_MASK
            } else {
                self.bits & !Rtenr::TR12_MASK
            },
        }
    }

    /// Set **TR11** `[11:11]`
    ///
    /// Rising trigger event configuration of
    /// line 11
    #[inline(always)]
    pub const fn with_tr11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR11_MASK
            } else {
                self.bits & !Rtenr::TR11_MASK
            },
        }
    }

    /// Set **TR10** `[10:10]`
    ///
    /// Rising trigger event configuration of
    /// line 10
    #[inline(always)]
    pub const fn with_tr10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR10_MASK
            } else {
                self.bits & !Rtenr::TR10_MASK
            },
        }
    }

    /// Set **TR9** `[9:9]`
    ///
    /// Rising trigger event configuration of
    /// line 9
    #[inline(always)]
    pub const fn with_tr9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR9_MASK
            } else {
                self.bits & !Rtenr::TR9_MASK
            },
        }
    }

    /// Set **TR8** `[8:8]`
    ///
    /// Rising trigger event configuration of
    /// line 8
    #[inline(always)]
    pub const fn with_tr8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR8_MASK
            } else {
                self.bits & !Rtenr::TR8_MASK
            },
        }
    }

    /// Set **TR7** `[7:7]`
    ///
    /// Rising trigger event configuration of
    /// line 7
    #[inline(always)]
    pub const fn with_tr7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR7_MASK
            } else {
                self.bits & !Rtenr::TR7_MASK
            },
        }
    }

    /// Set **TR6** `[6:6]`
    ///
    /// Rising trigger event configuration of
    /// line 6
    #[inline(always)]
    pub const fn with_tr6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR6_MASK
            } else {
                self.bits & !Rtenr::TR6_MASK
            },
        }
    }

    /// Set **TR5** `[5:5]`
    ///
    /// Rising trigger event configuration of
    /// line 5
    #[inline(always)]
    pub const fn with_tr5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR5_MASK
            } else {
                self.bits & !Rtenr::TR5_MASK
            },
        }
    }

    /// Set **TR4** `[4:4]`
    ///
    /// Rising trigger event configuration of
    /// line 4
    #[inline(always)]
    pub const fn with_tr4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR4_MASK
            } else {
                self.bits & !Rtenr::TR4_MASK
            },
        }
    }

    /// Set **TR3** `[3:3]`
    ///
    /// Rising trigger event configuration of
    /// line 3
    #[inline(always)]
    pub const fn with_tr3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR3_MASK
            } else {
                self.bits & !Rtenr::TR3_MASK
            },
        }
    }

    /// Set **TR2** `[2:2]`
    ///
    /// Rising trigger event configuration of
    /// line 2
    #[inline(always)]
    pub const fn with_tr2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR2_MASK
            } else {
                self.bits & !Rtenr::TR2_MASK
            },
        }
    }

    /// Set **TR1** `[1:1]`
    ///
    /// Rising trigger event configuration of
    /// line 1
    #[inline(always)]
    pub const fn with_tr1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR1_MASK
            } else {
                self.bits & !Rtenr::TR1_MASK
            },
        }
    }

    /// Set **TR0** `[0:0]`
    ///
    /// Rising trigger event configuration of
    /// line 0
    #[inline(always)]
    pub const fn with_tr0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtenr::TR0_MASK
            } else {
                self.bits & !Rtenr::TR0_MASK
            },
        }
    }
}

impl Default for RtenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rtenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RtenrVal {
        RtenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RtenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RtenrVal) -> RtenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FTENR Register
///
/// Falling Trigger selection register
/// (EXTI_FTENR)
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TR29** `[29]` (RW)
///
///   Falling trigger event configuration of
///   line 29
///
/// - **TR28** `[28]` (RW)
///
///   Falling trigger event configuration of
///   line 28
///
/// - **TR27** `[27]` (RW)
///
///   Falling trigger event configuration of
///   line 27
///
/// - **TR26** `[26]` (RW)
///
///   Falling trigger event configuration of
///   line 26
///
/// - **TR25** `[25]` (RW)
///
///   Falling trigger event configuration of
///   line 25
///
/// - **TR24** `[24]` (RW)
///
///   Falling trigger event configuration of
///   line 24
///
/// - **TR23** `[23]` (RW)
///
///   Falling trigger event configuration of
///   line 23
///
/// - **TR22** `[22]` (RW)
///
///   Falling trigger event configuration of
///   line 22
///
/// - **TR21** `[21]` (RW)
///
///   Falling trigger event configuration of
///   line 21
///
/// - **TR20** `[20]` (RW)
///
///   Falling trigger event configuration of
///   line 20
///
/// - **TR19** `[19]` (RW)
///
///   Falling trigger event configuration of
///   line 19
///
/// - **TR18** `[18]` (RW)
///
///   Falling trigger event configuration of
///   line 18
///
/// - **TR17** `[17]` (RW)
///
///   Falling trigger event configuration of
///   line 17
///
/// - **TR16** `[16]` (RW)
///
///   Falling trigger event configuration of
///   line 16
///
/// - **TR15** `[15]` (RW)
///
///   Falling trigger event configuration of
///   line 15
///
/// - **TR14** `[14]` (RW)
///
///   Falling trigger event configuration of
///   line 14
///
/// - **TR13** `[13]` (RW)
///
///   Falling trigger event configuration of
///   line 13
///
/// - **TR12** `[12]` (RW)
///
///   Falling trigger event configuration of
///   line 12
///
/// - **TR11** `[11]` (RW)
///
///   Falling trigger event configuration of
///   line 11
///
/// - **TR10** `[10]` (RW)
///
///   Falling trigger event configuration of
///   line 10
///
/// - **TR9** `[9]` (RW)
///
///   Falling trigger event configuration of
///   line 9
///
/// - **TR8** `[8]` (RW)
///
///   Falling trigger event configuration of
///   line 8
///
/// - **TR7** `[7]` (RW)
///
///   Falling trigger event configuration of
///   line 7
///
/// - **TR6** `[6]` (RW)
///
///   Falling trigger event configuration of
///   line 6
///
/// - **TR5** `[5]` (RW)
///
///   Falling trigger event configuration of
///   line 5
///
/// - **TR4** `[4]` (RW)
///
///   Falling trigger event configuration of
///   line 4
///
/// - **TR3** `[3]` (RW)
///
///   Falling trigger event configuration of
///   line 3
///
/// - **TR2** `[2]` (RW)
///
///   Falling trigger event configuration of
///   line 2
///
/// - **TR1** `[1]` (RW)
///
///   Falling trigger event configuration of
///   line 1
///
/// - **TR0** `[0]` (RW)
///
///   Falling trigger event configuration of
///   line 0
#[repr(transparent)]
pub struct Ftenr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ftenr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ftenr {
    // ======== Field Constants ========

    /// **TR29**
    ///
    /// Falling trigger event configuration of
    /// line 29
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TR29_OFFSET: u32 = 29;
    pub const TR29_MASK: u32 = 0x20000000;
    pub const TR29: u32 = 1 << 29;

    /// **TR28**
    ///
    /// Falling trigger event configuration of
    /// line 28
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TR28_OFFSET: u32 = 28;
    pub const TR28_MASK: u32 = 0x10000000;
    pub const TR28: u32 = 1 << 28;

    /// **TR27**
    ///
    /// Falling trigger event configuration of
    /// line 27
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TR27_OFFSET: u32 = 27;
    pub const TR27_MASK: u32 = 0x08000000;
    pub const TR27: u32 = 1 << 27;

    /// **TR26**
    ///
    /// Falling trigger event configuration of
    /// line 26
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TR26_OFFSET: u32 = 26;
    pub const TR26_MASK: u32 = 0x04000000;
    pub const TR26: u32 = 1 << 26;

    /// **TR25**
    ///
    /// Falling trigger event configuration of
    /// line 25
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TR25_OFFSET: u32 = 25;
    pub const TR25_MASK: u32 = 0x02000000;
    pub const TR25: u32 = 1 << 25;

    /// **TR24**
    ///
    /// Falling trigger event configuration of
    /// line 24
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TR24_OFFSET: u32 = 24;
    pub const TR24_MASK: u32 = 0x01000000;
    pub const TR24: u32 = 1 << 24;

    /// **TR23**
    ///
    /// Falling trigger event configuration of
    /// line 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TR23_OFFSET: u32 = 23;
    pub const TR23_MASK: u32 = 0x00800000;
    pub const TR23: u32 = 1 << 23;

    /// **TR22**
    ///
    /// Falling trigger event configuration of
    /// line 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TR22_OFFSET: u32 = 22;
    pub const TR22_MASK: u32 = 0x00400000;
    pub const TR22: u32 = 1 << 22;

    /// **TR21**
    ///
    /// Falling trigger event configuration of
    /// line 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const TR21_OFFSET: u32 = 21;
    pub const TR21_MASK: u32 = 0x00200000;
    pub const TR21: u32 = 1 << 21;

    /// **TR20**
    ///
    /// Falling trigger event configuration of
    /// line 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const TR20_OFFSET: u32 = 20;
    pub const TR20_MASK: u32 = 0x00100000;
    pub const TR20: u32 = 1 << 20;

    /// **TR19**
    ///
    /// Falling trigger event configuration of
    /// line 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const TR19_OFFSET: u32 = 19;
    pub const TR19_MASK: u32 = 0x00080000;
    pub const TR19: u32 = 1 << 19;

    /// **TR18**
    ///
    /// Falling trigger event configuration of
    /// line 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TR18_OFFSET: u32 = 18;
    pub const TR18_MASK: u32 = 0x00040000;
    pub const TR18: u32 = 1 << 18;

    /// **TR17**
    ///
    /// Falling trigger event configuration of
    /// line 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const TR17_OFFSET: u32 = 17;
    pub const TR17_MASK: u32 = 0x00020000;
    pub const TR17: u32 = 1 << 17;

    /// **TR16**
    ///
    /// Falling trigger event configuration of
    /// line 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const TR16_OFFSET: u32 = 16;
    pub const TR16_MASK: u32 = 0x00010000;
    pub const TR16: u32 = 1 << 16;

    /// **TR15**
    ///
    /// Falling trigger event configuration of
    /// line 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TR15_OFFSET: u32 = 15;
    pub const TR15_MASK: u32 = 0x00008000;
    pub const TR15: u32 = 1 << 15;

    /// **TR14**
    ///
    /// Falling trigger event configuration of
    /// line 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TR14_OFFSET: u32 = 14;
    pub const TR14_MASK: u32 = 0x00004000;
    pub const TR14: u32 = 1 << 14;

    /// **TR13**
    ///
    /// Falling trigger event configuration of
    /// line 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TR13_OFFSET: u32 = 13;
    pub const TR13_MASK: u32 = 0x00002000;
    pub const TR13: u32 = 1 << 13;

    /// **TR12**
    ///
    /// Falling trigger event configuration of
    /// line 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TR12_OFFSET: u32 = 12;
    pub const TR12_MASK: u32 = 0x00001000;
    pub const TR12: u32 = 1 << 12;

    /// **TR11**
    ///
    /// Falling trigger event configuration of
    /// line 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TR11_OFFSET: u32 = 11;
    pub const TR11_MASK: u32 = 0x00000800;
    pub const TR11: u32 = 1 << 11;

    /// **TR10**
    ///
    /// Falling trigger event configuration of
    /// line 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TR10_OFFSET: u32 = 10;
    pub const TR10_MASK: u32 = 0x00000400;
    pub const TR10: u32 = 1 << 10;

    /// **TR9**
    ///
    /// Falling trigger event configuration of
    /// line 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TR9_OFFSET: u32 = 9;
    pub const TR9_MASK: u32 = 0x00000200;
    pub const TR9: u32 = 1 << 9;

    /// **TR8**
    ///
    /// Falling trigger event configuration of
    /// line 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TR8_OFFSET: u32 = 8;
    pub const TR8_MASK: u32 = 0x00000100;
    pub const TR8: u32 = 1 << 8;

    /// **TR7**
    ///
    /// Falling trigger event configuration of
    /// line 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TR7_OFFSET: u32 = 7;
    pub const TR7_MASK: u32 = 0x00000080;
    pub const TR7: u32 = 1 << 7;

    /// **TR6**
    ///
    /// Falling trigger event configuration of
    /// line 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TR6_OFFSET: u32 = 6;
    pub const TR6_MASK: u32 = 0x00000040;
    pub const TR6: u32 = 1 << 6;

    /// **TR5**
    ///
    /// Falling trigger event configuration of
    /// line 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TR5_OFFSET: u32 = 5;
    pub const TR5_MASK: u32 = 0x00000020;
    pub const TR5: u32 = 1 << 5;

    /// **TR4**
    ///
    /// Falling trigger event configuration of
    /// line 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TR4_OFFSET: u32 = 4;
    pub const TR4_MASK: u32 = 0x00000010;
    pub const TR4: u32 = 1 << 4;

    /// **TR3**
    ///
    /// Falling trigger event configuration of
    /// line 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TR3_OFFSET: u32 = 3;
    pub const TR3_MASK: u32 = 0x00000008;
    pub const TR3: u32 = 1 << 3;

    /// **TR2**
    ///
    /// Falling trigger event configuration of
    /// line 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TR2_OFFSET: u32 = 2;
    pub const TR2_MASK: u32 = 0x00000004;
    pub const TR2: u32 = 1 << 2;

    /// **TR1**
    ///
    /// Falling trigger event configuration of
    /// line 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TR1_OFFSET: u32 = 1;
    pub const TR1_MASK: u32 = 0x00000002;
    pub const TR1: u32 = 1 << 1;

    /// **TR0**
    ///
    /// Falling trigger event configuration of
    /// line 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TR0_OFFSET: u32 = 0;
    pub const TR0_MASK: u32 = 0x00000001;
    pub const TR0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TR29** `[29]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 29
    #[inline(always)]
    pub fn tr29(&self) -> bool {
        (self.value.read_bits() & Self::TR29_MASK) != 0
    }

    /// Set TR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr29(&self) {
        self.value.modify_bits(|r| r | Self::TR29_MASK);
    }

    /// Clear TR29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr29(&self) {
        self.value.modify_bits(|r| r & !Self::TR29_MASK);
    }

    /// **TR28** `[28]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 28
    #[inline(always)]
    pub fn tr28(&self) -> bool {
        (self.value.read_bits() & Self::TR28_MASK) != 0
    }

    /// Set TR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr28(&self) {
        self.value.modify_bits(|r| r | Self::TR28_MASK);
    }

    /// Clear TR28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr28(&self) {
        self.value.modify_bits(|r| r & !Self::TR28_MASK);
    }

    /// **TR27** `[27]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 27
    #[inline(always)]
    pub fn tr27(&self) -> bool {
        (self.value.read_bits() & Self::TR27_MASK) != 0
    }

    /// Set TR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr27(&self) {
        self.value.modify_bits(|r| r | Self::TR27_MASK);
    }

    /// Clear TR27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr27(&self) {
        self.value.modify_bits(|r| r & !Self::TR27_MASK);
    }

    /// **TR26** `[26]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 26
    #[inline(always)]
    pub fn tr26(&self) -> bool {
        (self.value.read_bits() & Self::TR26_MASK) != 0
    }

    /// Set TR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr26(&self) {
        self.value.modify_bits(|r| r | Self::TR26_MASK);
    }

    /// Clear TR26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr26(&self) {
        self.value.modify_bits(|r| r & !Self::TR26_MASK);
    }

    /// **TR25** `[25]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 25
    #[inline(always)]
    pub fn tr25(&self) -> bool {
        (self.value.read_bits() & Self::TR25_MASK) != 0
    }

    /// Set TR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr25(&self) {
        self.value.modify_bits(|r| r | Self::TR25_MASK);
    }

    /// Clear TR25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr25(&self) {
        self.value.modify_bits(|r| r & !Self::TR25_MASK);
    }

    /// **TR24** `[24]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 24
    #[inline(always)]
    pub fn tr24(&self) -> bool {
        (self.value.read_bits() & Self::TR24_MASK) != 0
    }

    /// Set TR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr24(&self) {
        self.value.modify_bits(|r| r | Self::TR24_MASK);
    }

    /// Clear TR24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr24(&self) {
        self.value.modify_bits(|r| r & !Self::TR24_MASK);
    }

    /// **TR23** `[23]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 23
    #[inline(always)]
    pub fn tr23(&self) -> bool {
        (self.value.read_bits() & Self::TR23_MASK) != 0
    }

    /// Set TR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr23(&self) {
        self.value.modify_bits(|r| r | Self::TR23_MASK);
    }

    /// Clear TR23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr23(&self) {
        self.value.modify_bits(|r| r & !Self::TR23_MASK);
    }

    /// **TR22** `[22]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 22
    #[inline(always)]
    pub fn tr22(&self) -> bool {
        (self.value.read_bits() & Self::TR22_MASK) != 0
    }

    /// Set TR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr22(&self) {
        self.value.modify_bits(|r| r | Self::TR22_MASK);
    }

    /// Clear TR22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr22(&self) {
        self.value.modify_bits(|r| r & !Self::TR22_MASK);
    }

    /// **TR21** `[21]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 21
    #[inline(always)]
    pub fn tr21(&self) -> bool {
        (self.value.read_bits() & Self::TR21_MASK) != 0
    }

    /// Set TR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr21(&self) {
        self.value.modify_bits(|r| r | Self::TR21_MASK);
    }

    /// Clear TR21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr21(&self) {
        self.value.modify_bits(|r| r & !Self::TR21_MASK);
    }

    /// **TR20** `[20]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 20
    #[inline(always)]
    pub fn tr20(&self) -> bool {
        (self.value.read_bits() & Self::TR20_MASK) != 0
    }

    /// Set TR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr20(&self) {
        self.value.modify_bits(|r| r | Self::TR20_MASK);
    }

    /// Clear TR20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr20(&self) {
        self.value.modify_bits(|r| r & !Self::TR20_MASK);
    }

    /// **TR19** `[19]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 19
    #[inline(always)]
    pub fn tr19(&self) -> bool {
        (self.value.read_bits() & Self::TR19_MASK) != 0
    }

    /// Set TR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr19(&self) {
        self.value.modify_bits(|r| r | Self::TR19_MASK);
    }

    /// Clear TR19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr19(&self) {
        self.value.modify_bits(|r| r & !Self::TR19_MASK);
    }

    /// **TR18** `[18]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 18
    #[inline(always)]
    pub fn tr18(&self) -> bool {
        (self.value.read_bits() & Self::TR18_MASK) != 0
    }

    /// Set TR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr18(&self) {
        self.value.modify_bits(|r| r | Self::TR18_MASK);
    }

    /// Clear TR18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr18(&self) {
        self.value.modify_bits(|r| r & !Self::TR18_MASK);
    }

    /// **TR17** `[17]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 17
    #[inline(always)]
    pub fn tr17(&self) -> bool {
        (self.value.read_bits() & Self::TR17_MASK) != 0
    }

    /// Set TR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr17(&self) {
        self.value.modify_bits(|r| r | Self::TR17_MASK);
    }

    /// Clear TR17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr17(&self) {
        self.value.modify_bits(|r| r & !Self::TR17_MASK);
    }

    /// **TR16** `[16]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 16
    #[inline(always)]
    pub fn tr16(&self) -> bool {
        (self.value.read_bits() & Self::TR16_MASK) != 0
    }

    /// Set TR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr16(&self) {
        self.value.modify_bits(|r| r | Self::TR16_MASK);
    }

    /// Clear TR16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr16(&self) {
        self.value.modify_bits(|r| r & !Self::TR16_MASK);
    }

    /// **TR15** `[15]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 15
    #[inline(always)]
    pub fn tr15(&self) -> bool {
        (self.value.read_bits() & Self::TR15_MASK) != 0
    }

    /// Set TR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr15(&self) {
        self.value.modify_bits(|r| r | Self::TR15_MASK);
    }

    /// Clear TR15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr15(&self) {
        self.value.modify_bits(|r| r & !Self::TR15_MASK);
    }

    /// **TR14** `[14]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 14
    #[inline(always)]
    pub fn tr14(&self) -> bool {
        (self.value.read_bits() & Self::TR14_MASK) != 0
    }

    /// Set TR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr14(&self) {
        self.value.modify_bits(|r| r | Self::TR14_MASK);
    }

    /// Clear TR14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr14(&self) {
        self.value.modify_bits(|r| r & !Self::TR14_MASK);
    }

    /// **TR13** `[13]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 13
    #[inline(always)]
    pub fn tr13(&self) -> bool {
        (self.value.read_bits() & Self::TR13_MASK) != 0
    }

    /// Set TR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr13(&self) {
        self.value.modify_bits(|r| r | Self::TR13_MASK);
    }

    /// Clear TR13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr13(&self) {
        self.value.modify_bits(|r| r & !Self::TR13_MASK);
    }

    /// **TR12** `[12]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 12
    #[inline(always)]
    pub fn tr12(&self) -> bool {
        (self.value.read_bits() & Self::TR12_MASK) != 0
    }

    /// Set TR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr12(&self) {
        self.value.modify_bits(|r| r | Self::TR12_MASK);
    }

    /// Clear TR12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr12(&self) {
        self.value.modify_bits(|r| r & !Self::TR12_MASK);
    }

    /// **TR11** `[11]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 11
    #[inline(always)]
    pub fn tr11(&self) -> bool {
        (self.value.read_bits() & Self::TR11_MASK) != 0
    }

    /// Set TR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr11(&self) {
        self.value.modify_bits(|r| r | Self::TR11_MASK);
    }

    /// Clear TR11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr11(&self) {
        self.value.modify_bits(|r| r & !Self::TR11_MASK);
    }

    /// **TR10** `[10]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 10
    #[inline(always)]
    pub fn tr10(&self) -> bool {
        (self.value.read_bits() & Self::TR10_MASK) != 0
    }

    /// Set TR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr10(&self) {
        self.value.modify_bits(|r| r | Self::TR10_MASK);
    }

    /// Clear TR10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr10(&self) {
        self.value.modify_bits(|r| r & !Self::TR10_MASK);
    }

    /// **TR9** `[9]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 9
    #[inline(always)]
    pub fn tr9(&self) -> bool {
        (self.value.read_bits() & Self::TR9_MASK) != 0
    }

    /// Set TR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr9(&self) {
        self.value.modify_bits(|r| r | Self::TR9_MASK);
    }

    /// Clear TR9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr9(&self) {
        self.value.modify_bits(|r| r & !Self::TR9_MASK);
    }

    /// **TR8** `[8]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 8
    #[inline(always)]
    pub fn tr8(&self) -> bool {
        (self.value.read_bits() & Self::TR8_MASK) != 0
    }

    /// Set TR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr8(&self) {
        self.value.modify_bits(|r| r | Self::TR8_MASK);
    }

    /// Clear TR8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr8(&self) {
        self.value.modify_bits(|r| r & !Self::TR8_MASK);
    }

    /// **TR7** `[7]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 7
    #[inline(always)]
    pub fn tr7(&self) -> bool {
        (self.value.read_bits() & Self::TR7_MASK) != 0
    }

    /// Set TR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr7(&self) {
        self.value.modify_bits(|r| r | Self::TR7_MASK);
    }

    /// Clear TR7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr7(&self) {
        self.value.modify_bits(|r| r & !Self::TR7_MASK);
    }

    /// **TR6** `[6]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 6
    #[inline(always)]
    pub fn tr6(&self) -> bool {
        (self.value.read_bits() & Self::TR6_MASK) != 0
    }

    /// Set TR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr6(&self) {
        self.value.modify_bits(|r| r | Self::TR6_MASK);
    }

    /// Clear TR6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr6(&self) {
        self.value.modify_bits(|r| r & !Self::TR6_MASK);
    }

    /// **TR5** `[5]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 5
    #[inline(always)]
    pub fn tr5(&self) -> bool {
        (self.value.read_bits() & Self::TR5_MASK) != 0
    }

    /// Set TR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr5(&self) {
        self.value.modify_bits(|r| r | Self::TR5_MASK);
    }

    /// Clear TR5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr5(&self) {
        self.value.modify_bits(|r| r & !Self::TR5_MASK);
    }

    /// **TR4** `[4]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 4
    #[inline(always)]
    pub fn tr4(&self) -> bool {
        (self.value.read_bits() & Self::TR4_MASK) != 0
    }

    /// Set TR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr4(&self) {
        self.value.modify_bits(|r| r | Self::TR4_MASK);
    }

    /// Clear TR4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr4(&self) {
        self.value.modify_bits(|r| r & !Self::TR4_MASK);
    }

    /// **TR3** `[3]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 3
    #[inline(always)]
    pub fn tr3(&self) -> bool {
        (self.value.read_bits() & Self::TR3_MASK) != 0
    }

    /// Set TR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr3(&self) {
        self.value.modify_bits(|r| r | Self::TR3_MASK);
    }

    /// Clear TR3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr3(&self) {
        self.value.modify_bits(|r| r & !Self::TR3_MASK);
    }

    /// **TR2** `[2]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 2
    #[inline(always)]
    pub fn tr2(&self) -> bool {
        (self.value.read_bits() & Self::TR2_MASK) != 0
    }

    /// Set TR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr2(&self) {
        self.value.modify_bits(|r| r | Self::TR2_MASK);
    }

    /// Clear TR2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr2(&self) {
        self.value.modify_bits(|r| r & !Self::TR2_MASK);
    }

    /// **TR1** `[1]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 1
    #[inline(always)]
    pub fn tr1(&self) -> bool {
        (self.value.read_bits() & Self::TR1_MASK) != 0
    }

    /// Set TR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr1(&self) {
        self.value.modify_bits(|r| r | Self::TR1_MASK);
    }

    /// Clear TR1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr1(&self) {
        self.value.modify_bits(|r| r & !Self::TR1_MASK);
    }

    /// **TR0** `[0]` (RW)
    ///
    /// Falling trigger event configuration of
    /// line 0
    #[inline(always)]
    pub fn tr0(&self) -> bool {
        (self.value.read_bits() & Self::TR0_MASK) != 0
    }

    /// Set TR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tr0(&self) {
        self.value.modify_bits(|r| r | Self::TR0_MASK);
    }

    /// Clear TR0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tr0(&self) {
        self.value.modify_bits(|r| r & !Self::TR0_MASK);
    }
}

/// FTENR value type for batch read/modify/write operations
///
/// Use `ftenr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ftenr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FtenrVal {
    bits: u32,
}

impl FtenrVal {
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

    /// Read **TR29** `[29:29]`
    ///
    /// Falling trigger event configuration of
    /// line 29
    #[inline(always)]
    pub const fn tr29(&self) -> bool {
        (self.bits & Ftenr::TR29_MASK) != 0
    }

    /// Read **TR28** `[28:28]`
    ///
    /// Falling trigger event configuration of
    /// line 28
    #[inline(always)]
    pub const fn tr28(&self) -> bool {
        (self.bits & Ftenr::TR28_MASK) != 0
    }

    /// Read **TR27** `[27:27]`
    ///
    /// Falling trigger event configuration of
    /// line 27
    #[inline(always)]
    pub const fn tr27(&self) -> bool {
        (self.bits & Ftenr::TR27_MASK) != 0
    }

    /// Read **TR26** `[26:26]`
    ///
    /// Falling trigger event configuration of
    /// line 26
    #[inline(always)]
    pub const fn tr26(&self) -> bool {
        (self.bits & Ftenr::TR26_MASK) != 0
    }

    /// Read **TR25** `[25:25]`
    ///
    /// Falling trigger event configuration of
    /// line 25
    #[inline(always)]
    pub const fn tr25(&self) -> bool {
        (self.bits & Ftenr::TR25_MASK) != 0
    }

    /// Read **TR24** `[24:24]`
    ///
    /// Falling trigger event configuration of
    /// line 24
    #[inline(always)]
    pub const fn tr24(&self) -> bool {
        (self.bits & Ftenr::TR24_MASK) != 0
    }

    /// Read **TR23** `[23:23]`
    ///
    /// Falling trigger event configuration of
    /// line 23
    #[inline(always)]
    pub const fn tr23(&self) -> bool {
        (self.bits & Ftenr::TR23_MASK) != 0
    }

    /// Read **TR22** `[22:22]`
    ///
    /// Falling trigger event configuration of
    /// line 22
    #[inline(always)]
    pub const fn tr22(&self) -> bool {
        (self.bits & Ftenr::TR22_MASK) != 0
    }

    /// Read **TR21** `[21:21]`
    ///
    /// Falling trigger event configuration of
    /// line 21
    #[inline(always)]
    pub const fn tr21(&self) -> bool {
        (self.bits & Ftenr::TR21_MASK) != 0
    }

    /// Read **TR20** `[20:20]`
    ///
    /// Falling trigger event configuration of
    /// line 20
    #[inline(always)]
    pub const fn tr20(&self) -> bool {
        (self.bits & Ftenr::TR20_MASK) != 0
    }

    /// Read **TR19** `[19:19]`
    ///
    /// Falling trigger event configuration of
    /// line 19
    #[inline(always)]
    pub const fn tr19(&self) -> bool {
        (self.bits & Ftenr::TR19_MASK) != 0
    }

    /// Read **TR18** `[18:18]`
    ///
    /// Falling trigger event configuration of
    /// line 18
    #[inline(always)]
    pub const fn tr18(&self) -> bool {
        (self.bits & Ftenr::TR18_MASK) != 0
    }

    /// Read **TR17** `[17:17]`
    ///
    /// Falling trigger event configuration of
    /// line 17
    #[inline(always)]
    pub const fn tr17(&self) -> bool {
        (self.bits & Ftenr::TR17_MASK) != 0
    }

    /// Read **TR16** `[16:16]`
    ///
    /// Falling trigger event configuration of
    /// line 16
    #[inline(always)]
    pub const fn tr16(&self) -> bool {
        (self.bits & Ftenr::TR16_MASK) != 0
    }

    /// Read **TR15** `[15:15]`
    ///
    /// Falling trigger event configuration of
    /// line 15
    #[inline(always)]
    pub const fn tr15(&self) -> bool {
        (self.bits & Ftenr::TR15_MASK) != 0
    }

    /// Read **TR14** `[14:14]`
    ///
    /// Falling trigger event configuration of
    /// line 14
    #[inline(always)]
    pub const fn tr14(&self) -> bool {
        (self.bits & Ftenr::TR14_MASK) != 0
    }

    /// Read **TR13** `[13:13]`
    ///
    /// Falling trigger event configuration of
    /// line 13
    #[inline(always)]
    pub const fn tr13(&self) -> bool {
        (self.bits & Ftenr::TR13_MASK) != 0
    }

    /// Read **TR12** `[12:12]`
    ///
    /// Falling trigger event configuration of
    /// line 12
    #[inline(always)]
    pub const fn tr12(&self) -> bool {
        (self.bits & Ftenr::TR12_MASK) != 0
    }

    /// Read **TR11** `[11:11]`
    ///
    /// Falling trigger event configuration of
    /// line 11
    #[inline(always)]
    pub const fn tr11(&self) -> bool {
        (self.bits & Ftenr::TR11_MASK) != 0
    }

    /// Read **TR10** `[10:10]`
    ///
    /// Falling trigger event configuration of
    /// line 10
    #[inline(always)]
    pub const fn tr10(&self) -> bool {
        (self.bits & Ftenr::TR10_MASK) != 0
    }

    /// Read **TR9** `[9:9]`
    ///
    /// Falling trigger event configuration of
    /// line 9
    #[inline(always)]
    pub const fn tr9(&self) -> bool {
        (self.bits & Ftenr::TR9_MASK) != 0
    }

    /// Read **TR8** `[8:8]`
    ///
    /// Falling trigger event configuration of
    /// line 8
    #[inline(always)]
    pub const fn tr8(&self) -> bool {
        (self.bits & Ftenr::TR8_MASK) != 0
    }

    /// Read **TR7** `[7:7]`
    ///
    /// Falling trigger event configuration of
    /// line 7
    #[inline(always)]
    pub const fn tr7(&self) -> bool {
        (self.bits & Ftenr::TR7_MASK) != 0
    }

    /// Read **TR6** `[6:6]`
    ///
    /// Falling trigger event configuration of
    /// line 6
    #[inline(always)]
    pub const fn tr6(&self) -> bool {
        (self.bits & Ftenr::TR6_MASK) != 0
    }

    /// Read **TR5** `[5:5]`
    ///
    /// Falling trigger event configuration of
    /// line 5
    #[inline(always)]
    pub const fn tr5(&self) -> bool {
        (self.bits & Ftenr::TR5_MASK) != 0
    }

    /// Read **TR4** `[4:4]`
    ///
    /// Falling trigger event configuration of
    /// line 4
    #[inline(always)]
    pub const fn tr4(&self) -> bool {
        (self.bits & Ftenr::TR4_MASK) != 0
    }

    /// Read **TR3** `[3:3]`
    ///
    /// Falling trigger event configuration of
    /// line 3
    #[inline(always)]
    pub const fn tr3(&self) -> bool {
        (self.bits & Ftenr::TR3_MASK) != 0
    }

    /// Read **TR2** `[2:2]`
    ///
    /// Falling trigger event configuration of
    /// line 2
    #[inline(always)]
    pub const fn tr2(&self) -> bool {
        (self.bits & Ftenr::TR2_MASK) != 0
    }

    /// Read **TR1** `[1:1]`
    ///
    /// Falling trigger event configuration of
    /// line 1
    #[inline(always)]
    pub const fn tr1(&self) -> bool {
        (self.bits & Ftenr::TR1_MASK) != 0
    }

    /// Read **TR0** `[0:0]`
    ///
    /// Falling trigger event configuration of
    /// line 0
    #[inline(always)]
    pub const fn tr0(&self) -> bool {
        (self.bits & Ftenr::TR0_MASK) != 0
    }

    /// Set **TR29** `[29:29]`
    ///
    /// Falling trigger event configuration of
    /// line 29
    #[inline(always)]
    pub const fn with_tr29(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR29_MASK
            } else {
                self.bits & !Ftenr::TR29_MASK
            },
        }
    }

    /// Set **TR28** `[28:28]`
    ///
    /// Falling trigger event configuration of
    /// line 28
    #[inline(always)]
    pub const fn with_tr28(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR28_MASK
            } else {
                self.bits & !Ftenr::TR28_MASK
            },
        }
    }

    /// Set **TR27** `[27:27]`
    ///
    /// Falling trigger event configuration of
    /// line 27
    #[inline(always)]
    pub const fn with_tr27(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR27_MASK
            } else {
                self.bits & !Ftenr::TR27_MASK
            },
        }
    }

    /// Set **TR26** `[26:26]`
    ///
    /// Falling trigger event configuration of
    /// line 26
    #[inline(always)]
    pub const fn with_tr26(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR26_MASK
            } else {
                self.bits & !Ftenr::TR26_MASK
            },
        }
    }

    /// Set **TR25** `[25:25]`
    ///
    /// Falling trigger event configuration of
    /// line 25
    #[inline(always)]
    pub const fn with_tr25(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR25_MASK
            } else {
                self.bits & !Ftenr::TR25_MASK
            },
        }
    }

    /// Set **TR24** `[24:24]`
    ///
    /// Falling trigger event configuration of
    /// line 24
    #[inline(always)]
    pub const fn with_tr24(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR24_MASK
            } else {
                self.bits & !Ftenr::TR24_MASK
            },
        }
    }

    /// Set **TR23** `[23:23]`
    ///
    /// Falling trigger event configuration of
    /// line 23
    #[inline(always)]
    pub const fn with_tr23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR23_MASK
            } else {
                self.bits & !Ftenr::TR23_MASK
            },
        }
    }

    /// Set **TR22** `[22:22]`
    ///
    /// Falling trigger event configuration of
    /// line 22
    #[inline(always)]
    pub const fn with_tr22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR22_MASK
            } else {
                self.bits & !Ftenr::TR22_MASK
            },
        }
    }

    /// Set **TR21** `[21:21]`
    ///
    /// Falling trigger event configuration of
    /// line 21
    #[inline(always)]
    pub const fn with_tr21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR21_MASK
            } else {
                self.bits & !Ftenr::TR21_MASK
            },
        }
    }

    /// Set **TR20** `[20:20]`
    ///
    /// Falling trigger event configuration of
    /// line 20
    #[inline(always)]
    pub const fn with_tr20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR20_MASK
            } else {
                self.bits & !Ftenr::TR20_MASK
            },
        }
    }

    /// Set **TR19** `[19:19]`
    ///
    /// Falling trigger event configuration of
    /// line 19
    #[inline(always)]
    pub const fn with_tr19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR19_MASK
            } else {
                self.bits & !Ftenr::TR19_MASK
            },
        }
    }

    /// Set **TR18** `[18:18]`
    ///
    /// Falling trigger event configuration of
    /// line 18
    #[inline(always)]
    pub const fn with_tr18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR18_MASK
            } else {
                self.bits & !Ftenr::TR18_MASK
            },
        }
    }

    /// Set **TR17** `[17:17]`
    ///
    /// Falling trigger event configuration of
    /// line 17
    #[inline(always)]
    pub const fn with_tr17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR17_MASK
            } else {
                self.bits & !Ftenr::TR17_MASK
            },
        }
    }

    /// Set **TR16** `[16:16]`
    ///
    /// Falling trigger event configuration of
    /// line 16
    #[inline(always)]
    pub const fn with_tr16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR16_MASK
            } else {
                self.bits & !Ftenr::TR16_MASK
            },
        }
    }

    /// Set **TR15** `[15:15]`
    ///
    /// Falling trigger event configuration of
    /// line 15
    #[inline(always)]
    pub const fn with_tr15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR15_MASK
            } else {
                self.bits & !Ftenr::TR15_MASK
            },
        }
    }

    /// Set **TR14** `[14:14]`
    ///
    /// Falling trigger event configuration of
    /// line 14
    #[inline(always)]
    pub const fn with_tr14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR14_MASK
            } else {
                self.bits & !Ftenr::TR14_MASK
            },
        }
    }

    /// Set **TR13** `[13:13]`
    ///
    /// Falling trigger event configuration of
    /// line 13
    #[inline(always)]
    pub const fn with_tr13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR13_MASK
            } else {
                self.bits & !Ftenr::TR13_MASK
            },
        }
    }

    /// Set **TR12** `[12:12]`
    ///
    /// Falling trigger event configuration of
    /// line 12
    #[inline(always)]
    pub const fn with_tr12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR12_MASK
            } else {
                self.bits & !Ftenr::TR12_MASK
            },
        }
    }

    /// Set **TR11** `[11:11]`
    ///
    /// Falling trigger event configuration of
    /// line 11
    #[inline(always)]
    pub const fn with_tr11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR11_MASK
            } else {
                self.bits & !Ftenr::TR11_MASK
            },
        }
    }

    /// Set **TR10** `[10:10]`
    ///
    /// Falling trigger event configuration of
    /// line 10
    #[inline(always)]
    pub const fn with_tr10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR10_MASK
            } else {
                self.bits & !Ftenr::TR10_MASK
            },
        }
    }

    /// Set **TR9** `[9:9]`
    ///
    /// Falling trigger event configuration of
    /// line 9
    #[inline(always)]
    pub const fn with_tr9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR9_MASK
            } else {
                self.bits & !Ftenr::TR9_MASK
            },
        }
    }

    /// Set **TR8** `[8:8]`
    ///
    /// Falling trigger event configuration of
    /// line 8
    #[inline(always)]
    pub const fn with_tr8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR8_MASK
            } else {
                self.bits & !Ftenr::TR8_MASK
            },
        }
    }

    /// Set **TR7** `[7:7]`
    ///
    /// Falling trigger event configuration of
    /// line 7
    #[inline(always)]
    pub const fn with_tr7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR7_MASK
            } else {
                self.bits & !Ftenr::TR7_MASK
            },
        }
    }

    /// Set **TR6** `[6:6]`
    ///
    /// Falling trigger event configuration of
    /// line 6
    #[inline(always)]
    pub const fn with_tr6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR6_MASK
            } else {
                self.bits & !Ftenr::TR6_MASK
            },
        }
    }

    /// Set **TR5** `[5:5]`
    ///
    /// Falling trigger event configuration of
    /// line 5
    #[inline(always)]
    pub const fn with_tr5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR5_MASK
            } else {
                self.bits & !Ftenr::TR5_MASK
            },
        }
    }

    /// Set **TR4** `[4:4]`
    ///
    /// Falling trigger event configuration of
    /// line 4
    #[inline(always)]
    pub const fn with_tr4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR4_MASK
            } else {
                self.bits & !Ftenr::TR4_MASK
            },
        }
    }

    /// Set **TR3** `[3:3]`
    ///
    /// Falling trigger event configuration of
    /// line 3
    #[inline(always)]
    pub const fn with_tr3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR3_MASK
            } else {
                self.bits & !Ftenr::TR3_MASK
            },
        }
    }

    /// Set **TR2** `[2:2]`
    ///
    /// Falling trigger event configuration of
    /// line 2
    #[inline(always)]
    pub const fn with_tr2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR2_MASK
            } else {
                self.bits & !Ftenr::TR2_MASK
            },
        }
    }

    /// Set **TR1** `[1:1]`
    ///
    /// Falling trigger event configuration of
    /// line 1
    #[inline(always)]
    pub const fn with_tr1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR1_MASK
            } else {
                self.bits & !Ftenr::TR1_MASK
            },
        }
    }

    /// Set **TR0** `[0:0]`
    ///
    /// Falling trigger event configuration of
    /// line 0
    #[inline(always)]
    pub const fn with_tr0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ftenr::TR0_MASK
            } else {
                self.bits & !Ftenr::TR0_MASK
            },
        }
    }
}

impl Default for FtenrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ftenr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FtenrVal {
        FtenrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FtenrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FtenrVal) -> FtenrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SWIEVR Register
///
/// Software interrupt event register
/// (EXTI_SWIEVR)
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SWIER29** `[29]` (RW)
///
///   Software Interrupt on line
///   29
///
/// - **SWIER28** `[28]` (RW)
///
///   Software Interrupt on line
///   28
///
/// - **SWIER27** `[27]` (RW)
///
///   Software Interrupt on line
///   27
///
/// - **SWIER26** `[26]` (RW)
///
///   Software Interrupt on line
///   26
///
/// - **SWIER25** `[25]` (RW)
///
///   Software Interrupt on line
///   25
///
/// - **SWIER24** `[24]` (RW)
///
///   Software Interrupt on line
///   24
///
/// - **SWIER23** `[23]` (RW)
///
///   Software Interrupt on line
///   23
///
/// - **SWIER22** `[22]` (RW)
///
///   Software Interrupt on line
///   22
///
/// - **SWIER21** `[21]` (RW)
///
///   Software Interrupt on line
///   21
///
/// - **SWIER20** `[20]` (RW)
///
///   Software Interrupt on line
///   20
///
/// - **SWIER19** `[19]` (RW)
///
///   Software Interrupt on line
///   19
///
/// - **SWIER18** `[18]` (RW)
///
///   Software Interrupt on line
///   18
///
/// - **SWIER17** `[17]` (RW)
///
///   Software Interrupt on line
///   17
///
/// - **SWIER16** `[16]` (RW)
///
///   Software Interrupt on line
///   16
///
/// - **SWIER15** `[15]` (RW)
///
///   Software Interrupt on line
///   15
///
/// - **SWIER14** `[14]` (RW)
///
///   Software Interrupt on line
///   14
///
/// - **SWIER13** `[13]` (RW)
///
///   Software Interrupt on line
///   13
///
/// - **SWIER12** `[12]` (RW)
///
///   Software Interrupt on line
///   12
///
/// - **SWIER11** `[11]` (RW)
///
///   Software Interrupt on line
///   11
///
/// - **SWIER10** `[10]` (RW)
///
///   Software Interrupt on line
///   10
///
/// - **SWIER9** `[9]` (RW)
///
///   Software Interrupt on line
///   9
///
/// - **SWIER8** `[8]` (RW)
///
///   Software Interrupt on line
///   8
///
/// - **SWIER7** `[7]` (RW)
///
///   Software Interrupt on line
///   7
///
/// - **SWIER6** `[6]` (RW)
///
///   Software Interrupt on line
///   6
///
/// - **SWIER5** `[5]` (RW)
///
///   Software Interrupt on line
///   5
///
/// - **SWIER4** `[4]` (RW)
///
///   Software Interrupt on line
///   4
///
/// - **SWIER3** `[3]` (RW)
///
///   Software Interrupt on line
///   3
///
/// - **SWIER2** `[2]` (RW)
///
///   Software Interrupt on line
///   2
///
/// - **SWIER1** `[1]` (RW)
///
///   Software Interrupt on line
///   1
///
/// - **SWIER0** `[0]` (RW)
///
///   Software Interrupt on line
///   0
#[repr(transparent)]
pub struct Swievr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Swievr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Swievr {
    // ======== Field Constants ========

    /// **SWIER29**
    ///
    /// Software Interrupt on line
    /// 29
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SWIER29_OFFSET: u32 = 29;
    pub const SWIER29_MASK: u32 = 0x20000000;
    pub const SWIER29: u32 = 1 << 29;

    /// **SWIER28**
    ///
    /// Software Interrupt on line
    /// 28
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SWIER28_OFFSET: u32 = 28;
    pub const SWIER28_MASK: u32 = 0x10000000;
    pub const SWIER28: u32 = 1 << 28;

    /// **SWIER27**
    ///
    /// Software Interrupt on line
    /// 27
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SWIER27_OFFSET: u32 = 27;
    pub const SWIER27_MASK: u32 = 0x08000000;
    pub const SWIER27: u32 = 1 << 27;

    /// **SWIER26**
    ///
    /// Software Interrupt on line
    /// 26
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SWIER26_OFFSET: u32 = 26;
    pub const SWIER26_MASK: u32 = 0x04000000;
    pub const SWIER26: u32 = 1 << 26;

    /// **SWIER25**
    ///
    /// Software Interrupt on line
    /// 25
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SWIER25_OFFSET: u32 = 25;
    pub const SWIER25_MASK: u32 = 0x02000000;
    pub const SWIER25: u32 = 1 << 25;

    /// **SWIER24**
    ///
    /// Software Interrupt on line
    /// 24
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SWIER24_OFFSET: u32 = 24;
    pub const SWIER24_MASK: u32 = 0x01000000;
    pub const SWIER24: u32 = 1 << 24;

    /// **SWIER23**
    ///
    /// Software Interrupt on line
    /// 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SWIER23_OFFSET: u32 = 23;
    pub const SWIER23_MASK: u32 = 0x00800000;
    pub const SWIER23: u32 = 1 << 23;

    /// **SWIER22**
    ///
    /// Software Interrupt on line
    /// 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SWIER22_OFFSET: u32 = 22;
    pub const SWIER22_MASK: u32 = 0x00400000;
    pub const SWIER22: u32 = 1 << 22;

    /// **SWIER21**
    ///
    /// Software Interrupt on line
    /// 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SWIER21_OFFSET: u32 = 21;
    pub const SWIER21_MASK: u32 = 0x00200000;
    pub const SWIER21: u32 = 1 << 21;

    /// **SWIER20**
    ///
    /// Software Interrupt on line
    /// 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SWIER20_OFFSET: u32 = 20;
    pub const SWIER20_MASK: u32 = 0x00100000;
    pub const SWIER20: u32 = 1 << 20;

    /// **SWIER19**
    ///
    /// Software Interrupt on line
    /// 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SWIER19_OFFSET: u32 = 19;
    pub const SWIER19_MASK: u32 = 0x00080000;
    pub const SWIER19: u32 = 1 << 19;

    /// **SWIER18**
    ///
    /// Software Interrupt on line
    /// 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SWIER18_OFFSET: u32 = 18;
    pub const SWIER18_MASK: u32 = 0x00040000;
    pub const SWIER18: u32 = 1 << 18;

    /// **SWIER17**
    ///
    /// Software Interrupt on line
    /// 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SWIER17_OFFSET: u32 = 17;
    pub const SWIER17_MASK: u32 = 0x00020000;
    pub const SWIER17: u32 = 1 << 17;

    /// **SWIER16**
    ///
    /// Software Interrupt on line
    /// 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SWIER16_OFFSET: u32 = 16;
    pub const SWIER16_MASK: u32 = 0x00010000;
    pub const SWIER16: u32 = 1 << 16;

    /// **SWIER15**
    ///
    /// Software Interrupt on line
    /// 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SWIER15_OFFSET: u32 = 15;
    pub const SWIER15_MASK: u32 = 0x00008000;
    pub const SWIER15: u32 = 1 << 15;

    /// **SWIER14**
    ///
    /// Software Interrupt on line
    /// 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SWIER14_OFFSET: u32 = 14;
    pub const SWIER14_MASK: u32 = 0x00004000;
    pub const SWIER14: u32 = 1 << 14;

    /// **SWIER13**
    ///
    /// Software Interrupt on line
    /// 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SWIER13_OFFSET: u32 = 13;
    pub const SWIER13_MASK: u32 = 0x00002000;
    pub const SWIER13: u32 = 1 << 13;

    /// **SWIER12**
    ///
    /// Software Interrupt on line
    /// 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SWIER12_OFFSET: u32 = 12;
    pub const SWIER12_MASK: u32 = 0x00001000;
    pub const SWIER12: u32 = 1 << 12;

    /// **SWIER11**
    ///
    /// Software Interrupt on line
    /// 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SWIER11_OFFSET: u32 = 11;
    pub const SWIER11_MASK: u32 = 0x00000800;
    pub const SWIER11: u32 = 1 << 11;

    /// **SWIER10**
    ///
    /// Software Interrupt on line
    /// 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SWIER10_OFFSET: u32 = 10;
    pub const SWIER10_MASK: u32 = 0x00000400;
    pub const SWIER10: u32 = 1 << 10;

    /// **SWIER9**
    ///
    /// Software Interrupt on line
    /// 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SWIER9_OFFSET: u32 = 9;
    pub const SWIER9_MASK: u32 = 0x00000200;
    pub const SWIER9: u32 = 1 << 9;

    /// **SWIER8**
    ///
    /// Software Interrupt on line
    /// 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SWIER8_OFFSET: u32 = 8;
    pub const SWIER8_MASK: u32 = 0x00000100;
    pub const SWIER8: u32 = 1 << 8;

    /// **SWIER7**
    ///
    /// Software Interrupt on line
    /// 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SWIER7_OFFSET: u32 = 7;
    pub const SWIER7_MASK: u32 = 0x00000080;
    pub const SWIER7: u32 = 1 << 7;

    /// **SWIER6**
    ///
    /// Software Interrupt on line
    /// 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SWIER6_OFFSET: u32 = 6;
    pub const SWIER6_MASK: u32 = 0x00000040;
    pub const SWIER6: u32 = 1 << 6;

    /// **SWIER5**
    ///
    /// Software Interrupt on line
    /// 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SWIER5_OFFSET: u32 = 5;
    pub const SWIER5_MASK: u32 = 0x00000020;
    pub const SWIER5: u32 = 1 << 5;

    /// **SWIER4**
    ///
    /// Software Interrupt on line
    /// 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SWIER4_OFFSET: u32 = 4;
    pub const SWIER4_MASK: u32 = 0x00000010;
    pub const SWIER4: u32 = 1 << 4;

    /// **SWIER3**
    ///
    /// Software Interrupt on line
    /// 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SWIER3_OFFSET: u32 = 3;
    pub const SWIER3_MASK: u32 = 0x00000008;
    pub const SWIER3: u32 = 1 << 3;

    /// **SWIER2**
    ///
    /// Software Interrupt on line
    /// 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SWIER2_OFFSET: u32 = 2;
    pub const SWIER2_MASK: u32 = 0x00000004;
    pub const SWIER2: u32 = 1 << 2;

    /// **SWIER1**
    ///
    /// Software Interrupt on line
    /// 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SWIER1_OFFSET: u32 = 1;
    pub const SWIER1_MASK: u32 = 0x00000002;
    pub const SWIER1: u32 = 1 << 1;

    /// **SWIER0**
    ///
    /// Software Interrupt on line
    /// 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SWIER0_OFFSET: u32 = 0;
    pub const SWIER0_MASK: u32 = 0x00000001;
    pub const SWIER0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SWIER29** `[29]` (RW)
    ///
    /// Software Interrupt on line
    /// 29
    #[inline(always)]
    pub fn swier29(&self) -> bool {
        (self.value.read_bits() & Self::SWIER29_MASK) != 0
    }

    /// Set SWIER29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier29(&self) {
        self.value.modify_bits(|r| r | Self::SWIER29_MASK);
    }

    /// Clear SWIER29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier29(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER29_MASK);
    }

    /// **SWIER28** `[28]` (RW)
    ///
    /// Software Interrupt on line
    /// 28
    #[inline(always)]
    pub fn swier28(&self) -> bool {
        (self.value.read_bits() & Self::SWIER28_MASK) != 0
    }

    /// Set SWIER28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier28(&self) {
        self.value.modify_bits(|r| r | Self::SWIER28_MASK);
    }

    /// Clear SWIER28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier28(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER28_MASK);
    }

    /// **SWIER27** `[27]` (RW)
    ///
    /// Software Interrupt on line
    /// 27
    #[inline(always)]
    pub fn swier27(&self) -> bool {
        (self.value.read_bits() & Self::SWIER27_MASK) != 0
    }

    /// Set SWIER27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier27(&self) {
        self.value.modify_bits(|r| r | Self::SWIER27_MASK);
    }

    /// Clear SWIER27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier27(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER27_MASK);
    }

    /// **SWIER26** `[26]` (RW)
    ///
    /// Software Interrupt on line
    /// 26
    #[inline(always)]
    pub fn swier26(&self) -> bool {
        (self.value.read_bits() & Self::SWIER26_MASK) != 0
    }

    /// Set SWIER26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier26(&self) {
        self.value.modify_bits(|r| r | Self::SWIER26_MASK);
    }

    /// Clear SWIER26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier26(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER26_MASK);
    }

    /// **SWIER25** `[25]` (RW)
    ///
    /// Software Interrupt on line
    /// 25
    #[inline(always)]
    pub fn swier25(&self) -> bool {
        (self.value.read_bits() & Self::SWIER25_MASK) != 0
    }

    /// Set SWIER25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier25(&self) {
        self.value.modify_bits(|r| r | Self::SWIER25_MASK);
    }

    /// Clear SWIER25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier25(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER25_MASK);
    }

    /// **SWIER24** `[24]` (RW)
    ///
    /// Software Interrupt on line
    /// 24
    #[inline(always)]
    pub fn swier24(&self) -> bool {
        (self.value.read_bits() & Self::SWIER24_MASK) != 0
    }

    /// Set SWIER24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier24(&self) {
        self.value.modify_bits(|r| r | Self::SWIER24_MASK);
    }

    /// Clear SWIER24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier24(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER24_MASK);
    }

    /// **SWIER23** `[23]` (RW)
    ///
    /// Software Interrupt on line
    /// 23
    #[inline(always)]
    pub fn swier23(&self) -> bool {
        (self.value.read_bits() & Self::SWIER23_MASK) != 0
    }

    /// Set SWIER23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier23(&self) {
        self.value.modify_bits(|r| r | Self::SWIER23_MASK);
    }

    /// Clear SWIER23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier23(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER23_MASK);
    }

    /// **SWIER22** `[22]` (RW)
    ///
    /// Software Interrupt on line
    /// 22
    #[inline(always)]
    pub fn swier22(&self) -> bool {
        (self.value.read_bits() & Self::SWIER22_MASK) != 0
    }

    /// Set SWIER22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier22(&self) {
        self.value.modify_bits(|r| r | Self::SWIER22_MASK);
    }

    /// Clear SWIER22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier22(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER22_MASK);
    }

    /// **SWIER21** `[21]` (RW)
    ///
    /// Software Interrupt on line
    /// 21
    #[inline(always)]
    pub fn swier21(&self) -> bool {
        (self.value.read_bits() & Self::SWIER21_MASK) != 0
    }

    /// Set SWIER21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier21(&self) {
        self.value.modify_bits(|r| r | Self::SWIER21_MASK);
    }

    /// Clear SWIER21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier21(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER21_MASK);
    }

    /// **SWIER20** `[20]` (RW)
    ///
    /// Software Interrupt on line
    /// 20
    #[inline(always)]
    pub fn swier20(&self) -> bool {
        (self.value.read_bits() & Self::SWIER20_MASK) != 0
    }

    /// Set SWIER20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier20(&self) {
        self.value.modify_bits(|r| r | Self::SWIER20_MASK);
    }

    /// Clear SWIER20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier20(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER20_MASK);
    }

    /// **SWIER19** `[19]` (RW)
    ///
    /// Software Interrupt on line
    /// 19
    #[inline(always)]
    pub fn swier19(&self) -> bool {
        (self.value.read_bits() & Self::SWIER19_MASK) != 0
    }

    /// Set SWIER19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier19(&self) {
        self.value.modify_bits(|r| r | Self::SWIER19_MASK);
    }

    /// Clear SWIER19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier19(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER19_MASK);
    }

    /// **SWIER18** `[18]` (RW)
    ///
    /// Software Interrupt on line
    /// 18
    #[inline(always)]
    pub fn swier18(&self) -> bool {
        (self.value.read_bits() & Self::SWIER18_MASK) != 0
    }

    /// Set SWIER18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier18(&self) {
        self.value.modify_bits(|r| r | Self::SWIER18_MASK);
    }

    /// Clear SWIER18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier18(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER18_MASK);
    }

    /// **SWIER17** `[17]` (RW)
    ///
    /// Software Interrupt on line
    /// 17
    #[inline(always)]
    pub fn swier17(&self) -> bool {
        (self.value.read_bits() & Self::SWIER17_MASK) != 0
    }

    /// Set SWIER17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier17(&self) {
        self.value.modify_bits(|r| r | Self::SWIER17_MASK);
    }

    /// Clear SWIER17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier17(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER17_MASK);
    }

    /// **SWIER16** `[16]` (RW)
    ///
    /// Software Interrupt on line
    /// 16
    #[inline(always)]
    pub fn swier16(&self) -> bool {
        (self.value.read_bits() & Self::SWIER16_MASK) != 0
    }

    /// Set SWIER16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier16(&self) {
        self.value.modify_bits(|r| r | Self::SWIER16_MASK);
    }

    /// Clear SWIER16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier16(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER16_MASK);
    }

    /// **SWIER15** `[15]` (RW)
    ///
    /// Software Interrupt on line
    /// 15
    #[inline(always)]
    pub fn swier15(&self) -> bool {
        (self.value.read_bits() & Self::SWIER15_MASK) != 0
    }

    /// Set SWIER15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier15(&self) {
        self.value.modify_bits(|r| r | Self::SWIER15_MASK);
    }

    /// Clear SWIER15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier15(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER15_MASK);
    }

    /// **SWIER14** `[14]` (RW)
    ///
    /// Software Interrupt on line
    /// 14
    #[inline(always)]
    pub fn swier14(&self) -> bool {
        (self.value.read_bits() & Self::SWIER14_MASK) != 0
    }

    /// Set SWIER14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier14(&self) {
        self.value.modify_bits(|r| r | Self::SWIER14_MASK);
    }

    /// Clear SWIER14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier14(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER14_MASK);
    }

    /// **SWIER13** `[13]` (RW)
    ///
    /// Software Interrupt on line
    /// 13
    #[inline(always)]
    pub fn swier13(&self) -> bool {
        (self.value.read_bits() & Self::SWIER13_MASK) != 0
    }

    /// Set SWIER13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier13(&self) {
        self.value.modify_bits(|r| r | Self::SWIER13_MASK);
    }

    /// Clear SWIER13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier13(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER13_MASK);
    }

    /// **SWIER12** `[12]` (RW)
    ///
    /// Software Interrupt on line
    /// 12
    #[inline(always)]
    pub fn swier12(&self) -> bool {
        (self.value.read_bits() & Self::SWIER12_MASK) != 0
    }

    /// Set SWIER12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier12(&self) {
        self.value.modify_bits(|r| r | Self::SWIER12_MASK);
    }

    /// Clear SWIER12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier12(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER12_MASK);
    }

    /// **SWIER11** `[11]` (RW)
    ///
    /// Software Interrupt on line
    /// 11
    #[inline(always)]
    pub fn swier11(&self) -> bool {
        (self.value.read_bits() & Self::SWIER11_MASK) != 0
    }

    /// Set SWIER11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier11(&self) {
        self.value.modify_bits(|r| r | Self::SWIER11_MASK);
    }

    /// Clear SWIER11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier11(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER11_MASK);
    }

    /// **SWIER10** `[10]` (RW)
    ///
    /// Software Interrupt on line
    /// 10
    #[inline(always)]
    pub fn swier10(&self) -> bool {
        (self.value.read_bits() & Self::SWIER10_MASK) != 0
    }

    /// Set SWIER10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier10(&self) {
        self.value.modify_bits(|r| r | Self::SWIER10_MASK);
    }

    /// Clear SWIER10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier10(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER10_MASK);
    }

    /// **SWIER9** `[9]` (RW)
    ///
    /// Software Interrupt on line
    /// 9
    #[inline(always)]
    pub fn swier9(&self) -> bool {
        (self.value.read_bits() & Self::SWIER9_MASK) != 0
    }

    /// Set SWIER9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier9(&self) {
        self.value.modify_bits(|r| r | Self::SWIER9_MASK);
    }

    /// Clear SWIER9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier9(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER9_MASK);
    }

    /// **SWIER8** `[8]` (RW)
    ///
    /// Software Interrupt on line
    /// 8
    #[inline(always)]
    pub fn swier8(&self) -> bool {
        (self.value.read_bits() & Self::SWIER8_MASK) != 0
    }

    /// Set SWIER8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier8(&self) {
        self.value.modify_bits(|r| r | Self::SWIER8_MASK);
    }

    /// Clear SWIER8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier8(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER8_MASK);
    }

    /// **SWIER7** `[7]` (RW)
    ///
    /// Software Interrupt on line
    /// 7
    #[inline(always)]
    pub fn swier7(&self) -> bool {
        (self.value.read_bits() & Self::SWIER7_MASK) != 0
    }

    /// Set SWIER7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier7(&self) {
        self.value.modify_bits(|r| r | Self::SWIER7_MASK);
    }

    /// Clear SWIER7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier7(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER7_MASK);
    }

    /// **SWIER6** `[6]` (RW)
    ///
    /// Software Interrupt on line
    /// 6
    #[inline(always)]
    pub fn swier6(&self) -> bool {
        (self.value.read_bits() & Self::SWIER6_MASK) != 0
    }

    /// Set SWIER6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier6(&self) {
        self.value.modify_bits(|r| r | Self::SWIER6_MASK);
    }

    /// Clear SWIER6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier6(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER6_MASK);
    }

    /// **SWIER5** `[5]` (RW)
    ///
    /// Software Interrupt on line
    /// 5
    #[inline(always)]
    pub fn swier5(&self) -> bool {
        (self.value.read_bits() & Self::SWIER5_MASK) != 0
    }

    /// Set SWIER5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier5(&self) {
        self.value.modify_bits(|r| r | Self::SWIER5_MASK);
    }

    /// Clear SWIER5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier5(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER5_MASK);
    }

    /// **SWIER4** `[4]` (RW)
    ///
    /// Software Interrupt on line
    /// 4
    #[inline(always)]
    pub fn swier4(&self) -> bool {
        (self.value.read_bits() & Self::SWIER4_MASK) != 0
    }

    /// Set SWIER4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier4(&self) {
        self.value.modify_bits(|r| r | Self::SWIER4_MASK);
    }

    /// Clear SWIER4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier4(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER4_MASK);
    }

    /// **SWIER3** `[3]` (RW)
    ///
    /// Software Interrupt on line
    /// 3
    #[inline(always)]
    pub fn swier3(&self) -> bool {
        (self.value.read_bits() & Self::SWIER3_MASK) != 0
    }

    /// Set SWIER3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier3(&self) {
        self.value.modify_bits(|r| r | Self::SWIER3_MASK);
    }

    /// Clear SWIER3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier3(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER3_MASK);
    }

    /// **SWIER2** `[2]` (RW)
    ///
    /// Software Interrupt on line
    /// 2
    #[inline(always)]
    pub fn swier2(&self) -> bool {
        (self.value.read_bits() & Self::SWIER2_MASK) != 0
    }

    /// Set SWIER2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier2(&self) {
        self.value.modify_bits(|r| r | Self::SWIER2_MASK);
    }

    /// Clear SWIER2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier2(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER2_MASK);
    }

    /// **SWIER1** `[1]` (RW)
    ///
    /// Software Interrupt on line
    /// 1
    #[inline(always)]
    pub fn swier1(&self) -> bool {
        (self.value.read_bits() & Self::SWIER1_MASK) != 0
    }

    /// Set SWIER1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier1(&self) {
        self.value.modify_bits(|r| r | Self::SWIER1_MASK);
    }

    /// Clear SWIER1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier1(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER1_MASK);
    }

    /// **SWIER0** `[0]` (RW)
    ///
    /// Software Interrupt on line
    /// 0
    #[inline(always)]
    pub fn swier0(&self) -> bool {
        (self.value.read_bits() & Self::SWIER0_MASK) != 0
    }

    /// Set SWIER0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swier0(&self) {
        self.value.modify_bits(|r| r | Self::SWIER0_MASK);
    }

    /// Clear SWIER0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swier0(&self) {
        self.value.modify_bits(|r| r & !Self::SWIER0_MASK);
    }
}

/// SWIEVR value type for batch read/modify/write operations
///
/// Use `swievr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swievr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwievrVal {
    bits: u32,
}

impl SwievrVal {
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

    /// Read **SWIER29** `[29:29]`
    ///
    /// Software Interrupt on line
    /// 29
    #[inline(always)]
    pub const fn swier29(&self) -> bool {
        (self.bits & Swievr::SWIER29_MASK) != 0
    }

    /// Read **SWIER28** `[28:28]`
    ///
    /// Software Interrupt on line
    /// 28
    #[inline(always)]
    pub const fn swier28(&self) -> bool {
        (self.bits & Swievr::SWIER28_MASK) != 0
    }

    /// Read **SWIER27** `[27:27]`
    ///
    /// Software Interrupt on line
    /// 27
    #[inline(always)]
    pub const fn swier27(&self) -> bool {
        (self.bits & Swievr::SWIER27_MASK) != 0
    }

    /// Read **SWIER26** `[26:26]`
    ///
    /// Software Interrupt on line
    /// 26
    #[inline(always)]
    pub const fn swier26(&self) -> bool {
        (self.bits & Swievr::SWIER26_MASK) != 0
    }

    /// Read **SWIER25** `[25:25]`
    ///
    /// Software Interrupt on line
    /// 25
    #[inline(always)]
    pub const fn swier25(&self) -> bool {
        (self.bits & Swievr::SWIER25_MASK) != 0
    }

    /// Read **SWIER24** `[24:24]`
    ///
    /// Software Interrupt on line
    /// 24
    #[inline(always)]
    pub const fn swier24(&self) -> bool {
        (self.bits & Swievr::SWIER24_MASK) != 0
    }

    /// Read **SWIER23** `[23:23]`
    ///
    /// Software Interrupt on line
    /// 23
    #[inline(always)]
    pub const fn swier23(&self) -> bool {
        (self.bits & Swievr::SWIER23_MASK) != 0
    }

    /// Read **SWIER22** `[22:22]`
    ///
    /// Software Interrupt on line
    /// 22
    #[inline(always)]
    pub const fn swier22(&self) -> bool {
        (self.bits & Swievr::SWIER22_MASK) != 0
    }

    /// Read **SWIER21** `[21:21]`
    ///
    /// Software Interrupt on line
    /// 21
    #[inline(always)]
    pub const fn swier21(&self) -> bool {
        (self.bits & Swievr::SWIER21_MASK) != 0
    }

    /// Read **SWIER20** `[20:20]`
    ///
    /// Software Interrupt on line
    /// 20
    #[inline(always)]
    pub const fn swier20(&self) -> bool {
        (self.bits & Swievr::SWIER20_MASK) != 0
    }

    /// Read **SWIER19** `[19:19]`
    ///
    /// Software Interrupt on line
    /// 19
    #[inline(always)]
    pub const fn swier19(&self) -> bool {
        (self.bits & Swievr::SWIER19_MASK) != 0
    }

    /// Read **SWIER18** `[18:18]`
    ///
    /// Software Interrupt on line
    /// 18
    #[inline(always)]
    pub const fn swier18(&self) -> bool {
        (self.bits & Swievr::SWIER18_MASK) != 0
    }

    /// Read **SWIER17** `[17:17]`
    ///
    /// Software Interrupt on line
    /// 17
    #[inline(always)]
    pub const fn swier17(&self) -> bool {
        (self.bits & Swievr::SWIER17_MASK) != 0
    }

    /// Read **SWIER16** `[16:16]`
    ///
    /// Software Interrupt on line
    /// 16
    #[inline(always)]
    pub const fn swier16(&self) -> bool {
        (self.bits & Swievr::SWIER16_MASK) != 0
    }

    /// Read **SWIER15** `[15:15]`
    ///
    /// Software Interrupt on line
    /// 15
    #[inline(always)]
    pub const fn swier15(&self) -> bool {
        (self.bits & Swievr::SWIER15_MASK) != 0
    }

    /// Read **SWIER14** `[14:14]`
    ///
    /// Software Interrupt on line
    /// 14
    #[inline(always)]
    pub const fn swier14(&self) -> bool {
        (self.bits & Swievr::SWIER14_MASK) != 0
    }

    /// Read **SWIER13** `[13:13]`
    ///
    /// Software Interrupt on line
    /// 13
    #[inline(always)]
    pub const fn swier13(&self) -> bool {
        (self.bits & Swievr::SWIER13_MASK) != 0
    }

    /// Read **SWIER12** `[12:12]`
    ///
    /// Software Interrupt on line
    /// 12
    #[inline(always)]
    pub const fn swier12(&self) -> bool {
        (self.bits & Swievr::SWIER12_MASK) != 0
    }

    /// Read **SWIER11** `[11:11]`
    ///
    /// Software Interrupt on line
    /// 11
    #[inline(always)]
    pub const fn swier11(&self) -> bool {
        (self.bits & Swievr::SWIER11_MASK) != 0
    }

    /// Read **SWIER10** `[10:10]`
    ///
    /// Software Interrupt on line
    /// 10
    #[inline(always)]
    pub const fn swier10(&self) -> bool {
        (self.bits & Swievr::SWIER10_MASK) != 0
    }

    /// Read **SWIER9** `[9:9]`
    ///
    /// Software Interrupt on line
    /// 9
    #[inline(always)]
    pub const fn swier9(&self) -> bool {
        (self.bits & Swievr::SWIER9_MASK) != 0
    }

    /// Read **SWIER8** `[8:8]`
    ///
    /// Software Interrupt on line
    /// 8
    #[inline(always)]
    pub const fn swier8(&self) -> bool {
        (self.bits & Swievr::SWIER8_MASK) != 0
    }

    /// Read **SWIER7** `[7:7]`
    ///
    /// Software Interrupt on line
    /// 7
    #[inline(always)]
    pub const fn swier7(&self) -> bool {
        (self.bits & Swievr::SWIER7_MASK) != 0
    }

    /// Read **SWIER6** `[6:6]`
    ///
    /// Software Interrupt on line
    /// 6
    #[inline(always)]
    pub const fn swier6(&self) -> bool {
        (self.bits & Swievr::SWIER6_MASK) != 0
    }

    /// Read **SWIER5** `[5:5]`
    ///
    /// Software Interrupt on line
    /// 5
    #[inline(always)]
    pub const fn swier5(&self) -> bool {
        (self.bits & Swievr::SWIER5_MASK) != 0
    }

    /// Read **SWIER4** `[4:4]`
    ///
    /// Software Interrupt on line
    /// 4
    #[inline(always)]
    pub const fn swier4(&self) -> bool {
        (self.bits & Swievr::SWIER4_MASK) != 0
    }

    /// Read **SWIER3** `[3:3]`
    ///
    /// Software Interrupt on line
    /// 3
    #[inline(always)]
    pub const fn swier3(&self) -> bool {
        (self.bits & Swievr::SWIER3_MASK) != 0
    }

    /// Read **SWIER2** `[2:2]`
    ///
    /// Software Interrupt on line
    /// 2
    #[inline(always)]
    pub const fn swier2(&self) -> bool {
        (self.bits & Swievr::SWIER2_MASK) != 0
    }

    /// Read **SWIER1** `[1:1]`
    ///
    /// Software Interrupt on line
    /// 1
    #[inline(always)]
    pub const fn swier1(&self) -> bool {
        (self.bits & Swievr::SWIER1_MASK) != 0
    }

    /// Read **SWIER0** `[0:0]`
    ///
    /// Software Interrupt on line
    /// 0
    #[inline(always)]
    pub const fn swier0(&self) -> bool {
        (self.bits & Swievr::SWIER0_MASK) != 0
    }

    /// Set **SWIER29** `[29:29]`
    ///
    /// Software Interrupt on line
    /// 29
    #[inline(always)]
    pub const fn with_swier29(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER29_MASK
            } else {
                self.bits & !Swievr::SWIER29_MASK
            },
        }
    }

    /// Set **SWIER28** `[28:28]`
    ///
    /// Software Interrupt on line
    /// 28
    #[inline(always)]
    pub const fn with_swier28(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER28_MASK
            } else {
                self.bits & !Swievr::SWIER28_MASK
            },
        }
    }

    /// Set **SWIER27** `[27:27]`
    ///
    /// Software Interrupt on line
    /// 27
    #[inline(always)]
    pub const fn with_swier27(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER27_MASK
            } else {
                self.bits & !Swievr::SWIER27_MASK
            },
        }
    }

    /// Set **SWIER26** `[26:26]`
    ///
    /// Software Interrupt on line
    /// 26
    #[inline(always)]
    pub const fn with_swier26(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER26_MASK
            } else {
                self.bits & !Swievr::SWIER26_MASK
            },
        }
    }

    /// Set **SWIER25** `[25:25]`
    ///
    /// Software Interrupt on line
    /// 25
    #[inline(always)]
    pub const fn with_swier25(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER25_MASK
            } else {
                self.bits & !Swievr::SWIER25_MASK
            },
        }
    }

    /// Set **SWIER24** `[24:24]`
    ///
    /// Software Interrupt on line
    /// 24
    #[inline(always)]
    pub const fn with_swier24(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER24_MASK
            } else {
                self.bits & !Swievr::SWIER24_MASK
            },
        }
    }

    /// Set **SWIER23** `[23:23]`
    ///
    /// Software Interrupt on line
    /// 23
    #[inline(always)]
    pub const fn with_swier23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER23_MASK
            } else {
                self.bits & !Swievr::SWIER23_MASK
            },
        }
    }

    /// Set **SWIER22** `[22:22]`
    ///
    /// Software Interrupt on line
    /// 22
    #[inline(always)]
    pub const fn with_swier22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER22_MASK
            } else {
                self.bits & !Swievr::SWIER22_MASK
            },
        }
    }

    /// Set **SWIER21** `[21:21]`
    ///
    /// Software Interrupt on line
    /// 21
    #[inline(always)]
    pub const fn with_swier21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER21_MASK
            } else {
                self.bits & !Swievr::SWIER21_MASK
            },
        }
    }

    /// Set **SWIER20** `[20:20]`
    ///
    /// Software Interrupt on line
    /// 20
    #[inline(always)]
    pub const fn with_swier20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER20_MASK
            } else {
                self.bits & !Swievr::SWIER20_MASK
            },
        }
    }

    /// Set **SWIER19** `[19:19]`
    ///
    /// Software Interrupt on line
    /// 19
    #[inline(always)]
    pub const fn with_swier19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER19_MASK
            } else {
                self.bits & !Swievr::SWIER19_MASK
            },
        }
    }

    /// Set **SWIER18** `[18:18]`
    ///
    /// Software Interrupt on line
    /// 18
    #[inline(always)]
    pub const fn with_swier18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER18_MASK
            } else {
                self.bits & !Swievr::SWIER18_MASK
            },
        }
    }

    /// Set **SWIER17** `[17:17]`
    ///
    /// Software Interrupt on line
    /// 17
    #[inline(always)]
    pub const fn with_swier17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER17_MASK
            } else {
                self.bits & !Swievr::SWIER17_MASK
            },
        }
    }

    /// Set **SWIER16** `[16:16]`
    ///
    /// Software Interrupt on line
    /// 16
    #[inline(always)]
    pub const fn with_swier16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER16_MASK
            } else {
                self.bits & !Swievr::SWIER16_MASK
            },
        }
    }

    /// Set **SWIER15** `[15:15]`
    ///
    /// Software Interrupt on line
    /// 15
    #[inline(always)]
    pub const fn with_swier15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER15_MASK
            } else {
                self.bits & !Swievr::SWIER15_MASK
            },
        }
    }

    /// Set **SWIER14** `[14:14]`
    ///
    /// Software Interrupt on line
    /// 14
    #[inline(always)]
    pub const fn with_swier14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER14_MASK
            } else {
                self.bits & !Swievr::SWIER14_MASK
            },
        }
    }

    /// Set **SWIER13** `[13:13]`
    ///
    /// Software Interrupt on line
    /// 13
    #[inline(always)]
    pub const fn with_swier13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER13_MASK
            } else {
                self.bits & !Swievr::SWIER13_MASK
            },
        }
    }

    /// Set **SWIER12** `[12:12]`
    ///
    /// Software Interrupt on line
    /// 12
    #[inline(always)]
    pub const fn with_swier12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER12_MASK
            } else {
                self.bits & !Swievr::SWIER12_MASK
            },
        }
    }

    /// Set **SWIER11** `[11:11]`
    ///
    /// Software Interrupt on line
    /// 11
    #[inline(always)]
    pub const fn with_swier11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER11_MASK
            } else {
                self.bits & !Swievr::SWIER11_MASK
            },
        }
    }

    /// Set **SWIER10** `[10:10]`
    ///
    /// Software Interrupt on line
    /// 10
    #[inline(always)]
    pub const fn with_swier10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER10_MASK
            } else {
                self.bits & !Swievr::SWIER10_MASK
            },
        }
    }

    /// Set **SWIER9** `[9:9]`
    ///
    /// Software Interrupt on line
    /// 9
    #[inline(always)]
    pub const fn with_swier9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER9_MASK
            } else {
                self.bits & !Swievr::SWIER9_MASK
            },
        }
    }

    /// Set **SWIER8** `[8:8]`
    ///
    /// Software Interrupt on line
    /// 8
    #[inline(always)]
    pub const fn with_swier8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER8_MASK
            } else {
                self.bits & !Swievr::SWIER8_MASK
            },
        }
    }

    /// Set **SWIER7** `[7:7]`
    ///
    /// Software Interrupt on line
    /// 7
    #[inline(always)]
    pub const fn with_swier7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER7_MASK
            } else {
                self.bits & !Swievr::SWIER7_MASK
            },
        }
    }

    /// Set **SWIER6** `[6:6]`
    ///
    /// Software Interrupt on line
    /// 6
    #[inline(always)]
    pub const fn with_swier6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER6_MASK
            } else {
                self.bits & !Swievr::SWIER6_MASK
            },
        }
    }

    /// Set **SWIER5** `[5:5]`
    ///
    /// Software Interrupt on line
    /// 5
    #[inline(always)]
    pub const fn with_swier5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER5_MASK
            } else {
                self.bits & !Swievr::SWIER5_MASK
            },
        }
    }

    /// Set **SWIER4** `[4:4]`
    ///
    /// Software Interrupt on line
    /// 4
    #[inline(always)]
    pub const fn with_swier4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER4_MASK
            } else {
                self.bits & !Swievr::SWIER4_MASK
            },
        }
    }

    /// Set **SWIER3** `[3:3]`
    ///
    /// Software Interrupt on line
    /// 3
    #[inline(always)]
    pub const fn with_swier3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER3_MASK
            } else {
                self.bits & !Swievr::SWIER3_MASK
            },
        }
    }

    /// Set **SWIER2** `[2:2]`
    ///
    /// Software Interrupt on line
    /// 2
    #[inline(always)]
    pub const fn with_swier2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER2_MASK
            } else {
                self.bits & !Swievr::SWIER2_MASK
            },
        }
    }

    /// Set **SWIER1** `[1:1]`
    ///
    /// Software Interrupt on line
    /// 1
    #[inline(always)]
    pub const fn with_swier1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER1_MASK
            } else {
                self.bits & !Swievr::SWIER1_MASK
            },
        }
    }

    /// Set **SWIER0** `[0:0]`
    ///
    /// Software Interrupt on line
    /// 0
    #[inline(always)]
    pub const fn with_swier0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swievr::SWIER0_MASK
            } else {
                self.bits & !Swievr::SWIER0_MASK
            },
        }
    }
}

impl Default for SwievrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Swievr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwievrVal {
        SwievrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwievrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwievrVal) -> SwievrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTFR Register
///
/// Interrupt flag register (EXTI_INTFR)
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IF29** `[29]` (RW)
///
///   Interrupt flag bit 29
///
/// - **IF28** `[28]` (RW)
///
///   Interrupt flag bit 28
///
/// - **IF27** `[27]` (RW)
///
///   Interrupt flag bit 27
///
/// - **IF26** `[26]` (RW)
///
///   Interrupt flag bit 26
///
/// - **IF25** `[25]` (RW)
///
///   Interrupt flag bit 25
///
/// - **IF24** `[24]` (RW)
///
///   Interrupt flag bit 24
///
/// - **IF23** `[23]` (RW)
///
///   Interrupt flag bit 23
///
/// - **IF22** `[22]` (RW)
///
///   Interrupt flag bit 22
///
/// - **IF21** `[21]` (RW)
///
///   Interrupt flag bit 21
///
/// - **IF20** `[20]` (RW)
///
///   Interrupt flag bit 20
///
/// - **IF19** `[19]` (RW)
///
///   Interrupt flag bit 19
///
/// - **IF18** `[18]` (RW)
///
///   Interrupt flag bit 18
///
/// - **IF17** `[17]` (RW)
///
///   Interrupt flag bit 17
///
/// - **IF16** `[16]` (RW)
///
///   Interrupt flag bit 16
///
/// - **IF15** `[15]` (RW)
///
///   Interrupt flag bit 15
///
/// - **IF14** `[14]` (RW)
///
///   Interrupt flag bit 14
///
/// - **IF13** `[13]` (RW)
///
///   Interrupt flag bit 13
///
/// - **IF12** `[12]` (RW)
///
///   Interrupt flag bit 12
///
/// - **IF11** `[11]` (RW)
///
///   Interrupt flag bit 11
///
/// - **IF10** `[10]` (RW)
///
///   Interrupt flag bit 10
///
/// - **IF9** `[9]` (RW)
///
///   Interrupt flag bit 9
///
/// - **IF8** `[8]` (RW)
///
///   Interrupt flag bit 8
///
/// - **IF7** `[7]` (RW)
///
///   Interrupt flag bit 7
///
/// - **IF6** `[6]` (RW)
///
///   Interrupt flag bit 6
///
/// - **IF5** `[5]` (RW)
///
///   Interrupt flag bit 5
///
/// - **IF4** `[4]` (RW)
///
///   Interrupt flag bit 4
///
/// - **IF3** `[3]` (RW)
///
///   Interrupt flag bit 3
///
/// - **IF2** `[2]` (RW)
///
///   Interrupt flag bit 2
///
/// - **IF1** `[1]` (RW)
///
///   Interrupt flag bit 1
///
/// - **IF0** `[0]` (RW)
///
///   Interrupt flag bit 0
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

    /// **IF29**
    ///
    /// Interrupt flag bit 29
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const IF29_OFFSET: u32 = 29;
    pub const IF29_MASK: u32 = 0x20000000;
    pub const IF29: u32 = 1 << 29;

    /// **IF28**
    ///
    /// Interrupt flag bit 28
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const IF28_OFFSET: u32 = 28;
    pub const IF28_MASK: u32 = 0x10000000;
    pub const IF28: u32 = 1 << 28;

    /// **IF27**
    ///
    /// Interrupt flag bit 27
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const IF27_OFFSET: u32 = 27;
    pub const IF27_MASK: u32 = 0x08000000;
    pub const IF27: u32 = 1 << 27;

    /// **IF26**
    ///
    /// Interrupt flag bit 26
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const IF26_OFFSET: u32 = 26;
    pub const IF26_MASK: u32 = 0x04000000;
    pub const IF26: u32 = 1 << 26;

    /// **IF25**
    ///
    /// Interrupt flag bit 25
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const IF25_OFFSET: u32 = 25;
    pub const IF25_MASK: u32 = 0x02000000;
    pub const IF25: u32 = 1 << 25;

    /// **IF24**
    ///
    /// Interrupt flag bit 24
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const IF24_OFFSET: u32 = 24;
    pub const IF24_MASK: u32 = 0x01000000;
    pub const IF24: u32 = 1 << 24;

    /// **IF23**
    ///
    /// Interrupt flag bit 23
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const IF23_OFFSET: u32 = 23;
    pub const IF23_MASK: u32 = 0x00800000;
    pub const IF23: u32 = 1 << 23;

    /// **IF22**
    ///
    /// Interrupt flag bit 22
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const IF22_OFFSET: u32 = 22;
    pub const IF22_MASK: u32 = 0x00400000;
    pub const IF22: u32 = 1 << 22;

    /// **IF21**
    ///
    /// Interrupt flag bit 21
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const IF21_OFFSET: u32 = 21;
    pub const IF21_MASK: u32 = 0x00200000;
    pub const IF21: u32 = 1 << 21;

    /// **IF20**
    ///
    /// Interrupt flag bit 20
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const IF20_OFFSET: u32 = 20;
    pub const IF20_MASK: u32 = 0x00100000;
    pub const IF20: u32 = 1 << 20;

    /// **IF19**
    ///
    /// Interrupt flag bit 19
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const IF19_OFFSET: u32 = 19;
    pub const IF19_MASK: u32 = 0x00080000;
    pub const IF19: u32 = 1 << 19;

    /// **IF18**
    ///
    /// Interrupt flag bit 18
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const IF18_OFFSET: u32 = 18;
    pub const IF18_MASK: u32 = 0x00040000;
    pub const IF18: u32 = 1 << 18;

    /// **IF17**
    ///
    /// Interrupt flag bit 17
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const IF17_OFFSET: u32 = 17;
    pub const IF17_MASK: u32 = 0x00020000;
    pub const IF17: u32 = 1 << 17;

    /// **IF16**
    ///
    /// Interrupt flag bit 16
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const IF16_OFFSET: u32 = 16;
    pub const IF16_MASK: u32 = 0x00010000;
    pub const IF16: u32 = 1 << 16;

    /// **IF15**
    ///
    /// Interrupt flag bit 15
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const IF15_OFFSET: u32 = 15;
    pub const IF15_MASK: u32 = 0x00008000;
    pub const IF15: u32 = 1 << 15;

    /// **IF14**
    ///
    /// Interrupt flag bit 14
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IF14_OFFSET: u32 = 14;
    pub const IF14_MASK: u32 = 0x00004000;
    pub const IF14: u32 = 1 << 14;

    /// **IF13**
    ///
    /// Interrupt flag bit 13
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IF13_OFFSET: u32 = 13;
    pub const IF13_MASK: u32 = 0x00002000;
    pub const IF13: u32 = 1 << 13;

    /// **IF12**
    ///
    /// Interrupt flag bit 12
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IF12_OFFSET: u32 = 12;
    pub const IF12_MASK: u32 = 0x00001000;
    pub const IF12: u32 = 1 << 12;

    /// **IF11**
    ///
    /// Interrupt flag bit 11
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IF11_OFFSET: u32 = 11;
    pub const IF11_MASK: u32 = 0x00000800;
    pub const IF11: u32 = 1 << 11;

    /// **IF10**
    ///
    /// Interrupt flag bit 10
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const IF10_OFFSET: u32 = 10;
    pub const IF10_MASK: u32 = 0x00000400;
    pub const IF10: u32 = 1 << 10;

    /// **IF9**
    ///
    /// Interrupt flag bit 9
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const IF9_OFFSET: u32 = 9;
    pub const IF9_MASK: u32 = 0x00000200;
    pub const IF9: u32 = 1 << 9;

    /// **IF8**
    ///
    /// Interrupt flag bit 8
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IF8_OFFSET: u32 = 8;
    pub const IF8_MASK: u32 = 0x00000100;
    pub const IF8: u32 = 1 << 8;

    /// **IF7**
    ///
    /// Interrupt flag bit 7
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const IF7_OFFSET: u32 = 7;
    pub const IF7_MASK: u32 = 0x00000080;
    pub const IF7: u32 = 1 << 7;

    /// **IF6**
    ///
    /// Interrupt flag bit 6
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IF6_OFFSET: u32 = 6;
    pub const IF6_MASK: u32 = 0x00000040;
    pub const IF6: u32 = 1 << 6;

    /// **IF5**
    ///
    /// Interrupt flag bit 5
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IF5_OFFSET: u32 = 5;
    pub const IF5_MASK: u32 = 0x00000020;
    pub const IF5: u32 = 1 << 5;

    /// **IF4**
    ///
    /// Interrupt flag bit 4
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IF4_OFFSET: u32 = 4;
    pub const IF4_MASK: u32 = 0x00000010;
    pub const IF4: u32 = 1 << 4;

    /// **IF3**
    ///
    /// Interrupt flag bit 3
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IF3_OFFSET: u32 = 3;
    pub const IF3_MASK: u32 = 0x00000008;
    pub const IF3: u32 = 1 << 3;

    /// **IF2**
    ///
    /// Interrupt flag bit 2
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IF2_OFFSET: u32 = 2;
    pub const IF2_MASK: u32 = 0x00000004;
    pub const IF2: u32 = 1 << 2;

    /// **IF1**
    ///
    /// Interrupt flag bit 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IF1_OFFSET: u32 = 1;
    pub const IF1_MASK: u32 = 0x00000002;
    pub const IF1: u32 = 1 << 1;

    /// **IF0**
    ///
    /// Interrupt flag bit 0
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IF0_OFFSET: u32 = 0;
    pub const IF0_MASK: u32 = 0x00000001;
    pub const IF0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IF29** `[29]` (RW)
    ///
    /// Interrupt flag bit 29
    #[inline(always)]
    pub fn if29(&self) -> bool {
        (self.value.read_bits() & Self::IF29_MASK) != 0
    }

    /// Set IF29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if29(&self) {
        self.value.modify_bits(|r| r | Self::IF29_MASK);
    }

    /// Clear IF29 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if29(&self) {
        self.value.modify_bits(|r| r & !Self::IF29_MASK);
    }

    /// **IF28** `[28]` (RW)
    ///
    /// Interrupt flag bit 28
    #[inline(always)]
    pub fn if28(&self) -> bool {
        (self.value.read_bits() & Self::IF28_MASK) != 0
    }

    /// Set IF28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if28(&self) {
        self.value.modify_bits(|r| r | Self::IF28_MASK);
    }

    /// Clear IF28 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if28(&self) {
        self.value.modify_bits(|r| r & !Self::IF28_MASK);
    }

    /// **IF27** `[27]` (RW)
    ///
    /// Interrupt flag bit 27
    #[inline(always)]
    pub fn if27(&self) -> bool {
        (self.value.read_bits() & Self::IF27_MASK) != 0
    }

    /// Set IF27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if27(&self) {
        self.value.modify_bits(|r| r | Self::IF27_MASK);
    }

    /// Clear IF27 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if27(&self) {
        self.value.modify_bits(|r| r & !Self::IF27_MASK);
    }

    /// **IF26** `[26]` (RW)
    ///
    /// Interrupt flag bit 26
    #[inline(always)]
    pub fn if26(&self) -> bool {
        (self.value.read_bits() & Self::IF26_MASK) != 0
    }

    /// Set IF26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if26(&self) {
        self.value.modify_bits(|r| r | Self::IF26_MASK);
    }

    /// Clear IF26 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if26(&self) {
        self.value.modify_bits(|r| r & !Self::IF26_MASK);
    }

    /// **IF25** `[25]` (RW)
    ///
    /// Interrupt flag bit 25
    #[inline(always)]
    pub fn if25(&self) -> bool {
        (self.value.read_bits() & Self::IF25_MASK) != 0
    }

    /// Set IF25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if25(&self) {
        self.value.modify_bits(|r| r | Self::IF25_MASK);
    }

    /// Clear IF25 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if25(&self) {
        self.value.modify_bits(|r| r & !Self::IF25_MASK);
    }

    /// **IF24** `[24]` (RW)
    ///
    /// Interrupt flag bit 24
    #[inline(always)]
    pub fn if24(&self) -> bool {
        (self.value.read_bits() & Self::IF24_MASK) != 0
    }

    /// Set IF24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if24(&self) {
        self.value.modify_bits(|r| r | Self::IF24_MASK);
    }

    /// Clear IF24 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if24(&self) {
        self.value.modify_bits(|r| r & !Self::IF24_MASK);
    }

    /// **IF23** `[23]` (RW)
    ///
    /// Interrupt flag bit 23
    #[inline(always)]
    pub fn if23(&self) -> bool {
        (self.value.read_bits() & Self::IF23_MASK) != 0
    }

    /// Set IF23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if23(&self) {
        self.value.modify_bits(|r| r | Self::IF23_MASK);
    }

    /// Clear IF23 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if23(&self) {
        self.value.modify_bits(|r| r & !Self::IF23_MASK);
    }

    /// **IF22** `[22]` (RW)
    ///
    /// Interrupt flag bit 22
    #[inline(always)]
    pub fn if22(&self) -> bool {
        (self.value.read_bits() & Self::IF22_MASK) != 0
    }

    /// Set IF22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if22(&self) {
        self.value.modify_bits(|r| r | Self::IF22_MASK);
    }

    /// Clear IF22 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if22(&self) {
        self.value.modify_bits(|r| r & !Self::IF22_MASK);
    }

    /// **IF21** `[21]` (RW)
    ///
    /// Interrupt flag bit 21
    #[inline(always)]
    pub fn if21(&self) -> bool {
        (self.value.read_bits() & Self::IF21_MASK) != 0
    }

    /// Set IF21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if21(&self) {
        self.value.modify_bits(|r| r | Self::IF21_MASK);
    }

    /// Clear IF21 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if21(&self) {
        self.value.modify_bits(|r| r & !Self::IF21_MASK);
    }

    /// **IF20** `[20]` (RW)
    ///
    /// Interrupt flag bit 20
    #[inline(always)]
    pub fn if20(&self) -> bool {
        (self.value.read_bits() & Self::IF20_MASK) != 0
    }

    /// Set IF20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if20(&self) {
        self.value.modify_bits(|r| r | Self::IF20_MASK);
    }

    /// Clear IF20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if20(&self) {
        self.value.modify_bits(|r| r & !Self::IF20_MASK);
    }

    /// **IF19** `[19]` (RW)
    ///
    /// Interrupt flag bit 19
    #[inline(always)]
    pub fn if19(&self) -> bool {
        (self.value.read_bits() & Self::IF19_MASK) != 0
    }

    /// Set IF19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if19(&self) {
        self.value.modify_bits(|r| r | Self::IF19_MASK);
    }

    /// Clear IF19 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if19(&self) {
        self.value.modify_bits(|r| r & !Self::IF19_MASK);
    }

    /// **IF18** `[18]` (RW)
    ///
    /// Interrupt flag bit 18
    #[inline(always)]
    pub fn if18(&self) -> bool {
        (self.value.read_bits() & Self::IF18_MASK) != 0
    }

    /// Set IF18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if18(&self) {
        self.value.modify_bits(|r| r | Self::IF18_MASK);
    }

    /// Clear IF18 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if18(&self) {
        self.value.modify_bits(|r| r & !Self::IF18_MASK);
    }

    /// **IF17** `[17]` (RW)
    ///
    /// Interrupt flag bit 17
    #[inline(always)]
    pub fn if17(&self) -> bool {
        (self.value.read_bits() & Self::IF17_MASK) != 0
    }

    /// Set IF17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if17(&self) {
        self.value.modify_bits(|r| r | Self::IF17_MASK);
    }

    /// Clear IF17 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if17(&self) {
        self.value.modify_bits(|r| r & !Self::IF17_MASK);
    }

    /// **IF16** `[16]` (RW)
    ///
    /// Interrupt flag bit 16
    #[inline(always)]
    pub fn if16(&self) -> bool {
        (self.value.read_bits() & Self::IF16_MASK) != 0
    }

    /// Set IF16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if16(&self) {
        self.value.modify_bits(|r| r | Self::IF16_MASK);
    }

    /// Clear IF16 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if16(&self) {
        self.value.modify_bits(|r| r & !Self::IF16_MASK);
    }

    /// **IF15** `[15]` (RW)
    ///
    /// Interrupt flag bit 15
    #[inline(always)]
    pub fn if15(&self) -> bool {
        (self.value.read_bits() & Self::IF15_MASK) != 0
    }

    /// Set IF15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if15(&self) {
        self.value.modify_bits(|r| r | Self::IF15_MASK);
    }

    /// Clear IF15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if15(&self) {
        self.value.modify_bits(|r| r & !Self::IF15_MASK);
    }

    /// **IF14** `[14]` (RW)
    ///
    /// Interrupt flag bit 14
    #[inline(always)]
    pub fn if14(&self) -> bool {
        (self.value.read_bits() & Self::IF14_MASK) != 0
    }

    /// Set IF14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if14(&self) {
        self.value.modify_bits(|r| r | Self::IF14_MASK);
    }

    /// Clear IF14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if14(&self) {
        self.value.modify_bits(|r| r & !Self::IF14_MASK);
    }

    /// **IF13** `[13]` (RW)
    ///
    /// Interrupt flag bit 13
    #[inline(always)]
    pub fn if13(&self) -> bool {
        (self.value.read_bits() & Self::IF13_MASK) != 0
    }

    /// Set IF13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if13(&self) {
        self.value.modify_bits(|r| r | Self::IF13_MASK);
    }

    /// Clear IF13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if13(&self) {
        self.value.modify_bits(|r| r & !Self::IF13_MASK);
    }

    /// **IF12** `[12]` (RW)
    ///
    /// Interrupt flag bit 12
    #[inline(always)]
    pub fn if12(&self) -> bool {
        (self.value.read_bits() & Self::IF12_MASK) != 0
    }

    /// Set IF12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if12(&self) {
        self.value.modify_bits(|r| r | Self::IF12_MASK);
    }

    /// Clear IF12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if12(&self) {
        self.value.modify_bits(|r| r & !Self::IF12_MASK);
    }

    /// **IF11** `[11]` (RW)
    ///
    /// Interrupt flag bit 11
    #[inline(always)]
    pub fn if11(&self) -> bool {
        (self.value.read_bits() & Self::IF11_MASK) != 0
    }

    /// Set IF11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if11(&self) {
        self.value.modify_bits(|r| r | Self::IF11_MASK);
    }

    /// Clear IF11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if11(&self) {
        self.value.modify_bits(|r| r & !Self::IF11_MASK);
    }

    /// **IF10** `[10]` (RW)
    ///
    /// Interrupt flag bit 10
    #[inline(always)]
    pub fn if10(&self) -> bool {
        (self.value.read_bits() & Self::IF10_MASK) != 0
    }

    /// Set IF10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if10(&self) {
        self.value.modify_bits(|r| r | Self::IF10_MASK);
    }

    /// Clear IF10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if10(&self) {
        self.value.modify_bits(|r| r & !Self::IF10_MASK);
    }

    /// **IF9** `[9]` (RW)
    ///
    /// Interrupt flag bit 9
    #[inline(always)]
    pub fn if9(&self) -> bool {
        (self.value.read_bits() & Self::IF9_MASK) != 0
    }

    /// Set IF9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if9(&self) {
        self.value.modify_bits(|r| r | Self::IF9_MASK);
    }

    /// Clear IF9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if9(&self) {
        self.value.modify_bits(|r| r & !Self::IF9_MASK);
    }

    /// **IF8** `[8]` (RW)
    ///
    /// Interrupt flag bit 8
    #[inline(always)]
    pub fn if8(&self) -> bool {
        (self.value.read_bits() & Self::IF8_MASK) != 0
    }

    /// Set IF8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if8(&self) {
        self.value.modify_bits(|r| r | Self::IF8_MASK);
    }

    /// Clear IF8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if8(&self) {
        self.value.modify_bits(|r| r & !Self::IF8_MASK);
    }

    /// **IF7** `[7]` (RW)
    ///
    /// Interrupt flag bit 7
    #[inline(always)]
    pub fn if7(&self) -> bool {
        (self.value.read_bits() & Self::IF7_MASK) != 0
    }

    /// Set IF7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if7(&self) {
        self.value.modify_bits(|r| r | Self::IF7_MASK);
    }

    /// Clear IF7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if7(&self) {
        self.value.modify_bits(|r| r & !Self::IF7_MASK);
    }

    /// **IF6** `[6]` (RW)
    ///
    /// Interrupt flag bit 6
    #[inline(always)]
    pub fn if6(&self) -> bool {
        (self.value.read_bits() & Self::IF6_MASK) != 0
    }

    /// Set IF6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if6(&self) {
        self.value.modify_bits(|r| r | Self::IF6_MASK);
    }

    /// Clear IF6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if6(&self) {
        self.value.modify_bits(|r| r & !Self::IF6_MASK);
    }

    /// **IF5** `[5]` (RW)
    ///
    /// Interrupt flag bit 5
    #[inline(always)]
    pub fn if5(&self) -> bool {
        (self.value.read_bits() & Self::IF5_MASK) != 0
    }

    /// Set IF5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if5(&self) {
        self.value.modify_bits(|r| r | Self::IF5_MASK);
    }

    /// Clear IF5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if5(&self) {
        self.value.modify_bits(|r| r & !Self::IF5_MASK);
    }

    /// **IF4** `[4]` (RW)
    ///
    /// Interrupt flag bit 4
    #[inline(always)]
    pub fn if4(&self) -> bool {
        (self.value.read_bits() & Self::IF4_MASK) != 0
    }

    /// Set IF4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if4(&self) {
        self.value.modify_bits(|r| r | Self::IF4_MASK);
    }

    /// Clear IF4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if4(&self) {
        self.value.modify_bits(|r| r & !Self::IF4_MASK);
    }

    /// **IF3** `[3]` (RW)
    ///
    /// Interrupt flag bit 3
    #[inline(always)]
    pub fn if3(&self) -> bool {
        (self.value.read_bits() & Self::IF3_MASK) != 0
    }

    /// Set IF3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if3(&self) {
        self.value.modify_bits(|r| r | Self::IF3_MASK);
    }

    /// Clear IF3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if3(&self) {
        self.value.modify_bits(|r| r & !Self::IF3_MASK);
    }

    /// **IF2** `[2]` (RW)
    ///
    /// Interrupt flag bit 2
    #[inline(always)]
    pub fn if2(&self) -> bool {
        (self.value.read_bits() & Self::IF2_MASK) != 0
    }

    /// Set IF2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if2(&self) {
        self.value.modify_bits(|r| r | Self::IF2_MASK);
    }

    /// Clear IF2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if2(&self) {
        self.value.modify_bits(|r| r & !Self::IF2_MASK);
    }

    /// **IF1** `[1]` (RW)
    ///
    /// Interrupt flag bit 1
    #[inline(always)]
    pub fn if1(&self) -> bool {
        (self.value.read_bits() & Self::IF1_MASK) != 0
    }

    /// Set IF1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if1(&self) {
        self.value.modify_bits(|r| r | Self::IF1_MASK);
    }

    /// Clear IF1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if1(&self) {
        self.value.modify_bits(|r| r & !Self::IF1_MASK);
    }

    /// **IF0** `[0]` (RW)
    ///
    /// Interrupt flag bit 0
    #[inline(always)]
    pub fn if0(&self) -> bool {
        (self.value.read_bits() & Self::IF0_MASK) != 0
    }

    /// Set IF0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if0(&self) {
        self.value.modify_bits(|r| r | Self::IF0_MASK);
    }

    /// Clear IF0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if0(&self) {
        self.value.modify_bits(|r| r & !Self::IF0_MASK);
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

    /// Read **IF29** `[29:29]`
    ///
    /// Interrupt flag bit 29
    #[inline(always)]
    pub const fn if29(&self) -> bool {
        (self.bits & Intfr::IF29_MASK) != 0
    }

    /// Read **IF28** `[28:28]`
    ///
    /// Interrupt flag bit 28
    #[inline(always)]
    pub const fn if28(&self) -> bool {
        (self.bits & Intfr::IF28_MASK) != 0
    }

    /// Read **IF27** `[27:27]`
    ///
    /// Interrupt flag bit 27
    #[inline(always)]
    pub const fn if27(&self) -> bool {
        (self.bits & Intfr::IF27_MASK) != 0
    }

    /// Read **IF26** `[26:26]`
    ///
    /// Interrupt flag bit 26
    #[inline(always)]
    pub const fn if26(&self) -> bool {
        (self.bits & Intfr::IF26_MASK) != 0
    }

    /// Read **IF25** `[25:25]`
    ///
    /// Interrupt flag bit 25
    #[inline(always)]
    pub const fn if25(&self) -> bool {
        (self.bits & Intfr::IF25_MASK) != 0
    }

    /// Read **IF24** `[24:24]`
    ///
    /// Interrupt flag bit 24
    #[inline(always)]
    pub const fn if24(&self) -> bool {
        (self.bits & Intfr::IF24_MASK) != 0
    }

    /// Read **IF23** `[23:23]`
    ///
    /// Interrupt flag bit 23
    #[inline(always)]
    pub const fn if23(&self) -> bool {
        (self.bits & Intfr::IF23_MASK) != 0
    }

    /// Read **IF22** `[22:22]`
    ///
    /// Interrupt flag bit 22
    #[inline(always)]
    pub const fn if22(&self) -> bool {
        (self.bits & Intfr::IF22_MASK) != 0
    }

    /// Read **IF21** `[21:21]`
    ///
    /// Interrupt flag bit 21
    #[inline(always)]
    pub const fn if21(&self) -> bool {
        (self.bits & Intfr::IF21_MASK) != 0
    }

    /// Read **IF20** `[20:20]`
    ///
    /// Interrupt flag bit 20
    #[inline(always)]
    pub const fn if20(&self) -> bool {
        (self.bits & Intfr::IF20_MASK) != 0
    }

    /// Read **IF19** `[19:19]`
    ///
    /// Interrupt flag bit 19
    #[inline(always)]
    pub const fn if19(&self) -> bool {
        (self.bits & Intfr::IF19_MASK) != 0
    }

    /// Read **IF18** `[18:18]`
    ///
    /// Interrupt flag bit 18
    #[inline(always)]
    pub const fn if18(&self) -> bool {
        (self.bits & Intfr::IF18_MASK) != 0
    }

    /// Read **IF17** `[17:17]`
    ///
    /// Interrupt flag bit 17
    #[inline(always)]
    pub const fn if17(&self) -> bool {
        (self.bits & Intfr::IF17_MASK) != 0
    }

    /// Read **IF16** `[16:16]`
    ///
    /// Interrupt flag bit 16
    #[inline(always)]
    pub const fn if16(&self) -> bool {
        (self.bits & Intfr::IF16_MASK) != 0
    }

    /// Read **IF15** `[15:15]`
    ///
    /// Interrupt flag bit 15
    #[inline(always)]
    pub const fn if15(&self) -> bool {
        (self.bits & Intfr::IF15_MASK) != 0
    }

    /// Read **IF14** `[14:14]`
    ///
    /// Interrupt flag bit 14
    #[inline(always)]
    pub const fn if14(&self) -> bool {
        (self.bits & Intfr::IF14_MASK) != 0
    }

    /// Read **IF13** `[13:13]`
    ///
    /// Interrupt flag bit 13
    #[inline(always)]
    pub const fn if13(&self) -> bool {
        (self.bits & Intfr::IF13_MASK) != 0
    }

    /// Read **IF12** `[12:12]`
    ///
    /// Interrupt flag bit 12
    #[inline(always)]
    pub const fn if12(&self) -> bool {
        (self.bits & Intfr::IF12_MASK) != 0
    }

    /// Read **IF11** `[11:11]`
    ///
    /// Interrupt flag bit 11
    #[inline(always)]
    pub const fn if11(&self) -> bool {
        (self.bits & Intfr::IF11_MASK) != 0
    }

    /// Read **IF10** `[10:10]`
    ///
    /// Interrupt flag bit 10
    #[inline(always)]
    pub const fn if10(&self) -> bool {
        (self.bits & Intfr::IF10_MASK) != 0
    }

    /// Read **IF9** `[9:9]`
    ///
    /// Interrupt flag bit 9
    #[inline(always)]
    pub const fn if9(&self) -> bool {
        (self.bits & Intfr::IF9_MASK) != 0
    }

    /// Read **IF8** `[8:8]`
    ///
    /// Interrupt flag bit 8
    #[inline(always)]
    pub const fn if8(&self) -> bool {
        (self.bits & Intfr::IF8_MASK) != 0
    }

    /// Read **IF7** `[7:7]`
    ///
    /// Interrupt flag bit 7
    #[inline(always)]
    pub const fn if7(&self) -> bool {
        (self.bits & Intfr::IF7_MASK) != 0
    }

    /// Read **IF6** `[6:6]`
    ///
    /// Interrupt flag bit 6
    #[inline(always)]
    pub const fn if6(&self) -> bool {
        (self.bits & Intfr::IF6_MASK) != 0
    }

    /// Read **IF5** `[5:5]`
    ///
    /// Interrupt flag bit 5
    #[inline(always)]
    pub const fn if5(&self) -> bool {
        (self.bits & Intfr::IF5_MASK) != 0
    }

    /// Read **IF4** `[4:4]`
    ///
    /// Interrupt flag bit 4
    #[inline(always)]
    pub const fn if4(&self) -> bool {
        (self.bits & Intfr::IF4_MASK) != 0
    }

    /// Read **IF3** `[3:3]`
    ///
    /// Interrupt flag bit 3
    #[inline(always)]
    pub const fn if3(&self) -> bool {
        (self.bits & Intfr::IF3_MASK) != 0
    }

    /// Read **IF2** `[2:2]`
    ///
    /// Interrupt flag bit 2
    #[inline(always)]
    pub const fn if2(&self) -> bool {
        (self.bits & Intfr::IF2_MASK) != 0
    }

    /// Read **IF1** `[1:1]`
    ///
    /// Interrupt flag bit 1
    #[inline(always)]
    pub const fn if1(&self) -> bool {
        (self.bits & Intfr::IF1_MASK) != 0
    }

    /// Read **IF0** `[0:0]`
    ///
    /// Interrupt flag bit 0
    #[inline(always)]
    pub const fn if0(&self) -> bool {
        (self.bits & Intfr::IF0_MASK) != 0
    }

    /// Set **IF29** `[29:29]`
    ///
    /// Interrupt flag bit 29
    #[inline(always)]
    pub const fn with_if29(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF29_MASK
            } else {
                self.bits & !Intfr::IF29_MASK
            },
        }
    }

    /// Set **IF28** `[28:28]`
    ///
    /// Interrupt flag bit 28
    #[inline(always)]
    pub const fn with_if28(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF28_MASK
            } else {
                self.bits & !Intfr::IF28_MASK
            },
        }
    }

    /// Set **IF27** `[27:27]`
    ///
    /// Interrupt flag bit 27
    #[inline(always)]
    pub const fn with_if27(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF27_MASK
            } else {
                self.bits & !Intfr::IF27_MASK
            },
        }
    }

    /// Set **IF26** `[26:26]`
    ///
    /// Interrupt flag bit 26
    #[inline(always)]
    pub const fn with_if26(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF26_MASK
            } else {
                self.bits & !Intfr::IF26_MASK
            },
        }
    }

    /// Set **IF25** `[25:25]`
    ///
    /// Interrupt flag bit 25
    #[inline(always)]
    pub const fn with_if25(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF25_MASK
            } else {
                self.bits & !Intfr::IF25_MASK
            },
        }
    }

    /// Set **IF24** `[24:24]`
    ///
    /// Interrupt flag bit 24
    #[inline(always)]
    pub const fn with_if24(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF24_MASK
            } else {
                self.bits & !Intfr::IF24_MASK
            },
        }
    }

    /// Set **IF23** `[23:23]`
    ///
    /// Interrupt flag bit 23
    #[inline(always)]
    pub const fn with_if23(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF23_MASK
            } else {
                self.bits & !Intfr::IF23_MASK
            },
        }
    }

    /// Set **IF22** `[22:22]`
    ///
    /// Interrupt flag bit 22
    #[inline(always)]
    pub const fn with_if22(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF22_MASK
            } else {
                self.bits & !Intfr::IF22_MASK
            },
        }
    }

    /// Set **IF21** `[21:21]`
    ///
    /// Interrupt flag bit 21
    #[inline(always)]
    pub const fn with_if21(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF21_MASK
            } else {
                self.bits & !Intfr::IF21_MASK
            },
        }
    }

    /// Set **IF20** `[20:20]`
    ///
    /// Interrupt flag bit 20
    #[inline(always)]
    pub const fn with_if20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF20_MASK
            } else {
                self.bits & !Intfr::IF20_MASK
            },
        }
    }

    /// Set **IF19** `[19:19]`
    ///
    /// Interrupt flag bit 19
    #[inline(always)]
    pub const fn with_if19(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF19_MASK
            } else {
                self.bits & !Intfr::IF19_MASK
            },
        }
    }

    /// Set **IF18** `[18:18]`
    ///
    /// Interrupt flag bit 18
    #[inline(always)]
    pub const fn with_if18(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF18_MASK
            } else {
                self.bits & !Intfr::IF18_MASK
            },
        }
    }

    /// Set **IF17** `[17:17]`
    ///
    /// Interrupt flag bit 17
    #[inline(always)]
    pub const fn with_if17(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF17_MASK
            } else {
                self.bits & !Intfr::IF17_MASK
            },
        }
    }

    /// Set **IF16** `[16:16]`
    ///
    /// Interrupt flag bit 16
    #[inline(always)]
    pub const fn with_if16(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF16_MASK
            } else {
                self.bits & !Intfr::IF16_MASK
            },
        }
    }

    /// Set **IF15** `[15:15]`
    ///
    /// Interrupt flag bit 15
    #[inline(always)]
    pub const fn with_if15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF15_MASK
            } else {
                self.bits & !Intfr::IF15_MASK
            },
        }
    }

    /// Set **IF14** `[14:14]`
    ///
    /// Interrupt flag bit 14
    #[inline(always)]
    pub const fn with_if14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF14_MASK
            } else {
                self.bits & !Intfr::IF14_MASK
            },
        }
    }

    /// Set **IF13** `[13:13]`
    ///
    /// Interrupt flag bit 13
    #[inline(always)]
    pub const fn with_if13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF13_MASK
            } else {
                self.bits & !Intfr::IF13_MASK
            },
        }
    }

    /// Set **IF12** `[12:12]`
    ///
    /// Interrupt flag bit 12
    #[inline(always)]
    pub const fn with_if12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF12_MASK
            } else {
                self.bits & !Intfr::IF12_MASK
            },
        }
    }

    /// Set **IF11** `[11:11]`
    ///
    /// Interrupt flag bit 11
    #[inline(always)]
    pub const fn with_if11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF11_MASK
            } else {
                self.bits & !Intfr::IF11_MASK
            },
        }
    }

    /// Set **IF10** `[10:10]`
    ///
    /// Interrupt flag bit 10
    #[inline(always)]
    pub const fn with_if10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF10_MASK
            } else {
                self.bits & !Intfr::IF10_MASK
            },
        }
    }

    /// Set **IF9** `[9:9]`
    ///
    /// Interrupt flag bit 9
    #[inline(always)]
    pub const fn with_if9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF9_MASK
            } else {
                self.bits & !Intfr::IF9_MASK
            },
        }
    }

    /// Set **IF8** `[8:8]`
    ///
    /// Interrupt flag bit 8
    #[inline(always)]
    pub const fn with_if8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF8_MASK
            } else {
                self.bits & !Intfr::IF8_MASK
            },
        }
    }

    /// Set **IF7** `[7:7]`
    ///
    /// Interrupt flag bit 7
    #[inline(always)]
    pub const fn with_if7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF7_MASK
            } else {
                self.bits & !Intfr::IF7_MASK
            },
        }
    }

    /// Set **IF6** `[6:6]`
    ///
    /// Interrupt flag bit 6
    #[inline(always)]
    pub const fn with_if6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF6_MASK
            } else {
                self.bits & !Intfr::IF6_MASK
            },
        }
    }

    /// Set **IF5** `[5:5]`
    ///
    /// Interrupt flag bit 5
    #[inline(always)]
    pub const fn with_if5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF5_MASK
            } else {
                self.bits & !Intfr::IF5_MASK
            },
        }
    }

    /// Set **IF4** `[4:4]`
    ///
    /// Interrupt flag bit 4
    #[inline(always)]
    pub const fn with_if4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF4_MASK
            } else {
                self.bits & !Intfr::IF4_MASK
            },
        }
    }

    /// Set **IF3** `[3:3]`
    ///
    /// Interrupt flag bit 3
    #[inline(always)]
    pub const fn with_if3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF3_MASK
            } else {
                self.bits & !Intfr::IF3_MASK
            },
        }
    }

    /// Set **IF2** `[2:2]`
    ///
    /// Interrupt flag bit 2
    #[inline(always)]
    pub const fn with_if2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF2_MASK
            } else {
                self.bits & !Intfr::IF2_MASK
            },
        }
    }

    /// Set **IF1** `[1:1]`
    ///
    /// Interrupt flag bit 1
    #[inline(always)]
    pub const fn with_if1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF1_MASK
            } else {
                self.bits & !Intfr::IF1_MASK
            },
        }
    }

    /// Set **IF0** `[0:0]`
    ///
    /// Interrupt flag bit 0
    #[inline(always)]
    pub const fn with_if0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intfr::IF0_MASK
            } else {
                self.bits & !Intfr::IF0_MASK
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

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Interrupt mask register
    /// (EXTI_INTENR)
    pub intenr: Intenr,
    /// 0x0004
    /// Event mask register (EXTI_EVENR)
    pub evenr: Evenr,
    /// 0x0008
    /// Rising Trigger selection register
    /// (EXTI_RTENR)
    pub rtenr: Rtenr,
    /// 0x000c
    /// Falling Trigger selection register
    /// (EXTI_FTENR)
    pub ftenr: Ftenr,
    /// 0x0010
    /// Software interrupt event register
    /// (EXTI_SWIEVR)
    pub swievr: Swievr,
    /// 0x0014
    /// Interrupt flag register (EXTI_INTFR)
    pub intfr: Intfr,
}
