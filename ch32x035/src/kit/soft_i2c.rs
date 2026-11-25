//! Software (bit-banged) I2C master.
//!
//! The CH32X035 integrates exactly one hardware I2C peripheral (I2C1). When a
//! second bus is needed — or the I2C1 pins are occupied — [`SoftI2c`] provides
//! a bit-banged master on any two GPIO pins.
//!
//! # Open-drain emulation
//!
//! The chip has no true open-drain GPIO mode, so each line is toggled between
//! push-pull output-low and input-with-pull-up. "Driving a `0`" configures the
//! pin as an output and pulls it low; "releasing" the line configures it as a
//! pull-up input, letting a slave pull it low (ACK on SDA, clock stretching on
//! SCL).
//!
//! # Features
//!
//! - 7-bit slave addressing, with START / STOP / repeated-START.
//! - ACK/NACK detection on every written byte.
//! - Clock stretching: SCL is released each cycle and the master waits for the
//!   slave to release it, bounded by a configurable timeout ([`Config`]).
//! - Mirrors the hardware [`I2c1`](crate::i2c::I2c1) API: `try_write` /
//!   `try_read` / `try_write_read` return a [`Result`], while `write` / `read`
//!   / `write_read` panic on error. `SoftI2c` shares the same method signatures
//!   as `I2c1`; if a caller needs a bus abstraction (e.g. an `I2cBus` trait),
//!   it can define the trait itself and implement it for both types.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{
//!     CorePeripherals, Delay, DelayUs, Rcc, gpio::Pins,
//!     kit::soft_i2c::{SoftI2c, Config},
//! };
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let pins = Pins::init(dp.GPIOA, dp.GPIOB, dp.GPIOC, &mut rcc);
//! let cp = CorePeripherals::take().unwrap();
//! let delay = Delay::init(cp.SYST, 48_000_000);
//!
//! // SDA = PA1, SCL = PA2 (port 0 = A)
//! let mut i2c = SoftI2c::new(&pins, (0, 1), (0, 2), delay, Config::default());
//! i2c.write(0x50, &[0x00, 0xAA]); // write 0xAA to register 0x00
//! ```

use crate::DelayUs;
use crate::gpio::Pins;

/// I2C bus speed for the bit-banged driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    /// Standard mode (100 kHz nominal).
    Standard,
    /// Fast mode (400 kHz nominal).
    Fast,
}

impl Speed {
    /// Half-period delay in microseconds used for the bit timing.
    #[inline(always)]
    fn half_period_us(self) -> u32 {
        match self {
            Speed::Standard => 5,
            Speed::Fast => 1,
        }
    }
}

/// Software I2C configuration.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Bus speed.
    pub speed: Speed,
    /// Maximum time (µs) to wait for a slave to release SCL (clock
    /// stretching) before returning [`Error::Timeout`].
    pub stretch_timeout_us: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed: Speed::Standard,
            stretch_timeout_us: 5_000,
        }
    }
}

/// Errors reported by the bit-banged I2C master.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The addressed slave did not acknowledge a byte.
    Nack,
    /// A slave held SCL low (clock stretching) beyond the configured timeout.
    Timeout,
}

/// Bit-banged I2C master on two GPIO pins.
///
/// The SDA/SCL lines are addressed as `(port, pin)` tuples (see
/// [`SoftI2c::new`]) and driven in open-drain style through the
/// runtime-indexed GPIO accessors.
/// `D` is the delay provider (usually [`Delay`](crate::delay::Delay)).
pub struct SoftI2c<D: DelayUs> {
    sda: (u32, u32),
    scl: (u32, u32),
    delay: D,
    /// Half-period of the SCL clock in microseconds.
    half_period_us: u32,
    /// Clock-stretch timeout in microseconds.
    stretch_timeout_us: u32,
}

impl<D: DelayUs> SoftI2c<D> {
    /// Create a bit-banged I2C master on the `sda`/`scl` pins.
    ///
    /// `pins` proves the GPIO ports have been initialized via [`Pins::init`],
    /// which the runtime GPIO accessors require. `sda`/`scl` are `(port, pin)`
    /// tuples (`port`: 0 = A, 1 = B, 2 = C; `pin`: 0..=23) and are validated
    /// against the selected package. Both lines are released (pulled high) so
    /// the bus starts in the idle state. `config` selects the bit rate and
    /// clock-stretch timeout.
    pub fn new(_pins: &Pins, sda: (u32, u32), scl: (u32, u32), delay: D, config: Config) -> Self {
        let mut bus = Self {
            sda,
            scl,
            delay,
            half_period_us: config.speed.half_period_us(),
            stretch_timeout_us: config.stretch_timeout_us,
        };
        bus.release_bus();
        bus
    }

