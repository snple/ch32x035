//! [`ch32x035-rtos`] 的过程宏 crate。
//!
//! 提供把函数注册为协作任务的 `#[task]` 属性宏。实现见 `task.rs`。

mod task;

use proc_macro::TokenStream;

/// `#[task(priority = N)]` 接受的最大任务优先级。
/// 必须与 `ch32x035_rtos::task_mod::MAX_PRIO` 保持一致。
pub(crate) const MAX_PRIO_ATTR: u8 = 7;

/// `#[task]` 未指定 `stack` 时的默认栈大小（字节）。
/// 必须与 `ch32x035_rtos::task_mod::DEFAULT_STACK_SIZE` 保持一致。
pub(crate) const DEFAULT_STACK_ATTR: usize = 2048;

/// `#[task]` 可接受的最小栈大小（字节）。
/// 必须与 `ch32x035_rtos::task_mod::MIN_STACK_SIZE` 保持一致。
pub(crate) const MIN_STACK_ATTR: usize = 128;

/// 把函数注册为任务。
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
/// 三个参数均可选：`priority` 默认 0（最低），`name` 默认为函数名，`stack` 默认为
/// [`ch32x035_rtos::task_mod::DEFAULT_STACK_SIZE`]。
#[proc_macro_attribute]
pub fn task(args: TokenStream, input: TokenStream) -> TokenStream {
    task::task_impl(args, input)
}
