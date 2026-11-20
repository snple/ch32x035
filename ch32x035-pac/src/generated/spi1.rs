//! SPI1
//! Serial peripheral interface

#![allow(dead_code)]

pub const BASE: u32 = 0x40013000;

/// CTLR1 Register
///
/// control register 1
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BIDIMODE** `[15]` (RW)
///
///   Bidirectional data mode
///   enable
///
/// - **BIDIOE** `[14]` (RW)
///
///   Output enable in bidirectional
///   mode
///
/// - **CRCEN** `[13]` (RW)
///
///   Hardware CRC calculation
///   enable
///
/// - **CRCNEXT** `[12]` (RW)
///
///   CRC transfer next
///
/// - **DFF** `[11]` (RW)
///
///   Data frame format
///
/// - **RXONLY** `[10]` (RW)
///
///   Receive only
///
/// - **SSM** `[9]` (RW)
///
///   Software slave management
///
/// - **SSI** `[8]` (RW)
///
///   Internal slave select
///
/// - **LSBFIRST** `[7]` (RW)
///
///   Frame format
///
/// - **SPE** `[6]` (RW)
///
///   SPI enable
///
/// - **BR** `[5:3]` (RW)
///
///   Baud rate control
///
/// - **MSTR** `[2]` (RW)
///
///   Master selection
///
/// - **CPOL** `[1]` (RW)
///
///   Clock polarity
///
/// - **CPHA** `[0]` (RW)
///
///   Clock phase
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

    /// **BIDIMODE**
    ///
    /// Bidirectional data mode
    /// enable
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const BIDIMODE_OFFSET: u32 = 15;
    pub const BIDIMODE_MASK: u32 = 0x00008000;
    pub const BIDIMODE: u32 = 1 << 15;

    /// **BIDIOE**
    ///
    /// Output enable in bidirectional
    /// mode
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const BIDIOE_OFFSET: u32 = 14;
    pub const BIDIOE_MASK: u32 = 0x00004000;
    pub const BIDIOE: u32 = 1 << 14;

    /// **CRCEN**
    ///
    /// Hardware CRC calculation
    /// enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CRCEN_OFFSET: u32 = 13;
    pub const CRCEN_MASK: u32 = 0x00002000;
    pub const CRCEN: u32 = 1 << 13;

    /// **CRCNEXT**
    ///
    /// CRC transfer next
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CRCNEXT_OFFSET: u32 = 12;
    pub const CRCNEXT_MASK: u32 = 0x00001000;
    pub const CRCNEXT: u32 = 1 << 12;

    /// **DFF**
    ///
    /// Data frame format
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DFF_OFFSET: u32 = 11;
    pub const DFF_MASK: u32 = 0x00000800;
    pub const DFF: u32 = 1 << 11;

    /// **RXONLY**
    ///
    /// Receive only
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RXONLY_OFFSET: u32 = 10;
    pub const RXONLY_MASK: u32 = 0x00000400;
    pub const RXONLY: u32 = 1 << 10;

    /// **SSM**
    ///
    /// Software slave management
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SSM_OFFSET: u32 = 9;
    pub const SSM_MASK: u32 = 0x00000200;
    pub const SSM: u32 = 1 << 9;

    /// **SSI**
    ///
    /// Internal slave select
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SSI_OFFSET: u32 = 8;
    pub const SSI_MASK: u32 = 0x00000100;
    pub const SSI: u32 = 1 << 8;

    /// **LSBFIRST**
    ///
    /// Frame format
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LSBFIRST_OFFSET: u32 = 7;
    pub const LSBFIRST_MASK: u32 = 0x00000080;
    pub const LSBFIRST: u32 = 1 << 7;

    /// **SPE**
    ///
    /// SPI enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SPE_OFFSET: u32 = 6;
    pub const SPE_MASK: u32 = 0x00000040;
    pub const SPE: u32 = 1 << 6;

    /// **BR**
    ///
    /// Baud rate control
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const BR_OFFSET: u32 = 3;
    pub const BR_MASK: u32 = 0x00000038;

    /// **MSTR**
    ///
    /// Master selection
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MSTR_OFFSET: u32 = 2;
    pub const MSTR_MASK: u32 = 0x00000004;
    pub const MSTR: u32 = 1 << 2;

    /// **CPOL**
    ///
    /// Clock polarity
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CPOL_OFFSET: u32 = 1;
    pub const CPOL_MASK: u32 = 0x00000002;
    pub const CPOL: u32 = 1 << 1;

    /// **CPHA**
    ///
    /// Clock phase
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPHA_OFFSET: u32 = 0;
    pub const CPHA_MASK: u32 = 0x00000001;
    pub const CPHA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BIDIMODE** `[15]` (RW)
    ///
    /// Bidirectional data mode
    /// enable
    #[inline(always)]
    pub fn bidimode(&self) -> bool {
        (self.value.read_bits() & Self::BIDIMODE_MASK) != 0
    }

    /// Set BIDIMODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bidimode(&self) {
        self.value.modify_bits(|r| r | Self::BIDIMODE_MASK);
    }

    /// Clear BIDIMODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bidimode(&self) {
        self.value.modify_bits(|r| r & !Self::BIDIMODE_MASK);
    }

    /// **BIDIOE** `[14]` (RW)
    ///
    /// Output enable in bidirectional
    /// mode
    #[inline(always)]
    pub fn bidioe(&self) -> bool {
        (self.value.read_bits() & Self::BIDIOE_MASK) != 0
    }

    /// Set BIDIOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bidioe(&self) {
        self.value.modify_bits(|r| r | Self::BIDIOE_MASK);
    }

    /// Clear BIDIOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bidioe(&self) {
        self.value.modify_bits(|r| r & !Self::BIDIOE_MASK);
    }

    /// **CRCEN** `[13]` (RW)
    ///
    /// Hardware CRC calculation
    /// enable
    #[inline(always)]
    pub fn crcen(&self) -> bool {
        (self.value.read_bits() & Self::CRCEN_MASK) != 0
    }

    /// Set CRCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_crcen(&self) {
        self.value.modify_bits(|r| r | Self::CRCEN_MASK);
    }

    /// Clear CRCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_crcen(&self) {
        self.value.modify_bits(|r| r & !Self::CRCEN_MASK);
    }

    /// **CRCNEXT** `[12]` (RW)
    ///
    /// CRC transfer next
    #[inline(always)]
    pub fn crcnext(&self) -> bool {
        (self.value.read_bits() & Self::CRCNEXT_MASK) != 0
    }

    /// Set CRCNEXT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_crcnext(&self) {
        self.value.modify_bits(|r| r | Self::CRCNEXT_MASK);
    }

    /// Clear CRCNEXT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_crcnext(&self) {
        self.value.modify_bits(|r| r & !Self::CRCNEXT_MASK);
    }

    /// **DFF** `[11]` (RW)
    ///
    /// Data frame format
    #[inline(always)]
    pub fn dff(&self) -> bool {
        (self.value.read_bits() & Self::DFF_MASK) != 0
    }

    /// Set DFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dff(&self) {
        self.value.modify_bits(|r| r | Self::DFF_MASK);
    }

    /// Clear DFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dff(&self) {
        self.value.modify_bits(|r| r & !Self::DFF_MASK);
    }

    /// **RXONLY** `[10]` (RW)
    ///
    /// Receive only
    #[inline(always)]
    pub fn rxonly(&self) -> bool {
        (self.value.read_bits() & Self::RXONLY_MASK) != 0
    }

    /// Set RXONLY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxonly(&self) {
        self.value.modify_bits(|r| r | Self::RXONLY_MASK);
    }

    /// Clear RXONLY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxonly(&self) {
        self.value.modify_bits(|r| r & !Self::RXONLY_MASK);
    }

    /// **SSM** `[9]` (RW)
    ///
    /// Software slave management
    #[inline(always)]
    pub fn ssm(&self) -> bool {
        (self.value.read_bits() & Self::SSM_MASK) != 0
    }

    /// Set SSM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ssm(&self) {
        self.value.modify_bits(|r| r | Self::SSM_MASK);
    }

    /// Clear SSM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ssm(&self) {
        self.value.modify_bits(|r| r & !Self::SSM_MASK);
    }

    /// **SSI** `[8]` (RW)
    ///
    /// Internal slave select
    #[inline(always)]
    pub fn ssi(&self) -> bool {
        (self.value.read_bits() & Self::SSI_MASK) != 0
    }

    /// Set SSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ssi(&self) {
        self.value.modify_bits(|r| r | Self::SSI_MASK);
    }

    /// Clear SSI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ssi(&self) {
        self.value.modify_bits(|r| r & !Self::SSI_MASK);
    }

    /// **LSBFIRST** `[7]` (RW)
    ///
    /// Frame format
    #[inline(always)]
    pub fn lsbfirst(&self) -> bool {
        (self.value.read_bits() & Self::LSBFIRST_MASK) != 0
    }

    /// Set LSBFIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lsbfirst(&self) {
        self.value.modify_bits(|r| r | Self::LSBFIRST_MASK);
    }

    /// Clear LSBFIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lsbfirst(&self) {
        self.value.modify_bits(|r| r & !Self::LSBFIRST_MASK);
    }

    /// **SPE** `[6]` (RW)
    ///
    /// SPI enable
    #[inline(always)]
    pub fn spe(&self) -> bool {
        (self.value.read_bits() & Self::SPE_MASK) != 0
    }

    /// Set SPE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spe(&self) {
        self.value.modify_bits(|r| r | Self::SPE_MASK);
    }

    /// Clear SPE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spe(&self) {
        self.value.modify_bits(|r| r & !Self::SPE_MASK);
    }

    /// **BR** `[5:3]` (RW)
    ///
    /// Baud rate control
    #[inline(always)]
    pub fn br(&self) -> u32 {
        (self.value.read_bits() & Self::BR_MASK) >> Self::BR_OFFSET
    }

    /// Write **BR** `[5:3]` (RW)
    ///
    /// Baud rate control
    #[inline(always)]
    pub fn write_br(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::BR_MASK) | ((value << Self::BR_OFFSET) & Self::BR_MASK));
    }

    /// **MSTR** `[2]` (RW)
    ///
    /// Master selection
    #[inline(always)]
    pub fn mstr(&self) -> bool {
        (self.value.read_bits() & Self::MSTR_MASK) != 0
    }

    /// Set MSTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mstr(&self) {
        self.value.modify_bits(|r| r | Self::MSTR_MASK);
    }

    /// Clear MSTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mstr(&self) {
        self.value.modify_bits(|r| r & !Self::MSTR_MASK);
    }

    /// **CPOL** `[1]` (RW)
    ///
    /// Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> bool {
        (self.value.read_bits() & Self::CPOL_MASK) != 0
    }

    /// Set CPOL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpol(&self) {
        self.value.modify_bits(|r| r | Self::CPOL_MASK);
    }

    /// Clear CPOL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpol(&self) {
        self.value.modify_bits(|r| r & !Self::CPOL_MASK);
    }

    /// **CPHA** `[0]` (RW)
    ///
    /// Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> bool {
        (self.value.read_bits() & Self::CPHA_MASK) != 0
    }

    /// Set CPHA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpha(&self) {
        self.value.modify_bits(|r| r | Self::CPHA_MASK);
    }

    /// Clear CPHA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpha(&self) {
        self.value.modify_bits(|r| r & !Self::CPHA_MASK);
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

    /// Read **BIDIMODE** `[15:15]`
    ///
    /// Bidirectional data mode
    /// enable
    #[inline(always)]
    pub const fn bidimode(&self) -> bool {
        (self.bits & Ctlr1::BIDIMODE_MASK) != 0
    }

    /// Read **BIDIOE** `[14:14]`
    ///
    /// Output enable in bidirectional
    /// mode
    #[inline(always)]
    pub const fn bidioe(&self) -> bool {
        (self.bits & Ctlr1::BIDIOE_MASK) != 0
    }

    /// Read **CRCEN** `[13:13]`
    ///
    /// Hardware CRC calculation
    /// enable
    #[inline(always)]
    pub const fn crcen(&self) -> bool {
        (self.bits & Ctlr1::CRCEN_MASK) != 0
    }

    /// Read **CRCNEXT** `[12:12]`
    ///
    /// CRC transfer next
    #[inline(always)]
    pub const fn crcnext(&self) -> bool {
        (self.bits & Ctlr1::CRCNEXT_MASK) != 0
    }

    /// Read **DFF** `[11:11]`
    ///
    /// Data frame format
    #[inline(always)]
    pub const fn dff(&self) -> bool {
        (self.bits & Ctlr1::DFF_MASK) != 0
    }

    /// Read **RXONLY** `[10:10]`
    ///
    /// Receive only
    #[inline(always)]
    pub const fn rxonly(&self) -> bool {
        (self.bits & Ctlr1::RXONLY_MASK) != 0
    }

    /// Read **SSM** `[9:9]`
    ///
    /// Software slave management
    #[inline(always)]
    pub const fn ssm(&self) -> bool {
        (self.bits & Ctlr1::SSM_MASK) != 0
    }

    /// Read **SSI** `[8:8]`
    ///
    /// Internal slave select
    #[inline(always)]
    pub const fn ssi(&self) -> bool {
        (self.bits & Ctlr1::SSI_MASK) != 0
    }

    /// Read **LSBFIRST** `[7:7]`
    ///
    /// Frame format
    #[inline(always)]
    pub const fn lsbfirst(&self) -> bool {
        (self.bits & Ctlr1::LSBFIRST_MASK) != 0
    }

    /// Read **SPE** `[6:6]`
    ///
    /// SPI enable
    #[inline(always)]
    pub const fn spe(&self) -> bool {
        (self.bits & Ctlr1::SPE_MASK) != 0
    }

    /// Read **BR** `[5:3]`
    ///
    /// Baud rate control
    #[inline(always)]
    pub const fn br(&self) -> u32 {
        (self.bits & Ctlr1::BR_MASK) >> Ctlr1::BR_OFFSET
    }

    /// Read **MSTR** `[2:2]`
    ///
    /// Master selection
    #[inline(always)]
    pub const fn mstr(&self) -> bool {
        (self.bits & Ctlr1::MSTR_MASK) != 0
    }

    /// Read **CPOL** `[1:1]`
    ///
    /// Clock polarity
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        (self.bits & Ctlr1::CPOL_MASK) != 0
    }

    /// Read **CPHA** `[0:0]`
    ///
    /// Clock phase
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        (self.bits & Ctlr1::CPHA_MASK) != 0
    }

    /// Set **BIDIMODE** `[15:15]`
    ///
    /// Bidirectional data mode
    /// enable
    #[inline(always)]
    pub const fn with_bidimode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::BIDIMODE_MASK
            } else {
                self.bits & !Ctlr1::BIDIMODE_MASK
            },
        }
    }

    /// Set **BIDIOE** `[14:14]`
    ///
    /// Output enable in bidirectional
    /// mode
    #[inline(always)]
    pub const fn with_bidioe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::BIDIOE_MASK
            } else {
                self.bits & !Ctlr1::BIDIOE_MASK
            },
        }
    }

    /// Set **CRCEN** `[13:13]`
    ///
    /// Hardware CRC calculation
    /// enable
    #[inline(always)]
    pub const fn with_crcen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CRCEN_MASK
            } else {
                self.bits & !Ctlr1::CRCEN_MASK
            },
        }
    }

    /// Set **CRCNEXT** `[12:12]`
    ///
    /// CRC transfer next
    #[inline(always)]
    pub const fn with_crcnext(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CRCNEXT_MASK
            } else {
                self.bits & !Ctlr1::CRCNEXT_MASK
            },
        }
    }

    /// Set **DFF** `[11:11]`
    ///
    /// Data frame format
    #[inline(always)]
    pub const fn with_dff(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::DFF_MASK
            } else {
                self.bits & !Ctlr1::DFF_MASK
            },
        }
    }

    /// Set **RXONLY** `[10:10]`
    ///
    /// Receive only
    #[inline(always)]
    pub const fn with_rxonly(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::RXONLY_MASK
            } else {
                self.bits & !Ctlr1::RXONLY_MASK
            },
        }
    }

    /// Set **SSM** `[9:9]`
    ///
    /// Software slave management
    #[inline(always)]
    pub const fn with_ssm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::SSM_MASK
            } else {
                self.bits & !Ctlr1::SSM_MASK
            },
        }
    }

    /// Set **SSI** `[8:8]`
    ///
    /// Internal slave select
    #[inline(always)]
    pub const fn with_ssi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::SSI_MASK
            } else {
                self.bits & !Ctlr1::SSI_MASK
            },
        }
    }

    /// Set **LSBFIRST** `[7:7]`
    ///
    /// Frame format
    #[inline(always)]
    pub const fn with_lsbfirst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::LSBFIRST_MASK
            } else {
                self.bits & !Ctlr1::LSBFIRST_MASK
            },
        }
    }

    /// Set **SPE** `[6:6]`
    ///
    /// SPI enable
    #[inline(always)]
    pub const fn with_spe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::SPE_MASK
            } else {
                self.bits & !Ctlr1::SPE_MASK
            },
        }
    }

    /// Set **BR** `[5:3]`
    ///
    /// Baud rate control
    #[inline(always)]
    pub const fn with_br(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr1::BR_MASK) | ((value << Ctlr1::BR_OFFSET) & Ctlr1::BR_MASK),
        }
    }

    /// Set **MSTR** `[2:2]`
    ///
    /// Master selection
    #[inline(always)]
    pub const fn with_mstr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::MSTR_MASK
            } else {
                self.bits & !Ctlr1::MSTR_MASK
            },
        }
    }

    /// Set **CPOL** `[1:1]`
    ///
    /// Clock polarity
    #[inline(always)]
    pub const fn with_cpol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CPOL_MASK
            } else {
                self.bits & !Ctlr1::CPOL_MASK
            },
        }
    }

    /// Set **CPHA** `[0:0]`
    ///
    /// Clock phase
    #[inline(always)]
    pub const fn with_cpha(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::CPHA_MASK
            } else {
                self.bits & !Ctlr1::CPHA_MASK
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
/// - **TXEIE** `[7]` (RW)
///
///   Tx buffer empty interrupt
///   enable
///
/// - **RXNEIE** `[6]` (RW)
///
///   RX buffer not empty interrupt
///   enable
///
/// - **ERRIE** `[5]` (RW)
///
///   Error interrupt enable
///
/// - **SSOE** `[2]` (RW)
///
///   SS output enable
///
/// - **TXDMAEN** `[1]` (RW)
///
///   Tx buffer DMA enable
///
/// - **RXDMAEN** `[0]` (RW)
///
///   Rx buffer DMA enable
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

    /// **TXEIE**
    ///
    /// Tx buffer empty interrupt
    /// enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TXEIE_OFFSET: u32 = 7;
    pub const TXEIE_MASK: u32 = 0x00000080;
    pub const TXEIE: u32 = 1 << 7;

    /// **RXNEIE**
    ///
    /// RX buffer not empty interrupt
    /// enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RXNEIE_OFFSET: u32 = 6;
    pub const RXNEIE_MASK: u32 = 0x00000040;
    pub const RXNEIE: u32 = 1 << 6;

    /// **ERRIE**
    ///
    /// Error interrupt enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERRIE_OFFSET: u32 = 5;
    pub const ERRIE_MASK: u32 = 0x00000020;
    pub const ERRIE: u32 = 1 << 5;

    /// **SSOE**
    ///
    /// SS output enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SSOE_OFFSET: u32 = 2;
    pub const SSOE_MASK: u32 = 0x00000004;
    pub const SSOE: u32 = 1 << 2;

    /// **TXDMAEN**
    ///
    /// Tx buffer DMA enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXDMAEN_OFFSET: u32 = 1;
    pub const TXDMAEN_MASK: u32 = 0x00000002;
    pub const TXDMAEN: u32 = 1 << 1;

    /// **RXDMAEN**
    ///
    /// Rx buffer DMA enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXDMAEN_OFFSET: u32 = 0;
    pub const RXDMAEN_MASK: u32 = 0x00000001;
    pub const RXDMAEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXEIE** `[7]` (RW)
    ///
    /// Tx buffer empty interrupt
    /// enable
    #[inline(always)]
    pub fn txeie(&self) -> bool {
        (self.value.read_bits() & Self::TXEIE_MASK) != 0
    }

    /// Set TXEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txeie(&self) {
        self.value.modify_bits(|r| r | Self::TXEIE_MASK);
    }

    /// Clear TXEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txeie(&self) {
        self.value.modify_bits(|r| r & !Self::TXEIE_MASK);
    }

    /// **RXNEIE** `[6]` (RW)
    ///
    /// RX buffer not empty interrupt
    /// enable
    #[inline(always)]
    pub fn rxneie(&self) -> bool {
        (self.value.read_bits() & Self::RXNEIE_MASK) != 0
    }

    /// Set RXNEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxneie(&self) {
        self.value.modify_bits(|r| r | Self::RXNEIE_MASK);
    }

    /// Clear RXNEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxneie(&self) {
        self.value.modify_bits(|r| r & !Self::RXNEIE_MASK);
    }

    /// **ERRIE** `[5]` (RW)
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

    /// **SSOE** `[2]` (RW)
    ///
    /// SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> bool {
        (self.value.read_bits() & Self::SSOE_MASK) != 0
    }

    /// Set SSOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ssoe(&self) {
        self.value.modify_bits(|r| r | Self::SSOE_MASK);
    }

    /// Clear SSOE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ssoe(&self) {
        self.value.modify_bits(|r| r & !Self::SSOE_MASK);
    }

    /// **TXDMAEN** `[1]` (RW)
    ///
    /// Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> bool {
        (self.value.read_bits() & Self::TXDMAEN_MASK) != 0
    }

    /// Set TXDMAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txdmaen(&self) {
        self.value.modify_bits(|r| r | Self::TXDMAEN_MASK);
    }

    /// Clear TXDMAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txdmaen(&self) {
        self.value.modify_bits(|r| r & !Self::TXDMAEN_MASK);
    }

    /// **RXDMAEN** `[0]` (RW)
    ///
    /// Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> bool {
        (self.value.read_bits() & Self::RXDMAEN_MASK) != 0
    }

    /// Set RXDMAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxdmaen(&self) {
        self.value.modify_bits(|r| r | Self::RXDMAEN_MASK);
    }

    /// Clear RXDMAEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxdmaen(&self) {
        self.value.modify_bits(|r| r & !Self::RXDMAEN_MASK);
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

    /// Read **TXEIE** `[7:7]`
    ///
    /// Tx buffer empty interrupt
    /// enable
    #[inline(always)]
    pub const fn txeie(&self) -> bool {
        (self.bits & Ctlr2::TXEIE_MASK) != 0
    }

    /// Read **RXNEIE** `[6:6]`
    ///
    /// RX buffer not empty interrupt
    /// enable
    #[inline(always)]
    pub const fn rxneie(&self) -> bool {
        (self.bits & Ctlr2::RXNEIE_MASK) != 0
    }

    /// Read **ERRIE** `[5:5]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn errie(&self) -> bool {
        (self.bits & Ctlr2::ERRIE_MASK) != 0
    }

    /// Read **SSOE** `[2:2]`
    ///
    /// SS output enable
    #[inline(always)]
    pub const fn ssoe(&self) -> bool {
        (self.bits & Ctlr2::SSOE_MASK) != 0
    }

    /// Read **TXDMAEN** `[1:1]`
    ///
    /// Tx buffer DMA enable
    #[inline(always)]
    pub const fn txdmaen(&self) -> bool {
        (self.bits & Ctlr2::TXDMAEN_MASK) != 0
    }

    /// Read **RXDMAEN** `[0:0]`
    ///
    /// Rx buffer DMA enable
    #[inline(always)]
    pub const fn rxdmaen(&self) -> bool {
        (self.bits & Ctlr2::RXDMAEN_MASK) != 0
    }

    /// Set **TXEIE** `[7:7]`
    ///
    /// Tx buffer empty interrupt
    /// enable
    #[inline(always)]
    pub const fn with_txeie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::TXEIE_MASK
            } else {
                self.bits & !Ctlr2::TXEIE_MASK
            },
        }
    }

    /// Set **RXNEIE** `[6:6]`
    ///
    /// RX buffer not empty interrupt
    /// enable
    #[inline(always)]
    pub const fn with_rxneie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::RXNEIE_MASK
            } else {
                self.bits & !Ctlr2::RXNEIE_MASK
            },
        }
    }

    /// Set **ERRIE** `[5:5]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn with_errie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::ERRIE_MASK
            } else {
                self.bits & !Ctlr2::ERRIE_MASK
            },
        }
    }

    /// Set **SSOE** `[2:2]`
    ///
    /// SS output enable
    #[inline(always)]
    pub const fn with_ssoe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::SSOE_MASK
            } else {
                self.bits & !Ctlr2::SSOE_MASK
            },
        }
    }

    /// Set **TXDMAEN** `[1:1]`
    ///
    /// Tx buffer DMA enable
    #[inline(always)]
    pub const fn with_txdmaen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::TXDMAEN_MASK
            } else {
                self.bits & !Ctlr2::TXDMAEN_MASK
            },
        }
    }

    /// Set **RXDMAEN** `[0:0]`
    ///
    /// Rx buffer DMA enable
    #[inline(always)]
    pub const fn with_rxdmaen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::RXDMAEN_MASK
            } else {
                self.bits & !Ctlr2::RXDMAEN_MASK
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

/// STATR Register
///
/// status register
///
/// Address offset: 0x0008
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **BSY** `[7]` (RO)
///
///   Busy flag
///
/// - **OVR** `[6]` (RW)
///
///   Overrun flag
///
/// - **MODF** `[5]` (RO)
///
///   Mode fault
///
/// - **CRCERR** `[4]` (RW)
///
///   CRC error flag
///
/// - **UDR** `[3]` (RO)
///
///   Underrun flag
///
/// - **CHSIDE** `[2]` (RO)
///
///   Channel side
///
/// - **TXE** `[1]` (RO)
///
///   Transmit buffer empty
///
/// - **RXNE** `[0]` (RO)
///
///   Receive buffer not empty
#[repr(transparent)]
pub struct Statr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Statr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Statr {
    // ======== Field Constants ========

    /// **BSY**
    ///
    /// Busy flag
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BSY_OFFSET: u32 = 7;
    pub const BSY_MASK: u32 = 0x00000080;
    pub const BSY: u32 = 1 << 7;

    /// **OVR**
    ///
    /// Overrun flag
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OVR_OFFSET: u32 = 6;
    pub const OVR_MASK: u32 = 0x00000040;
    pub const OVR: u32 = 1 << 6;

    /// **MODF**
    ///
    /// Mode fault
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MODF_OFFSET: u32 = 5;
    pub const MODF_MASK: u32 = 0x00000020;
    pub const MODF: u32 = 1 << 5;

    /// **CRCERR**
    ///
    /// CRC error flag
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CRCERR_OFFSET: u32 = 4;
    pub const CRCERR_MASK: u32 = 0x00000010;
    pub const CRCERR: u32 = 1 << 4;

    /// **UDR**
    ///
    /// Underrun flag
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const UDR_OFFSET: u32 = 3;
    pub const UDR_MASK: u32 = 0x00000008;
    pub const UDR: u32 = 1 << 3;

    /// **CHSIDE**
    ///
    /// Channel side
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CHSIDE_OFFSET: u32 = 2;
    pub const CHSIDE_MASK: u32 = 0x00000004;
    pub const CHSIDE: u32 = 1 << 2;

    /// **TXE**
    ///
    /// Transmit buffer empty
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TXE_OFFSET: u32 = 1;
    pub const TXE_MASK: u32 = 0x00000002;
    pub const TXE: u32 = 1 << 1;

    /// **RXNE**
    ///
    /// Receive buffer not empty
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXNE_OFFSET: u32 = 0;
    pub const RXNE_MASK: u32 = 0x00000001;
    pub const RXNE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BSY** `[7]` (RO)
    ///
    /// Busy flag
    #[inline(always)]
    pub fn bsy(&self) -> bool {
        (self.value.read_bits() & Self::BSY_MASK) != 0
    }

    /// **OVR** `[6]` (RW)
    ///
    /// Overrun flag
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

    /// **MODF** `[5]` (RO)
    ///
    /// Mode fault
    #[inline(always)]
    pub fn modf(&self) -> bool {
        (self.value.read_bits() & Self::MODF_MASK) != 0
    }

    /// **CRCERR** `[4]` (RW)
    ///
    /// CRC error flag
    #[inline(always)]
    pub fn crcerr(&self) -> bool {
        (self.value.read_bits() & Self::CRCERR_MASK) != 0
    }

    /// Set CRCERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_crcerr(&self) {
        self.value.modify_bits(|r| r | Self::CRCERR_MASK);
    }

    /// Clear CRCERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_crcerr(&self) {
        self.value.modify_bits(|r| r & !Self::CRCERR_MASK);
    }

    /// **UDR** `[3]` (RO)
    ///
    /// Underrun flag
    #[inline(always)]
    pub fn udr(&self) -> bool {
        (self.value.read_bits() & Self::UDR_MASK) != 0
    }

    /// **CHSIDE** `[2]` (RO)
    ///
    /// Channel side
    #[inline(always)]
    pub fn chside(&self) -> bool {
        (self.value.read_bits() & Self::CHSIDE_MASK) != 0
    }

    /// **TXE** `[1]` (RO)
    ///
    /// Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> bool {
        (self.value.read_bits() & Self::TXE_MASK) != 0
    }

    /// **RXNE** `[0]` (RO)
    ///
    /// Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> bool {
        (self.value.read_bits() & Self::RXNE_MASK) != 0
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
        Self { bits: 0x00000002 }
    }

    /// Read **BSY** `[7:7]`
    ///
    /// Busy flag
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        (self.bits & Statr::BSY_MASK) != 0
    }

    /// Read **OVR** `[6:6]`
    ///
    /// Overrun flag
    #[inline(always)]
    pub const fn ovr(&self) -> bool {
        (self.bits & Statr::OVR_MASK) != 0
    }

    /// Read **MODF** `[5:5]`
    ///
    /// Mode fault
    #[inline(always)]
    pub const fn modf(&self) -> bool {
        (self.bits & Statr::MODF_MASK) != 0
    }

    /// Read **CRCERR** `[4:4]`
    ///
    /// CRC error flag
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        (self.bits & Statr::CRCERR_MASK) != 0
    }

    /// Read **UDR** `[3:3]`
    ///
    /// Underrun flag
    #[inline(always)]
    pub const fn udr(&self) -> bool {
        (self.bits & Statr::UDR_MASK) != 0
    }

    /// Read **CHSIDE** `[2:2]`
    ///
    /// Channel side
    #[inline(always)]
    pub const fn chside(&self) -> bool {
        (self.bits & Statr::CHSIDE_MASK) != 0
    }

    /// Read **TXE** `[1:1]`
    ///
    /// Transmit buffer empty
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        (self.bits & Statr::TXE_MASK) != 0
    }

    /// Read **RXNE** `[0:0]`
    ///
    /// Receive buffer not empty
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        (self.bits & Statr::RXNE_MASK) != 0
    }

    /// Set **OVR** `[6:6]`
    ///
    /// Overrun flag
    #[inline(always)]
    pub const fn with_ovr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::OVR_MASK
            } else {
                self.bits & !Statr::OVR_MASK
            },
        }
    }

    /// Set **CRCERR** `[4:4]`
    ///
    /// CRC error flag
    #[inline(always)]
    pub const fn with_crcerr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::CRCERR_MASK
            } else {
                self.bits & !Statr::CRCERR_MASK
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

/// DATAR Register
///
/// data register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DR** `[15:0]` (RW)
///
///   Data register
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

    /// **DR**
    ///
    /// Data register
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DR_OFFSET: u32 = 0;
    pub const DR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DR** `[15:0]` (RW)
    ///
    /// Data register
    #[inline(always)]
    pub fn dr(&self) -> u32 {
        (self.value.read_bits() & Self::DR_MASK) >> Self::DR_OFFSET
    }

    /// Write **DR** `[15:0]` (RW)
    ///
    /// Data register
    #[inline(always)]
    pub fn write_dr(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::DR_MASK) | ((value << Self::DR_OFFSET) & Self::DR_MASK));
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

    /// Read **DR** `[15:0]`
    ///
    /// Data register
    #[inline(always)]
    pub const fn dr(&self) -> u32 {
        (self.bits & Datar::DR_MASK) >> Datar::DR_OFFSET
    }

    /// Set **DR** `[15:0]`
    ///
    /// Data register
    #[inline(always)]
    pub const fn with_dr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Datar::DR_MASK) | ((value << Datar::DR_OFFSET) & Datar::DR_MASK),
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

