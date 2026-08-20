//! Minimal SysTick interrupt test — NO RTOS.
//!
//! Configures the SysTick timer exactly like the WCH SDK `SYSTICK_Init_Config`
//! (CMP, CTLR=0xF, PFIC-enable vector 12) and logs from the `SysTick` handler.
//! If the interrupt fires we see periodic `SysTick!` logs.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use ch32x035::rt::peripheral::{PFIC, SYST};
use ch32x035::{CorePeripherals, main, pac};

/// Actual tick work. A normal `extern "C"` function (`ret` returns back to
/// the trampoline).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SysTick_impl() {
    static mut COUNT: u32 = 0;
    unsafe {
        let mut syst = SYST::steal();
        syst.clear_flag();
        // Mimic the RTOS `on_tick` critical section, to check whether the
        // interrupt still re-fires after `mret`.
        ch32x035::interrupt::free(|_| {});
        COUNT = COUNT.wrapping_add(1);
        let c = COUNT;
        info!("SysTick! count={}", c);
    }
}

/// Vector-table entry: call the work function, then return from interrupt
/// with `mret` (restores `mstatus.MIE` — the `WCH-Interrupt-fast` return
/// protocol). Without this the interrupt fires only once.
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

#[main]
fn main() -> ! {
    // Same Board::init (Rcc::freeze) as rtos_min:
    let dp = ch32x035::pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        ..
    } = dp;
    let board = ch32x035_examples::board::Board::init(RCC, GPIOA, GPIOB, GPIOC);

    let cp = CorePeripherals::take().unwrap();
    let mut syst = cp.SYST;
    info!("systick_test (replicating rtos start) starting");

    // Replicate rtos::start exactly: cs -> tick::init -> cs
    ch32x035::interrupt::free(|_| {}); // like start step 1

    // tick::init
    syst.set_compare(board.clocks.sysclk_hz() / 1000);
    syst.clear_flag();
    unsafe { (*SYST::PTR).ctlr.write_bits(0xF) }; // STE|STIE|STCLK|STRE
    PFIC::set_priority(12, 15);
    unsafe { PFIC::enable(12) };

    ch32x035::interrupt::free(|_| {}); // like start step 3

    info!("configured, wfi loop");
    loop {
        // Does wfi get woken by the SysTick interrupt?
        ch32x035::rt::asm::wfi();
    }
}
