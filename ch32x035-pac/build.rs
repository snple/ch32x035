use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // The file `device.x` is required by ch32x035-rt when using the "rt" feature
    // It provides the interrupt vector table definitions for the CH32X035
    let device_x = include_bytes!("device.x");
    let mut f = File::create(out.join("device.x")).unwrap();
    f.write_all(device_x).unwrap();

    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-changed=build.rs");
}