/// CRCR Register
///
/// CRCR polynomial register
///
/// Address offset: 0x0010
/// Reset value: 0x00000007
///
/// ## Fields
///
/// - **CRCPOLY** `[15:0]` (RW)
///
///   CRC polynomial register
#[repr(transparent)]
pub struct Crcr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Crcr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Crcr {
    // ======== Field Constants ========

    /// **CRCPOLY**
    ///
    /// CRC polynomial register
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CRCPOLY_OFFSET: u32 = 0;
    pub const CRCPOLY_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CRCPOLY** `[15:0]` (RW)
    ///
    /// CRC polynomial register
    #[inline(always)]
    pub fn crcpoly(&self) -> u32 {
        (self.value.read_bits() & Self::CRCPOLY_MASK) >> Self::CRCPOLY_OFFSET
    }

    /// Write **CRCPOLY** `[15:0]` (RW)
    ///
    /// CRC polynomial register
    #[inline(always)]
    pub fn write_crcpoly(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CRCPOLY_MASK) | ((value << Self::CRCPOLY_OFFSET) & Self::CRCPOLY_MASK)
        });
    }
}

/// CRCR value type for batch read/modify/write operations
///
/// Use `crcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `crcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CrcrVal {
    bits: u32,
}

impl CrcrVal {
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
        Self { bits: 0x00000007 }
    }

    /// Read **CRCPOLY** `[15:0]`
    ///
    /// CRC polynomial register
    #[inline(always)]
    pub const fn crcpoly(&self) -> u32 {
        (self.bits & Crcr::CRCPOLY_MASK) >> Crcr::CRCPOLY_OFFSET
    }

    /// Set **CRCPOLY** `[15:0]`
    ///
    /// CRC polynomial register
    #[inline(always)]
    pub const fn with_crcpoly(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Crcr::CRCPOLY_MASK)
                | ((value << Crcr::CRCPOLY_OFFSET) & Crcr::CRCPOLY_MASK),
        }
    }
}

