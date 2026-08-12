//! mscratch CSR - Machine Scratch Register
//!
//! A scratch register available for use by machine-mode trap handlers.

use core::arch::asm;

/// mscratch register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mscratch {
    bits: u32,
}

impl Mscratch {
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

/// Read the mscratch register
#[inline(always)]
pub fn read() -> Mscratch {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mscratch",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mscratch::from_bits(bits)
}

/// Read the raw mscratch value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mscratch",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    bits
}

/// Write the raw mscratch value
///
/// # Safety
/// Writing mscratch changes the value of the scratch register, which may be used by machine-mode trap handlers
#[inline(always)]
pub unsafe fn write_raw(bits: u32) {
    unsafe {
        asm!(
            "csrw mscratch, {0}",
            in(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
}
