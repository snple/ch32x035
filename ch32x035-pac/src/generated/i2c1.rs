//! I2C1
//! Inter integrated circuit

#![allow(dead_code)]

pub const BASE: u32 = 0x40005400;

/// CTLR1 Register
///
/// Control register 1
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SWRST** `[15]` (RW)
///
///   Software reset
///
/// - **PEC** `[12]` (RW)
///
///   Packet error checking
///
/// - **POS** `[11]` (RW)
///
///   Acknowledge/PEC Position (for data
///   reception)
///
/// - **ACK** `[10]` (RW)
///
///   Acknowledge enable
///
/// - **STOP** `[9]` (RW)
///
///   Stop generation
///
/// - **START** `[8]` (RW)
///
///   Start generation
///
/// - **NOSTRETCH** `[7]` (RW)
///
///   Clock stretching disable (Slave
///   mode)
///
/// - **ENGC** `[6]` (RW)
///
///   General call enable
///
/// - **ENPEC** `[5]` (RW)
///
///   PEC enable
///
/// - **ENARP** `[4]` (RW)
///
///   ARP enable
///
/// - **PE** `[0]` (RW)
///
///   Peripheral enable
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

    /// **SWRST**
    ///
    /// Software reset
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SWRST_OFFSET: u32 = 15;
    pub const SWRST_MASK: u32 = 0x00008000;
    pub const SWRST: u32 = 1 << 15;

    /// **PEC**
    ///
    /// Packet error checking
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PEC_OFFSET: u32 = 12;
    pub const PEC_MASK: u32 = 0x00001000;
    pub const PEC: u32 = 1 << 12;

    /// **POS**
    ///
    /// Acknowledge/PEC Position (for data
    /// reception)
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const POS_OFFSET: u32 = 11;
    pub const POS_MASK: u32 = 0x00000800;
    pub const POS: u32 = 1 << 11;

    /// **ACK**
    ///
    /// Acknowledge enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ACK_OFFSET: u32 = 10;
    pub const ACK_MASK: u32 = 0x00000400;
    pub const ACK: u32 = 1 << 10;

    /// **STOP**
    ///
    /// Stop generation
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STOP_OFFSET: u32 = 9;
    pub const STOP_MASK: u32 = 0x00000200;
    pub const STOP: u32 = 1 << 9;

    /// **START**
    ///
    /// Start generation
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const START_OFFSET: u32 = 8;
    pub const START_MASK: u32 = 0x00000100;
    pub const START: u32 = 1 << 8;

    /// **NOSTRETCH**
    ///
    /// Clock stretching disable (Slave
    /// mode)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const NOSTRETCH_OFFSET: u32 = 7;
    pub const NOSTRETCH_MASK: u32 = 0x00000080;
    pub const NOSTRETCH: u32 = 1 << 7;

    /// **ENGC**
    ///
    /// General call enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ENGC_OFFSET: u32 = 6;
    pub const ENGC_MASK: u32 = 0x00000040;
    pub const ENGC: u32 = 1 << 6;

    /// **ENPEC**
    ///
    /// PEC enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ENPEC_OFFSET: u32 = 5;
    pub const ENPEC_MASK: u32 = 0x00000020;
    pub const ENPEC: u32 = 1 << 5;

    /// **ENARP**
    ///
    /// ARP enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ENARP_OFFSET: u32 = 4;
    pub const ENARP_MASK: u32 = 0x00000010;
    pub const ENARP: u32 = 1 << 4;

    /// **PE**
    ///
    /// Peripheral enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PE_OFFSET: u32 = 0;
    pub const PE_MASK: u32 = 0x00000001;
    pub const PE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SWRST** `[15]` (RW)
    ///
    /// Software reset
    #[inline(always)]
    pub fn swrst(&self) -> bool {
        (self.value.read_bits() & Self::SWRST_MASK) != 0
    }

    /// Set SWRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_swrst(&self) {
        self.value.modify_bits(|r| r | Self::SWRST_MASK);
    }

    /// Clear SWRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_swrst(&self) {
        self.value.modify_bits(|r| r & !Self::SWRST_MASK);
    }

    /// **PEC** `[12]` (RW)
    ///
    /// Packet error checking
    #[inline(always)]
    pub fn pec(&self) -> bool {
        (self.value.read_bits() & Self::PEC_MASK) != 0
    }

    /// Set PEC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pec(&self) {
        self.value.modify_bits(|r| r | Self::PEC_MASK);
    }

    /// Clear PEC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pec(&self) {
        self.value.modify_bits(|r| r & !Self::PEC_MASK);
    }

    /// **POS** `[11]` (RW)
    ///
    /// Acknowledge/PEC Position (for data
    /// reception)
    #[inline(always)]
    pub fn pos(&self) -> bool {
        (self.value.read_bits() & Self::POS_MASK) != 0
    }

    /// Set POS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pos(&self) {
        self.value.modify_bits(|r| r | Self::POS_MASK);
    }

    /// Clear POS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pos(&self) {
        self.value.modify_bits(|r| r & !Self::POS_MASK);
    }

    /// **ACK** `[10]` (RW)
    ///
    /// Acknowledge enable
    #[inline(always)]
    pub fn ack(&self) -> bool {
        (self.value.read_bits() & Self::ACK_MASK) != 0
    }

    /// Set ACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ack(&self) {
        self.value.modify_bits(|r| r | Self::ACK_MASK);
    }

    /// Clear ACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ack(&self) {
        self.value.modify_bits(|r| r & !Self::ACK_MASK);
    }

    /// **STOP** `[9]` (RW)
    ///
    /// Stop generation
    #[inline(always)]
    pub fn stop(&self) -> bool {
        (self.value.read_bits() & Self::STOP_MASK) != 0
    }

    /// Set STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_stop(&self) {
        self.value.modify_bits(|r| r | Self::STOP_MASK);
    }

    /// Clear STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_stop(&self) {
        self.value.modify_bits(|r| r & !Self::STOP_MASK);
    }

    /// **START** `[8]` (RW)
    ///
    /// Start generation
    #[inline(always)]
    pub fn start(&self) -> bool {
        (self.value.read_bits() & Self::START_MASK) != 0
    }

    /// Set START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start(&self) {
        self.value.modify_bits(|r| r | Self::START_MASK);
    }

    /// Clear START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start(&self) {
        self.value.modify_bits(|r| r & !Self::START_MASK);
    }

    /// **NOSTRETCH** `[7]` (RW)
    ///
    /// Clock stretching disable (Slave
    /// mode)
    #[inline(always)]
    pub fn nostretch(&self) -> bool {
        (self.value.read_bits() & Self::NOSTRETCH_MASK) != 0
    }

    /// Set NOSTRETCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nostretch(&self) {
        self.value.modify_bits(|r| r | Self::NOSTRETCH_MASK);
    }

    /// Clear NOSTRETCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nostretch(&self) {
        self.value.modify_bits(|r| r & !Self::NOSTRETCH_MASK);
    }

    /// **ENGC** `[6]` (RW)
    ///
    /// General call enable
    #[inline(always)]
    pub fn engc(&self) -> bool {
        (self.value.read_bits() & Self::ENGC_MASK) != 0
    }

    /// Set ENGC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_engc(&self) {
        self.value.modify_bits(|r| r | Self::ENGC_MASK);
    }

    /// Clear ENGC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_engc(&self) {
        self.value.modify_bits(|r| r & !Self::ENGC_MASK);
    }

    /// **ENPEC** `[5]` (RW)
    ///
    /// PEC enable
    #[inline(always)]
    pub fn enpec(&self) -> bool {
        (self.value.read_bits() & Self::ENPEC_MASK) != 0
    }

    /// Set ENPEC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_enpec(&self) {
        self.value.modify_bits(|r| r | Self::ENPEC_MASK);
    }

    /// Clear ENPEC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_enpec(&self) {
        self.value.modify_bits(|r| r & !Self::ENPEC_MASK);
    }

    /// **ENARP** `[4]` (RW)
    ///
    /// ARP enable
    #[inline(always)]
    pub fn enarp(&self) -> bool {
        (self.value.read_bits() & Self::ENARP_MASK) != 0
    }

    /// Set ENARP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_enarp(&self) {
        self.value.modify_bits(|r| r | Self::ENARP_MASK);
    }

    /// Clear ENARP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_enarp(&self) {
        self.value.modify_bits(|r| r & !Self::ENARP_MASK);
    }

    /// **PE** `[0]` (RW)
    ///
    /// Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> bool {
        (self.value.read_bits() & Self::PE_MASK) != 0
    }

    /// Set PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pe(&self) {
        self.value.modify_bits(|r| r | Self::PE_MASK);
    }

    /// Clear PE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pe(&self) {
        self.value.modify_bits(|r| r & !Self::PE_MASK);
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

    /// Read **SWRST** `[15:15]`
    ///
    /// Software reset
    #[inline(always)]
    pub const fn swrst(&self) -> bool {
        (self.bits & Ctlr1::SWRST_MASK) != 0
    }

    /// Read **PEC** `[12:12]`
    ///
    /// Packet error checking
    #[inline(always)]
    pub const fn pec(&self) -> bool {
        (self.bits & Ctlr1::PEC_MASK) != 0
    }

    /// Read **POS** `[11:11]`
    ///
    /// Acknowledge/PEC Position (for data
    /// reception)
    #[inline(always)]
    pub const fn pos(&self) -> bool {
        (self.bits & Ctlr1::POS_MASK) != 0
    }

    /// Read **ACK** `[10:10]`
    ///
    /// Acknowledge enable
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        (self.bits & Ctlr1::ACK_MASK) != 0
    }

    /// Read **STOP** `[9:9]`
    ///
    /// Stop generation
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        (self.bits & Ctlr1::STOP_MASK) != 0
    }

    /// Read **START** `[8:8]`
    ///
    /// Start generation
    #[inline(always)]
    pub const fn start(&self) -> bool {
        (self.bits & Ctlr1::START_MASK) != 0
    }

    /// Read **NOSTRETCH** `[7:7]`
    ///
    /// Clock stretching disable (Slave
    /// mode)
    #[inline(always)]
    pub const fn nostretch(&self) -> bool {
        (self.bits & Ctlr1::NOSTRETCH_MASK) != 0
    }

    /// Read **ENGC** `[6:6]`
    ///
    /// General call enable
    #[inline(always)]
    pub const fn engc(&self) -> bool {
        (self.bits & Ctlr1::ENGC_MASK) != 0
    }

    /// Read **ENPEC** `[5:5]`
    ///
    /// PEC enable
    #[inline(always)]
    pub const fn enpec(&self) -> bool {
        (self.bits & Ctlr1::ENPEC_MASK) != 0
    }

    /// Read **ENARP** `[4:4]`
    ///
    /// ARP enable
    #[inline(always)]
    pub const fn enarp(&self) -> bool {
        (self.bits & Ctlr1::ENARP_MASK) != 0
    }

    /// Read **PE** `[0:0]`
    ///
    /// Peripheral enable
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        (self.bits & Ctlr1::PE_MASK) != 0
    }

    /// Set **SWRST** `[15:15]`
    ///
    /// Software reset
    #[inline(always)]
    pub const fn with_swrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::SWRST_MASK
            } else {
                self.bits & !Ctlr1::SWRST_MASK
            },
        }
    }

    /// Set **PEC** `[12:12]`
    ///
    /// Packet error checking
    #[inline(always)]
    pub const fn with_pec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::PEC_MASK
            } else {
                self.bits & !Ctlr1::PEC_MASK
            },
        }
    }

    /// Set **POS** `[11:11]`
    ///
    /// Acknowledge/PEC Position (for data
    /// reception)
    #[inline(always)]
    pub const fn with_pos(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::POS_MASK
            } else {
                self.bits & !Ctlr1::POS_MASK
            },
        }
    }

    /// Set **ACK** `[10:10]`
    ///
    /// Acknowledge enable
    #[inline(always)]
    pub const fn with_ack(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::ACK_MASK
            } else {
                self.bits & !Ctlr1::ACK_MASK
            },
        }
    }

    /// Set **STOP** `[9:9]`
    ///
    /// Stop generation
    #[inline(always)]
    pub const fn with_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::STOP_MASK
            } else {
                self.bits & !Ctlr1::STOP_MASK
            },
        }
    }

    /// Set **START** `[8:8]`
    ///
    /// Start generation
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::START_MASK
            } else {
                self.bits & !Ctlr1::START_MASK
            },
        }
    }

    /// Set **NOSTRETCH** `[7:7]`
    ///
    /// Clock stretching disable (Slave
    /// mode)
    #[inline(always)]
    pub const fn with_nostretch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::NOSTRETCH_MASK
            } else {
                self.bits & !Ctlr1::NOSTRETCH_MASK
            },
        }
    }

    /// Set **ENGC** `[6:6]`
    ///
    /// General call enable
    #[inline(always)]
    pub const fn with_engc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::ENGC_MASK
            } else {
                self.bits & !Ctlr1::ENGC_MASK
            },
        }
    }

    /// Set **ENPEC** `[5:5]`
    ///
    /// PEC enable
    #[inline(always)]
    pub const fn with_enpec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::ENPEC_MASK
            } else {
                self.bits & !Ctlr1::ENPEC_MASK
            },
        }
    }

    /// Set **ENARP** `[4:4]`
    ///
    /// ARP enable
    #[inline(always)]
    pub const fn with_enarp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::ENARP_MASK
            } else {
                self.bits & !Ctlr1::ENARP_MASK
            },
        }
    }

    /// Set **PE** `[0:0]`
    ///
    /// Peripheral enable
    #[inline(always)]
    pub const fn with_pe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::PE_MASK
            } else {
                self.bits & !Ctlr1::PE_MASK
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
/// Control register 2
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LAST** `[12]` (RW)
///
///   DMA last transfer
///
/// - **DMAEN** `[11]` (RW)
///
///   DMA requests enable
///
/// - **ITBUFEN** `[10]` (RW)
///
///   Buffer interrupt enable
///
/// - **ITEVTEN** `[9]` (RW)
///
///   Event interrupt enable
///
/// - **ITERREN** `[8]` (RW)
///
///   Error interrupt enable
///
/// - **FREQ** `[5:0]` (RW)
///
///   Peripheral clock frequency
#[repr(transparent)]
pub struct Ctlr2 {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Ctlr2 {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctlr2 {
    // ======== Field Constants ========

    /// **LAST**
    ///
    /// DMA last transfer
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LAST_OFFSET: u32 = 12;
    pub const LAST_MASK: u32 = 0x00001000;
    pub const LAST: u32 = 1 << 12;

    /// **DMAEN**
    ///
    /// DMA requests enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DMAEN_OFFSET: u32 = 11;
    pub const DMAEN_MASK: u32 = 0x00000800;
    pub const DMAEN: u32 = 1 << 11;

    /// **ITBUFEN**
    ///
    /// Buffer interrupt enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ITBUFEN_OFFSET: u32 = 10;
    pub const ITBUFEN_MASK: u32 = 0x00000400;
    pub const ITBUFEN: u32 = 1 << 10;

    /// **ITEVTEN**
    ///
    /// Event interrupt enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ITEVTEN_OFFSET: u32 = 9;
    pub const ITEVTEN_MASK: u32 = 0x00000200;
    pub const ITEVTEN: u32 = 1 << 9;

    /// **ITERREN**
    ///
    /// Error interrupt enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ITERREN_OFFSET: u32 = 8;
    pub const ITERREN_MASK: u32 = 0x00000100;
    pub const ITERREN: u32 = 1 << 8;

    /// **FREQ**
    ///
    /// Peripheral clock frequency
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const FREQ_OFFSET: u32 = 0;
    pub const FREQ_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **LAST** `[12]` (RW)
    ///
    /// DMA last transfer
    #[inline(always)]
    pub fn last(&self) -> bool {
        (self.value.read_bits() & Self::LAST_MASK) != 0
    }

    /// Set LAST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_last(&self) {
        self.value.modify_bits(|r| r | Self::LAST_MASK);
    }

    /// Clear LAST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_last(&self) {
        self.value.modify_bits(|r| r & !Self::LAST_MASK);
    }

    /// **DMAEN** `[11]` (RW)
    ///
    /// DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> bool {
        (self.value.read_bits() & Self::DMAEN_MASK) != 0
    }

    /// Set DMAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmaen(&self) {
        self.value.modify_bits(|r| r | Self::DMAEN_MASK);
    }

    /// Clear DMAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmaen(&self) {
        self.value.modify_bits(|r| r & !Self::DMAEN_MASK);
    }

    /// **ITBUFEN** `[10]` (RW)
    ///
    /// Buffer interrupt enable
    #[inline(always)]
    pub fn itbufen(&self) -> bool {
        (self.value.read_bits() & Self::ITBUFEN_MASK) != 0
    }

    /// Set ITBUFEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_itbufen(&self) {
        self.value.modify_bits(|r| r | Self::ITBUFEN_MASK);
    }

    /// Clear ITBUFEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_itbufen(&self) {
        self.value.modify_bits(|r| r & !Self::ITBUFEN_MASK);
    }

    /// **ITEVTEN** `[9]` (RW)
    ///
    /// Event interrupt enable
    #[inline(always)]
    pub fn itevten(&self) -> bool {
        (self.value.read_bits() & Self::ITEVTEN_MASK) != 0
    }

    /// Set ITEVTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_itevten(&self) {
        self.value.modify_bits(|r| r | Self::ITEVTEN_MASK);
    }

    /// Clear ITEVTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_itevten(&self) {
        self.value.modify_bits(|r| r & !Self::ITEVTEN_MASK);
    }

    /// **ITERREN** `[8]` (RW)
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub fn iterren(&self) -> bool {
        (self.value.read_bits() & Self::ITERREN_MASK) != 0
    }

    /// Set ITERREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iterren(&self) {
        self.value.modify_bits(|r| r | Self::ITERREN_MASK);
    }

    /// Clear ITERREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iterren(&self) {
        self.value.modify_bits(|r| r & !Self::ITERREN_MASK);
    }

    /// **FREQ** `[5:0]` (RW)
    ///
    /// Peripheral clock frequency
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        (self.value.read_bits() & Self::FREQ_MASK) >> Self::FREQ_OFFSET
    }

    /// Write **FREQ** `[5:0]` (RW)
    ///
    /// Peripheral clock frequency
    #[inline(always)]
    pub fn write_freq(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FREQ_MASK) | ((value << Self::FREQ_OFFSET) & Self::FREQ_MASK)
        });
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

    /// Read **LAST** `[12:12]`
    ///
    /// DMA last transfer
    #[inline(always)]
    pub const fn last(&self) -> bool {
        (self.bits & Ctlr2::LAST_MASK) != 0
    }

    /// Read **DMAEN** `[11:11]`
    ///
    /// DMA requests enable
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        (self.bits & Ctlr2::DMAEN_MASK) != 0
    }

    /// Read **ITBUFEN** `[10:10]`
    ///
    /// Buffer interrupt enable
    #[inline(always)]
    pub const fn itbufen(&self) -> bool {
        (self.bits & Ctlr2::ITBUFEN_MASK) != 0
    }

    /// Read **ITEVTEN** `[9:9]`
    ///
    /// Event interrupt enable
    #[inline(always)]
    pub const fn itevten(&self) -> bool {
        (self.bits & Ctlr2::ITEVTEN_MASK) != 0
    }

    /// Read **ITERREN** `[8:8]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn iterren(&self) -> bool {
        (self.bits & Ctlr2::ITERREN_MASK) != 0
    }

    /// Read **FREQ** `[5:0]`
    ///
    /// Peripheral clock frequency
    #[inline(always)]
    pub const fn freq(&self) -> u32 {
        (self.bits & Ctlr2::FREQ_MASK) >> Ctlr2::FREQ_OFFSET
    }

    /// Set **LAST** `[12:12]`
    ///
    /// DMA last transfer
    #[inline(always)]
    pub const fn with_last(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::LAST_MASK
            } else {
                self.bits & !Ctlr2::LAST_MASK
            },
        }
    }

    /// Set **DMAEN** `[11:11]`
    ///
    /// DMA requests enable
    #[inline(always)]
    pub const fn with_dmaen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::DMAEN_MASK
            } else {
                self.bits & !Ctlr2::DMAEN_MASK
            },
        }
    }

    /// Set **ITBUFEN** `[10:10]`
    ///
    /// Buffer interrupt enable
    #[inline(always)]
    pub const fn with_itbufen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::ITBUFEN_MASK
            } else {
                self.bits & !Ctlr2::ITBUFEN_MASK
            },
        }
    }

    /// Set **ITEVTEN** `[9:9]`
    ///
    /// Event interrupt enable
    #[inline(always)]
    pub const fn with_itevten(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::ITEVTEN_MASK
            } else {
                self.bits & !Ctlr2::ITEVTEN_MASK
            },
        }
    }

    /// Set **ITERREN** `[8:8]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn with_iterren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::ITERREN_MASK
            } else {
                self.bits & !Ctlr2::ITERREN_MASK
            },
        }
    }

    /// Set **FREQ** `[5:0]`
    ///
    /// Peripheral clock frequency
    #[inline(always)]
    pub const fn with_freq(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr2::FREQ_MASK)
                | ((value << Ctlr2::FREQ_OFFSET) & Ctlr2::FREQ_MASK),
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

