# ch32x035 — Rust HAL for the WCH CH32X035

RISC-V (WCH QingKe V4C) HAL for the CH32X035 microcontroller, written in the
style of the [rp2350](https://github.com/rp-rs/rp2350) project. The project
is split into layered crates:

```
ch32x035-examples   →  ch32x035 (HAL)
                          ├─ ch32x035-rt    (RISC-V runtime: startup, PFIC, SysTick)
                          ├─ ch32x035-pac   (register access, SVD-generated)
                          └─ ch32x035-macro (#[main] / #[exception] / #[bind] / #[ramfunc])
```

| Crate | Purpose |
|---|---|
| `ch32x035-pac` | Peripheral Access Crate. Generated from `ch32x035.svd` by the `xtask` generator (`cargo xtask gen-pac`). |
| `ch32x035-rt` | Runtime support: RISC-V assembly/CSR access, PFIC interrupt controller, SysTick, reset handler + vector table, linker script (`link.x`). |
| `ch32x035-macro` | Procedural macros: `#[main]`, `#[exception]`, `#[bind(IRQ)]`, `#[ramfunc]`. |
| `ch32x035` | Hardware Abstraction Layer: `gpio`, `rcc`, `reset`, `uart`, `spi`, `i2c`, `adc`, `dma`, `timer`, `exti`, `afio`, `awu`, `opa`, `esig`, `usbpd`, `pwr`, `watchdog`, `flash`, `delay`, `time`, `interrupt`. |
| `ch32x035-examples` | `blink` and `hello_uart` examples. |
| `xtask` | PAC code generator (adapted from rp2350). |

## Target

`riscv32imac-unknown-none-elf` (Rust 1.88+, edition 2024).

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

## Key facts

- **Core**: WCH QingKe V4C, RV32IMAC, single core, machine mode.
- **Clocks**: boots on the internal HSI at **48 MHz** (no PLL in the SVD).
- **Memory**: 62K flash @ 0x00000000, 20K RAM @ 0x20000000.
- **Interrupts**: WCH **PFIC** at 0xE000_E000 (NVIC-like), SysTick at
  0xE000_F000. The vector table is an array of handler addresses in flash
  (entry 0 = reset, 1-15 = core exceptions, 16-54 = external interrupts);
  `mtvec` points to it in WCH vectored mode (mode 3).
- **Critical sections**: save `mstatus`, clear `mstatus.MIE`, nesting counter.

## License

MIT OR Apache-2.0
