//! CH32X035 sub-model (package) selection.
//!
//! The CH32X035 family is offered in seven packages. Every package exposes
//! the same register block, but each one bonds out a different subset of the
//! GPIO pins. A few peripheral blocks also differ in size:
//!
//! | Feature          | Package  | I/O | OPA | CMP | USB host | ADC ext. ch |
//! |------------------|----------|-----|-----|-----|----------|-------------|
//! | `ch32x035r8t6`   | LQFP64M  | 60  | 2   | 3   | yes      | 14          |
//! | `ch32x035c8t6`   | LQFP48   | 46  | 2   | 3   | yes      | 10          |
//! | `ch32x035g8u6`   | QFN28    | 27  | 2   | 1   | yes      | 12          |
//! | `ch32x035g8r6`   | QSOP28   | 26  | 2   | 3   | yes      | 11          |
//! | `ch32x035f8u6`   | QFN20    | 19  | 2   | 0   | no       | 10          |
//! | `ch32x035f7p6`   | TSSOP20  | 18  | 1   | 1   | no       | 11          |
//! | `ch32x035d8u6`   | QFN12    | 11  | 1   | 0   | no       | 5           |
//!
//! Sub-model features are additive. Enable the feature(s) for the package(s)
//! you target: with one model selected, [`crate::gpio::Pins`] rejects every
//! pin not bonded out on it; with several (e.g. `cargo check --all-features`),
//! the union is exposed instead. The pin availability helpers
//! ([`pa_available`], [`pb_available`], [`pc_available`]) and the peripheral
//! counts ([`OPA_COUNT`], [`CMP_COUNT`], [`ADC_CHANNELS`]) / flags
//! ([`HAS_USB_HOST`]) are all defined as that union, and [`crate::opa::Opa`]
//! gates its OPA2 / CMP1-3 methods accordingly. [`MODEL`] names the part when
//! exactly one model is selected.
//!
//! The pin bonding itself lives in [`crate::pin`]; this module only adds the
//! per-model peripheral counts and the `const fn` wrappers that
//! [`crate::gpio::Pins`] uses for its compile-time checks.
//!
//! # Notes on shorted pins
//!
//! Some packages bond two logical pins to the same physical pad, so they may
//! not be driven independently (see [`crate::pin::SHARED`]):
//!
//! - `ch32x035g8u6` / `ch32x035g8r6`: PB1 and PB5.
//! - `ch32x035g8r6`: PA12/PC14 and PA13/PC15.
//! - `ch32x035d8u6`: PA7/PC7.
//!
//! The CH32X033 (TSSOP20) is a separate part in the same datasheet with its
//! own pinout and no USB-PD; it is not covered by these features.

use crate::pin::{self, Port};

/// Evaluates to `true` if any of the listed sub-model features is enabled.
macro_rules! any_model {
    ($($feat:literal),+ $(,)?) => {
        false $(|| cfg!(feature = $feat))+
    };
}

/// Number of enabled sub-model features.
const MODEL_COUNT: u32 = cfg!(feature = "ch32x035r8t6") as u32
    + cfg!(feature = "ch32x035c8t6") as u32
    + cfg!(feature = "ch32x035g8u6") as u32
    + cfg!(feature = "ch32x035g8r6") as u32
    + cfg!(feature = "ch32x035f8u6") as u32
    + cfg!(feature = "ch32x035f7p6") as u32
    + cfg!(feature = "ch32x035d8u6") as u32;

/// Name of the selected sub-model.
///
/// Meaningful when exactly one sub-model feature is enabled. Returns a
/// placeholder when none are enabled or when several are enabled (e.g. under
/// `cargo check --all-features`).
///
/// Expressed as a `match` on the compile-time `MODEL_COUNT` so the "0 / 1 /
/// multiple" cases are explicit. Using comparisons against the constant would
/// make clippy's `absurd_extreme_comparisons` misfire: it constant-folds
/// `MODEL_COUNT` under the current feature set (e.g. `0` with no model
/// selected), flagging `== 0` / `> 1` branches as always true/false even
/// though they are reachable under other feature combinations.
pub const MODEL: &str = match MODEL_COUNT {
    0 => "CH32X035 (no model selected)",
    1 => {
        if cfg!(feature = "ch32x035r8t6") {
            "CH32X035R8T6"
        } else if cfg!(feature = "ch32x035c8t6") {
            "CH32X035C8T6"
        } else if cfg!(feature = "ch32x035g8u6") {
            "CH32X035G8U6"
        } else if cfg!(feature = "ch32x035g8r6") {
            "CH32X035G8R6"
        } else if cfg!(feature = "ch32x035f8u6") {
            "CH32X035F8U6"
        } else if cfg!(feature = "ch32x035f7p6") {
            "CH32X035F7P6"
        } else {
            "CH32X035D8U6"
        }
    }
    _ => "CH32X035 (multiple models)",
};

/// Number of physical GPIO pads on the selected package(s).
///
/// Pads shared by two logical pins (see [`crate::pin::SHARED`]) are counted
/// once, so this matches the datasheet "I/O" figure.
pub const GPIO_PADS: u8 = pin::gpio_pads_enabled();

/// Number of operational amplifiers on the selected package: 1 or 2.
///
/// OPA1 is present on every package; OPA2 is additionally present when this
/// is `2`.
pub const OPA_COUNT: u8 = if cfg!(feature = "__opa2") { 2 } else { 1 };

