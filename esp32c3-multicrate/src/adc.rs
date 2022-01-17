use esp_idf_sys::{adc2_channel_t, adc2_config_channel_atten, adc2_get_raw, adc_atten_t, adc_bits_width_t};

use crate::led_strip::esp_res;

pub struct Adc2 {
    channel: adc2_channel_t,
    atten: adc_atten_t,
    width_bit: adc_bits_width_t,
}

impl Adc2 {
    pub fn new() -> Self {
        let adc2 = Adc2 {
            channel: 0,
            atten: 3,
            width_bit: 3,
        };

        unsafe {
            esp_res(adc2_config_channel_atten(adc2.channel, adc2.atten)).unwrap();
        }

        return adc2;
    }

    pub fn read_analog_value(&self) -> f32 {
        let mut value: i32 = 0;
        let ptr: *mut i32 = &mut value;

        unsafe {
            // esp_res(adc2_config_channel_atten(self.channel, self.atten)).unwrap();
            esp_res(adc2_get_raw(self.channel, self.width_bit, ptr)).unwrap();
        }

        return value as f32 / 4096.0;
    }
}