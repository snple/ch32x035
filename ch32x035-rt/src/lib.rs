//! # ch32x035-rt
//!
//! Runtime support for CH32X035 RISC-V microcontrollers.
//!
//! The CH32X035 uses WCH's QingKe V4C core, an RV32IMAC RISC-V core with the
//! WCH custom extension (hardware stack, interrupt nesting) and the
//! Programmable Fast Interrupt Controller (PFIC) instead of the standard
//! RISC-V PLIC/CLINT.
//!
//! This crate provides:
//! - Low-level RISC-V assembly instructions (WFI, WFE, ECALL, MRET, fences, ...)
//! - Core CSR register access (mstatus, mie, mtvec, mepc, mcause, mscratch, ...)
//! - Core peripherals (PFIC interrupt controller, SysTick)
//! - Interrupt management
//! - Runtime startup code (reset handler, vector table)
//! - A linker script (`link.x`) matching the WCH flash/RAM layout
//!
//! ## Target
//!
//! This crate targets `riscv32imac-unknown-none-elf`.
//!
//! ## Features
//!
//! - `rt` - Runtime support (startup code, vector table, linker script)
//! - `run-from-ram` - Link the whole image into RAM (for flashing via USB)
//!
//! ## Linker scripts and `device.x`
//!
//! This crate ships `link.x`, which `INCLUDE memory.x` (from the application)
//! and `INCLUDE device.x` for the device-specific interrupt handler symbols.
//! `device.x` is **not** provided by this crate: it is generated and exposed by
//! [`ch32x035-pac`]'s build script (`cargo:rustc-link-search`).
//!
//! As a result, any binary that links against `ch32x035-rt` (and uses its
//! linker script) must also depend on `ch32x035-pac`.
//!
//! ## Example
//!
//! ```ignore
//! use ch32x035_rt::{asm, interrupt, peripheral};
//!
//! asm::nop();
//! asm::wfi();
//!
//! interrupt::disable();
//! unsafe { interrupt::enable(); }
//! ```

#![no_std]
#![deny(missing_docs)]
#![allow(asm_sub_register)]
#![allow(binary_asm_labels)]
#![allow(clippy::pointers_in_nomem_asm_block)]

pub mod asm;
pub mod interrupt;
pub mod peripheral;
pub mod register;
mod rt;

pub use peripheral::{Peripherals, pfic::InterruptNumber};
pub use rt::{__EXCEPTIONS, __RESET_VECTOR, Vector};
