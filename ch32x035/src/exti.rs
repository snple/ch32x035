//! EXTI (External Interrupt/Event Controller)
//!
//! Configures rising/falling triggers and interrupt enables for external
//! interrupt lines 0..=29, and reads/clears pending flags.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, exti::Exti};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let mut exti = Exti::init(dp.EXTI, &mut rcc);
//! exti.enable_rising::<0>();
//! exti.enable_interrupt::<0>();
//! ```

use crate::Steal;
use crate::pac;
use crate::rcc::{Apb2Periph, Rcc};

/// EXTI driver
pub struct Exti {
    exti: pac::EXTI,
}

impl Exti {
    /// Create a new EXTI driver, enabling the AFIO clock (needed for EXTI
    /// pin mapping). EXTI itself has no dedicated clock gate on the CH32X035.
    pub fn init(exti: pac::EXTI, rcc: &mut Rcc) -> Self {
        rcc.apb2_enable(Apb2Periph::Afio);
        Self { exti }
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
            exti: unsafe { pac::EXTI::steal() },
        })
    }

    /// Enable the rising-edge trigger on `LINE`.
    #[inline(always)]
    pub fn enable_rising<const LINE: u8>(&mut self) {
        self.exti.rtenr.modify_bits(|v| v | (1 << LINE));
    }

    /// Enable the falling-edge trigger on `LINE`.
    #[inline(always)]
    pub fn enable_falling<const LINE: u8>(&mut self) {
        self.exti.ftenr.modify_bits(|v| v | (1 << LINE));
    }

    /// Disable the rising-edge trigger on `LINE`.
    #[inline(always)]
    pub fn disable_rising<const LINE: u8>(&mut self) {
        self.exti.rtenr.modify_bits(|v| v & !(1 << LINE));
    }

    /// Disable the falling-edge trigger on `LINE`.
    #[inline(always)]
    pub fn disable_falling<const LINE: u8>(&mut self) {
        self.exti.ftenr.modify_bits(|v| v & !(1 << LINE));
    }

    /// Enable the interrupt for `LINE`.
    #[inline(always)]
    pub fn enable_interrupt<const LINE: u8>(&mut self) {
        self.exti.intenr.modify_bits(|v| v | (1 << LINE));
    }

    /// Disable the interrupt for `LINE`.
    #[inline(always)]
    pub fn disable_interrupt<const LINE: u8>(&mut self) {
        self.exti.intenr.modify_bits(|v| v & !(1 << LINE));
    }

    /// Check whether `LINE` is pending.
    #[inline(always)]
    pub fn is_pending<const LINE: u8>(&self) -> bool {
        self.exti.intfr.read_bits() & (1 << LINE) != 0
    }

    /// Clear the pending flag for `LINE`.
    #[inline(always)]
    pub fn clear_pending<const LINE: u8>(&mut self) {
        self.exti.intfr.write_bits(1 << LINE);
    }

    /// Trigger `LINE` by software.
    #[inline(always)]
    pub fn trigger<const LINE: u8>(&mut self) {
        self.exti.swievr.write_bits(1 << LINE);
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::EXTI {
        self.exti
    }
}
