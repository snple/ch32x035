//! I2C driver
//!
//! Blocking I2C for I2C1 (APB1), in both **master** and **slave** modes,
//! implemented with the standard START/STOP/ADDR/data sequence and flag
//! polling. Clock stretching is left enabled, so a blocking slave pauses the
//! bus while it processes each transaction.
//!
//! # Interrupt support
//!
//! Both drivers expose the low-level interrupt building blocks — enable and
//! disable interrupt sources, read status flags, move data and clear flags
//! with the required register-read/write sequences — so an application can
//! write its own interrupt-driven state machine with
//! [`bind`](crate::bind). The HAL deliberately does **not** implement that
//! state machine.
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

use crate::Steal;
use crate::dma::{DataSize, DmaChannel, DmaDir};
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

/// I2C error types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timed out waiting for a bus event.
    Timeout,
    /// No acknowledge received from the addressed slave.
    Acknowledge,
    /// Bus error detected.
    BusError,
    /// Arbitration lost.
    Arbitration,
}

/// Generate the interrupt-support API shared by the I2C1 master and slave
/// drivers.
///
/// These are low-level building blocks: enable/disable interrupt sources,
/// poll status flags and move data. The actual interrupt state machine is left
/// to the application (typically written with [`bind`](crate::bind)).
macro_rules! impl_i2c_interrupt {
    ($name:ident) => {
        impl $name {
            /// Enable the event interrupt (SB, ADDR, ADD10, BTF, STOPF).
            #[inline(always)]
            pub fn enable_event_interrupt(&mut self) {
                self.i2c.ctlr2.set_itevten();
            }

            /// Enable the buffer interrupt (RXNE, TXE).
            #[inline(always)]
            pub fn enable_buffer_interrupt(&mut self) {
                self.i2c.ctlr2.set_itbufen();
            }

            /// Enable the error interrupt (BERR, ARLO, AF, OVR, PECERR).
            #[inline(always)]
            pub fn enable_error_interrupt(&mut self) {
                self.i2c.ctlr2.set_iterren();
            }

            /// Disable all I2C interrupt sources.
            #[inline]
            pub fn disable_interrupts(&mut self) {
                self.i2c.ctlr2.clr_itevten();
                self.i2c.ctlr2.clr_itbufen();
                self.i2c.ctlr2.clr_iterren();
            }

            /// Received byte ready (RXNE flag).
            #[inline(always)]
            pub fn rx_not_empty(&self) -> bool {
                self.i2c.star1.rx_ne()
            }

            /// Data register empty, ready for the next byte (TXE flag).
            #[inline(always)]
            pub fn tx_empty(&self) -> bool {
                self.i2c.star1.tx_e()
            }

            /// Byte transfer finished (BTF flag).
            #[inline(always)]
            pub fn byte_transfer_finished(&self) -> bool {
                self.i2c.star1.btf()
            }

            /// Write the data register (from an interrupt handler).
            #[inline(always)]
            pub fn write_data(&mut self, byte: u8) {
                self.i2c.datar.write_datar(byte as u32);
            }

            /// Read the data register (from an interrupt handler).
            #[inline(always)]
            pub fn read_data(&self) -> u8 {
                self.i2c.datar.datar() as u8
            }

            /// Clear the ADDR flag (read STAR1, then read STAR2).
            #[inline(always)]
            pub fn clear_address(&mut self) {
                let _ = self.i2c.star1.read_bits();
                let _ = self.i2c.star2.read_bits();
            }

            /// Clear the error flags (AF, BERR, ARLO).
            #[inline]
            pub fn clear_errors(&mut self) {
                self.i2c.star1.clr_af();
                self.i2c.star1.clr_berr();
                self.i2c.star1.clr_arlo();
            }
        }
    };
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

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialized and that no
    /// data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            i2c: unsafe { pac::I2C1::steal() },
            clocks: unsafe { Clocks::steal() },
        })
    }

    /// Check the error flags, returning the matching error.
    #[inline]
    fn check_error(&self) -> Result<(), Error> {
        match self.pending_error() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// The currently-pending I2C error, if any.
    #[inline(always)]
    pub fn pending_error(&self) -> Option<Error> {
        if self.i2c.star1.af() {
            Some(Error::Acknowledge)
        } else if self.i2c.star1.berr() {
            Some(Error::BusError)
        } else if self.i2c.star1.arlo() {
            Some(Error::Arbitration)
        } else {
            None
        }
    }

    /// Send a START condition and wait for SB.
    fn try_start(&mut self) -> Result<(), Error> {
        self.i2c.ctlr1.set_start();
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.sb() || self.i2c.star1.berr() || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()
    }

    /// Write the 7-bit slave address and wait for ADDR.
    fn try_send_addr(&mut self, addr: u8, read: bool) -> Result<(), Error> {
        let byte = ((addr as u32) << 1) | (read as u32);
        self.i2c.datar.write_datar(byte);
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.addr()
                || self.i2c.star1.af()
                || self.i2c.star1.berr()
                || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;
        // Clear ADDR by reading STAR2
        let _ = self.i2c.star2.read_bits();
        Ok(())
    }

    /// Write `data` to the slave at 7-bit `addr`.
    pub fn write(&mut self, addr: u8, data: &[u8]) {
        self.try_write(addr, data)
            .unwrap_or_else(|e| panic!("I2C write failed: {e:?}"));
    }

    /// Write `data`, returning an error on timeout / bus fault.
    pub fn try_write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.try_start()?;
        self.try_send_addr(addr, false)?;
        for &b in data {
            if !crate::spin::try_spin_wait(|| {
                self.i2c.star1.tx_e() || self.i2c.star1.berr() || self.i2c.star1.arlo()
            }) {
                return Err(Error::Timeout);
            }
            self.check_error()?;
            self.i2c.datar.write_datar(b as u32);
        }
        // Wait for the last byte to be transferred
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;
        self.stop();
        Ok(())
    }

    /// Read `buf.len()` bytes from the slave at 7-bit `addr`.
    ///
    /// Uses the standard STM32-style sequence: every byte is acknowledged
    /// except the last, which is NACKed so the slave releases the bus.
    pub fn read(&mut self, addr: u8, buf: &mut [u8]) {
        self.try_read(addr, buf)
            .unwrap_or_else(|e| panic!("I2C read failed: {e:?}"));
    }

    /// Read `buf.len()` bytes, returning an error on timeout / bus fault.
    pub fn try_read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Error> {
        if buf.is_empty() {
            return Ok(());
        }
        self.try_start()?;
        self.try_send_addr(addr, true)?;
        self.try_read_after_addr(buf)
    }

    /// Write `write` then, without releasing the bus, (re)start and read into
    /// `read` (a proper repeated-START transaction).
    pub fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) {
        self.try_write_read(addr, write, read)
            .unwrap_or_else(|e| panic!("I2C write_read failed: {e:?}"));
    }

    /// Repeated-START transaction, returning an error on timeout / bus fault.
    pub fn try_write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<(), Error> {
        if write.is_empty() {
            return self.try_read(addr, read);
        }

        // Write phase: START + address + data, keeping the bus held.
        self.try_start()?;
        self.try_send_addr(addr, false)?;
        for &b in write {
            if !crate::spin::try_spin_wait(|| {
                self.i2c.star1.tx_e() || self.i2c.star1.berr() || self.i2c.star1.arlo()
            }) {
                return Err(Error::Timeout);
            }
            self.check_error()?;
            self.i2c.datar.write_datar(b as u32);
        }
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;

        if read.is_empty() {
            self.stop();
            return Ok(());
        }

        // Repeated START + read phase.
        self.try_start()?;
        self.try_send_addr(addr, true)?;
        self.try_read_after_addr(read)
    }

    /// Write `data` to the slave at 10-bit `addr`.
    pub fn write_10bit(&mut self, addr: u16, data: &[u8]) {
        self.try_write_10bit(addr, data)
            .unwrap_or_else(|e| panic!("I2C write failed: {e:?}"));
    }

    /// 10-bit write, returning an error on timeout / bus fault.
    pub fn try_write_10bit(&mut self, addr: u16, data: &[u8]) -> Result<(), Error> {
        self.try_start()?;
        self.try_send_addr_10bit(addr, false)?;
        for &b in data {
            if !crate::spin::try_spin_wait(|| {
                self.i2c.star1.tx_e() || self.i2c.star1.berr() || self.i2c.star1.arlo()
            }) {
                return Err(Error::Timeout);
            }
            self.check_error()?;
            self.i2c.datar.write_datar(b as u32);
        }
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;
        self.stop();
        Ok(())
    }

    /// Read `buf.len()` bytes from the slave at 10-bit `addr`.
    pub fn read_10bit(&mut self, addr: u16, buf: &mut [u8]) {
        self.try_read_10bit(addr, buf)
            .unwrap_or_else(|e| panic!("I2C read failed: {e:?}"));
    }

    /// 10-bit read, returning an error on timeout / bus fault.
    pub fn try_read_10bit(&mut self, addr: u16, buf: &mut [u8]) -> Result<(), Error> {
        if buf.is_empty() {
            return Ok(());
        }
        // 10-bit read = header write phase, then repeated START + read.
        self.try_start()?;
        self.try_send_addr_10bit(addr, false)?;
        self.try_start()?;
        self.try_send_addr_10bit(addr, true)?;
        self.try_read_after_addr(buf)
    }

    /// Repeated-START 10-bit write/read transaction.
    pub fn write_read_10bit(&mut self, addr: u16, write: &[u8], read: &mut [u8]) {
        self.try_write_read_10bit(addr, write, read)
            .unwrap_or_else(|e| panic!("I2C write_read failed: {e:?}"));
    }

    /// Repeated-START 10-bit transaction, returning an error on fault.
    pub fn try_write_read_10bit(
        &mut self,
        addr: u16,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Error> {
        if write.is_empty() {
            return self.try_read_10bit(addr, read);
        }

        self.try_start()?;
        self.try_send_addr_10bit(addr, false)?;
        for &b in write {
            if !crate::spin::try_spin_wait(|| {
                self.i2c.star1.tx_e() || self.i2c.star1.berr() || self.i2c.star1.arlo()
            }) {
                return Err(Error::Timeout);
            }
            self.check_error()?;
            self.i2c.datar.write_datar(b as u32);
        }
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;

        if read.is_empty() {
            self.stop();
            return Ok(());
        }

        self.try_start()?;
        self.try_send_addr_10bit(addr, true)?;
        self.try_read_after_addr(read)
    }

    /// Send a 10-bit address (header byte + low byte) and clear ADDR.
    fn try_send_addr_10bit(&mut self, addr: u16, read: bool) -> Result<(), Error> {
        // First byte: 11110 A9 A8 + R/W.
        let first = 0xF0 | (((addr >> 8) as u32 & 0x03) << 1) | read as u32;
        self.i2c.datar.write_datar(first);
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.add10()
                || self.i2c.star1.af()
                || self.i2c.star1.berr()
                || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;
        // Second byte: low 8 bits of the address.
        self.i2c.datar.write_datar((addr & 0xFF) as u32);
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.addr()
                || self.i2c.star1.af()
                || self.i2c.star1.berr()
                || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.check_error()?;
        // Clear ADDR by reading STAR2.
        let _ = self.i2c.star2.read_bits();
        Ok(())
    }

    /// ACK-managed receive loop; expects START + address already sent and
    /// `ADDR` cleared by the caller.
    fn try_read_after_addr(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let len = buf.len();
        match len {
            1 => {
                // Single byte: NACK it so the slave ends the transaction.
                self.i2c.ctlr1.clr_pos();
                self.i2c.ctlr1.clr_ack();
                if !crate::spin::try_spin_wait(|| {
                    self.i2c.star1.rx_ne() || self.i2c.star1.berr() || self.i2c.star1.arlo()
                }) {
                    return Err(Error::Timeout);
                }
                self.check_error()?;
                self.stop();
                buf[0] = self.i2c.datar.datar() as u8;
            }
            2 => {
                // Two bytes: POS=1 + ACK=1 NACKs the second byte
                // automatically; wait BTF, STOP, then read both.
                self.i2c.ctlr1.set_pos();
                self.i2c.ctlr1.set_ack();
                if !crate::spin::try_spin_wait(|| {
                    self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
                }) {
                    return Err(Error::Timeout);
                }
                self.check_error()?;
                self.stop();
                buf[0] = self.i2c.datar.datar() as u8;
                buf[1] = self.i2c.datar.datar() as u8;
            }
            _ => {
                // Three or more: ACK every byte but the last.
                self.i2c.ctlr1.clr_pos();
                self.i2c.ctlr1.set_ack();
                for slot in buf[..len - 2].iter_mut() {
                    if !crate::spin::try_spin_wait(|| {
                        self.i2c.star1.rx_ne() || self.i2c.star1.berr() || self.i2c.star1.arlo()
                    }) {
                        return Err(Error::Timeout);
                    }
                    self.check_error()?;
                    *slot = self.i2c.datar.datar() as u8;
                }
                // Last-but-one byte: wait BTF, NACK, read it.
                if !crate::spin::try_spin_wait(|| {
                    self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
                }) {
                    return Err(Error::Timeout);
                }
                self.check_error()?;
                self.i2c.ctlr1.clr_ack();
                buf[len - 2] = self.i2c.datar.datar() as u8;
                // Last byte: RXNE, STOP, read.
                if !crate::spin::try_spin_wait(|| {
                    self.i2c.star1.rx_ne() || self.i2c.star1.berr() || self.i2c.star1.arlo()
                }) {
                    return Err(Error::Timeout);
                }
                self.check_error()?;
                self.stop();
                buf[len - 1] = self.i2c.datar.datar() as u8;
            }
        }
        Ok(())
    }

    /// Write `data` to the slave at 7-bit `addr` using DMA.
    ///
    /// Enables the I2C DMA request and starts a memory-to-peripheral transfer
    /// into DATAR; the transfer is paced by the TXE flag.
    pub fn write_dma<const CH: u8>(
        &mut self,
        ch: &DmaChannel<CH>,
        addr: u8,
        data: &[u8],
    ) -> Result<(), Error> {
        self.i2c.ctlr2.set_dmaen();
        self.try_start()?;
        self.try_send_addr(addr, false)?;
        ch.start(
            DmaDir::MemoryToPeripheral,
            pac::i2c1::BASE + 0x10, // DATAR
            data.as_ptr() as u32,
            data.len() as u16,
            DataSize::Byte,
            DataSize::Byte,
            false,
            true,
        );
        ch.try_wait().map_err(|_| Error::Timeout)?;
        if !crate::spin::try_spin_wait(|| {
            self.i2c.star1.btf() || self.i2c.star1.berr() || self.i2c.star1.arlo()
        }) {
            return Err(Error::Timeout);
        }
        self.stop();
        Ok(())
    }

    /// Read `data.len()` bytes from the slave at 7-bit `addr` using DMA.
    ///
    /// The `LAST` bit makes the hardware NACK the final byte so the slave
    /// releases the bus.
    pub fn read_dma<const CH: u8>(
        &mut self,
        ch: &DmaChannel<CH>,
        addr: u8,
        data: &mut [u8],
    ) -> Result<(), Error> {
        if data.is_empty() {
            return Ok(());
        }
        self.i2c.ctlr2.set_dmaen();
        self.try_start()?;
        self.try_send_addr(addr, true)?;
        self.i2c.ctlr2.set_last();
        ch.start(
            DmaDir::PeripheralToMemory,
            pac::i2c1::BASE + 0x10, // DATAR
            data.as_ptr() as u32,
            data.len() as u16,
            DataSize::Byte,
            DataSize::Byte,
            false,
            true,
        );
        ch.try_wait().map_err(|_| Error::Timeout)?;
        self.i2c.ctlr2.clr_last();
        self.stop();
        Ok(())
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::I2C1 {
        self.i2c
    }

    /// Start condition generated (SB flag).
    #[inline(always)]
    pub fn start_sent(&self) -> bool {
        self.i2c.star1.sb()
    }

    /// Address sent (ADDR flag).
    #[inline(always)]
    pub fn address_sent(&self) -> bool {
        self.i2c.star1.addr()
    }

    /// 10-bit address header sent (ADD10 flag).
    #[inline(always)]
    pub fn addr10_sent(&self) -> bool {
        self.i2c.star1.add10()
    }

    /// Send a START condition (from an interrupt handler).
    #[inline(always)]
    pub fn start(&mut self) {
        self.i2c.ctlr1.set_start();
    }

    /// Send a STOP condition (from an interrupt handler).
    #[inline(always)]
    pub fn stop(&mut self) {
        self.i2c.ctlr1.set_stop();
    }

    /// Enable or disable ACK generation for received bytes.
    #[inline(always)]
    pub fn set_ack(&mut self, ack: bool) {
        if ack {
            self.i2c.ctlr1.set_ack();
        } else {
            self.i2c.ctlr1.clr_ack();
        }
    }

    /// Set the ACK position for two-byte reception (`POS` bit).
    #[inline(always)]
    pub fn set_ack_position(&mut self, second_last: bool) {
        if second_last {
            self.i2c.ctlr1.set_pos();
        } else {
            self.i2c.ctlr1.clr_pos();
        }
    }
}

impl_i2c_interrupt!(I2c1);

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

    /// Create a new I2C1 slave with a 10-bit `address`, resetting and
    /// clock-enabling the peripheral.
    pub fn init_10bit(i2c: pac::I2C1, rcc: &mut Rcc, clocks: Clocks, address: u16) -> Self {
        i2c.init(rcc);

        let s = Self { i2c };

        s.i2c.ctlr1.clr_pe();

        let pclk1 = clocks.apb1_hz();
        let freq = (pclk1 / 1_000_000).clamp(2, 50);
        s.i2c.ctlr2.write_freq(freq);

        // Own address in 10-bit mode.
        s.i2c.oaddr1.set_addmode();
        s.i2c.oaddr1.write_add9_8(((address >> 8) & 0x03) as u32);
        s.i2c.oaddr1.write_add7_1(((address >> 1) & 0x7F) as u32);
        if address & 1 != 0 {
            s.i2c.oaddr1.set_add0();
        } else {
            s.i2c.oaddr1.clr_add0();
        }

        s.i2c.ctlr1.set_ack();
        s.i2c.ctlr1.set_pe();

        s
    }

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialized and that no
    /// data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            i2c: unsafe { pac::I2C1::steal() },
        })
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
    #[inline(always)]
    pub fn read_requested(&self) -> bool {
        self.i2c.star2.tra()
    }

    /// True if a STOP condition was detected on the bus after a byte was
    /// acknowledged (slave mode).
    #[inline(always)]
    pub fn stop_detected(&self) -> bool {
        self.i2c.star1.stopf()
    }

    /// Clear the stop-detection flag (read STAR1, then write CTLR1).
    #[inline(always)]
    pub fn clear_stop(&mut self) {
        let _ = self.i2c.star1.read_bits();
        self.i2c.ctlr1.write_val(self.i2c.ctlr1.read_val());
    }

    /// True if an error flag (acknowledge failure, bus error or arbitration
    /// lost) is set.
    #[inline(always)]
    pub fn has_error(&self) -> bool {
        self.i2c.star1.af() || self.i2c.star1.berr() || self.i2c.star1.arlo()
    }

    /// True while the master is addressing this slave (ADDR flag).
    ///
    /// Useful from an interrupt handler before calling
    /// [`clear_address`](Self::clear_address).
    #[inline(always)]
    pub fn address_matched(&self) -> bool {
        self.i2c.star1.addr()
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::I2C1 {
        self.i2c
    }
}

impl_i2c_interrupt!(I2c1Slave);
