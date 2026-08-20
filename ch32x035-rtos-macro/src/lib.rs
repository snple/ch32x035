//! Proc-macro crate for [`ch32x035-rtos`].
//!
//! Provides the `#[task]` attribute macro that registers a function as a
//! cooperative task. The implementation lives in `task.rs`; see the crate
//! docs of `ch32x035-rtos` for usage.

#![doc = include_str!("../README.md")]

mod task;

use proc_macro::TokenStream;

/// Maximum task priority accepted by `#[task(priority = N)]`.
/// Must stay in sync with `ch32x035_rtos::task_mod::MAX_PRIO`.
pub(crate) const MAX_PRIO_ATTR: u8 = 7;

/// Default stack size (bytes) used by `#[task]` when `stack` is omitted.
/// Must stay in sync with `ch32x035_rtos::task_mod::DEFAULT_STACK_SIZE`.
pub(crate) const DEFAULT_STACK_ATTR: usize = 1024;

/// Register a function as a task.
///
/// ```ignore
/// #[task(priority = 1, name = "blink", stack = 1024)]
/// fn blink() -> ! {
///     loop {
///         // ...
///         ch32x035_rtos::sleep(500);
///     }
/// }
/// ```
///
/// All parameters are optional: `priority` defaults to 0 (lowest),
/// `name` defaults to the function name and `stack` defaults to
/// [`ch32x035_rtos::DEFAULT_STACK_SIZE`].
#[proc_macro_attribute]
pub fn task(args: TokenStream, input: TokenStream) -> TokenStream {
    task::task_impl(args, input)
}
