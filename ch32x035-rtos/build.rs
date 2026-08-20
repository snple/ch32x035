//! 构建脚本：把 `rtos.x` 链接脚本片段写入 OUT_DIR 并提供给链接器。
//!
//! 使用内核的可执行目标需要在自己的 build.rs 中加 `-Trtos.x`
//! （见 `ch32x035-examples/build.rs`，与 `defmt.x` 的注入方式一致）。

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=rtos.x");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("rtos.x"), include_str!("rtos.x")).expect("write rtos.x");

    println!("cargo:rustc-link-search={}", out_dir.display());
}
