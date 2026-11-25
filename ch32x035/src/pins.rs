//! Compile-time pin constraints.
//!
//! [`crate::pin`] describes every pad; this module turns that description into
//! traits so a peripheral can only be handed a pin that physically carries the
//! requested signal.
//!
//! Each implementation is guarded by a `const` assertion against
//! [`crate::pin::supports`], [`crate::pin::has_analog`] and
//! [`crate::pin::exists`], so the implementation lists and the description
//! tables cannot drift apart: a pin listed here that the tables do not know
//! fails to compile. Whether a pad is bonded out on the *selected* package is
//! enforced by [`crate::gpio::Pins`] when the pin token is created, so the
//! trait set does not vary with the package features.
//!
//! Every trait carries the pin identity ([`PinId`]) as `ID`, which lets the
//! pair helpers ([`usart1_remap`] … [`spi1_remap`]) find the `AFIO_PCFR1`
//! remap value that serves a whole bus at once.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::pins::{Spi1MisoPin, Spi1MosiPin, Spi1SckPin};
//!
//! // Only pins that really carry SPI1 MISO implement `Spi1MisoPin`.
//! let spi = Spi1::init_with_pins(
//!     dp.SPI1, &mut sck, &mut miso, &mut mosi, &mut afio, &mut rcc, clocks, cfg,
//! );
//! ```

use crate::gpio::Pin;
use crate::pin::{AnalogRole, PinFn, PinId, Port};

/// ADC channel/setup implementations.
macro_rules! impl_adc_pins {
    ($(($port:ident, $portnum:literal, $pnum:literal, $ch:literal)),+ $(,)?) => {
        $(
            const _: () = {
                assert!(crate::pin::exists(Port::$port, $pnum));
                assert!(matches!(
                    crate::pin::adc_channel(Port::$port, $pnum),
                    Some(ch) if ch == $ch
                ));
            };

            impl AdcPin for Pin<$portnum, $pnum> {
                const ID: PinId = PinId::new(Port::$port, $pnum);
                const CHANNEL: u8 = $ch;

                #[inline]
                fn configure_adc_pin(&mut self) {
                    self.into_analog();
                }
            }
        )+
    };
}

/// Implement one digital-function trait for a list of `(port, pin)` pairs.
///
/// The `const` assertion ties the implementation to [`crate::pin::supports`].
macro_rules! impl_fn_pins {
    ($trait:ident => $id:ident, $method:ident, $func:path, $setup:ident;
     $(($port:ident, $portnum:literal, $pnum:literal)),+ $(,)?) => {
        $(
            const _: () = {
                assert!(crate::pin::supports(Port::$port, $pnum, $func));
                assert!(crate::pin::exists(Port::$port, $pnum));
            };

            impl $trait for Pin<$portnum, $pnum> {
                const $id: PinId = PinId::new(Port::$port, $pnum);

                #[inline]
                fn $method(&mut self) {
                    self.$setup();
                }
            }
        )+
    };
}

/// Implement one "analog input with a select code" trait.
///
/// `$value` is the input-select code the peripheral needs, `$role` the
/// matching [`crate::pin::AnalogRole`], so the implementation is checked
/// against the pin tables.
macro_rules! impl_analog_select_pins {
    ($trait:ident => $id:ident, $sel:ident, $method:ident;
     $(($port:ident, $portnum:literal, $pnum:literal, $value:literal, $role:ident)),+ $(,)?) => {
        $(
            const _: () = {
                assert!(crate::pin::exists(Port::$port, $pnum));
                assert!(crate::pin::has_analog(AnalogRole::$role, Port::$port, $pnum));
            };

            impl $trait for Pin<$portnum, $pnum> {
                const $id: PinId = PinId::new(Port::$port, $pnum);
                const $sel: u8 = $value;

                #[inline]
                fn $method(&mut self) {
                    self.into_analog();
                }
            }
        )+
    };
}

// ============================================================================
// USART1
// ============================================================================

/// Pins that can carry the USART1 TX signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart1TxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_tx(&mut self);
}

impl_fn_pins!(Usart1TxPin => ID, configure_tx, PinFn::Usart1Tx, into_alt_push_pull;
    (A, 0, 7),
    (A, 0, 10),
    (B, 1, 10),
);

/// Pins that can carry the USART1 RX signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart1RxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rx(&mut self);
}

