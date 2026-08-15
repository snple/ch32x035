//! OPA (Operational Amplifier / Comparator)
//!
//! Drives the CH32X035's operational amplifiers (OPA1, plus OPA2 on packages
//! that have it, `CTLR1`) and comparators (CMP1-3 on packages that have them,
//! `CTLR2`), as well as the polling / timer-break (BKIN) / interrupt
//! configuration (`CFGR1`/`CFGR2`). The control registers must be unlocked
//! by writing the key sequence before configuration.
//!
//! The OPA2 and CMP1-3 methods only exist when the selected sub-model
//! actually has those blocks (see [`crate::package`]).
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::opa::{Opa, OpampConfig};
//!
//! let mut opa = Opa::new(dp.OPA);
//! opa.unlock();
//! opa.configure_opamp1(&OpampConfig {
//!     mode: false, psel: 0, nsel: 0, feedback: false,
//! });
//! opa.enable_opamp1();
//! ```

use crate::Steal;
use crate::pac;

/// Unlock key 1
const OPA_KEY1: u32 = 0x4567_0123;
/// Unlock key 2
const OPA_KEY2: u32 = 0xCDEF_89AB;

// `CFGR1` (offset 0x00) and `CFGR2` (offset 0x02) are a pair of adjacent
// 16-bit registers. The PAC models them as 32-bit registers, which would
// place `cfgr2` at the wrong address, so this driver accesses them directly
// with 16-bit volatile reads/writes (see `cfg1`/`set_cfg1`/`set_cfg2`).
/// `CFGR1` bit: OPA1 positive-input polling enable
const CFG_POLL_EN1: u16 = 1 << 0;
/// `CFGR1` bit: OPA2 positive-input polling enable
#[cfg(feature = "__opa2")]
const CFG_POLL_EN2: u16 = 1 << 1;
/// `CFGR1` bit: OPA1 timer-break (BKIN) enable
const CFG_BKIN_EN1: u16 = 1 << 2;
/// `CFGR1` bit: OPA2 timer-break (BKIN) enable
#[cfg(feature = "__opa2")]
const CFG_BKIN_EN2: u16 = 1 << 3;
/// `CFGR1` bit: OPA1 system-reset enable
const CFG_RST_EN1: u16 = 1 << 4;
/// `CFGR1` bit: OPA2 system-reset enable
#[cfg(feature = "__opa2")]
const CFG_RST_EN2: u16 = 1 << 5;
/// `CFGR1` bit: break input timer selection (swap OPA1/OPA2)
const CFG_BKIN_SEL: u16 = 1 << 6;
/// `CFGR1` bit: OPA1 output interrupt enable
const CFG_IE_OUT1: u16 = 1 << 8;
/// `CFGR1` bit: OPA2 output interrupt enable
#[cfg(feature = "__opa2")]
const CFG_IE_OUT2: u16 = 1 << 9;
/// `CFGR1` bit: polling-interval-end interrupt enable
const CFG_IE_CNT: u16 = 1 << 10;
/// `CFGR1` bit: OPA NMI enable
const CFG_NMI_EN: u16 = 1 << 11;
/// `CFGR1` flag: OPA1 output was high while polling
const CFG_IF_OUT1: u16 = 1 << 12;
/// `CFGR1` flag: OPA2 output was high while polling
const CFG_IF_OUT2: u16 = 1 << 13;
/// `CFGR1` flag: polling interval end
const CFG_IF_CNT: u16 = 1 << 14;

/// Number of positive inputs an OPA polls (`POLLx_NUM`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PollCount {
    /// One input (P0)
    One = 0b00,
    /// Two inputs (P0 + P1)
    Two = 0b01,
    /// Three inputs (P0 + P1 + P2)
    Three = 0b10,
}

/// Configuration for an operational amplifier (OPA1/OPA2).
#[derive(Clone, Copy, Debug, Default)]
pub struct OpampConfig {
    /// Amplifier mode
    pub mode: bool,
    /// Positive input select (0-3)
    pub psel: u8,
    /// Negative input select (0-7)
    pub nsel: u8,
    /// Internal feedback enable
    pub feedback: bool,
}

