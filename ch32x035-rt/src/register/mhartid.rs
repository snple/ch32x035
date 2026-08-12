//! mhartid CSR - Machine Hardware Thread ID
//!
//! Returns the hardware thread (core) ID. Always 0 on the single-core
//! CH32X035.

use core::arch::asm;

/// mhartid register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mhartid {
    bits: u32,
}

impl Mhartid {
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

/// Read the mhartid register
#[inline(always)]
pub fn read() -> Mhartid {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mhartid",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mhartid::from_bits(bits)
}
