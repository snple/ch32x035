//! Runtime startup code for the CH32X035 (RISC-V, WCH QingKe V4C)
//!
//! This module provides the reset handler and vector table. The actual
//! startup sequence mirrors the WCH `startup_ch32x035.S`:
//!
//! 1. `_start` (in `.init`, at the flash base) sets the stack pointer and
//!    jumps to `Reset`.
//! 2. `Reset` copies `.data` / `.ramfunc`, zeroes `.bss`, configures the WCH
//!    core (pipelining, interrupt nesting + hardware stack), enables global
//!    interrupts, points `mtvec` at the vector table (WCH vectored mode),
//!    then calls `main`.

use core::arch::global_asm;

// The very first code executed at reset: set the stack pointer then jump to
// the Rust `Reset` function. The linker places `.init` at the flash base.
//
// NOTE: the stack is set up in assembly here because the Rust `Reset` function
// may use the stack in its prologue before we could set `sp` from Rust.
global_asm!(
    r#"
    .section .init,"ax",@progbits
    .align 1
    .global _start
    _start:
        la sp, _stack_start
        j Reset
    "#
);

// External symbols from the linker script
unsafe extern "C" {
    // Data section
    static __sidata: u32; // LMA of .data
    static mut __sdata: u32; // Start of .data in RAM
    static mut __edata: u32; // End of .data in RAM

    // RAM function section
    static __siramfunc: u32; // LMA of .ramfunc
    static mut __sramfunc: u32; // Start of .ramfunc in RAM
    static mut __eramfunc: u32; // End of .ramfunc in RAM

    // BSS section
    static mut __sbss: u32; // Start of .bss
    static mut __ebss: u32; // End of .bss

    // Stack
    static _stack_start: u32;

    // Vector table (start of `.vector_table`, defined in link.x)
    static __vector_table: u32;

    // User entry point
    fn main() -> !;

    // Pre-init function
    fn __pre_init();
}

/// Reset handler - called on power-on or reset
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    // Initialize .data section (copy from flash to RAM)
    unsafe {
        let mut src = &raw const __sidata;
        let mut dst = &raw mut __sdata;
        let end = &raw mut __edata;

        while dst < end {
            core::ptr::write_volatile(dst, core::ptr::read_volatile(src));
            src = src.offset(1);
            dst = dst.offset(1);
        }
    }

    // Copy .ramfunc section (code to run from RAM)
    unsafe {
        let mut src = &raw const __siramfunc;
        let mut dst = &raw mut __sramfunc;
        let end = &raw mut __eramfunc;

        while dst < end {
            core::ptr::write_volatile(dst, core::ptr::read_volatile(src));
            src = src.offset(1);
            dst = dst.offset(1);
        }
    }

    // Zero .bss section
    unsafe {
        let mut dst = &raw mut __sbss;
        let end = &raw mut __ebss;

        while dst < end {
            core::ptr::write_volatile(dst, 0);
            dst = dst.offset(1);
        }
    }

    // ── WCH core configuration (mirrors startup_ch32x035.S) ─────────────
    unsafe {
        // Configure pipelining and instruction prediction (WCH custom CSR 0xbc0)
        core::arch::asm!(
            "csrw 0xbc0, {0}",
            in(reg) 0x1f_u32,
            options(nomem, nostack, preserves_flags)
        );

        // Enable interrupt nesting and hardware stack (WCH custom CSR 0x804)
        core::arch::asm!(
            "csrw 0x804, {0}",
            in(reg) 0x3_u32,
            options(nomem, nostack, preserves_flags)
        );

        // Enable global interrupts + previous-global flag
        // (mstatus MIE | MPIE = 0x88, same as WCH `__enable_irq`)
        core::arch::asm!(
            "csrw mstatus, {0}",
            in(reg) 0x88_u32,
            options(nomem, nostack, preserves_flags)
        );

        // Point mtvec at the vector table in WCH vectored mode (mode = 3)
        let vector_table = &raw const __vector_table as u32;
        core::arch::asm!(
            "csrw mtvec, {0}",
            in(reg) vector_table | 3,
            options(nomem, nostack, preserves_flags)
        );

        // Ensure any copied code (.ramfunc) is visible to instruction fetch
        core::arch::asm!("fence.i", options(nomem, nostack, preserves_flags));
    }

    // Call user's main function
    unsafe { __pre_init() }
    unsafe { main() }
}

/// Reset vector (entry point address) - vector table entry 0
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// Exception/interrupt handler type
pub union Vector {
    /// Exception handler function
    pub handler: unsafe extern "C" fn(),
    /// Reserved entry (set to 0)
    pub reserved: usize,
}

/// Core exception vectors (vector table entries 1-15)
///
/// Entry 0 is `__RESET_VECTOR`. Entries 16-54 (external interrupts) are
/// provided by the PAC (`__INTERRUPTS`).
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static __EXCEPTIONS: [Vector; 15] = [
    // 1: Reserved
    Vector { reserved: 0 },
    // 2: NMI
    Vector {
        handler: NonMaskableInt,
    },
    // 3: HardFault (exception)
    Vector { handler: HardFault },
    // 4: Reserved
    Vector { reserved: 0 },
    // 5: Ecall M-mode
    Vector { handler: EcallMMode },
    // 6: Reserved
    Vector { reserved: 0 },
    // 7: Reserved
    Vector { reserved: 0 },
    // 8: Ecall U-mode
    Vector { handler: EcallUMode },
    // 9: Break Point
    Vector { handler: BreakPoint },
    // 10: Reserved
    Vector { reserved: 0 },
    // 11: Reserved
    Vector { reserved: 0 },
    // 12: SysTick
    Vector { handler: SysTick },
    // 13: Reserved
    Vector { reserved: 0 },
    // 14: Software interrupt
    Vector { handler: Software },
    // 15: Reserved
    Vector { reserved: 0 },
];

/// Default pre-init function (does nothing)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultPreInit() {}

/// Default exception handler (infinite loop)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler_() {
    loop {
        crate::asm::wfi();
    }
}

/// Default HardFault handler (infinite loop)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HardFault_() {
    loop {
        crate::asm::wfi();
    }
}

// Core exception handler declarations (weak symbols, provided by link.x /
// overridden by the user via the `#[exception]` macro)
unsafe extern "C" {
    fn NonMaskableInt();
    fn HardFault();
    fn EcallMMode();
    fn EcallUMode();
    fn BreakPoint();
    fn SysTick();
    fn Software();
}
