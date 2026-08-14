//! USB PD physical-layer (PHY) driver.
//!
//! Drives the CH32X035's built-in USB-PD transceiver: BMC encode/decode,
//! preamble/SOP/CRC (all in hardware), CC detection and DMA receive/transmit.
//! The PD communication pins are **PC14** (CC1) and **PC15** (CC2); configure
//! them as floating inputs with [`crate::gpio`] first.
//!
//! Higher layers live in [`super::protocol`] (message build/parse) and
//! [`super::policy`] (Source/Sink negotiation).

use crate::Steal;
use crate::delay::DelayUs;
use crate::pac;
use crate::rcc::{AhbPeriph, Apb2Periph, Rcc};

// CONFIG register bits
const CONFIG_PD_ALL_CLR: u32 = 1 << 1;
const CONFIG_CC_SEL: u32 = 1 << 2;
const CONFIG_PD_DMA_EN: u32 = 1 << 3;
const CONFIG_IE_TX_END: u32 = 1 << 15;
const CONFIG_IE_RX_RESET: u32 = 1 << 14;
const CONFIG_IE_RX_ACT: u32 = 1 << 13;
const CONFIG_IE_RX_BYTE: u32 = 1 << 12;

// CONTROL register bits
const CONTROL_PD_TX_EN: u32 = 1 << 0;
const CONTROL_BMC_START: u32 = 1 << 1;

// STATUS register bits
const STATUS_BUF_ERR: u32 = 1 << 2;
const STATUS_IF_RX_BIT: u32 = 1 << 3;
const STATUS_IF_RX_BYTE: u32 = 1 << 4;
const STATUS_IF_RX_ACT: u32 = 1 << 5;
const STATUS_IF_RX_RESET: u32 = 1 << 6;
const STATUS_IF_TX_END: u32 = 1 << 7;

/// All STATUS interrupt flags (write these bits to clear).
const STATUS_ALL_FLAGS: u32 = STATUS_IF_RX_BIT
    | STATUS_IF_RX_BYTE
    | STATUS_IF_RX_ACT
    | STATUS_IF_RX_RESET
    | STATUS_IF_TX_END
    | STATUS_BUF_ERR;

// PORT_CC register bits (both CC1 and CC2 share the same layout)
const CC_PA_AI: u32 = 1 << 0; // comparator analogue input (read)
const CC_PD: u32 = 1 << 1; // pull-down resistor enable (sink / UFP)
const CC_PU_SHIFT: u32 = 2; // pull-up current, 2 bits
const CC_PU_MASK: u32 = 0b11 << CC_PU_SHIFT;
const CC_LV0: u32 = 1 << 4; // output low-voltage enable
// Comparator threshold field (bits [7:5]; not in the SVD but present in
// hardware, per the WCH SDK `ch32x035_usbpd.h`: CC_NO_CMP .. CC_CMP_123).
const CC_CMP_SHIFT: u32 = 5;
const CC_CMP_MASK: u32 = 0b111 << CC_CMP_SHIFT;

/// BMC clock counter for TX / RX at 48 MHz (see the WCH SDK).
const BMC_CLK_TX_48M: u32 = 80 - 1;
const BMC_CLK_RX_48M: u32 = 120 - 1;

/// A USB-PD CC port (CC1 = PC14, CC2 = PC15).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcPort {
    /// CC1 (PC14)
    Cc1,
    /// CC2 (PC15)
    Cc2,
}

/// CC pull-up current configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcPull {
    /// Pull-up disabled (sink)
    NoPull,
    /// 330 µA pull-up (source, default)
    Pull330uA,
    /// 180 µA pull-up
    Pull180uA,
    /// 80 µA pull-up
    Pull80uA,
}

impl CcPull {
    const fn bits(self) -> u32 {
        match self {
            CcPull::NoPull => 0,
            CcPull::Pull330uA => 1,
            CcPull::Pull180uA => 2,
            CcPull::Pull80uA => 3,
        }
    }
}

