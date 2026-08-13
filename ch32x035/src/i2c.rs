//! I2C driver
//!
//! Blocking I2C for I2C1 (APB1), in both **master** and **slave** modes,
//! implemented with the standard START/STOP/ADDR/data sequence and flag
//! polling. Clock stretching is left enabled, so a blocking slave pauses the
//! bus while it processes each transaction.
//!
//! # Example (master)
//!
//! ```ignore
//! use ch32x035::{Rcc, Clocks, i2c::{I2c1, Config}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = Clocks::new(&rcc);
//! let mut i2c = I2c1::init(dp.I2C1, &mut rcc, clocks, Config::default());
//! i2c.write(0x50, &[0x00, 0xAA]);
//! ```
//!
//! # Example (slave)
//!
//! ```ignore
//! use ch32x035::{Rcc, Clocks, i2c::{I2c1Slave, Transfer}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = Clocks::new(&rcc);
//! let mut i2c = I2c1Slave::init(dp.I2C1, &mut rcc, clocks, 0x50);
//!
//! let mut rx = [0u8; 8];
//! loop {
//!     match i2c.listen() {
//!         Transfer::Write => { let n = i2c.read(&mut rx); /* handle n bytes */ }
//!         Transfer::Read => { i2c.write(&[0x01, 0x02, 0x03]); }
//!     }
//! }
//! ```

use core::cell::RefCell;
use critical_section::Mutex;

use crate::pac;
use crate::rcc::{Clocks, Rcc};
use crate::reset::Resettable;

/// I2C bus speed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    /// Standard mode (100 kHz)
    Standard,
    /// Fast mode (400 kHz)
    Fast,
}

/// I2C configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Bus speed
    pub speed: Speed,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed: Speed::Standard,
        }
    }
}

/// I2C1 master driver
pub struct I2c1 {
    i2c: pac::I2C1,
    clocks: Clocks,
}

impl I2c1 {
    /// Create a new I2C1 master, resetting and clock-enabling the peripheral.
    pub fn init(i2c: pac::I2C1, rcc: &mut Rcc, clocks: Clocks, config: Config) -> Self {
        i2c.init(rcc);

        let s = Self { i2c, clocks };

        // Disable I2C while configuring
        s.i2c.ctlr1.clr_pe();

        // FREQ = PCLK1 in MHz (must be 2..=50)
        let pclk1 = s.clocks.apb1_hz();
        let freq = (pclk1 / 1_000_000).clamp(2, 50);
        s.i2c.ctlr2.write_freq(freq);

        // CCR for the requested speed
        match config.speed {
            Speed::Standard => {
                // F_S = 0: SCL = PCLK1 / (2 * CCR) → CCR = PCLK1 / (2 * 100k)
                s.i2c.ckcfgr.clr_f_s();
                let ccr = (pclk1 / 200_000).max(4);
                s.i2c.ckcfgr.write_ccr(ccr);
            }
            Speed::Fast => {
                // F_S = 1, DUTY = 0: SCL = PCLK1 / (3 * CCR) → CCR = PCLK1 / (3 * 400k)
                s.i2c.ckcfgr.set_f_s();
                s.i2c.ckcfgr.clr_duty();
                let ccr = (pclk1 / 1_200_000).max(1);
                s.i2c.ckcfgr.write_ccr(ccr);
            }
        }

        // Enable ACK and the I2C peripheral
        s.i2c.ctlr1.set_ack();
        s.i2c.ctlr1.set_pe();

        s
    }

    /// Send a START condition and wait for SB.
    fn start(&mut self) {
        self.i2c.ctlr1.set_start();
        while !self.i2c.star1.sb() {}
    }

    /// Send a STOP condition.
    fn stop(&mut self) {
        self.i2c.ctlr1.set_stop();
    }

    /// Write the 7-bit slave address and wait for ADDR.
    fn send_addr(&mut self, addr: u8, read: bool) {
        let byte = ((addr as u32) << 1) | (read as u32);
        self.i2c.datar.write_datar(byte);
        while !self.i2c.star1.addr() {}
        // Clear ADDR by reading STAR2
        let _ = self.i2c.star2.read_bits();
    }

