//! Programmable Fast Interrupt Controller (PFIC)
//!
//! The PFIC is WCH's interrupt controller, present on the QingKe V4C core in
//! place of the standard RISC-V PLIC/CLINT. Its register layout resembles an
//! ARM NVIC:
//!
//! - `ISR[8]`  - Interrupt Status (readable, reflects the IENR set)
//! - `IPR[8]`  - Interrupt Pending (readable)
//! - `IENR[8]` - Interrupt Enable (write-only: writing 1 enables)
//! - `IRER[8]` - Interrupt Clear-Enable (write-only: writing 1 disables)
//! - `IPSR[8]` - Interrupt Set-Pending (write-only)
//! - `IPRR[8]` - Interrupt Clear-Pending (write-only)
//! - `IACTR[8]`- Interrupt Active (readable)
//! - `IPRIOR[n]`- one 8-bit priority byte per interrupt
//!
//! Interrupt numbers are the **vector-table indices** (16..=54 for external
//! interrupts), matching the `IRQn_Type` enum in `ch32x035.h` and the PAC
//! `Interrupt` enum. Register index = `irq / 32`, bit = `irq % 32`.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035_rt::peripheral::PFIC;
//!
//! // Enable the USART1 interrupt (vector slot 32)
//! unsafe { PFIC::enable(32); }
//! ```

use core::marker::PhantomData;

use super::{PFIC_BASE, VolatileCell, peripheral};

/// PFIC Register Block (matches `PFIC_Type` in WCH `core_riscv.h`)
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Status Registers (readable)
    pub isr: [VolatileCell<u32>; 8],
    /// Interrupt Pending Registers (readable)
    pub ipr: [VolatileCell<u32>; 8],
    /// Interrupt Threshold Register
    pub ithresdr: VolatileCell<u32>,
    _reserved0: u32,
    /// Interrupt Configuration Register (system reset via CFGR)
    pub cfgr: VolatileCell<u32>,
    /// Global Interrupt Status Register
    pub gisr: VolatileCell<u32>,
    /// VTF Interrupt ID Registers
    pub vtfi: [VolatileCell<u8>; 4],
    _reserved1: [u8; 12],
    /// VTF Interrupt Address Registers
    pub vtfaddr: [VolatileCell<u32>; 4],
    _reserved2: [u8; 0x90],
    /// Interrupt Enable Registers (write-only: write 1 to enable)
    pub ienr: [VolatileCell<u32>; 8],
    _reserved3: [u8; 0x60],
    /// Interrupt Clear-Enable Registers (write-only: write 1 to disable)
    pub irer: [VolatileCell<u32>; 8],
    _reserved4: [u8; 0x60],
    /// Interrupt Set-Pending Registers (write-only)
    pub ipsr: [VolatileCell<u32>; 8],
    _reserved5: [u8; 0x60],
    /// Interrupt Clear-Pending Registers (write-only)
    pub iprr: [VolatileCell<u32>; 8],
    _reserved6: [u8; 0x60],
    /// Interrupt Active Registers (readable)
    pub iactr: [VolatileCell<u32>; 8],
    _reserved7: [u8; 0xE0],
    /// Interrupt Priority Registers (one byte per interrupt)
    pub iprior: [VolatileCell<u8>; 256],
    _reserved8: [u8; 0x810],
    /// System Control Register (SEV bits, WFE mode)
    pub sctlr: VolatileCell<u32>,
}

/// PFIC peripheral
pub struct PFIC {
    _marker: PhantomData<*const ()>,
}

peripheral!(PFIC, RegisterBlock, PFIC_BASE);

unsafe impl Send for PFIC {}

/// A type-safe interrupt number.
///
/// Implemented by the PAC `Interrupt` enum (and any user type that maps to
/// a vector index). The value is the vector-table index (16..=54 for the
/// external interrupts on the CH32X035).
///
/// # Safety
///
/// Implementors must ensure that `number()` always returns a valid interrupt
/// number that does not exceed the PFIC's supported range.
pub unsafe trait InterruptNumber: Copy {
    /// Convert to the hardware interrupt number (vector index)
    fn number(self) -> u16;
}

