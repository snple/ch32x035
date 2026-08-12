//! mtvec CSR - Machine Trap-Vector Base-Address Register
//!
//! Points to the interrupt vector table. On WCH QingKe cores the low two bits
//! select the vector mode: value `3` selects the WCH "table of handler
//! addresses" vectored mode used by the CH32X035 startup code.

use core::arch::asm;

/// mtvec register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mtvec {
    bits: u32,
}

/// Mask of the vector mode bits
const MODE_MASK: u32 = 0b11;

/// WCH vectored mode: `mtvec.BASE` points to an array of handler addresses.
pub const MODE_WCH_VECTORED: u32 = 0b11;

impl Mtvec {
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

    /// Get the vector table base address (low bits masked off)
    #[inline(always)]
    pub const fn base(self) -> u32 {
        self.bits & !MODE_MASK
    }

    /// Get the vector mode
    #[inline(always)]
    pub const fn mode(self) -> u32 {
        self.bits & MODE_MASK
    }
}

/// Read the mtvec register
#[inline(always)]
pub fn read() -> Mtvec {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mtvec",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mtvec::from_bits(bits)
}

/// Read the raw mtvec value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mtvec",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    bits
}

/// Write the raw mtvec value
///
/// # Safety
///
/// Changing mtvec redirects all trap handling. Ensure the new vector table is
/// valid before writing.
#[inline(always)]
pub unsafe fn write_raw(bits: u32) {
    unsafe {
        asm!(
            "csrw mtvec, {0}",
            in(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
}
