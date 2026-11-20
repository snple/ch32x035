//! 调度器：任务表、多级就绪队列，以及 `start` / `yield_now` / `sleep` 公共 API。
//!
//! 协作式调度：任务持有 CPU 直到显式让出、睡眠或阻塞在信号量/互斥上。每个让出点
//! 选择最高优先级就绪任务（同优先级 FIFO）。无任务可运行时执行 idle 任务（`wfi`）。

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use ch32x035::interrupt::{self, Mutex as CsMutex};
use ch32x035::rt::peripheral::SYST;

use crate::context;
use crate::sync::{RawMutex, Semaphore};
use crate::task::{MAX_PRIO, MAX_TASKS, StackStorage, Task, TaskDef, TaskInfo, TaskState, Wait};

// ---------------------------------------------------------------------------
// 全局 tick（1 ms/tick，由 SysTick 中断推进）
// ---------------------------------------------------------------------------

static TICK: AtomicU32 = AtomicU32::new(0);

#[inline(always)]
pub(crate) fn tick_now() -> u32 {
    TICK.load(Ordering::Relaxed)
}

/// tick 加一并返回新值。
#[inline(always)]
pub(crate) fn tick_inc() -> u32 {
    TICK.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
}

/// 从当前时刻开始的 `ms` 毫秒后对应的绝对 tick。
#[inline]
fn deadline_after(ms: u32) -> u32 {
    tick_now().wrapping_add(ms)
}