/// Configuration for a comparator (CMP1-3).
///
/// Every field is a single select bit:
/// - `psel` — positive input. CMP1: 0=PC19 / 1=PA0, CMP2: 0=PA12 / 1=PA11,
///   CMP3: 0=PA13 / 1=PA14.
/// - `nsel` — negative input. CMP1: 0=PC3 / 1=PA23, CMP2: 0=PA22 / 1=PC3,
///   CMP3: 0=PA2 / 1=PC3.
/// - `mode` — output channel: `false` = internal timer channel
///   (CMP1→TIM2_CH1, CMP2→TIM2_CH2, CMP3→TIM2_CH3), `true` = I/O pin
///   (CMP1→PA1, CMP2→PB2, CMP3→PB3).
#[derive(Clone, Copy, Debug, Default)]
pub struct ComparatorConfig {
    /// Output channel select
    pub mode: bool,
    /// Positive input select
    pub psel: bool,
    /// Negative input select
    pub nsel: bool,
    /// Hysteresis enable
    pub hysteresis: bool,
}

/// OPA / CMP driver
pub struct Opa {
    opa: pac::OPA,
    /// Shadow of the `CFGR2` polling fields (`POLL1_NUM`/`POLL2_NUM` are
    /// write-only, so the driver keeps the last written value).
    cfg2: u16,
}

