use std::ops::Add;
use std::time::{Duration, Instant};

pub struct CountdownTimer {
    duration: Duration,
    current_duration: Duration
}

impl CountdownTimer {
    pub fn new(duration: Duration) -> CountdownTimer {
        CountdownTimer {
            duration,
            current_duration: Duration::from_secs(0)
        }
    }

    pub fn advance(&mut self, time_passed: Duration) {
        if !self.is_finished() {
            self.current_duration = self.current_duration.add(time_passed)
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_duration >= self.duration
    }

    pub fn restart(&mut self) {
        self.current_duration = Duration::from_secs(0)
    }
}