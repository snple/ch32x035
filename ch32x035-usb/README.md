# ch32x035-usb

USB device stack for the WCH CH32X035 (USBFS full-speed device controller).

The MCU-independent part of the stack lives in the shared
[`usb-stack`](../../../rp2350/usb-stack) crate (the same core used by `rp2350-usb`)
and is re-exported here under `ch32x035_usb::device`, so
`ch32x035_usb::device::bus::…`, `ch32x035_usb::device::class::…`, … are the
generic implementation. The vocabulary shared with the host side
(`ch32x035_usb::{endpoint, descriptor, request}`) is re-exported too.

## Features

- **Bus layer** — [`UsbfsBus`] implements the `UsbBus` trait on top of the
  USBFS SIE: 64-byte packet transfers, manual data toggles, endpoint
  resource allocation and bus reset/suspend handling.
- **Shared device layer** (`usb-stack`) — `UsbDevice` / `UsbDeviceBuilder`
  control-transfer state machine, string descriptors built on the fly,
  standard-request handling (SET_ADDRESS, SET_CONFIGURATION,
  GET/SET_INTERFACE, CLEAR/SET_FEATURE, …).
- **Classes** (`usb-stack`) —
  - `device::class::cdc_acm::CdcAcm`: virtual serial port (2 interfaces, interrupt IN +
    bulk IN/OUT).
  - `device::class::hid::{HidClass, keyboard, mouse, custom, power}`: HID interfaces
    with caller-supplied or boot report descriptors.
  - `device::class::gs_usb::GsUsbClass`: candleLight/GS_USB CAN adapter.
  - `device::class::i2c_tiny_usb::I2cTinyUsb`: i2c-tiny-usb I2C bridge.
  - `device::class::uac::AudioClass`: USB Audio Class 1.0.
- No heap, no `usb-device` dependency, `defmt` logging optional.

## Quick start

```rust,ignore
use ch32x035::afio::{Afio, UsbPhySupply};
use ch32x035::gpio::Pins;
use ch32x035_usb::ch32x035::{Config, UsbfsBus};
use ch32x035_usb::device::class::cdc_acm::CdcAcm;
use ch32x035_usb::{UsbBusAllocator, UsbDeviceBuilder};

let mut rcc = Rcc::new(dp.RCC);
let pins = Pins::init(dp.GPIOA, dp.GPIOB, dp.GPIOC, &mut rcc);
let mut afio = Afio::init(dp.AFIO, &mut rcc);

// D− = PC16, D+ = PC17; the internal 1.5 kΩ pull-up is enabled by the driver.
pins.pc::<16>().into_input();
pins.pc::<17>().into_input_pullup();

let alloc = UsbBusAllocator::new(UsbfsBus::new(
    dp.USBFS,
    &mut afio,
    &mut rcc,
    Config { supply: UsbPhySupply::V33 },
));
let mut serial = CdcAcm::new(&alloc);
let mut dev = UsbDeviceBuilder::new(&alloc, 0x1209, 0x0001)
    .product("CH32X035 CDC")
    .build();

loop {
    dev.poll(&mut [&mut serial]);
    // read/write through the class endpoints
}
```

A complete echo firmware lives in
[`ch32x035-examples/src/usb_cdc.rs`](../ch32x035-examples/src/usb_cdc.rs).

## Endpoint mapping

| Endpoint | Use |
|---|---|
| EP0 | Control (64-byte shared IN/OUT buffer) |
| EP1, EP2, EP3, EP5, EP6, EP7 | Bulk / interrupt, IN and OUT can be used simultaneously |

EP4 shares its buffer area with EP0 on this part and is therefore not offered
for allocation. Classes are allocated greedily; an IN and an OUT request can
share an endpoint number. All endpoints have a fixed 64-byte maximum packet
size, and isochronous endpoints are not supported.

When an endpoint carries both directions, the SIE expects the OUT buffer at
offset 0 of its DMA region and the IN buffer at offset 64; the driver lays out
and programs the buffers accordingly.

## Polling model

The stack is polled: call `UsbDevice::poll` in a loop (or from the USBFS
interrupt). The USBFS interrupt enable register is programmed, but the driver
does not require an interrupt handler — the SIE holds the bus (NAK) while a
transfer flag is pending.

## Notes

- `SETUP` transactions are always treated as EP0; the SIE does not report a
  meaningful endpoint number for them.
- Clearing a halt on a non-control endpoint resets its data toggle, per
  USB 2.0 §9.4.5.
- The device address is applied after the SET_ADDRESS status stage, matching
  the WCH SDK.

## License

MIT OR Apache-2.0
