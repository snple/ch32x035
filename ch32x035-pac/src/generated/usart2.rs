//! USART2

#![allow(dead_code)]

pub const BASE: u32 = 0x40004400;

/// STATR Register
///
/// Status register
///
/// Address offset: 0x0000
/// Reset value: 0x000000c0
///
/// ## Fields
///
/// - **CTS** `[9]` (RW)
///
///   CTS flag
///
/// - **LBD** `[8]` (RW)
///
///   LIN break detection flag
///
/// - **TXE** `[7]` (RO)
///
///   Transmit data register
///   empty
///
/// - **TC** `[6]` (RW)
///
///   Transmission complete
///
/// - **RXNE** `[5]` (RW)
///
///   Read data register not
///   empty
///
/// - **IDLE** `[4]` (RO)
///
///   IDLE line detected
///
/// - **ORE** `[3]` (RO)
///
///   Overrun error
///
/// - **NE** `[2]` (RO)
///
///   Noise error flag
///
/// - **FE** `[1]` (RO)
///
///   Framing error
///
/// - **PE** `[0]` (RO)
///
///   Parity error
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

    /// **CTS**
    ///
    /// CTS flag
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CTS_OFFSET: u32 = 9;
    pub const CTS_MASK: u32 = 0x00000200;
    pub const CTS: u32 = 1 << 9;

    /// **LBD**
    ///
    /// LIN break detection flag
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LBD_OFFSET: u32 = 8;
    pub const LBD_MASK: u32 = 0x00000100;
    pub const LBD: u32 = 1 << 8;

    /// **TXE**
    ///
    /// Transmit data register
    /// empty
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TXE_OFFSET: u32 = 7;
    pub const TXE_MASK: u32 = 0x00000080;
    pub const TXE: u32 = 1 << 7;

    /// **TC**
    ///
    /// Transmission complete
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TC_OFFSET: u32 = 6;
    pub const TC_MASK: u32 = 0x00000040;
    pub const TC: u32 = 1 << 6;

    /// **RXNE**
    ///
    /// Read data register not
    /// empty
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RXNE_OFFSET: u32 = 5;
    pub const RXNE_MASK: u32 = 0x00000020;
    pub const RXNE: u32 = 1 << 5;

    /// **IDLE**
    ///
    /// IDLE line detected
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IDLE_OFFSET: u32 = 4;
    pub const IDLE_MASK: u32 = 0x00000010;
    pub const IDLE: u32 = 1 << 4;

    /// **ORE**
    ///
    /// Overrun error
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ORE_OFFSET: u32 = 3;
    pub const ORE_MASK: u32 = 0x00000008;
    pub const ORE: u32 = 1 << 3;

    /// **NE**
    ///
    /// Noise error flag
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const NE_OFFSET: u32 = 2;
    pub const NE_MASK: u32 = 0x00000004;
    pub const NE: u32 = 1 << 2;

    /// **FE**
    ///
    /// Framing error
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FE_OFFSET: u32 = 1;
    pub const FE_MASK: u32 = 0x00000002;
    pub const FE: u32 = 1 << 1;

    /// **PE**
    ///
    /// Parity error
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PE_OFFSET: u32 = 0;
    pub const PE_MASK: u32 = 0x00000001;
    pub const PE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CTS** `[9]` (RW)
    ///
    /// CTS flag
    #[inline(always)]
    pub fn cts(&self) -> bool {
        (self.value.read_bits() & Self::CTS_MASK) != 0
    }

    /// Set CTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cts(&self) {
        self.value.modify_bits(|r| r | Self::CTS_MASK);
    }

    /// Clear CTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cts(&self) {
        self.value.modify_bits(|r| r & !Self::CTS_MASK);
    }

    /// **LBD** `[8]` (RW)
    ///
    /// LIN break detection flag
    #[inline(always)]
    pub fn lbd(&self) -> bool {
        (self.value.read_bits() & Self::LBD_MASK) != 0
    }

    /// Set LBD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lbd(&self) {
        self.value.modify_bits(|r| r | Self::LBD_MASK);
    }

    /// Clear LBD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lbd(&self) {
        self.value.modify_bits(|r| r & !Self::LBD_MASK);
    }

    /// **TXE** `[7]` (RO)
    ///
    /// Transmit data register
    /// empty
    #[inline(always)]
    pub fn txe(&self) -> bool {
        (self.value.read_bits() & Self::TXE_MASK) != 0
    }

    /// **TC** `[6]` (RW)
    ///
    /// Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> bool {
        (self.value.read_bits() & Self::TC_MASK) != 0
    }

    /// Set TC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tc(&self) {
        self.value.modify_bits(|r| r | Self::TC_MASK);
    }

    /// Clear TC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tc(&self) {
        self.value.modify_bits(|r| r & !Self::TC_MASK);
    }

    /// **RXNE** `[5]` (RW)
    ///
    /// Read data register not
    /// empty
    #[inline(always)]
    pub fn rxne(&self) -> bool {
        (self.value.read_bits() & Self::RXNE_MASK) != 0
    }

    /// Set RXNE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rxne(&self) {
        self.value.modify_bits(|r| r | Self::RXNE_MASK);
    }

    /// Clear RXNE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rxne(&self) {
        self.value.modify_bits(|r| r & !Self::RXNE_MASK);
    }

    /// **IDLE** `[4]` (RO)
    ///
    /// IDLE line detected
    #[inline(always)]
    pub fn idle(&self) -> bool {
        (self.value.read_bits() & Self::IDLE_MASK) != 0
    }

    /// **ORE** `[3]` (RO)
    ///
    /// Overrun error
    #[inline(always)]
    pub fn ore(&self) -> bool {
        (self.value.read_bits() & Self::ORE_MASK) != 0
    }

    /// **NE** `[2]` (RO)
    ///
    /// Noise error flag
    #[inline(always)]
    pub fn ne(&self) -> bool {
        (self.value.read_bits() & Self::NE_MASK) != 0
    }

    /// **FE** `[1]` (RO)
    ///
    /// Framing error
    #[inline(always)]
    pub fn fe(&self) -> bool {
        (self.value.read_bits() & Self::FE_MASK) != 0
    }

    /// **PE** `[0]` (RO)
    ///
    /// Parity error
    #[inline(always)]
    pub fn pe(&self) -> bool {
        (self.value.read_bits() & Self::PE_MASK) != 0
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
        Self { bits: 0x000000c0 }
    }

    /// Read **CTS** `[9:9]`
    ///
    /// CTS flag
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        (self.bits & Statr::CTS_MASK) != 0
    }

    /// Read **LBD** `[8:8]`
    ///
    /// LIN break detection flag
    #[inline(always)]
    pub const fn lbd(&self) -> bool {
        (self.bits & Statr::LBD_MASK) != 0
    }

    /// Read **TXE** `[7:7]`
    ///
    /// Transmit data register
    /// empty
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        (self.bits & Statr::TXE_MASK) != 0
    }

    /// Read **TC** `[6:6]`
    ///
    /// Transmission complete
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        (self.bits & Statr::TC_MASK) != 0
    }

    /// Read **RXNE** `[5:5]`
    ///
    /// Read data register not
    /// empty
    #[inline(always)]
    pub const fn rxne(&self) -> bool {
        (self.bits & Statr::RXNE_MASK) != 0
    }

    /// Read **IDLE** `[4:4]`
    ///
    /// IDLE line detected
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        (self.bits & Statr::IDLE_MASK) != 0
    }

    /// Read **ORE** `[3:3]`
    ///
    /// Overrun error
    #[inline(always)]
    pub const fn ore(&self) -> bool {
        (self.bits & Statr::ORE_MASK) != 0
    }

    /// Read **NE** `[2:2]`
    ///
    /// Noise error flag
    #[inline(always)]
    pub const fn ne(&self) -> bool {
        (self.bits & Statr::NE_MASK) != 0
    }

    /// Read **FE** `[1:1]`
    ///
    /// Framing error
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        (self.bits & Statr::FE_MASK) != 0
    }

    /// Read **PE** `[0:0]`
    ///
    /// Parity error
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        (self.bits & Statr::PE_MASK) != 0
    }

    /// Set **CTS** `[9:9]`
    ///
    /// CTS flag
    #[inline(always)]
    pub const fn with_cts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::CTS_MASK
            } else {
                self.bits & !Statr::CTS_MASK
            },
        }
    }

    /// Set **LBD** `[8:8]`
    ///
    /// LIN break detection flag
    #[inline(always)]
    pub const fn with_lbd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::LBD_MASK
            } else {
                self.bits & !Statr::LBD_MASK
            },
        }
    }

    /// Set **TC** `[6:6]`
    ///
    /// Transmission complete
    #[inline(always)]
    pub const fn with_tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::TC_MASK
            } else {
                self.bits & !Statr::TC_MASK
            },
        }
    }

    /// Set **RXNE** `[5:5]`
    ///
    /// Read data register not
    /// empty
    #[inline(always)]
    pub const fn with_rxne(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Statr::RXNE_MASK
            } else {
                self.bits & !Statr::RXNE_MASK
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
/// Data register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DR** `[8:0]` (RW)
///
///   Data value
#[repr(transparent)]
pub struct Datar {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Datar {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Datar {
    // ======== Field Constants ========

    /// **DR**
    ///
    /// Data value
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const DR_OFFSET: u32 = 0;
    pub const DR_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **DR** `[8:0]` (RW)
    ///
    /// Data value
    #[inline(always)]
    pub fn dr(&self) -> u32 {
        (self.value.read_bits() & Self::DR_MASK) >> Self::DR_OFFSET
    }

    /// Write **DR** `[8:0]` (RW)
    ///
    /// Data value
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

    /// Read **DR** `[8:0]`
    ///
    /// Data value
    #[inline(always)]
    pub const fn dr(&self) -> u32 {
        (self.bits & Datar::DR_MASK) >> Datar::DR_OFFSET
    }

    /// Set **DR** `[8:0]`
    ///
    /// Data value
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

/// BRR Register
///
/// Baud rate register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DIV_Mantissa** `[15:4]` (RW)
///
///   mantissa of USARTDIV
///
/// - **DIV_Fraction** `[3:0]` (RW)
///
///   fraction of USARTDIV
#[repr(transparent)]
pub struct Brr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Brr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Brr {
    // ======== Field Constants ========

    /// **DIV_Mantissa**
    ///
    /// mantissa of USARTDIV
    ///
    /// Bits `[15:4]`, mask `0x0000fff0`
    pub const DIV_MANTISSA_OFFSET: u32 = 4;
    pub const DIV_MANTISSA_MASK: u32 = 0x0000fff0;

    /// **DIV_Fraction**
    ///
    /// fraction of USARTDIV
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DIV_FRACTION_OFFSET: u32 = 0;
    pub const DIV_FRACTION_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DIV_Mantissa** `[15:4]` (RW)
    ///
    /// mantissa of USARTDIV
    #[inline(always)]
    pub fn div_mantissa(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_MANTISSA_MASK) >> Self::DIV_MANTISSA_OFFSET
    }

    /// Write **DIV_Mantissa** `[15:4]` (RW)
    ///
    /// mantissa of USARTDIV
    #[inline(always)]
    pub fn write_div_mantissa(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_MANTISSA_MASK)
                | ((value << Self::DIV_MANTISSA_OFFSET) & Self::DIV_MANTISSA_MASK)
        });
    }

    /// **DIV_Fraction** `[3:0]` (RW)
    ///
    /// fraction of USARTDIV
    #[inline(always)]
    pub fn div_fraction(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_FRACTION_MASK) >> Self::DIV_FRACTION_OFFSET
    }

    /// Write **DIV_Fraction** `[3:0]` (RW)
    ///
    /// fraction of USARTDIV
    #[inline(always)]
    pub fn write_div_fraction(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIV_FRACTION_MASK)
                | ((value << Self::DIV_FRACTION_OFFSET) & Self::DIV_FRACTION_MASK)
        });
    }
}

