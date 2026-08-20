//! SysTick timer configuration and the tick interrupt handler.
//!
//! The WCH SysTick is configured to fire every 1 ms. The `SysTick` vector
//! entry calls [`SysTick_impl`] and returns with `mret` (the WCH interrupt
//! protocol), which advances the global tick and wakes any tasks whose
//! deadline has passed.
//!
//! # Interrupt path (WCH-specific)
//!
//! On the CH32X035 the SysTick timer interrupt (vector index 12) is enabled
//! through the **PFIC** (`NVIC_EnableIRQ(SysTick_IRQn)` in the WCH SDK), not
//! through the standard `mie` CSR. Also, handlers must return with **`mret`**
//! (like the SDK's `interrupt("WCH-Interrupt-fast")` attribute) so that
//! `mstatus.MIE` is restored; a plain `ret` only lets the interrupt fire
//! once.

use ch32x035::rt::peripheral::{PFIC, SYST};

/// SysTick IRQ number (vector-table index 12), per the WCH SDK.
const SYSTICK_IRQ: u16 = 12;

/// Configure the SysTick to interrupt every 1 ms.
///
/// `sysclk_hz` is the system clock frequency (e.g. 48 MHz after reset).
/// Mirrors the WCH SDK `SYSTICK_Init_Config`: `CTLR = 0xF`
/// (STE|STIE|STCLK|STRE), then enable the SysTick interrupt in the PFIC.
pub(crate) fn init(mut syst: SYST, sysclk_hz: u32) {
    let period = (sysclk_hz / 1000).max(1);
    syst.set_compare(period);
    syst.clear_flag();
    // CTLR = 0xF: counter enable, interrupt enable, HCLK clock source, reload.
    unsafe { (*SYST::PTR).ctlr.write_bits(0xF) };

    // The SysTick interrupt must be enabled in the PFIC (lowest priority).
    PFIC::set_priority(SYSTICK_IRQ, 15);
    unsafe { PFIC::enable(SYSTICK_IRQ) };
}

/// Tick work, called from the `SysTick` trampoline.
///
/// Runs in machine-interrupt context (interrupts disabled, QingKe hardware
/// stack active), so it must not call any blocking/yielding API.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SysTick_impl() {
    let mut syst = unsafe { SYST::steal() };
    syst.clear_flag();
    let now = crate::scheduler::tick_inc();
    crate::scheduler::on_tick(now);
}

// Vector-table entry for SysTick: call the work function, then return from
// the interrupt with `mret` (restores `mstatus.MIE`, the `WCH-Interrupt-fast`
// protocol). Without `mret` the interrupt fires only once.
core::arch::global_asm!(
    r#"
    .align 2
    .global SysTick
    .type SysTick, @function
    SysTick:
        call SysTick_impl
        mret
    "#
);