    /// Create the master at the given [`Speed`], with the default
    /// clock-stretch timeout.
    pub fn with_speed(
        pins: &Pins,
        sda: (u32, u32),
        scl: (u32, u32),
        delay: D,
        speed: Speed,
    ) -> Self {
        Self::new(
            pins,
            sda,
            scl,
            delay,
            Config {
                speed,
                ..Config::default()
            },
        )
    }

    // =====================================================================
    // Low-level line drivers (open-drain emulation)
    // =====================================================================

    /// Release both lines (pull-up inputs), putting the bus in the idle state.
    #[inline(always)]
    fn release_bus(&mut self) {
        self.sda_release();
        self.scl_release();
    }

    /// Drive SCL low.
    #[inline(always)]
    fn scl_low(&mut self) {
        // SAFETY: `new` requires a `&Pins` (GPIO initialized) and `&mut self`
        // guarantees exclusive access; the pin was validated at construction.
        unsafe { crate::gpio::set_low(self.scl) };
    }

    /// Release SCL (input pull-up; a slave may stretch it low).
    #[inline(always)]
    fn scl_release(&mut self) {
        // SAFETY: see `scl_low`.
        unsafe { crate::gpio::into_input_pullup(self.scl) };
    }

    /// Raise SCL and wait for the slave to release it (clock stretching),
    /// returning [`Error::Timeout`] if it stays low for too long.
    fn scl_high(&mut self) -> Result<(), Error> {
        // SAFETY: see `scl_low`.
        unsafe { crate::gpio::into_input_pullup(self.scl) };
        let mut waited = 0u32;
        // SAFETY: see `scl_low`.
        while !unsafe { crate::gpio::get(self.scl) } {
            if waited >= self.stretch_timeout_us {
                return Err(Error::Timeout);
            }
            self.delay.delay_us(1);
            waited += 1;
        }
        Ok(())
    }

    /// Drive SDA low.
    #[inline(always)]
    fn sda_low(&mut self) {
        // SAFETY: see `scl_low`.
        unsafe { crate::gpio::set_low(self.sda) };
    }

    /// Release SDA (input pull-up → high unless a slave pulls it low).
    #[inline(always)]
    fn sda_release(&mut self) {
        // SAFETY: see `scl_low`.
        unsafe { crate::gpio::into_input_pullup(self.sda) };
    }

    /// Write one data bit: `high` = release (1), `false` = drive low (0).
    #[inline(always)]
    fn sda_write(&mut self, high: bool) {
        if high {
            self.sda_release();
        } else {
            self.sda_low();
        }
    }

    /// Sample the current SDA level (the line must already be released).
    #[inline(always)]
    fn sda_read(&mut self) -> bool {
        // SAFETY: see `scl_low`.
        unsafe { crate::gpio::get(self.sda) }
    }

    // =====================================================================
    // I2C protocol primitives
    // =====================================================================

    /// START condition: SDA falls while SCL is high.
    fn start(&mut self) -> Result<(), Error> {
        self.sda_release();
        self.scl_high()?;
        self.delay.delay_us(self.half_period_us);
        self.sda_low();
        self.delay.delay_us(self.half_period_us);
        self.scl_low();
        Ok(())
    }

    /// STOP condition: SDA rises while SCL is high.
    fn stop(&mut self) -> Result<(), Error> {
        self.sda_low();
        self.scl_high()?;
        self.delay.delay_us(self.half_period_us);
        self.sda_release();
        self.delay.delay_us(self.half_period_us);
        Ok(())
    }

    /// Transmit one byte MSB-first; returns `true` if the slave ACKed.
    fn write_byte(&mut self, byte: u8) -> Result<bool, Error> {
        for i in (0..8).rev() {
            self.sda_write((byte >> i) & 1 != 0);
            self.delay.delay_us(self.half_period_us);
            self.scl_high()?;
            self.delay.delay_us(self.half_period_us);
            self.scl_low();
        }
        // 9th clock: the master releases SDA and samples the ACK bit.
        self.sda_release();
        self.delay.delay_us(self.half_period_us);
        self.scl_high()?;
        self.delay.delay_us(self.half_period_us);
        let ack = !self.sda_read();
        self.scl_low();
        Ok(ack)
    }

