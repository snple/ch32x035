//! AFIO (Alternate Function I/O)
//!
//! Configures:
//! - EXTI line → GPIO port mapping (`EXTICR1`/`EXTICR2`)
//! - peripheral pin remapping (`PCFR1`): SPI1, I2C1, USART1-4, TIM1-3, PIOC
//! - SWD pin release and the USB / USB-PD PHY options (`CTLR`)
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, afio::{Afio, ExtiPort, I2c1Remap}};
//!
//! let mut afio = Afio::init(dp.AFIO, &mut rcc);
//! // Route EXTI line 0 to port B (PB0)
//! afio.set_exti_source::<0>(ExtiPort::PortB);
//! // Move I2C1 SCL/SDA to PA13/PA14
//! afio.set_i2c1_remap(I2c1Remap::Remap1);
//! ```

use crate::Steal;
use crate::pac;
use crate::rcc::{Apb2Periph, Rcc};

/// The GPIO port an EXTI line is mapped to.
///
/// The chip has no port D; the `11` encoding is reserved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtiPort {
    /// Port A (`00`)
    PortA,
    /// Port B (`01`)
    PortB,
    /// Port C (`10`)
    PortC,
}

impl ExtiPort {
    #[inline]
    const fn bits(self) -> u32 {
        match self {
            ExtiPort::PortA => 0b00,
            ExtiPort::PortB => 0b01,
            ExtiPort::PortC => 0b10,
        }
    }
}

// =============================================================================
// Peripheral remapping options (AFIO_PCFR1)
// =============================================================================

/// SPI1 remap (`SPI1_RM[1:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spi1Remap {
    /// NSS/PA4, CK/PA5, MISO/PA6, MOSI/PA7
    Remap0 = 0b00,
    /// NSS/PB21, CK/PB15, MISO/PA8, MOSI/PA9
    Remap1 = 0b01,
    /// NSS/PA12, CK/PA11, MISO/PA9, MOSI/PA10
    Remap2 = 0b10,
    /// NSS/PC4, CK/PC5, MISO/PC6, MOSI/PC7
    Remap3 = 0b11,
}

/// I2C1 remap (`I2C1_RM[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2c1Remap {
    /// SCL/PA10, SDA/PA11
    Remap0 = 0b000,
    /// SCL/PA13, SDA/PA14
    Remap1 = 0b001,
    /// SCL/PC16, SDA/PC17
    Remap2 = 0b010,
    /// SCL/PC19, SDA/PC18
    Remap3 = 0b011,
    /// SCL/PC17, SDA/PC16
    Remap4 = 0b100,
    /// SCL/PC18, SDA/PC19
    Remap5 = 0b101,
}

/// USART1 remap (`USART1_RM[1:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart1Remap {
    /// RX/PB11, TX/PB10, CK/PB9, CTS/PC16, RTS/PC17
    Remap0 = 0b00,
    /// RX/PA11, TX/PA10, CK/PB9, CTS/PC16, RTS/PC17
    Remap1 = 0b01,
    /// RX/PB11, TX/PB10, CK/PB5, CTS/PA9, RTS/PA8
    Remap2 = 0b10,
    /// RX/PB2, TX/PA7, CK/PB12, CTS/PA13, RTS/PA14
    Remap3 = 0b11,
}

/// USART2 remap (`USART2_RM[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart2Remap {
    /// RX/PA3, TX/PA2, CK/PA4, CTS/PA0, RTS/PA1
    Remap0 = 0b000,
    /// RX/PA19, TX/PA20, CK/PA23, CTS/PA1, RTS/PA2
    Remap1 = 0b001,
    /// RX/PA16, TX/PA15, CK/PA22, CTS/PA17, RTS/PA21
    Remap2 = 0b010,
    /// RX/PC1, TX/PC0, CK/PB20, CTS/PC2, RTS/PC3
    Remap3 = 0b011,
    /// RX/PA16, TX/PA15, CK/PA22, CTS/PA17, RTS/PC3
    Remap4 = 0b100,
}

