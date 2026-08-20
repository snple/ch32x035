//! Mutex with priority inheritance.
//!
//! Built on a binary semaphore. When a task blocks on a locked mutex, the
//! current owner's effective priority is raised to the waiter's priority so
//! the owner gets scheduled sooner and releases the lock promptly.

use core::cell::UnsafeCell;

use crate::sync::Semaphore;

/// A mutex protecting shared state.
///
/// Must be `'static` (declare it as a `static`).
///
/// # Example
///
/// ```ignore
/// static M: ch32x035_rtos::sync::Mutex = ch32x035_rtos::sync::Mutex::new();
///
/// #[task]
/// fn consumer() -> ! {
///     loop {
///         M.lock();
///         // critical section protected by the mutex
///         M.unlock();
///         ch32x035_rtos::yield_now();
///     }
/// }
/// ```
pub struct Mutex {
    /// Binary semaphore backing the lock.
    pub(crate) inner: Semaphore,
    /// Index of the current owner task, if locked.
    pub(crate) owner: UnsafeCell<Option<u8>>,
}

// All fields are only mutated inside a critical section (see scheduler).
unsafe impl Sync for Mutex {}

impl Mutex {
    /// Create an unlocked mutex.
    pub const fn new() -> Self {
        Self {
            inner: Semaphore::new(1),
            owner: UnsafeCell::new(None),
        }
    }

    /// Acquire the lock, blocking (yielding) until available.
    pub fn lock(&self) {
        crate::scheduler::mutex_lock(self, 0);
    }

    /// Try to acquire the lock within `timeout_ms` (`None` = forever).
    /// Returns `true` on success.
    pub fn try_lock(&self, timeout_ms: Option<u32>) -> bool {
        let deadline = timeout_ms
            .map(|ms| crate::scheduler::tick_now().wrapping_add(ms))
            .unwrap_or(0);
        crate::scheduler::mutex_lock(self, deadline)
    }

    /// Release the lock.
    pub fn unlock(&self) {
        crate::scheduler::mutex_unlock(self);
    }
}

impl Default for Mutex {
    fn default() -> Self {
        Self::new()
    }
}
