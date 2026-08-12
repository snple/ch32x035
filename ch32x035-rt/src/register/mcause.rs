//! mcause CSR - Machine Cause Register
//!
//! Describes the cause of the last trap. The most significant bit (bit 31)
//! distinguishes interrupts from exceptions; the remaining bits hold the
//! exception/interrupt code.

use core::arch::asm;

/// mcause register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mcause {
    bits: u32,
}

/// Bit 31: 1 = interrupt, 0 = exception
const INTERRUPT_BIT: u32 = 1 << 31;

impl Mcause {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get raw bits
    #[inline(always)]
    pub const fn bits(self) -> u32 {
        self.bits
    }

    /// Check if the trap was an interrupt (as opposed to an exception)
    #[inline(always)]
    pub const fn is_interrupt(self) -> bool {
        (self.bits & INTERRUPT_BIT) != 0
    }

    /// Get the exception/interrupt code (low 31 bits)
    #[inline(always)]
    pub const fn code(self) -> u32 {
        self.bits & !INTERRUPT_BIT
    }
}

/// Read the mcause register
#[inline(always)]
pub fn read() -> Mcause {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mcause",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mcause::from_bits(bits)
}

/// Read the raw mcause value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mcause",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    bits
}
