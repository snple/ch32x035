//! Rust code generator - Proper memory-mapped registers with field accessors
//!
//! Generates PAC code where:
//! - Each register is a `#[repr(transparent)]` struct with `UnsafeCell<u32>`
//! - RegisterBlock is a `#[repr(C)]` struct that directly maps to hardware memory
//! - Each register has field accessor methods with proper documentation
//!
//! Adapted from the rp2350 project generator for the CH32X035 (RISC-V, WCH
//! QingKe V4C). Unlike the RP2350, the CH32X035 has **no** atomic SET/CLR/XOR
//! register aliases, so every register uses a plain `VolatileCell`.

use crate::types::*;
use anyhow::Result;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

/// Canonical CH32X035 interrupt list (symbol name, vector index, short description).
///
/// Derived from the WCH `ch32x035.h` `IRQn_Type` enum and the startup vector
/// table. The vendor SVD's interrupt list is incomplete and uses inconsistent
/// names (e.g. `DMA_Channel1` vs `DMA1_Channel1`), so this table is the
/// authoritative source for the PAC interrupt enum and vector table.
///
/// Names already carry the `_IRQ` suffix and are in shouty-snake case: they are
/// used verbatim as the linker handler symbols (`device.x` PROVIDEs), the PAC
/// `Interrupt` enum variants, and the `#[bind(...)]` macro arguments. This
/// mirrors the rp2350 convention (e.g. `TIMER0_IRQ_0`) and avoids colliding
/// with linker region names such as `FLASH`.
///
/// External interrupts occupy vector slots 16..=54. Slots 0..=15 are the
/// core (exception) vectors provided by `ch32x035-rt`.
const CH32X035_INTERRUPTS: &[(&str, u32, &str)] = &[
    ("WWDG_IRQ", 16, "Window Watchdog interrupt"),
    ("PVD_IRQ", 17, "PVD through EXTI Line detection interrupt"),
    ("FLASH_IRQ", 18, "FLASH global interrupt"),
    ("EXTI7_0_IRQ", 20, "External Line[7:0] interrupts"),
    ("AWU_IRQ", 21, "AWU global interrupt"),
    ("DMA1_CHANNEL1_IRQ", 22, "DMA1 Channel 1 global interrupt"),
    ("DMA1_CHANNEL2_IRQ", 23, "DMA1 Channel 2 global interrupt"),
    ("DMA1_CHANNEL3_IRQ", 24, "DMA1 Channel 3 global interrupt"),
    ("DMA1_CHANNEL4_IRQ", 25, "DMA1 Channel 4 global interrupt"),
    ("DMA1_CHANNEL5_IRQ", 26, "DMA1 Channel 5 global interrupt"),
    ("DMA1_CHANNEL6_IRQ", 27, "DMA1 Channel 6 global interrupt"),
    ("DMA1_CHANNEL7_IRQ", 28, "DMA1 Channel 7 global interrupt"),
    ("ADC1_IRQ", 29, "ADC1 global interrupt"),
    ("I2C1_EV_IRQ", 30, "I2C1 Event interrupt"),
    ("I2C1_ER_IRQ", 31, "I2C1 Error interrupt"),
    ("USART1_IRQ", 32, "USART1 global interrupt"),
    ("SPI1_IRQ", 33, "SPI1 global interrupt"),
    ("TIM1_BRK_IRQ", 34, "TIM1 Break interrupt"),
    ("TIM1_UP_IRQ", 35, "TIM1 Update interrupt"),
    (
        "TIM1_TRG_COM_IRQ",
        36,
        "TIM1 Trigger and Commutation interrupt",
    ),
    ("TIM1_CC_IRQ", 37, "TIM1 Capture Compare interrupt"),
    ("TIM2_UP_IRQ", 38, "TIM2 Update interrupt"),
    ("USART2_IRQ", 39, "USART2 global interrupt"),
    ("EXTI15_8_IRQ", 40, "External Line[15:8] interrupts"),
    ("EXTI25_16_IRQ", 41, "External Line[25:16] interrupts"),
    ("USART3_IRQ", 42, "USART3 global interrupt"),
    ("USART4_IRQ", 43, "USART4 global interrupt"),
    ("DMA1_CHANNEL8_IRQ", 44, "DMA1 Channel 8 global interrupt"),
    ("USBFS_IRQ", 45, "USBFS Host/Device global interrupt"),
    ("USBFS_WKUP_IRQ", 46, "USBFS Host/Device WakeUp interrupt"),
    ("PIOC_IRQ", 47, "PIOC global interrupt"),
    ("OPA_IRQ", 48, "OPA global interrupt"),
    ("USBPD_IRQ", 49, "USBPD global interrupt"),
    ("USBPD_WKUP_IRQ", 50, "USBPD WakeUp interrupt"),
    ("TIM2_CC_IRQ", 51, "TIM2 Capture Compare interrupt"),
    (
        "TIM2_TRG_COM_IRQ",
        52,
        "TIM2 Trigger and Commutation interrupt",
    ),
    ("TIM2_BRK_IRQ", 53, "TIM2 Break interrupt"),
    ("TIM3_IRQ", 54, "TIM3 global interrupt"),
];

