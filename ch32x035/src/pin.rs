//! Per-pin function description for the CH32X035.
//!
//! This module is the single source of truth for **what each pin can do** and
//! **which package bonds it out**. [`crate::pins`] turns the digital function
//! data into traits, so a peripheral can only be handed a pin that actually
//! carries the requested signal.
//!
//! Unlike the rp2350's `funcsel` scheme, the CH32X035 selects alternate
//! functions through the `AFIO_PCFR1` remap fields (`USARTn_RM`, `I2C1_RM`,
//! `SPI1_RM`, `TIMn_RM`, `PIOC_RM`): each remap value maps a whole signal
//! group to a fixed set of pins. The tables below are transcribed from the
//! reference manual chapter 8.3.2.1 and the datasheet tables 2-1 and 2-3.
//!
//! # Digital functions (remap tables)
//!
//! ## USART1 (`USART1_RM`, RM 8.3.2.1)
//!
//! | `USART1_RM` | TX | RX | CK | CTS | RTS |
//! |---|---|---|---|---|---|
//! | 00 | PB10 | PB11 | PB9 | PC16 | PC17 |
//! | 01 | PA10 | PA11 | PB9 | PC16 | PC17 |
//! | 10 | PB10 | PB11 | PB5 | PA9 | PA8 |
//! | 11 | PA7 | PB2 | PB12 | PA13 | PA14 |
//!
//! ## USART2 (`USART2_RM`, RM 8.3.2.1)
//!
//! | `USART2_RM` | TX | RX | CK | CTS | RTS |
//! |---|---|---|---|---|---|
//! | 000 | PA2 | PA3 | PA4 | PA0 | PA1 |
//! | 001 | PA20 | PA19 | PA23 | PA1 | PA2 |
//! | 010 | PA15 | PA16 | PA22 | PA17 | PA21 |
//! | 011 | PC0 | PC1 | PB20 | PC2 | PC3 |
//! | 100 | PA15 | PA16 | PA22 | PA17 | PC3 |
//! | 101 | PA15 | PA16 | PA22 | PA17 | PC3 |
//! | 110 | PA15 | PA16 | PA22 | PA17 | PC3 |
//! | 111 | PA15 | PA16 | PA22 | PA17 | PC3 |
//!
//! ## USART3 (`USART3_RM`, RM 8.3.2.1)
//!
//! | `USART3_RM` | TX | RX | CK | CTS | RTS |
//! |---|---|---|---|---|---|
//! | 00 | PB3 | PB4 | PB5 | PB6 | PB7 |
//! | 01 | PC18 | PC19 | PB5 | PB6 | PB7 |
//! | 10 | PA18 | PB14 | PB8 | PA3 | PA4 |
//! | 11 | PB16 | PB17 | — | PB18 | PB19 |
//!
//! ## USART4 (`USART4_RM`, RM 8.3.2.1)
//!
//! | `USART4_RM` | TX | RX | CK | CTS | RTS |
//! |---|---|---|---|---|---|
//! | 000 | PB0 | PB1 | PB2 | PB15 | PA8 |
//! | 001 | PA5 | PA9 | PA6 | PA7 | PB21 |
//! | 010 | PC16 | PC17 | PB2 | PB15 | PA8 |
//! | 011 | PB9 | PA10 | PB8 | PA14 | PA13 |
//! | 100 | PB13 | PC19 | PA8 | PA5 | PA6 |
//! | 101 | PC17 | PC16 | PB2 | PB15 | PA8 |
//! | 110 | PB13 | PC19 | PA8 | PA5 | PA6 |
//! | 111 | PC17 | PC16 | PB2 | PB15 | PA8 |
//!
//! ## I2C1 (`I2C1_RM`)
//!
//! | `I2C1_RM` | SCL | SDA |
//! |---|---|---|
//! | 000 | PA10 | PA11 |
//! | 001 | PA13 | PA14 |
//! | 010 | PC16 | PC17 |
//! | 011 | PC19 | PC18 |
//! | 100 | PC17 | PC16 |
//! | 101 | PC18 | PC19 |
//! | 110 | PC17 | PC16 |
//! | 111 | PC18 | PC19 |
//!
//! ## SPI1 (`SPI1_RM`)
//!
//! | `SPI1_RM` | NSS | SCK | MISO | MOSI |
//! |---|---|---|---|---|
//! | 00 | PA4 | PA5 | PA6 | PA7 |
//! | 01 | PB21 | PB15 | PA8 | PA9 |
//! | 10 | PA12 | PA11 | PA9 | PA10 |
//! | 11 | PC4 | PC5 | PC6 | PC7 |
//!
//! ## TIM1 (`TIM1_RM`)
//!
//! | `TIM1_RM` | CH1 | CH2 | CH3 | CH4 | CH1N | CH2N | CH3N | ETR | BKIN |
//! |---|---|---|---|---|---|---|---|---|---|
//! | 000 | PB9 | PB10 | PB11 | PC16 | PB6 | PB7 | PB8 | PC17 | PB5 |
//! | 001 | PB9 | PB10 | PB11 | PC16 | PA7 | PB0 | PB1 | PC17 | PA6 |
//! | 010 | PB9 | PB10 | PB11 | PB12 | PB6 | PB7 | PB8 | PC18 | PB5 |
//! | 011 | PC0 | PC1 | PC2 | PC3 | PC5 | PC6 | PC7 | PC18 | PC4 |
//! | 100 | — | — | PC14 | — | — | — | — | PC15 | — |
//! | 101 | — | — | PC14 | — | — | — | — | PC15 | — |
//! | 110 | — | — | PC14 | — | — | — | — | PC15 | — |
//! | 111 | — | — | PC14 | — | — | — | — | PC15 | — |
//!
//! ## TIM2 (`TIM2_RM`)
//!
//! | `TIM2_RM` | CH1 | CH2 | CH3 | CH4 | CH1N | CH2N | CH3N | ETR | BKIN |
//! |---|---|---|---|---|---|---|---|---|---|
//! | 000 | PA0 | PA1 | PA2 | PA3 | PA21 | PA22 | PA23 | PA19 | PA20 |
//! | 001 | PB21 | PB15 | PA2 | PA3 | PA12 | PA13 | PA14 | PA18 | PA9 |
//! | 010 | PA0 | PA1 | PB3 | PB4 | PC3 | PA22 | PA23 | PA19 | PA20 |
//! | 011 | PB21 | PB15 | PB3 | PB4 | PA12 | PA13 | PA14 | PA18 | PA9 |
//! | 100 | PB16 | PB17 | PB18 | PB19 | PC1 | PC2 | PC3 | PC4 | PC0 |
//! | 101 | PC19 | PA12 | PA13 | PC0 | PC18 | PB12 | PB3 | PA2 | PB4 |
//! | 110 | PC19 | PC14 | PC15 | PC0 | PB11 | PB12 | PB3 | PA2 | PB4 |
//! | 111 | PC19 | PC14 | PC15 | PC0 | PB11 | PB12 | PB3 | PA2 | PB4 |
//!
//! ## TIM3 (`TIM3_RM`)
//!
//! | `TIM3_RM` | CH1 | CH2 |
//! |---|---|---|
//! | 00 | PA6 | PA7 |
//! | 01 | PB4 | PB5 |
//! | 10 | PC19 | PC18 |
//! | 11 | PA3 | PA4 |
//!
//! ## PIOC (`PIOC_RM`)
//!
//! | `PIOC_RM` | IO0 | IO1 |
//! |---|---|---|
//! | 0 | PC18 | PC19 |
//! | 1 | PC7 | PC19 |
//!
//! # Analog functions
//!
//! The ADC channel of a pin is listed in the table below; the OPA/CMP/USB/PD
//! functions are listed per pin in [`AnalogRole`]. The touch-key controller
//! has no pins of its own: it converts the 14 external ADC channels through
//! the ADC (RM chapter 11).
//!
//! | IN0 | IN1 | IN2 | IN3 | IN4 | IN5 | IN6 |
//! |---|---|---|---|---|---|---|
//! | PA0 | PA1 | PA2 | PA3 | PA4 | PA5 | PA6 |
//! | IN7 | IN8 | IN9 | IN10 | IN11 | IN12 | IN13 |
//! |---|---|---|---|---|---|---|
//! | PA7 | PB0 | PB1 | PC0 | PC1 | PC2 | PC3 |
//!
//! # Packages
//!
//! Seven models bond out different subsets of the pins; see [`Package`] and
//! [`packages`]. Port sizes are A0..A23, B0..B21 and C0..C19 (PC8..PC13 and
//! PC20..PC23 exist on no package).
//!
//! | Model | Package | I/O pads | ADC | Notes |
//! |-------|---------|----------|-----|-------|
//! | R8T6  | LQFP64M | 60       | 14  | |
//! | C8T6  | LQFP48  | 46       | 10  | |
//! | G8U6  | QFN28   | 27       | 12  | PB1/PB5 share pad 16 |
//! | G8R6  | QSOP28  | 26       | 11  | PB1/PB5 pad 20, PA12/PC14 pad 4, PA13/PC15 pad 5 |
//! | F8U6  | QFN20   | 19       | 10  | |
//! | F7P6  | TSSOP20 | 18       | 11  | |
//! | D8U6  | QFN12   | 11       | 5   | PA7/PC7 share pad 6 |
//!
//! The I/O and ADC columns are the datasheet figures; the assertions at the
//! end of this module recompute them from the bonding table, so the two
//! cannot drift. Note that four packages bond *two logical pins onto one
//! physical pad*, see [`SHARED`].
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::pin::{self, PinFn, Port};
//!
//! // Compile-time query
//! const PA9_IS_USART4_RX: bool = pin::supports(Port::A, 9, PinFn::Usart4Rx);
//!
//! // Runtime query
//! assert_eq!(pin::remap(Port::B, 10, PinFn::Usart1Tx), Some(0));
//! assert_eq!(pin::adc_channel(Port::B, 0), Some(8));
//! ```

