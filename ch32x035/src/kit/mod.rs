//! Platform-independent utility library.
//!
//! This module groups general-purpose building blocks that do **not** access
//! hardware registers directly. It is the `no_std` equivalent of standard
//! library utilities for the CH32X035 ecosystem, mirroring the rp2350 `kit`.
//!
//! | Sub-module      | Purpose |
//! |-----------------|---------|
//! | [`ring_buffer`] | Fixed-capacity byte ring buffer for interrupt-driven I/O |
//! | [`soft_i2c`]    | Bit-banged I2C master on arbitrary GPIO pins |

pub mod ring_buffer;
pub mod soft_i2c;
