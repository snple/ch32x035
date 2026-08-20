//! The scheduler: task table, multi-level ready queue, and the public
//! `start` / `yield_now` / `sleep` API.
//!
//! Cooperative scheduling: a task keeps the CPU until it explicitly yields,
//! sleeps or blocks on a semaphore/mutex. At each yield point the highest
//! priority ready task is selected (FIFO within a priority). The idle task
//! runs `wfi` when nothing else is ready.

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use ch32x035::rt::peripheral::SYST;
use critical_section::Mutex as CsMutex;

use crate::context;
use crate::sync::{Mutex, Semaphore};
use crate::task::{MAX_PRIO, MAX_TASKS, Task, TaskDef, TaskInfo, TaskState, Wait};

// ---------------------------------------------------------------------------
// Global tick (1 ms per tick, driven by the SysTick IRQ)
// ---------------------------------------------------------------------------

static TICK: AtomicU32 = AtomicU32::new(0);

pub(crate) fn tick_now() -> u32 {
    TICK.load(Ordering::Relaxed)
}

/// Advance the tick by one and return the new value.
pub(crate) fn tick_inc() -> u32 {
    TICK.fetch_add(1, Ordering::Relaxed).wrapping_add(1)
}

// ---------------------------------------------------------------------------
// Scheduler state
// ---------------------------------------------------------------------------

/// Scheduler state. Only ever accessed inside a critical section.
pub(crate) struct Scheduler {
    /// Task table; `None` = free slot.
    tasks: [Option<Task>; MAX_TASKS],
    /// Index of the running task.
    current: usize,
    /// Index of the idle task.
    idle: usize,
    /// Ready queues: one FIFO per priority level, holding task indices.
    ready: [[u8; MAX_TASKS]; (MAX_PRIO + 1) as usize],
    /// Length of each ready queue.
    ready_len: [u8; (MAX_PRIO + 1) as usize],
    /// Highest non-empty ready priority; `MAX_PRIO + 1` = none ready.
    top_ready: u8,
    /// Number of registered tasks (including idle).
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

    /// Append `id` to the ready queue at priority `prio`.
    pub(crate) fn enqueue(&mut self, id: usize, prio: u8) {
        let p = prio as usize;
        let len = self.ready_len[p] as usize;
        debug_assert!(len < MAX_TASKS, "ready queue overflow");
        self.ready[p][len] = id as u8;
        self.ready_len[p] += 1;
        if self.top_ready > MAX_PRIO || prio > self.top_ready {
            self.top_ready = prio;
        }
    }

