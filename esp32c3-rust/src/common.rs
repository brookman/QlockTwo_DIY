use std::thread;

use chrono::Duration;
use esp_idf_sys::esp_timer_get_time;

/// Get time in microseconds (us)
#[inline]
pub fn get_time() -> i64 {
    unsafe {
        return esp_timer_get_time();
    };
}

pub fn thread_sleep(sleep_duration: Duration) {
    let micros = sleep_duration.num_microseconds().unwrap_or(0);
    if micros <= 0 {
        return;
    }
    thread::sleep(std::time::Duration::from_micros(micros as u64));
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct SimpleTime {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Timestamp {
    pub time: i64,
}

impl Timestamp {
    pub fn new(time: i64) -> Self {
        Timestamp {
            time
        }
    }

    pub fn now() -> Self {
        Timestamp {
            time: get_time()
        }
    }

    pub fn zero() -> Self {
        Timestamp {
            time: 0
        }
    }

    pub fn get_delta(self) -> Duration {
        let now = get_time();
        return Duration::microseconds(now - self.time);
    }

    pub fn update_and_get_delta(&mut self) -> Duration {
        let now = get_time();
        let delta = Duration::microseconds(now - self.time);
        self.time = now;
        return delta;
    }
}