//! SPI driver
//!
//! SPI1 (APB2) driver supporting master/slave modes, full- and half-duplex,
//! CRC and DMA transfers.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, Clocks, spi::{Spi1, Config, BaudDiv}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = Clocks::new(&rcc);
//! let mut spi = Spi1::init(dp.SPI1, &mut rcc, clocks, Config::default());
//! spi.write(0xAA);
//! ```

use crate::Steal;
use crate::dma::{DataSize, DmaChannel, DmaDir};
use crate::pac;
use crate::rcc::{Clocks, Rcc};
use crate::reset::Resettable;

/// SPI baud rate prescaler (BR field, bits 5:3)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaudDiv {
    /// PCLK / 2
    Div2,
    /// PCLK / 4
    Div4,
    /// PCLK / 8
    Div8,
    /// PCLK / 16
    Div16,
    /// PCLK / 32
    Div32,
    /// PCLK / 64
    Div64,
    /// PCLK / 128
    Div128,
    /// PCLK / 256
    Div256,
}

impl BaudDiv {
    #[inline]
    const fn br_value(self) -> u32 {
        match self {
            BaudDiv::Div2 => 0b000,
            BaudDiv::Div4 => 0b001,
            BaudDiv::Div8 => 0b010,
            BaudDiv::Div16 => 0b011,
            BaudDiv::Div32 => 0b100,
            BaudDiv::Div64 => 0b101,
            BaudDiv::Div128 => 0b110,
            BaudDiv::Div256 => 0b111,
        }
    }
}

/// SPI clock polarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    /// Clock idles low
    IdleLow,
    /// Clock idles high
    IdleHigh,
}

/// SPI clock phase
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    /// Sample on the first (leading) edge
    FirstEdge,
    /// Sample on the second (trailing) edge
    SecondEdge,
}

/// SPI frame format (data bits)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataBits {
    /// 8-bit frames
    Eight,
    /// 16-bit frames
    Sixteen,
}

/// SPI master/slave role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// SPI master.
    Master,
    /// SPI slave (software slave management, always selected).
    Slave,
}

/// SPI transfer direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Two-line full duplex.
    FullDuplex,
    /// Single-line half duplex, transmit only.
    HalfDuplexTx,
    /// Single-line half duplex, receive only.
    HalfDuplexRx,
    /// Two-line receive-only (master, `MISO` only).
    ReceiveOnly,
}

/// SPI configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Baud rate prescaler
    pub baud_div: BaudDiv,
    /// Clock polarity
    pub cpol: Cpol,
    /// Clock phase
    pub cpha: Cpha,
    /// Frame format
    pub data_bits: DataBits,
    /// Master or slave
    pub mode: Mode,
    /// Transfer direction
    pub direction: Direction,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baud_div: BaudDiv::Div16,
            cpol: Cpol::IdleLow,
            cpha: Cpha::FirstEdge,
            data_bits: DataBits::Eight,
            mode: Mode::Master,
            direction: Direction::FullDuplex,
        }
    }
}

/// SPI error types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timed out waiting for the transfer to complete.
    Timeout,
}

/// Abstract SPI bus trait for driver implementations.
///
/// This keeps drivers decoupled from the concrete [`Spi1`] type.
pub trait SpiBus {
    /// Error type returned by the bus.
    type Error;

    /// Write multiple bytes (8-bit mode), discarding received data.
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Read multiple bytes by clocking out dummy bytes (8-bit mode).
    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;

    /// Full-duplex transfer: write and read simultaneously (8-bit mode).
    ///
    /// The transfer length is `min(read.len(), write.len())`.
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error>;
}

/// SPI1 master driver
pub struct Spi1 {
    spi: pac::SPI1,
    #[allow(dead_code)]
    clocks: Clocks,
}

