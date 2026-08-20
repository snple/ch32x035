//! Build script for `ch32x035-rtos`.
//!
//! Generates the small linker script fragment `rtos.x` (defining the
//! `.rtos.tasks` section used by the `#[task]` macro to collect task
//! descriptors) and exposes it to the linker via `rustc-link-search`.
//!
//! Binaries that use the kernel must pass `-Trtos.x` to the linker (see
//! `ch32x035-examples/build.rs`), mirroring how `defmt.x` is linked.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=rtos.x");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("rtos.x"), include_str!("rtos.x")).expect("write rtos.x");

    println!("cargo:rustc-link-search={}", out_dir.display());
}
