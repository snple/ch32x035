//! AWU (Auto-Wakeup) driver
//!
//! The AWU generates periodic wake-up events from the low-speed internal
//! clock, typically used to wake the chip from standby mode.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::awu::Awu;
//!
//! let mut awu = Awu::new(dp.AWU);
//! awu.set_prescaler(0b1111); // 61440 divisor
//! awu.set_window(0x10);
//! awu.enable();
//! ```

use crate::Steal;
use crate::pac;

/// Auto-Wakeup driver
pub struct Awu {
    awu: pac::AWU,
}

impl Awu {
    /// Create a new AWU driver. AWU has no clock gate on the CH32X035.
    #[inline]
    pub fn new(awu: pac::AWU) -> Self {
        Self { awu }
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
            awu: unsafe { pac::AWU::steal() },
        })
    }

    /// Enable the auto-wakeup function.
    #[inline(always)]
    pub fn enable(&mut self) {
        self.awu.r16_awu_csr.set_awuen();
    }

    /// Disable the auto-wakeup function.
    #[inline(always)]
    pub fn disable(&mut self) {
        self.awu.r16_awu_csr.clr_awuen();
    }

    /// Check whether the auto-wakeup is enabled.
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.awu.r16_awu_csr.awuen()
    }

    /// Set the wake-up prescaler / time-base value (`AWU_TBR`, low 4 bits).
    ///
    /// Higher values give longer wake-up periods. See the WCH SDK
    /// `AWU_Prescaler_x` definitions (1, 2, 4, ... 61440).
    #[inline(always)]
    pub fn set_prescaler(&mut self, tbr: u8) {
        self.awu.r16_awu_psc.write_awu_tbr((tbr & 0x0F) as u32);
    }

    /// Set the wake-up window value (`AWU_APR`, low 6 bits).
    #[inline(always)]
    pub fn set_window(&mut self, apr: u8) {
        self.awu.r16_awu_wr.write_awu_apr((apr & 0x3F) as u32);
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::AWU {
        self.awu
    }
}
