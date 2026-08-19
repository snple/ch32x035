//! DMA driver
//!
//! DMA1 with 8 channels. Provides a type-safe [`DmaChannel<CH>`] with a
//! raw-register interface matching the CH32X035's STM32F1-style DMA.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, dma::{Dma, DmaDir, DataSize}};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let dma = Dma::init(dp.DMA, &mut rcc);
//! let ch = dma.channel::<1>();
//! ch.start(DmaDir::MemoryToPeripheral, 0x40013804, buf.as_ptr() as u32, 16,
//!          DataSize::Byte, DataSize::Byte, true, true);
//! ```

use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

use crate::Steal;
use crate::pac;
use crate::rcc::Rcc;
use crate::reset::Resettable;

/// DMA1 base address
const DMA1_BASE: u32 = 0x4002_0000;

// DMA register offsets
const OFFSET_INTFR: u32 = 0x00;
const OFFSET_INTFCR: u32 = 0x04;
const CH_CFGR: u32 = 0x08; // first channel's CFGR
const CH_STRIDE: u32 = 0x14; // 20 bytes per channel

// CFGR field bit positions (STM32F1-style)
const CFGR_EN: u32 = 1 << 0;
#[allow(dead_code)]
const CFGR_TCIE: u32 = 1 << 1;
#[allow(dead_code)]
const CFGR_HTIE: u32 = 1 << 2;
#[allow(dead_code)]
const CFGR_TEIE: u32 = 1 << 3;
const CFGR_DIR: u32 = 1 << 4;
#[allow(dead_code)]
const CFGR_CIRC: u32 = 1 << 5;
const CFGR_PINC: u32 = 1 << 6;
const CFGR_MINC: u32 = 1 << 7;
const CFGR_PSIZE_SHIFT: u32 = 8;
const CFGR_MSIZE_SHIFT: u32 = 10;
#[allow(dead_code)]
const CFGR_PL_SHIFT: u32 = 12;
const CFGR_MEM2MEM: u32 = 1 << 14;

/// DMA transfer direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaDir {
    /// Memory to peripheral
    MemoryToPeripheral,
    /// Peripheral to memory
    PeripheralToMemory,
    /// Memory to memory
    MemoryToMemory,
}

impl DmaDir {
    #[inline]
    const fn bits(self) -> u32 {
        match self {
            DmaDir::MemoryToPeripheral => 0,
            DmaDir::PeripheralToMemory => CFGR_DIR,
            DmaDir::MemoryToMemory => CFGR_MEM2MEM | CFGR_DIR,
        }
    }
}

/// DMA data size
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataSize {
    /// 8-bit
    Byte,
    /// 16-bit
    HalfWord,
    /// 32-bit
    Word,
}

impl DataSize {
    #[inline]
    const fn bits(self) -> u32 {
        match self {
            DataSize::Byte => 0b00,
            DataSize::HalfWord => 0b01,
            DataSize::Word => 0b10,
        }
    }
}

/// DMA error types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timed out waiting for the transfer to complete.
    Timeout,
}

/// DMA1 driver
pub struct Dma {
    dma: pac::DMA,
}

impl Dma {
    /// Create a new DMA driver, resetting and clock-enabling DMA1.
    pub fn init(dma: pac::DMA, rcc: &mut Rcc) -> Self {
        dma.init(rcc);
        Self { dma }
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
            dma: unsafe { pac::DMA::steal() },
        })
    }

    /// Get a handle to one of the 8 DMA channels (1..=8).
    #[inline]
    pub fn channel<const CH: u8>(&self) -> DmaChannel<CH> {
        const { assert!(CH >= 1 && CH <= 8, "DMA channels are 1..=8") };
        DmaChannel {
            _marker: PhantomData,
        }
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::DMA {
        self.dma
    }
}

/// A single DMA channel.
pub struct DmaChannel<const CH: u8> {
    _marker: PhantomData<*const ()>,
}

