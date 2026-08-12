//! Core CSR registers
//!
//! Value types and accessors for the RISC-V machine-mode CSRs used by the
//! CH32X035 (QingKe V4C) core.

pub mod mcause;
pub mod mepc;
pub mod mhartid;
pub mod mie;
pub mod mscratch;
pub mod mstatus;
pub mod mtvec;