/// BRR value type for batch read/modify/write operations
///
/// Use `brr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `brr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BrrVal {
    bits: u32,
}

impl BrrVal {
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

    /// Read **DIV_Mantissa** `[15:4]`
    ///
    /// mantissa of USARTDIV
    #[inline(always)]
    pub const fn div_mantissa(&self) -> u32 {
        (self.bits & Brr::DIV_MANTISSA_MASK) >> Brr::DIV_MANTISSA_OFFSET
    }

    /// Read **DIV_Fraction** `[3:0]`
    ///
    /// fraction of USARTDIV
    #[inline(always)]
    pub const fn div_fraction(&self) -> u32 {
        (self.bits & Brr::DIV_FRACTION_MASK) >> Brr::DIV_FRACTION_OFFSET
    }

    /// Set **DIV_Mantissa** `[15:4]`
    ///
    /// mantissa of USARTDIV
    #[inline(always)]
    pub const fn with_div_mantissa(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Brr::DIV_MANTISSA_MASK)
                | ((value << Brr::DIV_MANTISSA_OFFSET) & Brr::DIV_MANTISSA_MASK),
        }
    }

    /// Set **DIV_Fraction** `[3:0]`
    ///
    /// fraction of USARTDIV
    #[inline(always)]
    pub const fn with_div_fraction(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Brr::DIV_FRACTION_MASK)
                | ((value << Brr::DIV_FRACTION_OFFSET) & Brr::DIV_FRACTION_MASK),
        }
    }
}

impl Default for BrrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Brr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BrrVal {
        BrrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BrrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BrrVal) -> BrrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTLR1 Register