/// OADDR1 Register
///
/// Own address register 1
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDMODE** `[15]` (RW)
///
///   Addressing mode (slave
///   mode)
///
/// - **ADD9_8** `[9:8]` (RW)
///
///   Interface address
///
/// - **ADD7_1** `[7:1]` (RW)
///
///   Interface address
///
/// - **ADD0** `[0]` (RW)
///
///   Interface address
#[repr(transparent)]
pub struct Oaddr1 {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Oaddr1 {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Oaddr1 {
    // ======== Field Constants ========

    /// **ADDMODE**
    ///
    /// Addressing mode (slave
    /// mode)
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ADDMODE_OFFSET: u32 = 15;
    pub const ADDMODE_MASK: u32 = 0x00008000;
    pub const ADDMODE: u32 = 1 << 15;

    /// **ADD9_8**
    ///
    /// Interface address
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const ADD9_8_OFFSET: u32 = 8;
    pub const ADD9_8_MASK: u32 = 0x00000300;

    /// **ADD7_1**
    ///
    /// Interface address
    ///
    /// Bits `[7:1]`, mask `0x000000fe`
    pub const ADD7_1_OFFSET: u32 = 1;
    pub const ADD7_1_MASK: u32 = 0x000000fe;

    /// **ADD0**
    ///
    /// Interface address
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADD0_OFFSET: u32 = 0;
    pub const ADD0_MASK: u32 = 0x00000001;
    pub const ADD0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ADDMODE** `[15]` (RW)
    ///
    /// Addressing mode (slave
    /// mode)
    #[inline(always)]
    pub fn addmode(&self) -> bool {
        (self.value.read_bits() & Self::ADDMODE_MASK) != 0
    }

