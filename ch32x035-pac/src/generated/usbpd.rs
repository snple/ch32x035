//! USBPD
//! USBPD configuration

#![allow(dead_code)]

pub const BASE: u32 = 0x40027000;

/// CONFIG Register
///
/// PD interrupt enable register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IE_TX_END** `[15]` (RW)
///
///   transfer complete register
///
/// - **IE_RX_RESET** `[14]` (RW)
///
///   Receive complete rst register
///
/// - **IE_RX_ACT** `[13]` (RW)
///
///   Receive complete register
///
/// - **IE_RX_BYTE** `[12]` (RW)
///
///   Receive byte register
///
/// - **IE_RX_BIT** `[11]` (RW)
///
///   bit interrupt Enable
///
/// - **IE_PD_IO** `[10]` (RW)
///
///   IO Enable
///
/// - **WAKE_POLAR** `[5]` (RW)
///
///   wakeup polarity
///
/// - **PD_RST_EN** `[4]` (RW)
///
///   PD RST Enable
///
/// - **PD_DMA_EN** `[3]` (RW)
///
///   PD DMA Enable
///
/// - **CC_SEL** `[2]` (RW)
///
///   PD Commutation port
///
/// - **PD_ALL_CLR** `[1]` (RW)
///
///   PD ITClear
#[repr(transparent)]
pub struct Config {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config {
    // ======== Field Constants ========

    /// **IE_TX_END**
    ///
    /// transfer complete register
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const IE_TX_END_OFFSET: u32 = 15;
    pub const IE_TX_END_MASK: u32 = 0x00008000;
    pub const IE_TX_END: u32 = 1 << 15;

    /// **IE_RX_RESET**
    ///
    /// Receive complete rst register
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IE_RX_RESET_OFFSET: u32 = 14;
    pub const IE_RX_RESET_MASK: u32 = 0x00004000;
    pub const IE_RX_RESET: u32 = 1 << 14;

    /// **IE_RX_ACT**
    ///
    /// Receive complete register
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IE_RX_ACT_OFFSET: u32 = 13;
    pub const IE_RX_ACT_MASK: u32 = 0x00002000;
    pub const IE_RX_ACT: u32 = 1 << 13;

    /// **IE_RX_BYTE**
    ///
    /// Receive byte register
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IE_RX_BYTE_OFFSET: u32 = 12;
    pub const IE_RX_BYTE_MASK: u32 = 0x00001000;
    pub const IE_RX_BYTE: u32 = 1 << 12;

    /// **IE_RX_BIT**
    ///
    /// bit interrupt Enable
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IE_RX_BIT_OFFSET: u32 = 11;
    pub const IE_RX_BIT_MASK: u32 = 0x00000800;
    pub const IE_RX_BIT: u32 = 1 << 11;

    /// **IE_PD_IO**
    ///
    /// IO Enable
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const IE_PD_IO_OFFSET: u32 = 10;
    pub const IE_PD_IO_MASK: u32 = 0x00000400;
    pub const IE_PD_IO: u32 = 1 << 10;

    /// **WAKE_POLAR**
    ///
    /// wakeup polarity
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const WAKE_POLAR_OFFSET: u32 = 5;
    pub const WAKE_POLAR_MASK: u32 = 0x00000020;
    pub const WAKE_POLAR: u32 = 1 << 5;

    /// **PD_RST_EN**
    ///
    /// PD RST Enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PD_RST_EN_OFFSET: u32 = 4;
    pub const PD_RST_EN_MASK: u32 = 0x00000010;
    pub const PD_RST_EN: u32 = 1 << 4;

    /// **PD_DMA_EN**
    ///
    /// PD DMA Enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PD_DMA_EN_OFFSET: u32 = 3;
    pub const PD_DMA_EN_MASK: u32 = 0x00000008;
    pub const PD_DMA_EN: u32 = 1 << 3;

    /// **CC_SEL**
    ///
    /// PD Commutation port
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CC_SEL_OFFSET: u32 = 2;
    pub const CC_SEL_MASK: u32 = 0x00000004;
    pub const CC_SEL: u32 = 1 << 2;

    /// **PD_ALL_CLR**
    ///
    /// PD ITClear
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PD_ALL_CLR_OFFSET: u32 = 1;
    pub const PD_ALL_CLR_MASK: u32 = 0x00000002;
    pub const PD_ALL_CLR: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **IE_TX_END** `[15]` (RW)
    ///
    /// transfer complete register
    #[inline(always)]
    pub fn ie_tx_end(&self) -> bool {
        (self.value.read_bits() & Self::IE_TX_END_MASK) != 0
    }

    /// Set IE_TX_END bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_tx_end(&self) {
        self.value.modify_bits(|r| r | Self::IE_TX_END_MASK);
    }

    /// Clear IE_TX_END bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_tx_end(&self) {
        self.value.modify_bits(|r| r & !Self::IE_TX_END_MASK);
    }

    /// **IE_RX_RESET** `[14]` (RW)
    ///
    /// Receive complete rst register
    #[inline(always)]
    pub fn ie_rx_reset(&self) -> bool {
        (self.value.read_bits() & Self::IE_RX_RESET_MASK) != 0
    }

    /// Set IE_RX_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_rx_reset(&self) {
        self.value.modify_bits(|r| r | Self::IE_RX_RESET_MASK);
    }

    /// Clear IE_RX_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_rx_reset(&self) {
        self.value.modify_bits(|r| r & !Self::IE_RX_RESET_MASK);
    }

    /// **IE_RX_ACT** `[13]` (RW)
    ///
    /// Receive complete register
    #[inline(always)]
    pub fn ie_rx_act(&self) -> bool {
        (self.value.read_bits() & Self::IE_RX_ACT_MASK) != 0
    }

    /// Set IE_RX_ACT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_rx_act(&self) {
        self.value.modify_bits(|r| r | Self::IE_RX_ACT_MASK);
    }

    /// Clear IE_RX_ACT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_rx_act(&self) {
        self.value.modify_bits(|r| r & !Self::IE_RX_ACT_MASK);
    }

    /// **IE_RX_BYTE** `[12]` (RW)
    ///
    /// Receive byte register
    #[inline(always)]
    pub fn ie_rx_byte(&self) -> bool {
        (self.value.read_bits() & Self::IE_RX_BYTE_MASK) != 0
    }

    /// Set IE_RX_BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_rx_byte(&self) {
        self.value.modify_bits(|r| r | Self::IE_RX_BYTE_MASK);
    }

    /// Clear IE_RX_BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_rx_byte(&self) {
        self.value.modify_bits(|r| r & !Self::IE_RX_BYTE_MASK);
    }

    /// **IE_RX_BIT** `[11]` (RW)
    ///
    /// bit interrupt Enable
    #[inline(always)]
    pub fn ie_rx_bit(&self) -> bool {
        (self.value.read_bits() & Self::IE_RX_BIT_MASK) != 0
    }

    /// Set IE_RX_BIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_rx_bit(&self) {
        self.value.modify_bits(|r| r | Self::IE_RX_BIT_MASK);
    }

    /// Clear IE_RX_BIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_rx_bit(&self) {
        self.value.modify_bits(|r| r & !Self::IE_RX_BIT_MASK);
    }

    /// **IE_PD_IO** `[10]` (RW)
    ///
    /// IO Enable
    #[inline(always)]
    pub fn ie_pd_io(&self) -> bool {
        (self.value.read_bits() & Self::IE_PD_IO_MASK) != 0
    }

    /// Set IE_PD_IO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ie_pd_io(&self) {
        self.value.modify_bits(|r| r | Self::IE_PD_IO_MASK);
    }

    /// Clear IE_PD_IO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ie_pd_io(&self) {
        self.value.modify_bits(|r| r & !Self::IE_PD_IO_MASK);
    }

    /// **WAKE_POLAR** `[5]` (RW)
    ///
    /// wakeup polarity
    #[inline(always)]
    pub fn wake_polar(&self) -> bool {
        (self.value.read_bits() & Self::WAKE_POLAR_MASK) != 0
    }

    /// Set WAKE_POLAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wake_polar(&self) {
        self.value.modify_bits(|r| r | Self::WAKE_POLAR_MASK);
    }

    /// Clear WAKE_POLAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wake_polar(&self) {
        self.value.modify_bits(|r| r & !Self::WAKE_POLAR_MASK);
    }

    /// **PD_RST_EN** `[4]` (RW)
    ///
    /// PD RST Enable
    #[inline(always)]
    pub fn pd_rst_en(&self) -> bool {
        (self.value.read_bits() & Self::PD_RST_EN_MASK) != 0
    }

    /// Set PD_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pd_rst_en(&self) {
        self.value.modify_bits(|r| r | Self::PD_RST_EN_MASK);
    }

    /// Clear PD_RST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pd_rst_en(&self) {
        self.value.modify_bits(|r| r & !Self::PD_RST_EN_MASK);
    }

    /// **PD_DMA_EN** `[3]` (RW)
    ///
    /// PD DMA Enable
    #[inline(always)]
    pub fn pd_dma_en(&self) -> bool {
        (self.value.read_bits() & Self::PD_DMA_EN_MASK) != 0
    }

    /// Set PD_DMA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pd_dma_en(&self) {
        self.value.modify_bits(|r| r | Self::PD_DMA_EN_MASK);
    }

    /// Clear PD_DMA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pd_dma_en(&self) {
        self.value.modify_bits(|r| r & !Self::PD_DMA_EN_MASK);
    }

    /// **CC_SEL** `[2]` (RW)
    ///
    /// PD Commutation port
    #[inline(always)]
    pub fn cc_sel(&self) -> bool {
        (self.value.read_bits() & Self::CC_SEL_MASK) != 0
    }

    /// Set CC_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc_sel(&self) {
        self.value.modify_bits(|r| r | Self::CC_SEL_MASK);
    }

    /// Clear CC_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc_sel(&self) {
        self.value.modify_bits(|r| r & !Self::CC_SEL_MASK);
    }

    /// **PD_ALL_CLR** `[1]` (RW)
    ///
    /// PD ITClear
    #[inline(always)]
    pub fn pd_all_clr(&self) -> bool {
        (self.value.read_bits() & Self::PD_ALL_CLR_MASK) != 0
    }

    /// Set PD_ALL_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pd_all_clr(&self) {
        self.value.modify_bits(|r| r | Self::PD_ALL_CLR_MASK);
    }

    /// Clear PD_ALL_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pd_all_clr(&self) {
        self.value.modify_bits(|r| r & !Self::PD_ALL_CLR_MASK);
    }
}

