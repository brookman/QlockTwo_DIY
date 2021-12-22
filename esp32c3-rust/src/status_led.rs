use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::gpio::{Gpio3, Output};

pub struct StatusLed {
    high: bool,
    led: Gpio3<Output>,
}

impl StatusLed {

    pub fn new(mut led: Gpio3<Output>) -> Self {
        led.set_low().unwrap_or(());
        StatusLed {
            high: false,
            led,
        }
    }

    pub fn set(&mut self, high: bool) {
        if self.high == high {
            return;
        }
        if high {
            self.led.set_high().unwrap_or(());
        } else {
            self.led.set_low().unwrap_or(());
        }
        self.high = high;
    }

    pub fn set_high(&mut self) {
        self.set(true);
    }

    pub fn set_low(&mut self) {
        self.set(false);
    }
}