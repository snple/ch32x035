//! Fixed-capacity byte ring buffer.

/// A fixed-capacity byte ring buffer for interrupt-driven reception.
///
/// `push` (typically called from an interrupt handler) and `pop` (typically
/// called from application code) mutate shared state, so the buffer should be
/// wrapped in a critical section or otherwise synchronized:
///
/// ```ignore
/// use ch32x035::kit::ring_buffer::RingBuffer;
/// use ch32x035::interrupt::{self, Mutex};
/// use core::cell::RefCell;
///
/// static RX: Mutex<RefCell<RingBuffer<64>>> = Mutex::new(RefCell::new(RingBuffer::new()));
///
/// fn isr() {
///     interrupt::free(|cs| RX.borrow(cs).borrow_mut().push(byte));
/// }
/// ```
pub struct RingBuffer<const N: usize> {
    buf: [u8; N],
    head: usize,
    len: usize,
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> RingBuffer<N> {
    /// Create an empty ring buffer.
    #[inline]
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            head: 0,
            len: 0,
        }
    }

    /// Number of buffered bytes.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Whether the buffer is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Whether the buffer is full.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Append a byte, returning `false` if the buffer is full.
    #[inline]
    pub fn push(&mut self, byte: u8) -> bool {
        if self.is_full() {
            return false;
        }
        self.buf[self.head] = byte;
        self.head = (self.head + 1) % N;
        self.len += 1;
        true
    }

    /// Remove and return the oldest byte, if any.
    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let tail = (self.head + N - self.len) % N;
        let byte = self.buf[tail];
        self.len -= 1;
        Some(byte)
    }
}