///
/// Control register 1
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **UE** `[13]` (RW)
///
///   USART enable
///
/// - **M** `[12]` (RW)
///
///   Word length
///
/// - **WAKE** `[11]` (RW)
///
///   Wakeup method
///
/// - **PCE** `[10]` (RW)
///
///   Parity control enable
///
/// - **PS** `[9]` (RW)
///
///   Parity selection
///
/// - **PEIE** `[8]` (RW)
///
///   PE interrupt enable
///
/// - **TXEIE** `[7]` (RW)
///
///   TXE interrupt enable
///
/// - **TCIE** `[6]` (RW)
///
///   Transmission complete interrupt
///   enable
///
/// - **RXNEIE** `[5]` (RW)
///
///   RXNE interrupt enable
///
/// - **IDLEIE** `[4]` (RW)
///
///   IDLE interrupt enable
///
/// - **TE** `[3]` (RW)
///
///   Transmitter enable
///
/// - **RE** `[2]` (RW)
///
///   Receiver enable
///
/// - **RWU** `[1]` (RW)
///
///   Receiver wakeup
///
/// - **SBK** `[0]` (RW)
///
///   Send break
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

    /// **UE**
    ///
    /// USART enable
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const UE_OFFSET: u32 = 13;
    pub const UE_MASK: u32 = 0x00002000;
    pub const UE: u32 = 1 << 13;

    /// **M**
    ///
    /// Word length
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const M_OFFSET: u32 = 12;
    pub const M_MASK: u32 = 0x00001000;
    pub const M: u32 = 1 << 12;

    /// **WAKE**
    ///
    /// Wakeup method
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const WAKE_OFFSET: u32 = 11;
    pub const WAKE_MASK: u32 = 0x00000800;
    pub const WAKE: u32 = 1 << 11;

    /// **PCE**
    ///
    /// Parity control enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const PCE_OFFSET: u32 = 10;
    pub const PCE_MASK: u32 = 0x00000400;
    pub const PCE: u32 = 1 << 10;

    /// **PS**
    ///
    /// Parity selection
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PS_OFFSET: u32 = 9;
    pub const PS_MASK: u32 = 0x00000200;
    pub const PS: u32 = 1 << 9;

    /// **PEIE**
    ///
    /// PE interrupt enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PEIE_OFFSET: u32 = 8;
    pub const PEIE_MASK: u32 = 0x00000100;
    pub const PEIE: u32 = 1 << 8;

    /// **TXEIE**
    ///
    /// TXE interrupt enable
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TXEIE_OFFSET: u32 = 7;
    pub const TXEIE_MASK: u32 = 0x00000080;
    pub const TXEIE: u32 = 1 << 7;

    /// **TCIE**
    ///
    /// Transmission complete interrupt
    /// enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TCIE_OFFSET: u32 = 6;
    pub const TCIE_MASK: u32 = 0x00000040;
    pub const TCIE: u32 = 1 << 6;

    /// **RXNEIE**
    ///
    /// RXNE interrupt enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RXNEIE_OFFSET: u32 = 5;
    pub const RXNEIE_MASK: u32 = 0x00000020;
    pub const RXNEIE: u32 = 1 << 5;

    /// **IDLEIE**
    ///
    /// IDLE interrupt enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IDLEIE_OFFSET: u32 = 4;
    pub const IDLEIE_MASK: u32 = 0x00000010;
    pub const IDLEIE: u32 = 1 << 4;

    /// **TE**
    ///
    /// Transmitter enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TE_OFFSET: u32 = 3;
    pub const TE_MASK: u32 = 0x00000008;
    pub const TE: u32 = 1 << 3;

    /// **RE**
    ///
    /// Receiver enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RE_OFFSET: u32 = 2;
    pub const RE_MASK: u32 = 0x00000004;
    pub const RE: u32 = 1 << 2;

    /// **RWU**
    ///
    /// Receiver wakeup
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RWU_OFFSET: u32 = 1;
    pub const RWU_MASK: u32 = 0x00000002;
    pub const RWU: u32 = 1 << 1;

    /// **SBK**
    ///
    /// Send break
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SBK_OFFSET: u32 = 0;
    pub const SBK_MASK: u32 = 0x00000001;
    pub const SBK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **UE** `[13]` (RW)
    ///
    /// USART enable
    #[inline(always)]
    pub fn ue(&self) -> bool {
        (self.value.read_bits() & Self::UE_MASK) != 0
    }

    /// Set UE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ue(&self) {
        self.value.modify_bits(|r| r | Self::UE_MASK);
    }

    /// Clear UE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ue(&self) {
        self.value.modify_bits(|r| r & !Self::UE_MASK);
    }

    /// **M** `[12]` (RW)
    ///
    /// Word length
    #[inline(always)]
    pub fn m(&self) -> bool {
        (self.value.read_bits() & Self::M_MASK) != 0
    }

    /// Set M bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_m(&self) {
        self.value.modify_bits(|r| r | Self::M_MASK);
    }

    /// Clear M bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_m(&self) {
        self.value.modify_bits(|r| r & !Self::M_MASK);
    }

    /// **WAKE** `[11]` (RW)
    ///
    /// Wakeup method
    #[inline(always)]
    pub fn wake(&self) -> bool {
        (self.value.read_bits() & Self::WAKE_MASK) != 0
    }

    /// Set WAKE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wake(&self) {
        self.value.modify_bits(|r| r | Self::WAKE_MASK);
    }

    /// Clear WAKE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wake(&self) {
        self.value.modify_bits(|r| r & !Self::WAKE_MASK);
    }

    /// **PCE** `[10]` (RW)
    ///
    /// Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> bool {
        (self.value.read_bits() & Self::PCE_MASK) != 0
    }

    /// Set PCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pce(&self) {
        self.value.modify_bits(|r| r | Self::PCE_MASK);
    }

    /// Clear PCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pce(&self) {
        self.value.modify_bits(|r| r & !Self::PCE_MASK);
    }

    /// **PS** `[9]` (RW)
    ///
    /// Parity selection
    #[inline(always)]
    pub fn ps(&self) -> bool {
        (self.value.read_bits() & Self::PS_MASK) != 0
    }

    /// Set PS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ps(&self) {
        self.value.modify_bits(|r| r | Self::PS_MASK);
    }

    /// Clear PS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ps(&self) {
        self.value.modify_bits(|r| r & !Self::PS_MASK);
    }

    /// **PEIE** `[8]` (RW)
    ///
    /// PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> bool {
        (self.value.read_bits() & Self::PEIE_MASK) != 0
    }

    /// Set PEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_peie(&self) {
        self.value.modify_bits(|r| r | Self::PEIE_MASK);
    }

    /// Clear PEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_peie(&self) {
        self.value.modify_bits(|r| r & !Self::PEIE_MASK);
    }

    /// **TXEIE** `[7]` (RW)
    ///
    /// TXE interrupt enable
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

    /// **TCIE** `[6]` (RW)
    ///
    /// Transmission complete interrupt
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

    /// **RXNEIE** `[5]` (RW)
    ///
    /// RXNE interrupt enable
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

    /// **IDLEIE** `[4]` (RW)
    ///
    /// IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> bool {
        (self.value.read_bits() & Self::IDLEIE_MASK) != 0
    }

    /// Set IDLEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idleie(&self) {
        self.value.modify_bits(|r| r | Self::IDLEIE_MASK);
    }

    /// Clear IDLEIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idleie(&self) {
        self.value.modify_bits(|r| r & !Self::IDLEIE_MASK);
    }

    /// **TE** `[3]` (RW)
    ///
    /// Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> bool {
        (self.value.read_bits() & Self::TE_MASK) != 0
    }

    /// Set TE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_te(&self) {
        self.value.modify_bits(|r| r | Self::TE_MASK);
    }

    /// Clear TE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_te(&self) {
        self.value.modify_bits(|r| r & !Self::TE_MASK);
    }

    /// **RE** `[2]` (RW)
    ///
    /// Receiver enable
    #[inline(always)]
    pub fn re(&self) -> bool {
        (self.value.read_bits() & Self::RE_MASK) != 0
    }

    /// Set RE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_re(&self) {
        self.value.modify_bits(|r| r | Self::RE_MASK);
    }

    /// Clear RE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_re(&self) {
        self.value.modify_bits(|r| r & !Self::RE_MASK);
    }

    /// **RWU** `[1]` (RW)
    ///
    /// Receiver wakeup
    #[inline(always)]
    pub fn rwu(&self) -> bool {
        (self.value.read_bits() & Self::RWU_MASK) != 0
    }

    /// Set RWU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rwu(&self) {
        self.value.modify_bits(|r| r | Self::RWU_MASK);
    }

    /// Clear RWU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rwu(&self) {
        self.value.modify_bits(|r| r & !Self::RWU_MASK);
    }

    /// **SBK** `[0]` (RW)
    ///
    /// Send break
    #[inline(always)]
    pub fn sbk(&self) -> bool {
        (self.value.read_bits() & Self::SBK_MASK) != 0
    }

    /// Set SBK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sbk(&self) {
        self.value.modify_bits(|r| r | Self::SBK_MASK);
    }

    /// Clear SBK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sbk(&self) {
        self.value.modify_bits(|r| r & !Self::SBK_MASK);
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

    /// Read **UE** `[13:13]`
    ///
    /// USART enable
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        (self.bits & Ctlr1::UE_MASK) != 0
    }

    /// Read **M** `[12:12]`
    ///
    /// Word length
    #[inline(always)]
    pub const fn m(&self) -> bool {
        (self.bits & Ctlr1::M_MASK) != 0
    }

    /// Read **WAKE** `[11:11]`
    ///
    /// Wakeup method
    #[inline(always)]
    pub const fn wake(&self) -> bool {
        (self.bits & Ctlr1::WAKE_MASK) != 0
    }

    /// Read **PCE** `[10:10]`
    ///
    /// Parity control enable
    #[inline(always)]
    pub const fn pce(&self) -> bool {
        (self.bits & Ctlr1::PCE_MASK) != 0
    }

    /// Read **PS** `[9:9]`
    ///
    /// Parity selection
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        (self.bits & Ctlr1::PS_MASK) != 0
    }

    /// Read **PEIE** `[8:8]`
    ///
    /// PE interrupt enable
    #[inline(always)]
    pub const fn peie(&self) -> bool {
        (self.bits & Ctlr1::PEIE_MASK) != 0
    }

    /// Read **TXEIE** `[7:7]`
    ///
    /// TXE interrupt enable
    #[inline(always)]
    pub const fn txeie(&self) -> bool {
        (self.bits & Ctlr1::TXEIE_MASK) != 0
    }

    /// Read **TCIE** `[6:6]`
    ///
    /// Transmission complete interrupt
    /// enable
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        (self.bits & Ctlr1::TCIE_MASK) != 0
    }

    /// Read **RXNEIE** `[5:5]`
    ///
    /// RXNE interrupt enable
    #[inline(always)]
    pub const fn rxneie(&self) -> bool {
        (self.bits & Ctlr1::RXNEIE_MASK) != 0
    }

    /// Read **IDLEIE** `[4:4]`
    ///
    /// IDLE interrupt enable
    #[inline(always)]
    pub const fn idleie(&self) -> bool {
        (self.bits & Ctlr1::IDLEIE_MASK) != 0
    }

    /// Read **TE** `[3:3]`
    ///
    /// Transmitter enable
    #[inline(always)]
    pub const fn te(&self) -> bool {
        (self.bits & Ctlr1::TE_MASK) != 0
    }

    /// Read **RE** `[2:2]`
    ///
    /// Receiver enable
    #[inline(always)]
    pub const fn re(&self) -> bool {
        (self.bits & Ctlr1::RE_MASK) != 0
    }

    /// Read **RWU** `[1:1]`
    ///
    /// Receiver wakeup
    #[inline(always)]
    pub const fn rwu(&self) -> bool {
        (self.bits & Ctlr1::RWU_MASK) != 0
    }

    /// Read **SBK** `[0:0]`
    ///
    /// Send break
    #[inline(always)]
    pub const fn sbk(&self) -> bool {
        (self.bits & Ctlr1::SBK_MASK) != 0
    }

    /// Set **UE** `[13:13]`
    ///
    /// USART enable
    #[inline(always)]
    pub const fn with_ue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::UE_MASK
            } else {
                self.bits & !Ctlr1::UE_MASK
            },
        }
    }

    /// Set **M** `[12:12]`
    ///
    /// Word length
    #[inline(always)]
    pub const fn with_m(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::M_MASK
            } else {
                self.bits & !Ctlr1::M_MASK
            },
        }
    }

    /// Set **WAKE** `[11:11]`
    ///
    /// Wakeup method
    #[inline(always)]
    pub const fn with_wake(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::WAKE_MASK
            } else {
                self.bits & !Ctlr1::WAKE_MASK
            },
        }
    }

    /// Set **PCE** `[10:10]`
    ///
    /// Parity control enable
    #[inline(always)]
    pub const fn with_pce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::PCE_MASK
            } else {
                self.bits & !Ctlr1::PCE_MASK
            },
        }
    }

    /// Set **PS** `[9:9]`
    ///
    /// Parity selection
    #[inline(always)]
    pub const fn with_ps(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::PS_MASK
            } else {
                self.bits & !Ctlr1::PS_MASK
            },
        }
    }

    /// Set **PEIE** `[8:8]`
    ///
    /// PE interrupt enable
    #[inline(always)]
    pub const fn with_peie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::PEIE_MASK
            } else {
                self.bits & !Ctlr1::PEIE_MASK
            },
        }
    }

    /// Set **TXEIE** `[7:7]`
    ///
    /// TXE interrupt enable
    #[inline(always)]
    pub const fn with_txeie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::TXEIE_MASK
            } else {
                self.bits & !Ctlr1::TXEIE_MASK
            },
        }
    }

    /// Set **TCIE** `[6:6]`
    ///
    /// Transmission complete interrupt
    /// enable
    #[inline(always)]
    pub const fn with_tcie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::TCIE_MASK
            } else {
                self.bits & !Ctlr1::TCIE_MASK
            },
        }
    }

    /// Set **RXNEIE** `[5:5]`
    ///
    /// RXNE interrupt enable
    #[inline(always)]
    pub const fn with_rxneie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::RXNEIE_MASK
            } else {
                self.bits & !Ctlr1::RXNEIE_MASK
            },
        }
    }

    /// Set **IDLEIE** `[4:4]`
    ///
    /// IDLE interrupt enable
    #[inline(always)]
    pub const fn with_idleie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::IDLEIE_MASK
            } else {
                self.bits & !Ctlr1::IDLEIE_MASK
            },
        }
    }

    /// Set **TE** `[3:3]`
    ///
    /// Transmitter enable
    #[inline(always)]
    pub const fn with_te(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::TE_MASK
            } else {
                self.bits & !Ctlr1::TE_MASK
            },
        }
    }

    /// Set **RE** `[2:2]`
    ///
    /// Receiver enable
    #[inline(always)]
    pub const fn with_re(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::RE_MASK
            } else {
                self.bits & !Ctlr1::RE_MASK
            },
        }
    }

    /// Set **RWU** `[1:1]`
    ///
    /// Receiver wakeup
    #[inline(always)]
    pub const fn with_rwu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::RWU_MASK
            } else {
                self.bits & !Ctlr1::RWU_MASK
            },
        }
    }

    /// Set **SBK** `[0:0]`
    ///
    /// Send break
    #[inline(always)]
    pub const fn with_sbk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr1::SBK_MASK
            } else {
                self.bits & !Ctlr1::SBK_MASK
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
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LINEN** `[14]` (RW)
///
///   LIN mode enable
///
/// - **STOP** `[13:12]` (RW)
///
///   STOP bits
///
/// - **CLKEN** `[11]` (RW)
///
///   Clock enable
///
/// - **CPOL** `[10]` (RW)
///
///   Clock polarity
///
/// - **CPHA** `[9]` (RW)
///
///   Clock phase
///
/// - **LBCL** `[8]` (RW)
///
///   Last bit clock pulse
///
/// - **LBDIE** `[6]` (RW)
///
///   LIN break detection interrupt
///   enable
///
/// - **LBDL** `[5]` (RW)
///
///   lin break detection length
///
/// - **ADD** `[3:0]` (RW)
///
///   Address of the USART node
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

    /// **LINEN**
    ///
    /// LIN mode enable
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LINEN_OFFSET: u32 = 14;
    pub const LINEN_MASK: u32 = 0x00004000;
    pub const LINEN: u32 = 1 << 14;

    /// **STOP**
    ///
    /// STOP bits
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const STOP_OFFSET: u32 = 12;
    pub const STOP_MASK: u32 = 0x00003000;

    /// **CLKEN**
    ///
    /// Clock enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLKEN_OFFSET: u32 = 11;
    pub const CLKEN_MASK: u32 = 0x00000800;
    pub const CLKEN: u32 = 1 << 11;

    /// **CPOL**
    ///
    /// Clock polarity
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CPOL_OFFSET: u32 = 10;
    pub const CPOL_MASK: u32 = 0x00000400;
    pub const CPOL: u32 = 1 << 10;

    /// **CPHA**
    ///
    /// Clock phase
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CPHA_OFFSET: u32 = 9;
    pub const CPHA_MASK: u32 = 0x00000200;
    pub const CPHA: u32 = 1 << 9;

    /// **LBCL**
    ///
    /// Last bit clock pulse
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LBCL_OFFSET: u32 = 8;
    pub const LBCL_MASK: u32 = 0x00000100;
    pub const LBCL: u32 = 1 << 8;

    /// **LBDIE**
    ///
    /// LIN break detection interrupt
    /// enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LBDIE_OFFSET: u32 = 6;
    pub const LBDIE_MASK: u32 = 0x00000040;
    pub const LBDIE: u32 = 1 << 6;

    /// **LBDL**
    ///
    /// lin break detection length
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LBDL_OFFSET: u32 = 5;
    pub const LBDL_MASK: u32 = 0x00000020;
    pub const LBDL: u32 = 1 << 5;

    /// **ADD**
    ///
    /// Address of the USART node
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const ADD_OFFSET: u32 = 0;
    pub const ADD_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **LINEN** `[14]` (RW)
    ///
    /// LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> bool {
        (self.value.read_bits() & Self::LINEN_MASK) != 0
    }

    /// Set LINEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_linen(&self) {
        self.value.modify_bits(|r| r | Self::LINEN_MASK);
    }

    /// Clear LINEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_linen(&self) {
        self.value.modify_bits(|r| r & !Self::LINEN_MASK);
    }

    /// **STOP** `[13:12]` (RW)
    ///
    /// STOP bits
    #[inline(always)]
    pub fn stop(&self) -> u32 {
        (self.value.read_bits() & Self::STOP_MASK) >> Self::STOP_OFFSET
    }

    /// Write **STOP** `[13:12]` (RW)
    ///
    /// STOP bits
    #[inline(always)]
    pub fn write_stop(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::STOP_MASK) | ((value << Self::STOP_OFFSET) & Self::STOP_MASK)
        });
    }

    /// **CLKEN** `[11]` (RW)
    ///
    /// Clock enable
    #[inline(always)]
    pub fn clken(&self) -> bool {
        (self.value.read_bits() & Self::CLKEN_MASK) != 0
    }

    /// Set CLKEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clken(&self) {
        self.value.modify_bits(|r| r | Self::CLKEN_MASK);
    }

    /// Clear CLKEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clken(&self) {
        self.value.modify_bits(|r| r & !Self::CLKEN_MASK);
    }

    /// **CPOL** `[10]` (RW)
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

    /// **CPHA** `[9]` (RW)
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

    /// **LBCL** `[8]` (RW)
    ///
    /// Last bit clock pulse
    #[inline(always)]
    pub fn lbcl(&self) -> bool {
        (self.value.read_bits() & Self::LBCL_MASK) != 0
    }

    /// Set LBCL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lbcl(&self) {
        self.value.modify_bits(|r| r | Self::LBCL_MASK);
    }

    /// Clear LBCL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lbcl(&self) {
        self.value.modify_bits(|r| r & !Self::LBCL_MASK);
    }

    /// **LBDIE** `[6]` (RW)
    ///
    /// LIN break detection interrupt
    /// enable
    #[inline(always)]
    pub fn lbdie(&self) -> bool {
        (self.value.read_bits() & Self::LBDIE_MASK) != 0
    }

    /// Set LBDIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lbdie(&self) {
        self.value.modify_bits(|r| r | Self::LBDIE_MASK);
    }

    /// Clear LBDIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lbdie(&self) {
        self.value.modify_bits(|r| r & !Self::LBDIE_MASK);
    }

    /// **LBDL** `[5]` (RW)
    ///
    /// lin break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> bool {
        (self.value.read_bits() & Self::LBDL_MASK) != 0
    }

    /// Set LBDL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lbdl(&self) {
        self.value.modify_bits(|r| r | Self::LBDL_MASK);
    }

    /// Clear LBDL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lbdl(&self) {
        self.value.modify_bits(|r| r & !Self::LBDL_MASK);
    }

    /// **ADD** `[3:0]` (RW)
    ///
    /// Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> u32 {
        (self.value.read_bits() & Self::ADD_MASK) >> Self::ADD_OFFSET
    }

    /// Write **ADD** `[3:0]` (RW)
    ///
    /// Address of the USART node
    #[inline(always)]
    pub fn write_add(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADD_MASK) | ((value << Self::ADD_OFFSET) & Self::ADD_MASK)
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

    /// Read **LINEN** `[14:14]`
    ///
    /// LIN mode enable
    #[inline(always)]
    pub const fn linen(&self) -> bool {
        (self.bits & Ctlr2::LINEN_MASK) != 0
    }

    /// Read **STOP** `[13:12]`
    ///
    /// STOP bits
    #[inline(always)]
    pub const fn stop(&self) -> u32 {
        (self.bits & Ctlr2::STOP_MASK) >> Ctlr2::STOP_OFFSET
    }

    /// Read **CLKEN** `[11:11]`
    ///
    /// Clock enable
    #[inline(always)]
    pub const fn clken(&self) -> bool {
        (self.bits & Ctlr2::CLKEN_MASK) != 0
    }

    /// Read **CPOL** `[10:10]`
    ///
    /// Clock polarity
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        (self.bits & Ctlr2::CPOL_MASK) != 0
    }

    /// Read **CPHA** `[9:9]`
    ///
    /// Clock phase
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        (self.bits & Ctlr2::CPHA_MASK) != 0
    }

    /// Read **LBCL** `[8:8]`
    ///
    /// Last bit clock pulse
    #[inline(always)]
    pub const fn lbcl(&self) -> bool {
        (self.bits & Ctlr2::LBCL_MASK) != 0
    }

    /// Read **LBDIE** `[6:6]`
    ///
    /// LIN break detection interrupt
    /// enable
    #[inline(always)]
    pub const fn lbdie(&self) -> bool {
        (self.bits & Ctlr2::LBDIE_MASK) != 0
    }

    /// Read **LBDL** `[5:5]`
    ///
    /// lin break detection length
    #[inline(always)]
    pub const fn lbdl(&self) -> bool {
        (self.bits & Ctlr2::LBDL_MASK) != 0
    }

    /// Read **ADD** `[3:0]`
    ///
    /// Address of the USART node
    #[inline(always)]
    pub const fn add(&self) -> u32 {
        (self.bits & Ctlr2::ADD_MASK) >> Ctlr2::ADD_OFFSET
    }

    /// Set **LINEN** `[14:14]`
    ///
    /// LIN mode enable
    #[inline(always)]
    pub const fn with_linen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::LINEN_MASK
            } else {
                self.bits & !Ctlr2::LINEN_MASK
            },
        }
    }

    /// Set **STOP** `[13:12]`
    ///
    /// STOP bits
    #[inline(always)]
    pub const fn with_stop(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr2::STOP_MASK)
                | ((value << Ctlr2::STOP_OFFSET) & Ctlr2::STOP_MASK),
        }
    }

    /// Set **CLKEN** `[11:11]`
    ///
    /// Clock enable
    #[inline(always)]
    pub const fn with_clken(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CLKEN_MASK
            } else {
                self.bits & !Ctlr2::CLKEN_MASK
            },
        }
    }

    /// Set **CPOL** `[10:10]`
    ///
    /// Clock polarity
    #[inline(always)]
    pub const fn with_cpol(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CPOL_MASK
            } else {
                self.bits & !Ctlr2::CPOL_MASK
            },
        }
    }

    /// Set **CPHA** `[9:9]`
    ///
    /// Clock phase
    #[inline(always)]
    pub const fn with_cpha(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::CPHA_MASK
            } else {
                self.bits & !Ctlr2::CPHA_MASK
            },
        }
    }

    /// Set **LBCL** `[8:8]`
    ///
    /// Last bit clock pulse
    #[inline(always)]
    pub const fn with_lbcl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::LBCL_MASK
            } else {
                self.bits & !Ctlr2::LBCL_MASK
            },
        }
    }

    /// Set **LBDIE** `[6:6]`
    ///
    /// LIN break detection interrupt
    /// enable
    #[inline(always)]
    pub const fn with_lbdie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::LBDIE_MASK
            } else {
                self.bits & !Ctlr2::LBDIE_MASK
            },
        }
    }

    /// Set **LBDL** `[5:5]`
    ///
    /// lin break detection length
    #[inline(always)]
    pub const fn with_lbdl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr2::LBDL_MASK
            } else {
                self.bits & !Ctlr2::LBDL_MASK
            },
        }
    }

    /// Set **ADD** `[3:0]`
    ///
    /// Address of the USART node
    #[inline(always)]
    pub const fn with_add(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctlr2::ADD_MASK) | ((value << Ctlr2::ADD_OFFSET) & Ctlr2::ADD_MASK),
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

