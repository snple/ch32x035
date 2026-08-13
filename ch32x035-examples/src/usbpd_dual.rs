//! USB PD dual-role (DRP) example.
//!
//! Alternates between the sink and source roles: it first tries to draw power
//! as a sink; if no PD source is connected it falls back to supplying power
//! as a source (5 V/3 A). The [`Usbpd`] transceiver is released from one
//! engine and handed to the other via [`PdSink::release`]/[`PdSource::release`],
//! demonstrating role switching.
//!
//! This is a *demonstration* of the switching mechanism. The role detection
//! relies on the simplified CC detection of the policy engines; a production
//! DRP should use the WCH multi-threshold CC detection (`PD_Detect`) and
//! proper cable-connection/disconnection handling.
//!
//! PD pins: **CC1 = PC14**, **CC2 = PC15** (floating inputs).
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin usbpd_dual --release
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
use ch32x035::usbpd::{Pdo, Port, Request, Usbpd};
use ch32x035::{
    main, pac, Clocks, CorePeripherals, Delay, DelayUs, PdSink, PdSource, Rcc, SinkState,
    SourceState,
};

/// PD DMA buffers (4-byte aligned via `#[repr(align(4))]`).
#[repr(align(4))]
struct Align4([u8; 34]);

static mut RX: Align4 = Align4([0; 34]);
static mut TX: Align4 = Align4([0; 34]);

/// Source_Capabilities when acting as a source: 5 V/3 A.
static SRC_CAPS: [Pdo; 1] = [Pdo::fixed(5000, 3000)];

/// Build the sink engine over the given `port` (returns it via `into_port`).
fn make_sink(port: Port) -> PdSink {
    PdSink::new(port)
}

/// Build the source engine over the given `port` (returns it via `into_port`).
fn make_source(port: Port) -> PdSource {
    PdSource::new(port, &SRC_CAPS)
}

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

    // SAFETY: RX/TX are `static` and stay alive for the whole program.
    let mut port = unsafe {
        let rx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(RX)).0;
        let tx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(TX)).0;
        Port::new(usbpd, rx, tx)
    };

    loop {
        // ---- Phase 1: act as a sink (draw power) ----
        let mut sink = make_sink(port);
        loop {
            sink.poll(&delay);
            match sink.state() {
                SinkState::NeedRequest => {
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
                        "DRP as SINK: {} mV / {} mA (CC{})",
                        sink.requested_mv(),
                        sink.requested_ma(),
                        sink.cc_port() as u8 + 1
                    );
                    // Demo: hold this role for a while, then switch.
                    delay.delay_ms(2000);
                    break;
                }
                // No PD source connected, or negotiation failed: switch role.
                SinkState::Error | SinkState::Idle => break,
                _ => {}
            }
        }
        port = sink.into_port();
        delay.delay_ms(300);

        // ---- Phase 2: act as a source (supply power) ----
        let mut source = make_source(port);
        loop {
            source.poll(&delay);
            match source.state() {
                SourceState::Ready => {
                    defmt::info!(
                        "DRP as SOURCE: {} mV / {} mA (CC{})",
                        source.requested_mv(),
                        source.requested_ma(),
                        source.cc_port() as u8 + 1
                    );
                    delay.delay_ms(2000);
                    break;
                }
                // No sink connected, or negotiation failed: switch role.
                SourceState::Error | SourceState::Idle => break,
                _ => {}
            }
        }
        port = source.into_port();
        delay.delay_ms(300);
    }
}