/// CONFIG value type for batch read/modify/write operations
///
/// Use `config.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfigVal {
    bits: u32,
}

impl ConfigVal {
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

    /// Read **IE_TX_END** `[15:15]`
    ///
    /// transfer complete register
    #[inline(always)]
    pub const fn ie_tx_end(&self) -> bool {
        (self.bits & Config::IE_TX_END_MASK) != 0
    }

    /// Read **IE_RX_RESET** `[14:14]`
    ///
    /// Receive complete rst register
    #[inline(always)]
    pub const fn ie_rx_reset(&self) -> bool {
        (self.bits & Config::IE_RX_RESET_MASK) != 0
    }

    /// Read **IE_RX_ACT** `[13:13]`
    ///
    /// Receive complete register
    #[inline(always)]
    pub const fn ie_rx_act(&self) -> bool {
        (self.bits & Config::IE_RX_ACT_MASK) != 0
    }

    /// Read **IE_RX_BYTE** `[12:12]`
    ///
    /// Receive byte register
    #[inline(always)]
    pub const fn ie_rx_byte(&self) -> bool {
        (self.bits & Config::IE_RX_BYTE_MASK) != 0
    }

    /// Read **IE_RX_BIT** `[11:11]`
    ///
    /// bit interrupt Enable
    #[inline(always)]
    pub const fn ie_rx_bit(&self) -> bool {
        (self.bits & Config::IE_RX_BIT_MASK) != 0
    }

    /// Read **IE_PD_IO** `[10:10]`
    ///
    /// IO Enable
    #[inline(always)]
    pub const fn ie_pd_io(&self) -> bool {
        (self.bits & Config::IE_PD_IO_MASK) != 0
    }

    /// Read **WAKE_POLAR** `[5:5]`
    ///
    /// wakeup polarity
    #[inline(always)]
    pub const fn wake_polar(&self) -> bool {
        (self.bits & Config::WAKE_POLAR_MASK) != 0
    }

    /// Read **PD_RST_EN** `[4:4]`
    ///
    /// PD RST Enable
    #[inline(always)]
    pub const fn pd_rst_en(&self) -> bool {
        (self.bits & Config::PD_RST_EN_MASK) != 0
    }

    /// Read **PD_DMA_EN** `[3:3]`
    ///
    /// PD DMA Enable
    #[inline(always)]
    pub const fn pd_dma_en(&self) -> bool {
        (self.bits & Config::PD_DMA_EN_MASK) != 0
    }

    /// Read **CC_SEL** `[2:2]`
    ///
    /// PD Commutation port
    #[inline(always)]
    pub const fn cc_sel(&self) -> bool {
        (self.bits & Config::CC_SEL_MASK) != 0
    }

    /// Read **PD_ALL_CLR** `[1:1]`
    ///
    /// PD ITClear
    #[inline(always)]
    pub const fn pd_all_clr(&self) -> bool {
        (self.bits & Config::PD_ALL_CLR_MASK) != 0
    }

    /// Set **IE_TX_END** `[15:15]`
    ///
    /// transfer complete register
    #[inline(always)]
    pub const fn with_ie_tx_end(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IE_TX_END_MASK
            } else {
                self.bits & !Config::IE_TX_END_MASK
            },
        }
    }

    /// Set **IE_RX_RESET** `[14:14]`
    ///
    /// Receive complete rst register
    #[inline(always)]
    pub const fn with_ie_rx_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IE_RX_RESET_MASK
            } else {
                self.bits & !Config::IE_RX_RESET_MASK
            },
        }
    }

    /// Set **IE_RX_ACT** `[13:13]`
    ///
    /// Receive complete register
    #[inline(always)]
    pub const fn with_ie_rx_act(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IE_RX_ACT_MASK
            } else {
                self.bits & !Config::IE_RX_ACT_MASK
            },
        }
    }

    /// Set **IE_RX_BYTE** `[12:12]`
    ///
    /// Receive byte register
    #[inline(always)]
    pub const fn with_ie_rx_byte(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IE_RX_BYTE_MASK
            } else {
                self.bits & !Config::IE_RX_BYTE_MASK
            },
        }
    }

    /// Set **IE_RX_BIT** `[11:11]`
    ///
    /// bit interrupt Enable
    #[inline(always)]
    pub const fn with_ie_rx_bit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IE_RX_BIT_MASK
            } else {
                self.bits & !Config::IE_RX_BIT_MASK
            },
        }
    }

    /// Set **IE_PD_IO** `[10:10]`
    ///
    /// IO Enable
    #[inline(always)]
    pub const fn with_ie_pd_io(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::IE_PD_IO_MASK
            } else {
                self.bits & !Config::IE_PD_IO_MASK
            },
        }
    }

    /// Set **WAKE_POLAR** `[5:5]`
    ///
    /// wakeup polarity
    #[inline(always)]
    pub const fn with_wake_polar(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::WAKE_POLAR_MASK
            } else {
                self.bits & !Config::WAKE_POLAR_MASK
            },
        }
    }

    /// Set **PD_RST_EN** `[4:4]`
    ///
    /// PD RST Enable
    #[inline(always)]
    pub const fn with_pd_rst_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::PD_RST_EN_MASK
            } else {
                self.bits & !Config::PD_RST_EN_MASK
            },
        }
    }

    /// Set **PD_DMA_EN** `[3:3]`
    ///
    /// PD DMA Enable
    #[inline(always)]
    pub const fn with_pd_dma_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::PD_DMA_EN_MASK
            } else {
                self.bits & !Config::PD_DMA_EN_MASK
            },
        }
    }

    /// Set **CC_SEL** `[2:2]`
    ///
    /// PD Commutation port
    #[inline(always)]
    pub const fn with_cc_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::CC_SEL_MASK
            } else {
                self.bits & !Config::CC_SEL_MASK
            },
        }
    }

    /// Set **PD_ALL_CLR** `[1:1]`
    ///
    /// PD ITClear
    #[inline(always)]
    pub const fn with_pd_all_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::PD_ALL_CLR_MASK
            } else {
                self.bits & !Config::PD_ALL_CLR_MASK
            },
        }
    }
}

