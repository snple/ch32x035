//! 任务描述符、状态与任务表。

/// 内核可容纳的最大任务数（含 idle 任务）。
pub const MAX_TASKS: usize = 8;

/// `#[task]` 未指定 `stack` 时的默认任务栈大小（字节）。
///
/// dev（未优化）构建下 defmt 格式化与普通调用链的栈开销较大，取 2 KiB 以保证
/// 安全；ISR 使用独立栈，不占用任务栈预算。
pub const DEFAULT_STACK_SIZE: usize = 2048;

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
pub enum Wait {
    /// 睡眠到 `deadline`（含）tick。
    Sleep(u32),
    /// 等待信号量到 `deadline`（0 = 永远）。
    Sem {
        /// 所等待信号量的裸指针（信号量须为 `'static`）。
        sem: *const (),
        /// 超时 tick 截止时间；0 表示永不超时。
        deadline: u32,
        /// `give` 是否已授予本次等待的计数。
        granted: bool,
    },
}

/// 运行期任务控制块。
pub struct Task {
    /// 保存的栈指针（任务非 `Running` 时有意义）。
    pub sp: *mut u32,
    /// 任务栈底（金丝雀校验用）。
    pub stack_bottom: *mut u8,
    /// 任务栈大小。
    pub stack_size: usize,
    /// 声明的（基础）优先级。
    pub base_prio: u8,
    /// 有效优先级（互斥优先级继承时可能被提升）。
    pub cur_prio: u8,
    /// 任务名。
    pub name: &'static str,
    /// 当前调度状态。
    pub state: TaskState,
    /// 阻塞原因（`Blocked*` 时有意义）。
    pub wait: Option<Wait>,
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
