use esp_idf_sys::{
    esp_err_t, ESP_OK, gpio_num_t,
    rmt_carrier_level_t_RMT_CARRIER_LEVEL_HIGH, rmt_carrier_level_t_RMT_CARRIER_LEVEL_LOW, rmt_channel_t,
    rmt_config_t, rmt_config_t__bindgen_ty_1, rmt_item32_t, rmt_mode_t_RMT_MODE_TX, rmt_tx_config_t,
};
use num::clamp;

const WS6812_RGBW_T0H_NS: u32 = 300;
const WS6812_RGBW_T0L_NS: u32 = 900;
const WS6812_RGBW_T1H_NS: u32 = 600;
const WS6812_RGBW_T1L_NS: u32 = 600;

const MAX_BRIGHTNESS: u8 = 127; // half of max output

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub w: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, w: u8) -> Color {
        Color { r, g, b, w }
    }

    pub fn to_array(self, brightness: f32) -> [u8; 4] {
        [
            clamp((self.g as f32 / 256f32 * MAX_BRIGHTNESS as f32 * brightness) as u8, 0, MAX_BRIGHTNESS),
            clamp((self.r as f32 / 256f32 * MAX_BRIGHTNESS as f32 * brightness) as u8, 0, MAX_BRIGHTNESS),
            clamp((self.b as f32 / 256f32 * MAX_BRIGHTNESS as f32 * brightness) as u8, 0, MAX_BRIGHTNESS),
            clamp((self.w as f32 / 256f32 * MAX_BRIGHTNESS as f32 * brightness) as u8, 0, MAX_BRIGHTNESS),
        ]
    }
}

#[derive(Debug)]
pub struct EspError {
    inner: esp_err_t,
}

impl std::fmt::Display for EspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "esp error code: {}", self.inner)
    }
}

impl std::error::Error for EspError {}

pub fn esp_res(err_code: esp_err_t) -> Result<(), EspError> {
    if err_code == ESP_OK as i32 {
        Ok(())
    } else {
        Err(EspError { inner: err_code })
    }
}

// #[derive(Debug)]
pub struct LedStrip<const NUM_LEDS: usize> {
    pub colors: [Color; NUM_LEDS],
    pub brightness: f32,
    pub channel: rmt_channel_t,
    bit0: rmt_item32_t,
    bit1: rmt_item32_t,
    pub raw_data: Vec<rmt_item32_t>,
}

impl<const NUM_LEDS: usize> LedStrip<NUM_LEDS> {
    pub fn new(channel: rmt_channel_t, gpio_num: gpio_num_t) -> Result<Self, EspError> {
        let config = rmt_config_t {
            rmt_mode: rmt_mode_t_RMT_MODE_TX,
            gpio_num,
            channel,
            clk_div: 2,
            mem_block_num: 2,
            flags: 0,
            __bindgen_anon_1: rmt_config_t__bindgen_ty_1 {
                tx_config: rmt_tx_config_t {
                    carrier_freq_hz: 38000,
                    carrier_level: rmt_carrier_level_t_RMT_CARRIER_LEVEL_HIGH,
                    idle_level: rmt_carrier_level_t_RMT_CARRIER_LEVEL_LOW,
                    carrier_duty_percent: 38,
                    carrier_en: false,
                    loop_en: false,
                    idle_output_en: true,
                    loop_count: 0,
                },
            },
        };

        let config_ptr: *const esp_idf_sys::rmt_config_t = &config;
        unsafe {
            esp_res(esp_idf_sys::rmt_config(config_ptr))?;
            esp_res(esp_idf_sys::rmt_driver_install(config.channel, 0, 0))?;
        }

        let mut counter_clk_hz: u32 = 0;
        unsafe {
            esp_res(esp_idf_sys::rmt_get_counter_clock(
                config.channel,
                &mut counter_clk_hz,
            ))?;
        }
        println!("Counter clock: {} Hz", counter_clk_hz);

        let ratio = counter_clk_hz as f32 / 1e9;

        println!("Ticks per ns: {}", ratio);

        let zero_high_ticks = (ratio * WS6812_RGBW_T0H_NS as f32) as u32;
        let zero_low_ticks = (ratio * WS6812_RGBW_T0L_NS as f32) as u32;
        let one_high_ticks = (ratio * WS6812_RGBW_T1H_NS as f32) as u32;
        let one_low_ticks = (ratio * WS6812_RGBW_T1L_NS as f32) as u32;

        println!("zero_high_ticks {}, zero_low_ticks {}, one_high_ticks {}, one_low_ticks {}, ", zero_high_ticks, zero_low_ticks, one_high_ticks, one_low_ticks);

        let bit0 = Self::create_rmt_item32(zero_high_ticks, 1, zero_low_ticks, 0);
        let bit1 = Self::create_rmt_item32(one_high_ticks, 1, one_low_ticks, 0);

        Ok(LedStrip {
            colors: [Color::new(0, 0, 0, 0); NUM_LEDS],
            brightness: 1f32,
            channel,
            bit0,
            bit1,
            raw_data: vec![bit0; NUM_LEDS * 4 * 8],
        })
    }

    fn create_rmt_item32(duration0: u32, level0: u32, duration1: u32, level1: u32) -> rmt_item32_t {
        let mut tmp = esp_idf_sys::rmt_item32_t__bindgen_ty_1__bindgen_ty_1::default();
        tmp.set_duration0(duration0);
        tmp.set_duration1(duration1);
        tmp.set_level0(level0);
        tmp.set_level1(level1);

        esp_idf_sys::rmt_item32_t {
            __bindgen_anon_1: esp_idf_sys::rmt_item32_t__bindgen_ty_1 {
                __bindgen_anon_1: tmp,
            },
        }
    }
}

impl<const NUM_LEDS: usize> Drop for LedStrip<NUM_LEDS> {
    fn drop(&mut self) {
        unsafe {
            esp_idf_sys::rmt_driver_uninstall(self.channel);
        }
    }
}

impl<const NUM_LEDS: usize> LedStrip<NUM_LEDS> {
    pub fn update(&mut self) -> Result<(), EspError> {
        let mut num = 0;
        for color in self.colors {
            for byte in color.to_array(self.brightness) {
                for i in 0..8 {
                    let bit = byte & (1 << (7 - i));
                    self.raw_data[num] = if bit == 0 { self.bit0 } else { self.bit1 };
                    num += 1;
                }
            }
        }
        unsafe {
            esp_res(esp_idf_sys::rmt_write_items(self.channel, self.raw_data.as_ptr(), self.raw_data.len() as i32, true))?;
        }
        Ok(())
    }
}