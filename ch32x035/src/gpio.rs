//! GPIO (General Purpose I/O)
//!
//! Type-safe, ownership-based GPIO for the CH32X035's GPIOA/GPIOB/GPIOC
//! ports. A pin is represented by the zero-sized type [`Pin<PORT, N>`], where
//! `PORT` selects the port (0 = A, 1 = B, 2 = C) and `N` the pin number.
//!
//! # Ownership model
//!
//! [`Pins`] owns the GPIO ports. Pins are created from it with
//! [`Pins::pa`]/[`Pins::pb`]/[`Pins::pc`]. Each `Pin<N>` is `!Sync` (like
//! pico-rs) — create each pin token only once.
//!
//! # Example
//!
//! ```ignore
//! use ch32x035::{Rcc, gpio::Pins};
//!
//! let mut rcc = Rcc::new(dp.RCC);
//! let pins = Pins::init(dp.GPIOA, dp.GPIOB, dp.GPIOC, &mut rcc);
//! let mut led = pins.pa::<0>();
//! led.into_output();
//! led.set_high();
//! ```

use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

use crate::pac;
use crate::rcc::{Apb2Periph, Rcc};

// Register offsets within a GPIO port
const OFFSET_CFGLR: u32 = 0x00; // config, pins 0-7
const OFFSET_CFGHR: u32 = 0x04; // config, pins 8-15
const OFFSET_INDR: u32 = 0x08; // input data
const OFFSET_OUTDR: u32 = 0x0C; // output data
const OFFSET_BSHR: u32 = 0x10; // set/reset

/// GPIO port base addresses
const GPIOA_BASE: u32 = 0x4001_0800;
const GPIOB_BASE: u32 = 0x4001_0C00;
const GPIOC_BASE: u32 = 0x4001_1000;

/// MODE field values (2 bits)
const MODE_INPUT: u32 = 0b00;
const MODE_OUTPUT_50MHZ: u32 = 0b11;

/// CNF field values (2 bits)
const CNF_OUTPUT_PUSHPULL: u32 = 0b00;
const CNF_OUTPUT_OPENDRAIN: u32 = 0b01;
const CNF_INPUT_ANALOG: u32 = 0b00;
const CNF_INPUT_FLOATING: u32 = 0b01;
const CNF_INPUT_PULL: u32 = 0b10;

/// GPIO port ownership container.
pub struct Pins {
    _gpioa: pac::GPIOA,
    _gpiob: pac::GPIOB,
    _gpioc: pac::GPIOC,
}

impl Pins {
    /// Create the pin container, enabling and resetting the GPIO ports.
    ///
    /// Consumes the GPIO peripherals and the AFIO clock.
    pub fn init(
        gpioa: pac::GPIOA,
        gpiob: pac::GPIOB,
        gpioc: pac::GPIOC,
        rcc: &mut Rcc,
    ) -> Self {
        // Enable the port clocks and reset the ports to a known state
        let ports = Apb2Periph::Iopa | Apb2Periph::Iopb | Apb2Periph::Iopc | Apb2Periph::Afio;
        rcc.apb2_reset(ports);
        rcc.apb2_clear_reset(ports);
        rcc.apb2_enable(ports);

        Self {
            _gpioa: gpioa,
            _gpiob: gpiob,
            _gpioc: gpioc,
        }
    }

    /// Create a pin token for GPIO port A, pin `N` (0..=15).
    pub fn pa<const N: u32>(&self) -> Pin<0, N> {
        const { assert!(N <= 15, "GPIOA has pins 0..=15") };
        Pin::new()
    }

    /// Create a pin token for GPIO port B, pin `N` (0..=15).
    pub fn pb<const N: u32>(&self) -> Pin<1, N> {
        const { assert!(N <= 15, "GPIOB has pins 0..=15") };
        Pin::new()
    }

    /// Create a pin token for GPIO port C, pin `N` (0..=15).
    pub fn pc<const N: u32>(&self) -> Pin<2, N> {
        const { assert!(N <= 15, "GPIOC has pins 0..=15") };
        Pin::new()
    }
}

/// A single GPIO pin (zero-sized, type-state token).
///
/// `PORT`: 0 = A, 1 = B, 2 = C. `N`: pin number 0..=15.
///
/// `Pin` is `Send` but **not** `Sync`: only one task should own a given pin.
pub struct Pin<const PORT: u32, const N: u32> {
    _marker: PhantomData<*const ()>,
}

unsafe impl<const PORT: u32, const N: u32> Send for Pin<PORT, N> {}