/// WCH 全局中断门的使能位（CSR 0x800，与 HAL 的 `cs_acquire` 使用同一寄存器）。
// `asm_sub_register` 在 RV32 上是误报：`u32` 恰好就是一个寄存器的宽度，
// `{0}` 的默认格式就是正确的。HAL 的 `read_gintenr` 同样如此抑制。
#[allow(asm_sub_register)]
#[inline]
fn global_interrupts_enabled() -> bool {
    let value: u32;
    unsafe {
        core::arch::asm!(
            "csrr {0}, 0x800",
            out(reg) value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value & 0x8 != 0
}

/// 断言当前不处于临界区（全局中断已使能）。
///
/// 在临界区内让出会破坏调用方的临界区语义：切换期间别的任务与中断会看到本应被
/// 保护的状态。这里直接 panic，而不是让系统在别处莫名其妙地挂死。
///
/// 注意：本核心的中断服务程序运行在「全局中断使能」状态下，且 `mcause` 在
/// `mret` 之后不会被清零，因此无法可靠区分「任务上下文」与「ISR 上下文」——
/// 中断里禁止调用阻塞 API 只能靠调用约定保证。
fn assert_can_block(what: &str) {
    assert!(
        global_interrupts_enabled(),
        "ch32x035-rtos: {what} must not be called inside a critical section (interrupts are disabled)"
    );
}

/// `now` 是否已到达（或越过）绝对 tick `deadline`。
///
/// 用回绕安全的差值比较，而不是 `deadline <= now`：后者在 `u32` 回绕点（约 49.7 天）
/// 之后会永远为假，导致睡眠任务永不唤醒。要求超时长度小于 2³¹ ms（约 24.8 天）。
#[inline]
pub(crate) fn deadline_reached(now: u32, deadline: u32) -> bool {
    now.wrapping_sub(deadline) < 0x8000_0000
}

// ---------------------------------------------------------------------------
// 调度器状态
// ---------------------------------------------------------------------------

/// 调度器状态。仅在临界区内访问。
pub(crate) struct Scheduler {
    /// 任务表；`None` = 空槽。
    tasks: [Option<Task>; MAX_TASKS],
    /// 运行中任务的索引。
    current: usize,
    /// idle 任务索引。
    idle: usize,
    /// 每个优先级一个 FIFO 就绪队列，存任务索引。
    ready: [[u8; MAX_TASKS]; (MAX_PRIO + 1) as usize],
    /// 每个就绪队列的长度。
    ready_len: [u8; (MAX_PRIO + 1) as usize],
    /// 非空就绪队列的位图：bit `p` 置位表示优先级 `p` 的队列非空。
    ///
    /// 最高优先级由它直接算出，不再缓存「top_ready」字段——缓存一旦与队列失配，
    /// 就绪任务会永远看不见（空闲任务在 `wfi` 里空转，系统静默卡死）。
    ready_mask: u8,
    /// 已注册任务数（含 idle）。
    task_count: usize,
}

static SCHEDULER: CsMutex<RefCell<Scheduler>> = CsMutex::new(RefCell::new(Scheduler::new()));

impl Scheduler {
    const fn new() -> Self {
        Self {
            tasks: [const { None }; MAX_TASKS],
            current: 0,
            idle: 0,
            ready: [[0u8; MAX_TASKS]; (MAX_PRIO + 1) as usize],
            ready_len: [0u8; (MAX_PRIO + 1) as usize],
            ready_mask: 0,
            task_count: 0,
        }
    }

    /// 把 `id` 追加到优先级 `prio` 的就绪队列末尾。
    #[inline]
    fn enqueue(&mut self, id: usize, prio: u8) {
        let p = prio as usize;
        // 硬断言而不是 debug_assert：release 下越界会直接破坏调度器状态。
        assert!(
            p < self.ready_len.len() && (self.ready_len[p] as usize) < MAX_TASKS,
            "ch32x035-rtos: scheduler ready-queue invariant violated"
        );
        let len = self.ready_len[p] as usize;
        self.ready[p][len] = id as u8;
        self.ready_len[p] += 1;
        self.ready_mask |= 1 << prio;
    }

    /// 最高的非空就绪优先级（`None` = 没有就绪任务）。
    fn top_ready(&self) -> Option<u8> {
        if self.ready_mask == 0 {
            None
        } else {
            Some(self.ready_mask.ilog2() as u8)
        }
    }

    /// 从任意就绪队列中移除 `id`。
    fn dequeue(&mut self, id: usize) {
        for p in 0..=(MAX_PRIO as usize) {
            let len = self.ready_len[p] as usize;
            for i in 0..len {
                if self.ready[p][i] as usize == id {
                    for j in (i + 1)..len {
                        self.ready[p][j - 1] = self.ready[p][j];
                    }
                    self.ready_len[p] -= 1;
                    if self.ready_len[p] == 0 {
                        self.ready_mask &= !(1 << p);
                    }
                    return;
                }
            }
        }
    }

    /// 把任务 `id` 标记为就绪并以 `prio` 入队（保留 `wait` 供 `take` 读取返回值）。
    #[inline]
    fn make_ready(&mut self, id: usize, prio: u8) {
        self.tasks[id].as_mut().expect("task exists").state = TaskState::Ready;
        self.enqueue(id, prio);
    }

    /// 用 `wait` 阻塞运行中的任务。
    #[inline]
    fn block_current(&mut self, wait: Wait) {
        let cur = self.current;
        let t = self.tasks[cur].as_mut().expect("task exists");
        t.wait = Some(wait);
        t.state = match wait {
            Wait::Sleep(_) => TaskState::BlockedSleep,
            Wait::Sem { .. } | Wait::Mutex { .. } => TaskState::BlockedSem,
        };
    }

    /// 从就绪队列取出最高优先级任务。
    fn pick_next(&mut self) -> Option<usize> {
        let p = self.top_ready()? as usize;
        let id = self.ready[p][0] as usize;
        let len = self.ready_len[p] as usize;
        for i in 1..len {
            self.ready[p][i - 1] = self.ready[p][i];
        }
        self.ready_len[p] -= 1;
        if self.ready_len[p] == 0 {
            self.ready_mask &= !(1 << p);
        }
        Some(id)
    }

    /// 选出下一个任务并准备从 `cur` 切换。
    ///
    /// 返回 `Some((旧任务的 sp 槽位, 新任务的 sp))`，无需切换时返回 `None`。
    /// 选中的任务就是 `cur` 本身时**不能**走汇编切换：`Task::sp` 只在切出时写入，
    /// 任务运行期间它是上一次切出留下的旧值，拿它回切会跳回旧栈帧（甚至重跑入口）。
    #[inline]
    fn switch_prep(&mut self, cur: usize) -> Option<(*mut *mut u32, *mut u32)> {
        let n = self.pick_next().expect("idle must always be ready");
        if n == cur {
            self.tasks[cur].as_mut().expect("task exists").state = TaskState::Running;
            return None;
        }
        self.current = n;
        self.tasks[n].as_mut().expect("task exists").state = TaskState::Running;
        let new_sp = self.tasks[n].as_ref().expect("task exists").sp;
        let old_slot = &raw mut self.tasks[cur].as_mut().expect("task exists").sp;
        Some((old_slot, new_sp))
    }

    /// 是否存在除 idle 之外的就绪任务。
    fn has_ready_except_idle(&self) -> bool {
        // 任何优先级 > 0 的就绪任务都不是 idle（idle 固定为优先级 0）。
        if self.ready_mask & !1 != 0 {
            return true;
        }
        // 只有优先级 0 可能有任务；检查其中是否有 idle 之外的任务。
        for i in 0..self.ready_len[0] as usize {
            if self.ready[0][i] as usize != self.idle {
                return true;
            }
        }
        false
    }

    /// 唤醒截止时间已到的任务。
    fn wake_expired(&mut self, now: u32) {
        for i in 0..self.task_count {
            let Some(t) = self.tasks[i].as_ref() else {
                continue;
            };
            let expired = match (t.state, t.wait) {
                (TaskState::BlockedSleep, Some(Wait::Sleep(d))) => deadline_reached(now, d),
                (
                    TaskState::BlockedSem,
                    Some(Wait::Sem {
                        deadline: Some(d),
                        granted: false,
                        ..
                    }),
                )
                | (
                    TaskState::BlockedSem,
                    Some(Wait::Mutex {
                        deadline: Some(d),
                        granted: false,
                        ..
                    }),
                ) => deadline_reached(now, d),
                _ => false,
            };
            if !expired {
                continue;
            }
            let wait = t.wait.expect("wait set");
            let prio = t.cur_prio;
            let t = self.tasks[i].as_mut().expect("task exists");
            if matches!(wait, Wait::Sleep(_)) {
                t.wait = None;
            }
            // 保留 `Wait::Sem` / `Wait::Mutex`（granted = false）供等待方读取超时结果。
            t.state = TaskState::Ready;

            match wait {
                Wait::Sleep(_) => {}
                Wait::Sem { sem, .. } => unsafe { (*sem).remove_waiter(i as u8) },
                Wait::Mutex { m, .. } => {
                    unsafe { (*m).inner.remove_waiter(i as u8) };
                    // 等待者离开后，持有者可能不再需要继承来的优先级。
                    if let Some(o) = unsafe { *(*m).owner.get() } {
                        self.refresh_prio(o as usize);
                    }
                }
            }
            self.enqueue(i, prio);
        }
    }

    /// 从 `.rtos.tasks` 链接段填充任务表并注册 idle 任务。
    fn init_from_defs(&mut self, defs: &[TaskDef]) {
        if defs.len() + 1 > MAX_TASKS {
            panic!(
                "ch32x035-rtos: too many tasks ({}), max is {}",
                defs.len() + 1,
                MAX_TASKS
            );
        }
        for (i, d) in defs.iter().enumerate() {
            let sp = unsafe { context::init_stack_frame(d.entry, d.stack, d.stack_size) };
            self.tasks[i] = Some(Task {
                sp,
                stack_bottom: d.stack,
                stack_size: d.stack_size,
                base_prio: d.priority,
                cur_prio: d.priority,
                name: d.name,
                state: TaskState::Ready,
                wait: None,
                held: core::ptr::null(),
            });
            self.enqueue(i, d.priority);
            self.task_count += 1;
        }

        // idle 任务永远存在，且在没有其它就绪任务时总是就绪。
        let idle = self.task_count;
        let idle_bottom = (&raw mut IDLE_STACK).cast::<u8>();
        let idle_sp =
            unsafe { context::init_stack_frame(idle_entry, idle_bottom, IDLE_STACK_SIZE) };
        self.tasks[idle] = Some(Task {
            sp: idle_sp,
            stack_bottom: idle_bottom,
            stack_size: IDLE_STACK_SIZE,
            base_prio: 0,
            cur_prio: 0,
            name: "idle",
            state: TaskState::Ready,
            wait: None,
            held: core::ptr::null(),
        });
        self.idle = idle;
        self.enqueue(idle, 0);
        self.task_count += 1;
    }

    /// 选出 `start()` 后要运行的第一个任务。
    fn start_first(&mut self) {
        let id = self.pick_next().expect("at least idle is ready");
        self.current = id;
        self.tasks[id].as_mut().expect("task exists").state = TaskState::Running;
    }

    /// 运行中任务的索引。
    #[inline(always)]
    pub(crate) fn current_id(&self) -> usize {
        self.current
    }

    /// 给信号量计数或唤醒其等待最久的任务，返回被唤醒的任务索引。
    /// 必须在临界区内调用。
    #[inline]
    fn sem_give(&mut self, sem: &Semaphore) -> Option<u8> {
        let Some(id) = sem.pop_waiter() else {
            sem.count_inc();
            return None;
        };
        let t = self.tasks[id as usize]
            .as_mut()
            .expect("waiter task exists");
        match &mut t.wait {
            Some(Wait::Sem { granted, .. }) | Some(Wait::Mutex { granted, .. }) => *granted = true,
            _ => {}
        }
        let prio = t.cur_prio;
        self.make_ready(id as usize, prio);
        Some(id)
    }

    // -----------------------------------------------------------------------
    // 优先级继承
    //
    // 任务的有效优先级 = max(基础优先级, 它所持有的每把互斥锁上等待者的最高有效
    // 优先级)。任务持有哪些锁由 `Task::held` 单向链表记录，等待者的优先级则直接
    // 从互斥锁的等待环里取，因此不需要额外缓存，也不存在"缓存过期"的问题。
    //
    // 任意一把锁的等待者集合发生变化后，调用 `refresh_prio(持有者)` 即可；若持有者
    // 自己也阻塞在别的锁上，会沿等待链继续传播（覆盖 H→L→M 这类链式反转）。
    // -----------------------------------------------------------------------

    /// 直接设置有效优先级，必要时在就绪队列中移动到新的优先级。
    fn set_cur_prio(&mut self, id: usize, prio: u8) {
        let was_ready = self.tasks[id].as_ref().expect("task exists").state == TaskState::Ready;
        if was_ready {
            self.dequeue(id);
        }
        self.tasks[id].as_mut().expect("task exists").cur_prio = prio;
        if was_ready {
            self.enqueue(id, prio);
        }
    }

    /// 互斥锁等待者中的最高有效优先级（无等待者时为 0）。
    fn max_waiter_prio(&self, m: &RawMutex) -> u8 {
        let mut max = 0;
        for i in 0..m.inner.waiter_len() {
            let id = m.inner.waiter_at(i) as usize;
            if let Some(t) = self.tasks[id].as_ref() {
                max = max.max(t.cur_prio);
            }
        }
        max
    }

    /// 计算任务应有的有效优先级。
    fn compute_prio(&self, id: usize) -> u8 {
        let t = self.tasks[id].as_ref().expect("task exists");
        let mut prio = t.base_prio;
        let mut it = t.held;
        while !it.is_null() {
            prio = prio.max(self.max_waiter_prio(unsafe { &*it }));
            it = unsafe { *(*it).next_held.get() };
        }
        prio
    }

    /// 重算任务的有效优先级；若其阻塞在互斥锁上，沿等待链继续传播。
    #[inline(always)]
    fn refresh_prio(&mut self, id: usize) {
        self.refresh_prio_depth(id, 0);
    }

    fn refresh_prio_depth(&mut self, id: usize, depth: usize) {
        // 等待链最长不会超过任务数；超限说明出现了环，直接停止。
        if depth > MAX_TASKS {
            return;
        }
        let want = self.compute_prio(id);
        if self.tasks[id].as_ref().expect("task exists").cur_prio == want {
            return;
        }
        self.set_cur_prio(id, want);
        // 只有真正阻塞在互斥锁上时才需要沿着等待链继续传播；超时后残留的
        // `wait`（`granted = false`）不算。
        let t = self.tasks[id].as_ref().expect("task exists");
        if t.state == TaskState::BlockedSem
            && let Some(Wait::Mutex {
                m, granted: false, ..
            }) = t.wait
            && let Some(owner) = unsafe { *(*m).owner.get() }
        {
            self.refresh_prio_depth(owner as usize, depth + 1);
        }
    }

    /// 把互斥锁 `m` 记到任务 `id` 的已持锁链表头。
    fn link_held(&mut self, id: usize, m: &RawMutex) {
        let head = self.tasks[id].as_ref().expect("task exists").held;
        unsafe { *m.next_held.get() = head };
        self.tasks[id].as_mut().expect("task exists").held = m;
    }

    /// 把互斥锁 `m` 从任务 `id` 的已持锁链表中摘除。
    fn unlink_held(&mut self, id: usize, m: &RawMutex) {
        let mut prev: *const RawMutex = core::ptr::null();
        let mut cur = self.tasks[id].as_ref().expect("task exists").held;
        while !cur.is_null() {
            if core::ptr::eq(cur, m) {
                let next = unsafe { *(*cur).next_held.get() };
                if prev.is_null() {
                    self.tasks[id].as_mut().expect("task exists").held = next;
                } else {
                    unsafe { *(*prev).next_held.get() = next };
                }
                unsafe { *(*cur).next_held.get() = core::ptr::null() };
                return;
            }
            prev = cur;
            cur = unsafe { *(*cur).next_held.get() };
        }
    }
}

// ---------------------------------------------------------------------------
// idle 任务
// ---------------------------------------------------------------------------

const IDLE_STACK_SIZE: usize = 512;
static mut IDLE_STACK: StackStorage<IDLE_STACK_SIZE> = StackStorage::new();

unsafe extern "C" fn idle_entry() -> ! {
    loop {
        let has_ready = interrupt::free(|cs| SCHEDULER.borrow(cs).borrow().has_ready_except_idle());
        if has_ready {
            yield_now();
        } else {
            // 无事可做；睡眠直到被中断唤醒。
            unsafe {
                core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 任务注册（链接段收集）
// ---------------------------------------------------------------------------

unsafe extern "C" {
    static __rtos_tasks_start: u8;
    static __rtos_tasks_end: u8;
}

fn collect_task_defs() -> &'static [TaskDef] {
    unsafe {
        let start = &raw const __rtos_tasks_start as *const TaskDef;
        let end = &raw const __rtos_tasks_end as *const TaskDef;
        // 用字节差换算，避免依赖两个符号来自同一 allocation。
        let bytes = (end as usize) - (start as usize);
        core::slice::from_raw_parts(start, bytes / core::mem::size_of::<TaskDef>())
    }
}

// ---------------------------------------------------------------------------
// 公共 API
// ---------------------------------------------------------------------------

/// 启动调度器：配置 tick 定时器、注册所有任务并进入第一个任务。永不返回。
///
/// `syst` 是 SysTick 定时器（取自 `CorePeripherals::take()`），内核取得其所有权。
/// `sysclk_hz` 是当前系统时钟频率（如复位后的 48_000_000），用于推导 1 ms tick 周期。
pub fn start(syst: SYST, sysclk_hz: u32) -> ! {
    let defs = collect_task_defs();
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        sched.init_from_defs(defs);
        sched.start_first();
    });

    // 配置并启用 1 ms tick。PFIC 启用 SysTick 中断后它可能随时触发；此时调度器
    // 已初始化完毕，`on_tick` 是安全的。
    crate::tick::init(syst, sysclk_hz);

    let sp = interrupt::free(|cs| {
        let sched = SCHEDULER.borrow(cs).borrow();
        sched.tasks[sched.current].as_ref().expect("task exists").sp
    });
    unsafe { context::rtos_start_first(sp) }
}

/// 收集所有已注册任务的快照（空槽为 `None`）。
pub fn task_infos() -> [Option<TaskInfo>; MAX_TASKS] {
    interrupt::free(|cs| {
        let sched = SCHEDULER.borrow(cs).borrow();
        let mut out: [Option<TaskInfo>; MAX_TASKS] = [const { None }; MAX_TASKS];
        for (i, slot) in sched.tasks.iter().enumerate() {
            if let Some(t) = slot {
                out[i] = Some(TaskInfo {
                    name: t.name,
                    state: t.state,
                    base_prio: t.base_prio,
                    cur_prio: t.cur_prio,
                });
            }
        }
        out
    })
}

/// 让出 CPU 给最高优先级就绪任务；调用任务回到其优先级队列末尾。
///
/// 若调用者自己就是最高优先级就绪任务，则直接返回（不做栈切换）。
///
/// # Panics
///
/// 在临界区内调用会 panic。
pub fn yield_now() {
    assert_can_block("yield_now");
    let switch = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        let prio = sched.tasks[cur].as_ref().expect("task exists").cur_prio;
        sched.make_ready(cur, prio);
        sched.switch_prep(cur)
    });
    // 在中断使能状态下切换（上面的临界区已释放）。新任务会在其自身的调度点
    // 恢复执行。
    if let Some((old_slot, new_sp)) = switch {
        unsafe { context::rtos_switch(old_slot, new_sp) };
    }
}

/// 睡眠至少 `ms` 毫秒并让出 CPU。
///
/// 时间不精确：若没有其它任务让出，唤醒可能被推迟。`sleep(0)` 等价于 [`yield_now`]。
pub fn sleep(ms: u32) {
    assert_can_block("sleep");
    if ms == 0 {
        return yield_now();
    }
    let (old_slot, new_sp) = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        sched.block_current(Wait::Sleep(deadline_after(ms)));
        sched
            .switch_prep(cur)
            .expect("a blocked task must never be re-selected")
    });
    unsafe { context::rtos_switch(old_slot, new_sp) };
}

