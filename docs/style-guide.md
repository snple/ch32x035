# ch32x035 项目设计风格指南

本文档整理 `ch32x035`（WCH CH32X035，RISC-V QingKe V4C 内核）Rust HAL 的设计约定，
供同类嵌入式 Rust 项目参考。它刻意与 `rp2350` 项目保持同一套风格（见
`/home/danc/code/rp2350/rp2350/docs/style-guide.md`），只在平台差异处做适配。

> 范围：本仓库位于 `/home/danc/code/ch32x035/ch32x035`，HAL crate 名为 `ch32x035`。

---

## 1. 工作区结构（Crate 分层）

```
ch32x035/             # 工作区根
├── ch32x035/         # HAL：手写、面向用户
├── ch32x035-pac/     # 由 SVD 生成的 PAC（勿手改）
├── ch32x035-rt/      # 运行时：reset handler、栈、中断向量
├── ch32x035-macro/   # 过程宏：#[main] #[exception] #[bind] ramfunc
├── ch32x035-examples/# 各外设的独立二进制示例
└── xtask/            # PAC/SVD 生成器
```

与 rp2350 的差异：

- **usbpd 是 HAL 内的子模块**（`ch32x035/src/usbpd/`），不是独立 crate。
- 没有独立的 `-drivers`/`-usb` crate（ch32x035 无 USB 设备栈、设备驱动较少）。
- `edition = "2024"`，`#[deny(missing_docs)]` + `#![allow(clippy::missing_safety_doc)]`。

---

## 2. HAL 模块骨架（每个模块都长这样）

```rust
//! 模块文档：概述 → Features/Example（```ignore）。

use crate::Steal;
use crate::pac;
use crate::rcc::{Clocks, Rcc};
use crate::reset::Resettable; // 若可复位

/// 外设包装结构体。拥有 PAC ZST。
pub struct Foo {
    foo: pac::FOO,
    #[allow(dead_code)] // 仅 init 时用于算分频/波特率
    clocks: Clocks,
}

impl Foo {
    /// 从 PAC 拿所有权。可复位时先 `foo.init(&mut rcc)`（reset + 开时钟）。
    pub fn init(foo: pac::FOO, rcc: &mut Rcc, clocks: Clocks, config: Config) -> Self {
        foo.init(rcc);
        let s = Self { foo, clocks };
        // ... 应用 config ...
        s
    }

    /// 无所有权检查地拿句柄（ISR / 已初始化场景）。
    ///
    /// # Safety
    /// 调用者保证已初始化、且无数据竞争。
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            foo: unsafe { pac::FOO::steal() },
            clocks: unsafe { Clocks::steal() },
        })
    }

    /// 逃生口：返回底层寄存器块（可写外设标 `unsafe`）。
    pub unsafe fn regs(&self) -> &pac::foo::RegisterBlock { &self.foo }

    /// 归还 PAC 外设。
    pub fn release(self) -> pac::FOO { self.foo }

    // ... 具体方法 ...
}
```

要点：

- 每模块一个 `pub struct`，字段是 PAC ZST（+ 可选的 `Clocks` 值）。
- **有复位/时钟门控的外设用 `init(pac, &mut Rcc, ...)`；没有的用 `new(pac)`**
  （如 `Awu`、`Esig`、`Opa`、`Rcc` 无时钟门控）。
- 每个错误/状态用 `#[derive(Clone, Copy, PartialEq, Eq, Debug)]` +
  `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`。

---

## 3. PAC 关键事实

- PAC 外设结构体是非 `Copy` 的 ZST，`Deref<Target = RegisterBlock>`，自带
  `pub const unsafe fn steal() -> Self`（所以 HAL 的 `steal()` 里用
  `pac::FOO::steal()`；宏里注意写 `<$periph>::steal()` 加尖括号）。
- 寄存器访问用 `VolatileCell<u32>`（**不是 rp2350 的 `AtomicCell`**）：只有
  `read_bits`/`write_bits`/`modify_bits`，`modify_bits` 是读-改-写、**非原子**，
  没有 SET/CLR 硬件别名。清中断/清标志的 W1C 位用 `write_bits(mask)`。
