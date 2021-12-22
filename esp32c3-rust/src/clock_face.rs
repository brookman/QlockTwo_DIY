use crate::common::{get_time, SimpleTime};
use crate::led_strip::Color;
use crate::LedStrip;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ClockFaceState {
    Waiting,
    DisplayTime,
    DisplaySeconds,
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
                let b1 = self.current_time.seconds % 2;
                let b2 = (self.current_time.seconds >> 1) % 2;
                self.leds.colors[0] = Color::new(0, 0, 0, 0xff * b1);
                self.leds.colors[1] = Color::new(0, 0, 0, 0xff * b2);
                self.leds.brightness = 0.5;
            }
            ClockFaceState::DisplaySeconds => {}
        }

        if let Err(error) = self.leds.update() {
            println!("Error while updating LEDs: {:?}", error);
        }
    }
}