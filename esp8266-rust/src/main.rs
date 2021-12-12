#![no_std]
#![no_main]
#![feature(asm)]

use core::cmp::min;
use core::fmt::Write;

use dcf77::{DCF77Time, SimpleDCF77Decoder};
use esp8266_hal::gpio::{Gpio0, Gpio4, OpenDrain, Output, PushPull};
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
use esp8266_hal::time::Nanoseconds;
use esp8266_hal::timer::{Timer1, Timer2};
use panic_halt as _;

/// Get the core cycle count
#[inline]
pub fn get_cycle_count() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.ccount {0}", out(reg) x, options(nostack)) };
    x
}

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

struct ClockTimer {
    start: u32,
    // last: u32,
}

impl ClockTimer {
    fn new() -> ClockTimer {
        ClockTimer {
            start: 0,
            // last: 0,
        }
    }

    #[inline]
    fn start(&mut self) {
        self.start = get_cycle_count();
        // self.last = self.start;
    }

    #[inline]
    fn block_until(&mut self, clocks: u32) {
        loop {
            // self.last = get_cycle_count().wrapping_sub(self.start);
            if get_cycle_count().wrapping_sub(self.start) >= clocks {
                break;
            }
        }
    }

    #[inline]
    fn start_and_block_until(&mut self, clocks: u32) {
        self.start();
        self.block_until(clocks);
    }
}

const F_CPU: u32 = 80_000_000;

const NS_100: u32 = F_CPU / 10_000_001;
const NS_300: u32 = F_CPU / 3_333_333;
const NS_400: u32 = F_CPU / 2_500_001;
const NS_600: u32 = F_CPU / 1_666_666;
const NS_800: u32 = F_CPU / 1_250_001;
const NS_1250: u32 = F_CPU / 800_001;

const US_125: u32 = F_CPU / 8_001;
const S_1: u32 = F_CPU;

fn send(led_strip: &mut Gpio4<Output<PushPull>>, color: u32) {
    let mut low_timer = ClockTimer::new();
    let mut high_timer = ClockTimer::new();

    low_timer.start();
    for i in 0..32 {
        let bit = color & (1 << (32 - i - 1)) != 0;
        if bit {
            low_timer.block_until(NS_1250 * i);
            led_strip.set_high().unwrap();
            high_timer.start_and_block_until(NS_600);
            led_strip.set_low().unwrap();
        } else {
            low_timer.block_until(NS_1250 * i);
            led_strip.set_high().unwrap();
            high_timer.start_and_block_until(NS_300);
            led_strip.set_low().unwrap();
        }
    }
    low_timer.start_and_block_until(US_125);
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();

    let (mut timer1, mut timer2) = dp.TIMER.timers();

    let mut led_strip = pins.gpio4.into_push_pull_output();
    led_strip.set_low().unwrap();


    // G R B W
    let red = 0x00_01_00_00 as u32;
    let green = 0x01_00_00_00 as u32;
    let blue = 0x00_00_01_00 as u32;
    let white = 0x00_00_00_01 as u32;
    let pink = 0x00_01_01_00 as u32;

    let colors = [red, green, blue, pink, white];
    let mut current_color = colors[0];

    let mut clock_timer = ClockTimer::new();

    clock_timer.start_and_block_until(40_000_000);

    loop {
        send(&mut led_strip, current_color);
        clock_timer.start_and_block_until(10_000_000);

        // timer1.delay_ms(100);
        // period_timer.start_and_block_until(10_000_000);
    }
}

// #[entry]
// fn main() -> ! {
//     let dp = Peripherals::take().unwrap();
//     let pins = dp.GPIO.split();
//
//     let (mut timer1, mut timer2) = dp.TIMER.timers();
//
//     let mut led_strip = pins.gpio4.into_push_pull_output();
//     led_strip.set_low().unwrap();
//
//
//     // G R B W
//     let red = 0x00_01_00_00 as u32;
//     let green = 0x01_00_00_00 as u32;
//     let blue = 0x00_00_01_00 as u32;
//     let white = 0x00_00_00_10 as u32;
//     let pink = 0x00_01_01_00 as u32;
//
//     let colors = [red, green, blue, pink];
//     let mut current_color = colors[0];
//     let mut count = 0;
//
//     let mut low_timer = ClockTimer::new();
//     let mut high_timer = ClockTimer::new();
//     low_timer.start_and_block_until(40_000_000);
//
//     loop {
//
//         for i in 0..32 {
//             let bit = current_color & (1 << (32 - i - 1)) != 0;
//             if bit {
//                 low_timer.start_and_block_until(NS_1250 * i);
//                 led_strip.set_high().unwrap();
//                 high_timer.start_and_block_until(NS_600);
//                 led_strip.set_low().unwrap();
//             } else {
//                 low_timer.start_and_block_until(NS_1250 * i);
//                 led_strip.set_high().unwrap();
//                 high_timer.start_and_block_until(NS_300);
//                 led_strip.set_low().unwrap();
//             }
//         }
//
//         low_timer.start_and_block_until(US_125);
//
//         current_color = colors[count % colors.len()];
//         count += 1;
//
//         low_timer.start_and_block_until(40_000_000);
//     }
// }


// #[entry]
// fn main() -> ! {
//     let average_flank = AveragedValue::<i32, 60>::new(10);
//
//     let dp = Peripherals::take().unwrap();
//     let pins = dp.GPIO.split();
//     let mut serial = dp.UART0
//         .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());
//
//     let (mut timer, _) = dp.TIMER.timers();
//
//     let dcf77_input_pin = pins.gpio5.into_floating_input();
//
//     let mut led = pins.gpio2.into_push_pull_output();
//     led.set_high().unwrap();
//
//     let mut decoder = SimpleDCF77Decoder::new();
//
//     loop {
//         let current_bit = dcf77_input_pin.is_high().unwrap();
//
//         if current_bit {
//             led.set_low().unwrap();
//         } else {
//             led.set_high().unwrap();
//         }
//
//         decoder.read_bit(current_bit);
//
//         if decoder.bit_complete() {
//             write!(serial, "Bit complete: ").unwrap();
//             if decoder.bit_faulty() {
//                 write!(serial, "Faulty").unwrap();
//             } else {
//                 write!(serial, "{}", decoder.latest_bit()).unwrap();
//             }
//             write!(serial, ", seconds: {}\n", decoder.seconds()).unwrap();
//         }
//
//         if decoder.end_of_cycle() {
//             let time = DCF77Time::new(decoder.raw_data());
//
//             if let Ok(_) = time.validate_start() {
//                 if let Ok(hours) = time.hours() {
//                     if let Ok(minutes) = time.minutes() {
//                         write!(serial, "The time is ").unwrap();
//                         write!(serial, "{}", hours).unwrap();
//                         write!(serial, ":").unwrap();
//                         write!(serial, "{}", minutes).unwrap();
//                         write!(serial, "\n").unwrap();
//                     } else {
//                         write!(serial, "Minutes invalid\n").unwrap();
//                     }
//                 } else {
//                     write!(serial, "Hours invalid\n").unwrap();
//                 }
//             } else {
//                 write!(serial, "Start invalid\n").unwrap();
//             }
//         }
//
//         timer.delay_ms(10);
//     }
// }


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