/// CTLR3 Register
///
/// Control register 3
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CTSIE** `[10]` (RW)
///
///   CTS interrupt enable
///
/// - **CTSE** `[9]` (RW)
///
///   CTS enable
///
/// - **RTSE** `[8]` (RW)
///
///   RTS enable
///
/// - **DMAT** `[7]` (RW)
///
///   DMA enable transmitter
///
/// - **DMAR** `[6]` (RW)
///
///   DMA enable receiver
///
/// - **SCEN** `[5]` (RW)
///
///   Smartcard mode enable
///
/// - **NACK** `[4]` (RW)
///
///   Smartcard NACK enable
///
/// - **HDSEL** `[3]` (RW)
///
///   Half-duplex selection
///
/// - **IRLP** `[2]` (RW)
///
///   IrDA low-power
///
/// - **IREN** `[1]` (RW)
///
///   IrDA mode enable
///
/// - **EIE** `[0]` (RW)
///
///   Error interrupt enable
#[repr(transparent)]
pub struct Ctlr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctlr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctlr3 {
    // ======== Field Constants ========

    /// **CTSIE**
    ///
    /// CTS interrupt enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CTSIE_OFFSET: u32 = 10;
    pub const CTSIE_MASK: u32 = 0x00000400;
    pub const CTSIE: u32 = 1 << 10;