impl_fn_pins!(Usart1RxPin => ID, configure_rx, PinFn::Usart1Rx, into_input;
    (A, 0, 11),
    (B, 1, 2),
    (B, 1, 11),
);

/// Pins that can carry the USART1 CK signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart1CkPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ck(&mut self);
}

impl_fn_pins!(Usart1CkPin => ID, configure_ck, PinFn::Usart1Ck, into_alt_push_pull;
    (B, 1, 5),
    (B, 1, 9),
    (B, 1, 12),
);

/// Pins that can carry the USART1 CTS signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart1CtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_cts(&mut self);
}

impl_fn_pins!(Usart1CtsPin => ID, configure_cts, PinFn::Usart1Cts, into_input;
    (A, 0, 9),
    (A, 0, 13),
    (C, 2, 16),
);

/// Pins that can carry the USART1 RTS signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart1RtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rts(&mut self);
}

impl_fn_pins!(Usart1RtsPin => ID, configure_rts, PinFn::Usart1Rts, into_alt_push_pull;
    (A, 0, 8),
    (A, 0, 14),
    (C, 2, 17),
);

// ============================================================================
// USART2
// ============================================================================

/// Pins that can carry the USART2 TX signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart2TxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_tx(&mut self);
}

impl_fn_pins!(Usart2TxPin => ID, configure_tx, PinFn::Usart2Tx, into_alt_push_pull;
    (A, 0, 2),
    (A, 0, 15),
    (A, 0, 20),
    (C, 2, 0),
);

/// Pins that can carry the USART2 RX signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart2RxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rx(&mut self);
}

impl_fn_pins!(Usart2RxPin => ID, configure_rx, PinFn::Usart2Rx, into_input;
    (A, 0, 3),
    (A, 0, 16),
    (A, 0, 19),
    (C, 2, 1),
);

/// Pins that can carry the USART2 CK signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart2CkPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ck(&mut self);
}

impl_fn_pins!(Usart2CkPin => ID, configure_ck, PinFn::Usart2Ck, into_alt_push_pull;
    (A, 0, 4),
    (A, 0, 22),
    (A, 0, 23),
    (B, 1, 20),
);

/// Pins that can carry the USART2 CTS signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart2CtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_cts(&mut self);
}

impl_fn_pins!(Usart2CtsPin => ID, configure_cts, PinFn::Usart2Cts, into_input;
    (A, 0, 0),
    (A, 0, 1),
    (A, 0, 17),
    (C, 2, 2),
);

/// Pins that can carry the USART2 RTS signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart2RtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rts(&mut self);
}

impl_fn_pins!(Usart2RtsPin => ID, configure_rts, PinFn::Usart2Rts, into_alt_push_pull;
    (A, 0, 1),
    (A, 0, 2),
    (A, 0, 21),
    (C, 2, 3),
);

// ============================================================================
// USART3
// ============================================================================

/// Pins that can carry the USART3 TX signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart3TxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_tx(&mut self);
}

impl_fn_pins!(Usart3TxPin => ID, configure_tx, PinFn::Usart3Tx, into_alt_push_pull;
    (A, 0, 18),
    (B, 1, 3),
    (B, 1, 16),
    (C, 2, 18),
);

/// Pins that can carry the USART3 RX signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart3RxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rx(&mut self);
}

impl_fn_pins!(Usart3RxPin => ID, configure_rx, PinFn::Usart3Rx, into_input;
    (B, 1, 4),
    (B, 1, 14),
    (B, 1, 17),
    (C, 2, 19),
);

/// Pins that can carry the USART3 CK signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart3CkPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ck(&mut self);
}

impl_fn_pins!(Usart3CkPin => ID, configure_ck, PinFn::Usart3Ck, into_alt_push_pull;
    (B, 1, 5),
    (B, 1, 8),
);

/// Pins that can carry the USART3 CTS signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart3CtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_cts(&mut self);
}

impl_fn_pins!(Usart3CtsPin => ID, configure_cts, PinFn::Usart3Cts, into_input;
    (A, 0, 3),
    (B, 1, 6),
    (B, 1, 18),
);

/// Pins that can carry the USART3 RTS signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart3RtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rts(&mut self);
}

impl_fn_pins!(Usart3RtsPin => ID, configure_rts, PinFn::Usart3Rts, into_alt_push_pull;
    (A, 0, 4),
    (B, 1, 7),
    (B, 1, 19),
);

