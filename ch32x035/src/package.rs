//! CH32X035 sub-model (package) selection.
//!
//! The CH32X035 family is offered in seven packages. Every package exposes
//! the same register block, but each one bonds out a different subset of the
//! 60 GPIO pins. A few peripheral blocks also differ in size:
//!
//! | Feature          | Package  | GPIO | OPA | CMP | USB host | ADC ext. ch |
//! |------------------|----------|------|-----|-----|----------|-------------|
//! | `ch32x035r8t6`   | LQFP64M  | 60   | 2   | 3   | yes      | 14          |
//! | `ch32x035c8t6`   | LQFP48   | 46   | 2   | 3   | yes      | 10          |
//! | `ch32x035g8u6`   | QFN28    | 27   | 2   | 1   | yes      | 12          |
//! | `ch32x035g8r6`   | QSOP28   | 26   | 2   | 3   | yes      | 11          |
//! | `ch32x035f8u6`   | QFN20    | 19   | 2   | 0   | no       | 10          |
//! | `ch32x035f7p6`   | TSSOP20  | 18   | 1   | 1   | no       | 11          |
//! | `ch32x035d8u6`   | QFN12    | 11   | 1   | 0   | no       | 5           |
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
//! # Notes on shorted pins
//!
//! Some packages bond two logical pins to the same physical pin. They may not
//! be driven independently:
//!
//! - `ch32x035g8u6` / `ch32x035g8r6`: PB1 and PB5.
//! - `ch32x035g8r6`: PA12/PC14 and PA13/PC15.
//! - `ch32x035d8u6`: PA7/PC7.
//!
//! The CH32X033 (TSSOP20) is a separate part in the same datasheet with its
//! own pinout and no USB-PD; it is not covered by these features.

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
pub const MODEL: &str = if MODEL_COUNT == 0 {
    "CH32X035 (no model selected)"
} else if MODEL_COUNT > 1 {
    "CH32X035 (multiple models)"
} else if cfg!(feature = "ch32x035r8t6") {
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
};

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
pub const ADC_CHANNELS: u8 = if cfg!(feature = "ch32x035r8t6") {
    14
} else if cfg!(feature = "ch32x035c8t6") {
    10
} else if cfg!(feature = "ch32x035g8u6") {
    12
} else if cfg!(feature = "ch32x035g8r6") {
    11
} else if cfg!(feature = "ch32x035f8u6") {
    10
} else if cfg!(feature = "ch32x035f7p6") {
    11
} else {
    5
};

/// Generates the compile-time (`const N`) and runtime (`pin`) availability
/// checks for one port. Both share the same `match` body, so the pin lists
/// cannot drift.
macro_rules! impl_availability {
    ($const_doc:expr, $runtime_doc:expr, $const_name:ident, $runtime_name:ident, $body:tt) => {
        #[doc = $const_doc]
        pub const fn $const_name<const N: u32>() -> bool {
            match N $body
        }

        #[doc = $runtime_doc]
        pub fn $runtime_name(pin: u32) -> bool {
            match pin $body
        }
    };
}

impl_availability!(
    "Whether `PA{n}` is bonded out on the selected package (compile-time version).",
    "Whether `PA{pin}` is bonded out on the selected package (runtime version).",
    pa_available,
    pa_available_runtime,
    {
        0 | 1 | 4 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6",
            "ch32x035f7p6"
        ),
        2 | 3 | 5 | 6 | 7 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6",
            "ch32x035f7p6",
            "ch32x035d8u6"
        ),
        8..=11 | 14..=23 => any_model!("ch32x035r8t6", "ch32x035c8t6"),
        12 | 13 => any_model!("ch32x035r8t6", "ch32x035c8t6", "ch32x035g8r6"),
        _ => false,
    }
);

impl_availability!(
    "Whether `PB{n}` is bonded out on the selected package (compile-time version).",
    "Whether `PB{pin}` is bonded out on the selected package (runtime version).",
    pb_available,
    pb_available_runtime,
    {
        0 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6"
        ),
        1 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6",
            "ch32x035f7p6"
        ),
        2 => any_model!("ch32x035r8t6", "ch32x035c8t6"),
        3 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6"
        ),
        4..=10 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6"
        ),
        11 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6"
        ),
        12 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6",
            "ch32x035f7p6"
        ),
        13 => any_model!("ch32x035r8t6", "ch32x035c8t6"),
        14..=21 => any_model!("ch32x035r8t6"),
        _ => false,
    }
);

impl_availability!(
    "Whether `PC{n}` is bonded out on the selected package (compile-time version).\n\nPC8..=PC13 and PC20+ are never bonded out on any package.",
    "Whether `PC{pin}` is bonded out on the selected package (runtime version).\n\nPC8..=PC13 and PC20+ are never bonded out on any package.",
    pc_available,
    pc_available_runtime,
    {
        0 => any_model!("ch32x035r8t6", "ch32x035g8u6"),
        1 => any_model!("ch32x035r8t6", "ch32x035f7p6"),
        2 => any_model!("ch32x035r8t6"),
        3 => any_model!(
            "ch32x035r8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f7p6"
        ),
        4 | 5 => any_model!("ch32x035r8t6"),
        6 | 7 => any_model!("ch32x035r8t6", "ch32x035c8t6"),
        14..=19 => any_model!(
            "ch32x035r8t6",
            "ch32x035c8t6",
            "ch32x035g8u6",
            "ch32x035g8r6",
            "ch32x035f8u6",
            "ch32x035f7p6",
            "ch32x035d8u6"
        ),
        _ => false,
    }
);

/// Whether the pin at `(PORT, N)` is bonded out on the selected package.
///
/// `PORT`: 0 = A, 1 = B, 2 = C. Compile-time (const generic) version; see
/// [`pin_available_runtime`] for the runtime form.
pub const fn pin_available<const PORT: u32, const N: u32>() -> bool {
    match PORT {
        0 => pa_available::<N>(),
        1 => pb_available::<N>(),
        2 => pc_available::<N>(),
        _ => false,
    }
}

/// Whether the pin at `(port, pin)` is bonded out on the selected package.
///
/// `port`: 0 = A, 1 = B, 2 = C; `pin`: pin number 0..=23. Runtime version of
/// [`pin_available`], used by the runtime-indexed GPIO free functions.
pub fn pin_available_runtime(port: u32, pin: u32) -> bool {
    match port {
        0 => pa_available_runtime(pin),
        1 => pb_available_runtime(pin),
        2 => pc_available_runtime(pin),
        _ => false,
    }
}
