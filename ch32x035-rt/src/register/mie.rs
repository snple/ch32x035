//! mie CSR - Machine Interrupt-Enable Register
//!
//! On the CH32X035 individual interrupt sources are enabled via the PFIC, but
//! the standard `mie` CSR bits (MEIE/MTIE/MSIE) are still present and used by
//! the SysTick timer.

use core::arch::asm;

/// mie register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mie {
    bits: u32,
}

impl Mie {
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

    /// Machine software interrupt enable (MSIE, bit 3)
    #[inline(always)]
    pub const fn msie(self) -> bool {
        (self.bits & (1 << 3)) != 0
    }

    /// Machine timer interrupt enable (MTIE, bit 7)
    #[inline(always)]
    pub const fn mtie(self) -> bool {
        (self.bits & (1 << 7)) != 0
    }

    /// Machine external interrupt enable (MEIE, bit 11)
    #[inline(always)]
    pub const fn meie(self) -> bool {
        (self.bits & (1 << 11)) != 0
    }

    /// Enable or disable the machine timer interrupt
    #[inline(always)]
    pub const fn with_mtie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | (1 << 7)
            } else {
                self.bits & !(1 << 7)
            },
        }
    }
}

/// Read the mie register
#[inline(always)]
pub fn read() -> Mie {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mie",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    Mie::from_bits(bits)
}

/// Read the raw mie value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!(
            "csrr {0}, mie",
            out(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
    bits
}

/// Write the raw mie value
///
/// # Safety
///
/// Writing mie changes which interrupts can fire. Ensure the corresponding
/// handlers are configured first.
#[inline(always)]
pub unsafe fn write_raw(bits: u32) {
    unsafe {
        asm!(
            "csrw mie, {0}",
            in(reg) bits,
            options(nomem, nostack, preserves_flags)
        );
    }
}
