//! mepc CSR - Machine Exception Program Counter
//!
//! Holds the address of the instruction that was executing when the last trap
//! occurred. Writing it and executing `mret` resumes execution there.

use core::arch::asm;

/// mepc register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mepc {
    bits: u32,
}

impl Mepc {
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
}

/// Read the mepc register
#[inline(always)]
pub fn read() -> Mepc {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mepc",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mepc::from_bits(bits)
}

/// Read the raw mepc value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mepc",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    bits
}

/// Write the raw mepc value
///
/// # Safety
///
/// Writing mepc changes where a subsequent `mret` resumes.
#[inline(always)]
pub unsafe fn write_raw(bits: u32) {
    unsafe {
        asm!(
            "csrw mepc, {0}",
            in(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
}