/// First external interrupt vector slot (core exceptions occupy 0..=15).
const EXTERNAL_IRQ_BASE: u32 = 16;

/// Peripherals that are core (RT-owned) and must be excluded from the PAC.
///
/// The PFIC interrupt controller and SysTick are handled by `ch32x035-rt`
/// (mirroring how the RP2350 PAC excludes NVIC/SCB/SysTick).
const SKIP_PERIPHERALS: &[&str] = &["PFIC"];

/// Generate all modules
pub fn generate_all(device: &SvdDevice, output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir)?;

    let periph_map: HashMap<_, _> = device
        .peripherals
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();

    // Generate common module
    fs::write(output_dir.join("common.rs"), generate_common())?;
    println!("  Generated common.rs");

    // Collect peripherals to generate (skip core / RT-owned peripherals)
    let peripherals: Vec<_> = device
        .peripherals
        .iter()
        .filter(|p| p.base_address.is_some())
        .filter(|p| !SKIP_PERIPHERALS.contains(&p.name.as_str()))
        .collect();

    // Generate each peripheral
    for periph in &peripherals {
        let content = generate_peripheral(periph, &periph_map)?;
        let filename = format!("{}.rs", periph.module_name());
        fs::write(output_dir.join(&filename), content)?;
        println!("  Generated {}", filename);
    }

    // Generate mod.rs
    let mod_content = generate_mod_rs(device, &peripherals);
    fs::write(output_dir.join("mod.rs"), mod_content)?;
    println!("  Generated mod.rs");

    Ok(())
}

/// Build the canonical interrupt list, merging SVD descriptions where available.
fn interrupt_list(device: &SvdDevice) -> Vec<(Interrupt, String)> {
    // Map SVD interrupts by value for rich descriptions.
    let mut svd_by_value: HashMap<u32, &Interrupt> = HashMap::new();
    for periph in &device.peripherals {
        for int in &periph.interrupts {
            svd_by_value.entry(int.value).or_insert(int);
        }
    }

    CH32X035_INTERRUPTS
        .iter()
        .map(|(name, value, desc)| {
            let description = svd_by_value
                .get(value)
                .map(|i| short_desc(&i.description))
                .filter(|d| !d.is_empty())
                .unwrap_or_else(|| desc.to_string());
            (
                Interrupt {
                    name: name.to_string(),
                    value: *value,
                    description,
                },
                String::new(),
            )
        })
        .collect()
}