impl Opa {
    /// Create a new OPA driver. OPA has no clock gate on the CH32X035.
    pub fn new(opa: pac::OPA) -> Self {
        Self { opa, cfg2: 0 }
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
            opa: unsafe { pac::OPA::steal() },
            cfg2: 0,
        })
    }

    /// Unlock the OPA, CMP and polling configuration registers (write key
    /// sequence).
    pub fn unlock(&mut self) {
        self.opa.opa_key.write_opa_key(OPA_KEY1);
        self.opa.opa_key.write_opa_key(OPA_KEY2);
        self.opa.cmp_key.write_cmp_key(OPA_KEY1);
        self.opa.cmp_key.write_cmp_key(OPA_KEY2);
        self.opa.poll_key.write_poll_key(OPA_KEY1);
        self.opa.poll_key.write_poll_key(OPA_KEY2);
    }

    /// Lock the OPA and CMP configuration registers.
    pub fn lock(&mut self) {
        self.opa.ctlr1.set_opa_lock();
        self.opa.ctlr2.set_cmp_lock();
        // CFGR1 bit 7 (POLL_LOCK) is write-1-to-lock.
        self.set_cfg1(self.cfg1() | (1 << 7));
    }

    // =====================================================================
    // Operational amplifiers (CTLR1)
    // =====================================================================

    /// Configure OPA1 (mode, inputs, feedback).
    pub fn configure_opamp1(&mut self, cfg: &OpampConfig) {
        if cfg.mode {
            self.opa.ctlr1.set_mode1();
        } else {
            self.opa.ctlr1.clr_mode1();
        }
        self.opa.ctlr1.write_psel1((cfg.psel & 0b11) as u32);
        self.opa.ctlr1.write_nsel1((cfg.nsel & 0b111) as u32);
        if cfg.feedback {
            self.opa.ctlr1.set_fb_en1();
        } else {
            self.opa.ctlr1.clr_fb_en1();
        }
    }

    /// Enable OPA1.
    pub fn enable_opamp1(&mut self) {
        self.opa.ctlr1.set_en1();
    }

    /// Disable OPA1.
    pub fn disable_opamp1(&mut self) {
        self.opa.ctlr1.clr_en1();
    }

    /// Check whether OPA1 is enabled.
    pub fn is_opamp1_enabled(&self) -> bool {
        self.opa.ctlr1.en1()
    }

    /// Configure OPA2 (mode, inputs, feedback).
    #[cfg(feature = "__opa2")]
    pub fn configure_opamp2(&mut self, cfg: &OpampConfig) {
        if cfg.mode {
            self.opa.ctlr1.set_mode2();
        } else {
            self.opa.ctlr1.clr_mode2();
        }
        self.opa.ctlr1.write_psel2((cfg.psel & 0b11) as u32);
        self.opa.ctlr1.write_nsel2((cfg.nsel & 0b111) as u32);
        if cfg.feedback {
            self.opa.ctlr1.set_fb_en2();
        } else {
            self.opa.ctlr1.clr_fb_en2();
        }
    }

    /// Enable OPA2.
    #[cfg(feature = "__opa2")]
    pub fn enable_opamp2(&mut self) {
        self.opa.ctlr1.set_en2();
    }

    /// Disable OPA2.
    #[cfg(feature = "__opa2")]
    pub fn disable_opamp2(&mut self) {
        self.opa.ctlr1.clr_en2();
    }

    /// Check whether OPA2 is enabled.
    #[cfg(feature = "__opa2")]
    pub fn is_opamp2_enabled(&self) -> bool {
        self.opa.ctlr1.en2()
    }

    // =====================================================================
    // Comparators (CTLR2)
    // =====================================================================

    /// Configure comparator 1 (CMP1).
    ///
    /// See [`ComparatorConfig`] for the pin mapping of each field.
    #[cfg(feature = "__cmp1")]
    pub fn configure_comparator1(&mut self, cfg: &ComparatorConfig) {
        let reg = &self.opa.ctlr2;
        if cfg.mode {
            reg.set_mode1();
        } else {
            reg.clr_mode1();
        }
        if cfg.psel {
            reg.set_psel1();
        } else {
            reg.clr_psel1();
        }
        if cfg.nsel {
            reg.set_nsel1();
        } else {
            reg.clr_nsel1();
        }
        if cfg.hysteresis {
            reg.set_hyen1();
        } else {
            reg.clr_hyen1();
        }
    }

    /// Enable comparator 1.
    #[cfg(feature = "__cmp1")]
    pub fn enable_comparator1(&mut self) {
        self.opa.ctlr2.set_en1();
    }

    /// Disable comparator 1.
    #[cfg(feature = "__cmp1")]
    pub fn disable_comparator1(&mut self) {
        self.opa.ctlr2.clr_en1();
    }

    /// Check whether comparator 1 is enabled.
    #[cfg(feature = "__cmp1")]
    pub fn is_comparator1_enabled(&self) -> bool {
        self.opa.ctlr2.en1()
    }

    /// Configure comparator 2 (CMP2).
    #[cfg(feature = "__cmp2")]
    pub fn configure_comparator2(&mut self, cfg: &ComparatorConfig) {
        let reg = &self.opa.ctlr2;
        if cfg.mode {
            reg.set_mode2();
        } else {
            reg.clr_mode2();
        }
        if cfg.psel {
            reg.set_psel2();
        } else {
            reg.clr_psel2();
        }
        if cfg.nsel {
            reg.set_nsel2();
        } else {
            reg.clr_nsel2();
        }
        if cfg.hysteresis {
            reg.set_hyen2();
        } else {
            reg.clr_hyen2();
        }
    }

    /// Enable comparator 2.
    #[cfg(feature = "__cmp2")]
    pub fn enable_comparator2(&mut self) {
        self.opa.ctlr2.set_en2();
    }

    /// Disable comparator 2.
    #[cfg(feature = "__cmp2")]
    pub fn disable_comparator2(&mut self) {
        self.opa.ctlr2.clr_en2();
    }

    /// Check whether comparator 2 is enabled.
    #[cfg(feature = "__cmp2")]
    pub fn is_comparator2_enabled(&self) -> bool {
        self.opa.ctlr2.en2()
    }

    /// Configure comparator 3 (CMP3).
    #[cfg(feature = "__cmp3")]
    pub fn configure_comparator3(&mut self, cfg: &ComparatorConfig) {
        let reg = &self.opa.ctlr2;
        if cfg.mode {
            reg.set_mode3();
        } else {
            reg.clr_mode3();
        }
        if cfg.psel {
            reg.set_psel3();
        } else {
            reg.clr_psel3();
        }
        if cfg.nsel {
            reg.set_nsel3();
        } else {
            reg.clr_nsel3();
        }
        if cfg.hysteresis {
            reg.set_hyen3();
        } else {
            reg.clr_hyen3();
        }
    }

    /// Enable comparator 3.
    #[cfg(feature = "__cmp3")]
    pub fn enable_comparator3(&mut self) {
        self.opa.ctlr2.set_en3();
    }

    /// Disable comparator 3.
    #[cfg(feature = "__cmp3")]
    pub fn disable_comparator3(&mut self) {
        self.opa.ctlr2.clr_en3();
    }

    /// Check whether comparator 3 is enabled.
    #[cfg(feature = "__cmp3")]
    pub fn is_comparator3_enabled(&self) -> bool {
        self.opa.ctlr2.en3()
    }

    // =====================================================================
    // Polling / break / interrupt configuration (CFGR1 / CFGR2)
    // =====================================================================

    /// Read the 16-bit `CFGR1` register (offset 0x00).
    fn cfg1(&self) -> u16 {
        // SAFETY: `CFGR1` is a 16-bit register at the OPA base address.
        unsafe { core::ptr::read_volatile(pac::opa::BASE as *const u16) }
    }

    /// Write the 16-bit `CFGR1` register (offset 0x00).
    fn set_cfg1(&mut self, value: u16) {
        // SAFETY: `CFGR1` is a 16-bit register at the OPA base address.
        unsafe { core::ptr::write_volatile(pac::opa::BASE as *mut u16, value) }
    }

    /// Write the 16-bit `CFGR2` register (offset 0x02).
    fn set_cfg2(&mut self, value: u16) {
        // SAFETY: `CFGR2` is a 16-bit register at the OPA base address + 2.
        unsafe { core::ptr::write_volatile((pac::opa::BASE + 2) as *mut u16, value) }
    }

    /// Set or clear a single `CFGR1` bit.
    fn cfg1_bit(&mut self, mask: u16, value: bool) {
        let v = self.cfg1();
        self.set_cfg1(if value { v | mask } else { v & !mask });
    }

    /// Enable/disable OPA1 positive-input polling.
    pub fn set_poll_opamp1(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_POLL_EN1, enabled);
    }

    /// Enable/disable OPA2 positive-input polling.
    #[cfg(feature = "__opa2")]
    pub fn set_poll_opamp2(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_POLL_EN2, enabled);
    }

    /// Set how many positive inputs OPA1 polls (P0, P0+P1 or P0+P1+P2).
    pub fn set_poll_count_opamp1(&mut self, count: PollCount) {
        self.cfg2 = (self.cfg2 & !(0b11 << 9)) | ((count as u16) << 9);
        self.set_cfg2(self.cfg2);
    }

    /// Set how many positive inputs OPA2 polls (P0, P0+P1 or P0+P1+P2).
    #[cfg(feature = "__opa2")]
    pub fn set_poll_count_opamp2(&mut self, count: PollCount) {
        self.cfg2 = (self.cfg2 & !(0b11 << 11)) | ((count as u16) << 11);
        self.set_cfg2(self.cfg2);
    }

    /// Set the polling interval. `us` must be 1..=512; values are clamped.
    /// The interval is `us` µs.
    pub fn set_poll_interval_us(&mut self, us: u16) {
        let vlu = us.saturating_sub(1).min(0x1FF);
        self.cfg2 = (self.cfg2 & !0x1FF) | vlu;
        self.set_cfg2(self.cfg2);
    }

    /// Enable/disable OPA1 as the timer break (BKIN) source.
    pub fn set_break_opamp1(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_BKIN_EN1, enabled);
    }

    /// Enable/disable OPA2 as the timer break (BKIN) source.
    #[cfg(feature = "__opa2")]
    pub fn set_break_opamp2(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_BKIN_EN2, enabled);
    }

    /// Swap which OPA output drives the TIM1 vs TIM2 break input.
    ///
    /// The break signal is active-high: set the timer's `BKP` bit too.
    pub fn set_break_swapped(&mut self, swapped: bool) {
        self.cfg1_bit(CFG_BKIN_SEL, swapped);
    }

    /// Enable/disable OPA1 resetting the system.
    pub fn set_reset_opamp1(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_RST_EN1, enabled);
    }

    /// Enable/disable OPA2 resetting the system.
    #[cfg(feature = "__opa2")]
    pub fn set_reset_opamp2(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_RST_EN2, enabled);
    }

    /// Enable/disable the OPA1 output interrupt.
    pub fn set_interrupt_opamp1(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_IE_OUT1, enabled);
    }

    /// Enable/disable the OPA2 output interrupt.
    #[cfg(feature = "__opa2")]
    pub fn set_interrupt_opamp2(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_IE_OUT2, enabled);
    }

    /// Enable/disable the polling-interval-end interrupt.
    pub fn set_poll_end_interrupt(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_IE_CNT, enabled);
    }

    /// Enable/disable the OPA NMI.
    pub fn set_nmi_enabled(&mut self, enabled: bool) {
        self.cfg1_bit(CFG_NMI_EN, enabled);
    }

    /// Polling interval end flag.
    pub fn poll_end_flag(&self) -> bool {
        self.cfg1() & CFG_IF_CNT != 0
    }

    /// OPA1 output-high flag (set while polling).
    pub fn opamp1_output_flag(&self) -> bool {
        self.cfg1() & CFG_IF_OUT1 != 0
    }

    /// OPA2 output-high flag (set while polling).
    #[cfg(feature = "__opa2")]
    pub fn opamp2_output_flag(&self) -> bool {
        self.cfg1() & CFG_IF_OUT2 != 0
    }

    /// Clear all three interrupt flags (write 0).
    pub fn clear_interrupt_flags(&mut self) {
        let v = self.cfg1() & !(CFG_IF_CNT | CFG_IF_OUT1 | CFG_IF_OUT2);
        self.set_cfg1(v);
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::OPA {
        self.opa
    }
}
