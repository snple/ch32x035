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
use ch32x035::usbpd::{make_fixed_pdo, Usbpd};
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
static SRC_CAPS: [u32; 1] = [make_fixed_pdo(5000, 3000)];

/// Build the sink engine over the current `usbpd` (returns it on drop).
unsafe fn make_sink(usbpd: Usbpd) -> PdSink {
    // SAFETY: RX/TX are `static` and stay alive (and unaliased) for the whole
    // program; both are 4-byte aligned via the `Align4` wrapper.
    unsafe {
        let rx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(RX)).0;
        let tx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(TX)).0;
        PdSink::new(usbpd, rx, tx)
    }
}

/// Build the source engine over the current `usbpd` (returns it on drop).
unsafe fn make_source(usbpd: Usbpd) -> PdSource {
    // SAFETY: RX/TX are `static` and stay alive (and unaliased) for the whole
    // program; both are 4-byte aligned via the `Align4` wrapper.
    unsafe {
        let rx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(RX)).0;
        let tx: &'static mut [u8] = &mut (*core::ptr::addr_of_mut!(TX)).0;
        PdSource::new(usbpd, rx, tx, &SRC_CAPS)
    }
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
    let mut delay = Delay::init(cp.SYST, clocks.sysclk_hz());

    // SAFETY: RX/TX are `static` and stay alive for the whole program.
    let mut usbpd = usbpd;

    loop {
        // ---- Phase 1: act as a sink (draw power) ----
        let mut sink = unsafe { make_sink(usbpd) };
        loop {
            sink.poll(&mut delay);
            match sink.state() {
                SinkState::NeedRequest => {
                    sink.request_voltage(5000, None, &mut delay);
                }
                SinkState::Ready => {
                    defmt::info!(
                        "DRP as SINK: {} mV / {} mA (CC{})",
                        sink.requested_mv(),
                        sink.requested_ma(),
                        sink.port() as u8 + 1
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
        usbpd = sink.release();
        delay.delay_ms(300);

        // ---- Phase 2: act as a source (supply power) ----
        let mut source = unsafe { make_source(usbpd) };
        loop {
            source.poll(&mut delay);
            match source.state() {
                SourceState::Ready => {
                    defmt::info!(
                        "DRP as SOURCE: {} mV / {} mA (CC{})",
                        source.requested_mv(),
                        source.requested_ma(),
                        source.port() as u8 + 1
                    );
                    delay.delay_ms(2000);
                    break;
                }
                // No sink connected, or negotiation failed: switch role.
                SourceState::Error | SourceState::Idle => break,
                _ => {}
            }
        }
        usbpd = source.release();
        delay.delay_ms(300);
    }
}