// ============================================================================
// Ports and pin identifiers
// ============================================================================

/// GPIO port. The CH32X035 has ports A, B and C (there is no port D).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Port {
    /// GPIOA
    A,
    /// GPIOB
    B,
    /// GPIOC
    C,
}

impl Port {
    /// All ports, in register-map order.
    pub const ALL: [Port; 3] = [Port::A, Port::B, Port::C];

    /// Convert from the `Pin<PORT, N>` port number (0 = A, 1 = B, 2 = C).
    pub const fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Port::A),
            1 => Some(Port::B),
            2 => Some(Port::C),
            _ => None,
        }
    }

    /// The `Pin<PORT, N>` port number for this port.
    pub const fn index(self) -> u32 {
        self as u32
    }

    /// Port name as used in the datasheet (`A`, `B` or `C`).
    pub const fn name(self) -> char {
        match self {
            Port::A => 'A',
            Port::B => 'B',
            Port::C => 'C',
        }
    }
}

/// A pin identifier: `(port, pin number)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PinId {
    /// GPIO port.
    pub port: Port,
    /// Pin number within the port (0..=23).
    pub pin: u8,
}

impl PinId {
    /// Create a pin identifier.
    #[inline]
    pub const fn new(port: Port, pin: u8) -> Self {
        Self { port, pin }
    }
}

// ============================================================================
// Digital functions
// ============================================================================

/// A digital alternate function, selected through `AFIO_PCFR1`.
///
/// The value returned by [`remap`] is the `xxx_RM` field value that routes the
/// function to the pin.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PinFn {
    /// USART1 transmit
    Usart1Tx,
    /// USART1 receive
    Usart1Rx,
    /// USART1 clock
    Usart1Ck,
    /// USART1 clear-to-send input
    Usart1Cts,
    /// USART1 request-to-send output
    Usart1Rts,
    /// USART2 transmit
    Usart2Tx,
    /// USART2 receive
    Usart2Rx,
    /// USART2 clock
    Usart2Ck,
    /// USART2 clear-to-send input
    Usart2Cts,
    /// USART2 request-to-send output
    Usart2Rts,
    /// USART3 transmit
    Usart3Tx,
    /// USART3 receive
    Usart3Rx,
    /// USART3 clock
    Usart3Ck,
    /// USART3 clear-to-send input
    Usart3Cts,
    /// USART3 request-to-send output
    Usart3Rts,
    /// USART4 transmit
    Usart4Tx,
    /// USART4 receive
    Usart4Rx,
    /// USART4 clock
    Usart4Ck,
    /// USART4 clear-to-send input
    Usart4Cts,
    /// USART4 request-to-send output
    Usart4Rts,
    /// I2C1 clock
    I2c1Scl,
    /// I2C1 data
    I2c1Sda,
    /// SPI1 chip select
    Spi1Nss,
    /// SPI1 clock
    Spi1Sck,
    /// SPI1 MISO
    Spi1Miso,
    /// SPI1 MOSI
    Spi1Mosi,
    /// TIM1 channel 1
    Tim1Ch1,
    /// TIM1 channel 2
    Tim1Ch2,
    /// TIM1 channel 3
    Tim1Ch3,
    /// TIM1 channel 4
    Tim1Ch4,
    /// TIM1 complementary channel 1
    Tim1Ch1N,
    /// TIM1 complementary channel 2
    Tim1Ch2N,
    /// TIM1 complementary channel 3
    Tim1Ch3N,
    /// TIM1 external trigger
    Tim1Etr,
    /// TIM1 break input
    Tim1Bkin,
    /// TIM2 channel 1
    Tim2Ch1,
    /// TIM2 channel 2
    Tim2Ch2,
    /// TIM2 channel 3
    Tim2Ch3,
    /// TIM2 channel 4
    Tim2Ch4,
    /// TIM2 complementary channel 1
    Tim2Ch1N,
    /// TIM2 complementary channel 2
    Tim2Ch2N,
    /// TIM2 complementary channel 3
    Tim2Ch3N,
    /// TIM2 external trigger
    Tim2Etr,
    /// TIM2 break input
    Tim2Bkin,
    /// TIM3 channel 1
    Tim3Ch1,
    /// TIM3 channel 2
    Tim3Ch2,
    /// PIOC single-wire interface 0
    PiocIo0,
    /// PIOC single-wire interface 1
    PiocIo1,
}

