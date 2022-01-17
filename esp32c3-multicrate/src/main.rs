use embedded_hal::digital::v2::InputPin;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_sys as _;

use crate::adc::Adc2;
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

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let time_input = peripherals.pins.gpio7.into_input().unwrap();

    let mut led3_red = StatusLed::new(peripherals.pins.gpio3.into_output().unwrap());

    let leds = LedStrip::<NUMBER_OF_LEDS>::new(
        esp_idf_sys::rmt_channel_t_RMT_CHANNEL_0,
        esp_idf_sys::gpio_num_t_GPIO_NUM_1,
    ).unwrap();

    let adc2 = Adc2::new();

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
        // ...

        // handle brightness:
        let brightness = adc2.read_analog_value();
        // println!("brightness {}", brightness);
        clock_face.set_brightness(brightness);

        clock_face.update();

        let time_to_sleep = clock.get_time_to_sleep();
        thread_sleep(time_to_sleep)
    }
}