/// USART3 remap (`USART3_RM[1:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart3Remap {
    /// RX/PB4, TX/PB3, CK/PB5, CTS/PB6, RTS/PB7
    Remap0 = 0b00,
    /// RX/PC19, TX/PC18, CK/PB5, CTS/PB6, RTS/PB7
    Remap1 = 0b01,
    /// RX/PB14, TX/PA18, CK/PB8, CTS/PA3, RTS/PA4
    Remap2 = 0b10,
    /// RX/PB17, TX/PB16, CTS/PB18, RTS/PB19 (no CK)
    Remap3 = 0b11,
}

/// USART4 remap (`USART4_RM[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usart4Remap {
    /// RX/PB1, TX/PB0, CK/PB2, CTS/PB15, RTS/PA8
    Remap0 = 0b000,
    /// RX/PA9, TX/PA5, CK/PA6, CTS/PA7, RTS/PB21
    Remap1 = 0b001,
    /// RX/PC17, TX/PC16, CK/PB2, CTS/PB15, RTS/PA8
    Remap2 = 0b010,
    /// RX/PA10, TX/PB9, CK/PB8, CTS/PA14, RTS/PA13
    Remap3 = 0b011,
    /// RX/PC19, TX/PB13, CK/PA8, CTS/PA5, RTS/PA6
    Remap4 = 0b100,
    /// RX/PC16, TX/PC17, CK/PB2, CTS/PB15, RTS/PA8
    Remap5 = 0b101,
}

/// TIM1 remap (`TIM1_RM[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tim1Remap {
    /// CH1/PB9, CH2/PB10, CH3/PB11, CH4/PC16, ETR/PC17, BKIN/PB5,
    /// C1N/PB6, C2N/PB7, C3N/PB8
    Remap0 = 0b000,
    /// CH1/PB9, CH2/PB10, CH3/PB11, CH4/PC16, ETR/PC17, BKIN/PA6,
    /// C1N/PA7, C2N/PB0, C3N/PB1
    Remap1 = 0b001,
    /// CH1/PB9, CH2/PB10, CH3/PB11, CH4/PB12, ETR/PC18, BKIN/PB5,
    /// C1N/PB6, C2N/PB7, C3N/PB8
    Remap2 = 0b010,
    /// CH1/PC0, CH2/PC1, CH3/PC2, CH4/PC3, ETR/PC18, BKIN/PC4,
    /// C1N/PC5, C2N/PC6, C3N/PC7
    Remap3 = 0b011,
    /// CH3/PC14, ETR/PC15 (CH1/CH2/CH4/BKIN/C1N/C2N/C3N unavailable)
    Remap4 = 0b100,
}

/// TIM2 remap (`TIM2_RM[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tim2Remap {
    /// CH1/PA0, CH2/PA1, CH3/PA2, CH4/PA3, ETR/PA19, BKIN/PA20,
    /// C1N/PA21, C2N/PA22, C3N/PA23
    Remap0 = 0b000,
    /// CH1/PB21, CH2/PB15, CH3/PA2, CH4/PA3, ETR/PA18, BKIN/PA9,
    /// C1N/PA12, C2N/PA13, C3N/PA14
    Remap1 = 0b001,
    /// CH1/PA0, CH2/PA1, CH3/PB3, CH4/PB4, ETR/PA19, BKIN/PA20,
    /// C1N/PC3, C2N/PA22, C3N/PA23
    Remap2 = 0b010,
    /// CH1/PB21, CH2/PB15, CH3/PB3, CH4/PB4, ETR/PA18, BKIN/PA9,
    /// C1N/PA12, C2N/PA13, C3N/PA14
    Remap3 = 0b011,
    /// CH1/PB16, CH2/PB17, CH3/PB18, CH4/PB19, ETR/PC4, BKIN/PC0,
    /// C1N/PC1, C2N/PC2, C3N/PC3
    Remap4 = 0b100,
    /// CH1/PC19, CH2/PA12, CH3/PA13, CH4/PC0, ETR/PA2, BKIN/PB4,
    /// C1N/PC18, C2N/PB12, C3N/PB3
    Remap5 = 0b101,
    /// CH1/PC19, CH2/PC14, CH3/PC15, CH4/PC0, ETR/PA2, BKIN/PB4,
    /// C1N/PB11, C2N/PB12, C3N/PB3
    Remap6 = 0b110,
}