/// One `(function, remap code, pin)` entry from the reference-manual remap
/// tables.
#[derive(Clone, Copy, Debug)]
pub struct Route {
    /// The signal.
    pub func: PinFn,
    /// The `xxx_RM` field value selecting this routing.
    pub remap: u8,
    /// The pad the signal appears on.
    pub pin: PinId,
}

const fn route(func: PinFn, remap: u8, port: Port, pin: u8) -> Route {
    Route {
        func,
        remap,
        pin: PinId::new(port, pin),
    }
}

/// Every digital route of the CH32X035, transcribed from CH32X035RM 8.3.2.1.
///
/// Wildcard encodings of the manual (`1xx`, `11x`, `1x0`, `1x1`) are expanded
/// into one entry per selectable code. Signals the manual marks as
/// unavailable in a given remap (e.g. `USART3_RM = 11` has no CK) are absent.
pub const ROUTES: &[Route] = &[
    // ---- USART1 ----
    route(PinFn::Usart1Tx, 0, Port::B, 10),
    route(PinFn::Usart1Rx, 0, Port::B, 11),
    route(PinFn::Usart1Ck, 0, Port::B, 9),
    route(PinFn::Usart1Cts, 0, Port::C, 16),
    route(PinFn::Usart1Rts, 0, Port::C, 17),
    route(PinFn::Usart1Tx, 1, Port::A, 10),
    route(PinFn::Usart1Rx, 1, Port::A, 11),
    route(PinFn::Usart1Ck, 1, Port::B, 9),
    route(PinFn::Usart1Cts, 1, Port::C, 16),
    route(PinFn::Usart1Rts, 1, Port::C, 17),
    route(PinFn::Usart1Tx, 2, Port::B, 10),
    route(PinFn::Usart1Rx, 2, Port::B, 11),
    route(PinFn::Usart1Ck, 2, Port::B, 5),
    route(PinFn::Usart1Cts, 2, Port::A, 9),
    route(PinFn::Usart1Rts, 2, Port::A, 8),
    route(PinFn::Usart1Tx, 3, Port::A, 7),
    route(PinFn::Usart1Rx, 3, Port::B, 2),
    route(PinFn::Usart1Ck, 3, Port::B, 12),
    route(PinFn::Usart1Cts, 3, Port::A, 13),
    route(PinFn::Usart1Rts, 3, Port::A, 14),
    // ---- USART2 ----
    route(PinFn::Usart2Tx, 0, Port::A, 2),
    route(PinFn::Usart2Rx, 0, Port::A, 3),
    route(PinFn::Usart2Ck, 0, Port::A, 4),
    route(PinFn::Usart2Cts, 0, Port::A, 0),
    route(PinFn::Usart2Rts, 0, Port::A, 1),
    route(PinFn::Usart2Tx, 1, Port::A, 20),
    route(PinFn::Usart2Rx, 1, Port::A, 19),
    route(PinFn::Usart2Ck, 1, Port::A, 23),
    route(PinFn::Usart2Cts, 1, Port::A, 1),
    route(PinFn::Usart2Rts, 1, Port::A, 2),
    route(PinFn::Usart2Tx, 2, Port::A, 15),
    route(PinFn::Usart2Rx, 2, Port::A, 16),
    route(PinFn::Usart2Ck, 2, Port::A, 22),
    route(PinFn::Usart2Cts, 2, Port::A, 17),
    route(PinFn::Usart2Rts, 2, Port::A, 21),
    route(PinFn::Usart2Tx, 3, Port::C, 0),
    route(PinFn::Usart2Rx, 3, Port::C, 1),
    route(PinFn::Usart2Ck, 3, Port::B, 20),
    route(PinFn::Usart2Cts, 3, Port::C, 2),
    route(PinFn::Usart2Rts, 3, Port::C, 3),
    route(PinFn::Usart2Tx, 4, Port::A, 15),
    route(PinFn::Usart2Rx, 4, Port::A, 16),
    route(PinFn::Usart2Ck, 4, Port::A, 22),
    route(PinFn::Usart2Cts, 4, Port::A, 17),
    route(PinFn::Usart2Rts, 4, Port::C, 3),
    route(PinFn::Usart2Tx, 5, Port::A, 15),
    route(PinFn::Usart2Rx, 5, Port::A, 16),
    route(PinFn::Usart2Ck, 5, Port::A, 22),
    route(PinFn::Usart2Cts, 5, Port::A, 17),
    route(PinFn::Usart2Rts, 5, Port::C, 3),
    route(PinFn::Usart2Tx, 6, Port::A, 15),
    route(PinFn::Usart2Rx, 6, Port::A, 16),
    route(PinFn::Usart2Ck, 6, Port::A, 22),
    route(PinFn::Usart2Cts, 6, Port::A, 17),
    route(PinFn::Usart2Rts, 6, Port::C, 3),
    route(PinFn::Usart2Tx, 7, Port::A, 15),
    route(PinFn::Usart2Rx, 7, Port::A, 16),
    route(PinFn::Usart2Ck, 7, Port::A, 22),
    route(PinFn::Usart2Cts, 7, Port::A, 17),
    route(PinFn::Usart2Rts, 7, Port::C, 3),
    // ---- USART3 ----
    route(PinFn::Usart3Tx, 0, Port::B, 3),
    route(PinFn::Usart3Rx, 0, Port::B, 4),
    route(PinFn::Usart3Ck, 0, Port::B, 5),
    route(PinFn::Usart3Cts, 0, Port::B, 6),
    route(PinFn::Usart3Rts, 0, Port::B, 7),
    route(PinFn::Usart3Tx, 1, Port::C, 18),
    route(PinFn::Usart3Rx, 1, Port::C, 19),
    route(PinFn::Usart3Ck, 1, Port::B, 5),
    route(PinFn::Usart3Cts, 1, Port::B, 6),
    route(PinFn::Usart3Rts, 1, Port::B, 7),
    route(PinFn::Usart3Tx, 2, Port::A, 18),
    route(PinFn::Usart3Rx, 2, Port::B, 14),
    route(PinFn::Usart3Ck, 2, Port::B, 8),
    route(PinFn::Usart3Cts, 2, Port::A, 3),
    route(PinFn::Usart3Rts, 2, Port::A, 4),
    route(PinFn::Usart3Tx, 3, Port::B, 16),
    route(PinFn::Usart3Rx, 3, Port::B, 17),
    route(PinFn::Usart3Cts, 3, Port::B, 18),
    route(PinFn::Usart3Rts, 3, Port::B, 19),
    // ---- USART4 ----
    route(PinFn::Usart4Tx, 0, Port::B, 0),
    route(PinFn::Usart4Rx, 0, Port::B, 1),
    route(PinFn::Usart4Ck, 0, Port::B, 2),
    route(PinFn::Usart4Cts, 0, Port::B, 15),
    route(PinFn::Usart4Rts, 0, Port::A, 8),
    route(PinFn::Usart4Tx, 1, Port::A, 5),
    route(PinFn::Usart4Rx, 1, Port::A, 9),
    route(PinFn::Usart4Ck, 1, Port::A, 6),
    route(PinFn::Usart4Cts, 1, Port::A, 7),
    route(PinFn::Usart4Rts, 1, Port::B, 21),
    route(PinFn::Usart4Tx, 2, Port::C, 16),
    route(PinFn::Usart4Rx, 2, Port::C, 17),
    route(PinFn::Usart4Ck, 2, Port::B, 2),
    route(PinFn::Usart4Cts, 2, Port::B, 15),
    route(PinFn::Usart4Rts, 2, Port::A, 8),
    route(PinFn::Usart4Tx, 3, Port::B, 9),
    route(PinFn::Usart4Rx, 3, Port::A, 10),
    route(PinFn::Usart4Ck, 3, Port::B, 8),
    route(PinFn::Usart4Cts, 3, Port::A, 14),
    route(PinFn::Usart4Rts, 3, Port::A, 13),
    route(PinFn::Usart4Tx, 4, Port::B, 13),
    route(PinFn::Usart4Rx, 4, Port::C, 19),
    route(PinFn::Usart4Ck, 4, Port::A, 8),
    route(PinFn::Usart4Cts, 4, Port::A, 5),
    route(PinFn::Usart4Rts, 4, Port::A, 6),
    route(PinFn::Usart4Tx, 5, Port::C, 17),
    route(PinFn::Usart4Rx, 5, Port::C, 16),
    route(PinFn::Usart4Ck, 5, Port::B, 2),
    route(PinFn::Usart4Cts, 5, Port::B, 15),
    route(PinFn::Usart4Rts, 5, Port::A, 8),
    route(PinFn::Usart4Tx, 6, Port::B, 13),
    route(PinFn::Usart4Rx, 6, Port::C, 19),
    route(PinFn::Usart4Ck, 6, Port::A, 8),
    route(PinFn::Usart4Cts, 6, Port::A, 5),
    route(PinFn::Usart4Rts, 6, Port::A, 6),
    route(PinFn::Usart4Tx, 7, Port::C, 17),
    route(PinFn::Usart4Rx, 7, Port::C, 16),
    route(PinFn::Usart4Ck, 7, Port::B, 2),
    route(PinFn::Usart4Cts, 7, Port::B, 15),
    route(PinFn::Usart4Rts, 7, Port::A, 8),
    // ---- I2C1 ----
    route(PinFn::I2c1Scl, 0, Port::A, 10),
    route(PinFn::I2c1Sda, 0, Port::A, 11),
    route(PinFn::I2c1Scl, 1, Port::A, 13),
    route(PinFn::I2c1Sda, 1, Port::A, 14),
    route(PinFn::I2c1Scl, 2, Port::C, 16),
    route(PinFn::I2c1Sda, 2, Port::C, 17),
    route(PinFn::I2c1Scl, 3, Port::C, 19),
    route(PinFn::I2c1Sda, 3, Port::C, 18),
    route(PinFn::I2c1Scl, 4, Port::C, 17),
    route(PinFn::I2c1Sda, 4, Port::C, 16),
    route(PinFn::I2c1Scl, 5, Port::C, 18),
    route(PinFn::I2c1Sda, 5, Port::C, 19),
    route(PinFn::I2c1Scl, 6, Port::C, 17),
    route(PinFn::I2c1Sda, 6, Port::C, 16),
    route(PinFn::I2c1Scl, 7, Port::C, 18),
    route(PinFn::I2c1Sda, 7, Port::C, 19),
    // ---- SPI1 ----
    route(PinFn::Spi1Nss, 0, Port::A, 4),
    route(PinFn::Spi1Sck, 0, Port::A, 5),
    route(PinFn::Spi1Miso, 0, Port::A, 6),
    route(PinFn::Spi1Mosi, 0, Port::A, 7),
    route(PinFn::Spi1Nss, 1, Port::B, 21),
    route(PinFn::Spi1Sck, 1, Port::B, 15),
    route(PinFn::Spi1Miso, 1, Port::A, 8),
    route(PinFn::Spi1Mosi, 1, Port::A, 9),
    route(PinFn::Spi1Nss, 2, Port::A, 12),
    route(PinFn::Spi1Sck, 2, Port::A, 11),
    route(PinFn::Spi1Miso, 2, Port::A, 9),
    route(PinFn::Spi1Mosi, 2, Port::A, 10),
    route(PinFn::Spi1Nss, 3, Port::C, 4),
    route(PinFn::Spi1Sck, 3, Port::C, 5),
    route(PinFn::Spi1Miso, 3, Port::C, 6),
    route(PinFn::Spi1Mosi, 3, Port::C, 7),
    // ---- TIM1 ----
    route(PinFn::Tim1Ch1, 0, Port::B, 9),
    route(PinFn::Tim1Ch2, 0, Port::B, 10),
    route(PinFn::Tim1Ch3, 0, Port::B, 11),
    route(PinFn::Tim1Ch4, 0, Port::C, 16),
    route(PinFn::Tim1Ch1N, 0, Port::B, 6),
    route(PinFn::Tim1Ch2N, 0, Port::B, 7),
    route(PinFn::Tim1Ch3N, 0, Port::B, 8),
    route(PinFn::Tim1Etr, 0, Port::C, 17),
    route(PinFn::Tim1Bkin, 0, Port::B, 5),
    route(PinFn::Tim1Ch1, 1, Port::B, 9),
    route(PinFn::Tim1Ch2, 1, Port::B, 10),
    route(PinFn::Tim1Ch3, 1, Port::B, 11),
    route(PinFn::Tim1Ch4, 1, Port::C, 16),
    route(PinFn::Tim1Ch1N, 1, Port::A, 7),
    route(PinFn::Tim1Ch2N, 1, Port::B, 0),
    route(PinFn::Tim1Ch3N, 1, Port::B, 1),
    route(PinFn::Tim1Etr, 1, Port::C, 17),
    route(PinFn::Tim1Bkin, 1, Port::A, 6),
    route(PinFn::Tim1Ch1, 2, Port::B, 9),
    route(PinFn::Tim1Ch2, 2, Port::B, 10),
    route(PinFn::Tim1Ch3, 2, Port::B, 11),
    route(PinFn::Tim1Ch4, 2, Port::B, 12),
    route(PinFn::Tim1Ch1N, 2, Port::B, 6),
    route(PinFn::Tim1Ch2N, 2, Port::B, 7),
    route(PinFn::Tim1Ch3N, 2, Port::B, 8),
    route(PinFn::Tim1Etr, 2, Port::C, 18),
    route(PinFn::Tim1Bkin, 2, Port::B, 5),
    route(PinFn::Tim1Ch1, 3, Port::C, 0),
    route(PinFn::Tim1Ch2, 3, Port::C, 1),
    route(PinFn::Tim1Ch3, 3, Port::C, 2),
    route(PinFn::Tim1Ch4, 3, Port::C, 3),
    route(PinFn::Tim1Ch1N, 3, Port::C, 5),
    route(PinFn::Tim1Ch2N, 3, Port::C, 6),
    route(PinFn::Tim1Ch3N, 3, Port::C, 7),
    route(PinFn::Tim1Etr, 3, Port::C, 18),
    route(PinFn::Tim1Bkin, 3, Port::C, 4),
    route(PinFn::Tim1Ch3, 4, Port::C, 14),
    route(PinFn::Tim1Etr, 4, Port::C, 15),
    route(PinFn::Tim1Ch3, 5, Port::C, 14),
    route(PinFn::Tim1Etr, 5, Port::C, 15),
    route(PinFn::Tim1Ch3, 6, Port::C, 14),
    route(PinFn::Tim1Etr, 6, Port::C, 15),
    route(PinFn::Tim1Ch3, 7, Port::C, 14),
    route(PinFn::Tim1Etr, 7, Port::C, 15),
    // ---- TIM2 ----
    route(PinFn::Tim2Ch1, 0, Port::A, 0),
    route(PinFn::Tim2Ch2, 0, Port::A, 1),
    route(PinFn::Tim2Ch3, 0, Port::A, 2),
    route(PinFn::Tim2Ch4, 0, Port::A, 3),
    route(PinFn::Tim2Ch1N, 0, Port::A, 21),
    route(PinFn::Tim2Ch2N, 0, Port::A, 22),
    route(PinFn::Tim2Ch3N, 0, Port::A, 23),
    route(PinFn::Tim2Etr, 0, Port::A, 19),
    route(PinFn::Tim2Bkin, 0, Port::A, 20),
    route(PinFn::Tim2Ch1, 1, Port::B, 21),
    route(PinFn::Tim2Ch2, 1, Port::B, 15),
    route(PinFn::Tim2Ch3, 1, Port::A, 2),
    route(PinFn::Tim2Ch4, 1, Port::A, 3),
    route(PinFn::Tim2Ch1N, 1, Port::A, 12),
    route(PinFn::Tim2Ch2N, 1, Port::A, 13),
    route(PinFn::Tim2Ch3N, 1, Port::A, 14),
    route(PinFn::Tim2Etr, 1, Port::A, 18),
    route(PinFn::Tim2Bkin, 1, Port::A, 9),
    route(PinFn::Tim2Ch1, 2, Port::A, 0),
    route(PinFn::Tim2Ch2, 2, Port::A, 1),
    route(PinFn::Tim2Ch3, 2, Port::B, 3),
    route(PinFn::Tim2Ch4, 2, Port::B, 4),
    route(PinFn::Tim2Ch1N, 2, Port::C, 3),
    route(PinFn::Tim2Ch2N, 2, Port::A, 22),
    route(PinFn::Tim2Ch3N, 2, Port::A, 23),
    route(PinFn::Tim2Etr, 2, Port::A, 19),
    route(PinFn::Tim2Bkin, 2, Port::A, 20),
    route(PinFn::Tim2Ch1, 3, Port::B, 21),
    route(PinFn::Tim2Ch2, 3, Port::B, 15),
    route(PinFn::Tim2Ch3, 3, Port::B, 3),
    route(PinFn::Tim2Ch4, 3, Port::B, 4),
    route(PinFn::Tim2Ch1N, 3, Port::A, 12),
    route(PinFn::Tim2Ch2N, 3, Port::A, 13),
    route(PinFn::Tim2Ch3N, 3, Port::A, 14),
    route(PinFn::Tim2Etr, 3, Port::A, 18),
    route(PinFn::Tim2Bkin, 3, Port::A, 9),
    route(PinFn::Tim2Ch1, 4, Port::B, 16),
    route(PinFn::Tim2Ch2, 4, Port::B, 17),
    route(PinFn::Tim2Ch3, 4, Port::B, 18),
    route(PinFn::Tim2Ch4, 4, Port::B, 19),
    route(PinFn::Tim2Ch1N, 4, Port::C, 1),
    route(PinFn::Tim2Ch2N, 4, Port::C, 2),
    route(PinFn::Tim2Ch3N, 4, Port::C, 3),
    route(PinFn::Tim2Etr, 4, Port::C, 4),
    route(PinFn::Tim2Bkin, 4, Port::C, 0),
    route(PinFn::Tim2Ch1, 5, Port::C, 19),
    route(PinFn::Tim2Ch2, 5, Port::A, 12),
    route(PinFn::Tim2Ch3, 5, Port::A, 13),
    route(PinFn::Tim2Ch4, 5, Port::C, 0),
    route(PinFn::Tim2Ch1N, 5, Port::C, 18),
    route(PinFn::Tim2Ch2N, 5, Port::B, 12),
    route(PinFn::Tim2Ch3N, 5, Port::B, 3),
    route(PinFn::Tim2Etr, 5, Port::A, 2),
    route(PinFn::Tim2Bkin, 5, Port::B, 4),
    route(PinFn::Tim2Ch1, 6, Port::C, 19),
    route(PinFn::Tim2Ch2, 6, Port::C, 14),
    route(PinFn::Tim2Ch3, 6, Port::C, 15),
    route(PinFn::Tim2Ch4, 6, Port::C, 0),
    route(PinFn::Tim2Ch1N, 6, Port::B, 11),
    route(PinFn::Tim2Ch2N, 6, Port::B, 12),
    route(PinFn::Tim2Ch3N, 6, Port::B, 3),
    route(PinFn::Tim2Etr, 6, Port::A, 2),
    route(PinFn::Tim2Bkin, 6, Port::B, 4),
    route(PinFn::Tim2Ch1, 7, Port::C, 19),
    route(PinFn::Tim2Ch2, 7, Port::C, 14),
    route(PinFn::Tim2Ch3, 7, Port::C, 15),
    route(PinFn::Tim2Ch4, 7, Port::C, 0),
    route(PinFn::Tim2Ch1N, 7, Port::B, 11),
    route(PinFn::Tim2Ch2N, 7, Port::B, 12),
    route(PinFn::Tim2Ch3N, 7, Port::B, 3),
    route(PinFn::Tim2Etr, 7, Port::A, 2),
    route(PinFn::Tim2Bkin, 7, Port::B, 4),
    // ---- TIM3 ----
    route(PinFn::Tim3Ch1, 0, Port::A, 6),
    route(PinFn::Tim3Ch2, 0, Port::A, 7),
    route(PinFn::Tim3Ch1, 1, Port::B, 4),
    route(PinFn::Tim3Ch2, 1, Port::B, 5),
    route(PinFn::Tim3Ch1, 2, Port::C, 19),
    route(PinFn::Tim3Ch2, 2, Port::C, 18),
    route(PinFn::Tim3Ch1, 3, Port::A, 3),
    route(PinFn::Tim3Ch2, 3, Port::A, 4),
    // ---- PIOC ----
    route(PinFn::PiocIo0, 0, Port::C, 18),
    route(PinFn::PiocIo1, 0, Port::C, 19),
    route(PinFn::PiocIo0, 1, Port::C, 7),
    route(PinFn::PiocIo1, 1, Port::C, 19),
];