impl Default for ConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfigVal {
        ConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfigVal) -> ConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BMC_CLK_CNT Register
///
/// BMC sampling clock counter
///
/// Address offset: 0x0002
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BMC_CLK_CNT** `[8:0]` (RW)
///
///   R/T counter
#[repr(transparent)]
pub struct BmcClkCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BmcClkCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BmcClkCnt {
    // ======== Field Constants ========

    /// **BMC_CLK_CNT**
    ///
    /// R/T counter
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const BMC_CLK_CNT_OFFSET: u32 = 0;
    pub const BMC_CLK_CNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **BMC_CLK_CNT** `[8:0]` (RW)
    ///
    /// R/T counter
    #[inline(always)]
    pub fn bmc_clk_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BMC_CLK_CNT_MASK) >> Self::BMC_CLK_CNT_OFFSET
    }

    /// Write **BMC_CLK_CNT** `[8:0]` (RW)
    ///
    /// R/T counter
    #[inline(always)]
    pub fn write_bmc_clk_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BMC_CLK_CNT_MASK)
                | ((value << Self::BMC_CLK_CNT_OFFSET) & Self::BMC_CLK_CNT_MASK)
        });
    }
}

/// BMC_CLK_CNT value type for batch read/modify/write operations
///
/// Use `bmcclkcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bmcclkcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BmcClkCntVal {
    bits: u32,
}

impl BmcClkCntVal {
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

    /// Read **BMC_CLK_CNT** `[8:0]`
    ///
    /// R/T counter
    #[inline(always)]
    pub const fn bmc_clk_cnt(&self) -> u32 {
        (self.bits & BmcClkCnt::BMC_CLK_CNT_MASK) >> BmcClkCnt::BMC_CLK_CNT_OFFSET
    }

    /// Set **BMC_CLK_CNT** `[8:0]`
    ///
    /// R/T counter
    #[inline(always)]
    pub const fn with_bmc_clk_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BmcClkCnt::BMC_CLK_CNT_MASK)
                | ((value << BmcClkCnt::BMC_CLK_CNT_OFFSET) & BmcClkCnt::BMC_CLK_CNT_MASK),
        }
    }
}

impl Default for BmcClkCntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BmcClkCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BmcClkCntVal {
        BmcClkCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BmcClkCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BmcClkCntVal) -> BmcClkCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONTROL Register
///
/// PD Send and receive enable register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BMC_BYTE_HI** `[7]` (RO)
///
///   BMC_BYTE_HI value
///
/// - **TX_BIT_BACK** `[6]` (RO)
///
///   TX_BIT_BACK value
///
/// - **DATA_FLAG** `[5]` (RO)
///
///   DATA_FLAG value
///
/// - **BMC_START** `[1]` (RW)
///
///   BMC_START value
///
/// - **PD_TX_EN** `[0]` (RW)
///
///   PD_TX_EN value
#[repr(transparent)]
pub struct Control {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Control {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Control {
    // ======== Field Constants ========

    /// **BMC_BYTE_HI**
    ///
    /// BMC_BYTE_HI value
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BMC_BYTE_HI_OFFSET: u32 = 7;
    pub const BMC_BYTE_HI_MASK: u32 = 0x00000080;
    pub const BMC_BYTE_HI: u32 = 1 << 7;

    /// **TX_BIT_BACK**
    ///
    /// TX_BIT_BACK value
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_BIT_BACK_OFFSET: u32 = 6;
    pub const TX_BIT_BACK_MASK: u32 = 0x00000040;
    pub const TX_BIT_BACK: u32 = 1 << 6;

    /// **DATA_FLAG**
    ///
    /// DATA_FLAG value
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DATA_FLAG_OFFSET: u32 = 5;
    pub const DATA_FLAG_MASK: u32 = 0x00000020;
    pub const DATA_FLAG: u32 = 1 << 5;

    /// **BMC_START**
    ///
    /// BMC_START value
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BMC_START_OFFSET: u32 = 1;
    pub const BMC_START_MASK: u32 = 0x00000002;
    pub const BMC_START: u32 = 1 << 1;

    /// **PD_TX_EN**
    ///
    /// PD_TX_EN value
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PD_TX_EN_OFFSET: u32 = 0;
    pub const PD_TX_EN_MASK: u32 = 0x00000001;
    pub const PD_TX_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BMC_BYTE_HI** `[7]` (RO)
    ///
    /// BMC_BYTE_HI value
    #[inline(always)]
    pub fn bmc_byte_hi(&self) -> bool {
        (self.value.read_bits() & Self::BMC_BYTE_HI_MASK) != 0
    }

    /// **TX_BIT_BACK** `[6]` (RO)
    ///
    /// TX_BIT_BACK value
    #[inline(always)]
    pub fn tx_bit_back(&self) -> bool {
        (self.value.read_bits() & Self::TX_BIT_BACK_MASK) != 0
    }

    /// **DATA_FLAG** `[5]` (RO)
    ///
    /// DATA_FLAG value
    #[inline(always)]
    pub fn data_flag(&self) -> bool {
        (self.value.read_bits() & Self::DATA_FLAG_MASK) != 0
    }

    /// **BMC_START** `[1]` (RW)
    ///
    /// BMC_START value
    #[inline(always)]
    pub fn bmc_start(&self) -> bool {
        (self.value.read_bits() & Self::BMC_START_MASK) != 0
    }

    /// Set BMC_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bmc_start(&self) {
        self.value.modify_bits(|r| r | Self::BMC_START_MASK);
    }

    /// Clear BMC_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bmc_start(&self) {
        self.value.modify_bits(|r| r & !Self::BMC_START_MASK);
    }

    /// **PD_TX_EN** `[0]` (RW)
    ///
    /// PD_TX_EN value
    #[inline(always)]
    pub fn pd_tx_en(&self) -> bool {
        (self.value.read_bits() & Self::PD_TX_EN_MASK) != 0
    }

    /// Set PD_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pd_tx_en(&self) {
        self.value.modify_bits(|r| r | Self::PD_TX_EN_MASK);
    }

    /// Clear PD_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pd_tx_en(&self) {
        self.value.modify_bits(|r| r & !Self::PD_TX_EN_MASK);
    }
}

/// CONTROL value type for batch read/modify/write operations
///
/// Use `control.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `control.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ControlVal {
    bits: u32,
}

impl ControlVal {
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

    /// Read **BMC_BYTE_HI** `[7:7]`
    ///
    /// BMC_BYTE_HI value
    #[inline(always)]
    pub const fn bmc_byte_hi(&self) -> bool {
        (self.bits & Control::BMC_BYTE_HI_MASK) != 0
    }

    /// Read **TX_BIT_BACK** `[6:6]`
    ///
    /// TX_BIT_BACK value
    #[inline(always)]
    pub const fn tx_bit_back(&self) -> bool {
        (self.bits & Control::TX_BIT_BACK_MASK) != 0
    }

