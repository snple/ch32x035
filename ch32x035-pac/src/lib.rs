//! Peripheral Access Crate for CH32X035
//!
//! This crate provides low-level access to the hardware registers of the
//! CH32X035 microcontroller (WCH, RISC-V QingKe V4C core).
//!
//! Auto-generated from ch32x035.svd - DO NOT EDIT MANUALLY
//! (regenerate with `cargo xtask gen-pac`)

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::identity_op)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::self_named_constructors)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::private_intra_doc_links)]

// Re-export everything from the generated module
mod generated;
pub use generated::*;