- 字段命名统一：`fn field()` 读、`fn write_field(v)` 写多位数、
  `fn set_field()`/`fn clr_field()` 操作单比特位。
- **禁止裸指针 + 魔法数访问寄存器**（DMA 的 raw 访问是历史遗留，新代码走 PAC）。

### 芯片级关键事实（接入/改 HAL 前必核实）

- 内核 WCH QingKe V4C：RV32IMAC，单核 M-mode；target = `riscv32imac-unknown-none-elf`。
- 地址映射：FLASH 62K @ `0x00000000`，RAM 20K @ `0x20000000`。
- 中断控制器 PFIC @ `0xE000E000`（NVIC 风格：IENR/IRER/IPSR/IPRR/IACTR[8] + IPRIOR[256]）。
- SysTick @ `0xE000F000`（64 位向上计数 CTLR/SR/CNT/CMP）。
- 向量表：flash `0x0` = `.init`（la sp; j Reset），`0xC` 起 55 项 `.word` 处理器地址；
  `mtvec = base | 3`（WCH vectored 模式）。
- CSR：`0xbc0=0x1f`（流水线）、`0x804=0x3`（中断嵌套+硬件栈）、`mstatus=0x88`（MIE|MPIE）。
- 中断枚举用 WCH 规范名 + `_IRQ` 后缀。

### SVD 与厂商头文件的坑（历史教训）

- `esig` 的 `UNIID3` 被误标为 `AWU_APR`（用 `read_bits` 读原始值）。
- `opa` CTLR1/2 的 MODE/PSEL/NSEL 位宽与 WCH 头文件不同（以 SVD 为准）。
- `usbpd` 的 PORT_CC 无比较器字段定义（CC_CMP 位 5-7 需手写掩码）。
- `adc` **无 CTLR3**（CLK_DIV/AWD_SCAN/AWD_RST_EN 未入 SVD，PAC 无法访问）。
- `tim` CH2 输入捕获预分频器字段名是 **IC2PCS**（`write_ic2pcs`），不是 IC2PSC。
- **凡是依赖 WCH 头文件的结论，都要回到 SVD/PAC 核对**：头文件是 C 视角，
  SVD 才是 PAC 的事实源（生成器以 SVD 为准）。

---

## 4. 所有权模型与子句柄

- 控制器拥有 PAC ZST；`steal() -> Steal<Self>`。
- 子句柄借用：`struct X<'a> { regs: &'a PAC }`，`controller.sub(&self) -> X<'_>`。
- `Steal<T>`（定义在 `lib.rs`）：`pub struct Steal<T>(pub(crate) T)` + `Deref`
  （无 `DerefMut`）+ `AsRef` + `unsafe impl Send`，与 rp2350 完全一致。
- 类型状态 token：`Pin<const PORT, const N>` 是 ZST token（`Pins::pa/pb/pc`），
  端口所有权由 `Pins` 统一持有三个 GPIO PAC。

---

## 5. 复位与时钟（RCC）

`reset.rs` 提供 `Resettable` trait，**签名适配成 `&mut Rcc`**（RISC-V 上时钟
门控走 RCC）：

```rust
pub trait Resettable {
    fn enable_clock(&self, rcc: &mut Rcc);
    fn reset(&self, rcc: &mut Rcc);
    fn init(&self, rcc: &mut Rcc) { self.reset(rcc); self.enable_clock(rcc); }
}
```

用 `impl_apb2!`/`impl_apb1!`/`impl_ahb!` 三个宏为 PAC 类型批量实现。无复位线的
外设（如 `DMA`）只实现 `enable_clock`，`reset` 留空。

时钟树很简单：**固定 48 MHz HSI → `HPRE`（AHB 预分频器）→ 内核/HCLK/PCLK1/PCLK2**，
无 PLL、无 HSE、无时钟源切换。`rcc.rs` 提供：

- `HclkPrescaler`（`Div1`..`Div256`）、`McoSource`、`Config { hclk, hsi_trim, mco }`。
- `Rcc::freeze(Config) -> Clocks` 应用配置并返回时钟快照；
  `Rcc::hclk_prescaler()` 回读当前 `HPRE`。
