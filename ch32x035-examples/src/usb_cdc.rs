//! USB CDC-ACM echo example.
//!
//! Enumerates as a virtual serial port and echoes every received byte back to
//! the host. The USB pins are PC16 (D−) and PC17 (D+); both are configured as
//! inputs here, the internal D+ pull-up is enabled by the USB driver.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin usb_cdc --release
//! ```
//!
//! Then on a Linux host:
//! ```sh
//! lsusb | grep 1209:0001
//! picocom -b 115200 /dev/ttyACM0
//! ```

#![no_std]
#![no_main]

use ch32x035_examples as _;
use defmt_rtt as _;

use ch32x035::afio::{Afio, UsbPhySupply};
use ch32x035::{Clocks, Rcc, main, pac};
use ch32x035_usb::ch32x035::{Config, UsbfsBus};
use ch32x035_usb::device::class::cdc_acm::CdcAcm;
use ch32x035_usb::{UsbBusAllocator, UsbDeviceBuilder, UsbError};

/// VID/PID of the example (pid.codes test VID 0x1209).
const VID: u16 = 0x1209;
const PID: u16 = 0x0001;

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        AFIO,
        USBFS,
        ..
    } = dp;

    let mut rcc = Rcc::new(RCC);
    let clocks = Clocks::new(&rcc);
    let pins = ch32x035::gpio::Pins::init(GPIOA, GPIOB, GPIOC, &mut rcc);
    let mut afio = Afio::init(AFIO, &mut rcc);

    // USB D− (PC16) floating, D+ (PC17) pulled up. The USB driver enables the
    // internal 1.5 kΩ D+ pull-up used for full-speed enumeration.
    pins.pc::<16>().into_input();
    pins.pc::<17>().into_input_pullup();

    let bus = UsbfsBus::new(
        USBFS,
        &mut afio,
        &mut rcc,
        Config {
            supply: UsbPhySupply::V33,
        },
    );
    let alloc = UsbBusAllocator::new(bus);
    let mut serial = CdcAcm::new(&alloc);
    let mut dev = UsbDeviceBuilder::new(&alloc, VID, PID)
        .manufacturer("ch32x035")
        .product("CH32X035 CDC")
        .serial_number("0001")
        .max_power_ma(100)
        .build();

    defmt::info!("USB CDC echo ready ({} Hz)", clocks.sysclk_hz());

    // Echo one packet at a time: a packet read from the host is only dropped
    // once the IN endpoint has accepted it, so a multi-packet burst is
    // retried (NAKed) instead of losing data when the IN buffer is busy.
    let mut buf = [0u8; 64];
    let mut pending: Option<usize> = None;
    loop {
        dev.poll(&mut [&mut serial]);

        if let Some(len) = pending {
            match serial.write(&buf[..len]) {
                Ok(_) => pending = None,
                Err(UsbError::WouldBlock) => {}
                Err(_) => pending = None,
            }
        }
        if pending.is_none()
            && let Ok(n) = serial.read(&mut buf)
        {
            pending = Some(n);
        }
    }
}