    /// Remove `id` from whatever ready queue holds it.
    pub(crate) fn dequeue(&mut self, id: usize) {
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

    /// Fix `top_ready` after a removal.
    fn refresh_top(&mut self) {
        while self.top_ready > 0 && self.ready_len[self.top_ready as usize] == 0 {
            self.top_ready -= 1;
        }
        if self.ready_len[self.top_ready as usize] == 0 {
            self.top_ready = MAX_PRIO + 1;
        }
    }

    /// Mark task `id` ready at priority `prio` (keeps `wait` for the
    /// semaphore `take` return value).
    pub(crate) fn make_ready(&mut self, id: usize, prio: u8) {
        let t = self.tasks[id].as_mut().expect("task exists");
        t.state = TaskState::Ready;
        self.enqueue(id, prio);
    }

    /// Block the running task with `wait`.
    fn block_current(&mut self, wait: Wait) {
        let cur = self.current;
        let t = self.tasks[cur].as_mut().expect("task exists");
        t.wait = Some(wait);
        t.state = match wait {
            Wait::Sleep(_) => TaskState::BlockedSleep,
            Wait::Sem { .. } => TaskState::BlockedSem,
        };
    }

    /// Take the highest-priority ready task out of its queue.
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

    /// Pick the next task and prepare a switch away from `cur`.
    /// Returns `(old_sp_slot, new_sp)`.
    fn switch_prep(&mut self, cur: usize) -> (*mut *mut u32, *mut u32) {
        let n = self.pick_next().expect("idle must always be ready");
        self.current = n;
        self.tasks[n].as_mut().expect("task exists").state = TaskState::Running;
        let new_sp = self.tasks[n].as_ref().expect("task exists").sp;
        let old_slot = &raw mut self.tasks[cur].as_mut().expect("task exists").sp;
        (old_slot, new_sp)
    }

    /// Whether any task other than idle is ready.
    fn has_ready_except_idle(&self) -> bool {
        if self.top_ready > MAX_PRIO {
            return false;
        }
        if self.top_ready > 0 {
            return true;
        }
        // Only priority 0 is non-empty; check it holds something besides idle.
        for i in 0..self.ready_len[0] as usize {
            if self.ready[0][i] as usize != self.idle {
                return true;
            }
        }
        false
    }

    /// Wake tasks whose deadline has passed.
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
                        // Drop this task from the semaphore's waiter ring.
                        unsafe { (&*(sem as *const Semaphore)).remove_waiter(i as u8) };
                        // Keep `wait` (granted = false) for `take` to read.
                        let prio = t.cur_prio;
                        t.state = TaskState::Ready;
                        self.enqueue(i, prio);
                    }
                }
                _ => {}
            }
        }
    }

    /// Fill the task table from the `.rtos.tasks` link section and register
    /// the idle task.
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
                entry: d.entry,
                sp,
                stack_bottom: d.stack,
                stack_size: d.stack_size,
                base_prio: d.priority,
                cur_prio: d.priority,
                name: d.name,
                state: TaskState::Ready,
                wait: None,
            });
            self.make_ready(i, d.priority);
            self.task_count += 1;
        }

        // The idle task always exists and is always ready when nothing else
        // is runnable.
        let idle = self.task_count;
        let idle_bottom = &raw mut IDLE_STACK as *mut u8;
        let idle_sp =
            unsafe { context::init_stack_frame(idle_entry, idle_bottom, IDLE_STACK_SIZE) };
        self.tasks[idle] = Some(Task {
            entry: idle_entry,
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
        self.make_ready(idle, 0);
        self.task_count += 1;
    }

    /// Pick the first task to run after `start()`.
    fn start_first(&mut self) -> usize {
        let id = self.pick_next().expect("at least idle is ready");
        self.current = id;
        self.tasks[id].as_mut().expect("task exists").state = TaskState::Running;
        id
    }

    /// Index of the running task.
    pub(crate) fn current_id(&self) -> usize {
        self.current
    }

    /// Give a semaphore count or wake its longest-waiting task.
    /// Must be called inside a critical section.
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

    /// Raise `owner`'s effective priority to `to_prio` (mutex priority
    /// inheritance), re-queuing it if it is currently ready.
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

    /// Restore `owner`'s effective priority to its base priority, re-queuing
    /// it if it is currently ready.
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
// Idle task
// ---------------------------------------------------------------------------

const IDLE_STACK_SIZE: usize = 512;
static mut IDLE_STACK: [u8; IDLE_STACK_SIZE] = [0; IDLE_STACK_SIZE];

unsafe extern "C" fn idle_entry() -> ! {
    loop {
        let has_ready =
            critical_section::with(|cs| SCHEDULER.borrow_ref(cs).has_ready_except_idle());
        if has_ready {
            yield_now();
        } else {
            // Nothing to do; sleep until an interrupt wakes us.
            unsafe {
                core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Task registration (link-section collection)
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
// Public API
// ---------------------------------------------------------------------------

/// Start the scheduler: configure the tick timer, register all tasks, and
/// enter the first one. Never returns.
///
/// `syst` is the SysTick timer (from `CorePeripherals::take()`); the kernel
/// takes ownership of it. `sysclk_hz` is the current system clock frequency
/// (e.g. 48_000_000 after reset), used to derive the 1 ms tick period.
pub fn start(syst: SYST, sysclk_hz: u32) -> ! {
    let defs = collect_task_defs();
    let _first = critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        sched.init_from_defs(defs);
        sched.start_first()
    });

    crate::tick::init(syst, sysclk_hz);

    let sp = critical_section::with(|cs| {
        let sched = SCHEDULER.borrow_ref(cs);
        sched.tasks[sched.current].as_ref().expect("task exists").sp
    });
    unsafe { context::rtos_start_first(sp) }
}

/// Collect a snapshot of all registered tasks (empty slots are `None`).
///
/// Useful for logging scheduler state; e.g. print it from a task with
/// `defmt` or a UART.
pub fn task_infos() -> [Option<TaskInfo>; MAX_TASKS] {
    critical_section::with(|cs| {
        let sched = SCHEDULER.borrow_ref(cs);
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

/// Yield the CPU to the highest-priority ready task. The calling task is
/// placed at the tail of its priority queue.
pub fn yield_now() {
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        let prio = sched.tasks[cur].as_ref().expect("task exists").cur_prio;
        sched.make_ready(cur, prio);
        let (old_slot, new_sp) = sched.switch_prep(cur);
        drop(sched);
        // Switch with interrupts disabled; the new task resumes right after
        // this point and exits its own critical section, restoring its own
        // mstatus.
        unsafe { context::rtos_switch(old_slot, new_sp) };
    });
}

/// Sleep for at least `ms` milliseconds, yielding the CPU.
///
/// Timing is not exact: wakeup may be delayed if no other task yields.
/// `sleep(0)` is equivalent to [`yield_now`].
pub fn sleep(ms: u32) {
    if ms == 0 {
        return yield_now();
    }
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        let now = tick_now();
        sched.block_current(Wait::Sleep(now.wrapping_add(ms)));
        let (old_slot, new_sp) = sched.switch_prep(cur);
        drop(sched);
        unsafe { context::rtos_switch(old_slot, new_sp) };
    });
}

// ---------------------------------------------------------------------------
// Internal API used by the sync primitives
// ---------------------------------------------------------------------------

/// Called by the SysTick IRQ after advancing the tick.
pub(crate) fn on_tick(now: u32) {
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        sched.wake_expired(now);
    });
}