/// Mask of the remap codes that route `func` to `(port, pin)`.
pub const fn remap_mask(port: Port, pin: u8, func: PinFn) -> u8 {
    let mut mask = 0u8;
    let mut i = 0;
    while i < ROUTES.len() {
        let r = ROUTES[i];
        if r.pin.port as u8 == port as u8 && r.pin.pin == pin && r.func as u8 == func as u8 {
            mask |= 1 << r.remap;
        }
        i += 1;
    }
    mask
}

/// Whether `(port, pin)` carries `func` under any remap value.
pub const fn supports(port: Port, pin: u8, func: PinFn) -> bool {
    remap_mask(port, pin, func) != 0
}

/// The lowest remap code that routes `func` to `(port, pin)`.
pub const fn remap(port: Port, pin: u8, func: PinFn) -> Option<u8> {
    let mask = remap_mask(port, pin, func);
    if mask == 0 {
        None
    } else {
        Some(mask.trailing_zeros() as u8)
    }
}

// ============================================================================
// Analog functions
// ============================================================================

/// A non-ADC analog (or debug/clock) function of a pad.
///
/// Sources: CH32X035DS0 table 2-1 "default alternate function" column and
/// table 2-3, cross-checked against the OPA/CMP input select encodings in
/// [`crate::opa`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AnalogRole {
    /// OPA1 positive input 0 (PB0)
    Opa1P0,
    /// OPA1 positive input 1 (PB8)
    Opa1P1,
    /// OPA1 positive input 2 (PB4)
    Opa1P2,
    /// OPA1 negative input 0 (PA6)
    Opa1N0,
    /// OPA1 negative input 1 (PB6)
    Opa1N1,
    /// OPA1 negative input 2 (PA0)
    Opa1N2,
    /// OPA1 output 0 (PA3)
    Opa1O0,
    /// OPA1 output 1 (PB5)
    Opa1O1,
    /// OPA2 positive input 0 (PA7)
    Opa2P0,
    /// OPA2 positive input 1 (PB3)
    Opa2P1,
    /// OPA2 positive input 2 (PB7)
    Opa2P2,
    /// OPA2 negative input 0 (PA5)
    Opa2N0,
    /// OPA2 negative input 1 (PB1)
    Opa2N1,
    /// OPA2 negative input 2 (PA1)
    Opa2N2,
    /// OPA2 output 0 (PA4)
    Opa2O0,
    /// OPA2 output 1 (PA2)
    Opa2O1,
    /// CMP1 positive input 0 (PC19)
    Cmp1P0,
    /// CMP1 positive input 1 (PA0)
    Cmp1P1,
    /// CMP1 negative input 0 (PC3)
    Cmp1N0,
    /// CMP1 negative input 1 (PA23)
    Cmp1N1,
    /// CMP1 comparator output (PA1)
    Cmp1Out,
    /// CMP2 positive input 0 (PA12)
    Cmp2P0,
    /// CMP2 positive input 1 (PA11)
    Cmp2P1,
    /// CMP2 negative input 0 (PA22)
    Cmp2N0,
    /// CMP2 negative input 1 (PC3)
    Cmp2N1,
    /// CMP2 comparator output (PB2)
    Cmp2Out,
    /// CMP3 positive input 0 (PA13)
    Cmp3P0,
    /// CMP3 positive input 1 (PA14)
    Cmp3P1,
    /// CMP3 negative input 0 (PA2)
    Cmp3N0,
    /// CMP3 negative input 1 (PC3)
    Cmp3N1,
    /// CMP3 comparator output (PB3)
    Cmp3Out,
    /// USB D−
    UsbDm,
    /// USB D+
    UsbDp,
    /// Type-C CC1
    Cc1,
    /// Type-C CC2
    Cc2,
    /// Clock output (MCO)
    Mco,
    /// Debug data (SWDIO)
    Swdio,
    /// Debug clock (SWCLK)
    Swclk,
    /// Reset input (`RST_MODE`, PA21; PC3 on QSOP28/TSSOP20 only)
    Rst,
}

