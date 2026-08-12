//! Procedural macros for ch32x035
//!
//! This crate provides the following macros:
//!
//! - `#[main]` - Entry point for CH32X035 applications
//! - `#[exception]` - RISC-V machine exception handler
//! - `#[bind(IRQ)]` - Bind a function to an interrupt
//! - `#[ramfunc]` - Mark a function to execute from RAM

use proc_macro::TokenStream;

mod bind;
mod exception;
mod main_impl;
mod ramfunc;

// ---------------------------------------------------------------------------
// #[main]
// ---------------------------------------------------------------------------

/// Attribute to declare the entry point of the program
///
/// This macro generates the `main` entry point that the runtime calls after
/// initialization, adding CH32X035-specific initialization code.
///
/// ## Initialization performed:
///
/// Calls `ch32x035::startup::init()` before running the user code.
///
/// ## Usage
///
/// ```ignore
/// use ch32x035::main;
///
/// #[main]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// ## Note
///
/// The entry function must have the signature `fn() -> !` (never returns).
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    main_impl::main_impl(args, input)
}

// ---------------------------------------------------------------------------
// #[exception]
// ---------------------------------------------------------------------------

/// Attribute to declare a RISC-V machine exception handler
///
/// Supported exceptions:
///
/// - `DefaultHandler` - Called when no specific handler is defined
/// - `NonMaskableInt` - Non-maskable interrupt handler
/// - `HardFault` - Exception handler (machine exception)
/// - `EcallMMode` - Ecall from machine mode
/// - `EcallUMode` - Ecall from user mode
/// - `BreakPoint` - Breakpoint handler
/// - `SysTick` - System tick timer handler
/// - `Software` - Software interrupt handler
///
/// ## Usage
///
/// ```ignore
/// use ch32x035::exception;
///
/// #[exception]
/// fn SysTick() {
///     // Handle SysTick interrupt
/// }
/// ```
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    exception::exception_impl(args, input)
}

// ---------------------------------------------------------------------------
// #[bind(IRQ)]
// ---------------------------------------------------------------------------

/// Attribute to bind a function to an interrupt
///
/// This macro exports the function with the correct symbol name so the
/// linker's `device.x` weak-provide is overridden and the vector table entry
/// points to this handler.
///
/// ## Usage
///
/// ```ignore
/// use ch32x035::bind;
///
/// #[bind(USART1_IRQ)]
/// fn on_usart1() {
///     // Handle USART1 interrupt
/// }
/// ```
#[proc_macro_attribute]
pub fn bind(args: TokenStream, input: TokenStream) -> TokenStream {
    bind::bind_impl(args, input)
}

// ---------------------------------------------------------------------------
// #[ramfunc]
// ---------------------------------------------------------------------------

/// Attribute to mark a function to execute from RAM
///
/// The function is placed in the `.ramfunc` section, which the startup code
/// copies from flash to RAM before calling `main`.
#[proc_macro_attribute]
pub fn ramfunc(args: TokenStream, input: TokenStream) -> TokenStream {
    ramfunc::ramfunc_impl(args, input)
}
