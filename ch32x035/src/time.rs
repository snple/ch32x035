//! Time types: [`Duration`] and [`Instant`]
//!
//! Pure software time types (no hardware dependency), modeled on the pico-rs
//! `time` module.

/// A span of time, measured in microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Duration {
    micros: u64,
}

impl Duration {
    /// Create a duration from a number of microseconds
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// Create a duration from a number of milliseconds
    pub const fn from_millis(millis: u64) -> Self {
        Self {
            micros: millis * 1000,
        }
    }

    /// Create a duration from a number of seconds
    pub const fn from_secs(secs: u64) -> Self {
        Self {
            micros: secs * 1_000_000,
        }
    }

    /// Create a duration from a frequency in Hz (period = 1/freq)
    pub const fn from_hz(hz: u64) -> Self {
        Self {
            micros: 1_000_000 / hz,
        }
    }

    /// The duration in microseconds
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// The duration in milliseconds (rounded down)
    pub const fn as_millis(&self) -> u64 {
        self.micros / 1000
    }

    /// The duration in seconds (rounded down)
    pub const fn as_secs(&self) -> u64 {
        self.micros / 1_000_000
    }

    /// Checked addition
    pub const fn checked_add(self, rhs: Duration) -> Option<Duration> {
        match self.micros.checked_add(rhs.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// Saturating addition
    pub const fn saturating_add(self, rhs: Duration) -> Duration {
        match self.checked_add(rhs) {
            Some(d) => d,
            None => Duration {
                micros: u64::MAX,
            },
        }
    }

    /// Checked subtraction
    pub const fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        match self.micros.checked_sub(rhs.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// Saturating subtraction
    pub const fn saturating_sub(self, rhs: Duration) -> Duration {
        match self.checked_sub(rhs) {
            Some(d) => d,
            None => Duration { micros: 0 },
        }
    }
}

impl core::ops::Add for Duration {
    type Output = Duration;
    fn add(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros + rhs.micros,
        }
    }
}

impl core::ops::Sub for Duration {
    type Output = Duration;
    fn sub(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros - rhs.micros,
        }
    }
}

impl core::ops::AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.micros += rhs.micros;
    }
}

impl core::ops::SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        self.micros -= rhs.micros;
    }
}

/// An instant in time, measured in microseconds since an arbitrary epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    micros: u64,
}

impl Instant {
    /// Create an instant from a microsecond counter value
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// The instant's microsecond counter value
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// The duration elapsed since this instant
    pub fn elapsed(&self) -> Duration {
        // The CH32X035 is single-core; a global instant source is provided by
        // the caller (e.g. a SysTick tick counter).
        Duration::from_micros(self.micros.saturating_sub(0))
    }

    /// The duration between this instant and a later one
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_micros(self.micros.saturating_sub(earlier.micros))
    }

    /// Checked addition of a duration
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.micros
            .checked_add(duration.micros)
            .map(|micros| Instant { micros })
    }

    /// Checked subtraction of a duration
    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        self.micros
            .checked_sub(duration.micros)
            .map(|micros| Instant { micros })
    }
}

impl core::ops::Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Instant {
        Instant {
            micros: self.micros + rhs.micros,
        }
    }
}

impl core::ops::Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Instant {
        Instant {
            micros: self.micros - rhs.micros,
        }
    }
}

impl core::ops::Sub for Instant {
    type Output = Duration;
    fn sub(self, rhs: Instant) -> Duration {
        Duration::from_micros(self.micros - rhs.micros)
    }
}
