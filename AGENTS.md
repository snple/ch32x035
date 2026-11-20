# ch32x035 仓库约定

本文件只记录**本仓库特有**的硬性约束。通用协作规范（长期主义、事实优先、回答方式等）
由用户级 `~/.copilot/copilot-instructions.md` 提供，不在此重复。

## 项目定位

WCH CH32X035（RISC-V QingKe V4C）的 Rust HAL，分层 crate：

```
ch32x035-examples → ch32x035 (HAL)  ├─ ch32x035-rt    启动/中断向量/PFIC/SysTick
                └─ ch32x035-usb     ├─ ch32x035-pac   SVD 生成的寄存器访问
                                    └─ ch32x035-macro #[main] #[exception] #[bind] #[ramfunc]
```

设计约定见 `docs/style-guide.md`，与 `/home/danc/code/rp2350/rp2350` 保持同一套风格。

## 硬性约束

1. **`ch32x035-pac/src/generated/` 是生成产物，不要手改。**
   改生成逻辑去改 `xtask/src/{generator,svd_parser,types}.rs`，然后 `cargo xtask gen-pac`。
   提交前用 `cargo xtask gen-pac --check` 确认生成物与 `ch32x035.svd` 同步。
   SVD 里 `<size>` 决定寄存器位宽，USBFS/USBPD 这类按字节排布的外设依赖这个信息，
   不要把寄存器统一当成 u32。

2. **USB 分层不能倒灌。**
   通用协议层（`UsbBus` trait、设备状态机、控制传输、描述符、CDC/HID/GS_USB/
   i2c-tiny-usb/UAC class）在跨仓库共享的 `usb-stack` crate；
   `ch32x035-usb` 只保留 `UsbfsBus` 硬件驱动并重新导出；HAL 只提供 `afio` 的 USB PHY 配置。
   不要把通用代码复制回本仓库。

3. **rtos 相关改动要跑全量验证**（`ch32x035-rtos` 自研协作式内核，临界区与优先级继承改动
   影响面大）。

## 构建与验证

```sh
# target：riscv32imac-unknown-none-elf
cargo build -p ch32x035 --target riscv32imac-unknown-none-elf
cargo build -p ch32x035-examples --bins --release --target riscv32imac-unknown-none-elf
cargo fmt --all --check
```

- 示例目录自带 `.cargo/config.toml`：默认 target + `runner = "probe-rs run --chip CH32X035F8U6"`。
- 改动 USB 或底层外设时，**优先上板验证**（本机常备 WCH-Link）：
  `cd ch32x035-examples && cargo run --bin usb_cdc --release`，
  主机侧 `lsusb | grep 1209:0001` 与 `picocom -b 115200 /dev/ttyACM<n>` 回环测试。
  RTT 在 WCH-Link 上不稳定，必要时用 `probe-rs read` 直接读寄存器和 RAM。
- 只跑构建不算验证过硬件行为，报告时要区分「编译通过」和「上板通过」。

## 提交与发布

- **提交信息用简体中文**，Conventional Commits 前缀（`feat(usb):`、`fix(rt):`、`chore:`）。
- 各 crate 版本独立演进：`ch32x035`、`-pac`、`-rt`、`-macro`、`-usb` 分别维护自己的
  `version`，不要改 `[workspace.package] version` 来统一发版。
- 发布是真实外发动作，执行前确认版本号与影响范围；crates.io 对新 crate 有速率限制，
  连续发布会返回 429。
- `ch32x035-rt` 含 RISC-V 汇编，`cargo publish` 的主机验证会失败，
  用 `--no-verify`，并以交叉编译验证代替：
  先 `cargo package`，再把 `.crate` 解包后 `cargo build --target riscv32imac-unknown-none-elf`。

## 已知坑

- `ch32x035-rt` 在 x86 主机上编译不过（汇编），host 侧只验证纯逻辑 crate。
- WCH USBFS：SETUP 事务的 `INT_ST` 端点号不可靠（实测 0xB9），一律按 EP0 处理；
  双向端点的 OUT 缓冲区放 DMA 偏移 0、IN 放偏移 64；EP4 与 EP0 共用缓冲区，不可分配。
- `AGENTS.md` 之外的文档改动（README、style-guide）要与代码改动同一次提交。
