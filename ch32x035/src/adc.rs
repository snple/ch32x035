//! ADC driver
//!
//! ADC1 (APB2) driver for the CH32X035. Supports single and scan conversion,
//! continuous mode, injected channels, DMA, the analog watchdog and the touch
//! key interface.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, Clocks, adc::Adc};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let clocks = Clocks::new(&rcc);
//! let mut adc = Adc::init(dp.ADC, &mut rcc, clocks);
//! let value = adc.read_channel(0); // PA0
//! ```

use crate::Steal;
use crate::pac;
use crate::rcc::{Clocks, Rcc};
use crate::reset::Resettable;

/// ADC error types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Timed out waiting for the conversion to complete.
    Timeout,
}

/// ADC sample time (`SMPx` field value).
///
/// The CH32X035 ADC sample time is `4 + value` ADC clock cycles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SampleTime {
    /// 4 ADC clock cycles
    Cycles4 = 0,
    /// 5 ADC clock cycles
    Cycles5 = 1,
    /// 6 ADC clock cycles
    Cycles6 = 2,
    /// 7 ADC clock cycles
    Cycles7 = 3,
    /// 8 ADC clock cycles
    Cycles8 = 4,
    /// 9 ADC clock cycles
    Cycles9 = 5,
    /// 10 ADC clock cycles
    Cycles10 = 6,
    /// 11 ADC clock cycles
    Cycles11 = 7,
}

/// External trigger source for the regular group (`EXTSEL`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExternalTrigger {
    /// TIM1 TRGO
    Tim1Trgo = 0b000,
    /// TIM1 CC1
    Tim1Cc1 = 0b001,
    /// TIM1 CC2
    Tim1Cc2 = 0b010,
    /// TIM2 TRGO
    Tim2Trgo = 0b011,
    /// TIM2 CC1
    Tim2Cc1 = 0b100,
    /// TIM3 CC1
    Tim3Cc1 = 0b101,
    /// EXTI line 11
    Exti11 = 0b110,
}

/// External trigger source for the injected group (`JEXTSEL`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InjectedTrigger {
    /// TIM1 CC3
    Tim1Cc3 = 0b000,
    /// TIM1 CC4
    Tim1Cc4 = 0b001,
    /// TIM2 CC3
    Tim2Cc3 = 0b010,
    /// TIM2 CC4
    Tim2Cc4 = 0b011,
    /// TIM2 CC2
    Tim2Cc2 = 0b100,
    /// TIM3 CC2
    Tim3Cc2 = 0b101,
    /// EXTI line 15
    Exti15 = 0b110,
}

/// ADC1 driver (12-bit)
pub struct Adc {
    adc: pac::ADC,
    #[allow(dead_code)]
    clocks: Clocks,
}

