//! ESIG (Electronic Signature)
//!
//! Reads the factory-programmed electronic signature: the 96-bit unique
//! device ID and the flash size. No clock is required.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::esig::Esig;
//!
//! let esig = Esig::new(dp.ESIG);
//! let id = esig.unique_id();
//! let flash_kb = esig.flash_size_kb();
//! ```

use crate::Steal;
use crate::pac;

/// Electronic signature driver
pub struct Esig {
    esig: pac::ESIG,
}

impl Esig {
    /// Create a new ESIG driver.
    #[inline]
    pub fn new(esig: pac::ESIG) -> Self {
        Self { esig }
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
            esig: unsafe { pac::ESIG::steal() },
        })
    }

    /// The 96-bit unique device identifier (three 32-bit words).
    #[inline]
    pub fn unique_id(&self) -> [u32; 3] {
        [
            self.esig.uniid1.read_bits(),
            self.esig.uniid2.read_bits(),
            self.esig.uniid3.read_bits(),
        ]
    }

    /// The flash size in kilobytes.
    #[inline(always)]
    pub fn flash_size_kb(&self) -> u16 {
        self.esig.flacap.f_size() as u16
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::ESIG {
        self.esig
    }
}