impl Default for CrcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Crcr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CrcrVal {
        CrcrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CrcrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CrcrVal) -> CrcrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RCRCR Register
///
/// RX CRC register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXCRC** `[15:0]` (RW)
///
///   Rx CRC register
#[repr(transparent)]
pub struct Rcrcr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Rcrcr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rcrcr {
    // ======== Field Constants ========

    /// **RXCRC**
    ///
    /// Rx CRC register
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RXCRC_OFFSET: u32 = 0;
    pub const RXCRC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RXCRC** `[15:0]` (RW)
    ///
    /// Rx CRC register
    #[inline(always)]
    pub fn rxcrc(&self) -> u32 {
        (self.value.read_bits() & Self::RXCRC_MASK) >> Self::RXCRC_OFFSET
    }
}

/// RCRCR value type for batch read/modify/write operations
///
/// Use `rcrcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rcrcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RcrcrVal {
    bits: u32,
}

impl RcrcrVal {
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

    /// Read **RXCRC** `[15:0]`
    ///
    /// Rx CRC register
    #[inline(always)]
    pub const fn rxcrc(&self) -> u32 {
        (self.bits & Rcrcr::RXCRC_MASK) >> Rcrcr::RXCRC_OFFSET
    }

    /// Set **RXCRC** `[15:0]`
    ///
    /// Rx CRC register
    #[inline(always)]
    pub const fn with_rxcrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rcrcr::RXCRC_MASK)
                | ((value << Rcrcr::RXCRC_OFFSET) & Rcrcr::RXCRC_MASK),
        }
    }
}