/// `(analog role, port, pin)` table from the datasheet pin tables.
const ANALOG: &[(AnalogRole, Port, u8)] = &[
    (AnalogRole::Opa1P0, Port::B, 0),
    (AnalogRole::Opa1P1, Port::B, 8),
    (AnalogRole::Opa1P2, Port::B, 4),
    (AnalogRole::Opa1N0, Port::A, 6),
    (AnalogRole::Opa1N1, Port::B, 6),
    (AnalogRole::Opa1N2, Port::A, 0),
    (AnalogRole::Opa1O0, Port::A, 3),
    (AnalogRole::Opa1O1, Port::B, 5),
    (AnalogRole::Opa2P0, Port::A, 7),
    (AnalogRole::Opa2P1, Port::B, 3),
    (AnalogRole::Opa2P2, Port::B, 7),
    (AnalogRole::Opa2N0, Port::A, 5),
    (AnalogRole::Opa2N1, Port::B, 1),
    (AnalogRole::Opa2N2, Port::A, 1),
    (AnalogRole::Opa2O0, Port::A, 4),
    (AnalogRole::Opa2O1, Port::A, 2),
    (AnalogRole::Cmp1P0, Port::C, 19),
    (AnalogRole::Cmp1P1, Port::A, 0),
    (AnalogRole::Cmp1N0, Port::C, 3),
    (AnalogRole::Cmp1N1, Port::A, 23),
    (AnalogRole::Cmp1Out, Port::A, 1),
    (AnalogRole::Cmp2P0, Port::A, 12),
    (AnalogRole::Cmp2P1, Port::A, 11),
    (AnalogRole::Cmp2N0, Port::A, 22),
    (AnalogRole::Cmp2N1, Port::C, 3),
    (AnalogRole::Cmp2Out, Port::B, 2),
    (AnalogRole::Cmp3P0, Port::A, 13),
    (AnalogRole::Cmp3P1, Port::A, 14),
    (AnalogRole::Cmp3N0, Port::A, 2),
    (AnalogRole::Cmp3N1, Port::C, 3),
    (AnalogRole::Cmp3Out, Port::B, 3),
    (AnalogRole::UsbDm, Port::C, 16),
    (AnalogRole::UsbDp, Port::C, 17),
    (AnalogRole::Cc1, Port::C, 14),
    (AnalogRole::Cc2, Port::C, 15),
    (AnalogRole::Mco, Port::B, 9),
    (AnalogRole::Swdio, Port::C, 18),
    (AnalogRole::Swclk, Port::C, 19),
    (AnalogRole::Rst, Port::A, 21),
    (AnalogRole::Rst, Port::C, 3),
];

