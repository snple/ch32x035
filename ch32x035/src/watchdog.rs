//! Watchdog drivers
//!
//! - [`Iwdg`]: independent watchdog (free-running, clocked by the LSI)
//! - [`Wwdg`]: window watchdog (clocked by APB1)
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::watchdog::Iwdg;
//!
//! let mut iwdg = Iwdg::init(dp.IWDG, 0b110, 0xFFF);
//! loop {
//!     // ...
//!     iwdg.reload();
//! }
//! ```

use crate::pac;
use crate::rcc::Rcc;
use crate::reset::Resettable;

/// IWDG access key values
const IWDG_KEY_UNLOCK: u32 = 0x5555;
const IWDG_KEY_RELOAD: u32 = 0xAAAA;
const IWDG_KEY_START: u32 = 0xCCCC;

/// Independent watchdog driver
pub struct Iwdg {
    iwdg: pac::IWDG,
}

impl Iwdg {
    /// Create and start the independent watchdog.
    ///
    /// `prescaler` is the `PSCR` field value (0..=7); `reload` is the 12-bit
    /// reload value. The timeout is approximately
    /// `reload * 2^(prescaler+2) / LSI_HZ` seconds.
    pub fn init(iwdg: pac::IWDG, prescaler: u8, reload: u16) -> Self {
        let mut w = Self { iwdg };
        w.set_prescaler(prescaler);
        w.set_reload(reload);
        // Start the watchdog
        w.iwdg.ctlr.write_bits(IWDG_KEY_START);
        w
    }

    /// Reload the watchdog counter (feed the dog).
    pub fn reload(&self) {
        self.iwdg.ctlr.write_bits(IWDG_KEY_RELOAD);
    }

    /// Set the prescaler (requires unlocking; only valid before starting).
    pub fn set_prescaler(&mut self, prescaler: u8) {
        self.iwdg.ctlr.write_bits(IWDG_KEY_UNLOCK);
        self.iwdg.pscr.write_pr((prescaler & 0b111) as u32);
    }

    /// Set the reload value (requires unlocking; only valid before starting).
    pub fn set_reload(&mut self, reload: u16) {
        self.iwdg.ctlr.write_bits(IWDG_KEY_UNLOCK);
        self.iwdg.rldr.write_rl((reload & 0x0FFF) as u32);
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::IWDG {
        self.iwdg
    }
}

/// Window watchdog driver
pub struct Wwdg {
    wwdg: pac::WWDG,
}

impl Wwdg {
    /// Create and start the window watchdog.
    ///
    /// `prescaler` (0..=3) selects the `WDGTB` timebase; `window` is the
    /// 7-bit window value and `counter` the 7-bit initial counter value.
    pub fn init(wwdg: pac::WWDG, rcc: &mut Rcc, prescaler: u8, window: u8, counter: u8) -> Self {
        wwdg.init(rcc);

        let w = Self { wwdg };
        w.wwdg.cfgr.write_wdgtb((prescaler & 0b11) as u32);
        w.wwdg.cfgr.write_w((window & 0x7F) as u32);
        // Write the counter and enable the watchdog in one go
        w.wwdg.ctlr.write_bits(((counter & 0x7F) as u32) | (1 << 7));
        w
    }

    /// Feed the watchdog (rewrite the counter value).
    pub fn feed(&mut self, counter: u8) {
        let ctlr = self.wwdg.ctlr.read_bits() & !0x7F;
        self.wwdg
            .ctlr
            .write_bits(ctlr | ((counter & 0x7F) as u32));
    }

    /// Enable the early-wakeup interrupt.
    pub fn enable_early_wakeup_interrupt(&mut self) {
        self.wwdg.cfgr.set_ewi();
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::WWDG {
        self.wwdg
    }
}
