#![no_std]
#![no_main]

use core::cmp::min;
use core::fmt::Write;

use dcf77::{DCF77Time, SimpleDCF77Decoder};
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use panic_halt as _;

struct AveragedValue<T: Sized, const COUNT: usize> {
    min_size: usize,
    max_size: usize,
    current_size: usize,
    current_index: usize,
    sum: T,
    values: [T; COUNT],
}

impl<const COUNT: usize> AveragedValue<i32, COUNT> {
    fn new(min_size: usize) -> Self {
        AveragedValue {
            min_size,
            max_size: COUNT,
            current_size: 0,
            current_index: 0,
            sum: 0,
            values: [0i32; COUNT],
        }
    }
    fn update(mut self, value: i32) -> Option<i32> {
        self.sum -= self.values[self.current_index];
        self.values[self.current_index] = value;
        self.sum += value;

        self.current_index = (self.current_index + 1) % COUNT;
        self.current_size = min(self.current_size + 1, COUNT);
        if self.current_size >= self.min_size {
            let average = self.sum / self.current_size as i32;
            return Some(average);
        }
        return None;
    }
}



#[entry]
fn main() -> ! {
    let average_flank = AveragedValue::<i32, 60>::new(10);

    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let mut serial = dp.UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    let (mut timer, _) = dp.TIMER.timers();

    let dcf77_input_pin = pins.gpio5.into_floating_input();

    let mut led = pins.gpio2.into_push_pull_output();
    led.set_high().unwrap();

    let mut decoder = SimpleDCF77Decoder::new();

    loop {
        let current_bit = dcf77_input_pin.is_high().unwrap();

        if current_bit {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }

        decoder.read_bit(current_bit);

        if decoder.bit_complete() {
            write!(serial, "Bit complete: ").unwrap();
            if decoder.bit_faulty() {
                write!(serial, "Faulty").unwrap();
            } else {
                write!(serial, "{}", decoder.latest_bit()).unwrap();
            }
            write!(serial, ", seconds: {}\n", decoder.seconds()).unwrap();
        }

        if decoder.end_of_cycle() {
            let time = DCF77Time::new(decoder.raw_data());

            if let Ok(_) = time.validate_start() {
                if let Ok(hours) = time.hours() {
                    if let Ok(minutes) = time.minutes() {
                        write!(serial, "The time is ").unwrap();
                        write!(serial, "{}", hours).unwrap();
                        write!(serial, ":").unwrap();
                        write!(serial, "{}", minutes).unwrap();
                        write!(serial, "\n").unwrap();
                    } else {
                        write!(serial, "Minutes invalid\n").unwrap();
                    }
                } else {
                    write!(serial, "Hours invalid\n").unwrap();
                }
            } else {
                write!(serial, "Start invalid\n").unwrap();
            }
        }

        timer.delay_ms(10);
    }
}


// #[entry]
// fn main() -> ! {
//     let peripherals = target::Peripherals::take().unwrap();
//     let pins = Pins::new(peripherals.GPIO);
//
//     let (mut timer1, _) = peripherals.TIMER.timers();
//
//     // let tx = pins.d4.into_uart();
//
//     // let mut uart0 = peripherals.UART0.serial(pins.tx, pins.rx);
//
//     let mut uart1 = peripherals.UART1.serial(pins.d4.into_uart());
//
//
//
//     // let mut led = pins.d4.into_push_pull_output();
//     // led.set_high().unwrap();
//
//
//     // let mut led = pins.d4.into_push_pull_output();
//     // led.set_high().unwrap();
//
//
//     loop {
//         uart1.write_str("123").unwrap();
//
//         timer1.delay_ms(1_000);
//         // led.toggle().unwrap();
//     }
// }

// #[entry]
// fn main() -> ! {
//     let peripherals = target::Peripherals::take().unwrap();
//     let pins = Pins::new(peripherals.GPIO);
//
//     let (mut timer1, _) = peripherals.TIMER.timers();
//
//     let mut led = pins.d4.into_push_pull_output();
//     led.set_high().unwrap();
//
//
//     let mut button1_input = pins.d7.into_floating_input();
//
//     let mut time_input = pins.d1.into_floating_input();
//
//     loop {
//         if time_input.is_high().unwrap() {
//             led.set_low().unwrap();
//
//         } else {
//             led.set_high().unwrap();
//         }
//         // led.toggle().unwrap();
//     }
// }

// #[entry]
// fn main() -> ! {
//     let peripherals = target::Peripherals::take().unwrap();
//     let pins = Pins::new(peripherals.GPIO);
//
//     let (mut timer, _) = peripherals.TIMER.timers();
//     let mut led = pins.d4.into_push_pull_output();
//
//     led.set_high().unwrap();
//
//     let mut time_input = pins.d1.into_push_pull_output();
//
//     let mut serial = peripherals.UART0.serial(pins.tx, pins.rx);
//
//     let pins = dp.GPIO.split();
//
//     let mut led = pins.gpio2.into_push_pull_output();
//
//     loop {
//         timer.delay_ms(400);
//         led.toggle().unwrap();
//         serial.write_str("foo bar baz\r\n").unwrap();
//     }
// }