/// Core exceptions are numbered 0..=15 (handled by the vector table directly),
/// while external interrupts use vector indices 16..=54.
impl PFIC {
    /// Enable an interrupt
    ///
    /// # Safety
    ///
    /// Enabling an interrupt allows its handler to run at any time.
    /// Ensure the handler is properly set up before enabling.
    #[inline]
    pub unsafe fn enable(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).ienr[reg].write_bits(1 << bit);
        }
    }

    /// Disable an interrupt
    #[inline]
    pub fn disable(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).irer[reg].write_bits(1 << bit);
        }
    }

    /// Check if an interrupt is enabled
    #[inline]
    pub fn is_enabled(irq: u16) -> bool {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe { (*Self::PTR).isr[reg].read_bits() & (1 << bit) != 0 }
    }

    /// Set an interrupt as pending
    #[inline]
    pub fn pend(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).ipsr[reg].write_bits(1 << bit);
        }
    }

    /// Clear a pending interrupt
    #[inline]
    pub fn unpend(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).iprr[reg].write_bits(1 << bit);
        }
    }

    /// Check if an interrupt is pending
    #[inline]
    pub fn is_pending(irq: u16) -> bool {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe { (*Self::PTR).ipr[reg].read_bits() & (1 << bit) != 0 }
    }

    /// Check if an interrupt is active (currently being handled)
    #[inline]
    pub fn active(irq: u16) -> bool {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe { (*Self::PTR).iactr[reg].read_bits() & (1 << bit) != 0 }
    }

    /// Set the priority of an interrupt
    ///
    /// Lower values mean higher priority. On the CH32X035 the priority byte
    /// layout depends on interrupt nesting (CSR 0x804 bit 1): with nesting
    /// enabled, bit 7 is the preemption priority and bits 6:5 the sub-priority.
    #[inline]
    pub fn set_priority(irq: u16, priority: u8) {
        unsafe {
            (*Self::PTR).iprior[irq as usize].write_bits(priority);
        }
    }

    /// Get the priority of an interrupt
    #[inline]
    pub fn priority(irq: u16) -> u8 {
        unsafe { (*Self::PTR).iprior[irq as usize].read_bits() }
    }

    /// Initiate a system reset request
    ///
    /// Writes the WCH key value (`0xBEEF_0000 | (1 << 7)`) to the PFIC CFGR
    /// register.
    #[inline]
    pub fn system_reset() -> ! {
        const NVIC_KEY3: u32 = 0xBEEF_0000;
        const CFGR_RESET_BIT: u32 = 1 << 7;
        unsafe {
            (*Self::PTR).cfgr.write_bits(NVIC_KEY3 | CFGR_RESET_BIT);
        }
        // Wait for the reset to take effect
        loop {
            core::hint::spin_loop();
        }
    }

    // =====================================================================
    // Type-safe variants using the InterruptNumber trait
    // =====================================================================

    /// Enable an interrupt (type-safe)
    ///
    /// # Safety
    ///
    /// Enabling an interrupt allows its handler to run at any time.
    #[inline]
    pub unsafe fn unmask<I: InterruptNumber>(interrupt: I) {
        unsafe { Self::enable(interrupt.number()) }
    }

    /// Disable an interrupt (type-safe)
    #[inline]
    pub fn mask<I: InterruptNumber>(interrupt: I) {
        Self::disable(interrupt.number());
    }

    /// Check if an interrupt is enabled (type-safe)
    #[inline]
    pub fn is_enabled_int<I: InterruptNumber>(interrupt: I) -> bool {
        Self::is_enabled(interrupt.number())
    }

    /// Set an interrupt as pending (type-safe)
    #[inline]
    pub fn pend_int<I: InterruptNumber>(interrupt: I) {
        Self::pend(interrupt.number());
    }

    /// Clear a pending interrupt (type-safe)
    #[inline]
    pub fn unpend_int<I: InterruptNumber>(interrupt: I) {
        Self::unpend(interrupt.number());
    }

    /// Check if an interrupt is pending (type-safe)
    #[inline]
    pub fn is_pending_int<I: InterruptNumber>(interrupt: I) -> bool {
        Self::is_pending(interrupt.number())
    }

    /// Check if an interrupt is active (type-safe)
    #[inline]
    pub fn active_int<I: InterruptNumber>(interrupt: I) -> bool {
        Self::active(interrupt.number())
    }

    /// Set the priority of an interrupt (type-safe)
    #[inline]
    pub fn set_priority_int<I: InterruptNumber>(interrupt: I, priority: u8) {
        Self::set_priority(interrupt.number(), priority);
    }

    /// Get the priority of an interrupt (type-safe)
    #[inline]
    pub fn priority_int<I: InterruptNumber>(interrupt: I) -> u8 {
        Self::priority(interrupt.number())
    }
}