// ============================================================================
// USART4
// ============================================================================

/// Pins that can carry the USART4 TX signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart4TxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_tx(&mut self);
}

impl_fn_pins!(Usart4TxPin => ID, configure_tx, PinFn::Usart4Tx, into_alt_push_pull;
    (A, 0, 5),
    (B, 1, 0),
    (B, 1, 9),
    (B, 1, 13),
    (C, 2, 16),
    (C, 2, 17),
);

/// Pins that can carry the USART4 RX signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart4RxPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rx(&mut self);
}

impl_fn_pins!(Usart4RxPin => ID, configure_rx, PinFn::Usart4Rx, into_input;
    (A, 0, 9),
    (A, 0, 10),
    (B, 1, 1),
    (C, 2, 16),
    (C, 2, 17),
    (C, 2, 19),
);

/// Pins that can carry the USART4 CK signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart4CkPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ck(&mut self);
}

impl_fn_pins!(Usart4CkPin => ID, configure_ck, PinFn::Usart4Ck, into_alt_push_pull;
    (A, 0, 6),
    (A, 0, 8),
    (B, 1, 2),
    (B, 1, 8),
);

/// Pins that can carry the USART4 CTS signal.
///
/// The implementation configures the pad as a floating input.
pub trait Usart4CtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_cts(&mut self);
}

impl_fn_pins!(Usart4CtsPin => ID, configure_cts, PinFn::Usart4Cts, into_input;
    (A, 0, 5),
    (A, 0, 7),
    (A, 0, 14),
    (B, 1, 15),
);

/// Pins that can carry the USART4 RTS signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Usart4RtsPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_rts(&mut self);
}

impl_fn_pins!(Usart4RtsPin => ID, configure_rts, PinFn::Usart4Rts, into_alt_push_pull;
    (A, 0, 6),
    (A, 0, 8),
    (A, 0, 13),
    (B, 1, 21),
);

// ============================================================================
// I2C1
// ============================================================================

/// Pins that can carry the I2C1 SCL signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait I2c1SclPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_scl(&mut self);
}

impl_fn_pins!(I2c1SclPin => ID, configure_scl, PinFn::I2c1Scl, into_alt_push_pull;
    (A, 0, 10),
    (A, 0, 13),
    (C, 2, 16),
    (C, 2, 17),
    (C, 2, 18),
    (C, 2, 19),
);

/// Pins that can carry the I2C1 SDA signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait I2c1SdaPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_sda(&mut self);
}

impl_fn_pins!(I2c1SdaPin => ID, configure_sda, PinFn::I2c1Sda, into_alt_push_pull;
    (A, 0, 11),
    (A, 0, 14),
    (C, 2, 16),
    (C, 2, 17),
    (C, 2, 18),
    (C, 2, 19),
);

// ============================================================================
// SPI1
// ============================================================================

/// Pins that can carry the SPI1 NSS signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Spi1NssPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_nss(&mut self);
}

impl_fn_pins!(Spi1NssPin => ID, configure_nss, PinFn::Spi1Nss, into_alt_push_pull;
    (A, 0, 4),
    (A, 0, 12),
    (B, 1, 21),
    (C, 2, 4),
);

/// Pins that can carry the SPI1 SCK signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Spi1SckPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_sck(&mut self);
}

impl_fn_pins!(Spi1SckPin => ID, configure_sck, PinFn::Spi1Sck, into_alt_push_pull;
    (A, 0, 5),
    (A, 0, 11),
    (B, 1, 15),
    (C, 2, 5),
);

/// Pins that can carry the SPI1 Miso signal.
///
/// The implementation configures the pad as a floating input.
pub trait Spi1MisoPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_miso(&mut self);
}

impl_fn_pins!(Spi1MisoPin => ID, configure_miso, PinFn::Spi1Miso, into_input;
    (A, 0, 6),
    (A, 0, 8),
    (A, 0, 9),
    (C, 2, 6),
);

/// Pins that can carry the SPI1 Mosi signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Spi1MosiPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_mosi(&mut self);
}

impl_fn_pins!(Spi1MosiPin => ID, configure_mosi, PinFn::Spi1Mosi, into_alt_push_pull;
    (A, 0, 7),
    (A, 0, 9),
    (A, 0, 10),
    (C, 2, 7),
);

// ============================================================================
// TIM1
// ============================================================================