    /// Receive one byte MSB-first. `ack = true` drives an ACK after the byte
    /// (for every byte except the last of a read).
    fn read_byte(&mut self, ack: bool) -> Result<u8, Error> {
        let mut byte = 0u8;
        for i in (0..8).rev() {
            self.sda_release();
            self.scl_high()?;
            self.delay.delay_us(self.half_period_us);
            if self.sda_read() {
                byte |= 1 << i;
            }
            self.scl_low();
            self.delay.delay_us(self.half_period_us);
        }
        // 9th clock: the master drives ACK (low) or NACK (release → high).
        self.sda_write(ack);
        self.delay.delay_us(self.half_period_us);
        self.scl_high()?;
        self.delay.delay_us(self.half_period_us);
        self.scl_low();
        self.delay.delay_us(self.half_period_us);
        Ok(byte)
    }

    /// Send the 7-bit slave address with the R/W bit, checking the ACK.
    fn send_addr(&mut self, addr: u8, read: bool) -> Result<(), Error> {
        let byte = (addr << 1) | (read as u8);
        if self.write_byte(byte)? {
            Ok(())
        } else {
            Err(Error::Nack)
        }
    }

    // =====================================================================
    // Transactions
    // =====================================================================

    /// Write `data` to the slave at 7-bit `addr` (panics on error).
    pub fn write(&mut self, addr: u8, data: &[u8]) {
        self.try_write(addr, data)
            .unwrap_or_else(|e| panic!("soft I2C write failed: {e:?}"));
    }

    /// Write `data` to the slave at 7-bit `addr`.
    ///
    /// A STOP is always issued — even on error — so the bus is left in a
    /// defined state.
    pub fn try_write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.start()?;
        let result = self.send_addr(addr, false).and_then(|_| {
            for &b in data {
                if !self.write_byte(b)? {
                    return Err(Error::Nack);
                }
            }
            Ok(())
        });
        match result {
            Ok(()) => self.stop().map(|_| ()),
            Err(e) => {
                let _ = self.stop();
                Err(e)
            }
        }
    }

    /// Read `buffer.len()` bytes from the slave at 7-bit `addr` (panics on
    /// error).
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) {
        self.try_read(addr, buffer)
            .unwrap_or_else(|e| panic!("soft I2C read failed: {e:?}"));
    }

    /// Read `buffer.len()` bytes from the slave at 7-bit `addr`.
    ///
    /// Every byte is ACKed except the last, which is NACKed so the slave
    /// releases the bus.
    pub fn try_read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        if buffer.is_empty() {
            return Ok(());
        }
        self.start()?;
        let result = self.send_addr(addr, true).and_then(|_| {
            let len = buffer.len();
            for (i, slot) in buffer.iter_mut().enumerate() {
                let last = i == len - 1;
                *slot = self.read_byte(!last)?;
            }
            Ok(())
        });
        match result {
            Ok(()) => self.stop().map(|_| ()),
            Err(e) => {
                let _ = self.stop();
                Err(e)
            }
        }
    }

    /// Write `write` then (re)START and read into `read`, without releasing
    /// the bus in between (panics on error).
    pub fn write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) {
        self.try_write_read(addr, write, read)
            .unwrap_or_else(|e| panic!("soft I2C write_read failed: {e:?}"));
    }

    /// Repeated-START transaction: write `write`, then read into `read`.
    pub fn try_write_read(&mut self, addr: u8, write: &[u8], read: &mut [u8]) -> Result<(), Error> {
        if write.is_empty() {
            return self.try_read(addr, read);
        }

        self.start()?;
        let result = self.send_addr(addr, false).and_then(|_| {
            for &b in write {
                if !self.write_byte(b)? {
                    return Err(Error::Nack);
                }
            }
            Ok(())
        });
        if let Err(e) = result {
            let _ = self.stop();
            return Err(e);
        }
        if read.is_empty() {
            return self.stop().map(|_| ());
        }

        // Repeated START without releasing the bus, then the read phase.
        self.start()?;
        let result = self.send_addr(addr, true).and_then(|_| {
            let len = read.len();
            for (i, slot) in read.iter_mut().enumerate() {
                let last = i == len - 1;
                *slot = self.read_byte(!last)?;
            }
            Ok(())
        });
        match result {
            Ok(()) => self.stop().map(|_| ()),
            Err(e) => {
                let _ = self.stop();
                Err(e)
            }
        }
    }
}

// Error mapping: `SoftI2c` errors convert into the hardware `I2c1` error type,
// so callers can unify error handling (bus-abstraction traits are defined by
// the caller if needed).
impl From<Error> for crate::i2c::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Nack => Self::Acknowledge,
            Error::Timeout => Self::Timeout,
        }
    }
}
