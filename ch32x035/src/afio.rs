//! AFIO (Alternate Function I/O)
//!
//! Configures pin remapping: EXTI line → port mapping (`EXTICR`) and the
//! USB / USB-PD PHY options in the AFIO control register.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, afio::{Afio, ExtiPort}};
//!
//! let mut afio = Afio::init(dp.AFIO, &mut rcc);
//! // Route EXTI line 0 to port B (PB0)
//! afio.set_exti_source::<0>(ExtiPort::PortB);
//! ```

use core::ptr::{read_volatile, write_volatile};

use crate::pac;
use crate::rcc::{Apb2Periph, Rcc};

/// AFIO base address
const AFIO_BASE: u32 = 0x4001_0000;
/// EXTI configuration register 1 (lines 0-15)
const EXTICR1: u32 = 0x08;
/// EXTI configuration register 2 (lines 16-23)
const EXTICR2: u32 = 0x0C;

/// The GPIO port an EXTI line is mapped to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtiPort {
    /// Port A
    PortA,
    /// Port B
    PortB,
    /// Port C
    PortC,
    /// Port D
    PortD,
}

impl ExtiPort {
    const fn bits(self) -> u32 {
        match self {
            ExtiPort::PortA => 0,
            ExtiPort::PortB => 1,
            ExtiPort::PortC => 2,
            ExtiPort::PortD => 3,
        }
    }
}

/// AFIO driver
pub struct Afio {
    afio: pac::AFIO,
}

impl Afio {
    /// Create a new AFIO driver, enabling the AFIO clock.
    pub fn init(afio: pac::AFIO, rcc: &mut Rcc) -> Self {
        rcc.apb2_enable(Apb2Periph::Afio);
        Self { afio }
    }

    /// Map EXTI `LINE` (0..=23) to the given GPIO port.
    ///
    /// Must be called before using the EXTI interrupt for that line.
    pub fn set_exti_source<const LINE: u8>(&mut self, port: ExtiPort) {
        const { assert!(LINE <= 23, "EXTI lines are 0..=23") };

        let (offset, shift) = if LINE < 16 {
            (EXTICR1, LINE * 2)
        } else {
            (EXTICR2, (LINE - 16) * 2)
        };
        let mask = 0b11 << shift;
        let addr = (AFIO_BASE + offset) as *mut u32;
        unsafe {
            write_volatile(addr, (read_volatile(addr) & !mask) | (port.bits() << shift));
        }
    }

    /// Configure the USB-PD PHY (high-threshold input + 3.3 V PHY limit).
    ///
    /// Recommended before using [`crate::usbpd`].
    pub fn configure_pd_phy(&mut self) {
        self.afio.ctlr.set_usbpd_in_hvt();
        self.afio.ctlr.set_usbpd_phy_v33();
    }

    /// Enable the USB FS PHY (for [`crate::pac::USBFS`]).
    pub fn configure_usb_phy(&mut self) {
        self.afio.ctlr.set_usb_ioen();
        self.afio.ctlr.set_usb_phy_v33();
    }

    /// Consume the driver and return the underlying peripheral.
    pub fn release(self) -> pac::AFIO {
        self.afio
    }
}