    /// **CTSE**
    ///
    /// CTS enable
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CTSE_OFFSET: u32 = 9;
    pub const CTSE_MASK: u32 = 0x00000200;
    pub const CTSE: u32 = 1 << 9;

    /// **RTSE**
    ///
    /// RTS enable
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RTSE_OFFSET: u32 = 8;
    pub const RTSE_MASK: u32 = 0x00000100;
    pub const RTSE: u32 = 1 << 8;

    /// **DMAT**
    ///
    /// DMA enable transmitter
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DMAT_OFFSET: u32 = 7;
    pub const DMAT_MASK: u32 = 0x00000080;
    pub const DMAT: u32 = 1 << 7;

    /// **DMAR**
    ///
    /// DMA enable receiver
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMAR_OFFSET: u32 = 6;
    pub const DMAR_MASK: u32 = 0x00000040;
    pub const DMAR: u32 = 1 << 6;

    /// **SCEN**
    ///
    /// Smartcard mode enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SCEN_OFFSET: u32 = 5;
    pub const SCEN_MASK: u32 = 0x00000020;
    pub const SCEN: u32 = 1 << 5;

    /// **NACK**
    ///
    /// Smartcard NACK enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const NACK_OFFSET: u32 = 4;
    pub const NACK_MASK: u32 = 0x00000010;
    pub const NACK: u32 = 1 << 4;