// ---------------------------------------------------------------------------
// 供同步原语使用的内部 API
// ---------------------------------------------------------------------------

/// 由 SysTick 中断在 tick 加一后调用。
#[inline]
pub(crate) fn on_tick(now: u32) {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        sched.wake_expired(now);
    });
}

/// 给信号量计数或唤醒其等待最久的任务。
#[inline]
pub(crate) fn sem_give(sem: &Semaphore) {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let _ = sched.sem_give(sem);
    });
}

/// 等待信号量：消费计数，或在**同一个临界区**内登记为等待者并切换。
///
/// 「检查计数」与「登记等待者」必须是原子的，否则 `give_from_isr` 若挤在两步之间，
/// 会把计数加进信号量却没人来取，等待者永远睡着。
pub(crate) fn sem_take(sem: &Semaphore, timeout_ms: Option<u32>) -> bool {
    if timeout_ms != Some(0) {
        assert_can_block("Semaphore::take");
    }
    let mut acquired = false;
    let switch = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        if sem.try_take() {
            acquired = true;
            return None;
        }
        // `Some(0)` = 只做一次非阻塞尝试（`None` 才是永远等待）。
        if timeout_ms == Some(0) {
            return None;
        }
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        sem.register_waiter(cur as u8);
        sched.block_current(Wait::Sem {
            sem: sem as *const Semaphore,
            deadline: timeout_ms.map(deadline_after),
            granted: false,
        });
        Some(
            sched
                .switch_prep(cur)
                .expect("a blocked task must never be re-selected"),
        )
    });
    let Some((old_slot, new_sp)) = switch else {
        return acquired;
    };
    unsafe { context::rtos_switch(old_slot, new_sp) };
    clear_wait_return()
}