/// Give a semaphore count or wake its longest-waiting task.
pub(crate) fn sem_give(sem: &Semaphore) {
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        sched.sem_give(sem);
    });
}

/// Acquire a mutex, blocking the running task until granted or `deadline`
/// (0 = forever). Applies priority inheritance while blocked.
pub(crate) fn mutex_lock(m: &Mutex, deadline: u32) -> bool {
    // Fast path: acquire immediately if free.
    let got = critical_section::with(|cs| {
        let sched = SCHEDULER.borrow_ref_mut(cs);
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

    // Priority inheritance: raise the owner's effective priority so it is
    // scheduled sooner and releases the lock promptly.
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        if let Some(o) = unsafe { *m.owner.get() } {
            let cur = sched.current_id();
            let base = sched.tasks[cur].as_ref().expect("task exists").base_prio;
            sched.inherit_priority(o as usize, base);
        }
    });

    block_sem_and_switch(&m.inner, deadline);

    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        let cur = sched.current_id();
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

/// Release a mutex, restoring any inherited priority and waking a waiter.
pub(crate) fn mutex_unlock(m: &Mutex) {
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        if let Some(o) = unsafe { *m.owner.get() } {
            sched.restore_priority(o as usize);
        }
        unsafe { *m.owner.get() = None };
        sched.sem_give(&m.inner);
    });
}

/// Block the running task on `sem` and switch away.
///
/// Must only be called from task context. Returns when the task is granted
/// the semaphore or times out.
pub(crate) fn block_sem_and_switch(sem: &Semaphore, deadline: u32) {
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
        let cur = sched.current;
        context::check_canary(sched.tasks[cur].as_ref().expect("task exists"));
        sem.register_waiter(cur as u8);
        sched.block_current(Wait::Sem {
            sem: sem as *const Semaphore as *const (),
            deadline,
            granted: false,
        });
        let (old_slot, new_sp) = sched.switch_prep(cur);
        drop(sched);
        unsafe { context::rtos_switch(old_slot, new_sp) };
    });
}

/// Read whether the running task's semaphore wait was granted, and clear the
/// wait state.
pub(crate) fn clear_sem_wait_return(sem: &Semaphore) -> bool {
    critical_section::with(|cs| {
        let mut sched = SCHEDULER.borrow_ref_mut(cs);
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
