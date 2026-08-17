//! FLASH driver
//!
//! Provides flash unlock/lock, page erase, mass erase and word programming.
//! The CH32X035 flash runs from the same 0x00000000 address space as the
//! vector table, so take care not to erase the code currently executing.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::flash::Flash;
//!
//! let mut flash = Flash::init(dp.FLASH);
//! flash.unlock();
//! flash.erase_page(0x0800_0000);
//! flash.program_word(0x0800_0000, 0xDEAD_BEEF);
//! flash.lock();
//! ```

use core::ptr::{read_volatile, write_volatile};

use crate::Steal;
use crate::pac;

/// FLASH unlock key 1
const KEY1: u32 = 0x4567_0123;
/// FLASH unlock key 2
const KEY2: u32 = 0xCDEF_89AB;

/// FLASH error types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timed out waiting for the flash controller to become idle.
    Timeout,
}

/// FLASH driver
pub struct Flash {
    flash: pac::FLASH,
}

impl Flash {
    /// Create a new FLASH driver.
    pub fn init(flash: pac::FLASH) -> Self {
        Self { flash }
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
            flash: unsafe { pac::FLASH::steal() },
        })
    }

    /// Unlock the flash control registers (write the key sequence).
    pub fn unlock(&mut self) {
        self.flash.keyr.write_bits(KEY1);
        self.flash.keyr.write_bits(KEY2);
    }

    /// Lock the flash control registers.
    pub fn lock(&mut self) {
        self.flash.ctlr.set_lock();
    }

    /// Block until the flash controller is idle.
    pub fn wait_busy(&self) {
        crate::spin::spin_wait("flash idle", || !self.flash.statr.bsy());
    }

    /// Wait until the flash controller is idle, returning [`Error::Timeout`]
    /// if it stays busy.
    pub fn try_wait_busy(&self) -> Result<(), Error> {
        if crate::spin::try_spin_wait(|| !self.flash.statr.bsy()) {
            Ok(())
        } else {
            Err(Error::Timeout)
        }
    }

    /// Erase the page containing `addr`.
    ///
    /// The page size on the CH32X035 is 256 bytes; the address is
    /// automatically aligned to the page boundary by the hardware.
    pub fn erase_page(&mut self, addr: u32) {
        self.wait_busy();
        self.flash.ctlr.set_per();
        self.flash.addr.write_bits(addr);
        self.flash.ctlr.set_strt();
        self.wait_busy();
        self.flash.ctlr.clr_per();
    }

    /// Erase the entire flash.
    pub fn erase_all(&mut self) {
        self.wait_busy();
        self.flash.ctlr.set_mer();
        self.flash.ctlr.set_strt();
        self.wait_busy();
        self.flash.ctlr.clr_mer();
    }

    /// Program a 32-bit word at `addr`.
    ///
    /// # Safety
    ///
    /// `addr` must be a valid, aligned flash address in an erased page.
    pub unsafe fn program_word(&mut self, addr: u32, data: u32) {
        self.wait_busy();
        self.flash.ctlr.set_ftpg();
        unsafe {
            write_volatile(addr as *mut u32, data);
        }
        self.flash.ctlr.set_strt();
        self.wait_busy();
        self.flash.ctlr.clr_ftpg();
    }

    /// Read a 32-bit word from `addr`.
    pub fn read_word(&self, addr: u32) -> u32 {
        unsafe { read_volatile(addr as *const u32) }
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::FLASH {
        self.flash
    }
}
