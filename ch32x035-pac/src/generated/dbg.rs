//! DBG
//! Debug support

#![allow(dead_code)]

pub const BASE: u32 = 0xe000d000;

/// CR Register
///
/// DBGMCU_CR
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIM3_STOP** `[14]` (RW)
///
///   TIM3_STOP
///
/// - **TIM2_STOP** `[13]` (RW)
///
///   TIM2_STOP
///
/// - **TIM1_STOP** `[12]` (RW)
///
///   TIM1_STOP
///
/// - **WWDG_STOP** `[9]` (RW)
///
///   WWDG_STOP
///
/// - **IWDG_STOP** `[8]` (RW)
///
///   IWDG_STOP
///
/// - **STANDBY** `[2]` (RW)
///
///   STANDBY
///
/// - **STOP** `[1]` (RW)
///
///   STOP
///
/// - **SLEEP** `[0]` (RW)
///
///   SLEEP
#[repr(transparent)]
pub struct Cr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cr {
    // ======== Field Constants ========

    /// **TIM3_STOP**
    ///
    /// TIM3_STOP
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TIM3_STOP_OFFSET: u32 = 14;
    pub const TIM3_STOP_MASK: u32 = 0x00004000;
    pub const TIM3_STOP: u32 = 1 << 14;

    /// **TIM2_STOP**
    ///
    /// TIM2_STOP
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TIM2_STOP_OFFSET: u32 = 13;
    pub const TIM2_STOP_MASK: u32 = 0x00002000;
    pub const TIM2_STOP: u32 = 1 << 13;

    /// **TIM1_STOP**
    ///
    /// TIM1_STOP
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TIM1_STOP_OFFSET: u32 = 12;
    pub const TIM1_STOP_MASK: u32 = 0x00001000;
    pub const TIM1_STOP: u32 = 1 << 12;

    /// **WWDG_STOP**
    ///
    /// WWDG_STOP
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const WWDG_STOP_OFFSET: u32 = 9;
    pub const WWDG_STOP_MASK: u32 = 0x00000200;
    pub const WWDG_STOP: u32 = 1 << 9;

    /// **IWDG_STOP**
    ///
    /// IWDG_STOP
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const IWDG_STOP_OFFSET: u32 = 8;
    pub const IWDG_STOP_MASK: u32 = 0x00000100;
    pub const IWDG_STOP: u32 = 1 << 8;

    /// **STANDBY**
    ///
    /// STANDBY
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const STANDBY_OFFSET: u32 = 2;
    pub const STANDBY_MASK: u32 = 0x00000004;
    pub const STANDBY: u32 = 1 << 2;

    /// **STOP**
    ///
    /// STOP
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const STOP_OFFSET: u32 = 1;
    pub const STOP_MASK: u32 = 0x00000002;
    pub const STOP: u32 = 1 << 1;

    /// **SLEEP**
    ///
    /// SLEEP
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEEP_OFFSET: u32 = 0;
    pub const SLEEP_MASK: u32 = 0x00000001;
    pub const SLEEP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIM3_STOP** `[14]` (RW)
    ///
    /// TIM3_STOP
    #[inline(always)]
    pub fn tim3_stop(&self) -> bool {
        (self.value.read_bits() & Self::TIM3_STOP_MASK) != 0
    }