    /// Write `data` to the slave at 7-bit `addr`.
    pub fn write(&mut self, addr: u8, data: &[u8]) {
        self.start();
        self.send_addr(addr, false);
        for &b in data {
            while !self.i2c.star1.tx_e() {}
            self.i2c.datar.write_datar(b as u32);
        }
        // Wait for the last byte to be transferred
        while !self.i2c.star1.btf() {}
        self.stop();
    }

    /// Read `buf.len()` bytes from the slave at 7-bit `addr`.
    ///
    /// Uses the standard STM32-style sequence: every byte is acknowledged
    /// except the last, which is NACKed so the slave releases the bus.
    pub fn read(&mut self, addr: u8, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }
        self.start();
        self.send_addr(addr, true);
        self.read_after_addr(buf);
    }

    /// Write `write` then, without releasing the bus, (re)start and read into
    /// `read` (a proper repeated-START transaction).
    pub fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) {
        if write.is_empty() {
            self.read(addr, read);
            return;
        }

        // Write phase: START + address + data, keeping the bus held.
        self.start();
        self.send_addr(addr, false);
        for &b in write {
            while !self.i2c.star1.tx_e() {}
            self.i2c.datar.write_datar(b as u32);
        }
        while !self.i2c.star1.btf() {}

        if read.is_empty() {
            self.stop();
            return;
        }

        // Repeated START + read phase.
        self.start();
        self.send_addr(addr, true);
        self.read_after_addr(read);
    }

    /// ACK-managed receive loop; expects START + address already sent and
    /// `ADDR` cleared by the caller.
    fn read_after_addr(&mut self, buf: &mut [u8]) {
        let len = buf.len();
        match len {
            1 => {
                // Single byte: NACK it so the slave ends the transaction.
                self.i2c.ctlr1.clr_pos();
                self.i2c.ctlr1.clr_ack();
                while !self.i2c.star1.rx_ne() {}
                self.stop();
                buf[0] = self.i2c.datar.datar() as u8;
            }
            2 => {
                // Two bytes: POS=1 + ACK=1 NACKs the second byte
                // automatically; wait BTF, STOP, then read both.
                self.i2c.ctlr1.set_pos();
                self.i2c.ctlr1.set_ack();
                while !self.i2c.star1.btf() {}
                self.stop();
                buf[0] = self.i2c.datar.datar() as u8;
                buf[1] = self.i2c.datar.datar() as u8;
            }
            _ => {
                // Three or more: ACK every byte but the last.
                self.i2c.ctlr1.clr_pos();
                self.i2c.ctlr1.set_ack();
                for slot in buf[..len - 2].iter_mut() {
                    while !self.i2c.star1.rx_ne() {}
                    *slot = self.i2c.datar.datar() as u8;
                }
                // Last-but-one byte: wait BTF, NACK, read it.
                while !self.i2c.star1.btf() {}
                self.i2c.ctlr1.clr_ack();
                buf[len - 2] = self.i2c.datar.datar() as u8;
                // Last byte: RXNE, STOP, read.
                while !self.i2c.star1.rx_ne() {}
                self.stop();
                buf[len - 1] = self.i2c.datar.datar() as u8;
            }
        }
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::I2C1 {
        self.i2c
    }
}

// ============================================================================
// Interrupt-driven slave
// ============================================================================

/// Shared state for the interrupt-driven I2C slave.
///
/// Protected by a critical section so it can be touched from both the main
/// loop and the ISRs.
struct IrqState {
    initialized: bool,
    rx_buf: Option<&'static mut [u8]>,
    rx_len: usize,
    tx_buf: &'static [u8],
    tx_pos: usize,
    receiving: bool,
    transmitting: bool,
    write_ready: bool,
    read_requested: bool,
    read_done: bool,
    error: bool,
}

impl IrqState {
    const fn new() -> Self {
        Self {
            initialized: false,
            rx_buf: None,
            rx_len: 0,
            tx_buf: &[],
            tx_pos: 0,
            receiving: false,
            transmitting: false,
            write_ready: false,
            read_requested: false,
            read_done: false,
            error: false,
        }
    }
}

