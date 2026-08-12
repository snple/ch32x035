//! Default panic handler (private)
//!
//! Provides a `panic_handler` when the `panic-handler` feature is enabled.
//! The handler disables interrupts and enters an infinite loop after writing
//! the panic message to the defmt log (if enabled).

use core::panic::PanicInfo;

#[cfg(feature = "defmt")]
fn report_panic(info: &PanicInfo) {
    defmt::error!("panic: {}", defmt::Display2Format(info));
}

#[cfg(not(feature = "defmt"))]
fn report_panic(info: &PanicInfo) {
    // No logging available; the message is simply dropped.
    let _ = info;
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::rt::interrupt::disable();
    report_panic(info);
    loop {
        crate::rt::asm::wfi();
    }
}
