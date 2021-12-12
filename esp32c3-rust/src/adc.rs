use esp_idf_sys::{adc2_channel_t, adc2_config_channel_atten, adc2_get_raw, adc_atten_t, adc_bits_width_t, EspError};

use crate::led_strip::esp_res;

pub fn read_analog_value() -> i32 {
    let channel: adc2_channel_t = 0;
    let atten: adc_atten_t = 3;
    let width_bit: adc_bits_width_t = 3;

    let mut value: i32 = 0;
    let ptr: *mut i32 = &mut value;

    unsafe {
        esp_res(adc2_config_channel_atten(channel, atten)).unwrap();
        esp_res(adc2_get_raw(channel, width_bit, ptr)).unwrap();
    }

    return value;
}