/// Whether `(port, pin)` carries the given analog role.
pub const fn has_analog(role: AnalogRole, port: Port, pin: u8) -> bool {
    let mut i = 0;
    while i < ANALOG.len() {
        let (r, p, n) = ANALOG[i];
        if r as u8 == role as u8 && p as u8 == port as u8 && n == pin {
            return true;
        }
        i += 1;
    }
    false
}

/// `(port, pin, ADC channel)` table from the datasheet pin tables.
const ADC: &[(Port, u8, u8)] = &[
    (Port::A, 0, 0),
    (Port::A, 1, 1),
    (Port::A, 2, 2),
    (Port::A, 3, 3),
    (Port::A, 4, 4),
    (Port::A, 5, 5),
    (Port::A, 6, 6),
    (Port::A, 7, 7),
    (Port::B, 0, 8),
    (Port::B, 1, 9),
    (Port::C, 0, 10),
    (Port::C, 1, 11),
    (Port::C, 2, 12),
    (Port::C, 3, 13),
];

/// The ADC channel sampled from `(port, pin)`, if the pin is an ADC input.
pub const fn adc_channel(port: Port, pin: u8) -> Option<u8> {
    let mut i = 0;
    while i < ADC.len() {
        let (p, n, ch) = ADC[i];
        if p as u8 == port as u8 && n == pin {
            return Some(ch);
        }
        i += 1;
    }
    None
}