impl Default for RcrcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rcrcr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RcrcrVal {
        RcrcrVal::from_bits(self.value.read_bits())
    }
}

/// TCRCR Register
///
/// send CRC register
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXCRC** `[15:0]` (RW)
///
///   Tx CRC register
#[repr(transparent)]
pub struct Tcrcr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Tcrcr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tcrcr {
    // ======== Field Constants ========

    /// **TXCRC**
    ///
    /// Tx CRC register
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const TXCRC_OFFSET: u32 = 0;
    pub const TXCRC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **TXCRC** `[15:0]` (RW)
    ///
    /// Tx CRC register
    #[inline(always)]
    pub fn txcrc(&self) -> u32 {
        (self.value.read_bits() & Self::TXCRC_MASK) >> Self::TXCRC_OFFSET
    }
}

/// TCRCR value type for batch read/modify/write operations
///
/// Use `tcrcr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcrcr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcrcrVal {
    bits: u32,
}

impl TcrcrVal {
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

    /// Read **TXCRC** `[15:0]`
    ///
    /// Tx CRC register
    #[inline(always)]
    pub const fn txcrc(&self) -> u32 {
        (self.bits & Tcrcr::TXCRC_MASK) >> Tcrcr::TXCRC_OFFSET
    }

    /// Set **TXCRC** `[15:0]`
    ///
    /// Tx CRC register
    #[inline(always)]
    pub const fn with_txcrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tcrcr::TXCRC_MASK)
                | ((value << Tcrcr::TXCRC_OFFSET) & Tcrcr::TXCRC_MASK),
        }
    }
}

