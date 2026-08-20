//! Synchronisation primitives for the cooperative kernel.

mod mutex;
mod semaphore;

pub use mutex::Mutex;
pub use semaphore::Semaphore;
