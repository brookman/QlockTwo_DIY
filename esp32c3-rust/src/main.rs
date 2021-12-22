#![feature(asm)]

#[warn(incomplete_features)]
#[cfg_attr(std, esp32c3)]
use anyhow::*;
use embedded_hal::digital::v2::InputPin;
use esp_idf_hal::prelude::Peripherals;
use log::*;

use crate::adc::read_analog_value;
use crate::clock::{Clock, ClockState};
use crate::clock_face::ClockFace;
use crate::common::thread_sleep;
use crate::led_strip::LedStrip;
use crate::status_led::StatusLed;

mod led_strip;
mod adc;
mod clock;
mod clock_face;
mod common;
mod status_led;

const NUMBER_OF_LEDS: usize = 114;

fn main() -> Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let time_input = peripherals.pins.gpio7.into_input().unwrap();

    let mut led3_red = StatusLed::new(peripherals.pins.gpio3.into_output().unwrap());

    let leds = LedStrip::<NUMBER_OF_LEDS>::new(
        esp_idf_sys::rmt_channel_t_RMT_CHANNEL_0,
        esp_idf_sys::gpio_num_t_GPIO_NUM_1,
    ).unwrap();

    let mut clock = Clock::new();
    let mut clock_face = ClockFace::new(leds);

    loop {
        let time_high = time_input.is_high().unwrap_or(false);
        clock.update(time_high);

        if clock.clock_state == ClockState::Uninitialized {
            led3_red.set(!time_high);
            clock_face.set_uninitialized();
        } else {
            led3_red.set_low();
            clock_face.set_time(clock.get_time());
        }

        // handle buttons:

        // handle brightness:
        // let brightness = read_analog_value() as f32 / 4096.0;
        // watch_face.set_brightness(brightness);

        clock_face.update();

        let time_to_sleep = clock.get_time_to_sleep();
        thread_sleep(time_to_sleep)
    }
}
