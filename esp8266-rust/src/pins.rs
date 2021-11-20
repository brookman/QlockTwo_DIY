// use esp8266_hal::{gpio::*, target::GPIO};
// use paste;
//
// // This macro is a helper for defining a `Pins` type within a board support
// // crate. This type is used to provide more meaningful aliases for the various
// // GPIO pins for a given board.
// macro_rules! define_pins {
//     (
//         $(
//             $(#[$attr:meta])*
//             pin $name:ident = ($pin_ident:ident, $pin_mode:ty)
//         ),+ ,
//     ) => {
//
// paste::item! {
//     /// Maps labelled pin names to their physical pins.
//     pub struct Pins {
//         $(
//             $(#[$attr])*
//             pub $name: [<$pin_ident:camel>]<$pin_mode>
//         ),+
//     }
// }
//
// impl Pins {
//     /// Returns the pins for the device.
//     pub fn new(gpio: GPIO) -> Self {
//         let pins = gpio.split();
//
//         Self {
//             $( $name: pins.$pin_ident ),+
//         }
//     }
// }
//
//     };
// }
//
// define_pins!(
//     /// Digital pin 0
//     pin d0 = (gpio16, Input<Floating>),
//     /// Digital pin 1, SCL
//     pin d1 = (gpio5, Input<Floating>),
//     /// Digital pin 2, SDA
//     pin d2 = (gpio4, Input<Floating>),
//     /// Digital pin 3 (10k pull-up)
//     pin d3 = (gpio0, Input<Floating>),
//     /// Digital pin 4 (10k pull-up), built-in LED
//     pin d4 = (gpio2, Input<Floating>),
//     /// Digital pin 5, SCK
//     pin d5 = (gpio14, UnInitialized),
//     /// Digital pin 6, MISO
//     pin d6 = (gpio12, UnInitialized),
//     /// Digital pin 7, MOSI, CTS
//     pin d7 = (gpio13, UART),
//     /// Digital pin 8 (10k pull-down), SS, RTS
//     pin d8 = (gpio15, UART),
//
//     /// UART receive pin
//     pin rx = (gpio3, UART),
//     /// UART transmit pin
//     pin tx = (gpio1, UART),
// );