    /// Set ADDMODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_addmode(&self) {
        self.value.modify_bits(|r| r | Self::ADDMODE_MASK);
    }

    /// Clear ADDMODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_addmode(&self) {
        self.value.modify_bits(|r| r & !Self::ADDMODE_MASK);
    }

    /// **ADD9_8** `[9:8]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn add9_8(&self) -> u32 {
        (self.value.read_bits() & Self::ADD9_8_MASK) >> Self::ADD9_8_OFFSET
    }

    /// Write **ADD9_8** `[9:8]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn write_add9_8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADD9_8_MASK) | ((value << Self::ADD9_8_OFFSET) & Self::ADD9_8_MASK)
        });
    }

    /// **ADD7_1** `[7:1]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn add7_1(&self) -> u32 {
        (self.value.read_bits() & Self::ADD7_1_MASK) >> Self::ADD7_1_OFFSET
    }

    /// Write **ADD7_1** `[7:1]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn write_add7_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADD7_1_MASK) | ((value << Self::ADD7_1_OFFSET) & Self::ADD7_1_MASK)
        });
    }

    /// **ADD0** `[0]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn add0(&self) -> bool {
        (self.value.read_bits() & Self::ADD0_MASK) != 0
    }

    /// Set ADD0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_add0(&self) {
        self.value.modify_bits(|r| r | Self::ADD0_MASK);
    }

    /// Clear ADD0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_add0(&self) {
        self.value.modify_bits(|r| r & !Self::ADD0_MASK);
    }
}

