//! Minimal demo of the `#[bind(IRQ)]` attribute macro.
//!
//! Binds a handler to `USART1_IRQ` and enables the interrupt.  The USART1
//! peripheral itself is not configured, so the handler never actually fires;
//! this example is primarily a compile/link check that the `#[bind]` expansion
//! exports a `USART1_IRQ` symbol which overrides the weak `device.x` provide.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin bind_demo --release --target riscv32imac-unknown-none-elf
//! ```
//!
//! Check the vector table entry resolves to the user handler:
//! ```sh
//! riscv32-unknown-elf-nm target/riscv32imac-unknown-none-elf/release/bind_demo | grep -i USART1_IRQ
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt_rtt as _;

use ch32x035::{bind, interrupt, main, pac};

static FIRED: AtomicBool = AtomicBool::new(false);

#[bind(USART1_IRQ)]
fn on_usart1() {
    FIRED.store(true, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let _dp = pac::Peripherals::take().unwrap();

    interrupt::set_priority(pac::Interrupt::USART1_IRQ, 0);
    interrupt::enable(pac::Interrupt::USART1_IRQ);

    loop {
        // Placeholder: the interrupt never fires because USART1 is unconfigured.
        let _ = FIRED.load(Ordering::Relaxed);
    }
}
