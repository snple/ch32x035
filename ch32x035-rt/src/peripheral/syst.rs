//! SysTick Timer (WCH, 64-bit)
//!
//! The CH32X035 SysTick is a WCH-custom 64-bit count-up timer at
//! 0xE000_F000 (separate from the standard RISC-V `mtime`/CLINT). Its
//! registers:
//!
//! - `CTLR` - control (STE enable, STIE interrupt, STCLK source, STRE reload, MODE)
//! - `SR`   - status (bit 0: count flag, set when CNT reaches CMP)
//! - `CNT`  - 64-bit count value (two 32-bit halves)
//! - `CMP`  - 64-bit compare value (two 32-bit halves)
//!
//! It is typically used for OS ticks or simple delays.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035_rt::peripheral::SYST;
//!
//! let mut syst = SYST::steal();
//! syst.set_compare(48_000_000); // 1 second @ 48 MHz
//! syst.clear_flag();
//! syst.enable_counter();
//! ```

use core::marker::PhantomData;

use super::{SYST_BASE, VolatileCell, peripheral};

/// SysTick Register Block (matches `SysTick_Type` in WCH `core_riscv.h`)
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register
    pub ctlr: VolatileCell<u32>,
    /// Status Register
    pub sr: VolatileCell<u32>,
    /// Count value, low 32 bits
    pub cntl: VolatileCell<u32>,
    /// Count value, high 32 bits
    pub cnth: VolatileCell<u32>,
    /// Compare value, low 32 bits
    pub cmplr: VolatileCell<u32>,
    /// Compare value, high 32 bits
    pub cmphr: VolatileCell<u32>,
}

/// SysTick Timer peripheral
pub struct SYST {
    _marker: PhantomData<*const ()>,
}

peripheral!(SYST, RegisterBlock, SYST_BASE);

unsafe impl Send for SYST {}

// =============================================================================
// Constants
// =============================================================================

/// CTLR bit: System counter enable
const CTLR_STE: u32 = 1 << 0;
/// CTLR bit: System counter interrupt enable
const CTLR_STIE: u32 = 1 << 1;
/// CTLR bit: Clock source select (1 = HCLK, 0 = external)
#[allow(dead_code)]
const CTLR_STCLK: u32 = 1 << 2;
/// CTLR bit: System reload enable (reload CNT from CMP on match)
const CTLR_STRE: u32 = 1 << 3;
/// CTLR bit: System mode
#[allow(dead_code)]
const CTLR_MODE: u32 = 1 << 4;
/// CTLR bit: Software triggered interrupt enable
#[allow(dead_code)]
const CTLR_SWIE: u32 = 1 << 31;

/// SR bit: Count flag (set when CNT reaches CMP; cleared by writing 0, i.e.
/// `SR &= ~(1<<0)` — the WCH timer uses a plain R/W status register, unlike
/// the write-1-to-clear convention of the STM32/Cortex-M SysTick).
const SR_CNTIF: u32 = 1 << 0;

// =============================================================================
// Implementation
// =============================================================================

impl SYST {
    /// Enable the counter
    #[inline]
    pub fn enable_counter(&mut self) {
        unsafe {
            (*Self::PTR).ctlr.modify_bits(|v| v | CTLR_STE);
        }
    }

    /// Disable the counter
    #[inline]
    pub fn disable_counter(&mut self) {
        unsafe {
            (*Self::PTR).ctlr.modify_bits(|v| v & !CTLR_STE);
        }
    }

    /// Check if the counter is enabled
    #[inline]
    pub fn is_counter_enabled(&self) -> bool {
        unsafe { (*Self::PTR).ctlr.read_bits() & CTLR_STE != 0 }
    }

    /// Enable the SysTick interrupt
    #[inline]
    pub fn enable_interrupt(&mut self) {
        unsafe {
            (*Self::PTR).ctlr.modify_bits(|v| v | CTLR_STIE);
        }
    }

    /// Disable the SysTick interrupt
    #[inline]
    pub fn disable_interrupt(&mut self) {
        unsafe {
            (*Self::PTR).ctlr.modify_bits(|v| v & !CTLR_STIE);
        }
    }

    /// Check if the SysTick interrupt is enabled
    #[inline]
    pub fn is_interrupt_enabled(&self) -> bool {
        unsafe { (*Self::PTR).ctlr.read_bits() & CTLR_STIE != 0 }
    }

    /// Enable the reload mode (CNT restarts from CMP on match)
    #[inline]
    pub fn enable_reload(&mut self) {
        unsafe {
            (*Self::PTR).ctlr.modify_bits(|v| v | CTLR_STRE);
        }
    }

    /// Set the compare value (lower 32 bits)
    #[inline]
    pub fn set_compare(&mut self, value: u32) {
        unsafe {
            (*Self::PTR).cmplr.write_bits(value);
            (*Self::PTR).cmphr.write_bits(0);
        }
    }

    /// Get the current count value (64-bit)
    #[inline]
    pub fn get_count(&self) -> u64 {
        unsafe {
            let low = (*Self::PTR).cntl.read_bits() as u64;
            let high = (*Self::PTR).cnth.read_bits() as u64;
            (high << 32) | low
        }
    }

    /// Set the count value (lower 32 bits)
    #[inline]
    pub fn set_count(&mut self, value: u32) {
        unsafe {
            (*Self::PTR).cntl.write_bits(value);
            (*Self::PTR).cnth.write_bits(0);
        }
    }

    /// Clear the count flag
    ///
    /// The flag (SR bit 0) is cleared by writing 0 (`SR &= ~(1<<0)`), matching
    /// the WCH SDK `SysTick->SR &= ~(1<<0)` — NOT write-1-to-clear (STM32).
    #[inline]
    pub fn clear_flag(&mut self) {
        unsafe {
            (*Self::PTR).sr.modify_bits(|v| v & !SR_CNTIF);
        }
    }

    /// Check if the count flag is set (CNT reached CMP)
    #[inline]
    pub fn is_flag(&self) -> bool {
        unsafe { (*Self::PTR).sr.read_bits() & SR_CNTIF != 0 }
    }
}
