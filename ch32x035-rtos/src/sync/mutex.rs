//! 带优先级继承的互斥锁，API 对齐 `std::sync::Mutex`。
//!
//! 基于二值信号量实现。任务阻塞在已锁定的互斥锁上时，当前持有者的有效优先级被
//! 提升到等待者的优先级，使持有者尽快被调度并释放锁。
//!
//! [`Mutex::lock`] 返回 RAII 守卫 [`MutexGuard`]，离开作用域自动解锁，避免忘记
//! `unlock` 或异常路径泄漏锁。

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use crate::sync::Semaphore;

/// 锁的底层原语（非泛型），供调度器使用。
///
/// 与泛型的 [`Mutex`] 分离，使调度器的 `mutex_lock` / `mutex_unlock` 不依赖 `T`。
pub(crate) struct RawMutex {
    /// 支撑加锁的二值信号量。
    pub(crate) inner: Semaphore,
    /// 当前持有者任务索引（已锁定时）。
    pub(crate) owner: UnsafeCell<Option<u8>>,
}

// 所有字段仅在临界区内修改（见 scheduler）。
unsafe impl Sync for RawMutex {}

impl RawMutex {
    pub(crate) const fn new() -> Self {
        Self {
            inner: Semaphore::new(1),
            owner: UnsafeCell::new(None),
        }
    }
}

/// 保护共享数据 `T` 的互斥锁。
///
/// 必须为 `'static`（声明为 `static`）。
///
/// # Example
///
/// ```ignore
/// static M: ch32x035_rtos::sync::Mutex<u32> = ch32x035_rtos::sync::Mutex::new(0);
///
/// *M.lock() += 1; // guard 在此语句末尾自动解锁
/// ```
pub struct Mutex<T: ?Sized> {
    raw: RawMutex,
    data: UnsafeCell<T>,
}

// Mutex 可跨任务共享：`T: Send` 保证数据可安全共享，实际互斥由锁保证。
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// 以数据 `data` 创建未锁定的互斥锁。
    pub const fn new(data: T) -> Self {
        Self {
            raw: RawMutex::new(),
            data: UnsafeCell::new(data),
        }
    }

    /// 消费锁，取出内部数据（无需加锁）。
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: ?Sized> Mutex<T> {
    /// 获取锁，阻塞（让出）直到可用。返回 RAII 守卫，离开作用域自动解锁。
    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        crate::scheduler::mutex_lock(&self.raw, 0);
        MutexGuard {
            mutex: self,
            data: self.data.get(),
        }
    }

    /// 在 `timeout_ms` 内尝试获取锁（`None` = 永远）。
    /// 成功返回守卫，超时返回 `None`。
    #[inline]
    pub fn try_lock(&self, timeout_ms: Option<u32>) -> Option<MutexGuard<'_, T>> {
        let deadline = timeout_ms
            .map(|ms| crate::scheduler::tick_now().wrapping_add(ms))
            .unwrap_or(0);
        if crate::scheduler::mutex_lock(&self.raw, deadline) {
            Some(MutexGuard {
                mutex: self,
                data: self.data.get(),
            })
        } else {
            None
        }
    }

    /// 返回底层数据的可变引用（无需加锁；要求 `&mut self` 独占）。
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }
}

/// [`Mutex::lock`] / [`Mutex::try_lock`] 返回的 RAII 守卫。
///
/// 通过 [`Deref`]/[`DerefMut`] 访问受保护数据；离开作用域（`Drop`）自动解锁。
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    mutex: &'a Mutex<T>,
    data: *mut T,
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    #[inline(always)]
    fn drop(&mut self) {
        crate::scheduler::mutex_unlock(&self.mutex.raw);
    }
}

// 与 std 一致：guard 可 Sync（T: Sync），但不可 Send——解锁必须发生在持锁任务内。
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}