/// Pins that can carry the TIM1 CH1 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch1Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch1(&mut self);
}

impl_fn_pins!(Tim1Ch1Pin => ID, configure_ch1, PinFn::Tim1Ch1, into_alt_push_pull;
    (B, 1, 9),
    (C, 2, 0),
);

/// Pins that can carry the TIM1 CH2 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch2Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch2(&mut self);
}

impl_fn_pins!(Tim1Ch2Pin => ID, configure_ch2, PinFn::Tim1Ch2, into_alt_push_pull;
    (B, 1, 10),
    (C, 2, 1),
);

/// Pins that can carry the TIM1 CH3 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch3Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch3(&mut self);
}

impl_fn_pins!(Tim1Ch3Pin => ID, configure_ch3, PinFn::Tim1Ch3, into_alt_push_pull;
    (B, 1, 11),
    (C, 2, 2),
    (C, 2, 14),
);

/// Pins that can carry the TIM1 CH4 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch4Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch4(&mut self);
}

impl_fn_pins!(Tim1Ch4Pin => ID, configure_ch4, PinFn::Tim1Ch4, into_alt_push_pull;
    (B, 1, 12),
    (C, 2, 3),
    (C, 2, 16),
);

/// Pins that can carry the TIM1 Ch1N signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch1NPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch1n(&mut self);
}

impl_fn_pins!(Tim1Ch1NPin => ID, configure_ch1n, PinFn::Tim1Ch1N, into_alt_push_pull;
    (A, 0, 7),
    (B, 1, 6),
    (C, 2, 5),
);

/// Pins that can carry the TIM1 Ch2N signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch2NPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch2n(&mut self);
}

impl_fn_pins!(Tim1Ch2NPin => ID, configure_ch2n, PinFn::Tim1Ch2N, into_alt_push_pull;
    (B, 1, 0),
    (B, 1, 7),
    (C, 2, 6),
);

/// Pins that can carry the TIM1 Ch3N signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim1Ch3NPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch3n(&mut self);
}

impl_fn_pins!(Tim1Ch3NPin => ID, configure_ch3n, PinFn::Tim1Ch3N, into_alt_push_pull;
    (B, 1, 1),
    (B, 1, 8),
    (C, 2, 7),
);

/// Pins that can carry the TIM1 ETR signal.
///
/// The implementation configures the pad as a floating input.
pub trait Tim1EtrPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_etr(&mut self);
}

impl_fn_pins!(Tim1EtrPin => ID, configure_etr, PinFn::Tim1Etr, into_input;
    (C, 2, 15),
    (C, 2, 17),
    (C, 2, 18),
);

/// Pins that can carry the TIM1 Bkin signal.
///
/// The implementation configures the pad as a floating input.
pub trait Tim1BkinPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_bkin(&mut self);
}

impl_fn_pins!(Tim1BkinPin => ID, configure_bkin, PinFn::Tim1Bkin, into_input;
    (A, 0, 6),
    (B, 1, 5),
    (C, 2, 4),
);

// ============================================================================
// TIM2
// ============================================================================

/// Pins that can carry the TIM2 CH1 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch1Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch1(&mut self);
}

impl_fn_pins!(Tim2Ch1Pin => ID, configure_ch1, PinFn::Tim2Ch1, into_alt_push_pull;
    (A, 0, 0),
    (B, 1, 16),
    (B, 1, 21),
    (C, 2, 19),
);

/// Pins that can carry the TIM2 CH2 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch2Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch2(&mut self);
}

impl_fn_pins!(Tim2Ch2Pin => ID, configure_ch2, PinFn::Tim2Ch2, into_alt_push_pull;
    (A, 0, 1),
    (A, 0, 12),
    (B, 1, 15),
    (B, 1, 17),
    (C, 2, 14),
);

/// Pins that can carry the TIM2 CH3 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch3Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch3(&mut self);
}

impl_fn_pins!(Tim2Ch3Pin => ID, configure_ch3, PinFn::Tim2Ch3, into_alt_push_pull;
    (A, 0, 2),
    (A, 0, 13),
    (B, 1, 3),
    (B, 1, 18),
    (C, 2, 15),
);

/// Pins that can carry the TIM2 CH4 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch4Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch4(&mut self);
}

impl_fn_pins!(Tim2Ch4Pin => ID, configure_ch4, PinFn::Tim2Ch4, into_alt_push_pull;
    (A, 0, 3),
    (B, 1, 4),
    (B, 1, 19),
    (C, 2, 0),
);