/// 获取互斥锁。`timeout_ms` 为 `None` 时永远等待，`Some(0)` 只做一次非阻塞尝试。
pub(crate) fn mutex_lock(m: &RawMutex, timeout_ms: Option<u32>) -> bool {
    if timeout_ms != Some(0) {
        assert_can_block("Mutex::lock");
    }
    let mut acquired = false;
    let switch = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current_id();

        // 快路径：空闲则立即获取。
        if m.inner.try_take() {
            debug_assert!(
                unsafe { (*m.owner.get()).is_none() },
                "ch32x035-rtos: mutex acquired while still owned"
            );
            unsafe { *m.owner.get() = Some(cur as u8) };
            sched.link_held(cur, m);
            acquired = true;
            return None;
        }
        // `Some(0)` = 只做一次非阻塞尝试（`None` 才是永远等待）。
        if timeout_ms == Some(0) {
            return None;
        }

        assert!(
            unsafe { *m.owner.get() } != Some(cur as u8),
            "ch32x035-rtos: task '{}' tried to lock a mutex it already holds",
            sched.tasks[cur].as_ref().expect("task exists").name
        );

        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        m.inner.register_waiter(cur as u8);
        sched.block_current(Wait::Mutex {
            m: m as *const RawMutex,
            deadline: timeout_ms.map(deadline_after),
            granted: false,
        });
        // 新的等待者可能抬高持有者的有效优先级。
        if let Some(o) = unsafe { *m.owner.get() } {
            sched.refresh_prio(o as usize);
        }
        Some(
            sched
                .switch_prep(cur)
                .expect("a blocked task must never be re-selected"),
        )
    });
    let Some((old_slot, new_sp)) = switch else {
        return acquired;
    };
    unsafe { context::rtos_switch(old_slot, new_sp) };
    clear_wait_return()
}

/// 释放互斥锁，重算自身的有效优先级并唤醒一个等待者。
#[inline]
pub(crate) fn mutex_unlock(m: &RawMutex) {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        if let Some(o) = unsafe { *m.owner.get() } {
            sched.unlink_held(o as usize, m);
            unsafe { *m.owner.get() = None };
            sched.refresh_prio(o as usize);
        }
        // 交接时立刻登记新持有者，避免出现 "已加锁但 owner 为空" 的窗口。
        if let Some(woken) = sched.sem_give(&m.inner) {
            unsafe { *m.owner.get() = Some(woken) };
            sched.link_held(woken as usize, m);
            sched.refresh_prio(woken as usize);
        }
    });
}

/// 读取运行中任务本次阻塞是否获授，并清除等待状态。
#[inline]
pub(crate) fn clear_wait_return() -> bool {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        let t = sched.tasks[cur].as_mut().expect("task exists");
        let granted = matches!(
            t.wait,
            Some(Wait::Sem { granted: true, .. }) | Some(Wait::Mutex { granted: true, .. })
        );
        t.wait = None;
        granted
    })
}