/// OADDR1 value type for batch read/modify/write operations
///
/// Use `oaddr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `oaddr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Oaddr1Val {
    bits: u32,
}

impl Oaddr1Val {
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

    /// Read **ADDMODE** `[15:15]`
    ///
    /// Addressing mode (slave
    /// mode)
    #[inline(always)]
    pub const fn addmode(&self) -> bool {
        (self.bits & Oaddr1::ADDMODE_MASK) != 0
    }

    /// Read **ADD9_8** `[9:8]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn add9_8(&self) -> u32 {
        (self.bits & Oaddr1::ADD9_8_MASK) >> Oaddr1::ADD9_8_OFFSET
    }

    /// Read **ADD7_1** `[7:1]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn add7_1(&self) -> u32 {
        (self.bits & Oaddr1::ADD7_1_MASK) >> Oaddr1::ADD7_1_OFFSET
    }

    /// Read **ADD0** `[0:0]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn add0(&self) -> bool {
        (self.bits & Oaddr1::ADD0_MASK) != 0
    }

    /// Set **ADDMODE** `[15:15]`
    ///
    /// Addressing mode (slave
    /// mode)
    #[inline(always)]
    pub const fn with_addmode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Oaddr1::ADDMODE_MASK
            } else {
                self.bits & !Oaddr1::ADDMODE_MASK
            },
        }
    }

    /// Set **ADD9_8** `[9:8]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn with_add9_8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Oaddr1::ADD9_8_MASK)
                | ((value << Oaddr1::ADD9_8_OFFSET) & Oaddr1::ADD9_8_MASK),
        }
    }

    /// Set **ADD7_1** `[7:1]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn with_add7_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Oaddr1::ADD7_1_MASK)
                | ((value << Oaddr1::ADD7_1_OFFSET) & Oaddr1::ADD7_1_MASK),
        }
    }

    /// Set **ADD0** `[0:0]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn with_add0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Oaddr1::ADD0_MASK
            } else {
                self.bits & !Oaddr1::ADD0_MASK
            },
        }
    }
}

impl Default for Oaddr1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Oaddr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Oaddr1Val {
        Oaddr1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Oaddr1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Oaddr1Val) -> Oaddr1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OADDR2 Register
///
/// Own address register 2
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADD2** `[7:1]` (RW)
///
///   Interface address
///
/// - **ENDUAL** `[0]` (RW)
///
///   Dual addressing mode
///   enable
#[repr(transparent)]
pub struct Oaddr2 {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Oaddr2 {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Oaddr2 {
    // ======== Field Constants ========

    /// **ADD2**
    ///
    /// Interface address
    ///
    /// Bits `[7:1]`, mask `0x000000fe`
    pub const ADD2_OFFSET: u32 = 1;
    pub const ADD2_MASK: u32 = 0x000000fe;