/// Take the next byte to transmit, advancing the position (private helper).
fn tx_next_byte(tx_buf: &[u8], tx_pos: &mut usize) -> Option<u8> {
    if *tx_pos < tx_buf.len() {
        let b = tx_buf[*tx_pos];
        *tx_pos += 1;
        Some(b)
    } else {
        None
    }
}

/// I2C1 register block, accessed inside a critical section (private helper).
///
/// The peripheral lives at a fixed MMIO address; the ISR and `init` methods
/// only touch it while holding the critical section, so this is sound.
fn i2c_regs() -> &'static pac::i2c1::RegisterBlock {
    // SAFETY: fixed MMIO address, only reached under `critical_section`.
    unsafe { &*pac::I2C1::PTR }
}

/// Interrupt-driven I2C1 slave.
///
/// Unlike the blocking [`I2c1Slave`], this driver is driven entirely from the
/// `I2C1_EV_IRQ` / `I2C1_ER_IRQ` handlers and never blocks. It is meant to be
/// stored in a `static` and used together with the [`bind`](crate::bind)
/// attribute:
///
/// ```ignore
/// use ch32x035::{bind, interrupt, pac::Interrupt};
/// use ch32x035::i2c::I2c1SlaveIrq;
///
/// static mut RX: [u8; 32] = [0; 32];
/// static I2C: I2c1SlaveIrq = I2c1SlaveIrq::new();
///
/// #[bind(I2C1_EV_IRQ)]
/// fn i2c1_ev() {
///     I2C.on_event();
/// }
///
/// #[bind(I2C1_ER_IRQ)]
/// fn i2c1_er() {
///     I2C.on_error();
/// }
///
/// fn main() {
///     // ...
///     I2C.init(dp.I2C1, &mut rcc, clocks, 0x50, unsafe { &mut RX });
///     interrupt::enable(Interrupt::I2C1_EV_IRQ);
///     interrupt::enable(Interrupt::I2C1_ER_IRQ);
///     loop {
///         if I2C.read_requested() {
///             I2C.respond(b"payload");
///         }
///         if I2C.write_ready() {
///             let n = I2C.rx_len();
///             // process RX[..n], then:
///             I2C.clear_write_ready();
///         }
///     }
/// }
/// ```
pub struct I2c1SlaveIrq {
    state: Mutex<RefCell<IrqState>>,
}

