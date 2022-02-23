use std::ops::{Add, Sub};
use std::time::{Duration};

pub struct CountdownTimer {
    duration: Duration,
    current_duration: Duration,
    paused: bool,
}

impl CountdownTimer {
    pub fn new(duration: Duration) -> CountdownTimer {
        CountdownTimer {
            duration,
            current_duration: Duration::from_secs(0),
            paused: false,
        }
    }

    pub fn advance(&mut self, time_passed: Duration) {
        if !self.is_finished() && !self.paused {
            self.current_duration = self.current_duration.add(time_passed)
        }
    }

    pub fn pause(&mut self) {
        self.paused = true
    }

    pub fn unpause(&mut self) {
        self.paused = false
    }

    pub fn is_paused(&self) -> &bool {
        &self.paused
    }

    pub fn is_finished(&self) -> bool {
        self.current_duration >= self.duration
    }

    pub fn restart(&mut self) {
        self.current_duration = Duration::from_secs(0)
    }

    pub fn get_percent_complete(&self) -> f32 {
        ((self.current_duration.as_nanos() as f64 / self.duration.as_nanos() as f64) as f32).min(1.0)
    }

    pub fn has_started_running(&self) -> bool {
        !self.current_duration.is_zero()
    }

    pub fn get_duration_left(&self) -> Duration {
        self.duration.sub(self.current_duration)
    }
}