    /// **ENDUAL**
    ///
    /// Dual addressing mode
    /// enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENDUAL_OFFSET: u32 = 0;
    pub const ENDUAL_MASK: u32 = 0x00000001;
    pub const ENDUAL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ADD2** `[7:1]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn add2(&self) -> u32 {
        (self.value.read_bits() & Self::ADD2_MASK) >> Self::ADD2_OFFSET
    }

    /// Write **ADD2** `[7:1]` (RW)
    ///
    /// Interface address
    #[inline(always)]
    pub fn write_add2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADD2_MASK) | ((value << Self::ADD2_OFFSET) & Self::ADD2_MASK)
        });
    }

    /// **ENDUAL** `[0]` (RW)
    ///
    /// Dual addressing mode
    /// enable
    #[inline(always)]
    pub fn endual(&self) -> bool {
        (self.value.read_bits() & Self::ENDUAL_MASK) != 0
    }

    /// Set ENDUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_endual(&self) {
        self.value.modify_bits(|r| r | Self::ENDUAL_MASK);
    }

    /// Clear ENDUAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_endual(&self) {
        self.value.modify_bits(|r| r & !Self::ENDUAL_MASK);
    }
}

/// OADDR2 value type for batch read/modify/write operations
///
/// Use `oaddr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `oaddr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Oaddr2Val {
    bits: u32,
}

impl Oaddr2Val {
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

    /// Read **ADD2** `[7:1]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn add2(&self) -> u32 {
        (self.bits & Oaddr2::ADD2_MASK) >> Oaddr2::ADD2_OFFSET
    }

    /// Read **ENDUAL** `[0:0]`
    ///
    /// Dual addressing mode
    /// enable
    #[inline(always)]
    pub const fn endual(&self) -> bool {
        (self.bits & Oaddr2::ENDUAL_MASK) != 0
    }

    /// Set **ADD2** `[7:1]`
    ///
    /// Interface address
    #[inline(always)]
    pub const fn with_add2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Oaddr2::ADD2_MASK)
                | ((value << Oaddr2::ADD2_OFFSET) & Oaddr2::ADD2_MASK),
        }
    }

    /// Set **ENDUAL** `[0:0]`
    ///
    /// Dual addressing mode
    /// enable
    #[inline(always)]
    pub const fn with_endual(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Oaddr2::ENDUAL_MASK
            } else {
                self.bits & !Oaddr2::ENDUAL_MASK
            },
        }
    }
}

impl Default for Oaddr2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Oaddr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Oaddr2Val {
        Oaddr2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Oaddr2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Oaddr2Val) -> Oaddr2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATAR Register
///
/// Data register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DATAR** `[7:0]` (RW)
///
///   8-bit data register
#[repr(transparent)]
pub struct Datar {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Datar {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Datar {
    // ======== Field Constants ========

    /// **DATAR**
    ///
    /// 8-bit data register
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATAR_OFFSET: u32 = 0;
    pub const DATAR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATAR** `[7:0]` (RW)
    ///
    /// 8-bit data register
    #[inline(always)]
    pub fn datar(&self) -> u32 {
        (self.value.read_bits() & Self::DATAR_MASK) >> Self::DATAR_OFFSET
    }

    /// Write **DATAR** `[7:0]` (RW)
    ///
    /// 8-bit data register
    #[inline(always)]
    pub fn write_datar(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATAR_MASK) | ((value << Self::DATAR_OFFSET) & Self::DATAR_MASK)
        });
    }
}

/// DATAR value type for batch read/modify/write operations
///
/// Use `datar.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `datar.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DatarVal {
    bits: u32,
}

impl DatarVal {
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

    /// Read **DATAR** `[7:0]`
    ///
    /// 8-bit data register
    #[inline(always)]
    pub const fn datar(&self) -> u32 {
        (self.bits & Datar::DATAR_MASK) >> Datar::DATAR_OFFSET
    }

    /// Set **DATAR** `[7:0]`
    ///
    /// 8-bit data register
    #[inline(always)]
    pub const fn with_datar(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Datar::DATAR_MASK)
                | ((value << Datar::DATAR_OFFSET) & Datar::DATAR_MASK),
        }
    }
}

impl Default for DatarVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Datar {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DatarVal {
        DatarVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DatarVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DatarVal) -> DatarVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STAR1 Register
///
/// Status register 1
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PECERR** `[12]` (RW)
///
///   PEC Error in reception
///
/// - **OVR** `[11]` (RW)
///
///   Overrun/Underrun
///
/// - **AF** `[10]` (RW)
///
///   Acknowledge failure
///
/// - **ARLO** `[9]` (RW)
///
///   Arbitration lost (master
///   mode)
///
/// - **BERR** `[8]` (RW)
///
///   Bus error
///
/// - **TxE** `[7]` (RO)
///
///   Data register empty
///   (transmitters)
///
/// - **RxNE** `[6]` (RO)
///
///   Data register not empty
///   (receivers)
///
/// - **STOPF** `[4]` (RO)
///
///   Stop detection (slave
///   mode)
///
/// - **ADD10** `[3]` (RO)
///
///   10-bit header sent (Master
///   mode)
///
/// - **BTF** `[2]` (RO)
///
///   Byte transfer finished
///
/// - **ADDR** `[1]` (RW)
///
///   Address sent (master mode)/matched
///   (slave mode)
///
/// - **SB** `[0]` (RO)
///
///   Start bit (Master mode)
#[repr(transparent)]
pub struct Star1 {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Star1 {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Star1 {
    // ======== Field Constants ========

    /// **PECERR**
    ///
    /// PEC Error in reception
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PECERR_OFFSET: u32 = 12;
    pub const PECERR_MASK: u32 = 0x00001000;
    pub const PECERR: u32 = 1 << 12;

    /// **OVR**
    ///
    /// Overrun/Underrun
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const OVR_OFFSET: u32 = 11;
    pub const OVR_MASK: u32 = 0x00000800;
    pub const OVR: u32 = 1 << 11;

    /// **AF**
    ///
    /// Acknowledge failure
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const AF_OFFSET: u32 = 10;
    pub const AF_MASK: u32 = 0x00000400;
    pub const AF: u32 = 1 << 10;

    /// **ARLO**
    ///
    /// Arbitration lost (master
    /// mode)
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ARLO_OFFSET: u32 = 9;
    pub const ARLO_MASK: u32 = 0x00000200;
    pub const ARLO: u32 = 1 << 9;

    /// **BERR**
    ///
    /// Bus error
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const BERR_OFFSET: u32 = 8;
    pub const BERR_MASK: u32 = 0x00000100;
    pub const BERR: u32 = 1 << 8;

    /// **TxE**
    ///
    /// Data register empty
    /// (transmitters)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_E_OFFSET: u32 = 7;
    pub const TX_E_MASK: u32 = 0x00000080;
    pub const TX_E: u32 = 1 << 7;

    /// **RxNE**
    ///
    /// Data register not empty
    /// (receivers)
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RX_NE_OFFSET: u32 = 6;
    pub const RX_NE_MASK: u32 = 0x00000040;
    pub const RX_NE: u32 = 1 << 6;