impl I2c1SlaveIrq {
    /// Create a new (uninitialised) interrupt-driven slave instance.
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(RefCell::new(IrqState::new())),
        }
    }

    /// Initialise the slave with the 7-bit `address` and enable the I2C
    /// event/buffer/error interrupts.
    ///
    /// `rx_buf` is where bytes written by the master are stored; keep your
    /// own reference to it to inspect the data (as in the `static mut RX`
    /// example above).
    ///
    /// # Safety
    ///
    /// Must be called exactly once. The application must keep the buffer
    /// alive and must not read it concurrently with the ISRs.
    pub unsafe fn init(
        &self,
        i2c: pac::I2C1,
        rcc: &mut Rcc,
        clocks: Clocks,
        address: u8,
        rx_buf: &'static mut [u8],
    ) {
        // Reset + clock-enable, and consume the singleton so it cannot be
        // used elsewhere afterwards.
        i2c.init(rcc);

        critical_section::with(|cs| {
            let mut st = self.state.borrow(cs).borrow_mut();
            st.initialized = true;
            st.rx_buf = Some(rx_buf);
            st.rx_len = 0;
            st.tx_buf = &[];
            st.tx_pos = 0;
            st.receiving = false;
            st.transmitting = false;
            st.write_ready = false;
            st.read_requested = false;
            st.read_done = false;
            st.error = false;

            let i2c = i2c_regs();
            i2c.ctlr1.clr_pe();

            // FREQ = PCLK1 in MHz (must be 2..=50)
            let freq = (clocks.apb1_hz() / 1_000_000).clamp(2, 50);
            i2c.ctlr2.write_freq(freq);

            // Own address, 7-bit mode: ADD[7:1] = address, ADD0 = 0
            i2c.oaddr1.clr_addmode();
            i2c.oaddr1.write_add7_1((address & 0x7F) as u32);

            // ACK, clock stretching enabled, event + buffer + error interrupts
            i2c.ctlr1.set_ack();
            i2c.ctlr1.set_pe();
            i2c.ctlr2.set_itevten();
            i2c.ctlr2.set_itbufen();
            i2c.ctlr2.set_iterren();
        });
    }

    /// I2C event interrupt handler. Call this from the `I2C1_EV_IRQ` handler.
    pub fn on_event(&self) {
        critical_section::with(|cs| {
            let mut st = self.state.borrow(cs).borrow_mut();
            if !st.initialized {
                return;
            }
            let i2c = i2c_regs();

            // Address matched: start a new transaction.
            if i2c.star1.addr() {
                if i2c.star2.tra() {
                    // Master reads → slave transmits.
                    st.read_requested = true;
                    if st.tx_buf.is_empty() {
                        // No data yet: keep ADDR set (clock stretching holds
                        // SCL low) and wait for `respond`.
                        i2c.ctlr2.clr_itevten();
                        st.transmitting = false;
                    } else {
                        // Data ready: clear ADDR and write the first byte.
                        let _ = i2c.star1.read_bits();
                        let _ = i2c.star2.read_bits();
                        st.tx_pos = 0;
                        st.transmitting = true;
                        if let Some(b) = tx_next_byte(st.tx_buf, &mut st.tx_pos) {
                            i2c.datar.write_datar(b as u32);
                        }
                    }
                } else {
                    // Master writes → slave receives.
                    let _ = i2c.star1.read_bits();
                    let _ = i2c.star2.read_bits();
                    st.receiving = true;
                    st.rx_len = 0;
                    st.write_ready = false;
                }
                return;
            }

            // Data available (ITBUFEN): receive one byte. Handled before
            // STOPF so the final byte that arrives together with the STOP
            // condition is not lost.
            if st.receiving && i2c.star1.rx_ne() {
                let b = i2c.datar.datar() as u8;
                // Take the buffer out briefly so no long-lived borrow of
                // `st` (through `RefMut`) overlaps other field accesses.
                if let Some(buf) = st.rx_buf.take() {
                    if st.rx_len < buf.len() {
                        buf[st.rx_len] = b;
                        st.rx_len += 1;
                    }
                    // Buffer full: discard the rest to keep SCL flowing.
                    st.rx_buf = Some(buf);
                }
                return;
            }

            // Stop detected after an ACK: the master finished writing.
            if i2c.star1.stopf() {
                st.receiving = false;
                st.write_ready = true;
                // STOPF is cleared by reading STAR1 then writing CTLR1.
                let _ = i2c.star1.read_bits();
                i2c.ctlr1.write_val(i2c.ctlr1.read_val());
                return;
            }

            // Transmit register empty (ITBUFEN): send the next byte.
            if st.transmitting
                && i2c.star1.tx_e()
                && let Some(b) = tx_next_byte(st.tx_buf, &mut st.tx_pos)
            {
                i2c.datar.write_datar(b as u32);
            }
            // No more bytes: the master NACKs and `on_error` finishes.
        });
    }

    /// I2C error interrupt handler. Call this from the `I2C1_ER_IRQ` handler.
    pub fn on_error(&self) {
        critical_section::with(|cs| {
            let mut st = self.state.borrow(cs).borrow_mut();
            if !st.initialized {
                return;
            }
            let i2c = i2c_regs();

            // NACK: ends a transmit (master read) transaction.
            if i2c.star1.af() {
                st.transmitting = false;
                st.receiving = false;
                st.read_done = true;
                i2c.star1.clr_af();
            }
            if i2c.star1.berr() {
                i2c.star1.clr_berr();
                st.error = true;
            }
            if i2c.star1.arlo() {
                i2c.star1.clr_arlo();
                st.error = true;
            }
            if i2c.star1.ovr() {
                i2c.star1.clr_ovr();
                st.error = true;
            }
        });
    }

    /// Number of bytes received in the current (or last) write transaction.
    pub fn rx_len(&self) -> usize {
        critical_section::with(|cs| self.state.borrow(cs).borrow().rx_len)
    }

    /// True when the master has finished a write transaction (STOP received).
    pub fn write_ready(&self) -> bool {
        critical_section::with(|cs| self.state.borrow(cs).borrow().write_ready)
    }

    /// Clear the `write_ready` flag.
    pub fn clear_write_ready(&self) {
        critical_section::with(|cs| self.state.borrow(cs).borrow_mut().write_ready = false);
    }

    /// True while the master is requesting a read (slave transmit direction).
    ///
    /// Call [`respond`](Self::respond) to provide the data as soon as this is
    /// set.
    pub fn read_requested(&self) -> bool {
        critical_section::with(|cs| self.state.borrow(cs).borrow().read_requested)
    }

    /// True when a read (transmit) transaction finished with a NACK.
    pub fn read_done(&self) -> bool {
        critical_section::with(|cs| self.state.borrow(cs).borrow().read_done)
    }

    /// Clear the `read_done` flag.
    pub fn clear_read_done(&self) {
        critical_section::with(|cs| self.state.borrow(cs).borrow_mut().read_done = false);
    }

    /// True if a bus error (BERR/ARLO/OVR) occurred.
    pub fn has_error(&self) -> bool {
        critical_section::with(|cs| self.state.borrow(cs).borrow().error)
    }

    /// Clear the error flag.
    pub fn clear_error(&self) {
        critical_section::with(|cs| self.state.borrow(cs).borrow_mut().error = false);
    }

    /// Provide the data to transmit when the master requests a read.
    ///
    /// This also clears the `read_requested` flag. If the master is already
    /// waiting (ADDR still set because the previous request had no data
    /// yet), this clears ADDR, writes the first byte and re-enables the
    /// event interrupt.
    pub fn respond(&self, data: &'static [u8]) {
        critical_section::with(|cs| {
            let mut st = self.state.borrow(cs).borrow_mut();
            st.tx_buf = data;
            st.tx_pos = 0;
            st.read_requested = false;
            if !st.initialized {
                return;
            }
            let i2c = i2c_regs();
            if i2c.star1.addr() && i2c.star2.tra() {
                // Master is waiting (clock stretched): start transmitting now.
                let _ = i2c.star1.read_bits();
                let _ = i2c.star2.read_bits();
                st.transmitting = true;
                if let Some(b) = tx_next_byte(st.tx_buf, &mut st.tx_pos) {
                    i2c.datar.write_datar(b as u32);
                }
                i2c.ctlr2.set_itevten();
            }
        });
    }
}

