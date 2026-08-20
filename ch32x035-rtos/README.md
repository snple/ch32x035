# ch32x035-rtos

CH32X035（RISC-V QingKe V4C）的**协作式（非抢占）多任务内核**。

## 特性

- **协作式调度**：任务一直运行到主动让出 CPU（`yield_now` / `sleep` / 信号量等待），不抢占。
- **`#[task]` 过程宏注册任务**：`#[task(priority = N, name = "..", stack = N)] fn task() -> !`，三个参数均可选，编译期静态注册。
- **静态优先级**（`0..=7`，数字越大优先级越高）：让出点总是选择最高优先级就绪任务；同优先级内 FIFO 轮转（公平）。优先级只在任务主动让出时生效，不打断正在运行的任务。
- **`yield_now()`**：立即让出，当前任务回到就绪队尾。
- **`sleep(ms)`**：让出 CPU，约 `ms` 毫秒后被唤醒；时间不精确（其他任务不让出会延后）。`sleep(0)` 等价于 `yield_now()`。
- **计数信号量** [`Semaphore`]：`take(timeout_ms)` 让出直到被 `give` / `give_from_isr`（中断里安全）唤醒或超时；中断 `give` 只更新调度器、不切换任务。
- **互斥锁** [`Mutex`]：基于信号量实现，带**优先级继承**（等锁的高优先级任务会提升持锁者的有效优先级，避免锁释放被中间优先级任务拖延）。
- **idle 任务**：无就绪任务时执行 `wfi` 等待中断，不忙等。
- **栈金丝雀**：每次切换校验任务栈底哨兵值，溢出即 panic 并报告任务名。
- **调试快照** [`task_infos`]：返回所有任务的名字 / 状态 / 声明优先级 / 有效优先级。
- 时间基准：WCH SysTick 定时器 1 ms tick。

## 用法

```rust
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use ch32x035::{CorePeripherals, main, pac};
use ch32x035_rtos::{sleep, start, task};
use ch32x035_rtos::sync::Semaphore;

static EVT: Semaphore = Semaphore::new(0);

#[task(priority = 2, name = "led", stack = 1024)]
fn led() -> ! {
    // 初始化外设（Peripherals 只能 take 一次，建议只在一个任务里做）
    loop {
        info!("tick");
        sleep(500);
    }
}

#[task(priority = 1, name = "wait", stack = 1024)]
fn wait() -> ! {
    loop {
        if EVT.take(Some(1000)) {
            info!("signalled");
        }
    }
}

#[main]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    start(cp.SYST, 48_000_000) // 系统时钟 48 MHz（复位默认）
}
```

完整示例见 `ch32x035-examples/src/rtos_demo.rs` 与 `rtos_min.rs`。

## API 一览

| 函数 / 类型 | 说明 |
|---|---|
| [`start(syst, sysclk_hz)`] | 启动调度器（配置 tick、注册任务、进入第一个任务，永不返回） |
| [`yield_now()`] | 立即让出 CPU |
| [`sleep(ms)`] | 让出 CPU 约 ms 毫秒 |
| [`task`] | 注册任务的属性宏 |
| [`Semaphore::take`] / [`give`] / [`give_from_isr`] | 计数信号量 |
| [`Mutex::lock`] / [`unlock`] / [`try_lock`] | 互斥锁（优先级继承） |
| [`task_infos`] | 调试：任务状态快照 |

## 约束与约定

- 任务签名必须为 `fn() -> !`（永不返回），无参数。
- **禁止在临界区（`critical_section` / `interrupt::free`）内调用**任何让出/阻塞函数（`yield_now`、`sleep`、`take`、`lock`）。
- 内核独占 WCH SysTick，**不要**再使用 HAL 的 `Delay` 忙等驱动。
- 中断处理函数中**不要**调用阻塞 API；需要唤醒任务时用 `Semaphore::give_from_isr`（只更新调度状态）。
- `MAX_TASKS = 8`（含 idle），即最多 7 个用户任务；`MAX_PRIO = 7`。

## 内存预算（CH32X035：62K Flash / 20K RAM）

- 每个任务独立静态栈，建议 512 B – 1 KB（`#[task(stack = N)]`，默认 1 KB）。
- `rtos_demo` 7 个任务（768 B × 7 + idle 512 B）：Flash 约 16 KB，RAM 约 6 KB，余量充足。
- 栈底写入金丝雀（4 B/任务），切换时校验。

## 构建与烧录

```sh
# 构建（从工作区根需显式指定 target）
cargo build -p ch32x035-examples --bin rtos_demo --release \
    --target riscv32imac-unknown-none-elf

# 用 WCH-Link + probe-rs 烧录并运行（见 examples/.cargo/config.toml）
cd ch32x035-examples
cargo run --bin rtos_demo --release
```

通过 RTT（defmt）观察任务调度日志。

## 链接脚本说明

`#[task]` 把每个任务的 `TaskDef` 放进 `.rtos.tasks` 链接段，由 `rtos.x`（本 crate 的 `build.rs` 生成并提供 link-search）定义段边界符号，`start()` 在启动时收集。使用本内核的**可执行目标**需要在它自己的 `build.rs` 中加：

```rust
println!("cargo:rustc-link-arg=-Trtos.x");
```

（见 `ch32x035-examples/build.rs`，与 `defmt.x` 的注入方式一致。）

[`start`]: https://docs.rs/ch32x035-rtos
[`yield_now`]: https://docs.rs/ch32x035-rtos
[`sleep`]: https://docs.rs/ch32x035-rtos
[`task`]: https://docs.rs/ch32x035-rtos
[`Semaphore`]: https://docs.rs/ch32x035-rtos
[`Mutex`]: https://docs.rs/ch32x035-rtos
[`task_infos`]: https://docs.rs/ch32x035-rtos