    /// Read **DATA_FLAG** `[5:5]`
    ///
    /// DATA_FLAG value
    #[inline(always)]
    pub const fn data_flag(&self) -> bool {
        (self.bits & Control::DATA_FLAG_MASK) != 0
    }

    /// Read **BMC_START** `[1:1]`
    ///
    /// BMC_START value
    #[inline(always)]
    pub const fn bmc_start(&self) -> bool {
        (self.bits & Control::BMC_START_MASK) != 0
    }

    /// Read **PD_TX_EN** `[0:0]`
    ///
    /// PD_TX_EN value
    #[inline(always)]
    pub const fn pd_tx_en(&self) -> bool {
        (self.bits & Control::PD_TX_EN_MASK) != 0
    }

    /// Set **BMC_START** `[1:1]`
    ///
    /// BMC_START value
    #[inline(always)]
    pub const fn with_bmc_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Control::BMC_START_MASK
            } else {
                self.bits & !Control::BMC_START_MASK
            },
        }
    }

    /// Set **PD_TX_EN** `[0:0]`
    ///
    /// PD_TX_EN value
    #[inline(always)]
    pub const fn with_pd_tx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Control::PD_TX_EN_MASK
            } else {
                self.bits & !Control::PD_TX_EN_MASK
            },
        }
    }
}

impl Default for ControlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Control {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ControlVal {
        ControlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ControlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ControlVal) -> ControlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TX_SEL Register
///
/// SOP port selection register
///
/// Address offset: 0x0005
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TX_SEL4** `[7:6]` (RW)
///
///   TX_SEL4 value
///
/// - **TX_SEL3** `[5:4]` (RW)
///
///   TX_SEL3 value
///
/// - **TX_SEL2** `[3:2]` (RW)
///
///   TX_SEL2 value
///
/// - **TX_SEL1** `[0]` (RW)
///
///   TX_SEL1 value
#[repr(transparent)]
pub struct TxSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TxSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TxSel {
    // ======== Field Constants ========

    /// **TX_SEL4**
    ///
    /// TX_SEL4 value
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const TX_SEL4_OFFSET: u32 = 6;
    pub const TX_SEL4_MASK: u32 = 0x000000c0;

    /// **TX_SEL3**
    ///
    /// TX_SEL3 value
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const TX_SEL3_OFFSET: u32 = 4;
    pub const TX_SEL3_MASK: u32 = 0x00000030;

    /// **TX_SEL2**
    ///
    /// TX_SEL2 value
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const TX_SEL2_OFFSET: u32 = 2;
    pub const TX_SEL2_MASK: u32 = 0x0000000c;

    /// **TX_SEL1**
    ///
    /// TX_SEL1 value
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_SEL1_OFFSET: u32 = 0;
    pub const TX_SEL1_MASK: u32 = 0x00000001;
    pub const TX_SEL1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TX_SEL4** `[7:6]` (RW)
    ///
    /// TX_SEL4 value
    #[inline(always)]
    pub fn tx_sel4(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SEL4_MASK) >> Self::TX_SEL4_OFFSET
    }

    /// Write **TX_SEL4** `[7:6]` (RW)
    ///
    /// TX_SEL4 value
    #[inline(always)]
    pub fn write_tx_sel4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_SEL4_MASK) | ((value << Self::TX_SEL4_OFFSET) & Self::TX_SEL4_MASK)
        });
    }

    /// **TX_SEL3** `[5:4]` (RW)
    ///
    /// TX_SEL3 value
    #[inline(always)]
    pub fn tx_sel3(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SEL3_MASK) >> Self::TX_SEL3_OFFSET
    }

    /// Write **TX_SEL3** `[5:4]` (RW)
    ///
    /// TX_SEL3 value
    #[inline(always)]
    pub fn write_tx_sel3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_SEL3_MASK) | ((value << Self::TX_SEL3_OFFSET) & Self::TX_SEL3_MASK)
        });
    }

    /// **TX_SEL2** `[3:2]` (RW)
    ///
    /// TX_SEL2 value
    #[inline(always)]
    pub fn tx_sel2(&self) -> u32 {
        (self.value.read_bits() & Self::TX_SEL2_MASK) >> Self::TX_SEL2_OFFSET
    }

    /// Write **TX_SEL2** `[3:2]` (RW)
    ///
    /// TX_SEL2 value
    #[inline(always)]
    pub fn write_tx_sel2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_SEL2_MASK) | ((value << Self::TX_SEL2_OFFSET) & Self::TX_SEL2_MASK)
        });
    }

    /// **TX_SEL1** `[0]` (RW)
    ///
    /// TX_SEL1 value
    #[inline(always)]
    pub fn tx_sel1(&self) -> bool {
        (self.value.read_bits() & Self::TX_SEL1_MASK) != 0
    }

    /// Set TX_SEL1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_sel1(&self) {
        self.value.modify_bits(|r| r | Self::TX_SEL1_MASK);
    }

    /// Clear TX_SEL1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_sel1(&self) {
        self.value.modify_bits(|r| r & !Self::TX_SEL1_MASK);
    }
}

/// TX_SEL value type for batch read/modify/write operations
///
/// Use `txsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TxSelVal {
    bits: u32,
}

impl TxSelVal {
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

    /// Read **TX_SEL4** `[7:6]`
    ///
    /// TX_SEL4 value
    #[inline(always)]
    pub const fn tx_sel4(&self) -> u32 {
        (self.bits & TxSel::TX_SEL4_MASK) >> TxSel::TX_SEL4_OFFSET
    }

    /// Read **TX_SEL3** `[5:4]`
    ///
    /// TX_SEL3 value
    #[inline(always)]
    pub const fn tx_sel3(&self) -> u32 {
        (self.bits & TxSel::TX_SEL3_MASK) >> TxSel::TX_SEL3_OFFSET
    }

    /// Read **TX_SEL2** `[3:2]`
    ///
    /// TX_SEL2 value
    #[inline(always)]
    pub const fn tx_sel2(&self) -> u32 {
        (self.bits & TxSel::TX_SEL2_MASK) >> TxSel::TX_SEL2_OFFSET
    }

    /// Read **TX_SEL1** `[0:0]`
    ///
    /// TX_SEL1 value
    #[inline(always)]
    pub const fn tx_sel1(&self) -> bool {
        (self.bits & TxSel::TX_SEL1_MASK) != 0
    }

    /// Set **TX_SEL4** `[7:6]`
    ///
    /// TX_SEL4 value
    #[inline(always)]
    pub const fn with_tx_sel4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxSel::TX_SEL4_MASK)
                | ((value << TxSel::TX_SEL4_OFFSET) & TxSel::TX_SEL4_MASK),
        }
    }

    /// Set **TX_SEL3** `[5:4]`
    ///
    /// TX_SEL3 value
    #[inline(always)]
    pub const fn with_tx_sel3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxSel::TX_SEL3_MASK)
                | ((value << TxSel::TX_SEL3_OFFSET) & TxSel::TX_SEL3_MASK),
        }
    }

    /// Set **TX_SEL2** `[3:2]`
    ///
    /// TX_SEL2 value
    #[inline(always)]
    pub const fn with_tx_sel2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TxSel::TX_SEL2_MASK)
                | ((value << TxSel::TX_SEL2_OFFSET) & TxSel::TX_SEL2_MASK),
        }
    }

    /// Set **TX_SEL1** `[0:0]`
    ///
    /// TX_SEL1 value
    #[inline(always)]
    pub const fn with_tx_sel1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TxSel::TX_SEL1_MASK
            } else {
                self.bits & !TxSel::TX_SEL1_MASK
            },
        }
    }
}

impl Default for TxSelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TxSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TxSelVal {
        TxSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TxSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TxSelVal) -> TxSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BMC_TX_SZ Register
