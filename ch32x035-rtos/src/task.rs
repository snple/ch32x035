//! 任务描述符、状态与任务表。

use crate::sync::{RawMutex, Semaphore};

/// 内核可容纳的最大任务数（含 idle 任务）。
pub const MAX_TASKS: usize = 8;

/// `#[task]` 未指定 `stack` 时的默认任务栈大小（字节）。
///
/// dev（未优化）构建下 defmt 格式化与普通调用链的栈开销较大，取 2 KiB 以保证
/// 安全；SysTick 使用独立栈，不占用任务栈预算。
pub const DEFAULT_STACK_SIZE: usize = 2048;

/// `#[task]` 可接受的最小栈大小（字节）。
///
/// 至少要容纳上下文帧（13 个寄存器）外加一帧普通调用链。
pub const MIN_STACK_SIZE: usize = 128;

/// 任务优先级范围：0（最低）..= [`MAX_PRIO`]。
pub const MAX_PRIO: u8 = 7;

/// 编译期任务描述符，由 `#[task]` 宏生成，放入 `.rtos.tasks` 链接段。
///
/// `start()` 遍历 `__rtos_tasks_start`..`__rtos_tasks_end` 收集它们并构建
/// 运行期任务表。
#[repr(C)]
pub struct TaskDef {
    /// 任务入口（返回 `!`）。
    pub entry: unsafe extern "C" fn() -> !,
    /// 任务栈基址。
    pub stack: *mut u8,
    /// 任务栈大小（字节）。
    pub stack_size: usize,
    /// 声明的（基础）优先级。
    pub priority: u8,
    /// 调试用任务名。
    pub name: &'static str,
}

// `TaskDef` 由 `#[task]` 宏放入 static，只被调度器读取，跨任务共享是安全的。
unsafe impl Sync for TaskDef {}

/// 任务栈存储，16 字节对齐（RISC-V ABI 的栈对齐要求）。
///
/// `#[task]` 用它声明任务栈。裸 `[u8; N]` 的对齐只有 1，而保存的上下文帧与栈底
/// 金丝雀都按 `u32` 访问，必须保证对齐。
#[repr(align(16))]
pub struct StackStorage<const N: usize>([u8; N]);

impl<const N: usize> StackStorage<N> {
    /// 创建全零栈（供 `static` 初始化，最终落在 `.bss`）。
    pub const fn new() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Default for StackStorage<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// 任务运行期状态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskState {
    /// 在就绪队列中等待。
    Ready,
    /// 正在执行。
    Running,
    /// 阻塞直到某个 tick 截止时间。
    BlockedSleep,
    /// 阻塞等待信号量/互斥（带可选截止时间）。
    BlockedSem,
}

/// 阻塞任务的等待原因。
#[derive(Clone, Copy)]
pub(crate) enum Wait {
    /// 睡眠到 `deadline` tick（绝对时间，回绕安全比较）。
    Sleep(u32),
    /// 等待信号量。
    Sem {
        /// 所等待的信号量（内核不拥有它，但要求其为 `'static`）。
        sem: *const Semaphore,
        /// 超时绝对 tick；`None` 表示永不超时。
        deadline: Option<u32>,
        /// `give` 是否已授予本次等待的计数。
        granted: bool,
    },
    /// 等待互斥锁。与 [`Wait::Sem`] 分开记录，优先级继承才能沿等待链传播。
    Mutex {
        /// 所等待的互斥锁（要求为 `'static`）。
        m: *const RawMutex,
        /// 超时绝对 tick；`None` 表示永不超时。
        deadline: Option<u32>,
        /// 是否已获授该锁。
        granted: bool,
    },
}

/// 运行期任务控制块。
pub struct Task {
    /// 保存的栈指针（任务非 `Running` 时有意义）。
    pub(crate) sp: *mut u32,
    /// 任务栈底（金丝雀校验用）。
    pub(crate) stack_bottom: *mut u8,
    /// 任务栈大小。
    pub(crate) stack_size: usize,
    /// 声明的（基础）优先级。
    pub(crate) base_prio: u8,
    /// 有效优先级（互斥优先级继承时可能被提升）。
    pub(crate) cur_prio: u8,
    /// 任务名。
    pub(crate) name: &'static str,
    /// 当前调度状态。
    pub(crate) state: TaskState,
    /// 阻塞原因（`Blocked*` 时有意义）。
    pub(crate) wait: Option<Wait>,
    /// 该任务当前持有的互斥锁链表头（用于重算有效优先级）。
    pub(crate) held: *const RawMutex,
}

// `Task` 只存在于单核调度器内，且在临界区中访问；其中的裸指针在此上下文中安全。
unsafe impl Send for Task {}

/// 单个任务的只读快照，用于调试和日志。
#[derive(Clone, Copy, Debug)]
pub struct TaskInfo {
    /// 任务名。
    pub name: &'static str,
    /// 当前调度状态。
    pub state: TaskState,
    /// 声明的（基础）优先级。
    pub base_prio: u8,
    /// 有效优先级（互斥优先级继承时可能被提升）。
    pub cur_prio: u8,
}