    /// **HDSEL**
    ///
    /// Half-duplex selection
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const HDSEL_OFFSET: u32 = 3;
    pub const HDSEL_MASK: u32 = 0x00000008;
    pub const HDSEL: u32 = 1 << 3;

    /// **IRLP**
    ///
    /// IrDA low-power
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IRLP_OFFSET: u32 = 2;
    pub const IRLP_MASK: u32 = 0x00000004;
    pub const IRLP: u32 = 1 << 2;

    /// **IREN**
    ///
    /// IrDA mode enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IREN_OFFSET: u32 = 1;
    pub const IREN_MASK: u32 = 0x00000002;
    pub const IREN: u32 = 1 << 1;

    /// **EIE**
    ///
    /// Error interrupt enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EIE_OFFSET: u32 = 0;
    pub const EIE_MASK: u32 = 0x00000001;
    pub const EIE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CTSIE** `[10]` (RW)
    ///
    /// CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> bool {
        (self.value.read_bits() & Self::CTSIE_MASK) != 0
    }

    /// Set CTSIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ctsie(&self) {
        self.value.modify_bits(|r| r | Self::CTSIE_MASK);
    }

    /// Clear CTSIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ctsie(&self) {
        self.value.modify_bits(|r| r & !Self::CTSIE_MASK);
    }

    /// **CTSE** `[9]` (RW)
    ///
    /// CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> bool {
        (self.value.read_bits() & Self::CTSE_MASK) != 0
    }

    /// Set CTSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ctse(&self) {
        self.value.modify_bits(|r| r | Self::CTSE_MASK);
    }

    /// Clear CTSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ctse(&self) {
        self.value.modify_bits(|r| r & !Self::CTSE_MASK);
    }

    /// **RTSE** `[8]` (RW)
    ///
    /// RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> bool {
        (self.value.read_bits() & Self::RTSE_MASK) != 0
    }

    /// Set RTSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rtse(&self) {
        self.value.modify_bits(|r| r | Self::RTSE_MASK);
    }

    /// Clear RTSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rtse(&self) {
        self.value.modify_bits(|r| r & !Self::RTSE_MASK);
    }

    /// **DMAT** `[7]` (RW)
    ///
    /// DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> bool {
        (self.value.read_bits() & Self::DMAT_MASK) != 0
    }

    /// Set DMAT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmat(&self) {
        self.value.modify_bits(|r| r | Self::DMAT_MASK);
    }

    /// Clear DMAT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmat(&self) {
        self.value.modify_bits(|r| r & !Self::DMAT_MASK);
    }

    /// **DMAR** `[6]` (RW)
    ///
    /// DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> bool {
        (self.value.read_bits() & Self::DMAR_MASK) != 0
    }

    /// Set DMAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmar(&self) {
        self.value.modify_bits(|r| r | Self::DMAR_MASK);
    }

    /// Clear DMAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmar(&self) {
        self.value.modify_bits(|r| r & !Self::DMAR_MASK);
    }

    /// **SCEN** `[5]` (RW)
    ///
    /// Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> bool {
        (self.value.read_bits() & Self::SCEN_MASK) != 0
    }

    /// Set SCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scen(&self) {
        self.value.modify_bits(|r| r | Self::SCEN_MASK);
    }

    /// Clear SCEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scen(&self) {
        self.value.modify_bits(|r| r & !Self::SCEN_MASK);
    }

    /// **NACK** `[4]` (RW)
    ///
    /// Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> bool {
        (self.value.read_bits() & Self::NACK_MASK) != 0
    }

    /// Set NACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nack(&self) {
        self.value.modify_bits(|r| r | Self::NACK_MASK);
    }

    /// Clear NACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nack(&self) {
        self.value.modify_bits(|r| r & !Self::NACK_MASK);
    }

    /// **HDSEL** `[3]` (RW)
    ///
    /// Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> bool {
        (self.value.read_bits() & Self::HDSEL_MASK) != 0
    }

    /// Set HDSEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hdsel(&self) {
        self.value.modify_bits(|r| r | Self::HDSEL_MASK);
    }

    /// Clear HDSEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hdsel(&self) {
        self.value.modify_bits(|r| r & !Self::HDSEL_MASK);
    }

    /// **IRLP** `[2]` (RW)
    ///
    /// IrDA low-power
    #[inline(always)]
    pub fn irlp(&self) -> bool {
        (self.value.read_bits() & Self::IRLP_MASK) != 0
    }

    /// Set IRLP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_irlp(&self) {
        self.value.modify_bits(|r| r | Self::IRLP_MASK);
    }

    /// Clear IRLP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_irlp(&self) {
        self.value.modify_bits(|r| r & !Self::IRLP_MASK);
    }

    /// **IREN** `[1]` (RW)
    ///
    /// IrDA mode enable
    #[inline(always)]
    pub fn iren(&self) -> bool {
        (self.value.read_bits() & Self::IREN_MASK) != 0
    }

    /// Set IREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iren(&self) {
        self.value.modify_bits(|r| r | Self::IREN_MASK);
    }

    /// Clear IREN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iren(&self) {
        self.value.modify_bits(|r| r & !Self::IREN_MASK);
    }

    /// **EIE** `[0]` (RW)
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> bool {
        (self.value.read_bits() & Self::EIE_MASK) != 0
    }

    /// Set EIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eie(&self) {
        self.value.modify_bits(|r| r | Self::EIE_MASK);
    }

    /// Clear EIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eie(&self) {
        self.value.modify_bits(|r| r & !Self::EIE_MASK);
    }
}

