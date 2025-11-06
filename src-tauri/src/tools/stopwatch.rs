use std::time::{Duration, Instant};

pub struct Stopwatch {
    start_time: Option<Instant>,
    elapsed: Duration,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::ZERO,
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now())
        }
    }

    // WILL BE USED WHEN MULTIPLE CONVERSION ADDED
    /* pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            self.elapsed += start.elapsed();
            self.start_time = None
        }
    } */

    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed = Duration::ZERO
    }

    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start_time {
            self.elapsed + start.elapsed()
        } else {
            self.elapsed
        }
    }
}