///
/// PD send length register
///
/// Address offset: 0x0006
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BMC_TX_SZ** `[8:0]` (RW)
///
///   BMC_TX_SZ value
#[repr(transparent)]
pub struct BmcTxSz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BmcTxSz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BmcTxSz {
    // ======== Field Constants ========

    /// **BMC_TX_SZ**
    ///
    /// BMC_TX_SZ value
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const BMC_TX_SZ_OFFSET: u32 = 0;
    pub const BMC_TX_SZ_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **BMC_TX_SZ** `[8:0]` (RW)
    ///
    /// BMC_TX_SZ value
    #[inline(always)]
    pub fn bmc_tx_sz(&self) -> u32 {
        (self.value.read_bits() & Self::BMC_TX_SZ_MASK) >> Self::BMC_TX_SZ_OFFSET
    }

    /// Write **BMC_TX_SZ** `[8:0]` (RW)
    ///
    /// BMC_TX_SZ value
    #[inline(always)]
    pub fn write_bmc_tx_sz(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BMC_TX_SZ_MASK) | ((value << Self::BMC_TX_SZ_OFFSET) & Self::BMC_TX_SZ_MASK)
        });
    }
}

/// BMC_TX_SZ value type for batch read/modify/write operations
///
/// Use `bmctxsz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bmctxsz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BmcTxSzVal {
    bits: u32,
}

impl BmcTxSzVal {
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

    /// Read **BMC_TX_SZ** `[8:0]`
    ///
    /// BMC_TX_SZ value
    #[inline(always)]
    pub const fn bmc_tx_sz(&self) -> u32 {
        (self.bits & BmcTxSz::BMC_TX_SZ_MASK) >> BmcTxSz::BMC_TX_SZ_OFFSET
    }

    /// Set **BMC_TX_SZ** `[8:0]`
    ///
    /// BMC_TX_SZ value
    #[inline(always)]
    pub const fn with_bmc_tx_sz(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BmcTxSz::BMC_TX_SZ_MASK)
                | ((value << BmcTxSz::BMC_TX_SZ_OFFSET) & BmcTxSz::BMC_TX_SZ_MASK),
        }
    }
}

impl Default for BmcTxSzVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BmcTxSz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BmcTxSzVal {
        BmcTxSzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BmcTxSzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BmcTxSzVal) -> BmcTxSzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_BUF Register
///
/// DMA cache data register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DATA_BUF** `[7:0]` (RO)
///
///   DATA_BUF value
#[repr(transparent)]
pub struct DataBuf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DataBuf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DataBuf {
    // ======== Field Constants ========

    /// **DATA_BUF**
    ///
    /// DATA_BUF value
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DATA_BUF_OFFSET: u32 = 0;
    pub const DATA_BUF_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DATA_BUF** `[7:0]` (RO)
    ///
    /// DATA_BUF value
    #[inline(always)]
    pub fn data_buf(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_BUF_MASK) >> Self::DATA_BUF_OFFSET
    }
}

/// DATA_BUF value type for batch read/modify/write operations
///
/// Use `databuf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `databuf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DataBufVal {
    bits: u32,
}

impl DataBufVal {
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

    /// Read **DATA_BUF** `[7:0]`
    ///
    /// DATA_BUF value
    #[inline(always)]
    pub const fn data_buf(&self) -> u32 {
        (self.bits & DataBuf::DATA_BUF_MASK) >> DataBuf::DATA_BUF_OFFSET
    }
}

impl Default for DataBufVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DataBuf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DataBufVal {
        DataBufVal::from_bits(self.value.read_bits())
    }
}

/// STATUS Register
///
/// PD interrupt flag register
///
/// Address offset: 0x0009
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IF_TX_END** `[7]` (RW)
///
///   IF_TX_END value
///
/// - **IF_RX_RESET** `[6]` (RW)
///
///   IF_RX_RESET value
///
/// - **IF_RX_ACT** `[5]` (RW)
///
///   IF_RX_ACT value
///
/// - **IF_RX_BYTE** `[4]` (RW)
///
///   IF_RX_BYTE value
///
/// - **IF_RX_BIT** `[3]` (RW)
///
///   IF_RX_BIT value
///
/// - **BUF_ERR** `[2]` (RW)
///
///   BUF_ERR value
///
/// - **BMC_AUX** `[1:0]` (RW)
///
///   BMC_AUX value
#[repr(transparent)]
pub struct Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status {
    // ======== Field Constants ========

    /// **IF_TX_END**
    ///
    /// IF_TX_END value
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const IF_TX_END_OFFSET: u32 = 7;
    pub const IF_TX_END_MASK: u32 = 0x00000080;
    pub const IF_TX_END: u32 = 1 << 7;

    /// **IF_RX_RESET**
    ///
    /// IF_RX_RESET value
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IF_RX_RESET_OFFSET: u32 = 6;
    pub const IF_RX_RESET_MASK: u32 = 0x00000040;
    pub const IF_RX_RESET: u32 = 1 << 6;

    /// **IF_RX_ACT**
    ///
    /// IF_RX_ACT value
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IF_RX_ACT_OFFSET: u32 = 5;
    pub const IF_RX_ACT_MASK: u32 = 0x00000020;
    pub const IF_RX_ACT: u32 = 1 << 5;

    /// **IF_RX_BYTE**
    ///
    /// IF_RX_BYTE value
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const IF_RX_BYTE_OFFSET: u32 = 4;
    pub const IF_RX_BYTE_MASK: u32 = 0x00000010;
    pub const IF_RX_BYTE: u32 = 1 << 4;

    /// **IF_RX_BIT**
    ///
    /// IF_RX_BIT value
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const IF_RX_BIT_OFFSET: u32 = 3;
    pub const IF_RX_BIT_MASK: u32 = 0x00000008;
    pub const IF_RX_BIT: u32 = 1 << 3;

    /// **BUF_ERR**
    ///
    /// BUF_ERR value
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BUF_ERR_OFFSET: u32 = 2;
    pub const BUF_ERR_MASK: u32 = 0x00000004;
    pub const BUF_ERR: u32 = 1 << 2;

    /// **BMC_AUX**
    ///
    /// BMC_AUX value
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const BMC_AUX_OFFSET: u32 = 0;
    pub const BMC_AUX_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **IF_TX_END** `[7]` (RW)
    ///
    /// IF_TX_END value
    #[inline(always)]
    pub fn if_tx_end(&self) -> bool {
        (self.value.read_bits() & Self::IF_TX_END_MASK) != 0
    }