impl Spi1 {
    /// Create a new SPI1 master, resetting and clock-enabling the peripheral.
    pub fn init(spi: pac::SPI1, rcc: &mut Rcc, clocks: Clocks, config: Config) -> Self {
        spi.init(rcc);

        let s = Self { spi, clocks };

        // Disable SPI while configuring
        s.spi.ctlr1.clr_spe();

        // Data format (DFF)
        match config.data_bits {
            DataBits::Eight => s.spi.ctlr1.clr_dff(),
            DataBits::Sixteen => s.spi.ctlr1.set_dff(),
        }

        // CPOL / CPHA
        match config.cpol {
            Cpol::IdleLow => s.spi.ctlr1.clr_cpol(),
            Cpol::IdleHigh => s.spi.ctlr1.set_cpol(),
        }
        match config.cpha {
            Cpha::FirstEdge => s.spi.ctlr1.clr_cpha(),
            Cpha::SecondEdge => s.spi.ctlr1.set_cpha(),
        }

        // Baud rate prescaler
        s.spi.ctlr1.write_br(config.baud_div.br_value());

        // Master / slave mode with software slave management.
        s.spi.ctlr1.set_ssm();
        match config.mode {
            Mode::Master => {
                s.spi.ctlr1.set_mstr();
                s.spi.ctlr1.set_ssi(); // deassert internal NSS
            }
            Mode::Slave => {
                s.spi.ctlr1.clr_mstr();
                s.spi.ctlr1.clr_ssi(); // selected via software
            }
        }

        // Transfer direction (full / half duplex, receive-only).
        match config.direction {
            Direction::FullDuplex => {
                s.spi.ctlr1.clr_bidimode();
                s.spi.ctlr1.clr_rxonly();
            }
            Direction::HalfDuplexTx => {
                s.spi.ctlr1.set_bidimode();
                s.spi.ctlr1.set_bidioe();
            }
            Direction::HalfDuplexRx => {
                s.spi.ctlr1.set_bidimode();
                s.spi.ctlr1.clr_bidioe();
            }
            Direction::ReceiveOnly => {
                s.spi.ctlr1.clr_bidimode();
                s.spi.ctlr1.set_rxonly();
            }
        }

        // Enable SPI
        s.spi.ctlr1.set_spe();

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
            spi: unsafe { pac::SPI1::steal() },
            clocks: unsafe { Clocks::steal() },
        })
    }

    /// Transfer a word (8- or 16-bit depending on the frame format).
    pub fn transfer(&mut self, data: u16) -> u16 {
        self.try_transfer(data)
            .unwrap_or_else(|_| panic!("SPI transfer timed out"))
    }

    /// Transfer a word, returning [`Error::Timeout`] if the transfer does not
    /// complete.
    pub fn try_transfer(&mut self, data: u16) -> Result<u16, Error> {
        if !crate::spin::try_spin_wait(|| self.spi.statr.txe()) {
            return Err(Error::Timeout);
        }
        self.spi.datar.write_dr(data as u32);
        if !crate::spin::try_spin_wait(|| self.spi.statr.rxne()) {
            return Err(Error::Timeout);
        }
        Ok(self.spi.datar.dr() as u16)
    }

    /// Write a byte and discard the received byte.
    pub fn write(&mut self, byte: u8) {
        let _ = self.transfer(byte as u16);
    }

    /// Write a slice of bytes.
    pub fn write_bytes(&mut self, data: &[u8]) {
        for &b in data {
            self.write(b);
        }
    }

    /// Read a byte (must clock out a dummy byte).
    pub fn read(&mut self) -> u8 {
        self.transfer(0) as u8
    }

    /// Read a slice of bytes (clocking out zeros).
    pub fn read_bytes(&mut self, buf: &mut [u8]) {
        for b in buf.iter_mut() {
            *b = self.read();
        }
    }

    /// Set the first-bit order (`true` = LSB first, `false` = MSB first).
    #[inline]
    pub fn set_first_bit_lsb(&mut self, lsb_first: bool) {
        if lsb_first {
            self.spi.ctlr1.set_lsbfirst();
        } else {
            self.spi.ctlr1.clr_lsbfirst();
        }
    }

    /// Enable CRC generation with the given polynomial.
    #[inline]
    pub fn enable_crc(&mut self, polynomial: u8) {
        self.spi.crcr.write_crcpoly(polynomial as u32);
        self.spi.ctlr1.set_crcen();
    }

    /// Disable CRC generation.
    #[inline(always)]
    pub fn disable_crc(&mut self) {
        self.spi.ctlr1.clr_crcen();
    }

    /// Request that the CRC be transmitted on the next transfer.
    ///
    /// Call this after clocking out the last data word so the hardware appends
    /// the CRC.
    #[inline(always)]
    pub fn send_crc(&mut self) {
        self.spi.ctlr1.set_crcnext();
    }

    /// Read the TX CRC register.
    #[inline(always)]
    pub fn read_tx_crc(&self) -> u16 {
        self.spi.tcrcr.txcrc() as u16
    }

    /// Read the RX CRC register.
    #[inline(always)]
    pub fn read_rx_crc(&self) -> u16 {
        self.spi.rcrcr.rxcrc() as u16
    }

    /// Transmit `data` using DMA.
    pub fn write_dma<const CH: u8>(
        &mut self,
        ch: &DmaChannel<CH>,
        data: &[u8],
    ) -> Result<(), Error> {
        self.spi.ctlr2.set_txdmaen();
        ch.start(
            DmaDir::MemoryToPeripheral,
            pac::spi1::BASE + 0x0C, // DATAR
            data.as_ptr() as u32,
            data.len() as u16,
            DataSize::Byte,
            DataSize::Byte,
            false,
            true,
        );
        ch.try_wait().map_err(|_| Error::Timeout)?;
        crate::spin::spin_wait("SPI TX complete", || !self.spi.statr.bsy());
        Ok(())
    }

    /// Receive into `data` using DMA.
    pub fn read_dma<const CH: u8>(
        &mut self,
        ch: &DmaChannel<CH>,
        data: &mut [u8],
    ) -> Result<(), Error> {
        self.spi.ctlr2.set_rxdmaen();
        ch.start(
            DmaDir::PeripheralToMemory,
            pac::spi1::BASE + 0x0C, // DATAR
            data.as_ptr() as u32,
            data.len() as u16,
            DataSize::Byte,
            DataSize::Byte,
            false,
            true,
        );
        ch.try_wait().map_err(|_| Error::Timeout)
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::SPI1 {
        self.spi
    }
}

impl SpiBus for Spi1 {
    type Error = Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        for &b in data {
            self.try_transfer(b as u16)?;
        }
        Ok(())
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), Error> {
        for slot in data.iter_mut() {
            *slot = self.try_transfer(0)? as u8;
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let len = read.len().min(write.len());
        for i in 0..len {
            read[i] = self.try_transfer(write[i] as u16)? as u8;
        }
        Ok(())
    }
}