impl Default for I2c1SlaveIrq {
    fn default() -> Self {
        Self::new()
    }
}

/// Direction of an in-progress slave transaction, as reported by
/// [`I2c1Slave::listen`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transfer {
    /// The master is writing to the slave: the slave receives data.
    Write,
    /// The master is reading from the slave: the slave transmits data.
    Read,
}

/// I2C1 slave driver.
///
/// Unlike [`I2c1`] (master), this driver answers to its own address on the
/// bus. It is blocking: with clock stretching enabled (the default) the bus
/// is paused while the slave runs, so the master waits for the slave.
///
/// Typical use is a polling loop:
///
/// ```ignore
/// loop {
///     match slave.listen() {
///         Transfer::Write => { let n = slave.read(&mut rx); /* ... */ }
///         Transfer::Read => { slave.write(&tx); }
///     }
/// }
/// ```
pub struct I2c1Slave {
    i2c: pac::I2C1,
}

impl I2c1Slave {
    /// Create a new I2C1 slave with 7-bit `address`, resetting and
    /// clock-enabling the peripheral.
    pub fn init(i2c: pac::I2C1, rcc: &mut Rcc, clocks: Clocks, address: u8) -> Self {
        i2c.init(rcc);

        let s = Self { i2c };

        // Disable I2C while configuring
        s.i2c.ctlr1.clr_pe();

        // FREQ = PCLK1 in MHz (must be 2..=50)
        let pclk1 = clocks.apb1_hz();
        let freq = (pclk1 / 1_000_000).clamp(2, 50);
        s.i2c.ctlr2.write_freq(freq);

        // Own address, 7-bit mode: ADD[7:1] = address, ADD0 = 0
        s.i2c.oaddr1.clr_addmode();
        s.i2c.oaddr1.write_add7_1((address & 0x7F) as u32);

        // ACK addresses/data; clock stretching stays enabled (NOSTRETCH = 0)
        s.i2c.ctlr1.set_ack();
        s.i2c.ctlr1.set_pe();

        s
    }