/// CC comparator threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcComparator {
    /// Comparator closed
    Closed,
    /// 0.22 V threshold
    Cmp0p22,
    /// 0.45 V threshold
    Cmp0p45,
    /// 0.55 V threshold
    Cmp0p55,
    /// 0.66 V threshold
    Cmp0p66,
    /// 0.95 V threshold
    Cmp0p95,
    /// 1.23 V threshold
    Cmp1p23,
}

impl CcComparator {
    const fn bits(self) -> u32 {
        match self {
            CcComparator::Closed => 0,
            CcComparator::Cmp0p22 => 2,
            CcComparator::Cmp0p45 => 3,
            CcComparator::Cmp0p55 => 4,
            CcComparator::Cmp0p66 => 5,
            CcComparator::Cmp0p95 => 6,
            CcComparator::Cmp1p23 => 7,
        }
    }
}

/// The Start-of-Packet sequence to send.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SopType {
    /// SOP' (normal PD packet)
    Sop0,
    /// SOP' prime
    Sop1,
    /// SOP'' double prime
    Sop2,
    /// Hard reset (RST1/RST2)
    HardReset,
    /// Cable reset
    CableReset,
}

impl SopType {
    /// The `TX_SEL` register value for this SOP type.
    pub const fn tx_sel(self) -> u8 {
        match self {
            // TX_SEL1_SYNC1 | TX_SEL2_SYNC1 | TX_SEL3_SYNC1 | TX_SEL4_SYNC2
            SopType::Sop0 => 0x00,
            // TX_SEL1_SYNC1 | TX_SEL2_SYNC1 | TX_SEL3_SYNC3 | TX_SEL4_SYNC3
            SopType::Sop1 => 0x50,
            // TX_SEL1_SYNC1 | TX_SEL2_SYNC3 | TX_SEL3_SYNC1 | TX_SEL4_SYNC3
            SopType::Sop2 => 0x44,
            // TX_SEL1_RST1 | TX_SEL2_RST1 | TX_SEL3_RST1 | TX_SEL4_RST2
            SopType::HardReset => 0xA9,
            // TX_SEL1_SYNC1 | TX_SEL2_RST1 | TX_SEL3_RST1 | TX_SEL4_RST2
            SopType::CableReset => 0xA8,
        }
    }
}

/// USB PD transceiver driver.
pub struct Usbpd {
    usbpd: pac::USBPD,
}

