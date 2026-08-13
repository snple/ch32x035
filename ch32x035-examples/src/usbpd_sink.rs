//! USB PD sink example.
//!
//! Negotiates power from a USB PD source (e.g. a PD charger) as a sink.
//! Requests 5 V at max current first; after a short delay tries a PPS
//! (programmable power) renegotiation to 9 V / 1.5 A if the source advertises
//! an APDO. On failure the engine is reset and the negotiation retried.
//!
//! PD pins: **CC1 = PC14**, **CC2 = PC15** (floating inputs). Connect VBUS
//! and GND to the charger/board appropriately, and pull CC to the bus.
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin usbpd_sink --release
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
use ch32x035::usbpd::{PdoType, Port, Request, Usbpd};
use ch32x035::{Clocks, CorePeripherals, Delay, DelayUs, PdSink, Rcc, SinkState, main, pac};

/// PD DMA receive buffer (4-byte aligned via `#[repr(align(4))]`).
#[repr(align(4))]
struct Align4([u8; 34]);

static mut RX: Align4 = Align4([0; 34]);
static mut TX: Align4 = Align4([0; 34]);

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
    let delay = Delay::init(cp.SYST, clocks.sysclk_hz());

    // SAFETY: RX/TX are `static` and stay alive (and unaliased) for the whole
    // program; both are 4-byte aligned via the `Align4` wrapper.
    let port = unsafe {
        let rx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(RX)).0;
        let tx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(TX)).0;
        Port::new(usbpd, rx, tx)
    };
    let mut sink = PdSink::new(port);

    let mut pps_tried = false;

    loop {
        sink.poll(&delay);

        match sink.state() {
            SinkState::NeedRequest => {
                // List the advertised PDOs, then request 5 V at max current.
                defmt::info!("Source advertises {} PDO(s)", sink.pdo_count());
                for i in 0..sink.pdo_count() {
                    if let Some(p) = sink.pdo(i as usize) {
                        defmt::info!(
                            "  PDO{}: type={} v={}mV i={}mA",
                            i + 1,
                            p.pdo_type() as u8,
                            p.voltage_mv(),
                            p.max_current_ma()
                        );
                    }
                }
                sink.request(
                    Request::FixedVoltage {
                        voltage_mv: 5000,
                        current_ma: None,
                    },
                    &delay,
                );
            }
            SinkState::Ready => {
                defmt::info!(
                    "PD Ready: {} mV / {} mA (port CC{})",
                    sink.requested_mv(),
                    sink.requested_ma(),
                    sink.cc_port() as u8 + 1
                );

                // After a short delay, try a PPS renegotiation if an APDO
                // covering 9 V is advertised.
                if !pps_tried {
                    delay.delay_ms(2000);
                    let has_apdo = (0..sink.pdo_count()).any(|i| {
                        sink.pdo(i as usize)
                            .map(|p| p.pdo_type() == PdoType::Apdo)
                            .unwrap_or(false)
                    });
                    if has_apdo {
                        defmt::info!("Requesting PPS 9V / 1.5A");
                        sink.request(
                            Request::Pps {
                                voltage_mv: 9000,
                                current_ma: 1500,
                            },
                            &delay,
                        );
                    }
                    pps_tried = true;
                }
            }
            SinkState::Error => {
                defmt::error!("PD negotiation failed; retrying");
                sink.reset();
            }
            _ => {}
        }
    }
}
