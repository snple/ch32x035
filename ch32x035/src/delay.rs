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

// WCH SysTick control/status bits (see CH32X035RM 7.5.5 STK_CTLR).
// These match the WCH SDK (EVT/EXAM/SRC/Debug/debug.c Delay_Us/Delay_Ms):
// down-counting (MODE=1) with INIT reload and STRE=0, running off HCLK/8
// (STCLK=0, the reset default) — the SDK keeps STCLK unset and scales the
// compare value by SystemCoreClock/8e6.
const CTLR_STE: u32 = 1 << 0; // System counter enable
const CTLR_MODE: u32 = 1 << 4; // 1: down counting
const CTLR_INIT: u32 = 1 << 5; // INIT: reload CNT from CMP (RW1)
const SR_CNTIF: u32 = 1 << 0; // Count flag (write 0 to clear, write 1 ignored)

/// A trait for types that can delay a given number of microseconds.
pub trait DelayUs {
    /// Delay for `us` microseconds
    fn delay_us(&self, us: u32);

    /// Delay for `ms` milliseconds (generic fallback; `Delay` overrides this
    /// with the WCH SDK one-shot compare, CMP = n * p_ms).
    fn delay_ms(&self, ms: u32) {
        self.delay_us(ms.saturating_mul(1_000));
    }

    /// Delay for `secs` seconds
    fn delay_secs(&self, secs: u32) {
        self.delay_ms(secs.saturating_mul(1_000));
    }
}

/// Busy-wait delay using the WCH SysTick timer.
///
/// Mirrors the WCH SDK `Delay_Us`/`Delay_Ms` (EVT/EXAM/SRC/Debug/debug.c): the
/// WCH SysTick runs off HCLK/8 by default (STCLK=0), so the compare value is
/// scaled by `p_us = sysclk/8e6` / `p_ms = p_us*1000`.
pub struct Delay {
    #[allow(dead_code)] // kept for ownership semantics
    syst: SYST,
    /// SysTick cycles per microsecond (sysclk / 8e6, HCLK/8 timebase)
    p_us: u32,
    /// SysTick cycles per millisecond (p_us * 1000)
    p_ms: u32,
}

impl Delay {
    /// Create a new delay driver from the SysTick timer.
    ///
    /// `sysclk_hz` is the current system clock frequency (e.g. 48 MHz after
    /// reset on the CH32X035).
    pub fn init(syst: SYST, sysclk_hz: u32) -> Self {
        // WCH SDK Delay_Init: p_us = SystemCoreClock / 8000000.
        let p_us = (sysclk_hz / 8_000_000).max(1);
        let p_ms = p_us * 1_000;
        let delay = Self { syst, p_us, p_ms };
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

    /// Block for `cycles` SysTick cycles using the compare timer.
    ///
    /// Exact WCH SDK sequence (`Delay_Us`/`Delay_Ms` in debug.c):
    /// clear CNTIF (write 0), write CMP, set MODE, then INIT|STE, spin until
    /// CNTIF, finally disable the counter.
    ///
    /// Note: the WCH SysTick `SR` is a plain R/W register — the count flag is
    /// cleared by **writing 0** (`SR &= ~(1<<0)`), NOT by writing 1 (that's the
    /// STM32 Cortex-M convention, which this timer does not follow).
    fn busy_wait(&self, cycles: u32) {
        let r = self.regs();
        r.sr.modify_bits(|v| v & !SR_CNTIF); // clear the count flag (write 0)
        r.cmplr.write_bits(cycles);
        r.cmphr.write_bits(0);
        // Explicitly load CNT from CMP. The SDK relies on the one-shot INIT
        // bit alone; at power-on a stale CNT (left by the Boot ROM, or wrapped
        // to ~2^64 from 0 with STRE=0) can otherwise count down from a huge
        // value and never reach 0, hanging forever.
        r.cntl.write_bits(cycles);
        r.cnth.write_bits(0);
        // WCH SDK: CTLR |= (1<<4); CTLR |= (1<<5)|(1<<0);  (MODE, then INIT+STE)
        r.ctlr.modify_bits(|v| v | CTLR_MODE);
        r.ctlr.modify_bits(|v| v | CTLR_INIT | CTLR_STE);
        while r.sr.read_bits() & SR_CNTIF == 0 {
            core::hint::spin_loop();
        }
        r.ctlr.modify_bits(|v| v & !CTLR_STE);
    }
}

impl DelayUs for Delay {
    fn delay_us(&self, us: u32) {
        self.busy_wait(us.saturating_mul(self.p_us));
    }

    fn delay_ms(&self, ms: u32) {
        // One-shot compare exactly like the WCH SDK Delay_Ms: CMP = n * p_ms.
        self.busy_wait(ms.saturating_mul(self.p_ms));
    }

    fn delay_secs(&self, secs: u32) {
        self.busy_wait(secs.saturating_mul(self.p_ms).saturating_mul(1_000));
    }
}
