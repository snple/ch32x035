//! `ch32x035-rtos` feature demo.
//!
//! Demonstrates, over RTT (defmt):
//!
//! - cooperative tasks with static priorities and `sleep` (`led` toggles
//!   PB12 every 500 ms; other tasks log at their own cadence),
//! - a counting semaphore (`waiter` blocks up to 3 s, `giver` signals every
//!   1.5 s),
//! - a mutex with **priority inheritance** (`low` holds the lock across a
//!   `yield_now`; thanks to inheritance its effective priority is raised
//!   above `mid` while `high` waits, so it resumes and releases promptly),
//! - a scheduler-state dump (`monitor` prints `task_infos()` every 5 s).
//!
//! Build:
//! ```sh
//! cargo build -p ch32x035-examples --bin rtos_demo --release
//! ```
//! Flash/run (WCH-Link):
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

/// Event used by the `waiter`/`giver` pair.
static EVT: Semaphore = Semaphore::new(0);
/// Mutex used by the priority-inheritance demonstration.
static MUT: Mutex = Mutex::new();

#[task(priority = 2, name = "led", stack = 768)]
fn led() -> ! {
    // Take the peripherals and bring up the board here; `main` only starts
    // the scheduler, so no other task/context races us for `Peripherals`.
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

#[task(priority = 3, name = "waiter", stack = 768)]
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

#[task(priority = 1, name = "giver", stack = 768)]
fn giver() -> ! {
    loop {
        sleep(1500);
        info!("[giver] signalling");
        EVT.give();
    }
}

#[task(priority = 1, name = "low", stack = 768)]
fn low() -> ! {
    loop {
        MUT.lock();
        info!("[low] holds lock, about to yield");
        yield_now();
        info!("[low] resumed, releasing lock");
        MUT.unlock();
        sleep(2000);
    }
}

#[task(priority = 2, name = "mid", stack = 768)]
fn mid() -> ! {
    loop {
        info!("[mid] running");
        // Yield so lower-priority tasks get a chance; sleep a little so the
        // RTT log doesn't flood (busy printing would fill the RTT buffer).
        sleep(100);
    }
}

#[task(priority = 3, name = "high", stack = 768)]
fn high() -> ! {
    loop {
        MUT.lock();
        info!("[high] acquired lock");
        MUT.unlock();
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

#[task(priority = 0, name = "monitor", stack = 768)]
fn monitor() -> ! {
    loop {
        sleep(5000);
        let infos = task_infos();
        for (i, t) in infos.iter().enumerate() {
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

    // The system clock is 48 MHz HSI after reset (the `led` task uses
    // `Board::init` with the default config, so it stays 48 MHz).
    start(cp.SYST, 48_000_000)
}
