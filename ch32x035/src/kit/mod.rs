//! Platform-independent utility library.
//!
//! This module groups general-purpose building blocks that do **not** access
//! hardware registers directly. It is the `no_std` equivalent of standard
//! library utilities for the CH32X035 ecosystem, mirroring the rp2350 `kit`.
//!
//! | Sub-module      | Purpose |
//! |-----------------|---------|
//! | [`ring_buffer`] | Fixed-capacity byte ring buffer for interrupt-driven I/O |

pub mod ring_buffer;
