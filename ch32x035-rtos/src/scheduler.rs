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
use crate::task::{MAX_PRIO, MAX_TASKS, Task, TaskDef, TaskInfo, TaskState, Wait};

// ---------------------------------------------------------------------------
// 全局 tick（1 ms/tick，由 SysTick 中断推进）
// ---------------------------------------------------------------------------

static TICK: AtomicU32 = AtomicU32::new(0);

pub(crate) fn tick_now() -> u32 {
    TICK.load(Ordering::Relaxed)
}

/// tick 加一并返回新值。
pub(crate) fn tick_inc() -> u32 {
    TICK.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
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
    /// 最高的非空就绪优先级；`MAX_PRIO + 1` = 无就绪任务。
    top_ready: u8,
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
            top_ready: MAX_PRIO + 1,
            task_count: 0,
        }
    }

    /// 把 `id` 追加到优先级 `prio` 的就绪队列末尾。
    fn enqueue(&mut self, id: usize, prio: u8) {
        let p = prio as usize;
        let len = self.ready_len[p] as usize;
        debug_assert!(len < MAX_TASKS, "ready queue overflow");
        self.ready[p][len] = id as u8;
        self.ready_len[p] += 1;
        if self.top_ready > MAX_PRIO || prio > self.top_ready {
            self.top_ready = prio;
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
                    self.refresh_top();
                    return;
                }
            }
        }
    }

    /// 移除任务后修正 `top_ready`。
    fn refresh_top(&mut self) {
        while self.top_ready > 0 && self.ready_len[self.top_ready as usize] == 0 {
            self.top_ready -= 1;
        }
        if self.ready_len[self.top_ready as usize] == 0 {
            self.top_ready = MAX_PRIO + 1;
        }
    }

    /// 把任务 `id` 标记为就绪并以 `prio` 入队（保留 `wait` 供 `take` 读取返回值）。
    fn make_ready(&mut self, id: usize, prio: u8) {
        self.tasks[id].as_mut().expect("task exists").state = TaskState::Ready;
        self.enqueue(id, prio);
    }

    /// 用 `wait` 阻塞运行中的任务。
    fn block_current(&mut self, wait: Wait) {
        let cur = self.current;
        let t = self.tasks[cur].as_mut().expect("task exists");
        t.wait = Some(wait);
        t.state = match wait {
            Wait::Sleep(_) => TaskState::BlockedSleep,
            Wait::Sem { .. } => TaskState::BlockedSem,
        };
    }

    /// 从就绪队列取出最高优先级任务。
    fn pick_next(&mut self) -> Option<usize> {
        if self.top_ready > MAX_PRIO {
            return None;
        }
        let p = self.top_ready as usize;
        let id = self.ready[p][0] as usize;
        let len = self.ready_len[p] as usize;
        for i in 1..len {
            self.ready[p][i - 1] = self.ready[p][i];
        }
        self.ready_len[p] -= 1;
        self.refresh_top();
        Some(id)
    }

    /// 选出下一个任务并准备从 `cur` 切换。
    /// 返回 `(旧任务的 sp 槽位, 新任务的 sp)`。
    fn switch_prep(&mut self, cur: usize) -> (*mut *mut u32, *mut u32) {
        let n = self.pick_next().expect("idle must always be ready");
        self.current = n;
        self.tasks[n].as_mut().expect("task exists").state = TaskState::Running;
        let new_sp = self.tasks[n].as_ref().expect("task exists").sp;
        let old_slot = &raw mut self.tasks[cur].as_mut().expect("task exists").sp;
        (old_slot, new_sp)
    }

    /// 是否存在除 idle 之外的就绪任务。
    fn has_ready_except_idle(&self) -> bool {
        if self.top_ready > MAX_PRIO {
            return false;
        }
        if self.top_ready > 0 {
            return true;
        }
        // 只有优先级 0 非空；检查其中是否有 idle 之外的任务。
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
            let Some(t) = self.tasks[i].as_mut() else {
                continue;
            };
            match t.state {
                TaskState::BlockedSleep => {
                    if let Some(Wait::Sleep(d)) = t.wait
                        && d <= now
                    {
                        t.wait = None;
                        let prio = t.cur_prio;
                        t.state = TaskState::Ready;
                        self.enqueue(i, prio);
                    }
                }
                TaskState::BlockedSem => {
                    if let Some(Wait::Sem {
                        sem,
                        deadline,
                        granted: false,
                    }) = t.wait
                        && deadline != 0
                        && deadline <= now
                    {
                        // 把任务从信号量的等待环中移除。
                        unsafe { (&*(sem as *const Semaphore)).remove_waiter(i as u8) };
                        // 保留 `wait`（granted = false）供 `take` 读取超时结果。
                        let prio = t.cur_prio;
                        t.state = TaskState::Ready;
                        self.enqueue(i, prio);
                    }
                }
                _ => {}
            }
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
            });
            self.enqueue(i, d.priority);
            self.task_count += 1;
        }

        // idle 任务永远存在，且在没有其它就绪任务时总是就绪。
        let idle = self.task_count;
        let idle_bottom = &raw mut IDLE_STACK as *mut u8;
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
    pub(crate) fn current_id(&self) -> usize {
        self.current
    }

    /// 给信号量计数或唤醒其等待最久的任务。
    /// 必须在临界区内调用。
    fn sem_give(&mut self, sem: &Semaphore) {
        if let Some(id) = sem.pop_waiter() {
            let t = self.tasks[id as usize]
                .as_mut()
                .expect("waiter task exists");
            if let Some(Wait::Sem { granted, .. }) = &mut t.wait {
                *granted = true;
            }
            let prio = t.cur_prio;
            self.make_ready(id as usize, prio);
        } else {
            sem.count_inc();
        }
    }

    /// 把 `owner` 的有效优先级提升到 `to_prio`（互斥优先级继承），若其就绪则重新入队。
    pub(crate) fn inherit_priority(&mut self, owner: usize, to_prio: u8) {
        {
            let t = self.tasks[owner].as_ref().expect("owner task exists");
            if to_prio <= t.cur_prio {
                return;
            }
        }
        let was_ready =
            self.tasks[owner].as_ref().expect("owner task exists").state == TaskState::Ready;
        if was_ready {
            self.dequeue(owner);
        }
        self.tasks[owner]
            .as_mut()
            .expect("owner task exists")
            .cur_prio = to_prio;
        if was_ready {
            self.enqueue(owner, to_prio);
        }
    }

    /// 把 `owner` 的有效优先级恢复为其基础优先级，若其就绪则重新入队。
    pub(crate) fn restore_priority(&mut self, owner: usize) {
        let base = self.tasks[owner]
            .as_ref()
            .expect("owner task exists")
            .base_prio;
        {
            let t = self.tasks[owner].as_ref().expect("owner task exists");
            if t.cur_prio == base {
                return;
            }
        }
        let was_ready =
            self.tasks[owner].as_ref().expect("owner task exists").state == TaskState::Ready;
        if was_ready {
            self.dequeue(owner);
        }
        self.tasks[owner]
            .as_mut()
            .expect("owner task exists")
            .cur_prio = base;
        if was_ready {
            self.enqueue(owner, base);
        }
    }
}

