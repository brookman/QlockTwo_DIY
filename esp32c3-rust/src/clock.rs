use chrono::{Duration, NaiveDate, NaiveDateTime, Timelike};
use dcf77::{DCF77Time, SimpleDCF77Decoder};

use crate::common::{SimpleTime, Timestamp};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ClockState {
    Uninitialized,
    Valid,
    Problematic,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct DateTimeReading {
    date_time: NaiveDateTime,
    timestamp: Timestamp,
}

impl DateTimeReading {
    pub fn extrapolated_to_now(&self) -> NaiveDateTime {
        self.date_time + self.timestamp.get_delta()
    }
}

pub struct Clock {
    pub clock_state: ClockState,
    pub estimated_date_time: NaiveDateTime,
    last_date_time_reading: DateTimeReading,
    decoder: SimpleDCF77Decoder,
    last_update: Timestamp,
    last_valid: Timestamp,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            clock_state: ClockState::Uninitialized,
            estimated_date_time: NaiveDateTime::from_timestamp(0, 0),
            last_date_time_reading: DateTimeReading {
                date_time: NaiveDateTime::from_timestamp(0, 0),
                timestamp: Timestamp::zero(),
            },
            decoder: SimpleDCF77Decoder::new(),
            last_update: Timestamp::zero(),
            last_valid: Timestamp::zero(),
        }
    }

    pub fn update(&mut self, current_bit: bool) {
        let delta = self.last_update.update_and_get_delta();
        self.decoder.read_bit(current_bit);
        self.estimated_date_time += delta;

        if self.decoder.bit_complete() {
            print!("[Clock] Bit complete: ");
            if self.decoder.bit_faulty() {
                print!("Faulty");
            } else {
                print!("{}", if self.decoder.latest_bit() { 1 } else { 0 });
                // self.time.seconds = self.decoder.seconds() as u8 % 60;
            }
            println!(", seconds: {:0>2}\n", self.decoder.seconds());

            if self.decoder.seconds() >= 60 {
                println!("Seconds overflow. Resetting the decoder.");
                self.decoder = SimpleDCF77Decoder::new();
            }
        }

        if self.decoder.end_of_cycle() {
            self.update_date_time(DCF77Time::new(self.decoder.raw_data()));
        }
    }

    pub fn get_time(&self) -> SimpleTime {
        SimpleTime {
            hours: self.estimated_date_time.hour() as u8,
            minutes: self.estimated_date_time.minute() as u8,
            seconds: self.estimated_date_time.second() as u8,
        }
    }

    fn update_date_time(&mut self, date_time: DCF77Time) {
        match date_time.validate_start() {
            Ok(_) => {}
            Err(_) => {
                self.on_problem("Start invalid");
                return;
            }
        };
        println!("[Clock] start valid");

        let (year, month, day, _) = match date_time.date() {
            Ok(date) => date,
            Err(_) => {
                self.on_problem("Date invalid");
                return;
            }
        };
        println!("[Clock] date valid");

        let hours = match date_time.hours() {
            Ok(hours) => hours,
            Err(_) => {
                self.on_problem("Hours invalid");
                return;
            }
        };
        println!("[Clock] hours valid");

        let minutes = match date_time.minutes() {
            Ok(minutes) => minutes,
            Err(_) => {
                self.on_problem("Minutes invalid");
                return;
            }
        };
        println!("[Clock] minutes valid");

        let date_time = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
            .and_then(|d| d.and_hms_opt(hours as u32, minutes as u32, 0));

        if date_time == None {
            println!("[Clock] Invalid date/time {}-{}-{} {}:{}", year, month, day, hours, minutes);
            return;
        }

        self.on_valid(DateTimeReading {
            timestamp: Timestamp::now(),
            date_time: date_time.unwrap(),
        });
    }

    pub fn get_time_to_sleep(&self) -> Duration {
        Duration::milliseconds(10) - self.last_update.get_delta()
    }

    fn on_problem(&mut self, problem: &str) {
        println!("[Clock] Problem: {}", problem);
        if self.clock_state == ClockState::Uninitialized {
            return;
        }
        self.clock_state = if self.last_valid.get_delta() > Duration::hours(3) {
            println!("[Clock] Warning: No valid time signal for more than 3 hours.");
            ClockState::Uninitialized
        } else {
            ClockState::Problematic
        };
    }

    fn on_valid(&mut self, reading: DateTimeReading) {
        println!("[Clock] Valid date time reading: {:?}", reading);

        let delta = reading.date_time - self.estimated_date_time;
        println!("[Clock] Delta to internal clock: {} s", delta.num_seconds());

        if self.clock_state == ClockState::Uninitialized || delta.abs() <= Duration::minutes(90) {
            self.set_valid_date_time(reading.date_time);
        } else {
            println!("[Clock] Time reading differs by more than 90 min from internal clock.");
            let delta_to_last_reading = self.last_date_time_reading.extrapolated_to_now() - reading.date_time;

            if delta_to_last_reading.abs() <= Duration::minutes(1) {
                println!("[Clock] But it's consistent with the last reading.");
                self.set_valid_date_time(reading.date_time);
            } else {
                println!("[Clock] Warning: Inconsistent time readings!");
                self.clock_state = ClockState::Problematic;
            }
        }

        self.last_date_time_reading = reading;
    }

    fn set_valid_date_time(&mut self, date_time: NaiveDateTime) {
        println!("[Clock] Setting date/time to: {}", date_time);
        self.estimated_date_time = date_time;
        self.clock_state = ClockState::Valid;
        self.last_valid.update_and_get_delta();
    }
}