    /// **STOPF**
    ///
    /// Stop detection (slave
    /// mode)
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const STOPF_OFFSET: u32 = 4;
    pub const STOPF_MASK: u32 = 0x00000010;
    pub const STOPF: u32 = 1 << 4;

    /// **ADD10**
    ///
    /// 10-bit header sent (Master
    /// mode)
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ADD10_OFFSET: u32 = 3;
    pub const ADD10_MASK: u32 = 0x00000008;
    pub const ADD10: u32 = 1 << 3;

    /// **BTF**
    ///
    /// Byte transfer finished
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BTF_OFFSET: u32 = 2;
    pub const BTF_MASK: u32 = 0x00000004;
    pub const BTF: u32 = 1 << 2;

    /// **ADDR**
    ///
    /// Address sent (master mode)/matched
    /// (slave mode)
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ADDR_OFFSET: u32 = 1;
    pub const ADDR_MASK: u32 = 0x00000002;
    pub const ADDR: u32 = 1 << 1;

    /// **SB**
    ///
    /// Start bit (Master mode)
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SB_OFFSET: u32 = 0;
    pub const SB_MASK: u32 = 0x00000001;
    pub const SB: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PECERR** `[12]` (RW)
    ///
    /// PEC Error in reception
    #[inline(always)]
    pub fn pecerr(&self) -> bool {
        (self.value.read_bits() & Self::PECERR_MASK) != 0
    }

    /// Set PECERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pecerr(&self) {
        self.value.modify_bits(|r| r | Self::PECERR_MASK);
    }

    /// Clear PECERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pecerr(&self) {
        self.value.modify_bits(|r| r & !Self::PECERR_MASK);
    }

    /// **OVR** `[11]` (RW)
    ///
    /// Overrun/Underrun
    #[inline(always)]
    pub fn ovr(&self) -> bool {
        (self.value.read_bits() & Self::OVR_MASK) != 0
    }

    /// Set OVR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ovr(&self) {
        self.value.modify_bits(|r| r | Self::OVR_MASK);
    }

    /// Clear OVR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ovr(&self) {
        self.value.modify_bits(|r| r & !Self::OVR_MASK);
    }

    /// **AF** `[10]` (RW)
    ///
    /// Acknowledge failure
    #[inline(always)]
    pub fn af(&self) -> bool {
        (self.value.read_bits() & Self::AF_MASK) != 0
    }

    /// Set AF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_af(&self) {
        self.value.modify_bits(|r| r | Self::AF_MASK);
    }

    /// Clear AF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_af(&self) {
        self.value.modify_bits(|r| r & !Self::AF_MASK);
    }

    /// **ARLO** `[9]` (RW)
    ///
    /// Arbitration lost (master
    /// mode)
    #[inline(always)]
    pub fn arlo(&self) -> bool {
        (self.value.read_bits() & Self::ARLO_MASK) != 0
    }

    /// Set ARLO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arlo(&self) {
        self.value.modify_bits(|r| r | Self::ARLO_MASK);
    }

    /// Clear ARLO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arlo(&self) {
        self.value.modify_bits(|r| r & !Self::ARLO_MASK);
    }

    /// **BERR** `[8]` (RW)
    ///
    /// Bus error
    #[inline(always)]
    pub fn berr(&self) -> bool {
        (self.value.read_bits() & Self::BERR_MASK) != 0
    }

    /// Set BERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_berr(&self) {
        self.value.modify_bits(|r| r | Self::BERR_MASK);
    }

    /// Clear BERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_berr(&self) {
        self.value.modify_bits(|r| r & !Self::BERR_MASK);
    }

    /// **TxE** `[7]` (RO)
    ///
    /// Data register empty
    /// (transmitters)
    #[inline(always)]
    pub fn tx_e(&self) -> bool {
        (self.value.read_bits() & Self::TX_E_MASK) != 0
    }

    /// **RxNE** `[6]` (RO)
    ///
    /// Data register not empty
    /// (receivers)
    #[inline(always)]
    pub fn rx_ne(&self) -> bool {
        (self.value.read_bits() & Self::RX_NE_MASK) != 0
    }

    /// **STOPF** `[4]` (RO)
    ///
    /// Stop detection (slave
    /// mode)
    #[inline(always)]
    pub fn stopf(&self) -> bool {
        (self.value.read_bits() & Self::STOPF_MASK) != 0
    }

    /// **ADD10** `[3]` (RO)
    ///
    /// 10-bit header sent (Master
    /// mode)
    #[inline(always)]
    pub fn add10(&self) -> bool {
        (self.value.read_bits() & Self::ADD10_MASK) != 0
    }

    /// **BTF** `[2]` (RO)
    ///
    /// Byte transfer finished
    #[inline(always)]
    pub fn btf(&self) -> bool {
        (self.value.read_bits() & Self::BTF_MASK) != 0
    }

    /// **ADDR** `[1]` (RW)
    ///
    /// Address sent (master mode)/matched
    /// (slave mode)
    #[inline(always)]
    pub fn addr(&self) -> bool {
        (self.value.read_bits() & Self::ADDR_MASK) != 0
    }

    /// Set ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_addr(&self) {
        self.value.modify_bits(|r| r | Self::ADDR_MASK);
    }

    /// Clear ADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_addr(&self) {
        self.value.modify_bits(|r| r & !Self::ADDR_MASK);
    }

    /// **SB** `[0]` (RO)
    ///
    /// Start bit (Master mode)
    #[inline(always)]
    pub fn sb(&self) -> bool {
        (self.value.read_bits() & Self::SB_MASK) != 0
    }
}

/// STAR1 value type for batch read/modify/write operations
///
/// Use `star1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `star1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Star1Val {
    bits: u32,
}

impl Star1Val {
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

    /// Read **PECERR** `[12:12]`
    ///
    /// PEC Error in reception
    #[inline(always)]
    pub const fn pecerr(&self) -> bool {
        (self.bits & Star1::PECERR_MASK) != 0
    }

    /// Read **OVR** `[11:11]`
    ///
    /// Overrun/Underrun
    #[inline(always)]
    pub const fn ovr(&self) -> bool {
        (self.bits & Star1::OVR_MASK) != 0
    }

    /// Read **AF** `[10:10]`
    ///
    /// Acknowledge failure
    #[inline(always)]
    pub const fn af(&self) -> bool {
        (self.bits & Star1::AF_MASK) != 0
    }

    /// Read **ARLO** `[9:9]`
    ///
    /// Arbitration lost (master
    /// mode)
    #[inline(always)]
    pub const fn arlo(&self) -> bool {
        (self.bits & Star1::ARLO_MASK) != 0
    }

    /// Read **BERR** `[8:8]`
    ///
    /// Bus error
    #[inline(always)]
    pub const fn berr(&self) -> bool {
        (self.bits & Star1::BERR_MASK) != 0
    }

