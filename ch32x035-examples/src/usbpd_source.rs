//! USB PD source example.
//!
//! Supplies power as a USB PD source: advertises 5 V/3 A and 9 V/2 A fixed
//! PDOs, waits for the sink's `Request`, then sends `Accept` + `PS_RDY`.
//! While [`Ready`](SourceState::Ready) it keeps listening for re-requests,
//! `Get_Source_Cap` and `Soft_Reset` so power delivery can be maintained.
//!
//! PD pins: **CC1 = PC14**, **CC2 = PC15** (floating inputs; the engine adds
//! the 330 µA pull-up itself). Connect VBUS/GND to the sink appropriately.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin usbpd_source --release
//! ```

#![no_std]
#![no_main]

use defmt_rtt as _;

/// defmt timestamp (see the examples `common` module).
#[unsafe(export_name = "_defmt_timestamp")]
fn defmt_timestamp() -> u64 {
    0
}

use ch32x035::gpio::Pins;
use ch32x035::usbpd::{Usbpd, make_fixed_pdo};
use ch32x035::{Clocks, CorePeripherals, Delay, PdSource, Rcc, SourceState, main, pac};

/// PD DMA buffers (4-byte aligned via `#[repr(align(4))]`).
#[repr(align(4))]
struct Align4([u8; 34]);

static mut RX: Align4 = Align4([0; 34]);
static mut TX: Align4 = Align4([0; 34]);

/// Source_Capabilities: 5 V/3 A and 9 V/2 A fixed supplies.
static SRC_CAPS: [u32; 2] = [make_fixed_pdo(5000, 3000), make_fixed_pdo(9000, 2000)];

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        AFIO,
        USBPD,
        ..
    } = dp;
    let cp = CorePeripherals::take().unwrap();

    let mut rcc = Rcc::new(RCC);
    let clocks = Clocks::new(&rcc);
    let pins = Pins::init(GPIOA, GPIOB, GPIOC, &mut rcc);

    // CC1 = PC14, CC2 = PC15 as floating inputs.
    let mut cc1 = pins.pc::<14>();
    let mut cc2 = pins.pc::<15>();
    cc1.into_input();
    cc2.into_input();

    let usbpd = Usbpd::init(USBPD, &AFIO, &mut rcc);
    let mut delay = Delay::init(cp.SYST, clocks.sysclk_hz());

    // SAFETY: RX/TX are `static` and stay alive for the whole program.
    let mut source = unsafe {
        let rx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(RX)).0;
        let tx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(TX)).0;
        PdSource::new(usbpd, rx, tx, &SRC_CAPS)
    };

    loop {
        source.poll(&mut delay);

        match source.state() {
            SourceState::Ready => {
                defmt::info!(
                    "Supplying {} mV / {} mA (port CC{})",
                    source.requested_mv(),
                    source.requested_ma(),
                    source.port() as u8 + 1
                );
            }
            SourceState::Error => {
                defmt::error!("PD source negotiation failed; retrying");
                source.reset();
            }
            _ => {}
        }
    }
}