fn generate_mod_rs(device: &SvdDevice, peripherals: &[&Peripheral]) -> String {
    let mut out = String::new();

    writeln!(out, "//! CH32X035 Hardware Register Definitions").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(
        out,
        "//! Auto-generated from ch32x035.svd - DO NOT EDIT MANUALLY"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "#![allow(dead_code)]").unwrap();
    writeln!(out, "#![allow(non_camel_case_types)]").unwrap();
    writeln!(out, "#![allow(clippy::identity_op)]").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "use core::marker::PhantomData;").unwrap();
    writeln!(out, "use core::ops::Deref;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "pub mod common;").unwrap();
    writeln!(out).unwrap();

    let all_interrupts = interrupt_list(device);

    // Generate interrupt handlers
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Interrupt Handlers").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "unsafe extern \"C\" {{").unwrap();
    for (int, _) in &all_interrupts {
        writeln!(out, "    fn {}();", int.name.to_shouty_snake_case()).unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/// Union type for interrupt vectors").unwrap();
    writeln!(out, "#[doc(hidden)]").unwrap();
    writeln!(out, "#[repr(C)]").unwrap();
    writeln!(out, "pub union Vector {{").unwrap();
    writeln!(out, "    _handler: unsafe extern \"C\" fn(),").unwrap();
    writeln!(out, "    _reserved: u32,").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    let first = EXTERNAL_IRQ_BASE;
    // The vector table spans every slot from the first external interrupt to
    // the highest-numbered one (including reserved gaps such as slot 19).
    let last = all_interrupts
        .iter()
        .map(|(int, _)| int.value)
        .max()
        .unwrap_or(first);
    let count = (last - first + 1) as usize;
    writeln!(
        out,
        "/// CH32X035 interrupt vector table (external interrupts {}..={}, {} entries)",
        first, last, count
    )
    .unwrap();
    writeln!(out, "///").unwrap();
    writeln!(
        out,
        "/// Placed after the core exception vectors (slots 0..=15) in flash."
    )
    .unwrap();
    writeln!(out, "#[doc(hidden)]").unwrap();
    writeln!(
        out,
        "#[unsafe(link_section = \".vector_table.interrupts\")]"
    )
    .unwrap();
    writeln!(out, "#[unsafe(no_mangle)]").unwrap();
    writeln!(out, "pub static __INTERRUPTS: [Vector; {}] = [", count).unwrap();

    let int_map: HashMap<_, _> = all_interrupts
        .iter()
        .map(|(int, _)| (int.value, int))
        .collect();
    for i in 0..count as u32 {
        let value = first + i;
        if let Some(int) = int_map.get(&value) {
            writeln!(
                out,
                "    Vector {{ _handler: {} }}, // {}",
                int.name.to_shouty_snake_case(),
                value
            )
            .unwrap();
        } else {
            writeln!(out, "    Vector {{ _reserved: 0 }}, // {} reserved", value).unwrap();
        }
    }
    writeln!(out, "];").unwrap();
    writeln!(out).unwrap();

    // Generate Interrupt enum
    writeln!(out, "/// CH32X035 Interrupt enumeration").unwrap();
    writeln!(out, "#[derive(Copy, Clone, Debug, PartialEq, Eq)]").unwrap();
    writeln!(out, "#[repr(u16)]").unwrap();
    writeln!(out, "pub enum Interrupt {{").unwrap();
    for (int, _) in &all_interrupts {
        let desc = if int.description.is_empty() {
            format!("Interrupt {}", int.value)
        } else {
            int.description.clone()
        };
        writeln!(out, "    /// {} - {}", int.value, desc).unwrap();
        writeln!(
            out,
            "    {} = {},",
            int.name.to_shouty_snake_case(),
            int.value
        )
        .unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "unsafe impl ch32x035_rt::peripheral::pfic::InterruptNumber for Interrupt {{"
    )
    .unwrap();
    writeln!(out, "    #[inline(always)]").unwrap();
    writeln!(out, "    fn number(self) -> u16 {{").unwrap();
    writeln!(out, "        self as u16").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Generate peripheral structs
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripheral Wrapper Macro").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/// Helper macro to create peripheral wrappers").unwrap();
    writeln!(out, "macro_rules! peripheral {{").unwrap();
    writeln!(out, "    ($name:ident, $reg:ty, $base:expr) => {{").unwrap();
    writeln!(out, "        impl $name {{").unwrap();
    writeln!(
        out,
        "            /// Pointer to the peripheral's register block"
    )
    .unwrap();
    writeln!(
        out,
        "            pub const PTR: *const $reg = $base as *const $reg;"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "            /// Steal an instance of this peripheral").unwrap();
    writeln!(out, "            ///").unwrap();
    writeln!(out, "            /// # Safety").unwrap();
    writeln!(out, "            ///").unwrap();
    writeln!(
        out,
        "            /// Ensure that the new instance of the peripheral cannot be used in a way"
    )
    .unwrap();
    writeln!(
        out,
        "            /// that may race with any existing instances."
    )
    .unwrap();
    writeln!(out, "            #[inline]").unwrap();
    writeln!(out, "            pub const unsafe fn steal() -> Self {{").unwrap();
    writeln!(out, "                Self {{").unwrap();
    writeln!(out, "                    _marker: PhantomData,").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        impl Deref for $name {{").unwrap();
    writeln!(out, "            type Target = $reg;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "            #[inline(always)]").unwrap();
    writeln!(out, "            fn deref(&self) -> &Self::Target {{").unwrap();
    writeln!(out, "                unsafe {{ &*Self::PTR }}").unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        impl core::fmt::Debug for $name {{").unwrap();
    writeln!(
        out,
        "            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {{"
    )
    .unwrap();
    writeln!(
        out,
        "                f.debug_struct(stringify!($name)).finish()"
    )
    .unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }};").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripheral Definitions").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    for periph in peripherals {
        let struct_name = periph.name.to_shouty_snake_case();
        let mod_name = periph.module_name();
        let base_addr = periph.base_address.unwrap();

        if !periph.description.is_empty() {
            writeln!(out, "/// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "/// {} peripheral", periph.name).unwrap();
        }
        writeln!(out, "pub struct {} {{", struct_name).unwrap();
        writeln!(out, "    _marker: PhantomData<*const ()>,").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "unsafe impl Send for {} {{}}", struct_name).unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "peripheral!({}, {}::RegisterBlock, {:#010x});",
            struct_name, mod_name, base_addr
        )
        .unwrap();
        writeln!(out).unwrap();
        if !periph.description.is_empty() {
            writeln!(out, "/// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "/// {} register module", periph.name).unwrap();
        }
        writeln!(out, "pub mod {};", mod_name).unwrap();
        writeln!(out).unwrap();
    }

    // Generate Peripherals struct
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out, "// Peripherals - All device peripherals").unwrap();
    writeln!(
        out,
        "// ============================================================================"
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "use core::sync::atomic::{{AtomicBool, Ordering}};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "/// Guard to ensure peripherals are taken only once").unwrap();
    writeln!(out, "static TAKEN: AtomicBool = AtomicBool::new(false);").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/// All the peripherals.").unwrap();
    writeln!(out, "#[allow(non_snake_case)]").unwrap();
    writeln!(out, "pub struct Peripherals {{").unwrap();
    for periph in peripherals {
        let struct_name = periph.name.to_shouty_snake_case();
        if !periph.description.is_empty() {
            writeln!(out, "    /// {}", short_desc(&periph.description)).unwrap();
        } else {
            writeln!(out, "    /// {} peripheral", periph.name).unwrap();
        }
        writeln!(out, "    pub {}: {},", struct_name, struct_name).unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "impl Peripherals {{").unwrap();
    writeln!(out, "    /// Returns all the peripherals *once*.").unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub fn take() -> Option<Self> {{").unwrap();
    writeln!(out, "        if TAKEN.swap(true, Ordering::AcqRel) {{").unwrap();
    writeln!(out, "            None").unwrap();
    writeln!(out, "        }} else {{").unwrap();
    writeln!(out, "            Some(unsafe {{ Self::steal() }})").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    /// Unchecked version of `Peripherals::take`.").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(out, "    /// # Safety").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(
        out,
        "    /// Each of the returned peripherals must be used at most once."
    )
    .unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub unsafe fn steal() -> Self {{").unwrap();
    // Rust 2024: `unsafe fn` bodies are not implicit unsafe blocks, so the
    // inner `unsafe {}` is required for the `X::steal()` calls below
    // (see the `unsafe-op-in-unsafe-fn` lint).
    writeln!(out, "        unsafe {{").unwrap();
    writeln!(out, "            Peripherals {{").unwrap();
    for periph in peripherals {
        let struct_name = periph.name.to_shouty_snake_case();
        writeln!(
            out,
            "                {}: {}::steal(),",
            struct_name, struct_name
        )
        .unwrap();
    }
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    /// Release the peripherals back to the pool").unwrap();
    writeln!(out, "    ///").unwrap();
    writeln!(
        out,
        "    /// After calling this, `take()` will succeed again."
    )
    .unwrap();
    writeln!(out, "    #[inline]").unwrap();
    writeln!(out, "    pub fn release(self) {{").unwrap();
    writeln!(out, "        TAKEN.store(false, Ordering::Release);").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

fn generate_common() -> String {
    r##"//! Common types and utilities for register access

#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// A volatile cell for memory-mapped register access.
///
/// Similar to `vcell::VolatileCell`, this provides volatile read/write access
/// to a memory location with proper semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for VolatileCell<T> {}
unsafe impl<T> Sync for VolatileCell<T> {}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> VolatileCell<T> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> T {
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// Spin until condition is true
#[inline(always)]
pub fn wait_until<F: Fn() -> bool>(cond: F) {
    while !cond() {
        core::hint::spin_loop();
    }
}

/// Spin while condition is true
#[inline(always)]
pub fn wait_while<F: Fn() -> bool>(cond: F) {
    while cond() {
        core::hint::spin_loop();
    }
}
"##
    .to_string()
}

/// The CH32X035 has no RP2350-style atomic SET/CLR/XOR register aliases, so
/// atomic support is always disabled and every register uses a `VolatileCell`.
fn supports_atomic_aliases(_base_address: u32) -> bool {
    false
}

fn generate_peripheral(
    periph: &Peripheral,
    periph_map: &HashMap<&str, &Peripheral>,
) -> Result<String> {
    let registers = if let Some(ref derived) = periph.derived_from {
        if let Some(base) = periph_map.get(derived.as_str()) {
            &base.registers
        } else {
            &periph.registers
        }
    } else {
        &periph.registers
    };

    generate_generic(periph, registers)
}

fn generate_generic(periph: &Peripheral, registers: &[Register]) -> Result<String> {
    let mut out = String::new();

    let supports_atomic = periph
        .base_address
        .map(supports_atomic_aliases)
        .unwrap_or(false);

    write_header(&mut out, periph, supports_atomic)?;

    // Generate each register struct with field accessors
    for reg in registers {
        write_register_struct(&mut out, reg, supports_atomic)?;
        // Generate the value struct for batch read/write operations
        write_register_value_struct(&mut out, reg)?;
    }

    // Generate RegisterBlock
    write_register_block(&mut out, &periph.name, registers, supports_atomic)?;

    Ok(out)
}

/// Generate a single register struct with all field accessors
fn write_register_struct(out: &mut String, reg: &Register, supports_atomic: bool) -> Result<()> {
    let struct_name = reg.name.to_upper_camel_case();

    // Document the register
    writeln!(out, "/// {} Register", reg.name)?;
    if !reg.description.is_empty() {
        writeln!(out, "///")?;
        for line in clean_desc(&reg.description).lines() {
            writeln!(out, "/// {}", line)?;
        }
    }
    writeln!(out, "///")?;
    writeln!(out, "/// Address offset: {:#06x}", reg.offset)?;
    if let Some(reset_val) = reg.reset_value {
        writeln!(out, "/// Reset value: {:#010x}", reset_val)?;
    }

    // Document fields
    if !reg.fields.is_empty() {
        writeln!(out, "///")?;
        writeln!(out, "/// ## Fields")?;
        writeln!(out, "///")?;
        for (i, field) in reg.fields.iter().enumerate() {
            if i > 0 {
                writeln!(out, "///")?;
            }
            let access_str = match field.access {
                Access::ReadOnly => "RO",
                Access::WriteOnly => "WO",
                Access::ReadWrite => "RW",
            };
            let bit_range = if field.lsb == field.msb {
                format!("`[{}]`", field.lsb)
            } else {
                format!("`[{}:{}]`", field.msb, field.lsb)
            };
            // First line: field name, bit range, access type
            writeln!(
                out,
                "/// - **{}** {} ({})",
                field.name, bit_range, access_str
            )?;
            // Description lines (preserve original line breaks)
            if !field.description.is_empty() {
                writeln!(out, "///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "///   {}", line)?;
                }
            }
        }
    }

    // The CH32X035 has no atomic register aliases, so every register (except
    // read-only ones) uses a VolatileCell. set/clr accessors therefore do a
    // plain read-modify-write.
    let writable = !matches!(reg.access, Access::ReadOnly);
    let cell_type = if supports_atomic && writable {
        "crate::common::AtomicCell<u32>"
    } else {
        "crate::common::VolatileCell<u32>"
    };

    writeln!(out, "#[repr(transparent)]")?;
    writeln!(out, "pub struct {} {{", struct_name)?;
    writeln!(out, "    value: {},", cell_type)?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl core::ops::Deref for {} {{", struct_name)?;
    writeln!(out, "    type Target = {};", cell_type)?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    fn deref(&self) -> &Self::Target {{")?;
    writeln!(out, "        &self.value")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl {} {{", struct_name)?;

    // Generate field constants
    if !reg.fields.is_empty() {
        writeln!(out, "    // ======== Field Constants ========")?;
        writeln!(out)?;
        for field in &reg.fields {
            let field_shouty = field.name.to_shouty_snake_case();

            // Field description (preserve original line breaks)
            writeln!(out, "    /// **{}**", field.name)?;
            if !field.description.is_empty() {
                writeln!(out, "    ///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "    /// {}", line)?;
                }
                writeln!(out, "    ///")?;
            }
            writeln!(
                out,
                "    /// Bits `[{}:{}]`, mask `{:#010x}`",
                field.msb,
                field.lsb,
                field.mask()
            )?;
            writeln!(
                out,
                "    pub const {}_OFFSET: u32 = {};",
                field_shouty, field.lsb
            )?;
            writeln!(
                out,
                "    pub const {}_MASK: u32 = {:#010x};",
                field_shouty,
                field.mask()
            )?;

            // For single-bit fields, add a simple bit constant
            if field.width() == 1 {
                writeln!(
                    out,
                    "    pub const {}: u32 = 1 << {};",
                    field_shouty, field.lsb
                )?;
            }

            // Enum values - generate both shifted (for direct register access) and unshifted (for xxxVal)
            for ev in &field.enum_values {
                let ev_name = ev.name.to_shouty_snake_case();
                if !ev.description.is_empty() {
                    writeln!(out, "    /// {} = {}", ev.name, ev.value)?;
                    for line in clean_desc(&ev.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                // Shifted value - for direct register write via Basic Access Methods
                writeln!(
                    out,
                    "    pub const {}_{}: u32 = {} << {};",
                    field_shouty, ev_name, ev.value, field.lsb
                )?;
                // Unshifted value - for use with xxxVal's with_xxx() methods
                writeln!(
                    out,
                    "    pub const {}_{}_VAL: u32 = {};",
                    field_shouty, ev_name, ev.value
                )?;
            }
            writeln!(out)?;
        }
    }

    // Generate field accessor methods
    if !reg.fields.is_empty() {
        writeln!(out, "    // ======== Field Accessors ========")?;
        writeln!(out)?;

        for field in &reg.fields {
            let field_snake = safe_snake_case(&field.name);
            let field_shouty = field.name.to_shouty_snake_case();
            let is_readable = matches!(field.access, Access::ReadOnly | Access::ReadWrite);
            let width = field.width();

            // Check if we will generate any method for this field
            let will_generate_read = is_readable;
            // RW multi-bit field in RW register -> write_xxx method using modify
            let will_generate_write = matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width > 1;
            // RW single-bit field in RW register -> set_xxx/clr_xxx methods
            let will_generate_set_clr = matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width == 1;
            // WriteOnly single-bit field -> trigger_xxx method
            let will_generate_trigger = matches!(field.access, Access::WriteOnly) && width == 1;
            // WriteOnly multi-bit field -> write_xxx method
            let will_generate_write_only = matches!(field.access, Access::WriteOnly) && width > 1;

            // Skip if no methods will be generated
            if !will_generate_read
                && !will_generate_write
                && !will_generate_set_clr
                && !will_generate_trigger
                && !will_generate_write_only
            {
                continue;
            }

            // Access string for documentation
            let access_str = match field.access {
                Access::ReadOnly => "RO",
                Access::WriteOnly => "WO",
                Access::ReadWrite => "RW",
            };

            // Bit range string
            let bit_range = if field.lsb == field.msb {
                format!("`[{}]`", field.lsb)
            } else {
                format!("`[{}:{}]`", field.msb, field.lsb)
            };

            // Document the field (only once, before the first method)
            // Skip for WriteOnly fields as they'll be documented with their trigger/write methods
            if !matches!(field.access, Access::WriteOnly) {
                writeln!(
                    out,
                    "    /// **{}** {} ({})",
                    field.name, bit_range, access_str
                )?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
            }

            // Read accessor (for readable fields)
            if is_readable {
                if width == 1 {
                    // Single bit - return bool
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(out, "    pub fn {}(&self) -> bool {{", field_snake)?;
                    writeln!(
                        out,
                        "        (self.value.read_bits() & Self::{}_MASK) != 0",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                } else {
                    // Multi-bit - return u32
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(out, "    pub fn {}(&self) -> u32 {{", field_snake)?;
                    writeln!(
                        out,
                        "        (self.value.read_bits() & Self::{}_MASK) >> Self::{}_OFFSET",
                        field_shouty, field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                }
                writeln!(out)?;
            }

            // Write accessor (for RW single-bit fields in RW register)
            if matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width == 1
            {
                if supports_atomic {
                    // Use atomic set/clr methods (generated on the register struct)
                    writeln!(out, "    /// Set {} bit atomically (RW)", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("set", &field.name)
                    )?;
                    writeln!(out, "        self.set_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;

                    writeln!(out, "    /// Clear {} bit atomically (RW)", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("clr", &field.name)
                    )?;
                    writeln!(out, "        self.clr_bits(Self::{}_MASK);", field_shouty)?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                } else {
                    // No atomic support - use read-modify-write
                    writeln!(out, "    /// Set {} bit (RW) - NOT atomic", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("set", &field.name)
                    )?;
                    writeln!(
                        out,
                        "        self.value.modify_bits(|r| r | Self::{}_MASK);",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;

                    writeln!(out, "    /// Clear {} bit (RW) - NOT atomic", field.name)?;
                    writeln!(out, "    #[inline(always)]")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) {{",
                        prefixed_method_name("clr", &field.name)
                    )?;
                    writeln!(
                        out,
                        "        self.value.modify_bits(|r| r & !Self::{}_MASK);",
                        field_shouty
                    )?;
                    writeln!(out, "    }}")?;
                    writeln!(out)?;
                }
            }

            // Write accessor (for RW multi-bit fields in RW register) - use modify
            if matches!(field.access, Access::ReadWrite)
                && matches!(reg.access, Access::ReadWrite)
                && width > 1
            {
                writeln!(out, "    /// Write **{}** {} (RW)", field.name, bit_range)?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                writeln!(out, "    #[inline(always)]")?;
                writeln!(
                    out,
                    "    pub fn {}(&self, value: u32) {{",
                    prefixed_method_name("write", &field.name)
                )?;
                writeln!(
                    out,
                    "        self.value.modify_bits(|r| (r & !Self::{}_MASK) | ((value << Self::{}_OFFSET) & Self::{}_MASK));",
                    field_shouty, field_shouty, field_shouty
                )?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
            }

            // WriteOnly fields - trigger method (single bit)
            // For peripherals without atomic support, use write_bits/modify_bits
            if matches!(field.access, Access::WriteOnly) && width == 1 {
                writeln!(out, "    /// Trigger **{}** (WO)", field.name)?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                writeln!(out, "    #[inline(always)]")?;
                writeln!(
                    out,
                    "    pub fn {}(&self) {{",
                    prefixed_method_name("trigger", &field.name)
                )?;
                if supports_atomic {
                    writeln!(out, "        self.set_bits(Self::{}_MASK);", field_shouty)?;
                } else if matches!(reg.access, Access::WriteOnly) {
                    // WO register: cannot read, use direct write
                    writeln!(
                        out,
                        "        self.value.write_bits(Self::{}_MASK);",
                        field_shouty
                    )?;
                } else {
                    writeln!(
                        out,
                        "        self.value.modify_bits(|r| r | Self::{}_MASK);",
                        field_shouty
                    )?;
                }
                writeln!(out, "    }}")?;
                writeln!(out)?;
            }

            // WriteOnly fields - write method (multi-bit)
            if matches!(field.access, Access::WriteOnly) && width > 1 {
                writeln!(out, "    /// Write **{}** {} (WO)", field.name, bit_range)?;
                if !field.description.is_empty() {
                    writeln!(out, "    ///")?;
                    for line in clean_desc(&field.description).lines() {
                        writeln!(out, "    /// {}", line)?;
                    }
                }
                writeln!(out, "    #[inline(always)]")?;
                writeln!(
                    out,
                    "    pub fn {}(&self, value: u32) {{",
                    prefixed_method_name("write", &field.name)
                )?;
                writeln!(
                    out,
                    "        self.value.write_bits((value << Self::{}_OFFSET) & Self::{}_MASK);",
                    field_shouty, field_shouty
                )?;
                writeln!(out, "    }}")?;
                writeln!(out)?;
            }
        }
    }

    // Note: For peripherals that support atomic operations, the AtomicCell type
    // provides set_bits/clr_bits/xor_bits via Deref

    trim_trailing_blank_line(out);
    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

/// Generate the value struct for batch read/write operations
/// This allows reading/writing the entire register once and manipulating fields in memory
fn write_register_value_struct(out: &mut String, reg: &Register) -> Result<()> {
    let struct_name = reg.name.to_upper_camel_case();
    let val_name = format!("{}Val", struct_name);

    // Only generate for registers with fields
    if reg.fields.is_empty() {
        return Ok(());
    }

    writeln!(
        out,
        "/// {} value type for batch read/modify/write operations",
        reg.name
    )?;
    writeln!(out, "///")?;
    writeln!(
        out,
        "/// Use `{}.read_val()` to read the register into this struct,",
        struct_name.to_lowercase()
    )?;
    writeln!(
        out,
        "/// modify fields with builder methods, then write back with `{}.write_val(val)`.",
        struct_name.to_lowercase()
    )?;
    writeln!(out, "#[derive(Clone, Copy)]")?;
    writeln!(out, "pub struct {} {{", val_name)?;
    writeln!(out, "    bits: u32,")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "impl {} {{", val_name)?;

    // Constructor from raw bits
    writeln!(out, "    /// Create from raw bits")?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    pub const fn from_bits(bits: u32) -> Self {{")?;
    writeln!(out, "        Self {{ bits }}")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;

    // Get raw bits
    writeln!(out, "    /// Get the raw bits")?;
    writeln!(out, "    #[inline(always)]")?;
    writeln!(out, "    pub const fn bits(&self) -> u32 {{")?;
    writeln!(out, "        self.bits")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;

    // Default (reset value or zero)
    // Check if any field is named "reset" to avoid method name collision
    let has_reset_field = reg.fields.iter().any(|f| f.name.to_lowercase() == "reset");

    if let Some(reset_val) = reg.reset_value
        && !has_reset_field
    {
        writeln!(out, "    /// Create with reset value")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub const fn reset_val() -> Self {{")?;
        writeln!(out, "        Self {{ bits: {:#010x} }}", reset_val)?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    // Field reader methods
    for field in &reg.fields {
        let field_snake = safe_snake_case(&field.name);
        let field_shouty = field.name.to_shouty_snake_case();
        let is_readable = matches!(field.access, Access::ReadOnly | Access::ReadWrite);
        let width = field.width();

        if is_readable {
            writeln!(
                out,
                "    /// Read **{}** `[{}:{}]`",
                field.name, field.msb, field.lsb
            )?;
            if !field.description.is_empty() {
                writeln!(out, "    ///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "    /// {}", line)?;
                }
            }
            writeln!(out, "    #[inline(always)]")?;
            if width == 1 {
                writeln!(out, "    pub const fn {}(&self) -> bool {{", field_snake)?;
                writeln!(
                    out,
                    "        (self.bits & {}::{}_MASK) != 0",
                    struct_name, field_shouty
                )?;
                writeln!(out, "    }}")?;
            } else {
                writeln!(out, "    pub const fn {}(&self) -> u32 {{", field_snake)?;
                writeln!(
                    out,
                    "        (self.bits & {}::{}_MASK) >> {}::{}_OFFSET",
                    struct_name, field_shouty, struct_name, field_shouty
                )?;
                writeln!(out, "    }}")?;
            }
            writeln!(out)?;
        }
    }

    // Field builder methods (for writable fields)
    for field in &reg.fields {
        let field_shouty = field.name.to_shouty_snake_case();
        let is_writable = matches!(field.access, Access::WriteOnly | Access::ReadWrite);
        let width = field.width();
        let with_method = prefixed_method_name("with", &field.name);

        if is_writable {
            writeln!(
                out,
                "    /// Set **{}** `[{}:{}]`",
                field.name, field.msb, field.lsb
            )?;
            if !field.description.is_empty() {
                writeln!(out, "    ///")?;
                for line in clean_desc(&field.description).lines() {
                    writeln!(out, "    /// {}", line)?;
                }
            }
            writeln!(out, "    #[inline(always)]")?;
            if width == 1 {
                writeln!(
                    out,
                    "    pub const fn {}(self, value: bool) -> Self {{",
                    with_method
                )?;
                writeln!(out, "        Self {{")?;
                writeln!(out, "            bits: if value {{")?;
                writeln!(
                    out,
                    "                self.bits | {}::{}_MASK",
                    struct_name, field_shouty
                )?;
                writeln!(out, "            }} else {{")?;
                writeln!(
                    out,
                    "                self.bits & !{}::{}_MASK",
                    struct_name, field_shouty
                )?;
                writeln!(out, "            }}")?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
            } else {
                writeln!(
                    out,
                    "    pub const fn {}(self, value: u32) -> Self {{",
                    with_method
                )?;
                writeln!(out, "        Self {{")?;
                writeln!(
                    out,
                    "            bits: (self.bits & !{}::{}_MASK) | ((value << {}::{}_OFFSET) & {}::{}_MASK)",
                    struct_name, field_shouty, struct_name, field_shouty, struct_name, field_shouty
                )?;
                writeln!(out, "        }}")?;
                writeln!(out, "    }}")?;
            }
            writeln!(out)?;
        }
    }

    trim_trailing_blank_line(out);
    writeln!(out, "}}")?;
    writeln!(out)?;

    // Default impl
    writeln!(out, "impl Default for {} {{", val_name)?;
    writeln!(out, "    fn default() -> Self {{")?;
    if reg.reset_value.is_some() && !has_reset_field {
        writeln!(out, "        Self::reset_val()")?;
    } else if let Some(reset_val) = reg.reset_value {
        writeln!(out, "        Self {{ bits: {:#010x} }}", reset_val)?;
    } else {
        writeln!(out, "        Self {{ bits: 0 }}")?;
    }
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    // Add read_val/write_val/modify_val methods that use XxxVal to the register struct
    writeln!(out, "impl {} {{", struct_name)?;

    if matches!(reg.access, Access::ReadOnly | Access::ReadWrite) {
        writeln!(out, "    /// Read the register into a value struct")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn read_val(&self) -> {} {{", val_name)?;
        writeln!(
            out,
            "        {}::from_bits(self.value.read_bits())",
            val_name
        )?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    if matches!(reg.access, Access::WriteOnly | Access::ReadWrite) {
        writeln!(out, "    /// Write a value struct to the register")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(out, "    pub fn write_val(&self, val: {}) {{", val_name)?;
        writeln!(out, "        self.value.write_bits(val.bits())")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    if matches!(reg.access, Access::ReadWrite) {
        writeln!(out, "    /// Modify the register using a value struct")?;
        writeln!(out, "    #[inline(always)]")?;
        writeln!(
            out,
            "    pub fn modify_val<F: FnOnce({}) -> {}>(&self, f: F) {{",
            val_name, val_name
        )?;
        writeln!(out, "        self.write_val(f(self.read_val()))")?;
        writeln!(out, "    }}")?;
        writeln!(out)?;
    }

    trim_trailing_blank_line(out);
    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

/// Generate the RegisterBlock struct
fn write_register_block(
    out: &mut String,
    _periph_name: &str,
    registers: &[Register],
    _supports_atomic: bool,
) -> Result<()> {
    writeln!(out, "/// Register block")?;
    writeln!(out, "#[repr(C)]")?;
    writeln!(out, "pub struct RegisterBlock {{")?;

    let mut sorted_regs: Vec<_> = registers.iter().collect();
    sorted_regs.sort_by_key(|r| r.offset);

    let mut prev_end: u32 = 0;
    let mut padding_idx = 0;

    for reg in &sorted_regs {
        if reg.offset > prev_end {
            let gap = reg.offset - prev_end;
            writeln!(out, "    _reserved{}: [u8; {}],", padding_idx, gap)?;
            padding_idx += 1;
        }

        let field_name = reg.name.to_snake_case();
        let type_name = reg.name.to_upper_camel_case();

        writeln!(out, "    /// {:#06x}", reg.offset)?;
        if !reg.description.is_empty() {
            for line in clean_desc(&reg.description).lines() {
                writeln!(out, "    /// {}", line)?;
            }
        }
        writeln!(out, "    pub {}: {},", field_name, type_name)?;

        prev_end = reg.offset + 4;
    }

    writeln!(out, "}}")?;
    writeln!(out)?;

    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn escape_keyword(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

/// Safe snake_case for standalone identifiers (e.g., field reads)
fn safe_snake_case(name: &str) -> String {
    let snake = name.to_snake_case();
    escape_keyword(&snake)
}

/// Build prefixed method name, handling keyword escaping properly
fn prefixed_method_name(prefix: &str, field_name: &str) -> String {
    let base = field_name.to_snake_case();
    let combined = format!("{}_{}", prefix, base);
    // If the base name is a keyword, append underscore rather than using r#
    if RUST_KEYWORDS.contains(&base.as_str()) {
        format!("{}_", combined)
    } else {
        combined
    }
}

fn write_header(out: &mut String, periph: &Peripheral, supports_atomic: bool) -> Result<()> {
    writeln!(out, "//! {}", periph.name)?;
    if !periph.description.is_empty() {
        // Only include first few meaningful lines of description
        // Skip configuration constant lists and boilerplate
        let desc = clean_desc(&periph.description);
        let meaningful_lines: Vec<&str> = desc
            .lines()
            .filter(|line| {
                // Skip lines that indicate a list of constants follows
                !line.contains("List of configuration") && !line.contains("you may see references")
            })
            .take_while(|line| {
                // Stop at configuration constant lists (lines with lots of dots or equals)
                !line.contains("....") && !line.contains(" = ")
            })
            .take(3) // Max 3 lines
            .collect();
        for line in meaningful_lines {
            writeln!(out, "//! {}", line)?;
        }
    }
    if supports_atomic {
        writeln!(out, "//!")?;
        writeln!(
            out,
            "//! This peripheral supports atomic SET/CLR/XOR register aliases."
        )?;
    }
    writeln!(out)?;
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out)?;
    if let Some(addr) = periph.base_address {
        writeln!(out, "pub const BASE: u32 = {:#010x};", addr)?;
        writeln!(out)?;
    }
    Ok(())
}

/// Trim trailing blank line from output buffer.
fn trim_trailing_blank_line(out: &mut String) {
    if out.ends_with("\n\n") {
        out.truncate(out.len() - 1);
    }
}

/// Clean up description: normalize whitespace within each line, preserve original line breaks
fn clean_desc(s: &str) -> String {
    s.lines()
        .map(|line| {
            // Normalize whitespace within each line
            line.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Get the first line of description (for short summaries in mod.rs)
fn short_desc(s: &str) -> String {
    clean_desc(s).lines().next().unwrap_or("").to_string()
}