    /// Read **TxE** `[7:7]`
    ///
    /// Data register empty
    /// (transmitters)
    #[inline(always)]
    pub const fn tx_e(&self) -> bool {
        (self.bits & Star1::TX_E_MASK) != 0
    }

    /// Read **RxNE** `[6:6]`
    ///
    /// Data register not empty
    /// (receivers)
    #[inline(always)]
    pub const fn rx_ne(&self) -> bool {
        (self.bits & Star1::RX_NE_MASK) != 0
    }

    /// Read **STOPF** `[4:4]`
    ///
    /// Stop detection (slave
    /// mode)
    #[inline(always)]
    pub const fn stopf(&self) -> bool {
        (self.bits & Star1::STOPF_MASK) != 0
    }

    /// Read **ADD10** `[3:3]`
    ///
    /// 10-bit header sent (Master
    /// mode)
    #[inline(always)]
    pub const fn add10(&self) -> bool {
        (self.bits & Star1::ADD10_MASK) != 0
    }

    /// Read **BTF** `[2:2]`
    ///
    /// Byte transfer finished
    #[inline(always)]
    pub const fn btf(&self) -> bool {
        (self.bits & Star1::BTF_MASK) != 0
    }

    /// Read **ADDR** `[1:1]`
    ///
    /// Address sent (master mode)/matched
    /// (slave mode)
    #[inline(always)]
    pub const fn addr(&self) -> bool {
        (self.bits & Star1::ADDR_MASK) != 0
    }

    /// Read **SB** `[0:0]`
    ///
    /// Start bit (Master mode)
    #[inline(always)]
    pub const fn sb(&self) -> bool {
        (self.bits & Star1::SB_MASK) != 0
    }

    /// Set **PECERR** `[12:12]`
    ///
    /// PEC Error in reception
    #[inline(always)]
    pub const fn with_pecerr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Star1::PECERR_MASK
            } else {
                self.bits & !Star1::PECERR_MASK
            },
        }
    }

    /// Set **OVR** `[11:11]`
    ///
    /// Overrun/Underrun
    #[inline(always)]
    pub const fn with_ovr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Star1::OVR_MASK
            } else {
                self.bits & !Star1::OVR_MASK
            },
        }
    }

    /// Set **AF** `[10:10]`
    ///
    /// Acknowledge failure
    #[inline(always)]
    pub const fn with_af(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Star1::AF_MASK
            } else {
                self.bits & !Star1::AF_MASK
            },
        }
    }

    /// Set **ARLO** `[9:9]`
    ///
    /// Arbitration lost (master
    /// mode)
    #[inline(always)]
    pub const fn with_arlo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Star1::ARLO_MASK
            } else {
                self.bits & !Star1::ARLO_MASK
            },
        }
    }

    /// Set **BERR** `[8:8]`
    ///
    /// Bus error
    #[inline(always)]
    pub const fn with_berr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Star1::BERR_MASK
            } else {
                self.bits & !Star1::BERR_MASK
            },
        }
    }

    /// Set **ADDR** `[1:1]`
    ///
    /// Address sent (master mode)/matched
    /// (slave mode)
    #[inline(always)]
    pub const fn with_addr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Star1::ADDR_MASK
            } else {
                self.bits & !Star1::ADDR_MASK
            },
        }
    }
}

impl Default for Star1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Star1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Star1Val {
        Star1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Star1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Star1Val) -> Star1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STAR2 Register
///
/// Status register 2
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PEC** `[15:8]` (RO)
///
///   PEC
///
/// - **DUALF** `[7]` (RO)
///
///   Dual flag (Slave mode)
///
/// - **GENCALL** `[4]` (RO)
///
///   General call address (Slave
///   mode)
///
/// - **TRA** `[2]` (RO)
///
///   Transmitter/receiver
///
/// - **BUSY** `[1]` (RO)
///
///   Bus busy
///
/// - **MSL** `[0]` (RO)
///
///   Master/slave
#[repr(transparent)]
pub struct Star2 {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Star2 {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Star2 {
    // ======== Field Constants ========

    /// **PEC**
    ///
    /// PEC
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const PEC_OFFSET: u32 = 8;
    pub const PEC_MASK: u32 = 0x0000ff00;

    /// **DUALF**
    ///
    /// Dual flag (Slave mode)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DUALF_OFFSET: u32 = 7;
    pub const DUALF_MASK: u32 = 0x00000080;
    pub const DUALF: u32 = 1 << 7;

    /// **GENCALL**
    ///
    /// General call address (Slave
    /// mode)
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const GENCALL_OFFSET: u32 = 4;
    pub const GENCALL_MASK: u32 = 0x00000010;
    pub const GENCALL: u32 = 1 << 4;

    /// **TRA**
    ///
    /// Transmitter/receiver
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TRA_OFFSET: u32 = 2;
    pub const TRA_MASK: u32 = 0x00000004;
    pub const TRA: u32 = 1 << 2;

    /// **BUSY**
    ///
    /// Bus busy
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BUSY_OFFSET: u32 = 1;
    pub const BUSY_MASK: u32 = 0x00000002;
    pub const BUSY: u32 = 1 << 1;

    /// **MSL**
    ///
    /// Master/slave
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MSL_OFFSET: u32 = 0;
    pub const MSL_MASK: u32 = 0x00000001;
    pub const MSL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PEC** `[15:8]` (RO)
    ///
    /// PEC
    #[inline(always)]
    pub fn pec(&self) -> u32 {
        (self.value.read_bits() & Self::PEC_MASK) >> Self::PEC_OFFSET
    }

    /// **DUALF** `[7]` (RO)
    ///
    /// Dual flag (Slave mode)
    #[inline(always)]
    pub fn dualf(&self) -> bool {
        (self.value.read_bits() & Self::DUALF_MASK) != 0
    }

    /// **GENCALL** `[4]` (RO)
    ///
    /// General call address (Slave
    /// mode)
    #[inline(always)]
    pub fn gencall(&self) -> bool {
        (self.value.read_bits() & Self::GENCALL_MASK) != 0
    }

    /// **TRA** `[2]` (RO)
    ///
    /// Transmitter/receiver
    #[inline(always)]
    pub fn tra(&self) -> bool {
        (self.value.read_bits() & Self::TRA_MASK) != 0
    }

    /// **BUSY** `[1]` (RO)
    ///
    /// Bus busy
    #[inline(always)]
    pub fn busy(&self) -> bool {
        (self.value.read_bits() & Self::BUSY_MASK) != 0
    }

    /// **MSL** `[0]` (RO)
    ///
    /// Master/slave
    #[inline(always)]
    pub fn msl(&self) -> bool {
        (self.value.read_bits() & Self::MSL_MASK) != 0
    }
}

/// STAR2 value type for batch read/modify/write operations
///
/// Use `star2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `star2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Star2Val {
    bits: u32,
}

impl Star2Val {
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

