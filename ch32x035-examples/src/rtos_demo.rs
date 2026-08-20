//! `ch32x035-rtos` 功能演示。
//!
//! 通过 RTT（defmt）演示：
//!
//! - 带静态优先级与 `sleep` 的协作任务（`led` 每 500 ms 翻转 PB12；其它任务按各自
//!   节奏打印）；
//! - 计数信号量（`waiter` 最多阻塞 3 s，`giver` 每 1.5 s 唤醒一次）；
//! - 带**优先级继承**的互斥锁（`low` 持锁跨 `yield_now`；借助继承其有效优先级被
//!   提升到 `mid` 之上，使它能尽快恢复并释放锁）；
//! - 调度器状态快照（`monitor` 每 5 s 打印 `task_infos()`）。
//!
//! 构建：
//! ```sh
//! cargo build -p ch32x035-examples --bin rtos_demo --release
//! ```
//! 烧录/运行（WCH-Link）：
//! ```sh
//! cargo run -p ch32x035-examples --bin rtos_demo --release
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use ch32x035::{CorePeripherals, main, pac};
use ch32x035_examples::board::Board;
use ch32x035_rtos::task_mod::TaskState;
use ch32x035_rtos::{
    sleep, start,
    sync::{Mutex, Semaphore},
    task, task_infos, yield_now,
};

/// `waiter`/`giver` 之间的事件。
static EVT: Semaphore = Semaphore::new(0);
/// 优先级继承演示用互斥锁。
static MUT: Mutex<()> = Mutex::new(());

#[task(priority = 2, name = "led")]
fn led() -> ! {
    // 在此获取外设并初始化板子；`main` 只启动调度器，没有其它任务/上下文与
    // 我们竞争 `Peripherals`。
    let dp = pac::Peripherals::take().unwrap();
    let pac::Peripherals {
        RCC,
        GPIOA,
        GPIOB,
        GPIOC,
        ..
    } = dp;
    let board = Board::init(RCC, GPIOA, GPIOB, GPIOC);
    let mut led = board.pins.pb::<12>();

    led.into_output();
    loop {
        info!("[led] toggle");
        led.toggle();
        sleep(500);
    }
}

#[task(priority = 3, name = "waiter")]
fn waiter() -> ! {
    loop {
        info!("[waiter] waiting for event (3 s timeout)");
        if EVT.take(Some(3000)) {
            info!("[waiter] event received");
        } else {
            info!("[waiter] timed out");
        }
    }
}

#[task(priority = 1, name = "giver")]
fn giver() -> ! {
    loop {
        sleep(1500);
        info!("[giver] signalling");
        EVT.give();
    }
}

#[task(priority = 1, name = "low")]
fn low() -> ! {
    loop {
        {
            let _guard = MUT.lock();
            info!("[low] holds lock, about to yield");
            yield_now();
            info!("[low] resumed, releasing lock");
        } // `_guard` 在此自动解锁
        sleep(2000);
    }
}

#[task(priority = 2, name = "mid")]
fn mid() -> ! {
    loop {
        info!("[mid] running");
        // 让出以给低优先级任务机会；稍睡一会避免 RTT 日志刷屏。
        sleep(100);
    }
}

#[task(priority = 3, name = "high")]
fn high() -> ! {
    loop {
        {
            let _guard = MUT.lock();
            info!("[high] acquired lock");
        } // `_guard` 在此自动解锁
        sleep(1000);
    }
}

fn state_str(s: TaskState) -> &'static str {
    match s {
        TaskState::Ready => "Ready",
        TaskState::Running => "Running",
        TaskState::BlockedSleep => "Sleep",
        TaskState::BlockedSem => "Sem",
    }
}

#[task(priority = 0, name = "monitor")]
fn monitor() -> ! {
    loop {
        sleep(5000);
        for (i, t) in task_infos().iter().enumerate() {
            if let Some(t) = t {
                info!(
                    "[monitor] #{} {} state={} prio={}/{}",
                    i,
                    t.name,
                    state_str(t.state),
                    t.base_prio,
                    t.cur_prio
                );
            }
        }
    }
}

#[main]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();

    info!("rtos_demo starting");

    // 复位后系统时钟为 48 MHz HSI（`led` 任务用 `Board::init` 的默认配置，保持
    // 48 MHz）。
    start(cp.SYST, 48_000_000)
}
