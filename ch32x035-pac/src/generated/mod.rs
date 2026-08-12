//! CH32X035 Hardware Register Definitions
//!
//! Auto-generated from ch32x035.svd - DO NOT EDIT MANUALLY

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::identity_op)]

use core::marker::PhantomData;
use core::ops::Deref;

pub mod common;

// ============================================================================
// Interrupt Handlers
// ============================================================================

unsafe extern "C" {
    fn WWDG_IRQ();
    fn PVD_IRQ();
    fn FLASH_IRQ();
    fn EXTI7_0_IRQ();
    fn AWU_IRQ();
    fn DMA1_CHANNEL1_IRQ();
    fn DMA1_CHANNEL2_IRQ();
    fn DMA1_CHANNEL3_IRQ();
    fn DMA1_CHANNEL4_IRQ();
    fn DMA1_CHANNEL5_IRQ();
    fn DMA1_CHANNEL6_IRQ();
    fn DMA1_CHANNEL7_IRQ();
    fn ADC1_IRQ();
    fn I2C1_EV_IRQ();
    fn I2C1_ER_IRQ();
    fn USART1_IRQ();
    fn SPI1_IRQ();
    fn TIM1_BRK_IRQ();
    fn TIM1_UP_IRQ();
    fn TIM1_TRG_COM_IRQ();
    fn TIM1_CC_IRQ();
    fn TIM2_UP_IRQ();
    fn USART2_IRQ();
    fn EXTI15_8_IRQ();
    fn EXTI25_16_IRQ();
    fn USART3_IRQ();
    fn USART4_IRQ();
    fn DMA1_CHANNEL8_IRQ();
    fn USBFS_IRQ();
    fn USBFS_WKUP_IRQ();
    fn PIOC_IRQ();
    fn OPA_IRQ();
    fn USBPD_IRQ();
    fn USBPD_WKUP_IRQ();
    fn TIM2_CC_IRQ();
    fn TIM2_TRG_COM_IRQ();
    fn TIM2_BRK_IRQ();
    fn TIM3_IRQ();
}

/// Union type for interrupt vectors
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

/// CH32X035 interrupt vector table (external interrupts 16..=54, 39 entries)
///
/// Placed after the core exception vectors (slots 0..=15) in flash.
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 39] = [
    Vector { _handler: WWDG_IRQ }, // 16
    Vector { _handler: PVD_IRQ },  // 17
    Vector {
        _handler: FLASH_IRQ,
    }, // 18
    Vector { _reserved: 0 },       // 19 reserved
    Vector {
        _handler: EXTI7_0_IRQ,
    }, // 20
    Vector { _handler: AWU_IRQ },  // 21
    Vector {
        _handler: DMA1_CHANNEL1_IRQ,
    }, // 22
    Vector {
        _handler: DMA1_CHANNEL2_IRQ,
    }, // 23
    Vector {
        _handler: DMA1_CHANNEL3_IRQ,
    }, // 24
    Vector {
        _handler: DMA1_CHANNEL4_IRQ,
    }, // 25
    Vector {
        _handler: DMA1_CHANNEL5_IRQ,
    }, // 26
    Vector {
        _handler: DMA1_CHANNEL6_IRQ,
    }, // 27
    Vector {
        _handler: DMA1_CHANNEL7_IRQ,
    }, // 28
    Vector { _handler: ADC1_IRQ }, // 29
    Vector {
        _handler: I2C1_EV_IRQ,
    }, // 30
    Vector {
        _handler: I2C1_ER_IRQ,
    }, // 31
    Vector {
        _handler: USART1_IRQ,
    }, // 32
    Vector { _handler: SPI1_IRQ }, // 33
    Vector {
        _handler: TIM1_BRK_IRQ,
    }, // 34
    Vector {
        _handler: TIM1_UP_IRQ,
    }, // 35
    Vector {
        _handler: TIM1_TRG_COM_IRQ,
    }, // 36
    Vector {
        _handler: TIM1_CC_IRQ,
    }, // 37
    Vector {
        _handler: TIM2_UP_IRQ,
    }, // 38
    Vector {
        _handler: USART2_IRQ,
    }, // 39
    Vector {
        _handler: EXTI15_8_IRQ,
    }, // 40
    Vector {
        _handler: EXTI25_16_IRQ,
    }, // 41
    Vector {
        _handler: USART3_IRQ,
    }, // 42
    Vector {
        _handler: USART4_IRQ,
    }, // 43
    Vector {
        _handler: DMA1_CHANNEL8_IRQ,
    }, // 44
    Vector {
        _handler: USBFS_IRQ,
    }, // 45
    Vector {
        _handler: USBFS_WKUP_IRQ,
    }, // 46
    Vector { _handler: PIOC_IRQ }, // 47
    Vector { _handler: OPA_IRQ },  // 48
    Vector {
        _handler: USBPD_IRQ,
    }, // 49
    Vector {
        _handler: USBPD_WKUP_IRQ,
    }, // 50
    Vector {
        _handler: TIM2_CC_IRQ,
    }, // 51
    Vector {
        _handler: TIM2_TRG_COM_IRQ,
    }, // 52
    Vector {
        _handler: TIM2_BRK_IRQ,
    }, // 53
    Vector { _handler: TIM3_IRQ }, // 54
];