    /// Set TIM3_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim3_stop(&self) {
        self.value.modify_bits(|r| r | Self::TIM3_STOP_MASK);
    }

    /// Clear TIM3_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim3_stop(&self) {
        self.value.modify_bits(|r| r & !Self::TIM3_STOP_MASK);
    }

    /// **TIM2_STOP** `[13]` (RW)
    ///
    /// TIM2_STOP
    #[inline(always)]
    pub fn tim2_stop(&self) -> bool {
        (self.value.read_bits() & Self::TIM2_STOP_MASK) != 0
    }

    /// Set TIM2_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim2_stop(&self) {
        self.value.modify_bits(|r| r | Self::TIM2_STOP_MASK);
    }

    /// Clear TIM2_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim2_stop(&self) {
        self.value.modify_bits(|r| r & !Self::TIM2_STOP_MASK);
    }

    /// **TIM1_STOP** `[12]` (RW)
    ///
    /// TIM1_STOP
    #[inline(always)]
    pub fn tim1_stop(&self) -> bool {
        (self.value.read_bits() & Self::TIM1_STOP_MASK) != 0
    }

    /// Set TIM1_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tim1_stop(&self) {
        self.value.modify_bits(|r| r | Self::TIM1_STOP_MASK);
    }

    /// Clear TIM1_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tim1_stop(&self) {
        self.value.modify_bits(|r| r & !Self::TIM1_STOP_MASK);
    }

    /// **WWDG_STOP** `[9]` (RW)
    ///
    /// WWDG_STOP
    #[inline(always)]
    pub fn wwdg_stop(&self) -> bool {
        (self.value.read_bits() & Self::WWDG_STOP_MASK) != 0
    }

    /// Set WWDG_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wwdg_stop(&self) {
        self.value.modify_bits(|r| r | Self::WWDG_STOP_MASK);
    }

    /// Clear WWDG_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wwdg_stop(&self) {
        self.value.modify_bits(|r| r & !Self::WWDG_STOP_MASK);
    }

    /// **IWDG_STOP** `[8]` (RW)
    ///
    /// IWDG_STOP
    #[inline(always)]
    pub fn iwdg_stop(&self) -> bool {
        (self.value.read_bits() & Self::IWDG_STOP_MASK) != 0
    }

    /// Set IWDG_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_iwdg_stop(&self) {
        self.value.modify_bits(|r| r | Self::IWDG_STOP_MASK);
    }

    /// Clear IWDG_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_iwdg_stop(&self) {
        self.value.modify_bits(|r| r & !Self::IWDG_STOP_MASK);
    }

    /// **STANDBY** `[2]` (RW)
    ///
    /// STANDBY
    #[inline(always)]
    pub fn standby(&self) -> bool {
        (self.value.read_bits() & Self::STANDBY_MASK) != 0
    }

    /// Set STANDBY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_standby(&self) {
        self.value.modify_bits(|r| r | Self::STANDBY_MASK);
    }

    /// Clear STANDBY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_standby(&self) {
        self.value.modify_bits(|r| r & !Self::STANDBY_MASK);
    }

    /// **STOP** `[1]` (RW)
    ///
    /// STOP
    #[inline(always)]
    pub fn stop(&self) -> bool {
        (self.value.read_bits() & Self::STOP_MASK) != 0
    }

    /// Set STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_stop(&self) {
        self.value.modify_bits(|r| r | Self::STOP_MASK);
    }

    /// Clear STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_stop(&self) {
        self.value.modify_bits(|r| r & !Self::STOP_MASK);
    }

    /// **SLEEP** `[0]` (RW)
    ///
    /// SLEEP
    #[inline(always)]
    pub fn sleep(&self) -> bool {
        (self.value.read_bits() & Self::SLEEP_MASK) != 0
    }

    /// Set SLEEP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sleep(&self) {
        self.value.modify_bits(|r| r | Self::SLEEP_MASK);
    }

    /// Clear SLEEP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sleep(&self) {
        self.value.modify_bits(|r| r & !Self::SLEEP_MASK);
    }
}

/// CR value type for batch read/modify/write operations
///
/// Use `cr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CrVal {
    bits: u32,
}

