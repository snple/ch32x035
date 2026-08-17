//! PWR (Power Control)
//!
//! Provides access to the power control registers (deep-sleep mode, PVD
//! level). The CH32X035 runs in normal mode by default; this module is useful
//! for low-power designs.

use crate::Steal;
use crate::pac;
use crate::rcc::Rcc;
use crate::reset::Resettable;

/// PWR driver
pub struct Pwr {
    pwr: pac::PWR,
}

impl Pwr {
    /// Create a new PWR driver, resetting and clock-enabling the peripheral.
    pub fn init(pwr: pac::PWR, rcc: &mut Rcc) -> Self {
        pwr.init(rcc);
        Self { pwr }
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
            pwr: unsafe { pac::PWR::steal() },
        })
    }

    /// Enable deep-sleep (power-down) mode when `WFI` is executed.
    pub fn enter_deep_sleep(&mut self) {
        self.pwr.ctlr.set_pdds();
    }

    /// Disable deep-sleep mode (normal sleep on `WFI`).
    pub fn exit_deep_sleep(&mut self) {
        self.pwr.ctlr.clr_pdds();
    }

    /// Check whether deep-sleep mode is selected.
    pub fn is_deep_sleep(&self) -> bool {
        self.pwr.ctlr.pdds()
    }

    /// Set the PVD (programmable voltage detector) threshold level (0..=7).
    pub fn set_pvd_level(&mut self, level: u8) {
        self.pwr.ctlr.write_pls((level & 0b111) as u32);
    }

    /// Check whether the supply voltage is above the PVD threshold.
    pub fn is_pvd_above(&self) -> bool {
        self.pwr.csr.pvdo()
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::PWR {
        self.pwr
    }
}