/// CH32X035 Interrupt enumeration
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 16 - Window Watchdog interrupt
    WWDG_IRQ = 16,
    /// 17 - PVD through EXTI line detection
    PVD_IRQ = 17,
    /// 18 - Flash global interrupt
    FLASH_IRQ = 18,
    /// 20 - EXTI Line[7:0] interrupt
    EXTI7_0_IRQ = 20,
    /// 21 - AWU global interrupt
    AWU_IRQ = 21,
    /// 22 - DMA Channel1 global interrupt
    DMA1_CHANNEL1_IRQ = 22,
    /// 23 - DMA Channel2 global interrupt
    DMA1_CHANNEL2_IRQ = 23,
    /// 24 - DMA Channel3 global interrupt
    DMA1_CHANNEL3_IRQ = 24,
    /// 25 - DMA Channel4 global interrupt
    DMA1_CHANNEL4_IRQ = 25,
    /// 26 - DMA Channel5 global interrupt
    DMA1_CHANNEL5_IRQ = 26,
    /// 27 - DMA Channel6 global interrupt
    DMA1_CHANNEL6_IRQ = 27,
    /// 28 - DMA Channel7 global interrupt
    DMA1_CHANNEL7_IRQ = 28,
    /// 29 - ADC global interrupt
    ADC1_IRQ = 29,
    /// 30 - I2C1 event interrupt
    I2C1_EV_IRQ = 30,
    /// 31 - I2C1 error interrupt
    I2C1_ER_IRQ = 31,
    /// 32 - USART1 global interrupt
    USART1_IRQ = 32,
    /// 33 - SPI1 global interrupt
    SPI1_IRQ = 33,
    /// 34 - TIM1 Break
    TIM1_BRK_IRQ = 34,
    /// 35 - TIM1 Update
    TIM1_UP_IRQ = 35,
    /// 36 - TIM1 Trigger and Commutation interrupts
    TIM1_TRG_COM_IRQ = 36,
    /// 37 - TIM1 Capture Compare interrupt
    TIM1_CC_IRQ = 37,
    /// 38 - TIM2 Update
    TIM2_UP_IRQ = 38,
    /// 39 - USART2 global interrupt
    USART2_IRQ = 39,
    /// 40 - EXTI Line[15:8] interrupts
    EXTI15_8_IRQ = 40,
    /// 41 - EXTI Line[25:16] interrupts
    EXTI25_16_IRQ = 41,
    /// 42 - USART3 global interrupt
    USART3_IRQ = 42,
    /// 43 - UART4 global interrupt
    USART4_IRQ = 43,
    /// 44 - DMA Channel8 global interrupt
    DMA1_CHANNEL8_IRQ = 44,
    /// 45 - USBFS
    USBFS_IRQ = 45,
    /// 46 - USBFS_WKUP
    USBFS_WKUP_IRQ = 46,
    /// 47 - PIOC global interrupt
    PIOC_IRQ = 47,
    /// 48 - OPA global interrupt
    OPA_IRQ = 48,
    /// 49 - USBPD global interrupt
    USBPD_IRQ = 49,
    /// 50 - USBPD_WKUP global interrupt
    USBPD_WKUP_IRQ = 50,
    /// 51 - TIM2 Capture Compare interrupt
    TIM2_CC_IRQ = 51,
    /// 52 - TIM2 Trigger and Commutation interrupts
    TIM2_TRG_COM_IRQ = 52,
    /// 53 - TIM2 Break
    TIM2_BRK_IRQ = 53,
    /// 54 - TIM3 global interrupt
    TIM3_IRQ = 54,
}

unsafe impl ch32x035_rt::peripheral::pfic::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}

