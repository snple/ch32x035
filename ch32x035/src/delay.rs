//! Delay primitives
//!
//! [`Delay`] provides busy-wait delays based on the WCH SysTick timer
//! (count-up with compare), following the `Delay_Us`/`Delay_Ms` sequence from
//! the WCH SDK.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Delay, DelayUs, CorePeripherals};
//!
//! let cp = CorePeripherals::take().unwrap();
//! let mut delay = Delay::init(cp.SYST, 48_000_000);
//! delay.delay_ms(500);
//! ```

use crate::rt::peripheral::SYST;
use crate::rt::peripheral::syst::RegisterBlock;

// WCH SysTick control/status bits (see the SVD STK_CTLR / WCH core_riscv.h)
const CTLR_STE: u32 = 1 << 0; // System counter enable
const CTLR_MODE: u32 = 1 << 4; // System mode (set by WCH Delay_Us)
const CTLR_DELAY: u32 = 1 << 5; // Additional bit set by WCH Delay_Us
const SR_CNTIF: u32 = 1 << 0; // Count flag (CNT reached CMP)

/// A trait for types that can delay a given number of microseconds.
pub trait DelayUs {
    /// Delay for `us` microseconds
    fn delay_us(&self, us: u32);

    /// Delay for `ms` milliseconds (default: repeated microsecond delays)
    fn delay_ms(&self, ms: u32) {
        for _ in 0..ms {
            self.delay_us(1000);
        }
    }

    /// Delay for `secs` seconds
    fn delay_secs(&self, secs: u32) {
        for _ in 0..secs {
            self.delay_ms(1000);
        }
    }
}

/// Busy-wait delay using the WCH SysTick timer.
pub struct Delay {
    #[allow(dead_code)] // kept for ownership semantics
    syst: SYST,
    cycles_per_us: u32,
}

impl Delay {
    /// Create a new delay driver from the SysTick timer.
    ///
    /// `sysclk_hz` is the current system clock frequency (e.g. 48 MHz after
    /// reset on the CH32X035).
    pub fn init(syst: SYST, sysclk_hz: u32) -> Self {
        let delay = Self {
            syst,
            cycles_per_us: (sysclk_hz / 1_000_000).max(1),
        };
        delay.disable_counter();
        delay
    }

    fn regs(&self) -> &RegisterBlock {
        unsafe { &*SYST::PTR }
    }

    fn disable_counter(&self) {
        let r = self.regs();
        r.ctlr.modify_bits(|v| v & !CTLR_STE);
    }

    /// Block for `cycles` system-clock cycles using the SysTick compare timer.
    fn busy_wait_cycles(&self, cycles: u32) {
        let r = self.regs();
        r.sr.write_bits(SR_CNTIF); // clear the count flag
        r.cmplr.write_bits(cycles);
        r.cmphr.write_bits(0);
        r.ctlr
            .modify_bits(|v| v | CTLR_MODE | CTLR_DELAY | CTLR_STE);
        while r.sr.read_bits() & SR_CNTIF == 0 {
            core::hint::spin_loop();
        }
        r.ctlr.modify_bits(|v| v & !CTLR_STE);
    }
}

impl DelayUs for Delay {
    fn delay_us(&self, us: u32) {
        let cycles = (us as u64 * self.cycles_per_us as u64) as u32;
        self.busy_wait_cycles(cycles);
    }
}
