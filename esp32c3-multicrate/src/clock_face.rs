use spanish_clockface::get_led_pattern;

use crate::common::{get_time, SimpleTime};
use crate::led_strip::Color;
use crate::LedStrip;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ClockFaceState {
    Waiting,
    DisplayTime,
}

pub struct ClockFace {
    pub state: ClockFaceState,
    current_time: SimpleTime,
    brightness: f32,
    leds: LedStrip::<114>,
}

impl ClockFace {
    pub fn new(leds: LedStrip::<114>) -> Self {
        ClockFace {
            state: ClockFaceState::Waiting,
            current_time: SimpleTime {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
            brightness: 0.5,
            leds,
        }
    }

    pub fn set_uninitialized(&mut self) {
        self.state = ClockFaceState::Waiting;
    }

    pub fn set_time(&mut self, time: SimpleTime) {
        self.state = ClockFaceState::DisplayTime;
        self.current_time = time;
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
    }

    pub fn update(&mut self) {
        match self.state {
            ClockFaceState::Waiting => {
                for i in 0..self.leds.colors.len() {
                    self.leds.colors[i] = Color::new(0, 0, 0, 0xff);
                }
                self.leds.brightness = ((get_time() as f32 / 250_000f32).sin() + 1.0) / 4.0 + 0.25;
            }
            ClockFaceState::DisplayTime => {
                let pattern = get_led_pattern(self.current_time.hours, self.current_time.minutes);
                for i in 0..self.leds.colors.len() {
                    if pattern & (1 << i) != 0 {
                        self.leds.colors[i] = Color::new(0, 0, 0, 0xff);
                    } else {
                        self.leds.colors[i] = Color::new(0, 0, 0, 0);
                    }
                }
                self.leds.brightness = 0.5;
            }
        }

        if let Err(error) = self.leds.update() {
            println!("Error while updating LEDs: {:?}", error);
        }
    }
}