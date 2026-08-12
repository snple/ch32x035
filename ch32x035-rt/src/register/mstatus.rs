//! mstatus CSR - Machine Status Register
//!
//! Holds the global interrupt-enable bits and privilege-level state.

use core::arch::asm;

/// mstatus register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mstatus {
    bits: u32,
}

/// mstatus field bit positions
const MIE_BIT: u32 = 3;
const MPIE_BIT: u32 = 7;
const MPP_SHIFT: u32 = 11;
const MPP_MASK: u32 = 0b11 << MPP_SHIFT;

impl Mstatus {
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

    /// Check if machine interrupts are globally enabled (MIE)
    #[inline(always)]
    pub const fn mie(self) -> bool {
        (self.bits & (1 << MIE_BIT)) != 0
    }

    /// Check if a machine interrupt was pending before the last trap (MPIE)
    #[inline(always)]
    pub const fn mpie(self) -> bool {
        (self.bits & (1 << MPIE_BIT)) != 0
    }

    /// Get the machine previous-privilege mode (MPP)
    #[inline(always)]
    pub const fn mpp(self) -> u32 {
        (self.bits & MPP_MASK) >> MPP_SHIFT
    }

    /// Set the machine interrupt-enable bit
    #[inline(always)]
    pub const fn with_mie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | (1 << MIE_BIT)
            } else {
                self.bits & !(1 << MIE_BIT)
            },
        }
    }

    /// Set the machine previous-interrupt-enable bit
    #[inline(always)]
    pub const fn with_mpie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | (1 << MPIE_BIT)
            } else {
                self.bits & !(1 << MPIE_BIT)
            },
        }
    }

    /// Set the machine previous-privilege mode (MPP)
    #[inline(always)]
    pub const fn with_mpp(self, mode: u32) -> Self {
        Self {
            bits: (self.bits & !MPP_MASK) | ((mode << MPP_SHIFT) & MPP_MASK),
        }
    }
}

/// Read the mstatus register
#[inline(always)]
pub fn read() -> Mstatus {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mstatus",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mstatus::from_bits(bits)
}

/// Read the raw mstatus value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mstatus",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    bits
}

/// Write the raw mstatus value
///
/// # Safety
///
/// Writing mstatus changes interrupt masking and privilege state, which can
/// affect system behavior. Ensure proper nesting with `read_raw()` for
/// restore.
#[inline(always)]
pub unsafe fn write_raw(bits: u32) {
    unsafe {
        asm!(
            "csrw mstatus, {0}",
            in(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
}
