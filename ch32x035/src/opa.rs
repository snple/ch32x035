//! OPA (Operational Amplifier / Comparator)
//!
//! Drives the CH32X035's two operational amplifiers (OPA1/OPA2, `CTLR1`) and
//! three comparators (CMP1-3, `CTLR2`). The control registers must be
//! unlocked by writing the key sequence before configuration.
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

use crate::pac;

/// Unlock key 1
const OPA_KEY1: u32 = 0x4567_0123;
/// Unlock key 2
const OPA_KEY2: u32 = 0xCDEF_89AB;

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
#[derive(Clone, Copy, Debug, Default)]
pub struct ComparatorConfig {
    /// Comparator mode
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
}

impl Opa {
    /// Create a new OPA driver. OPA has no clock gate on the CH32X035.
    pub fn new(opa: pac::OPA) -> Self {
        Self { opa }
    }

    /// Unlock the OPA and CMP configuration registers (write key sequence).
    pub fn unlock(&mut self) {
        self.opa.opa_key.write_opa_key(OPA_KEY1);
        self.opa.opa_key.write_opa_key(OPA_KEY2);
        self.opa.cmp_key.write_cmp_key(OPA_KEY1);
        self.opa.cmp_key.write_cmp_key(OPA_KEY2);
    }

    /// Lock the OPA configuration registers.
    pub fn lock(&mut self) {
        self.opa.ctlr1.set_opa_lock();
        self.opa.ctlr2.set_cmp_lock();
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
    pub fn enable_opamp2(&mut self) {
        self.opa.ctlr1.set_en2();
    }

    /// Disable OPA2.
    pub fn disable_opamp2(&mut self) {
        self.opa.ctlr1.clr_en2();
    }

    /// Check whether OPA2 is enabled.
    pub fn is_opamp2_enabled(&self) -> bool {
        self.opa.ctlr1.en2()
    }

    // =====================================================================
    // Comparators (CTLR2)
    // =====================================================================

    /// Configure comparator 1.
    pub fn configure_comparator1(&mut self, cfg: &ComparatorConfig) {
        self.apply_cmp(1, cfg);
    }

    /// Enable comparator 1.
    pub fn enable_comparator1(&mut self) {
        self.opa.ctlr2.set_en1();
    }

    /// Disable comparator 1.
    pub fn disable_comparator1(&mut self) {
        self.opa.ctlr2.clr_en1();
    }

    /// Configure comparator 2.
    pub fn configure_comparator2(&mut self, cfg: &ComparatorConfig) {
        self.apply_cmp(2, cfg);
    }

    /// Enable comparator 2.
    pub fn enable_comparator2(&mut self) {
        self.opa.ctlr2.set_en2();
    }

    /// Disable comparator 2.
    pub fn disable_comparator2(&mut self) {
        self.opa.ctlr2.clr_en2();
    }

    /// Configure comparator 3.
    pub fn configure_comparator3(&mut self, cfg: &ComparatorConfig) {
        self.apply_cmp(3, cfg);
    }

    /// Enable comparator 3.
    pub fn enable_comparator3(&mut self) {
        self.opa.ctlr2.set_en3();
    }

    /// Disable comparator 3.
    pub fn disable_comparator3(&mut self) {
        self.opa.ctlr2.clr_en3();
    }

    /// Apply the common single-bit comparator fields.
    fn apply_cmp(&mut self, cmp: u8, cfg: &ComparatorConfig) {
        let mode = cfg.mode;
        let nsel = cfg.nsel;
        let psel = cfg.psel;
        let hy = cfg.hysteresis;
        match cmp {
            1 => {
                self.cmp_bit("mode1", mode);
                self.cmp_bit("nsel1", nsel);
                self.cmp_bit("psel1", psel);
                self.cmp_bit("hyen1", hy);
            }
            2 => {
                self.cmp_bit("mode2", mode);
                self.cmp_bit("nsel2", nsel);
                self.cmp_bit("psel2", psel);
                self.cmp_bit("hyen2", hy);
            }
            _ => {
                self.cmp_bit("mode3", mode);
                self.cmp_bit("nsel3", nsel);
                self.cmp_bit("psel3", psel);
                self.cmp_bit("hyen3", hy);
            }
        }
    }

    /// Set or clear a single-bit comparator field by name.
    fn cmp_bit(&mut self, name: &str, value: bool) {
        let reg = &self.opa.ctlr2;
        if value {
            match name {
                "mode1" => reg.set_mode1(),
                "nsel1" => reg.set_nsel1(),
                "psel1" => reg.set_psel1(),
                "hyen1" => reg.set_hyen1(),
                "mode2" => reg.set_mode2(),
                "nsel2" => reg.set_nsel2(),
                "psel2" => reg.set_psel2(),
                "hyen2" => reg.set_hyen2(),
                _ => reg.set_hyen3(),
            }
        } else {
            match name {
                "mode1" => reg.clr_mode1(),
                "nsel1" => reg.clr_nsel1(),
                "psel1" => reg.clr_psel1(),
                "hyen1" => reg.clr_hyen1(),
                "mode2" => reg.clr_mode2(),
                "nsel2" => reg.clr_nsel2(),
                "psel2" => reg.clr_psel2(),
                "hyen2" => reg.clr_hyen2(),
                _ => reg.clr_hyen3(),
            }
        }
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::OPA {
        self.opa
    }
}
