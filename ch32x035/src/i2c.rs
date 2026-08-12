//! I2C driver
//!
//! Blocking master-mode I2C for I2C1 (APB1). Implements the standard
//! START/STOP/ADDR/data transfer sequence with flag polling.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, Clocks, i2c::{I2c1, Config}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = Clocks::new(&rcc);
//! let mut i2c = I2c1::init(dp.I2C1, &mut rcc, clocks, Config::default());
//! i2c.write(0x50, &[0x00, 0xAA]);
//! ```

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
    pub fn read(&mut self, addr: u8, buf: &mut [u8]) {
        let len = buf.len();
        self.start();
        self.send_addr(addr, true);

        for (i, slot) in buf.iter_mut().enumerate() {
            // For the last byte, send STOP after the byte is received.
            if i + 1 == len {
                while !self.i2c.star1.rx_ne() {}
                // Wait for BTF (data available, stop can be sent)
                while !self.i2c.star1.btf() {}
                self.stop();
                *slot = self.i2c.datar.datar() as u8;
            } else {
                while !self.i2c.star1.rx_ne() {}
                *slot = self.i2c.datar.datar() as u8;
            }
        }
    }

    /// Write `write` then (re)start and read into `read`.
    pub fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) {
        self.write(addr, write);
        self.read(addr, read);
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::I2C1 {
        self.i2c
    }
}