/// CTLR3 value type for batch read/modify/write operations
///
/// Use `ctlr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctlr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ctlr3Val {
    bits: u32,
}

impl Ctlr3Val {
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

    /// Read **CTSIE** `[10:10]`
    ///
    /// CTS interrupt enable
    #[inline(always)]
    pub const fn ctsie(&self) -> bool {
        (self.bits & Ctlr3::CTSIE_MASK) != 0
    }

    /// Read **CTSE** `[9:9]`
    ///
    /// CTS enable
    #[inline(always)]
    pub const fn ctse(&self) -> bool {
        (self.bits & Ctlr3::CTSE_MASK) != 0
    }

    /// Read **RTSE** `[8:8]`
    ///
    /// RTS enable
    #[inline(always)]
    pub const fn rtse(&self) -> bool {
        (self.bits & Ctlr3::RTSE_MASK) != 0
    }

    /// Read **DMAT** `[7:7]`
    ///
    /// DMA enable transmitter
    #[inline(always)]
    pub const fn dmat(&self) -> bool {
        (self.bits & Ctlr3::DMAT_MASK) != 0
    }

    /// Read **DMAR** `[6:6]`
    ///
    /// DMA enable receiver
    #[inline(always)]
    pub const fn dmar(&self) -> bool {
        (self.bits & Ctlr3::DMAR_MASK) != 0
    }

    /// Read **SCEN** `[5:5]`
    ///
    /// Smartcard mode enable
    #[inline(always)]
    pub const fn scen(&self) -> bool {
        (self.bits & Ctlr3::SCEN_MASK) != 0
    }