// ============================================================================
// Packages
// ============================================================================

/// A CH32X035 package variant.
///
/// Each variant matches the HAL cargo feature of the same name (lower-case).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Package {
    /// CH32X035R8T6 — LQFP64M
    R8T6,
    /// CH32X035C8T6 — LQFP48
    C8T6,
    /// CH32X035G8U6 — QFN28
    G8U6,
    /// CH32X035G8R6 — QSOP28
    G8R6,
    /// CH32X035F8U6 — QFN20
    F8U6,
    /// CH32X035F7P6 — TSSOP20
    F7P6,
    /// CH32X035D8U6 — QFN12
    D8U6,
}

impl Package {
    /// All packages, in feature order.
    pub const ALL: [Package; 7] = [
        Package::R8T6,
        Package::C8T6,
        Package::G8U6,
        Package::G8R6,
        Package::F8U6,
        Package::F7P6,
        Package::D8U6,
    ];

    /// One-bit mask for this package (see [`packages`]).
    pub const fn bit(self) -> u8 {
        1 << (self as u8)
    }

    /// Part number (`CH32X035R8T6`).
    pub const fn name(self) -> &'static str {
        match self {
            Package::R8T6 => "CH32X035R8T6",
            Package::C8T6 => "CH32X035C8T6",
            Package::G8U6 => "CH32X035G8U6",
            Package::G8R6 => "CH32X035G8R6",
            Package::F8U6 => "CH32X035F8U6",
            Package::F7P6 => "CH32X035F7P6",
            Package::D8U6 => "CH32X035D8U6",
        }
    }

    /// Whether this package is selected by the enabled cargo features.
    pub const fn enabled(self) -> bool {
        enabled_packages() & self.bit() != 0
    }
}

const R8T6: u8 = Package::R8T6.bit();
const C8T6: u8 = Package::C8T6.bit();
const G8U6: u8 = Package::G8U6.bit();
const G8R6: u8 = Package::G8R6.bit();
const F8U6: u8 = Package::F8U6.bit();
const F7P6: u8 = Package::F7P6.bit();
const D8U6: u8 = Package::D8U6.bit();

/// Mask of the packages selected through cargo features.
///
/// Sub-model features are additive: enabling several (e.g. in a diagnostic
/// build) exposes their union.
pub const fn enabled_packages() -> u8 {
    let mut mask = 0;
    if cfg!(feature = "ch32x035r8t6") {
        mask |= R8T6;
    }
    if cfg!(feature = "ch32x035c8t6") {
        mask |= C8T6;
    }
    if cfg!(feature = "ch32x035g8u6") {
        mask |= G8U6;
    }
    if cfg!(feature = "ch32x035g8r6") {
        mask |= G8R6;
    }
    if cfg!(feature = "ch32x035f8u6") {
        mask |= F8U6;
    }
    if cfg!(feature = "ch32x035f7p6") {
        mask |= F7P6;
    }
    if cfg!(feature = "ch32x035d8u6") {
        mask |= D8U6;
    }
    mask
}