impl Default for TcrcrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Tcrcr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcrcrVal {
        TcrcrVal::from_bits(self.value.read_bits())
    }
}

/// HSCR Register
///
/// high speed control register
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **HSRXEN** `[0]` (RW)
///
///   High speed mode read enable
#[repr(transparent)]
pub struct Hscr {
    value: crate::common::VolatileCellU16,
}

impl core::ops::Deref for Hscr {
    type Target = crate::common::VolatileCellU16;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Hscr {
    // ======== Field Constants ========

    /// **HSRXEN**
    ///
    /// High speed mode read enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HSRXEN_OFFSET: u32 = 0;
    pub const HSRXEN_MASK: u32 = 0x00000001;
    pub const HSRXEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **HSRXEN** `[0]` (RW)
    ///
    /// High speed mode read enable
    #[inline(always)]
    pub fn hsrxen(&self) -> bool {
        (self.value.read_bits() & Self::HSRXEN_MASK) != 0
    }
}

/// HSCR value type for batch read/modify/write operations
///
/// Use `hscr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hscr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HscrVal {
    bits: u32,
}

impl HscrVal {
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

    /// Read **HSRXEN** `[0:0]`
    ///
    /// High speed mode read enable
    #[inline(always)]
    pub const fn hsrxen(&self) -> bool {
        (self.bits & Hscr::HSRXEN_MASK) != 0
    }

    /// Set **HSRXEN** `[0:0]`
    ///
    /// High speed mode read enable
    #[inline(always)]
    pub const fn with_hsrxen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hscr::HSRXEN_MASK
            } else {
                self.bits & !Hscr::HSRXEN_MASK
            },
        }
    }
}

impl Default for HscrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Hscr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HscrVal) {
        self.value.write_bits(val.bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// control register 1
    pub ctlr1: Ctlr1,
    _reserved0: [u8; 2],
    /// 0x0004
    /// control register 2
    pub ctlr2: Ctlr2,
    _reserved1: [u8; 2],
    /// 0x0008
    /// status register
    pub statr: Statr,
    _reserved2: [u8; 2],
    /// 0x000c
    /// data register
    pub datar: Datar,
    _reserved3: [u8; 2],
    /// 0x0010
    /// CRCR polynomial register
    pub crcr: Crcr,
    _reserved4: [u8; 2],
    /// 0x0014
    /// RX CRC register
    pub rcrcr: Rcrcr,
    _reserved5: [u8; 2],
    /// 0x0018
    /// send CRC register
    pub tcrcr: Tcrcr,
    _reserved6: [u8; 10],
    /// 0x0024
    /// high speed control register
    pub hscr: Hscr,
}
