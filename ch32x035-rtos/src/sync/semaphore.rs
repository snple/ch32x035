//! Counting semaphore with timeout support.
//!
//! `take` blocks the calling task (yielding the CPU) until a `give` makes a
//! count available, or until an optional timeout expires. `give_from_isr`
//! may be called from interrupt context; it only pokes the scheduler, so the
//! awakened task runs at the next scheduling point.

use core::cell::UnsafeCell;

use crate::task::MAX_TASKS;

/// A counting semaphore.
///
/// Must live in `'static` storage (declare it as a `static`) so that blocked
/// tasks can safely reference it.
///
/// # Example
///
/// ```ignore
/// static EVT: ch32x035_rtos::sync::Semaphore = ch32x035_rtos::sync::Semaphore::new(0);
///
/// #[task]
/// fn worker() -> ! {
///     loop {
///         if EVT.take(Some(1000)) {
///             // signalled
///         } else {
///             // timed out
///         }
///     }
/// }
/// ```
pub struct Semaphore {
    /// Available count (only touched inside a critical section).
    count: UnsafeCell<u32>,
    /// Ring buffer of blocked task indices, `0xFF` = free slot.
    waiters: UnsafeCell<[u8; MAX_TASKS]>,
    /// Index of the longest-waiting task (FIFO pop head).
    head: UnsafeCell<u8>,
    /// Index of the next insertion slot.
    tail: UnsafeCell<u8>,
}

// All fields are only mutated inside a critical section (see scheduler).
unsafe impl Sync for Semaphore {}

impl Semaphore {
    /// Create a semaphore with an initial `count`.
    pub const fn new(count: u32) -> Self {
        Self {
            count: UnsafeCell::new(count),
            waiters: UnsafeCell::new([0xFF; MAX_TASKS]),
            head: UnsafeCell::new(0),
            tail: UnsafeCell::new(0),
        }
    }

    /// Wait for a count. `timeout_ms` of `None` waits forever.
    ///
    /// Returns `true` if a count was acquired, `false` on timeout.
    ///
    /// # Panics
    ///
    /// Panics if called from an interrupt handler.
    pub fn take(&self, timeout_ms: Option<u32>) -> bool {
        if critical_section::with(|_| self.try_take()) {
            return true;
        }
        let deadline = timeout_ms
            .map(|ms| crate::scheduler::tick_now().wrapping_add(ms))
            .unwrap_or(0);
        crate::scheduler::block_sem_and_switch(self, deadline);
        crate::scheduler::clear_sem_wait_return(self)
    }

    /// Give one count, waking the longest-waiting blocked task if any.
    pub fn give(&self) {
        crate::scheduler::sem_give(self);
    }

    /// Give one count from interrupt context.
    ///
    /// Safe to call from an interrupt handler; only updates scheduler state
    /// and does not switch tasks.
    pub fn give_from_isr(&self) {
        self.give();
    }

    // ------------------------------------------------------------------
    // Internal helpers; MUST be called inside a critical section.
    // ------------------------------------------------------------------

    /// Consume one count if available.
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

    /// Add one count.
    pub(crate) fn count_inc(&self) {
        unsafe {
            *self.count.get() += 1;
        }
    }

    /// Number of queued waiters.
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

    /// Push a task onto the waiter ring.
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

    /// Pop the longest-waiting task, if any.
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

    /// Remove a specific task from the waiter ring (timeout path).
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
                    // Shift subsequent entries down, then rewind the tail.
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