    /// Set IF_TX_END bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_tx_end(&self) {
        self.value.modify_bits(|r| r | Self::IF_TX_END_MASK);
    }

    /// Clear IF_TX_END bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_tx_end(&self) {
        self.value.modify_bits(|r| r & !Self::IF_TX_END_MASK);
    }

    /// **IF_RX_RESET** `[6]` (RW)
    ///
    /// IF_RX_RESET value
    #[inline(always)]
    pub fn if_rx_reset(&self) -> bool {
        (self.value.read_bits() & Self::IF_RX_RESET_MASK) != 0
    }

    /// Set IF_RX_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_rx_reset(&self) {
        self.value.modify_bits(|r| r | Self::IF_RX_RESET_MASK);
    }

    /// Clear IF_RX_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_rx_reset(&self) {
        self.value.modify_bits(|r| r & !Self::IF_RX_RESET_MASK);
    }

    /// **IF_RX_ACT** `[5]` (RW)
    ///
    /// IF_RX_ACT value
    #[inline(always)]
    pub fn if_rx_act(&self) -> bool {
        (self.value.read_bits() & Self::IF_RX_ACT_MASK) != 0
    }

    /// Set IF_RX_ACT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_rx_act(&self) {
        self.value.modify_bits(|r| r | Self::IF_RX_ACT_MASK);
    }

    /// Clear IF_RX_ACT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_rx_act(&self) {
        self.value.modify_bits(|r| r & !Self::IF_RX_ACT_MASK);
    }

    /// **IF_RX_BYTE** `[4]` (RW)
    ///
    /// IF_RX_BYTE value
    #[inline(always)]
    pub fn if_rx_byte(&self) -> bool {
        (self.value.read_bits() & Self::IF_RX_BYTE_MASK) != 0
    }

    /// Set IF_RX_BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_rx_byte(&self) {
        self.value.modify_bits(|r| r | Self::IF_RX_BYTE_MASK);
    }

    /// Clear IF_RX_BYTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_rx_byte(&self) {
        self.value.modify_bits(|r| r & !Self::IF_RX_BYTE_MASK);
    }

    /// **IF_RX_BIT** `[3]` (RW)
    ///
    /// IF_RX_BIT value
    #[inline(always)]
    pub fn if_rx_bit(&self) -> bool {
        (self.value.read_bits() & Self::IF_RX_BIT_MASK) != 0
    }

    /// Set IF_RX_BIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_if_rx_bit(&self) {
        self.value.modify_bits(|r| r | Self::IF_RX_BIT_MASK);
    }

    /// Clear IF_RX_BIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_if_rx_bit(&self) {
        self.value.modify_bits(|r| r & !Self::IF_RX_BIT_MASK);
    }

    /// **BUF_ERR** `[2]` (RW)
    ///
    /// BUF_ERR value
    #[inline(always)]
    pub fn buf_err(&self) -> bool {
        (self.value.read_bits() & Self::BUF_ERR_MASK) != 0
    }

    /// Set BUF_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_buf_err(&self) {
        self.value.modify_bits(|r| r | Self::BUF_ERR_MASK);
    }

    /// Clear BUF_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_buf_err(&self) {
        self.value.modify_bits(|r| r & !Self::BUF_ERR_MASK);
    }

    /// **BMC_AUX** `[1:0]` (RW)
    ///
    /// BMC_AUX value
    #[inline(always)]
    pub fn bmc_aux(&self) -> u32 {
        (self.value.read_bits() & Self::BMC_AUX_MASK) >> Self::BMC_AUX_OFFSET
    }

    /// Write **BMC_AUX** `[1:0]` (RW)
    ///
    /// BMC_AUX value
    #[inline(always)]
    pub fn write_bmc_aux(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BMC_AUX_MASK) | ((value << Self::BMC_AUX_OFFSET) & Self::BMC_AUX_MASK)
        });
    }
}

/// STATUS value type for batch read/modify/write operations
///
/// Use `status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusVal {
    bits: u32,
}

impl StatusVal {
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

    /// Read **IF_TX_END** `[7:7]`
    ///
    /// IF_TX_END value
    #[inline(always)]
    pub const fn if_tx_end(&self) -> bool {
        (self.bits & Status::IF_TX_END_MASK) != 0
    }

    /// Read **IF_RX_RESET** `[6:6]`
    ///
    /// IF_RX_RESET value
    #[inline(always)]
    pub const fn if_rx_reset(&self) -> bool {
        (self.bits & Status::IF_RX_RESET_MASK) != 0
    }

    /// Read **IF_RX_ACT** `[5:5]`
    ///
    /// IF_RX_ACT value
    #[inline(always)]
    pub const fn if_rx_act(&self) -> bool {
        (self.bits & Status::IF_RX_ACT_MASK) != 0
    }

    /// Read **IF_RX_BYTE** `[4:4]`
    ///
    /// IF_RX_BYTE value
    #[inline(always)]
    pub const fn if_rx_byte(&self) -> bool {
        (self.bits & Status::IF_RX_BYTE_MASK) != 0
    }

    /// Read **IF_RX_BIT** `[3:3]`
    ///
    /// IF_RX_BIT value
    #[inline(always)]
    pub const fn if_rx_bit(&self) -> bool {
        (self.bits & Status::IF_RX_BIT_MASK) != 0
    }

    /// Read **BUF_ERR** `[2:2]`
    ///
    /// BUF_ERR value
    #[inline(always)]
    pub const fn buf_err(&self) -> bool {
        (self.bits & Status::BUF_ERR_MASK) != 0
    }

    /// Read **BMC_AUX** `[1:0]`
    ///
    /// BMC_AUX value
    #[inline(always)]
    pub const fn bmc_aux(&self) -> u32 {
        (self.bits & Status::BMC_AUX_MASK) >> Status::BMC_AUX_OFFSET
    }

    /// Set **IF_TX_END** `[7:7]`
    ///
    /// IF_TX_END value
    #[inline(always)]
    pub const fn with_if_tx_end(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::IF_TX_END_MASK
            } else {
                self.bits & !Status::IF_TX_END_MASK
            },
        }
    }

    /// Set **IF_RX_RESET** `[6:6]`
    ///
    /// IF_RX_RESET value
    #[inline(always)]
    pub const fn with_if_rx_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::IF_RX_RESET_MASK
            } else {
                self.bits & !Status::IF_RX_RESET_MASK
            },
        }
    }

    /// Set **IF_RX_ACT** `[5:5]`
    ///
    /// IF_RX_ACT value
    #[inline(always)]
    pub const fn with_if_rx_act(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::IF_RX_ACT_MASK
            } else {
                self.bits & !Status::IF_RX_ACT_MASK
            },
        }
    }

    /// Set **IF_RX_BYTE** `[4:4]`
    ///
    /// IF_RX_BYTE value
    #[inline(always)]
    pub const fn with_if_rx_byte(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::IF_RX_BYTE_MASK
            } else {
                self.bits & !Status::IF_RX_BYTE_MASK
            },
        }
    }

    /// Set **IF_RX_BIT** `[3:3]`
    ///
    /// IF_RX_BIT value
    #[inline(always)]
    pub const fn with_if_rx_bit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::IF_RX_BIT_MASK
            } else {
                self.bits & !Status::IF_RX_BIT_MASK
            },
        }
    }

    /// Set **BUF_ERR** `[2:2]`
    ///
    /// BUF_ERR value
    #[inline(always)]
    pub const fn with_buf_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::BUF_ERR_MASK
            } else {
                self.bits & !Status::BUF_ERR_MASK
            },
        }
    }

    /// Set **BMC_AUX** `[1:0]`
    ///
    /// BMC_AUX value
    #[inline(always)]
    pub const fn with_bmc_aux(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Status::BMC_AUX_MASK)
                | ((value << Status::BMC_AUX_OFFSET) & Status::BMC_AUX_MASK),
        }
    }
}

impl Default for StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusVal {
        StatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StatusVal) -> StatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BMC_BYTE_CNT Register
///
/// Byte counter
///
/// Address offset: 0x000a
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BMC_BYTE_CNT** `[8:0]` (RO)
///
///   BMC_BYTE_CNT value
#[repr(transparent)]
pub struct BmcByteCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BmcByteCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BmcByteCnt {
    // ======== Field Constants ========

    /// **BMC_BYTE_CNT**
    ///
    /// BMC_BYTE_CNT value
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const BMC_BYTE_CNT_OFFSET: u32 = 0;
    pub const BMC_BYTE_CNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **BMC_BYTE_CNT** `[8:0]` (RO)
    ///
    /// BMC_BYTE_CNT value
    #[inline(always)]
    pub fn bmc_byte_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BMC_BYTE_CNT_MASK) >> Self::BMC_BYTE_CNT_OFFSET
    }
}

/// BMC_BYTE_CNT value type for batch read/modify/write operations
///
/// Use `bmcbytecnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bmcbytecnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BmcByteCntVal {
    bits: u32,
}

impl BmcByteCntVal {
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

    /// Read **BMC_BYTE_CNT** `[8:0]`
    ///
    /// BMC_BYTE_CNT value
    #[inline(always)]
    pub const fn bmc_byte_cnt(&self) -> u32 {
        (self.bits & BmcByteCnt::BMC_BYTE_CNT_MASK) >> BmcByteCnt::BMC_BYTE_CNT_OFFSET
    }
}

