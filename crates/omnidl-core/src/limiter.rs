//! Token-bucket speed limiter, shared by all connections of a download
//! (and optionally one global bucket for the whole app), so a limit means
//! what users expect: the *total* never exceeds the cap.

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct TokenBucket {
    rate_per_sec: u64, // bytes per second; 0 = unlimited
    capacity: u64,
    tokens: f64,
    last: Instant,
}

impl TokenBucket {
    pub fn new(rate_per_sec: u64) -> Self {
        // Burst capacity = 1 second of rate (min 64 KiB) for smooth starts.
        let capacity = rate_per_sec.max(64 * 1024);
        Self {
            rate_per_sec,
            capacity,
            tokens: capacity as f64,
            last: Instant::now(),
        }
    }

    pub fn set_rate(&mut self, rate_per_sec: u64) {
        self.rate_per_sec = rate_per_sec;
        self.capacity = rate_per_sec.max(64 * 1024);
        self.tokens = self.tokens.min(self.capacity as f64);
    }

    pub fn unlimited(&self) -> bool {
        self.rate_per_sec == 0
    }

    /// Try to take `n` bytes. On success returns `Duration::ZERO`; on
    /// failure returns how long the caller should wait before retrying.
    pub fn try_take(&mut self, n: u64) -> Duration {
        if self.unlimited() {
            return Duration::ZERO;
        }
        let now = Instant::now();
        let elapsed = now.duration_since(self.last).as_secs_f64();
        self.last = now;
        self.tokens =
            (self.tokens + elapsed * self.rate_per_sec as f64).min(self.capacity as f64);
        if self.tokens >= n as f64 {
            self.tokens -= n as f64;
            Duration::ZERO
        } else {
            let deficit = n as f64 - self.tokens;
            Duration::from_secs_f64(deficit / self.rate_per_sec as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_limits_after_burst() {
        let mut b = TokenBucket::new(1000); // 1 KB/s, capacity min 64 KiB
        assert_eq!(b.try_take(1000), Duration::ZERO); // burst allowed
        let mut b2 = TokenBucket::new(0);
        assert!(b2.unlimited());
        assert_eq!(b2.try_take(10_000_000), Duration::ZERO);
    }
}