/// Pins that can carry the TIM2 Ch1N signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch1NPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch1n(&mut self);
}

impl_fn_pins!(Tim2Ch1NPin => ID, configure_ch1n, PinFn::Tim2Ch1N, into_alt_push_pull;
    (A, 0, 12),
    (A, 0, 21),
    (B, 1, 11),
    (C, 2, 1),
    (C, 2, 3),
    (C, 2, 18),
);

/// Pins that can carry the TIM2 Ch2N signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch2NPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch2n(&mut self);
}

impl_fn_pins!(Tim2Ch2NPin => ID, configure_ch2n, PinFn::Tim2Ch2N, into_alt_push_pull;
    (A, 0, 13),
    (A, 0, 22),
    (B, 1, 12),
    (C, 2, 2),
);

/// Pins that can carry the TIM2 Ch3N signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim2Ch3NPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch3n(&mut self);
}

impl_fn_pins!(Tim2Ch3NPin => ID, configure_ch3n, PinFn::Tim2Ch3N, into_alt_push_pull;
    (A, 0, 14),
    (A, 0, 23),
    (B, 1, 3),
    (C, 2, 3),
);

/// Pins that can carry the TIM2 ETR signal.
///
/// The implementation configures the pad as a floating input.
pub trait Tim2EtrPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_etr(&mut self);
}

impl_fn_pins!(Tim2EtrPin => ID, configure_etr, PinFn::Tim2Etr, into_input;
    (A, 0, 2),
    (A, 0, 18),
    (A, 0, 19),
    (C, 2, 4),
);

/// Pins that can carry the TIM2 Bkin signal.
///
/// The implementation configures the pad as a floating input.
pub trait Tim2BkinPin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_bkin(&mut self);
}

impl_fn_pins!(Tim2BkinPin => ID, configure_bkin, PinFn::Tim2Bkin, into_input;
    (A, 0, 9),
    (A, 0, 20),
    (B, 1, 4),
    (C, 2, 0),
);

// ============================================================================
// TIM3
// ============================================================================

/// Pins that can carry the TIM3 CH1 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim3Ch1Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch1(&mut self);
}

impl_fn_pins!(Tim3Ch1Pin => ID, configure_ch1, PinFn::Tim3Ch1, into_alt_push_pull;
    (A, 0, 3),
    (A, 0, 6),
    (B, 1, 4),
    (C, 2, 19),
);

/// Pins that can carry the TIM3 CH2 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait Tim3Ch2Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_ch2(&mut self);
}

impl_fn_pins!(Tim3Ch2Pin => ID, configure_ch2, PinFn::Tim3Ch2, into_alt_push_pull;
    (A, 0, 4),
    (A, 0, 7),
    (B, 1, 5),
    (C, 2, 18),
);

// ============================================================================
// PIOC
// ============================================================================

/// Pins that can carry the PIOC IO0 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait PiocIo0Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_io0(&mut self);
}

impl_fn_pins!(PiocIo0Pin => ID, configure_io0, PinFn::PiocIo0, into_alt_push_pull;
    (C, 2, 7),
    (C, 2, 18),
);

/// Pins that can carry the PIOC IO1 signal.
///
/// The implementation configures the pad as an alternate-function
/// push-pull output.
pub trait PiocIo1Pin {
    /// Pin identifier.
    const ID: PinId;

    /// Configure the pad for this function.
    fn configure_io1(&mut self);
}

impl_fn_pins!(PiocIo1Pin => ID, configure_io1, PinFn::PiocIo1, into_alt_push_pull;
    (C, 2, 19),
);

// ============================================================================
// ADC
// ============================================================================

/// Pins that are ADC inputs.
///
/// The implementation configures the pad as an analog input and exposes the
/// channel number, so [`crate::adc::Adc::read_pin`] needs no hand-written
/// channel constant. The touch-key controller samples the same 14 channels
/// (RM chapter 11), so a pad listed here is also a usable touch channel.
pub trait AdcPin {
    /// Pin identifier.
    const ID: PinId;

    /// ADC channel sampled on this pin.
    const CHANNEL: u8;

    /// Configure the pad as an analog input.
    fn configure_adc_pin(&mut self);
}