/// Pin number of this pin
impl<const PORT: u32, const N: u32> Pin<PORT, N> {
    /// The pin number
    pub const fn pin_number() -> u32 {
        N
    }

    /// The port number (0 = A, 1 = B, 2 = C)
    pub const fn port_number() -> u32 {
        PORT
    }

    /// Create a new pin token (used internally by `Pins`)
    const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Base address of this pin's port
    const fn port_base() -> u32 {
        match PORT {
            0 => GPIOA_BASE,
            1 => GPIOB_BASE,
            2 => GPIOC_BASE,
            _ => panic!("invalid GPIO port"),
        }
    }

    /// Volatile read a 32-bit register at `base + offset`
    #[inline(always)]
    fn read_reg(offset: u32) -> u32 {
        unsafe { read_volatile((Self::port_base() + offset) as *const u32) }
    }

    /// Volatile write a 32-bit register at `base + offset`
    #[inline(always)]
    fn write_reg(offset: u32, value: u32) {
        unsafe { write_volatile((Self::port_base() + offset) as *mut u32, value) }
    }

    /// Volatile read-modify-write a 32-bit register at `base + offset`
    #[inline(always)]
    fn modify_reg<F: FnOnce(u32) -> u32>(offset: u32, f: F) {
        let addr = (Self::port_base() + offset) as *mut u32;
        unsafe { write_volatile(addr, f(read_volatile(addr))) }
    }

    /// The CFG register offset and bit shift for this pin's 4-bit config
    const fn cfg_pos() -> (u32, u32) {
        if N < 8 {
            (OFFSET_CFGLR, N * 4)
        } else {
            (OFFSET_CFGHR, (N - 8) * 4)
        }
    }

    /// Set the pin's CNF/MODE configuration
    fn set_config(cnf: u32, mode: u32) {
        let (reg, shift) = Self::cfg_pos();
        let mask = 0b1111 << shift;
        let value = ((cnf & 0b11) << (shift + 2)) | ((mode & 0b11) << shift);
        Self::modify_reg(reg, |v| (v & !mask) | value);
    }

    // =====================================================================
    // Output configuration
    // =====================================================================

    /// Configure the pin as a push-pull output at 50 MHz
    pub fn into_output(&mut self) {
        Self::set_config(CNF_OUTPUT_PUSHPULL, MODE_OUTPUT_50MHZ);
    }

    /// Configure the pin as an open-drain output at 50 MHz
    pub fn into_output_open_drain(&mut self) {
        Self::set_config(CNF_OUTPUT_OPENDRAIN, MODE_OUTPUT_50MHZ);
    }

    /// Set the pin output high
    #[inline(always)]
    pub fn set_high(&mut self) {
        Self::write_reg(OFFSET_BSHR, 1 << N);
    }

    /// Set the pin output low
    #[inline(always)]
    pub fn set_low(&mut self) {
        Self::write_reg(OFFSET_BSHR, 1 << (N + 16));
    }

    /// Toggle the pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        Self::modify_reg(OFFSET_OUTDR, |v| v ^ (1 << N));
    }

    /// Get the current output data value of the pin
    #[inline(always)]
    pub fn is_output_high(&self) -> bool {
        Self::read_reg(OFFSET_OUTDR) & (1 << N) != 0
    }

    // =====================================================================
    // Input configuration
    // =====================================================================

    /// Configure the pin as a floating input
    pub fn into_input(&mut self) {
        Self::set_config(CNF_INPUT_FLOATING, MODE_INPUT);
    }

    /// Configure the pin as a pull-up input
    pub fn into_input_pullup(&mut self) {
        Self::set_config(CNF_INPUT_PULL, MODE_INPUT);
        // Set the output data bit to select pull-up (1 = pull-up, 0 = pull-down)
        Self::modify_reg(OFFSET_OUTDR, |v| v | (1 << N));
    }

    /// Configure the pin as a pull-down input
    pub fn into_input_pulldown(&mut self) {
        Self::set_config(CNF_INPUT_PULL, MODE_INPUT);
        Self::modify_reg(OFFSET_OUTDR, |v| v & !(1 << N));
    }

    /// Configure the pin as an analog input
    pub fn into_analog(&mut self) {
        Self::set_config(CNF_INPUT_ANALOG, MODE_INPUT);
    }

    /// Read the pin's input value
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        Self::read_reg(OFFSET_INDR) & (1 << N) != 0
    }

    /// Read the pin's input value
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}
