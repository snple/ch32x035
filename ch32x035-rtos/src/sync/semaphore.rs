//! 带超时的计数信号量。
//!
//! `take` 阻塞调用任务（让出 CPU）直到某个 `give` 提供计数，或可选超时到期。
//! `give_from_isr` 可从中断上下文调用；它只更新调度器状态，被唤醒的任务在下一个
//! 调度点运行。

use core::cell::UnsafeCell;

use crate::task::MAX_TASKS;

/// 计数信号量。
///
/// 必须放在 `'static` 存储中（声明为 `static`），以便阻塞任务安全地引用它。
pub struct Semaphore {
    /// 可用计数（仅在临界区内访问）。
    count: UnsafeCell<u32>,
    /// 阻塞任务索引的环形缓冲，`0xFF` = 空槽。
    waiters: UnsafeCell<[u8; MAX_TASKS]>,
    /// 等待最久任务的弹出位置（FIFO 头）。
    head: UnsafeCell<u8>,
    /// 下一个插入位置。
    tail: UnsafeCell<u8>,
}

// 所有字段仅在临界区内修改（见 scheduler）。
unsafe impl Sync for Semaphore {}

impl Semaphore {
    /// 以初始 `count` 创建信号量。
    pub const fn new(count: u32) -> Self {
        Self {
            count: UnsafeCell::new(count),
            waiters: UnsafeCell::new([0xFF; MAX_TASKS]),
            head: UnsafeCell::new(0),
            tail: UnsafeCell::new(0),
        }
    }

    /// 等待一个计数。`timeout_ms` 为 `None` 时永远等待，`Some(0)` 只做一次非阻塞尝试。
    ///
    /// 获取到计数返回 `true`，超时返回 `false`。
    ///
    /// 「检查计数」与「登记等待者」在同一个临界区内完成，因此 `give_from_isr`
    /// 不会丢失唤醒。
    ///
    /// # Panics
    ///
    /// 不得在中断处理函数中调用（阻塞需要任务上下文，且内核用当前任务索引记录
    /// 等待者）。
    pub fn take(&self, timeout_ms: Option<u32>) -> bool {
        crate::scheduler::sem_take(self, timeout_ms)
    }

    /// 给出一个计数，唤醒等待最久的阻塞任务（若有）。
    pub fn give(&self) {
        crate::scheduler::sem_give(self);
    }

    /// 从中断上下文给出一个计数。
    ///
    /// 可在中断处理函数中安全调用；只更新调度器状态，不切换任务。
    pub fn give_from_isr(&self) {
        self.give();
    }

    // ------------------------------------------------------------------
    // 内部辅助方法；必须在临界区内调用。
    // ------------------------------------------------------------------

    /// 消费一个计数（若有）。
    pub(crate) fn try_take(&self) -> bool {
        unsafe {
            let c = &mut *self.count.get();
            if *c > 0 {
                *c -= 1;
                true
            } else {
                false
            }
        }
    }

    /// 增加一个计数。
    pub(crate) fn count_inc(&self) {
        unsafe {
            *self.count.get() += 1;
        }
    }

    /// 排队等待者数量。
    pub(crate) fn waiter_len(&self) -> u8 {
        unsafe {
            let head = *self.head.get();
            let tail = *self.tail.get();
            if tail >= head {
                tail - head
            } else {
                (MAX_TASKS as u8) - head + tail
            }
        }
    }

    /// 第 `i` 个等待者（`i` 从等待最久的开始计）。
    pub(crate) fn waiter_at(&self, i: u8) -> u8 {
        unsafe {
            let head = *self.head.get();
            (*self.waiters.get())[((head + i) % (MAX_TASKS as u8)) as usize]
        }
    }

    /// 把任务压入等待环。
    pub(crate) fn register_waiter(&self, id: u8) {
        unsafe {
            debug_assert!(
                self.waiter_len() < MAX_TASKS as u8,
                "semaphore waiter overflow"
            );
            let tail = *self.tail.get();
            (*self.waiters.get())[tail as usize] = id;
            *self.tail.get() = (tail + 1) % (MAX_TASKS as u8);
        }
    }

    /// 弹出等待最久的任务（若有）。
    pub(crate) fn pop_waiter(&self) -> Option<u8> {
        unsafe {
            let head = *self.head.get();
            let tail = *self.tail.get();
            if head == tail {
                return None;
            }
            let id = (*self.waiters.get())[head as usize];
            (*self.waiters.get())[head as usize] = 0xFF;
            *self.head.get() = (head + 1) % (MAX_TASKS as u8);
            Some(id)
        }
    }

    /// 从等待环中移除指定任务（超时路径）。
    pub(crate) fn remove_waiter(&self, id: u8) {
        unsafe {
            let head = *self.head.get();
            let tail = *self.tail.get();
            if head == tail {
                return;
            }
            let w = &mut *self.waiters.get();
            let len = self.waiter_len();
            let mut i = head;
            for _ in 0..len {
                if w[i as usize] == id {
                    // 把后续项前移，并回退 tail。
                    let mut j = i;
                    let mut next = (j + 1) % (MAX_TASKS as u8);
                    while next != tail {
                        w[j as usize] = w[next as usize];
                        j = next;
                        next = (j + 1) % (MAX_TASKS as u8);
                    }
                    *self.tail.get() =
                        ((tail as i32 - 1 + MAX_TASKS as i32) % MAX_TASKS as i32) as u8;
                    return;
                }
                i = (i + 1) % (MAX_TASKS as u8);
            }
        }
    }
}