impl<const CH: u8> DmaChannel<CH> {
    /// Base offset of this channel's register group
    #[inline(always)]
    const fn ch_base() -> u32 {
        CH_CFGR + (CH as u32 - 1) * CH_STRIDE
    }

    #[inline(always)]
    fn read(offset: u32) -> u32 {
        unsafe { read_volatile((DMA1_BASE + offset) as *const u32) }
    }

    #[inline(always)]
    fn write(offset: u32, value: u32) {
        unsafe { write_volatile((DMA1_BASE + offset) as *mut u32, value) }
    }

    #[inline(always)]
    fn modify<F: FnOnce(u32) -> u32>(offset: u32, f: F) {
        let addr = (DMA1_BASE + offset) as *mut u32;
        unsafe { write_volatile(addr, f(read_volatile(addr))) }
    }

    /// Set the peripheral address register
    #[inline(always)]
    pub fn set_peripheral_address(&self, addr: u32) {
        Self::write(Self::ch_base() + 0x08, addr);
    }

    /// Set the memory address register
    #[inline(always)]
    pub fn set_memory_address(&self, addr: u32) {
        Self::write(Self::ch_base() + 0x0C, addr);
    }

    /// Set the transfer count (number of transfers)
    #[inline(always)]
    pub fn set_transfer_count(&self, count: u16) {
        Self::write(Self::ch_base() + 0x04, count as u32);
    }

    /// Check whether the channel is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        Self::read(Self::ch_base()) & CFGR_EN != 0
    }

    /// Configure the channel (without starting it).
    ///
    /// `pinc`/`minc` enable peripheral/memory address increments.
    #[inline]
    pub fn configure(&self, dir: DmaDir, psize: DataSize, msize: DataSize, pinc: bool, minc: bool) {
        let mut cfg = dir.bits()
            | (psize.bits() << CFGR_PSIZE_SHIFT)
            | (msize.bits() << CFGR_MSIZE_SHIFT)
            | if pinc { CFGR_PINC } else { 0 }
            | if minc { CFGR_MINC } else { 0 };
        // Preserve the EN bit if set
        cfg |= Self::read(Self::ch_base()) & CFGR_EN;
        Self::write(Self::ch_base(), cfg);
    }

    /// Enable the channel.
    #[inline(always)]
    pub fn enable(&self) {
        Self::modify(Self::ch_base(), |v| v | CFGR_EN);
    }

    /// Disable the channel.
    #[inline(always)]
    pub fn disable(&self) {
        Self::modify(Self::ch_base(), |v| v & !CFGR_EN);
    }

    /// Start a one-shot transfer with the given parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        &self,
        dir: DmaDir,
        peripheral_addr: u32,
        memory_addr: u32,
        count: u16,
        psize: DataSize,
        msize: DataSize,
        pinc: bool,
        minc: bool,
    ) {
        self.disable();
        self.set_peripheral_address(peripheral_addr);
        self.set_memory_address(memory_addr);
        self.set_transfer_count(count);
        self.configure(dir, psize, msize, pinc, minc);
        self.enable();
    }

    /// Whether the transfer-complete interrupt flag is set.
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        let bit = 1 << (CH as u32 % 8 * 4 + 1); // TCIFn = bit 4*(n-1)+1
        Self::read(OFFSET_INTFR) & bit != 0
    }

    /// Clear all interrupt flags for this channel.
    #[inline(always)]
    pub fn clear_flags(&self) {
        let bits = 0xF << (CH as u32 % 8 * 4); // GIFn|TCIFn|HTIFn|TEIFn
        Self::write(OFFSET_INTFCR, bits);
    }

    /// Block until the transfer completes.
    pub fn wait(&self) {
        crate::spin::spin_wait("DMA transfer complete", || self.is_complete());
        self.clear_flags();
    }

    /// Wait for the transfer to complete, returning [`Error::Timeout`] if it
    /// does not complete in time.
    pub fn try_wait(&self) -> Result<(), Error> {
        if crate::spin::try_spin_wait(|| self.is_complete()) {
            self.clear_flags();
            Ok(())
        } else {
            Err(Error::Timeout)
        }
    }
}