    /// Block until the master addresses the slave, then return the
    /// transaction direction.
    ///
    /// ADDR is cleared here (read STAR1 then STAR2), so call
    /// [`read`](Self::read) or [`write`](Self::write) right afterwards.
    pub fn listen(&mut self) -> Transfer {
        while !self.i2c.star1.addr() {}
        let dir = if self.i2c.star2.tra() {
            Transfer::Read
        } else {
            Transfer::Write
        };
        // Clear ADDR by reading STAR1 then STAR2
        let _ = self.i2c.star1.read_bits();
        let _ = self.i2c.star2.read_bits();
        dir
    }

    /// Receive bytes from the master into `buf`.
    ///
    /// Blocks until the master issues a STOP condition. Returns the number of
    /// bytes received, which may be less than `buf.len()` if the master sent
    /// fewer.
    ///
    /// Must be called after [`listen`](Self::listen) returned
    /// [`Transfer::Write`].
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut n = 0;
        loop {
            if self.i2c.star1.stopf() {
                // Drain any final byte(s) that arrived before the STOP
                while self.i2c.star1.rx_ne() && n < buf.len() {
                    buf[n] = self.i2c.datar.datar() as u8;
                    n += 1;
                }
                self.clear_stop();
                break;
            }
            if self.i2c.star1.rx_ne() {
                if n < buf.len() {
                    buf[n] = self.i2c.datar.datar() as u8;
                    n += 1;
                } else {
                    // Buffer full: keep the bus flowing by discarding
                    let _ = self.i2c.datar.datar();
                }
            }
        }
        n
    }

    /// Transmit `data` to the master (slave transmitter).
    ///
    /// Blocks until the master has read every byte and NACKs the last one.
    ///
    /// Must be called after [`listen`](Self::listen) returned
    /// [`Transfer::Read`].
    pub fn write(&mut self, data: &[u8]) {
        // The first byte is written right after ADDR was cleared by `listen`
        let mut it = data.iter();
        if let Some(&b) = it.next() {
            self.i2c.datar.write_datar(b as u32);
        }
        for &b in it {
            while !self.i2c.star1.tx_e() {}
            self.i2c.datar.write_datar(b as u32);
        }
        // Wait for the master to NACK the last byte → transfer finished
        while !self.i2c.star1.af() {}
        self.i2c.star1.clr_af();
    }

    /// True while the master is requesting a read (slave transmitter
    /// direction). Only meaningful during the address phase, i.e. before
    /// [`listen`](Self::listen) clears ADDR.
    pub fn read_requested(&self) -> bool {
        self.i2c.star2.tra()
    }

    /// True if a STOP condition was detected on the bus after a byte was
    /// acknowledged (slave mode).
    pub fn stop_detected(&self) -> bool {
        self.i2c.star1.stopf()
    }

    /// Clear the stop-detection flag (read STAR1, then write CTLR1).
    pub fn clear_stop(&mut self) {
        let _ = self.i2c.star1.read_bits();
        self.i2c.ctlr1.write_val(self.i2c.ctlr1.read_val());
    }

    /// True if an error flag (acknowledge failure, bus error or arbitration
    /// lost) is set.
    pub fn has_error(&self) -> bool {
        self.i2c.star1.af() || self.i2c.star1.berr() || self.i2c.star1.arlo()
    }

    /// Clear all error flags.
    pub fn clear_errors(&mut self) {
        self.i2c.star1.clr_af();
        self.i2c.star1.clr_berr();
        self.i2c.star1.clr_arlo();
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::I2C1 {
        self.i2c
    }
}