// ============================================================================
// Peripheral Wrapper Macro
// ============================================================================

/// Helper macro to create peripheral wrappers
macro_rules! peripheral {
    ($name:ident, $reg:ty, $base:expr) => {
        impl $name {
            /// Pointer to the peripheral's register block
            pub const PTR: *const $reg = $base as *const $reg;

            /// Steal an instance of this peripheral
            ///
            /// # Safety
            ///
            /// Ensure that the new instance of the peripheral cannot be used in a way
            /// that may race with any existing instances.
            #[inline]
            pub const unsafe fn steal() -> Self {
                Self {
                    _marker: PhantomData,
                }
            }
        }

        impl Deref for $name {
            type Target = $reg;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::PTR }
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
}

// ============================================================================
// Peripheral Definitions
// ============================================================================

/// Power control
pub struct PWR {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PWR {}

peripheral!(PWR, pwr::RegisterBlock, 0x40007000);

/// Power control
pub mod pwr;

/// Reset and clock control
pub struct RCC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for RCC {}

peripheral!(RCC, rcc::RegisterBlock, 0x40021000);

/// Reset and clock control
pub mod rcc;

/// General purpose I/O
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GPIOA {}

peripheral!(GPIOA, gpioa::RegisterBlock, 0x40010800);

/// General purpose I/O
pub mod gpioa;

/// GPIOB peripheral
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GPIOB {}

peripheral!(GPIOB, gpiob::RegisterBlock, 0x40010c00);

/// GPIOB register module
pub mod gpiob;

/// GPIOC peripheral
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GPIOC {}

peripheral!(GPIOC, gpioc::RegisterBlock, 0x40011000);

/// GPIOC register module
pub mod gpioc;

/// Alternate function I/O
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for AFIO {}

peripheral!(AFIO, afio::RegisterBlock, 0x40010000);

/// Alternate function I/O
pub mod afio;

/// EXTI
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for EXTI {}

peripheral!(EXTI, exti::RegisterBlock, 0x40010400);

/// EXTI
pub mod exti;

/// DMA controller
pub struct DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DMA {}

peripheral!(DMA, dma::RegisterBlock, 0x40020000);

/// DMA controller
pub mod dma;

/// Independent watchdog
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for IWDG {}

peripheral!(IWDG, iwdg::RegisterBlock, 0x40003000);

/// Independent watchdog
pub mod iwdg;

/// Window watchdog
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for WWDG {}

peripheral!(WWDG, wwdg::RegisterBlock, 0x40002c00);

/// Window watchdog
pub mod wwdg;

/// Advanced timer
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIM1 {}

peripheral!(TIM1, tim1::RegisterBlock, 0x40012c00);

/// Advanced timer
pub mod tim1;

/// TIM2 peripheral
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIM2 {}

peripheral!(TIM2, tim2::RegisterBlock, 0x40000000);

/// TIM2 register module
pub mod tim2;

/// General purpose timer
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIM3 {}

peripheral!(TIM3, tim3::RegisterBlock, 0x40000400);

/// General purpose timer
pub mod tim3;

/// Inter integrated circuit
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C1 {}

peripheral!(I2C1, i2c1::RegisterBlock, 0x40005400);

/// Inter integrated circuit
pub mod i2c1;

/// Serial peripheral interface
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI1 {}

peripheral!(SPI1, spi1::RegisterBlock, 0x40013000);

/// Serial peripheral interface
pub mod spi1;

/// Universal synchronous asynchronous receiver
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USART1 {}

peripheral!(USART1, usart1::RegisterBlock, 0x40013800);

/// Universal synchronous asynchronous receiver
pub mod usart1;

/// USART2 peripheral
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USART2 {}

peripheral!(USART2, usart2::RegisterBlock, 0x40004400);

/// USART2 register module
pub mod usart2;

/// USART3 peripheral
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USART3 {}

peripheral!(USART3, usart3::RegisterBlock, 0x40004800);

/// USART3 register module
pub mod usart3;

/// UART4 peripheral
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART4 {}

peripheral!(UART4, uart4::RegisterBlock, 0x40004c00);

/// UART4 register module
pub mod uart4;

/// Analog to digital converter
pub struct ADC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ADC {}

peripheral!(ADC, adc::RegisterBlock, 0x40012400);

/// Analog to digital converter
pub mod adc;

/// Debug support
pub struct DBG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DBG {}

peripheral!(DBG, dbg::RegisterBlock, 0xe000d000);

/// Debug support
pub mod dbg;

/// USB register
pub struct USBFS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USBFS {}

peripheral!(USBFS, usbfs::RegisterBlock, 0x40023400);

/// USB register
pub mod usbfs;

/// FLASH
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for FLASH {}

peripheral!(FLASH, flash::RegisterBlock, 0x40022000);

/// FLASH
pub mod flash;

/// AWU configuration
pub struct AWU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for AWU {}

peripheral!(AWU, awu::RegisterBlock, 0x40026400);

/// AWU configuration
pub mod awu;

/// OPA configuration
pub struct OPA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for OPA {}

peripheral!(OPA, opa::RegisterBlock, 0x40026000);

/// OPA configuration
pub mod opa;

/// ESIG configuration
pub struct ESIG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ESIG {}

peripheral!(ESIG, esig::RegisterBlock, 0x1ffff7e0);

/// ESIG configuration
pub mod esig;

/// USBPD configuration
pub struct USBPD {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USBPD {}

peripheral!(USBPD, usbpd::RegisterBlock, 0x40027000);

/// USBPD configuration
pub mod usbpd;

// ============================================================================
// Peripherals - All device peripherals
// ============================================================================

use core::sync::atomic::{AtomicBool, Ordering};

/// Guard to ensure peripherals are taken only once
static TAKEN: AtomicBool = AtomicBool::new(false);

/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    /// Power control
    pub PWR: PWR,
    /// Reset and clock control
    pub RCC: RCC,
    /// General purpose I/O
    pub GPIOA: GPIOA,
    /// GPIOB peripheral
    pub GPIOB: GPIOB,
    /// GPIOC peripheral
    pub GPIOC: GPIOC,
    /// Alternate function I/O
    pub AFIO: AFIO,
    /// EXTI
    pub EXTI: EXTI,
    /// DMA controller
    pub DMA: DMA,
    /// Independent watchdog
    pub IWDG: IWDG,
    /// Window watchdog
    pub WWDG: WWDG,
    /// Advanced timer
    pub TIM1: TIM1,
    /// TIM2 peripheral
    pub TIM2: TIM2,
    /// General purpose timer
    pub TIM3: TIM3,
    /// Inter integrated circuit
    pub I2C1: I2C1,
    /// Serial peripheral interface
    pub SPI1: SPI1,
    /// Universal synchronous asynchronous receiver
    pub USART1: USART1,
    /// USART2 peripheral
    pub USART2: USART2,
    /// USART3 peripheral
    pub USART3: USART3,
    /// UART4 peripheral
    pub UART4: UART4,
    /// Analog to digital converter
    pub ADC: ADC,
    /// Debug support
    pub DBG: DBG,
    /// USB register
    pub USBFS: USBFS,
    /// FLASH
    pub FLASH: FLASH,
    /// AWU configuration
    pub AWU: AWU,
    /// OPA configuration
    pub OPA: OPA,
    /// ESIG configuration
    pub ESIG: ESIG,
    /// USBPD configuration
    pub USBPD: USBPD,
}

impl Peripherals {
    /// Returns all the peripherals *once*.
    #[inline]
    pub fn take() -> Option<Self> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        unsafe {
            Peripherals {
                PWR: PWR::steal(),
                RCC: RCC::steal(),
                GPIOA: GPIOA::steal(),
                GPIOB: GPIOB::steal(),
                GPIOC: GPIOC::steal(),
                AFIO: AFIO::steal(),
                EXTI: EXTI::steal(),
                DMA: DMA::steal(),
                IWDG: IWDG::steal(),
                WWDG: WWDG::steal(),
                TIM1: TIM1::steal(),
                TIM2: TIM2::steal(),
                TIM3: TIM3::steal(),
                I2C1: I2C1::steal(),
                SPI1: SPI1::steal(),
                USART1: USART1::steal(),
                USART2: USART2::steal(),
                USART3: USART3::steal(),
                UART4: UART4::steal(),
                ADC: ADC::steal(),
                DBG: DBG::steal(),
                USBFS: USBFS::steal(),
                FLASH: FLASH::steal(),
                AWU: AWU::steal(),
                OPA: OPA::steal(),
                ESIG: ESIG::steal(),
                USBPD: USBPD::steal(),
            }
        }
    }

    /// Release the peripherals back to the pool
    ///
    /// After calling this, `take()` will succeed again.
    #[inline]
    pub fn release(self) {
        TAKEN.store(false, Ordering::Release);
    }
}
