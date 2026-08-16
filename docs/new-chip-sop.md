# 新 MCU 接入 SOP

> 目标：拿到一份 SVD / 寄存器手册，按本 SOP 产出一个与两个样板项目**同风格**的 Rust HAL。
>
> 两个样板（刻意镜像，风格约定见各自 `docs/style-guide.md`）：
>
> | 样板 | 路径 | 内核 | 寄存器 cell |
> |------|------|------|------------|
> | ch32x035 | `/home/danc/code/ch32x035/ch32x035` | RISC-V QingKe V4C（RV32IMAC，M-mode） | 全 `VolatileCell`（非原子） |
> | rp2350 | `/home/danc/code/rp2350/rp2350` | Cortex-M33 | `AtomicCell`（可写，有 SET/CLR 别名）+ `VolatileCell`（只读） |
>
> 每一步都有「参考实现」和「验收标准」。按顺序做，别跳步。

---

## 0. 准备

- 收集资料：**SVD 优先**（生成 PAC 的唯一事实源）；没有 SVD 就用手册手写寄存器定义。
  厂商头文件（如 `ch32x035.h`）只作交叉参考，**结论必须回到 SVD/PAC 核对**。
- 确定内核类型与 `target` triple（RISC-V `riscv32imac-unknown-none-elf` /
  Cortex-M `thumbv8m.main-none-eabihf`）。
- 确认工具链：`rust-toolchain.toml` 固定 stable + 对应 target。

## 1. 搭工作区骨架（Crate 分层）

按样板建立五层结构：

```
<chip>/             # 工作区根
├── <chip>/         # HAL（手写，面向用户）
├── <chip>-pac/     # SVD 生成，勿手改
├── <chip>-rt/      # 运行时：reset handler、栈、向量表
├── <chip>-macro/   # 过程宏：#[main] #[exception] #[bind] ramfunc
├── <chip>-examples/# 各外设独立二进制
└── xtask/          # PAC/SVD 生成器
```

参考：`ch32x035/Cargo.toml`、`rp2350/Cargo.toml`（workspace 依赖、edition、features）。

验收：`cargo check --workspace` 能跑通空骨架。

## 2. 生成 PAC

- 移植 `xtask` 的生成器（从两个样板任选一份抄，改 SVD 路径与 crate 名）。
- 关键点：**cell 类型按寄存器类型区分**（rp2350 的 generator 已实现）——
  可写（RW/WO）且在原子别名地址范围内的用 `AtomicCell`，只读（RO）用 `VolatileCell`；
  RISC-V 无原子别名时全用 `VolatileCell`。
- 生成后跑 `cargo xtask gen-pac --check` 保证生成代码与 SVD 无漂移，**且不要手改生成代码**。

参考：`rp2350/xtask/src/generator.rs`、`ch32x035-pac/src/generated/common.rs`。

验收：PAC 有 27 个外设 ZST、`peripheral!` 宏注入 `Deref` + `PTR` + `steal()`。

## 3. 运行时 rt

手写零依赖运行时（或从样板拷贝改）：

- reset handler：栈初始化、`.bss` 清零、核级初始化（RISC-V 写 CSR；Cortex-M 使能 FPU/协处理器）。
- 向量表 / 中断控制器初始化。
- `link.x` / `link_ram.x`（RAM 执行变体）、`device.x`（由 PAC build.rs 提供）。

参考：`ch32x035-rt/src/`、`rp2350-rt/src/`。

验收：`#[main]` 最小程序能烧录、能进 `main`。

## 4. 过程宏 macro

拷贝样板宏，改中断/异常命名：

- `#[main]`（生成入口 + 调 `startup::init`）、`#[exception]`、`#[bind(IRQ)]`、`#[ramfunc]`。

参考：`ch32x035-macro/src/`、`rp2350-macro/src/`。

坑：新宏**首次被示例使用时必测**（`#[bind]` 曾缺 `fn` 关键字而没暴露）。

## 5. HAL 模块（按 style-guide 骨架逐个实现）

- 照 `docs/style-guide.md` 第 2 节骨架写每个外设：`pub struct` 持 PAC ZST、
  `init(pac, rcc/clocks, …)` / `steal()` / `regs()` / `release()`。
- 顺序建议：`gpio` → `rcc`（时钟树）→ `reset`（Resettable）→ `uart`/`spi`/`i2c`（总线）→
  `delay`/`time` → 其余外设 → 中断 API（见下）。
- **中断支持约定**：HAL 不写中断状态机，只给低层 API（使能/状态标志/数据读写/清标志时序），
  用户用 `#[bind]` 自己写。模板见 `ch32x035/src/i2c.rs` 的 `impl_i2c_interrupt!` 宏。
- 平台无关工具放 `kit` 模块（仿 rp2350 `kit`），不塞外设模块。

验收：`cargo clippy -p <chip> --lib` 零警告；每个外设都有阻塞 + `try_*` 两层。

## 6. 通用基础设施

- `Steal<T>`（`Deref` 无 `DerefMut` + `AsRef` + `unsafe impl Send`）。
- `spin` 模块（`spin_wait` / `try_spin_wait` / 可调上界）。
- `reset::Resettable` + 批量实现宏。
- `kit` 模块（`RingBuffer` 等平台无关工具）。

参考：两个样板的 `lib.rs`、`spin.rs`、`reset.rs`、`kit/`。

## 7. 示例

至少：`blink`（GPIO + 延时）+ 一个 `hello_uart`（串口打印）。样例走 `#[main]` + `Board::init`。

参考：`ch32x035-examples/src/{blink,hello_uart}.rs`、`common/board.rs`。

## 8. 验证（每个改动都跑）

```bash
cargo check -p <chip>
cargo clippy -p <chip> --lib
cargo check -p <chip>-examples --bins
cargo fmt -p <chip> -- --check
```

关键坑：**`inline const` block 只在 monomorphization 求值，`cargo check` 不触发 E0080，必须 `cargo build`。**

## 9. 坑的沉淀

把过程中踩的坑（SVD 字段名错、位宽不符、缺寄存器、芯片 quirks）**写回 `docs/style-guide.md` 的
「SVD 与厂商头文件的坑」和「常见 bug 模式」两节**——这是样板项目最重要的可复用资产，别只记在本地。

---

## 一页速查：两个样板的差异点（新芯片对齐时先看这里）

| 维度 | ch32x035 | rp2350 |
|------|----------|--------|
| 内核 / target | RV32IMAC / riscv32imac-unknown-none-elf | Cortex-M33 / thumbv8m.main-none-eabihf |
| 寄存器 cell | 全 VolatileCell（非原子） | AtomicCell（RW/WO）+ VolatileCell（RO） |
| 时钟门控 | 走 RCC（`Resettable::init(&mut Rcc)`） | 走复位寄存器（`Resettable::init()`） |
| 中断控制器 | PFIC（NVIC 风格） | NVIC |
| I2C 硬件 | STM32F1 风格（软件状态机） | DesignWare（FIFO） |
| USBPD | HAL 内子模块 | 无 |