impl Default for BmcByteCntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BmcByteCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BmcByteCntVal {
        BmcByteCntVal::from_bits(self.value.read_bits())
    }
}

/// PORT_CC1 Register
///
/// CC1 port control register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CC1_CE** `[7:5]` (RW)
///
///   CC1_CE value
///
/// - **CC1_LV0** `[4]` (RW)
///
///   CC1_LV0 value
///
/// - **CC1_PU** `[3:2]` (RW)
///
///   CC1_PU value
///
/// - **PA_CC1_AI** `[0]` (RO)
///
///   PA_CC1_AI value
#[repr(transparent)]
pub struct PortCc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PortCc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PortCc1 {
    // ======== Field Constants ========

    /// **CC1_CE**
    ///
    /// CC1_CE value
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const CC1_CE_OFFSET: u32 = 5;
    pub const CC1_CE_MASK: u32 = 0x000000e0;

    /// **CC1_LV0**
    ///
    /// CC1_LV0 value
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CC1_LV0_OFFSET: u32 = 4;
    pub const CC1_LV0_MASK: u32 = 0x00000010;
    pub const CC1_LV0: u32 = 1 << 4;

    /// **CC1_PU**
    ///
    /// CC1_PU value
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const CC1_PU_OFFSET: u32 = 2;
    pub const CC1_PU_MASK: u32 = 0x0000000c;

    /// **PA_CC1_AI**
    ///
    /// PA_CC1_AI value
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PA_CC1_AI_OFFSET: u32 = 0;
    pub const PA_CC1_AI_MASK: u32 = 0x00000001;
    pub const PA_CC1_AI: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CC1_CE** `[7:5]` (RW)
    ///
    /// CC1_CE value
    #[inline(always)]
    pub fn cc1_ce(&self) -> u32 {
        (self.value.read_bits() & Self::CC1_CE_MASK) >> Self::CC1_CE_OFFSET
    }

    /// Write **CC1_CE** `[7:5]` (RW)
    ///
    /// CC1_CE value
    #[inline(always)]
    pub fn write_cc1_ce(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC1_CE_MASK) | ((value << Self::CC1_CE_OFFSET) & Self::CC1_CE_MASK)
        });
    }

    /// **CC1_LV0** `[4]` (RW)
    ///
    /// CC1_LV0 value
    #[inline(always)]
    pub fn cc1_lv0(&self) -> bool {
        (self.value.read_bits() & Self::CC1_LV0_MASK) != 0
    }

    /// Set CC1_LV0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc1_lv0(&self) {
        self.value.modify_bits(|r| r | Self::CC1_LV0_MASK);
    }

    /// Clear CC1_LV0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc1_lv0(&self) {
        self.value.modify_bits(|r| r & !Self::CC1_LV0_MASK);
    }

    /// **CC1_PU** `[3:2]` (RW)
    ///
    /// CC1_PU value
    #[inline(always)]
    pub fn cc1_pu(&self) -> u32 {
        (self.value.read_bits() & Self::CC1_PU_MASK) >> Self::CC1_PU_OFFSET
    }

    /// Write **CC1_PU** `[3:2]` (RW)
    ///
    /// CC1_PU value
    #[inline(always)]
    pub fn write_cc1_pu(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC1_PU_MASK) | ((value << Self::CC1_PU_OFFSET) & Self::CC1_PU_MASK)
        });
    }

    /// **PA_CC1_AI** `[0]` (RO)
    ///
    /// PA_CC1_AI value
    #[inline(always)]
    pub fn pa_cc1_ai(&self) -> bool {
        (self.value.read_bits() & Self::PA_CC1_AI_MASK) != 0
    }
}

/// PORT_CC1 value type for batch read/modify/write operations
///
/// Use `portcc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `portcc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PortCc1Val {
    bits: u32,
}

impl PortCc1Val {
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

    /// Read **CC1_CE** `[7:5]`
    ///
    /// CC1_CE value
    #[inline(always)]
    pub const fn cc1_ce(&self) -> u32 {
        (self.bits & PortCc1::CC1_CE_MASK) >> PortCc1::CC1_CE_OFFSET
    }

    /// Read **CC1_LV0** `[4:4]`
    ///
    /// CC1_LV0 value
    #[inline(always)]
    pub const fn cc1_lv0(&self) -> bool {
        (self.bits & PortCc1::CC1_LV0_MASK) != 0
    }

    /// Read **CC1_PU** `[3:2]`
    ///
    /// CC1_PU value
    #[inline(always)]
    pub const fn cc1_pu(&self) -> u32 {
        (self.bits & PortCc1::CC1_PU_MASK) >> PortCc1::CC1_PU_OFFSET
    }

    /// Read **PA_CC1_AI** `[0:0]`
    ///
    /// PA_CC1_AI value
    #[inline(always)]
    pub const fn pa_cc1_ai(&self) -> bool {
        (self.bits & PortCc1::PA_CC1_AI_MASK) != 0
    }

    /// Set **CC1_CE** `[7:5]`
    ///
    /// CC1_CE value
    #[inline(always)]
    pub const fn with_cc1_ce(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PortCc1::CC1_CE_MASK)
                | ((value << PortCc1::CC1_CE_OFFSET) & PortCc1::CC1_CE_MASK),
        }
    }

    /// Set **CC1_LV0** `[4:4]`
    ///
    /// CC1_LV0 value
    #[inline(always)]
    pub const fn with_cc1_lv0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PortCc1::CC1_LV0_MASK
            } else {
                self.bits & !PortCc1::CC1_LV0_MASK
            },
        }
    }

    /// Set **CC1_PU** `[3:2]`
    ///
    /// CC1_PU value
    #[inline(always)]
    pub const fn with_cc1_pu(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PortCc1::CC1_PU_MASK)
                | ((value << PortCc1::CC1_PU_OFFSET) & PortCc1::CC1_PU_MASK),
        }
    }
}

impl Default for PortCc1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PortCc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PortCc1Val {
        PortCc1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PortCc1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PortCc1Val) -> PortCc1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PORT_CC2 Register
///
/// CC2 port control register
///
/// Address offset: 0x000e
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CC2_CE** `[7:5]` (RW)
///
///   CC2_CE value
///
/// - **CC2_LV0** `[4]` (RW)
///
///   CC2_LV0 value
///
/// - **CC2_PU** `[3:2]` (RW)
///
///   CC2_PU value
///
/// - **PA_CC2_AI** `[0]` (RW)
///
///   PA_CC2_AI value
#[repr(transparent)]
pub struct PortCc2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PortCc2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PortCc2 {
    // ======== Field Constants ========

    /// **CC2_CE**
    ///
    /// CC2_CE value
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const CC2_CE_OFFSET: u32 = 5;
    pub const CC2_CE_MASK: u32 = 0x000000e0;

    /// **CC2_LV0**
    ///
    /// CC2_LV0 value
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CC2_LV0_OFFSET: u32 = 4;
    pub const CC2_LV0_MASK: u32 = 0x00000010;
    pub const CC2_LV0: u32 = 1 << 4;

    /// **CC2_PU**
    ///
    /// CC2_PU value
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const CC2_PU_OFFSET: u32 = 2;
    pub const CC2_PU_MASK: u32 = 0x0000000c;

    /// **PA_CC2_AI**
    ///
    /// PA_CC2_AI value
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PA_CC2_AI_OFFSET: u32 = 0;
    pub const PA_CC2_AI_MASK: u32 = 0x00000001;
    pub const PA_CC2_AI: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CC2_CE** `[7:5]` (RW)
    ///
    /// CC2_CE value
    #[inline(always)]
    pub fn cc2_ce(&self) -> u32 {
        (self.value.read_bits() & Self::CC2_CE_MASK) >> Self::CC2_CE_OFFSET
    }