// ---------------------------------------------------------------------------
// idle 任务
// ---------------------------------------------------------------------------

const IDLE_STACK_SIZE: usize = 512;
static mut IDLE_STACK: [u8; IDLE_STACK_SIZE] = [0; IDLE_STACK_SIZE];

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
        let count = end.offset_from(start) as usize;
        core::slice::from_raw_parts(start, count)
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
pub fn yield_now() {
    let (old_slot, new_sp) = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        let prio = sched.tasks[cur].as_ref().expect("task exists").cur_prio;
        sched.make_ready(cur, prio);
        sched.switch_prep(cur)
    });
    // 在中断使能状态下切换（上面的临界区已释放）。新任务会在其自身的调度点
    // 恢复执行。
    unsafe { context::rtos_switch(old_slot, new_sp) };
}

/// 睡眠至少 `ms` 毫秒并让出 CPU。
///
/// 时间不精确：若没有其它任务让出，唤醒可能被推迟。`sleep(0)` 等价于 [`yield_now`]。
pub fn sleep(ms: u32) {
    if ms == 0 {
        return yield_now();
    }
    let (old_slot, new_sp) = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        let now = tick_now();
        sched.block_current(Wait::Sleep(now.wrapping_add(ms)));
        sched.switch_prep(cur)
    });
    unsafe { context::rtos_switch(old_slot, new_sp) };
}