impl Usbpd {
    /// Create a new USBPD driver.
    ///
    /// Enables the USBPD (AHB) and AFIO clocks, configures the PD PHY
    /// (high-threshold input + 3.3 V PHY limit) in the AFIO controller and
    /// clears all USBPD interrupt flags.
    ///
    /// PC14/PC15 must be configured as floating inputs by the caller
    /// (e.g. with [`crate::gpio::Pins::pc`]).
    pub fn init(usbpd: pac::USBPD, afio: &pac::AFIO, rcc: &mut Rcc) -> Self {
        rcc.ahb_enable(AhbPeriph::Usbpd);
        rcc.apb2_enable(Apb2Periph::Afio);

        // PD PHY: high-threshold input mode + 3.3 V PHY limit
        afio.ctlr.set_usbpd_in_hvt();
        afio.ctlr.set_usbpd_phy_v33();

        let mut u = Self { usbpd };
        u.clear_all_flags();
        // Enable the PD DMA path
        u.usbpd.config.modify_bits(|v| v | CONFIG_PD_DMA_EN);
        u
    }

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialized and that no
    /// data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            usbpd: unsafe { pac::USBPD::steal() },
        })
    }

    // =====================================================================
    // CC port configuration
    // =====================================================================

    /// Apply a read-modify-write to the given CC port's register.
    fn cc_modify<F: FnOnce(u32) -> u32>(&self, port: CcPort, f: F) {
        match port {
            CcPort::Cc1 => self.usbpd.port_cc1.modify_bits(f),
            CcPort::Cc2 => self.usbpd.port_cc2.modify_bits(f),
        }
    }

    /// Set the CC pull-up current (source) or disable it (sink).
    ///
    /// Enabling a pull-up disables the pull-down resistor, and vice versa —
    /// the two are mutually exclusive (source vs. sink).
    pub fn set_cc_pull(&mut self, port: CcPort, pull: CcPull) {
        self.cc_modify(port, |v| {
            let v = (v & !CC_PU_MASK) | (pull.bits() << CC_PU_SHIFT);
            if pull.bits() == 0 { v } else { v & !CC_PD }
        });
    }

    /// Enable or disable the CC pull-down resistor (sink / UFP).
    ///
    /// Enabling the pull-down disables the pull-up current, and vice versa.
    pub fn set_cc_pull_down(&mut self, port: CcPort, enabled: bool) {
        self.cc_modify(port, |v| {
            if enabled {
                (v & !CC_PU_MASK) | CC_PD
            } else {
                v & !CC_PD
            }
        });
    }

    /// Set the CC comparator threshold.
    pub fn set_cc_comparator(&mut self, port: CcPort, threshold: CcComparator) {
        self.cc_modify(port, |v| {
            (v & !CC_CMP_MASK) | (threshold.bits() << CC_CMP_SHIFT)
        });
    }

    /// Enable the CC output low-voltage (used before BMC transmission).
    pub fn enable_cc_low_voltage(&mut self, port: CcPort) {
        self.cc_modify(port, |v| v | CC_LV0);
    }

    /// Check whether a cable is connected on `port`.
    ///
    /// Selects the 0.22 V comparator threshold, waits for it to settle and
    /// reads the analogue input pin.
    pub fn check_cc(&mut self, port: CcPort, delay: &impl DelayUs) -> bool {
        self.cc_modify(port, |v| {
            (v & !(CC_CMP_MASK | CC_PA_AI)) | (CcComparator::Cmp0p22.bits() << CC_CMP_SHIFT)
        });
        // Let the comparator settle (see the WCH SDK `PD_Detect`).
        delay.delay_us(2);
        match port {
            CcPort::Cc1 => self.usbpd.port_cc1.read_bits() & CC_PA_AI != 0,
            CcPort::Cc2 => self.usbpd.port_cc2.read_bits() & CC_PA_AI != 0,
        }
    }

    /// Select the active CC port for PD communication (`CC_SEL`).
    pub fn select_cc(&mut self, port: CcPort) {
        match port {
            CcPort::Cc1 => self.usbpd.config.modify_bits(|v| v & !CONFIG_CC_SEL),
            CcPort::Cc2 => self.usbpd.config.modify_bits(|v| v | CONFIG_CC_SEL),
        }
    }

    // =====================================================================
    // Receive
    // =====================================================================

    /// Start receiving a PD packet into `buf` (via DMA).
    ///
    /// The hardware writes up to `buf.len()` bytes into `buf` when a packet
    /// arrives. `buf` must be 4-byte aligned and must stay alive until the
    /// receive completes (poll [`Usbpd::is_rx_complete`] or the USBPD IRQ).
    ///
    /// # Safety
    ///
    /// The DMA engine writes directly to `buf` asynchronously; the caller
    /// must ensure `buf` remains valid and unaliased until the receive
    /// completes.
    pub unsafe fn start_receive(&mut self, buf: &mut [u8]) {
        debug_assert!(
            (buf.as_ptr() as usize).is_multiple_of(4),
            "PD DMA buffer must be 4-byte aligned"
        );

        // Clear all flags, then set up RX mode
        self.usbpd.config.modify_bits(|v| v | CONFIG_PD_ALL_CLR);
        self.usbpd.config.modify_bits(|v| v & !CONFIG_PD_ALL_CLR);
        self.usbpd
            .config
            .modify_bits(|v| v | CONFIG_IE_RX_ACT | CONFIG_IE_RX_RESET | CONFIG_PD_DMA_EN);

        // Set the DMA destination and enter receive mode
        self.usbpd.dma.write_bits(buf.as_ptr() as u32);
        self.usbpd.control.modify_bits(|v| v & !CONTROL_PD_TX_EN);
        self.usbpd.bmc_clk_cnt.write_bits(BMC_CLK_RX_48M);
        self.usbpd.control.modify_bits(|v| v | CONTROL_BMC_START);
    }

    /// Whether a packet has been received (IF_RX_ACT set).
    pub fn is_rx_complete(&self) -> bool {
        self.usbpd.status.read_bits() & STATUS_IF_RX_ACT != 0
    }

    /// The number of bytes received in the last packet.
    pub fn received_len(&self) -> u8 {
        self.usbpd.bmc_byte_cnt.read_bits() as u8
    }

    // =====================================================================
    // Transmit
    // =====================================================================

    /// Send a PD packet (`buf`) with the given SOP type (blocking).
    ///
    /// The packet is transmitted using BMC and this method blocks until the
    /// hardware reports `TX_END`. `buf` must be 4-byte aligned.
    pub fn send(&mut self, buf: &[u8], sop: SopType) {
        debug_assert!(
            (buf.as_ptr() as usize).is_multiple_of(4),
            "PD DMA buffer must be 4-byte aligned"
        );

        self.usbpd.config.modify_bits(|v| v | CONFIG_IE_TX_END);

        // Drive the selected CC port low for transmission.
        let cc2 = self.usbpd.config.read_bits() & CONFIG_CC_SEL != 0;
        if cc2 {
            self.usbpd.port_cc2.modify_bits(|v| v | CC_LV0);
        } else {
            self.usbpd.port_cc1.modify_bits(|v| v | CC_LV0);
        }

        self.usbpd.bmc_clk_cnt.write_bits(BMC_CLK_TX_48M);
        self.usbpd.dma.write_bits(buf.as_ptr() as u32);
        self.usbpd.tx_sel.write_bits(sop.tx_sel() as u32);
        self.usbpd.bmc_tx_sz.write_bits(buf.len() as u32);
        self.usbpd.control.modify_bits(|v| v | CONTROL_PD_TX_EN);

        // Clear stale status, then start the BMC transmission
        self.usbpd.status.write_bits(0);
        self.usbpd.control.modify_bits(|v| v | CONTROL_BMC_START);

        crate::spin::spin_wait("USBPD TX end", || {
            self.usbpd.status.read_bits() & STATUS_IF_TX_END != 0
        });
        self.usbpd.status.write_bits(STATUS_IF_TX_END);

        // Release the CC line so reception can resume (WCH SDK).
        if cc2 {
            self.usbpd.port_cc2.modify_bits(|v| v & !CC_LV0);
        } else {
            self.usbpd.port_cc1.modify_bits(|v| v & !CC_LV0);
        }
    }

    /// Whether the last transmission has completed (IF_TX_END set).
    pub fn is_tx_done(&self) -> bool {
        self.usbpd.status.read_bits() & STATUS_IF_TX_END != 0
    }

    // =====================================================================
    // Interrupts & status
    // =====================================================================

    /// Clear all USBPD interrupt flags.
    pub fn clear_all_flags(&mut self) {
        self.usbpd.status.write_bits(STATUS_ALL_FLAGS);
    }

    /// Read the raw STATUS register value.
    pub fn status(&self) -> u32 {
        self.usbpd.status.read_bits()
    }

    /// Enable the USBPD IRQ handler for receive events.
    pub fn enable_rx_interrupt(&mut self) {
        self.usbpd
            .config
            .modify_bits(|v| v | CONFIG_IE_RX_ACT | CONFIG_IE_RX_RESET | CONFIG_IE_RX_BYTE);
    }

    /// Disable all USBPD interrupts.
    pub fn disable_interrupt(&mut self) {
        self.usbpd.config.modify_bits(|v| {
            v & !(CONFIG_IE_TX_END | CONFIG_IE_RX_ACT | CONFIG_IE_RX_RESET | CONFIG_IE_RX_BYTE)
        });
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::USBPD {
        self.usbpd
    }
}