- `Clocks::new(&Rcc)` 从 `CFGR0.HPRE` **回读真实频率**（`sysclk_hz = 48 MHz / 除数`），
  另有 `hclk_hz()` / `apb1_hz()` / `apb2_hz()`（三者与 `sysclk_hz` 相同）、
  `from_frequency(hz)` / `unsafe steal()`。

---

## 6. 有界等待（`spin` 模块）

与 rp2350 同款 `spin.rs`，三个 `pub(crate)` 助手：

- `spin_wait(what, cond)` — 有界自旋，超时 panic（hang-guard）。
- `try_spin_wait(cond) -> bool` — 有界自旋，不 panic。
- `spin_wait_max(max, what, cond)` — 自定义上界（暂无调用者，预留）。

**规则**：硬件「配置正确就必然完成」的等待（TXE/RXNE/SB/ADDR/BTF/EOC/BSY/TC）
一律用 `spin_wait`（主模式 I2C、SPI、UART、ADC、DMA、FLASH、USBPD 都已改）。
**例外**：由外部主机驱动的等待（I2C 从机等 STOP/ADDR/NACK）保持无界 `while`，
因为那是协议时序，不是硬件卡死。

---

## 7. 错误处理分层

每模块定义自己的 `Error` 枚举，分两层：

- **阻塞方法**（`write`/`read`/`transfer`/`read_channel`/`wait_busy`/`wait`/`flush`）：
  内部 `spin_wait`，超时 panic。
- **`try_*` 方法**（`try_write`/`try_read`/`try_transfer`/`try_read_channel`/
  `try_wait_busy`/`try_wait`/`try_flush`）：内部 `try_spin_wait`，返回
  `Result<_, Error>`。

阻塞方法通常委托给 `try_*` + `unwrap_or_else(|_| panic!(...))`，避免重复逻辑。

典型 `Error`：

```rust
pub enum Error { Timeout }                       // SPI/UART/ADC/DMA/FLASH
pub enum Error { Timeout, Acknowledge, BusError, Arbitration } // I2C
```

---

## 8. 驱动解耦（总线 trait 由调用者提供）

HAL **不内置** `SpiBus` / `I2cBus` / `UartBus` 等总线抽象 trait：需要抽象时由
调用者（驱动 crate 或最终应用）自行定义并实现。HAL 的具体类型（`Spi1`、
`I2c1`、`Uart1`…）已暴露完整的方法签名（阻塞方法 + `try_*` 返回 `Result`），
调用者可据此定义 trait 并为具体类型实现，或直接使用具体类型。

注意：具体类型上**同名阻塞方法（返回 `()`）会遮蔽 trait 方法（返回 `Result`）**
——直接调用走阻塞语义，泛型 `<B: XxxBus>` 走 `Result` 语义，这是有意为之。

### 中断支持约定（重要）

**HAL 不实现中断状态机**，只提供足够的低层 API，让用户用 `#[bind]` 自己写：

- 中断使能/禁能：`enable_*_interrupt` / `disable_interrupts`。
- 状态标志读取：`rx_not_empty` / `tx_empty` / `byte_transfer_finished` 等。
- 数据读写：`write_data` / `read_data`。
- **清标志的时序必须封装**（这是易错点）：I2C 的 ADDR 清法 =「读 STAR1→读 STAR2」、
  STOPF 清法 =「读 STAR1→写 CTLR1」；USART 的 IDLE/错误清法 =「读 STATR→读 DATAR」。
- 模板：`i2c.rs` 的 `impl_i2c_interrupt!` 宏（主/从共用的低层中断 API）。

### 平台无关工具放 `kit`

与硬件寄存器无关的通用数据结构/工具放 `kit` 模块（仿 rp2350），**不要塞进外设模块**：

- `kit::ring_buffer::RingBuffer`（中断接收用的环形缓冲；最初误放 `uart.rs`，已迁出）。
- 新增通用工具前先问「它碰不碰寄存器」——不碰就放 `kit`。

---

## 9. 宏约定

- 过程宏在 `ch32x035-macro`：`#[main]`、`#[exception]`、`#[bind(IRQ)]`、`#[ramfunc]`。
- 实例化宏用 `paste`/`macro_rules!` 生成同类外设（`impl_uart!`、`impl_timer!`、
  `impl_timer_pwm!`、gpio 的 per-pin 宏、`impl_apb1!` 等）。
