//! Shared code for the CH32X035 examples.
//!
//! The `ch32x035-examples` crate is a `lib` so the example binaries can share
//! common initialization helpers (e.g. a `Board::init()`), mirroring the
//! pico-rs examples layout.

#![no_std]

pub mod board;

/// defmt timestamp implementation.
///
/// The CH32X035 has no cycle-counter timer that defmt can read portably, so
/// we return a fixed value (timestamps are all 0). Override this if you have
/// a monotonic time source.
#[doc(hidden)]
#[unsafe(export_name = "_defmt_timestamp")]
pub fn timestamp() -> u64 {
    0
}