/// `(port, pin, package mask)` table from CH32X035DS0 table 2-1.
const BONDED: &[(Port, u8, u8)] = &[
    (Port::A, 0, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6),
    (Port::A, 1, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6),
    (Port::A, 2, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::A, 3, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::A, 4, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6),
    (Port::A, 5, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::A, 6, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::A, 7, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::A, 8, R8T6 | C8T6),
    (Port::A, 9, R8T6 | C8T6),
    (Port::A, 10, R8T6 | C8T6),
    (Port::A, 11, R8T6 | C8T6),
    (Port::A, 12, R8T6 | C8T6 | G8R6),
    (Port::A, 13, R8T6 | C8T6 | G8R6),
    (Port::A, 14, R8T6 | C8T6),
    (Port::A, 15, R8T6 | C8T6),
    (Port::A, 16, R8T6 | C8T6),
    (Port::A, 17, R8T6 | C8T6),
    (Port::A, 18, R8T6 | C8T6),
    (Port::A, 19, R8T6 | C8T6),
    (Port::A, 20, R8T6 | C8T6),
    (Port::A, 21, R8T6 | C8T6),
    (Port::A, 22, R8T6 | C8T6),
    (Port::A, 23, R8T6 | C8T6),
    (Port::B, 0, R8T6 | C8T6 | G8U6 | G8R6 | F8U6),
    (Port::B, 1, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6),
    (Port::B, 2, R8T6 | C8T6),
    (Port::B, 3, R8T6 | C8T6 | G8U6 | G8R6 | F8U6),
    (Port::B, 4, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 5, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 6, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 7, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 8, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 9, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 10, R8T6 | C8T6 | G8U6 | G8R6),
    (Port::B, 11, R8T6 | C8T6 | G8U6 | G8R6 | F8U6),
    (Port::B, 12, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6),
    (Port::B, 13, R8T6 | C8T6),
    (Port::B, 14, R8T6),
    (Port::B, 15, R8T6),
    (Port::B, 16, R8T6),
    (Port::B, 17, R8T6),
    (Port::B, 18, R8T6),
    (Port::B, 19, R8T6),
    (Port::B, 20, R8T6),
    (Port::B, 21, R8T6),
    (Port::C, 0, R8T6 | G8U6),
    (Port::C, 1, R8T6 | F7P6),
    (Port::C, 2, R8T6),
    (Port::C, 3, R8T6 | G8U6 | G8R6 | F7P6),
    (Port::C, 4, R8T6),
    (Port::C, 5, R8T6),
    (Port::C, 6, R8T6 | C8T6),
    (Port::C, 7, R8T6 | C8T6 | D8U6),
    (Port::C, 14, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::C, 15, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::C, 16, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::C, 17, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::C, 18, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
    (Port::C, 19, R8T6 | C8T6 | G8U6 | G8R6 | F8U6 | F7P6 | D8U6),
];

/// Mask of the packages that bond out `(port, pin)`.
pub const fn packages(port: Port, pin: u8) -> u8 {
    let mut i = 0;
    while i < BONDED.len() {
        let (p, n, mask) = BONDED[i];
        if p as u8 == port as u8 && n == pin {
            return mask;
        }
        i += 1;
    }
    0
}

/// Whether `(port, pin)` exists on at least one CH32X035 package.
///
/// This is the compile-time "does this pad exist at all" check used by
/// [`crate::pins`]; whether it is bonded out on the *selected* package is
/// enforced by [`crate::gpio::Pins`] when the pin token is created.
pub const fn exists(port: Port, pin: u8) -> bool {
    packages(port, pin) != 0
}

/// Whether `(port, pin)` is bonded out on at least one selected package.
pub const fn bonded(port: Port, pin: u8) -> bool {
    packages(port, pin) & enabled_packages() != 0
}

/// Logical pins that share one physical pad on a package.
///
/// The datasheet pin table gives them the same pin number on that package
/// (CH32X035DS0 table 2-1). Both pins keep their full function set but they
/// must not be driven as outputs at the same time. The datasheet's "I/O"
/// count counts the shared pad once, which is why [`gpio_pads`] is lower than
/// the number of bonded pins on those packages.
pub const SHARED: &[(Port, u8, Port, u8, u8)] = &[
    (Port::B, 1, Port::B, 5, G8U6),
    (Port::B, 1, Port::B, 5, G8R6),
    (Port::A, 12, Port::C, 14, G8R6),
    (Port::A, 13, Port::C, 15, G8R6),
    (Port::A, 7, Port::C, 7, D8U6),
];

/// The pin `(port, pin)` is bonded together with on `package`, if any.
pub const fn shared_with(port: Port, pin: u8, package: u8) -> Option<PinId> {
    let mut i = 0;
    while i < SHARED.len() {
        let (pa, na, pb, nb, mask) = SHARED[i];
        if mask & package != 0 {
            if pa as u8 == port as u8 && na == pin {
                return Some(PinId::new(pb, nb));
            }
            if pb as u8 == port as u8 && nb == pin {
                return Some(PinId::new(pa, na));
            }
        }
        i += 1;
    }
    None
}

/// Number of physical GPIO pads on the packages in `mask`.
///
/// This is the number of bonded pins minus one per shared pad, and matches the
/// "I/O" figure of the datasheet model table.
pub const fn gpio_pads(mask: u8) -> u8 {
    let mut count: u8 = 0;
    let mut shared: u8 = 0;
    let mut i = 0;
    while i < BONDED.len() {
        let (port, pin, bond) = BONDED[i];
        if bond & mask != 0 {
            count += 1;
            let mut j = 0;
            while j < SHARED.len() {
                let (pa, na, _, _, smask) = SHARED[j];
                if smask & mask != 0 && pa as u8 == port as u8 && na == pin {
                    shared += 1;
                }
                j += 1;
            }
        }
        i += 1;
    }
    count - shared
}

/// Number of physical GPIO pads on the enabled package(s).
///
/// Enabling several packages reports the union, i.e. a pad only shared on one
/// of them still counts twice.
pub const fn gpio_pads_enabled() -> u8 {
    gpio_pads(enabled_packages())
}

/// ADC inputs on the packages in `mask`.
pub const fn adc_channels(mask: u8) -> u8 {
    let mut count = 0;
    let mut i = 0;
    while i < ADC.len() {
        let (port, pin, _) = ADC[i];
        if packages(port, pin) & mask != 0 {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Number of ADC inputs bonded out on the selected package(s).
pub const fn adc_channels_enabled() -> u8 {
    adc_channels(enabled_packages())
}

// ============================================================================
// Summary
// ============================================================================

/// Everything [`pin`](self) knows about one pad.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PinSummary {
    /// The pin.
    pub id: PinId,
    /// ADC channel, if the pin is an ADC input.
    pub adc: Option<u8>,
    /// Mask of packages bonding out this pin (see [`packages`]).
    pub packages: u8,
}

/// Describe `(port, pin)`, or `None` if no package bonds it out.
pub const fn desc(port: Port, pin: u8) -> Option<PinSummary> {
    if packages(port, pin) == 0 {
        return None;
    }
    Some(PinSummary {
        id: PinId::new(port, pin),
        adc: adc_channel(port, pin),
        packages: packages(port, pin),
    })
}

// ============================================================================
// Table consistency checks (evaluated at compile time)
// ============================================================================

// A route may only reference a pad that exists on some package; this catches
// typos in the tables without a test harness.
const _: () = {
    let mut i = 0;
    while i < ROUTES.len() {
        let r = ROUTES[i];
        assert!(exists(r.pin.port, r.pin.pin), "route on a non-existent pin");
        i += 1;
    }
    let mut i = 0;
    while i < ANALOG.len() {
        let (_, port, pin) = ANALOG[i];
        assert!(exists(port, pin), "analog role on a non-existent pin");
        i += 1;
    }
    let mut i = 0;
    while i < ADC.len() {
        let (port, pin, _) = ADC[i];
        assert!(exists(port, pin), "ADC channel on a non-existent pin");
        i += 1;
    }
};

// ADC channels are unique (each channel reaches exactly one pad).
const _: () = {
    let mut i = 0;
    while i < ADC.len() {
        let mut j = i + 1;
        while j < ADC.len() {
            assert!(ADC[i].2 != ADC[j].2, "duplicate ADC channel");
            j += 1;
        }
        i += 1;
    }
};

// Every shared pad must carry the same pin number on that package, exactly
// twice (the datasheet marks both logical pins with one physical pin).
const _: () = {
    let mut i = 0;
    while i < SHARED.len() {
        let (pa, na, pb, nb, mask) = SHARED[i];
        assert!(packages(pa, na) & mask != 0, "shared pin not bonded");
        assert!(packages(pb, nb) & mask != 0, "shared pin not bonded");
        assert!(
            na != nb || pa as u8 != pb as u8,
            "a pin cannot share with itself"
        );
        i += 1;
    }
};

// The I/O-pad and ADC columns of the datasheet model table must match the pin
// bonding above. This is the check that keeps `BONDED` honest.
const _: () = {
    assert!(gpio_pads(R8T6) == 60);
    assert!(gpio_pads(C8T6) == 46);
    assert!(gpio_pads(G8U6) == 27);
    assert!(gpio_pads(G8R6) == 26);
    assert!(gpio_pads(F8U6) == 19);
    assert!(gpio_pads(F7P6) == 18);
    assert!(gpio_pads(D8U6) == 11);

    assert!(adc_channels(R8T6) == 14);
    assert!(adc_channels(C8T6) == 10);
    assert!(adc_channels(G8U6) == 12);
    assert!(adc_channels(G8R6) == 11);
    assert!(adc_channels(F8U6) == 10);
    assert!(adc_channels(F7P6) == 11);
    assert!(adc_channels(D8U6) == 5);
};

// Spot checks copied from the datasheet and the reference manual.
const _: () = {
    assert!(supports(Port::B, 10, PinFn::Usart1Tx)); // USART1 TX, USART1_RM = 00
    assert!(supports(Port::C, 16, PinFn::I2c1Scl)); // I2C1 SCL, I2C1_RM = 010
    assert!(supports(Port::A, 6, PinFn::Spi1Miso)); // SPI1 MISO, SPI1_RM = 00
    assert!(matches!(adc_channel(Port::B, 0), Some(8))); // ADC_IN8 = PB0
    assert!(has_analog(AnalogRole::UsbDp, Port::C, 17)); // UDP = PC17
    assert!(has_analog(AnalogRole::Cc1, Port::C, 14)); // CC1 = PC14
    assert!(!exists(Port::C, 13)); // PC8..PC13 are bonded on no package
    assert!(!exists(Port::A, 24)); // port A ends at A23
    assert!(packages(Port::A, 15) == R8T6 | C8T6); // PA15 is LQFP64/48 only
    assert!(matches!(
        shared_with(Port::B, 1, G8U6),
        Some(id) if matches!(id.port, Port::B) && id.pin == 5
    ));
    // TIM1 remap 1xx only carries CH3/PC14 and ETR/PC15.
    assert!(matches!(remap(Port::B, 9, PinFn::Tim1Ch1), Some(0)));
    assert!(!supports(Port::C, 14, PinFn::Tim1Ch1));
    // USART3_RM = 11 has no CK.
    assert!(!supports(Port::B, 16, PinFn::Usart3Ck));
};