- **`macro_rules!` 卫生坑**：宏内 `let x = ...` 绑定的标识符对调用方传入的
  `$body:block` 不可见（E0425），把标识符作为 `$x:ident` 参数传入。
- **`ty` 片段做限定路径要加尖括号**：`<$periph>::steal()`，不能 `$periph::steal()`。
- **`paste!` 里 `::[<Ident>]` 路径段会报错**（"expected identifier, found `[`"），
  改用方法调用形式 `regs.reg.[<method $n>]()`。
- **`macro_rules!` 子模块导入**：父模块定义 `macro_rules! peripheral` 后必须
  `pub(crate) use peripheral;`，否则子模块 `use super::peripheral` 报
  "no `peripheral` in `peripheral`"（rustc 1.97）。

---

## 10. 命名与文档风格

- 模块名与外设缩写一致：`rcc`/`afio`/`esig`/`usbpd`。
- 枚举 `#[repr(u32)]` 或枚举变体直接映射寄存器编码（如 `BaudDiv::Div2 => 0b000`）。
- 模块文档：概述 → `# Example`（```ignore）。`#[deny(missing_docs)]` 强制补全
  所有公开项文档，包括 `# Safety`。
- 跨模块复用类型从源头 `pub` 后引用（如 `crate::gpio::{DriveStrength, ...}`）。

---

## 11. 验证流程（每个改动都跑）

```bash
cargo check -p ch32x035
cargo clippy -p ch32x035 --lib
cargo check -p ch32x035-examples --bins
cargo fmt -p ch32x035 -- --check
```

### 烧录与链接

- `ch32x035-rt` 的 build.rs 按 `run-from-ram` feature 复制 `link.x` / `link_ram.x`。
- `-Tlink.x` 由 workspace `.cargo/config.toml` 的 rustflags 提供；
  `memory.x` / `device.x` 由示例 / PAC 的 build.rs 提供。
- defmt：示例需 `use defmt_rtt as _;` 且自己提供 `_defmt_timestamp`（defmt-rtt 不含）。

---

## 12. 常见 bug 模式（历史教训）

- **无界 `while !flag {}`** → 有界 `spin_wait`（本指南第 6 节已完成改造）。
- **W1C 寄存器清位用 `write_bits(mask)`**，不要 `modify_bits(|v| v & !mask)`
  （ch32x035 PAC 无 `set_bits`/`clr_bits`，读-改-写非原子）。
- **除零**（分频/波特率计算，`compute_brr` 里 `debug_assert!(baudrate != 0)`）。
- **`1u32 << 32` 是 UB**；位移错误。
- **空数据不碰总线**（`I2c1::read`/`write_read` 对空缓冲提前返回）。
- **事务必须等硬件完成**（UART `flush` 等 TC 位，不是只等 TXE）。
- **I2C 从机 `loop` 等 STOP 是外部主机时序**，不要套固定上界。
- **`inline const` block 只在 monomorphization 时求值**：`cargo check` 不触发
  E0080（未被调用的导出泛型不实例化），必须 `cargo build` 才能看到报错。
- **新过程宏首次被示例使用时必测**（`#[bind]` 曾缺 `fn` 关键字，示例从没用过就没暴露）。
- **`enumflags2` 0.7.x 是属性宏** `#[enumflags2::bitflags]`（不是 `#[derive(BitFlags)]`）。
- **`critical_section::RawRestoreState` 是 `u32` 类型别名**，直接返回/接收 u32，
  不能 `RawRestoreState(x)` 构造。
- **函数参数不会自动 `Into`**：需要泛型 `<I: Into<...>>` 或显式 `.into()`。
- **`static mut` 引用用 `&mut *core::ptr::addr_of_mut!(X)`**（Rust 2024 下 `&mut X` 直接写报错）。

---

## 13. 新增外设模块清单

1. 确认 PAC 已有对应外设（`ch32x035-pac/src/generated/`）。
2. 新建 `src/<name>.rs`，照第 2 节骨架；确认是否有 RCC 复位/门控（第 5 节）。
3. 在 `lib.rs` 加 `pub mod <name>;` + 重导出。
4. 跑第 11 节四条命令，全绿。