impl_adc_pins!(
    (A, 0, 0, 0),
    (A, 0, 1, 1),
    (A, 0, 2, 2),
    (A, 0, 3, 3),
    (A, 0, 4, 4),
    (A, 0, 5, 5),
    (A, 0, 6, 6),
    (A, 0, 7, 7),
    (B, 1, 0, 8),
    (B, 1, 1, 9),
    (C, 2, 0, 10),
    (C, 2, 1, 11),
    (C, 2, 2, 12),
    (C, 2, 3, 13),
);

// ============================================================================
// OPA / CMP analog inputs
// ============================================================================

/// Pins that can be OPA1's non-inverting input (`PSEL` 0..=2).
pub trait Opa1PosPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_opa1_pos(&mut self);
}

impl_analog_select_pins!(Opa1PosPin => ID, SELECT, configure_opa1_pos;
    (B, 1, 0, 0, Opa1P0),
    (B, 1, 8, 1, Opa1P1),
    (B, 1, 4, 2, Opa1P2),
);

/// Pins that can be OPA1's inverting input (`NSEL` 0..=2).
pub trait Opa1NegPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_opa1_neg(&mut self);
}

impl_analog_select_pins!(Opa1NegPin => ID, SELECT, configure_opa1_neg;
    (A, 0, 6, 0, Opa1N0),
    (B, 1, 6, 1, Opa1N1),
    (A, 0, 0, 2, Opa1N2),
);

#[cfg(feature = "__opa2")]
/// Pins that can be OPA2's non-inverting input (`PSEL` 0..=2).
pub trait Opa2PosPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_opa2_pos(&mut self);
}

#[cfg(feature = "__opa2")]
impl_analog_select_pins!(Opa2PosPin => ID, SELECT, configure_opa2_pos;
    (A, 0, 7, 0, Opa2P0),
    (B, 1, 3, 1, Opa2P1),
    (B, 1, 7, 2, Opa2P2),
);

#[cfg(feature = "__opa2")]
/// Pins that can be OPA2's inverting input (`NSEL` 0..=2).
pub trait Opa2NegPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_opa2_neg(&mut self);
}

#[cfg(feature = "__opa2")]
impl_analog_select_pins!(Opa2NegPin => ID, SELECT, configure_opa2_neg;
    (A, 0, 5, 0, Opa2N0),
    (B, 1, 1, 1, Opa2N1),
    (A, 0, 1, 2, Opa2N2),
);

#[cfg(feature = "__cmp1")]
/// Pins that can be CMP1's non-inverting input (`PSEL` 0/1).
pub trait Cmp1PosPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_cmp1_pos(&mut self);
}

#[cfg(feature = "__cmp1")]
impl_analog_select_pins!(Cmp1PosPin => ID, SELECT, configure_cmp1_pos;
    (C, 2, 19, 0, Cmp1P0),
    (A, 0, 0, 1, Cmp1P1),
);

#[cfg(feature = "__cmp1")]
/// Pins that can be CMP1's inverting input (`NSEL` 0/1).
pub trait Cmp1NegPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_cmp1_neg(&mut self);
}

#[cfg(feature = "__cmp1")]
impl_analog_select_pins!(Cmp1NegPin => ID, SELECT, configure_cmp1_neg;
    (C, 2, 3, 0, Cmp1N0),
    (A, 0, 23, 1, Cmp1N1),
);

#[cfg(feature = "__cmp2")]
/// Pins that can be CMP2's non-inverting input (`PSEL` 0/1).
pub trait Cmp2PosPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_cmp2_pos(&mut self);
}

#[cfg(feature = "__cmp2")]
impl_analog_select_pins!(Cmp2PosPin => ID, SELECT, configure_cmp2_pos;
    (A, 0, 12, 0, Cmp2P0),
    (A, 0, 11, 1, Cmp2P1),
);

#[cfg(feature = "__cmp2")]
/// Pins that can be CMP2's inverting input (`NSEL` 0/1).
pub trait Cmp2NegPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_cmp2_neg(&mut self);
}

#[cfg(feature = "__cmp2")]
impl_analog_select_pins!(Cmp2NegPin => ID, SELECT, configure_cmp2_neg;
    (A, 0, 22, 0, Cmp2N0),
    (C, 2, 3, 1, Cmp2N1),
);

#[cfg(feature = "__cmp3")]
/// Pins that can be CMP3's non-inverting input (`PSEL` 0/1).
pub trait Cmp3PosPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_cmp3_pos(&mut self);
}