impl CrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **TIM3_STOP** `[14:14]`
    ///
    /// TIM3_STOP
    #[inline(always)]
    pub const fn tim3_stop(&self) -> bool {
        (self.bits & Cr::TIM3_STOP_MASK) != 0
    }

    /// Read **TIM2_STOP** `[13:13]`
    ///
    /// TIM2_STOP
    #[inline(always)]
    pub const fn tim2_stop(&self) -> bool {
        (self.bits & Cr::TIM2_STOP_MASK) != 0
    }

    /// Read **TIM1_STOP** `[12:12]`
    ///
    /// TIM1_STOP
    #[inline(always)]
    pub const fn tim1_stop(&self) -> bool {
        (self.bits & Cr::TIM1_STOP_MASK) != 0
    }

    /// Read **WWDG_STOP** `[9:9]`
    ///
    /// WWDG_STOP
    #[inline(always)]
    pub const fn wwdg_stop(&self) -> bool {
        (self.bits & Cr::WWDG_STOP_MASK) != 0
    }

    /// Read **IWDG_STOP** `[8:8]`
    ///
    /// IWDG_STOP
    #[inline(always)]
    pub const fn iwdg_stop(&self) -> bool {
        (self.bits & Cr::IWDG_STOP_MASK) != 0
    }

    /// Read **STANDBY** `[2:2]`
    ///
    /// STANDBY
    #[inline(always)]
    pub const fn standby(&self) -> bool {
        (self.bits & Cr::STANDBY_MASK) != 0
    }

    /// Read **STOP** `[1:1]`
    ///
    /// STOP
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        (self.bits & Cr::STOP_MASK) != 0
    }

    /// Read **SLEEP** `[0:0]`
    ///
    /// SLEEP
    #[inline(always)]
    pub const fn sleep(&self) -> bool {
        (self.bits & Cr::SLEEP_MASK) != 0
    }

    /// Set **TIM3_STOP** `[14:14]`
    ///
    /// TIM3_STOP
    #[inline(always)]
    pub const fn with_tim3_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::TIM3_STOP_MASK
            } else {
                self.bits & !Cr::TIM3_STOP_MASK
            },
        }
    }

    /// Set **TIM2_STOP** `[13:13]`
    ///
    /// TIM2_STOP
    #[inline(always)]
    pub const fn with_tim2_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::TIM2_STOP_MASK
            } else {
                self.bits & !Cr::TIM2_STOP_MASK
            },
        }
    }

    /// Set **TIM1_STOP** `[12:12]`
    ///
    /// TIM1_STOP
    #[inline(always)]
    pub const fn with_tim1_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::TIM1_STOP_MASK
            } else {
                self.bits & !Cr::TIM1_STOP_MASK
            },
        }
    }

    /// Set **WWDG_STOP** `[9:9]`
    ///
    /// WWDG_STOP
    #[inline(always)]
    pub const fn with_wwdg_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::WWDG_STOP_MASK
            } else {
                self.bits & !Cr::WWDG_STOP_MASK
            },
        }
    }

    /// Set **IWDG_STOP** `[8:8]`
    ///
    /// IWDG_STOP
    #[inline(always)]
    pub const fn with_iwdg_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::IWDG_STOP_MASK
            } else {
                self.bits & !Cr::IWDG_STOP_MASK
            },
        }
    }

    /// Set **STANDBY** `[2:2]`
    ///
    /// STANDBY
    #[inline(always)]
    pub const fn with_standby(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::STANDBY_MASK
            } else {
                self.bits & !Cr::STANDBY_MASK
            },
        }
    }

    /// Set **STOP** `[1:1]`
    ///
    /// STOP
    #[inline(always)]
    pub const fn with_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::STOP_MASK
            } else {
                self.bits & !Cr::STOP_MASK
            },
        }
    }

    /// Set **SLEEP** `[0:0]`
    ///
    /// SLEEP
    #[inline(always)]
    pub const fn with_sleep(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cr::SLEEP_MASK
            } else {
                self.bits & !Cr::SLEEP_MASK
            },
        }
    }
}

impl Default for CrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CrVal {
        CrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CrVal) -> CrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// DBGMCU_CR
    pub cr: Cr,
}