impl Adc {
    /// Create a new ADC driver, resetting and clock-enabling the peripheral.
    ///
    /// The ADC is powered on (`ADON`) but not yet started.
    pub fn init(adc: pac::ADC, rcc: &mut Rcc, clocks: Clocks) -> Self {
        adc.init(rcc);

        let a = Self { adc, clocks };

        // Power on the ADC (ADON = 1)
        a.adc.ctlr2.set_adon();

        a
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
            adc: unsafe { pac::ADC::steal() },
            clocks: unsafe { Clocks::steal() },
        })
    }

    /// Perform a single conversion on an ADC pin.
    ///
    /// The channel is taken from the pin type, so only pins that are really
    /// wired to the ADC can be passed (see [`crate::pins::AdcPin`]). The pad
    /// is configured as an analog input.
    pub fn read_pin<P: crate::pins::AdcPin>(&mut self, pin: &mut P) -> u16 {
        pin.configure_adc_pin();
        self.read_channel(P::CHANNEL)
    }

    /// Perform a single conversion on the given channel (0..=17).
    ///
    /// Blocks until the conversion completes and returns the 12-bit result.
    pub fn read_channel(&mut self, channel: u8) -> u16 {
        self.try_read_channel(channel)
            .unwrap_or_else(|_| panic!("ADC conversion timed out"))
    }

    /// Perform a single conversion, returning [`Error::Timeout`] if the
    /// conversion does not complete.
    pub fn try_read_channel(&mut self, channel: u8) -> Result<u16, Error> {
        self.set_regular_sequence(core::slice::from_ref(&channel));
        self.start_conversion();
        if crate::spin::try_spin_wait(|| self.is_conversion_complete()) {
            Ok(self.read_data())
        } else {
            Err(Error::Timeout)
        }
    }

    /// Set the sample time for the given channel (0..=17).
    pub fn set_sample_time(&mut self, channel: u8, sample_time: SampleTime) {
        debug_assert!(channel <= 17, "invalid ADC channel");
        let v = sample_time as u32;
        match channel {
            0 => self.adc.samptr2_charge2.write_smp0_tkcg0(v),
            1 => self.adc.samptr2_charge2.write_smp1_tkcg1(v),
            2 => self.adc.samptr2_charge2.write_smp2_tkcg2(v),
            3 => self.adc.samptr2_charge2.write_smp3_tkcg3(v),
            4 => self.adc.samptr2_charge2.write_smp4_tkcg4(v),
            5 => self.adc.samptr2_charge2.write_smp5_tkcg5(v),
            6 => self.adc.samptr2_charge2.write_smp6_tkcg6(v),
            7 => self.adc.samptr2_charge2.write_smp7_tkcg7(v),
            8 => self.adc.samptr2_charge2.write_smp8_tkcg8(v),
            9 => self.adc.samptr2_charge2.write_smp9_tkcg9(v),
            10 => self.adc.samptr1_charge1.write_smp10_tkcg10(v),
            11 => self.adc.samptr1_charge1.write_smp11_tkcg11(v),
            12 => self.adc.samptr1_charge1.write_smp12_tkcg12(v),
            13 => self.adc.samptr1_charge1.write_smp13_tkcg13(v),
            14 => self.adc.samptr1_charge1.write_smp14_tkcg14(v),
            15 => self.adc.samptr1_charge1.write_smp15_tkcg15(v),
            16 => self.adc.samptr1_charge1.write_smp16_tkcg16(v),
            _ => self.adc.samptr1_charge1.write_smp17_tkcg17(v),
        }
    }

    /// Enable scan mode (multi-channel regular conversion).
    #[inline(always)]
    pub fn enable_scan(&mut self) {
        self.adc.ctlr1.set_scan();
    }

    /// Disable scan mode.
    #[inline(always)]
    pub fn disable_scan(&mut self) {
        self.adc.ctlr1.clr_scan();
    }

    /// Enable continuous conversion mode.
    #[inline(always)]
    pub fn enable_continuous(&mut self) {
        self.adc.ctlr2.set_cont();
    }

    /// Disable continuous conversion mode.
    #[inline(always)]
    pub fn disable_continuous(&mut self) {
        self.adc.ctlr2.clr_cont();
    }

    /// Enable DMA mode (conversion results are written to memory via DMA).
    #[inline(always)]
    pub fn enable_dma(&mut self) {
        self.adc.ctlr2.set_dma();
    }

    /// Disable DMA mode.
    #[inline(always)]
    pub fn disable_dma(&mut self) {
        self.adc.ctlr2.clr_dma();
    }

    /// Enable the touch-key interface (`TKEYEN`).
    #[inline(always)]
    pub fn enable_touchkey(&mut self) {
        self.adc.ctlr1.set_tkeyen();
    }

    /// Disable the touch-key interface.
    #[inline(always)]
    pub fn disable_touchkey(&mut self) {
        self.adc.ctlr1.clr_tkeyen();
    }

    /// Program the regular conversion sequence (up to 16 channels).
    ///
    /// Scan mode is enabled automatically when more than one channel is given.
    pub fn set_regular_sequence(&mut self, channels: &[u8]) {
        debug_assert!(
            !channels.is_empty() && channels.len() <= 16,
            "invalid ADC sequence"
        );

        if channels.len() > 1 {
            self.enable_scan();
        }
        self.adc.rsqr1.write_l((channels.len() as u32) - 1);

        for (i, &ch) in channels.iter().enumerate() {
            let ch = ch as u32;
            match i + 1 {
                1 => self.adc.rsqr3.write_sq1(ch),
                2 => self.adc.rsqr3.write_sq2(ch),
                3 => self.adc.rsqr3.write_sq3(ch),
                4 => self.adc.rsqr3.write_sq4(ch),
                5 => self.adc.rsqr3.write_sq5(ch),
                6 => self.adc.rsqr3.write_sq6(ch),
                7 => self.adc.rsqr2.write_sq7(ch),
                8 => self.adc.rsqr2.write_sq8(ch),
                9 => self.adc.rsqr2.write_sq9(ch),
                10 => self.adc.rsqr2.write_sq10(ch),
                11 => self.adc.rsqr2.write_sq11(ch),
                12 => self.adc.rsqr2.write_sq12(ch),
                13 => self.adc.rsqr1.write_sq13(ch),
                14 => self.adc.rsqr1.write_sq14(ch),
                15 => self.adc.rsqr1.write_sq15(ch),
                _ => self.adc.rsqr1.write_sq16(ch),
            }
        }
    }

    /// Program the injected conversion sequence (up to 4 channels).
    pub fn set_injected_sequence(&mut self, channels: &[u8]) {
        debug_assert!(
            !channels.is_empty() && channels.len() <= 4,
            "invalid injected sequence"
        );

        self.adc.isqr.write_jl((channels.len() as u32) - 1);
        for (i, &ch) in channels.iter().enumerate() {
            let ch = ch as u32;
            match i {
                0 => self.adc.isqr.write_jsq1(ch),
                1 => self.adc.isqr.write_jsq2(ch),
                2 => self.adc.isqr.write_jsq3(ch),
                _ => self.adc.isqr.write_jsq4(ch),
            }
        }
    }

    /// Start a software-triggered regular conversion.
    #[inline(always)]
    pub fn start_conversion(&mut self) {
        self.adc.ctlr2.set_swstart();
    }

    /// Start a software-triggered injected conversion.
    #[inline(always)]
    pub fn start_injected_conversion(&mut self) {
        self.adc.ctlr2.set_jswstart();
    }

    /// Check whether the regular conversion has completed.
    #[inline(always)]
    pub fn is_conversion_complete(&self) -> bool {
        self.adc.statr.eoc()
    }

    /// Check whether the injected conversion has completed.
    #[inline(always)]
    pub fn is_injected_complete(&self) -> bool {
        self.adc.statr.jeoc()
    }

    /// Clear the regular end-of-conversion flag.
    #[inline(always)]
    pub fn clear_eoc(&mut self) {
        self.adc.statr.clr_eoc();
    }

    /// Clear the injected end-of-conversion flag.
    #[inline(always)]
    pub fn clear_jeoc(&mut self) {
        self.adc.statr.clr_jeoc();
    }

    /// Read the most recent regular conversion result (12-bit).
    #[inline(always)]
    pub fn read_data(&self) -> u16 {
        (self.adc.rdatar_dr_act_dcg.read_bits() & 0x0FFF) as u16
    }

    /// Read an injected conversion result (0-based index into the injected
    /// sequence, 0..=3).
    pub fn read_injected(&self, index: u8) -> u16 {
        match index {
            0 => {
                let r = &self.adc.idatar1_chgoffset;
                (r.jdata0_7_tkcgoffset() | (r.jdata8_15() << 8)) as u16
            }
            1 => self.adc.idatar2.jdata() as u16,
            2 => self.adc.idatar3.jdata() as u16,
            _ => self.adc.idatar4.jdata() as u16,
        }
    }

    /// Configure the analog watchdog on a single channel.
    ///
    /// The watchdog flag (and optionally an interrupt) is raised whenever the
    /// conversion result of `channel` falls outside `low..=high`.
    pub fn configure_watchdog(&mut self, channel: u8, low: u16, high: u16) {
        self.adc.wdltr.write_lt(low as u32 & 0x0FFF);
        self.adc.wdhtr.write_ht(high as u32 & 0x0FFF);
        self.adc.ctlr1.write_awdch(channel as u32 & 0x0F);
        self.adc.ctlr1.set_awdsgl();
        self.adc.ctlr1.set_awden();
    }

    /// Check whether the analog watchdog flag is set.
    #[inline(always)]
    pub fn is_watchdog_flag(&self) -> bool {
        self.adc.statr.awd()
    }

    /// Clear the analog watchdog flag.
    #[inline(always)]
    pub fn clear_watchdog_flag(&mut self) {
        self.adc.statr.clr_awd();
    }

    /// Select the external trigger source for regular conversions and enable
    /// external triggering.
    pub fn set_external_trigger(&mut self, trigger: ExternalTrigger) {
        self.adc.ctlr2.write_extsel(trigger as u32);
        self.adc.ctlr2.set_exttrig();
    }

    /// Disable external triggering of regular conversions.
    #[inline(always)]
    pub fn disable_external_trigger(&mut self) {
        self.adc.ctlr2.clr_exttrig();
    }

    /// Select the external trigger source for injected conversions and enable
    /// external triggering.
    pub fn set_injected_trigger(&mut self, trigger: InjectedTrigger) {
        self.adc.ctlr2.write_jextsel(trigger as u32);
        self.adc.ctlr2.set_jexttrig();
    }

    /// Disable external triggering of injected conversions.
    #[inline(always)]
    pub fn disable_injected_trigger(&mut self) {
        self.adc.ctlr2.clr_jexttrig();
    }

    /// Enable the regular end-of-conversion interrupt.
    #[inline(always)]
    pub fn enable_eoc_interrupt(&mut self) {
        self.adc.ctlr1.set_eocie();
    }

    /// Disable the regular end-of-conversion interrupt.
    #[inline(always)]
    pub fn disable_eoc_interrupt(&mut self) {
        self.adc.ctlr1.clr_eocie();
    }

    /// Enable the injected end-of-conversion interrupt.
    #[inline(always)]
    pub fn enable_injected_interrupt(&mut self) {
        self.adc.ctlr1.set_jeocie();
    }

    /// Disable the injected end-of-conversion interrupt.
    #[inline(always)]
    pub fn disable_injected_interrupt(&mut self) {
        self.adc.ctlr1.clr_jeocie();
    }

    /// Enable the analog watchdog interrupt.
    #[inline(always)]
    pub fn enable_watchdog_interrupt(&mut self) {
        self.adc.ctlr1.set_awdie();
    }

    /// Disable the analog watchdog interrupt.
    #[inline(always)]
    pub fn disable_watchdog_interrupt(&mut self) {
        self.adc.ctlr1.clr_awdie();
    }

    /// Consume the driver and return the underlying peripheral.
    #[inline(always)]
    pub fn release(self) -> pac::ADC {
        self.adc
    }
}
