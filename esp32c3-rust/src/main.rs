#[warn(incomplete_features)]
#[cfg_attr(std, esp32c3)]
use std::thread;
use std::time::Duration;

use anyhow::*;
use embedded_hal::adc::Channel;
use embedded_hal::digital::v2::InputPin;
use esp_idf_hal::gpio::{ Unknown};
use esp_idf_hal::prelude::Peripherals;
use log::*;

use crate::adc::read_analog_value;
use crate::led_strip::{Color, LedStrip};

mod led_strip;
mod adc;

fn main() -> Result<()> {
    println!("Tick tock, tick tock!");

    let peripherals = Peripherals::take().unwrap();
    let time_input = peripherals.pins.gpio1.into_input().unwrap();

    const NUMBER_OF_LEDS: usize = 114;
    let mut led = LedStrip::<NUMBER_OF_LEDS>::new(
        esp_idf_sys::rmt_channel_t_RMT_CHANNEL_0,
        esp_idf_sys::gpio_num_t_GPIO_NUM_2,
    )?;

    led.colors[0] = Color::new(0, 0, 0, 0xff);
    led.colors[1] = Color::new(0, 0, 0, 0xff);

    let mut v = 0f32;

    loop {
        if time_input.is_high().unwrap() {
            led.colors[0] = Color::new(0xff, 0, 0, 0);
        } else {
            led.colors[0] = Color::new(0x00, 0, 0, 0);
        }

        let brightness = read_analog_value() as f32 / 4096.0;
        println!("brightness {}", brightness);
        // let l = (brightness * 255.0) as u8;

        // colors.rotate_right(shift);
        // shift += 1;
        // led.set_color(&to_array(colors.clone()))?;

        led.brightness = ((v.sin() + 1f32) / 2f32) * brightness;

        v += 0.1f32;

        led.update()?;
        thread::sleep(Duration::from_millis(1));
    }
}