    /// Write **CC2_CE** `[7:5]` (RW)
    ///
    /// CC2_CE value
    #[inline(always)]
    pub fn write_cc2_ce(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC2_CE_MASK) | ((value << Self::CC2_CE_OFFSET) & Self::CC2_CE_MASK)
        });
    }

    /// **CC2_LV0** `[4]` (RW)
    ///
    /// CC2_LV0 value
    #[inline(always)]
    pub fn cc2_lv0(&self) -> bool {
        (self.value.read_bits() & Self::CC2_LV0_MASK) != 0
    }

    /// Set CC2_LV0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cc2_lv0(&self) {
        self.value.modify_bits(|r| r | Self::CC2_LV0_MASK);
    }

    /// Clear CC2_LV0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cc2_lv0(&self) {
        self.value.modify_bits(|r| r & !Self::CC2_LV0_MASK);
    }

    /// **CC2_PU** `[3:2]` (RW)
    ///
    /// CC2_PU value
    #[inline(always)]
    pub fn cc2_pu(&self) -> u32 {
        (self.value.read_bits() & Self::CC2_PU_MASK) >> Self::CC2_PU_OFFSET
    }

    /// Write **CC2_PU** `[3:2]` (RW)
    ///
    /// CC2_PU value
    #[inline(always)]
    pub fn write_cc2_pu(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CC2_PU_MASK) | ((value << Self::CC2_PU_OFFSET) & Self::CC2_PU_MASK)
        });
    }

    /// **PA_CC2_AI** `[0]` (RW)
    ///
    /// PA_CC2_AI value
    #[inline(always)]
    pub fn pa_cc2_ai(&self) -> bool {
        (self.value.read_bits() & Self::PA_CC2_AI_MASK) != 0
    }

    /// Set PA_CC2_AI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pa_cc2_ai(&self) {
        self.value.modify_bits(|r| r | Self::PA_CC2_AI_MASK);
    }

    /// Clear PA_CC2_AI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pa_cc2_ai(&self) {
        self.value.modify_bits(|r| r & !Self::PA_CC2_AI_MASK);
    }
}

/// PORT_CC2 value type for batch read/modify/write operations
///
/// Use `portcc2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `portcc2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PortCc2Val {
    bits: u32,
}

impl PortCc2Val {
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

    /// Read **CC2_CE** `[7:5]`
    ///
    /// CC2_CE value
    #[inline(always)]
    pub const fn cc2_ce(&self) -> u32 {
        (self.bits & PortCc2::CC2_CE_MASK) >> PortCc2::CC2_CE_OFFSET
    }

    /// Read **CC2_LV0** `[4:4]`
    ///
    /// CC2_LV0 value
    #[inline(always)]
    pub const fn cc2_lv0(&self) -> bool {
        (self.bits & PortCc2::CC2_LV0_MASK) != 0
    }

    /// Read **CC2_PU** `[3:2]`
    ///
    /// CC2_PU value
    #[inline(always)]
    pub const fn cc2_pu(&self) -> u32 {
        (self.bits & PortCc2::CC2_PU_MASK) >> PortCc2::CC2_PU_OFFSET
    }

    /// Read **PA_CC2_AI** `[0:0]`
    ///
    /// PA_CC2_AI value
    #[inline(always)]
    pub const fn pa_cc2_ai(&self) -> bool {
        (self.bits & PortCc2::PA_CC2_AI_MASK) != 0
    }

    /// Set **CC2_CE** `[7:5]`
    ///
    /// CC2_CE value
    #[inline(always)]
    pub const fn with_cc2_ce(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PortCc2::CC2_CE_MASK)
                | ((value << PortCc2::CC2_CE_OFFSET) & PortCc2::CC2_CE_MASK),
        }
    }

    /// Set **CC2_LV0** `[4:4]`
    ///
    /// CC2_LV0 value
    #[inline(always)]
    pub const fn with_cc2_lv0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PortCc2::CC2_LV0_MASK
            } else {
                self.bits & !PortCc2::CC2_LV0_MASK
            },
        }
    }

    /// Set **CC2_PU** `[3:2]`
    ///
    /// CC2_PU value
    #[inline(always)]
    pub const fn with_cc2_pu(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PortCc2::CC2_PU_MASK)
                | ((value << PortCc2::CC2_PU_OFFSET) & PortCc2::CC2_PU_MASK),
        }
    }

    /// Set **PA_CC2_AI** `[0:0]`
    ///
    /// PA_CC2_AI value
    #[inline(always)]
    pub const fn with_pa_cc2_ai(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PortCc2::PA_CC2_AI_MASK
            } else {
                self.bits & !PortCc2::PA_CC2_AI_MASK
            },
        }
    }
}

impl Default for PortCc2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PortCc2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PortCc2Val {
        PortCc2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PortCc2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PortCc2Val) -> PortCc2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA Register
///
/// PD buffer start address register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USBPD_DMA_ADDR** `[15:0]` (RW)
///
///   USBPD_DMA_ADDR value
#[repr(transparent)]
pub struct Dma {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dma {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma {
    // ======== Field Constants ========

    /// **USBPD_DMA_ADDR**
    ///
    /// USBPD_DMA_ADDR value
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const USBPD_DMA_ADDR_OFFSET: u32 = 0;
    pub const USBPD_DMA_ADDR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **USBPD_DMA_ADDR** `[15:0]` (RW)
    ///
    /// USBPD_DMA_ADDR value
    #[inline(always)]
    pub fn usbpd_dma_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USBPD_DMA_ADDR_MASK) >> Self::USBPD_DMA_ADDR_OFFSET
    }

    /// Write **USBPD_DMA_ADDR** `[15:0]` (RW)
    ///
    /// USBPD_DMA_ADDR value
    #[inline(always)]
    pub fn write_usbpd_dma_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USBPD_DMA_ADDR_MASK)
                | ((value << Self::USBPD_DMA_ADDR_OFFSET) & Self::USBPD_DMA_ADDR_MASK)
        });
    }
}

/// DMA value type for batch read/modify/write operations
///
/// Use `dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaVal {
    bits: u32,
}

impl DmaVal {
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

    /// Read **USBPD_DMA_ADDR** `[15:0]`
    ///
    /// USBPD_DMA_ADDR value
    #[inline(always)]
    pub const fn usbpd_dma_addr(&self) -> u32 {
        (self.bits & Dma::USBPD_DMA_ADDR_MASK) >> Dma::USBPD_DMA_ADDR_OFFSET
    }

    /// Set **USBPD_DMA_ADDR** `[15:0]`
    ///
    /// USBPD_DMA_ADDR value
    #[inline(always)]
    pub const fn with_usbpd_dma_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dma::USBPD_DMA_ADDR_MASK)
                | ((value << Dma::USBPD_DMA_ADDR_OFFSET) & Dma::USBPD_DMA_ADDR_MASK),
        }
    }
}

impl Default for DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaVal {
        DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaVal) -> DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// PD interrupt enable register
    pub config: Config,
    /// 0x0002
    /// BMC sampling clock counter
    pub bmc_clk_cnt: BmcClkCnt,
    /// 0x0004
    /// PD Send and receive enable register
    pub control: Control,
    /// 0x0005
    /// SOP port selection register
    pub tx_sel: TxSel,
    /// 0x0006
    /// PD send length register
    pub bmc_tx_sz: BmcTxSz,
    /// 0x0008
    /// DMA cache data register
    pub data_buf: DataBuf,
    /// 0x0009
    /// PD interrupt flag register
    pub status: Status,
    /// 0x000a
    /// Byte counter
    pub bmc_byte_cnt: BmcByteCnt,
    /// 0x000c
    /// CC1 port control register
    pub port_cc1: PortCc1,
    /// 0x000e
    /// CC2 port control register
    pub port_cc2: PortCc2,
    /// 0x0010
    /// PD buffer start address register
    pub dma: Dma,
}