/// TIM3 remap (`TIM3_RM[1:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tim3Remap {
    /// CH1/PA6, CH2/PA7
    Remap0 = 0b00,
    /// CH1/PB4, CH2/PB5
    Remap1 = 0b01,
    /// CH1/PC19, CH2/PC18
    Remap2 = 0b10,
    /// CH1/PA3, CH2/PA4
    Remap3 = 0b11,
}

/// USB FS PHY supply configuration.
///
/// Selects the D+ pull-up value used to signal full-speed attachment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbPhySupply {
    /// Device powered from 3.3 V: internal 1.5 kΩ pull-up, PHY limiter on.
    V33,
    /// Device powered from 5 V: 10 kΩ pull-up, PHY limiter off.
    V5,
}

/// AFIO driver
pub struct Afio {
    afio: pac::AFIO,
}

impl Afio {
    /// `CTLR.UDP_PUE` (D+ pull-up) mask
    const UDP_PUE_MASK: u32 = 0x0000_000c;
    /// `CTLR.UDM_PUE` (D− pull-up) mask
    const UDM_PUE_MASK: u32 = 0x0000_0003;
    /// D+ 1.5 kΩ pull-up (full-speed device, 3.3 V supply)
    const UDP_PUE_1K5: u32 = 0x0000_000c;
    /// D+ 10 kΩ pull-up (5 V supply)
    const UDP_PUE_10K: u32 = 0x0000_0008;