// ---------------------------------------------------------------------------
// 供同步原语使用的内部 API
// ---------------------------------------------------------------------------

/// 由 SysTick 中断在 tick 加一后调用。
pub(crate) fn on_tick(now: u32) {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        sched.wake_expired(now);
    });
}

/// 给信号量计数或唤醒其等待最久的任务。
pub(crate) fn sem_give(sem: &Semaphore) {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        sched.sem_give(sem);
    });
}

/// 获取互斥锁，阻塞直到获授或 `deadline`（0 = 永远）。阻塞期间应用优先级继承。
pub(crate) fn mutex_lock(m: &RawMutex, deadline: u32) -> bool {
    // 快路径：空闲则立即获取。
    let got = interrupt::free(|cs| {
        let sched = SCHEDULER.borrow(cs).borrow_mut();
        if m.inner.try_take() {
            unsafe { *m.owner.get() = Some(sched.current_id() as u8) };
            true
        } else {
            false
        }
    });
    if got {
        return true;
    }

    // 优先级继承：提升持有者的有效优先级，使其尽快被调度并释放锁。
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        if let Some(o) = unsafe { *m.owner.get() } {
            let base = sched.tasks[sched.current]
                .as_ref()
                .expect("task exists")
                .base_prio;
            sched.inherit_priority(o as usize, base);
        }
    });

    block_sem_and_switch(&m.inner, deadline);

    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        let granted = matches!(
            sched.tasks[cur].as_ref().expect("task exists").wait,
            Some(Wait::Sem { granted: true, .. })
        );
        sched.tasks[cur].as_mut().expect("task exists").wait = None;
        if granted {
            unsafe { *m.owner.get() = Some(cur as u8) };
        }
        granted
    })
}

/// 释放互斥锁，恢复任何继承的优先级并唤醒一个等待者。
pub(crate) fn mutex_unlock(m: &RawMutex) {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        if let Some(o) = unsafe { *m.owner.get() } {
            sched.restore_priority(o as usize);
        }
        unsafe { *m.owner.get() = None };
        sched.sem_give(&m.inner);
    });
}

/// 阻塞运行中任务于 `sem` 并切换。
///
/// 只能在任务上下文调用。任务获授信号量或超时后返回。
pub(crate) fn block_sem_and_switch(sem: &Semaphore, deadline: u32) {
    let (old_slot, new_sp) = interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        sem.register_waiter(cur as u8);
        sched.block_current(Wait::Sem {
            sem: sem as *const Semaphore as *const (),
            deadline,
            granted: false,
        });
        sched.switch_prep(cur)
    });
    unsafe { context::rtos_switch(old_slot, new_sp) };
}

/// 读取运行中任务本次信号量等待是否获授，并清除等待状态。
pub(crate) fn clear_sem_wait_return(sem: &Semaphore) -> bool {
    interrupt::free(|cs| {
        let mut sched = SCHEDULER.borrow(cs).borrow_mut();
        let cur = sched.current;
        let t = sched.tasks[cur].as_mut().expect("task exists");
        let granted = matches!(
            t.wait,
            Some(Wait::Sem {
                sem: s,
                granted: true,
                ..
            }) if s == sem as *const Semaphore as *const ()
        );
        t.wait = None;
        granted
    })
}