    /// Read **PEC** `[15:8]`
    ///
    /// PEC
    #[inline(always)]
    pub const fn pec(&self) -> u32 {
        (self.bits & Star2::PEC_MASK) >> Star2::PEC_OFFSET
    }

    /// Read **DUALF** `[7:7]`
    ///
    /// Dual flag (Slave mode)
    #[inline(always)]
    pub const fn dualf(&self) -> bool {
        (self.bits & Star2::DUALF_MASK) != 0
    }

    /// Read **GENCALL** `[4:4]`
    ///
    /// General call address (Slave
    /// mode)
    #[inline(always)]
    pub const fn gencall(&self) -> bool {
        (self.bits & Star2::GENCALL_MASK) != 0
    }

    /// Read **TRA** `[2:2]`
    ///
    /// Transmitter/receiver
    #[inline(always)]
    pub const fn tra(&self) -> bool {
        (self.bits & Star2::TRA_MASK) != 0
    }

    /// Read **BUSY** `[1:1]`
    ///
    /// Bus busy
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        (self.bits & Star2::BUSY_MASK) != 0
    }

    /// Read **MSL** `[0:0]`
    ///
    /// Master/slave
    #[inline(always)]
    pub const fn msl(&self) -> bool {
        (self.bits & Star2::MSL_MASK) != 0
    }
}

impl Default for Star2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Star2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Star2Val {
        Star2Val::from_bits(self.value.read_bits())
    }
}

/// CKCFGR Register
///
/// Clock control register
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **F_S** `[15]` (RW)
///
///   I2C master mode selection
///
/// - **DUTY** `[14]` (RW)
///
///   Fast mode duty cycle
///
/// - **CCR** `[11:0]` (RW)
///
///   Clock control register in Fast/Standard
///   mode (Master mode)
#[repr(transparent)]
pub struct Ckcfgr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Ckcfgr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ckcfgr {
    // ======== Field Constants ========

    /// **F_S**
    ///
    /// I2C master mode selection
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const F_S_OFFSET: u32 = 15;
    pub const F_S_MASK: u32 = 0x00008000;
    pub const F_S: u32 = 1 << 15;

    /// **DUTY**
    ///
    /// Fast mode duty cycle
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DUTY_OFFSET: u32 = 14;
    pub const DUTY_MASK: u32 = 0x00004000;
    pub const DUTY: u32 = 1 << 14;

    /// **CCR**
    ///
    /// Clock control register in Fast/Standard
    /// mode (Master mode)
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const CCR_OFFSET: u32 = 0;
    pub const CCR_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **F_S** `[15]` (RW)
    ///
    /// I2C master mode selection
    #[inline(always)]
    pub fn f_s(&self) -> bool {
        (self.value.read_bits() & Self::F_S_MASK) != 0
    }

    /// Set F_S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_f_s(&self) {
        self.value.modify_bits(|r| r | Self::F_S_MASK);
    }

    /// Clear F_S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_f_s(&self) {
        self.value.modify_bits(|r| r & !Self::F_S_MASK);
    }

    /// **DUTY** `[14]` (RW)
    ///
    /// Fast mode duty cycle
    #[inline(always)]
    pub fn duty(&self) -> bool {
        (self.value.read_bits() & Self::DUTY_MASK) != 0
    }

    /// Set DUTY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_duty(&self) {
        self.value.modify_bits(|r| r | Self::DUTY_MASK);
    }

    /// Clear DUTY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_duty(&self) {
        self.value.modify_bits(|r| r & !Self::DUTY_MASK);
    }

    /// **CCR** `[11:0]` (RW)
    ///
    /// Clock control register in Fast/Standard
    /// mode (Master mode)
    #[inline(always)]
    pub fn ccr(&self) -> u32 {
        (self.value.read_bits() & Self::CCR_MASK) >> Self::CCR_OFFSET
    }

    /// Write **CCR** `[11:0]` (RW)
    ///
    /// Clock control register in Fast/Standard
    /// mode (Master mode)
    #[inline(always)]
    pub fn write_ccr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CCR_MASK) | ((value << Self::CCR_OFFSET) & Self::CCR_MASK)
        });
    }
}

/// CKCFGR value type for batch read/modify/write operations
///
/// Use `ckcfgr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ckcfgr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CkcfgrVal {
    bits: u32,
}

impl CkcfgrVal {
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

    /// Read **F_S** `[15:15]`
    ///
    /// I2C master mode selection
    #[inline(always)]
    pub const fn f_s(&self) -> bool {
        (self.bits & Ckcfgr::F_S_MASK) != 0
    }

    /// Read **DUTY** `[14:14]`
    ///
    /// Fast mode duty cycle
    #[inline(always)]
    pub const fn duty(&self) -> bool {
        (self.bits & Ckcfgr::DUTY_MASK) != 0
    }

    /// Read **CCR** `[11:0]`
    ///
    /// Clock control register in Fast/Standard
    /// mode (Master mode)
    #[inline(always)]
    pub const fn ccr(&self) -> u32 {
        (self.bits & Ckcfgr::CCR_MASK) >> Ckcfgr::CCR_OFFSET
    }

    /// Set **F_S** `[15:15]`
    ///
    /// I2C master mode selection
    #[inline(always)]
    pub const fn with_f_s(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ckcfgr::F_S_MASK
            } else {
                self.bits & !Ckcfgr::F_S_MASK
            },
        }
    }

    /// Set **DUTY** `[14:14]`
    ///
    /// Fast mode duty cycle
    #[inline(always)]
    pub const fn with_duty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ckcfgr::DUTY_MASK
            } else {
                self.bits & !Ckcfgr::DUTY_MASK
            },
        }
    }

    /// Set **CCR** `[11:0]`
    ///
    /// Clock control register in Fast/Standard
    /// mode (Master mode)
    #[inline(always)]
    pub const fn with_ccr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ckcfgr::CCR_MASK)
                | ((value << Ckcfgr::CCR_OFFSET) & Ckcfgr::CCR_MASK),
        }
    }
}

impl Default for CkcfgrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ckcfgr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CkcfgrVal {
        CkcfgrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CkcfgrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CkcfgrVal) -> CkcfgrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control register 1
    pub ctlr1: Ctlr1,
    _reserved0: [u8; 2],
    /// 0x0004
    /// Control register 2
    pub ctlr2: Ctlr2,
    _reserved1: [u8; 2],
    /// 0x0008
    /// Own address register 1
    pub oaddr1: Oaddr1,
    _reserved2: [u8; 2],
    /// 0x000c
    /// Own address register 2
    pub oaddr2: Oaddr2,
    _reserved3: [u8; 2],
    /// 0x0010
    /// Data register
    pub datar: Datar,
    _reserved4: [u8; 2],
    /// 0x0014
    /// Status register 1
    pub star1: Star1,
    _reserved5: [u8; 2],
    /// 0x0018
    /// Status register 2
    pub star2: Star2,
    _reserved6: [u8; 2],
    /// 0x001c
    /// Clock control register
    pub ckcfgr: Ckcfgr,
}