    /// Create a new AFIO driver, enabling the AFIO clock.
    pub fn init(afio: pac::AFIO, rcc: &mut Rcc) -> Self {
        rcc.apb2_enable(Apb2Periph::Afio);
        Self { afio }
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
            afio: unsafe { pac::AFIO::steal() },
        })
    }

    /// Map EXTI `LINE` (0..=23) to the given GPIO port.
    ///
    /// Must be called before using the EXTI interrupt for that line.
    pub fn set_exti_source<const LINE: u8>(&mut self, port: ExtiPort) {
        const { assert!(LINE <= 23, "EXTI lines are 0..=23") };

        let port = port.bits();
        // `% 16` folds lines 16-23 into the low half of `EXTICR2`.
        let shift = (LINE as u32 % 16) * 2;
        if LINE < 16 {
            let reg = &self.afio.exticr1;
            let bits = (reg.read_val().bits() & !(0b11 << shift)) | (port << shift);
            reg.write_val(pac::afio::Exticr1Val::from_bits(bits));
        } else {
            let reg = &self.afio.exticr2;
            let bits = (reg.read_val().bits() & !(0b11 << shift)) | (port << shift);
            reg.write_val(pac::afio::Exticr2Val::from_bits(bits));
        }
    }

    /// Set the SPI1 pin remapping.
    #[inline(always)]
    pub fn set_spi1_remap(&mut self, remap: Spi1Remap) {
        self.afio.pcfr1.write_spi1rm(remap as u32);
    }

    /// Set the raw `SPI1_RM` code (used by the [`crate::pins`] remap
    /// helpers).
    #[inline(always)]
    pub fn set_spi1_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_spi1rm(code as u32);
    }

    /// Set the I2C1 pin remapping.
    #[inline(always)]
    pub fn set_i2c1_remap(&mut self, remap: I2c1Remap) {
        self.afio.pcfr1.write_i2c1rm(remap as u32);
    }

    /// Set the raw `I2C1_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_i2c1_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_i2c1rm(code as u32);
    }

    /// Set the USART1 pin remapping.
    #[inline(always)]
    pub fn set_usart1_remap(&mut self, remap: Usart1Remap) {
        self.afio.pcfr1.write_usart1rm(remap as u32);
    }

    /// Set the raw `USART1_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_usart1_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_usart1rm(code as u32);
    }

    /// Set the USART2 pin remapping.
    #[inline(always)]
    pub fn set_usart2_remap(&mut self, remap: Usart2Remap) {
        self.afio.pcfr1.write_usart2rm(remap as u32);
    }

    /// Set the raw `USART2_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_usart2_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_usart2rm(code as u32);
    }

    /// Set the USART3 pin remapping.
    #[inline(always)]
    pub fn set_usart3_remap(&mut self, remap: Usart3Remap) {
        self.afio.pcfr1.write_usart3rm(remap as u32);
    }

    /// Set the raw `USART3_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_usart3_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_usart3rm(code as u32);
    }

    /// Set the USART4 pin remapping.
    #[inline(always)]
    pub fn set_usart4_remap(&mut self, remap: Usart4Remap) {
        self.afio.pcfr1.write_usart4rm(remap as u32);
    }

    /// Set the raw `USART4_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_usart4_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_usart4rm(code as u32);
    }

    /// Set the TIM1 pin remapping.
    #[inline(always)]
    pub fn set_tim1_remap(&mut self, remap: Tim1Remap) {
        self.afio.pcfr1.write_tim1rm(remap as u32);
    }

    /// Set the raw `TIM1_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_tim1_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_tim1rm(code as u32);
    }

    /// Set the TIM2 pin remapping.
    #[inline(always)]
    pub fn set_tim2_remap(&mut self, remap: Tim2Remap) {
        self.afio.pcfr1.write_tim2rm(remap as u32);
    }

    /// Set the raw `TIM2_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_tim2_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_tim2rm(code as u32);
    }

    /// Set the TIM3 pin remapping.
    #[inline(always)]
    pub fn set_tim3_remap(&mut self, remap: Tim3Remap) {
        self.afio.pcfr1.write_tim3rm(remap as u32);
    }

    /// Set the raw `TIM3_RM` code (used by the
    /// [`crate::pins`] remap helpers).
    #[inline(always)]
    pub fn set_tim3_remap_code(&mut self, code: u8) {
        self.afio.pcfr1.write_tim3rm(code as u32);
    }

    /// Remap the PIOC interface: `true` maps interface 0 to PC7 (default
    /// PC18), interface 1 stays on PC19.
    pub fn set_pioc_remap(&mut self, remapped: bool) {
        if remapped {
            self.afio.pcfr1.set_ploc_rm();
        } else {
            self.afio.pcfr1.clr_ploc_rm();
        }
    }

    /// Disable the SWD (RVSWD) pins so they can be used as ordinary GPIO.
    ///
    /// The SWD pins are enabled as the debug port after reset; passing
    /// `true` releases them, `false` keeps the debug port. This cannot be
    /// undone without a reset.
    #[inline(always)]
    pub fn set_swd_disabled(&mut self, disabled: bool) {
        // 0xx = SWD enabled, 100 = SWD disabled (GPIO), others invalid.
        self.afio
            .pcfr1
            .write_sw_cfg(if disabled { 0b100 } else { 0b000 });
    }

    /// Configure the USB-PD PHY (high-threshold input + 3.3 V PHY limit).
    ///
    /// Recommended before using [`crate::usbpd`].
    #[inline(always)]
    pub fn configure_pd_phy(&mut self) {
        self.afio.ctlr.set_usbpd_in_hvt();
        self.afio.ctlr.set_usbpd_phy_v33();
    }

    /// Enable the USB FS PHY and its D+ pull-up (for [`crate::pac::USBFS`]).
    ///
    /// The pull-up value depends on the board supply: a 3.3 V powered device
    /// uses the internal 1.5 kΩ pull-up (USB full-speed requirement), while a
    /// 5 V powered device selects the 10 kΩ pull-up and leaves the PHY
    /// limiter disabled, mirroring the WCH SDK.
    #[inline(always)]
    pub fn configure_usb_phy(&mut self, supply: UsbPhySupply) {
        // Clear both pull-up fields first (they are read-modify-write).
        self.afio
            .ctlr
            .modify_bits(|v| v & !(Self::UDP_PUE_MASK | Self::UDM_PUE_MASK));
        self.afio.ctlr.set_usb_ioen();
        match supply {
            UsbPhySupply::V33 => {
                self.afio.ctlr.set_usb_phy_v33();
                self.afio.ctlr.modify_bits(|v| v | Self::UDP_PUE_1K5);
            }
            UsbPhySupply::V5 => {
                self.afio.ctlr.clr_usb_phy_v33();
                self.afio.ctlr.modify_bits(|v| v | Self::UDP_PUE_10K);
            }
        }
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::AFIO {
        self.afio
    }
}
