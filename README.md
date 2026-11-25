# ch32x035 — Rust HAL for the WCH CH32X035

RISC-V (WCH QingKe V4C) HAL for the CH32X035 microcontroller, written in the
style of the [rp2350](https://github.com/rp-rs/rp2350) project. The project
is split into layered crates:

```
ch32x035-examples   →  ch32x035 (HAL)
                          ├─ ch32x035-rt    (RISC-V runtime: startup, PFIC, SysTick)
                          ├─ ch32x035-pac   (register access, SVD-generated)
                          └─ ch32x035-macro (#[main] / #[exception] / #[bind] / #[ramfunc])

ch32x035-examples   →  ch32x035-usb (USB device stack: UsbBus, device, CDC/HID)
```

| Crate | Purpose |
|---|---|
| `ch32x035-pac` | Peripheral Access Crate. Generated from `ch32x035.svd` by the `xtask` generator (`cargo xtask gen-pac`). |
| `ch32x035-rt` | Runtime support: RISC-V assembly/CSR access, PFIC interrupt controller, SysTick, reset handler + vector table, linker script (`link.x`). |
| `ch32x035-macro` | Procedural macros: `#[main]`, `#[exception]`, `#[bind(IRQ)]`, `#[ramfunc]`. |
| `ch32x035` | Hardware Abstraction Layer: `gpio`, `rcc`, `reset`, `uart`, `spi`, `i2c`, `adc`, `dma`, `timer`, `exti`, `afio`, `awu`, `opa`, `esig`, `usbpd`, `pwr`, `watchdog`, `flash`, `delay`, `time`, `interrupt`, `pin`, `pins`. |
| `ch32x035-usb` | USB **device** stack for the USBFS controller: `UsbfsBus` (bus layer) plus the shared [`usb-stack`](../../rp2350/usb-stack) core (`UsbDevice`, CDC-ACM, HID, GS_USB, i2c-tiny-usb, UAC), the same core used by `rp2350-usb`. |
| `ch32x035-examples` | Peripheral examples, including `usb_cdc` (virtual serial echo). |
| `xtask` | PAC code generator (adapted from rp2350). |

## Target

`riscv32imac-unknown-none-elf` (Rust 1.100+, edition 2024).

## Toolchain

```sh
rustup target add riscv32imac-unknown-none-elf
```

## Building

```sh
# Everything (from the workspace root)
cargo build -p ch32x035 --target riscv32imac-unknown-none-elf

# Examples
cargo build -p ch32x035-examples --bin blink --target riscv32imac-unknown-none-elf --release
cargo build -p ch32x035-examples --bin hello_uart --target riscv32imac-unknown-none-elf --release
cargo build -p ch32x035-examples --bin usb_cdc --target riscv32imac-unknown-none-elf --release
```

USB example (enumerates as a CDC ACM port with VID:PID `1209:0001`):

```sh
cd ch32x035-examples
cargo run --bin usb_cdc --release
# on the host:
lsusb | grep 1209:0001
picocom -b 115200 /dev/ttyACM<n>
```

## Regenerating the PAC

```sh
cargo xtask gen-pac          # generate ch32x035-pac/src/generated/ from ch32x035.svd
cargo xtask gen-pac --check  # verify the checked-in code is up to date
```

## Flashing

Use a WCH-Link with `wchisp` (or the WCH OpenOCD toolchain):

```sh
wchisp flash target/riscv32imac-unknown-none-elf/release/blink
```

## Running examples with probe-rs

The example crate is configured so `cargo run` flashes and runs via a WCH-Link
(`probe-rs run --chip CH32X035F8U6`, see `ch32x035-examples/.cargo/config.toml`):

```sh
cd ch32x035-examples
cargo run --bin blink
```

### Linux: udev permissions

probe-rs needs read/write access to the probe's USB node. Install the official
[probe-rs udev rules](https://probe.rs/files/69-probe-rs.rules) and add a rule
for the WCH-Link (its RISC-V mode `1a86:8010` is already covered by the stock
rules; the DAP mode `1a86:8012` is not):

```
# /etc/udev/rules.d/69-probe-rs.rules
# WCH-Link (CH549 based, DAP mode)
ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="8012", MODE="660", GROUP="plugdev", TAG+="uaccess"
```

Then reload the rules and make sure your user is in the `plugdev` group:

```sh
sudo udevadm control --reload && sudo udevadm trigger
sudo usermod -a -G plugdev $USER   # log out and back in afterwards
```

### WCH-Link mode (RISC-V vs DAP)

The WCH-Link has two modes. The CH32X035 is a RISC-V chip, so the probe must be
in **RISC-V mode** (native WCH-Link protocol):

| Mode      | USB ID             | `probe-rs list` shows        |
|-----------|--------------------|------------------------------|
| RISC-V    | `1a86:8010`        | `WCH-Link` (no suffix)       |
| DAP (ARM) | `1a86:8012`        | `WCH-Link ... (CMSIS-DAP)`   |

A probe left in DAP mode fails with `Failed to send the JTAG sequence` /
`CMSIS-DAP responded with an error`. Check the current mode with:

```sh
probe-rs list
```

Switch to RISC-V mode with the **ModeS button**:

1. Unplug the probe.
2. Hold down the ModeS button.
3. Plug it back in (power-on) while holding it.
4. Short-press ModeS to cycle modes — RISC-V = blue LED **off** when idle,
   DAP = blue LED **on** — then long-press ~2 s to confirm.
5. Verify with `probe-rs list`: the entry should no longer show `(CMSIS-DAP)`.

## Pin model

Two modules describe and constrain the pads, in the style of `rp2350`:

- [`pin`](ch32x035/src/pin.rs) — the data: the `AFIO_PCFR1` remap tables
  (`USART1`–`USART4`, `I2C1`, `SPI1`, `TIM1`–`TIM3`, `PIOC`), the ADC channels
  (`ADC_IN0`–`ADC_IN13`), the analog roles (OPA1/OPA2 inputs and outputs,
  CMP1–CMP3 inputs and outputs, USB D±, PD CC1/CC2, MCO, SWD), the pads shared
  by two logical pins (`SHARED`) and which model bonds each pad out.
- `pins` — the constraints: one trait per signal (`Usart1TxPin`, `I2c1SclPin`,
  `Spi1MisoPin`, `Tim1Ch1Pin`, `Tim2Ch1NPin`, `AdcPin`, `Opa1PosPin`,
  `Cmp2NegPin`, …). Each implementation is verified against `pin` at compile
  time, and every table entry is asserted against the datasheet model figures
  (I/O pads and ADC channels per package).

Typed constructors derive the remap value from the pins, so a mis-wired bus
fails to compile (wrong pin) or panics at init (no shared remap):

```rust
let mut tx = board.pins.pb::<0>();
let mut rx = board.pins.pb::<1>();
let mut afio = Afio::init(dp.AFIO, &mut rcc);
let uart = Uart4::init_with_pins(
    dp.UART4, &mut tx, &mut rx, &mut afio, &mut rcc, board.clocks,
    Config::default(),
);

let mut ain = board.pins.pa::<0>();
let value = adc.read_pin(&mut ain); // ADC_IN0, taken from the pin type
```

`Pins::pa/pb/pc` still rejects pads that the *selected* package does not bond
out, and `pin::desc(port, pin)` reports the bonded models and ADC channel at
runtime, and `package::GPIO_PADS` / `package::ADC_CHANNELS` report the
per-package pad and channel counts.

## Key facts

- **Core**: WCH QingKe V4C, RV32IMAC, single core, machine mode.
- **Clocks**: boots on the internal HSI at **48 MHz** (no PLL in the SVD).
- **Memory**: 62K flash @ 0x00000000, 20K RAM @ 0x20000000.
- **USB**: USBFS full-speed device controller at 0x4002_3400 (8 endpoints,
  64-byte packets, byte-spaced 8/16/32-bit registers). USB D−/D+ are PC16/PC17.
- **Interrupts**: WCH **PFIC** at 0xE000_E000 (NVIC-like), SysTick at
  0xE000_F000. The vector table is an array of handler addresses in flash
  (entry 0 = reset, 1-15 = core exceptions, 16-54 = external interrupts);
  `mtvec` points to it in WCH vectored mode (mode 3).
- **Critical sections**: save `mstatus`, clear `mstatus.MIE`, nesting counter.

## License

MIT OR Apache-2.0