#[cfg(feature = "__cmp3")]
impl_analog_select_pins!(Cmp3PosPin => ID, SELECT, configure_cmp3_pos;
    (A, 0, 13, 0, Cmp3P0),
    (A, 0, 14, 1, Cmp3P1),
);

#[cfg(feature = "__cmp3")]
/// Pins that can be CMP3's inverting input (`NSEL` 0/1).
pub trait Cmp3NegPin {
    /// Pin identifier.
    const ID: PinId;

    /// Input-select code this pad corresponds to (`psel`/`nsel` in
    /// [`crate::opa::OpampConfig`], `psel`/`nsel` as `bool` in
    /// [`crate::opa::ComparatorConfig`]).
    const SELECT: u8;

    /// Configure the pad as an analog input.
    fn configure_cmp3_neg(&mut self);
}

#[cfg(feature = "__cmp3")]
impl_analog_select_pins!(Cmp3NegPin => ID, SELECT, configure_cmp3_neg;
    (A, 0, 2, 0, Cmp3N0),
    (C, 2, 3, 1, Cmp3N1),
);

// ============================================================================
// Bus remap helpers
// ============================================================================

/// Find the `USART1_RM` value that serves both pins.
pub fn usart1_remap<T: Usart1TxPin, R: Usart1RxPin>(_tx: &T, _rx: &R) -> Option<u8> {
    common_remap(
        crate::pin::remap_mask(T::ID.port, T::ID.pin, PinFn::Usart1Tx),
        crate::pin::remap_mask(R::ID.port, R::ID.pin, PinFn::Usart1Rx),
    )
}

/// Find the `USART2_RM` value that serves both pins.
pub fn usart2_remap<T: Usart2TxPin, R: Usart2RxPin>(_tx: &T, _rx: &R) -> Option<u8> {
    common_remap(
        crate::pin::remap_mask(T::ID.port, T::ID.pin, PinFn::Usart2Tx),
        crate::pin::remap_mask(R::ID.port, R::ID.pin, PinFn::Usart2Rx),
    )
}

/// Find the `USART3_RM` value that serves both pins.
pub fn usart3_remap<T: Usart3TxPin, R: Usart3RxPin>(_tx: &T, _rx: &R) -> Option<u8> {
    common_remap(
        crate::pin::remap_mask(T::ID.port, T::ID.pin, PinFn::Usart3Tx),
        crate::pin::remap_mask(R::ID.port, R::ID.pin, PinFn::Usart3Rx),
    )
}

/// Find the `USART4_RM` value that serves both pins.
pub fn usart4_remap<T: Usart4TxPin, R: Usart4RxPin>(_tx: &T, _rx: &R) -> Option<u8> {
    common_remap(
        crate::pin::remap_mask(T::ID.port, T::ID.pin, PinFn::Usart4Tx),
        crate::pin::remap_mask(R::ID.port, R::ID.pin, PinFn::Usart4Rx),
    )
}

/// Find the `I2C1_RM` value that serves both pins.
pub fn i2c1_remap<C: I2c1SclPin, D: I2c1SdaPin>(_scl: &C, _sda: &D) -> Option<u8> {
    common_remap(
        crate::pin::remap_mask(C::ID.port, C::ID.pin, PinFn::I2c1Scl),
        crate::pin::remap_mask(D::ID.port, D::ID.pin, PinFn::I2c1Sda),
    )
}

/// Find the `SPI1_RM` value that serves SCK, MISO and MOSI together.
///
/// NSS is not included: the SPI driver uses software slave management
/// (`CTLR1.SSM`), so the chip-select pad is free for GPIO use.
pub fn spi1_remap<S: Spi1SckPin, M: Spi1MisoPin, O: Spi1MosiPin>(
    _sck: &S,
    _miso: &M,
    _mosi: &O,
) -> Option<u8> {
    common_remap(
        crate::pin::remap_mask(S::ID.port, S::ID.pin, PinFn::Spi1Sck)
            & crate::pin::remap_mask(M::ID.port, M::ID.pin, PinFn::Spi1Miso),
        crate::pin::remap_mask(O::ID.port, O::ID.pin, PinFn::Spi1Mosi),
    )
}

/// Lowest remap code present in both masks.
fn common_remap(a: u8, b: u8) -> Option<u8> {
    let common = a & b;
    if common == 0 {
        None
    } else {
        Some(common.trailing_zeros() as u8)
    }
}
