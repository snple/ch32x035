//! Typed-pin demo: UART4 on PB0/PB1 and an ADC read on PA0.
//!
//! The pins are chosen *by the compiler*: `Uart4::init_with_pins` accepts only
//! pins that really carry USART4 TX/RX and derives the `USART4_RM` value from
//! the pair, while `Adc::read_pin` takes the channel number from the pin type.
//!
//! Pins are valid on the CH32X035F8U6 (QFN20) the examples target:
//! USART4 remap 0 is TX/PB0, RX/PB1; PA0 is ADC_IN0.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin typed_pins --release
//! ```

#![no_std]
#![no_main]

use defmt_rtt as _;

use ch32x035::{
    CorePeripherals, Delay, DelayUs,
    adc::Adc,
    afio::Afio,
    main, pac,
    uart::{Config, Uart4},
};
use ch32x035_examples::board::Board;

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        AFIO,
        ADC,
        UART4,
        ..
    } = dp;

    let board = Board::init(RCC, GPIOA, GPIOB, GPIOC);
    let cp = CorePeripherals::take().unwrap();
    let mut rcc = board.rcc;
    let mut afio = Afio::init(AFIO, &mut rcc);
    let delay = Delay::init(cp.SYST, board.clocks.sysclk_hz());

    let mut tx = board.pins.pb::<0>();
    let mut rx = board.pins.pb::<1>();
    let mut uart = Uart4::init_with_pins(
        UART4,
        &mut tx,
        &mut rx,
        &mut afio,
        &mut rcc,
        board.clocks,
        Config::default(),
    );

    let mut ain = board.pins.pa::<0>();
    let mut adc = Adc::init(ADC, &mut rcc, board.clocks);

    loop {
        let value = adc.read_pin(&mut ain);
        uart.write_str("ADC_IN0 = ");
        let mut buf = [0u8; 10];
        let mut n = value as u32;
        let mut idx = buf.len();
        loop {
            idx -= 1;
            buf[idx] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        for &b in &buf[idx..] {
            uart.write_byte(b);
        }
        uart.write_str("\r\n");
        delay.delay_ms(500);
    }
}