    /// Read **NACK** `[4:4]`
    ///
    /// Smartcard NACK enable
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        (self.bits & Ctlr3::NACK_MASK) != 0
    }

    /// Read **HDSEL** `[3:3]`
    ///
    /// Half-duplex selection
    #[inline(always)]
    pub const fn hdsel(&self) -> bool {
        (self.bits & Ctlr3::HDSEL_MASK) != 0
    }

    /// Read **IRLP** `[2:2]`
    ///
    /// IrDA low-power
    #[inline(always)]
    pub const fn irlp(&self) -> bool {
        (self.bits & Ctlr3::IRLP_MASK) != 0
    }

    /// Read **IREN** `[1:1]`
    ///
    /// IrDA mode enable
    #[inline(always)]
    pub const fn iren(&self) -> bool {
        (self.bits & Ctlr3::IREN_MASK) != 0
    }

    /// Read **EIE** `[0:0]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn eie(&self) -> bool {
        (self.bits & Ctlr3::EIE_MASK) != 0
    }

    /// Set **CTSIE** `[10:10]`
    ///
    /// CTS interrupt enable
    #[inline(always)]
    pub const fn with_ctsie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::CTSIE_MASK
            } else {
                self.bits & !Ctlr3::CTSIE_MASK
            },
        }
    }

    /// Set **CTSE** `[9:9]`
    ///
    /// CTS enable
    #[inline(always)]
    pub const fn with_ctse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::CTSE_MASK
            } else {
                self.bits & !Ctlr3::CTSE_MASK
            },
        }
    }

    /// Set **RTSE** `[8:8]`
    ///
    /// RTS enable
    #[inline(always)]
    pub const fn with_rtse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::RTSE_MASK
            } else {
                self.bits & !Ctlr3::RTSE_MASK
            },
        }
    }

    /// Set **DMAT** `[7:7]`
    ///
    /// DMA enable transmitter
    #[inline(always)]
    pub const fn with_dmat(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::DMAT_MASK
            } else {
                self.bits & !Ctlr3::DMAT_MASK
            },
        }
    }

    /// Set **DMAR** `[6:6]`
    ///
    /// DMA enable receiver
    #[inline(always)]
    pub const fn with_dmar(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::DMAR_MASK
            } else {
                self.bits & !Ctlr3::DMAR_MASK
            },
        }
    }

    /// Set **SCEN** `[5:5]`
    ///
    /// Smartcard mode enable
    #[inline(always)]
    pub const fn with_scen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::SCEN_MASK
            } else {
                self.bits & !Ctlr3::SCEN_MASK
            },
        }
    }

    /// Set **NACK** `[4:4]`
    ///
    /// Smartcard NACK enable
    #[inline(always)]
    pub const fn with_nack(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::NACK_MASK
            } else {
                self.bits & !Ctlr3::NACK_MASK
            },
        }
    }

    /// Set **HDSEL** `[3:3]`
    ///
    /// Half-duplex selection
    #[inline(always)]
    pub const fn with_hdsel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::HDSEL_MASK
            } else {
                self.bits & !Ctlr3::HDSEL_MASK
            },
        }
    }

    /// Set **IRLP** `[2:2]`
    ///
    /// IrDA low-power
    #[inline(always)]
    pub const fn with_irlp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::IRLP_MASK
            } else {
                self.bits & !Ctlr3::IRLP_MASK
            },
        }
    }

    /// Set **IREN** `[1:1]`
    ///
    /// IrDA mode enable
    #[inline(always)]
    pub const fn with_iren(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::IREN_MASK
            } else {
                self.bits & !Ctlr3::IREN_MASK
            },
        }
    }

    /// Set **EIE** `[0:0]`
    ///
    /// Error interrupt enable
    #[inline(always)]
    pub const fn with_eie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctlr3::EIE_MASK
            } else {
                self.bits & !Ctlr3::EIE_MASK
            },
        }
    }
}

impl Default for Ctlr3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctlr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ctlr3Val {
        Ctlr3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ctlr3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ctlr3Val) -> Ctlr3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPR Register
///
/// Guard time and prescaler
/// register
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GT** `[15:8]` (RW)
///
///   Guard time value
///
/// - **PSC** `[7:0]` (RW)
///
///   Prescaler value
#[repr(transparent)]
pub struct Gpr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Gpr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpr {
    // ======== Field Constants ========

    /// **GT**
    ///
    /// Guard time value
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const GT_OFFSET: u32 = 8;
    pub const GT_MASK: u32 = 0x0000ff00;

    /// **PSC**
    ///
    /// Prescaler value
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PSC_OFFSET: u32 = 0;
    pub const PSC_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **GT** `[15:8]` (RW)
    ///
    /// Guard time value
    #[inline(always)]
    pub fn gt(&self) -> u32 {
        (self.value.read_bits() & Self::GT_MASK) >> Self::GT_OFFSET
    }

    /// Write **GT** `[15:8]` (RW)
    ///
    /// Guard time value
    #[inline(always)]
    pub fn write_gt(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::GT_MASK) | ((value << Self::GT_OFFSET) & Self::GT_MASK));
    }

    /// **PSC** `[7:0]` (RW)
    ///
    /// Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> u32 {
        (self.value.read_bits() & Self::PSC_MASK) >> Self::PSC_OFFSET
    }

    /// Write **PSC** `[7:0]` (RW)
    ///
    /// Prescaler value
    #[inline(always)]
    pub fn write_psc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PSC_MASK) | ((value << Self::PSC_OFFSET) & Self::PSC_MASK)
        });
    }
}

/// GPR value type for batch read/modify/write operations
///
/// Use `gpr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GprVal {
    bits: u32,
}

impl GprVal {
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

    /// Read **GT** `[15:8]`
    ///
    /// Guard time value
    #[inline(always)]
    pub const fn gt(&self) -> u32 {
        (self.bits & Gpr::GT_MASK) >> Gpr::GT_OFFSET
    }

    /// Read **PSC** `[7:0]`
    ///
    /// Prescaler value
    #[inline(always)]
    pub const fn psc(&self) -> u32 {
        (self.bits & Gpr::PSC_MASK) >> Gpr::PSC_OFFSET
    }

    /// Set **GT** `[15:8]`
    ///
    /// Guard time value
    #[inline(always)]
    pub const fn with_gt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpr::GT_MASK) | ((value << Gpr::GT_OFFSET) & Gpr::GT_MASK),
        }
    }

    /// Set **PSC** `[7:0]`
    ///
    /// Prescaler value
    #[inline(always)]
    pub const fn with_psc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpr::PSC_MASK) | ((value << Gpr::PSC_OFFSET) & Gpr::PSC_MASK),
        }
    }
}

impl Default for GprVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GprVal {
        GprVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GprVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GprVal) -> GprVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Status register
    pub statr: Statr,
    /// 0x0004
    /// Data register
    pub datar: Datar,
    /// 0x0008
    /// Baud rate register
    pub brr: Brr,
    /// 0x000c
    /// Control register 1
    pub ctlr1: Ctlr1,
    /// 0x0010
    /// Control register 2
    pub ctlr2: Ctlr2,
    /// 0x0014
    /// Control register 3
    pub ctlr3: Ctlr3,
    /// 0x0018
    /// Guard time and prescaler
    /// register
    pub gpr: Gpr,
}