/// Number of comparators (CMP) on the selected package: 0, 1 or 3.
pub const CMP_COUNT: u8 = if cfg!(feature = "__cmp3") {
    3
} else if cfg!(feature = "__cmp1") {
    1
} else {
    0
};

/// Whether the selected package has the USB host (USBFS) peripheral.
pub const HAS_USB_HOST: bool = any_model!(
    "ch32x035r8t6",
    "ch32x035c8t6",
    "ch32x035g8u6",
    "ch32x035g8r6"
);

/// Number of external ADC input channels bonded out on the selected package.
///
/// This counts the ADC-capable pads that the enabled package(s) actually bond
/// out (union when several are enabled); see [`crate::pin::adc_channels`].
pub const ADC_CHANNELS: u8 = pin::adc_channels_enabled();

/// Whether the pin at `(PORT, N)` is bonded out on the selected package.
///
/// `PORT`: 0 = A, 1 = B, 2 = C. Compile-time (const generic) version.
#[inline]
pub const fn pin_available<const PORT: u32, const N: u32>() -> bool {
    match PORT {
        0 => pin::bonded(Port::A, N as u8),
        1 => pin::bonded(Port::B, N as u8),
        2 => pin::bonded(Port::C, N as u8),
        _ => false,
    }
}

/// Whether the pin at `(port, pin)` is bonded out on the selected package.
///
/// `port`: 0 = A, 1 = B, 2 = C; `pin`: pin number 0..=23. Runtime version of
/// [`pin_available`], used by the runtime-indexed GPIO free functions.
pub fn pin_available_runtime(port: u32, pin: u32) -> bool {
    if pin > 23 {
        return false;
    }
    match port {
        0 => pin::bonded(Port::A, pin as u8),
        1 => pin::bonded(Port::B, pin as u8),
        2 => pin::bonded(Port::C, pin as u8),
        _ => false,
    }
}

/// Whether `PA{n}` is bonded out on the selected package (compile-time version).
#[inline]
pub const fn pa_available<const N: u32>() -> bool {
    pin_available::<0, N>()
}

/// Whether `PA{pin}` is bonded out on the selected package (runtime version).
pub fn pa_available_runtime(pin: u32) -> bool {
    pin_available_runtime(0, pin)
}

/// Whether `PB{n}` is bonded out on the selected package (compile-time version).
#[inline]
pub const fn pb_available<const N: u32>() -> bool {
    pin_available::<1, N>()
}

/// Whether `PB{pin}` is bonded out on the selected package (runtime version).
pub fn pb_available_runtime(pin: u32) -> bool {
    pin_available_runtime(1, pin)
}

/// Whether `PC{n}` is bonded out on the selected package (compile-time version).
///
/// PC8..=PC13 and PC20+ are never bonded out on any package.
#[inline]
pub const fn pc_available<const N: u32>() -> bool {
    pin_available::<2, N>()
}

/// Whether `PC{pin}` is bonded out on the selected package (runtime version).
pub fn pc_available_runtime(pin: u32) -> bool {
    pin_available_runtime(2, pin)
}

// =============================================================================
// Consistency checks
// =============================================================================

// The per-model peripheral counts documented above must agree with the pin
// tables. Checked for single-model builds (with several models enabled the
// constants are unions, so they are not comparable one-to-one).
const _: () = {
    if MODEL_COUNT == 1 {
        assert!(
            ADC_CHANNELS == pin::adc_channels_enabled(),
            "ADC_CHANNELS disagrees with pin::adc_channels"
        );
        assert!(
            GPIO_PADS == pin::gpio_pads_enabled(),
            "GPIO_PADS disagrees with pin::gpio_pads"
        );
    }
};

// Per-model figures from the datasheet model table, cross-checked against the
// bonding table (see the assertions in `crate::pin`).
const _: () = {
    if cfg!(feature = "ch32x035r8t6") {
        assert!(pin::gpio_pads(pin::Package::R8T6.bit()) == 60);
        assert!(pin::adc_channels(pin::Package::R8T6.bit()) == 14);
    }
    if cfg!(feature = "ch32x035c8t6") {
        assert!(pin::gpio_pads(pin::Package::C8T6.bit()) == 46);
        assert!(pin::adc_channels(pin::Package::C8T6.bit()) == 10);
    }
    if cfg!(feature = "ch32x035g8u6") {
        assert!(pin::gpio_pads(pin::Package::G8U6.bit()) == 27);
        assert!(pin::adc_channels(pin::Package::G8U6.bit()) == 12);
    }
    if cfg!(feature = "ch32x035g8r6") {
        assert!(pin::gpio_pads(pin::Package::G8R6.bit()) == 26);
        assert!(pin::adc_channels(pin::Package::G8R6.bit()) == 11);
    }
    if cfg!(feature = "ch32x035f8u6") {
        assert!(pin::gpio_pads(pin::Package::F8U6.bit()) == 19);
        assert!(pin::adc_channels(pin::Package::F8U6.bit()) == 10);
    }
    if cfg!(feature = "ch32x035f7p6") {
        assert!(pin::gpio_pads(pin::Package::F7P6.bit()) == 18);
        assert!(pin::adc_channels(pin::Package::F7P6.bit()) == 11);
    }
    if cfg!(feature = "ch32x035d8u6") {
        assert!(pin::gpio_pads(pin::Package::D8U6.bit()) == 11);
        assert!(pin::adc_channels(pin::Package::D8U6.bit()) == 5);
    }
};
