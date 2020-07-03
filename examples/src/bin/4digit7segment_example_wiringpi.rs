use chrono::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use tm1637_gpio_driver::extern_api::setup_wiringpi;
use tm1637_gpio_driver::{TM1637Adapter, DisplayState, Brightness};
use tm1637_gpio_driver::mappings::SpecialCharBits;
use tm1637_gpio_driver::fourdigit7segdis::display_current_time_in_loop;

// We have 4 displays
const DISPLAYS_COUNT: usize = 4;

/// Simple example that shows you how you can use the driver along with crate "wiringpi" to display
/// content on the 4-digit 7-segment display by AZDelivery.
/// This is the display shown in the gif of the readme.
///
/// This demo shows 4 kinds of using the display to show data.
fn main() {
    // use any GPIO pin you want. This is the number of the pin on the board.
    // The numbers in the example here are available on the Raspberry Pi for example.
    let clk_pin = 18;
    let dio_pin = 23;

    // setup
    let bit_delay_fn = || sleep(Duration::from_millis(100));
    let bit_delay_fn = Box::from(bit_delay_fn);
    let mut tm1637display = setup_wiringpi(clk_pin, dio_pin, bit_delay_fn);

    // display "1 2 3 4"
    let data: [u8; DISPLAYS_COUNT] = [
        TM1637Adapter::encode_digit(1),
        TM1637Adapter::encode_digit(2),
        TM1637Adapter::encode_digit(3),
        TM1637Adapter::encode_digit(4),
    ];
    tm1637display.write_segments_raw(&data, DISPLAYS_COUNT as u8, 0);
    sleep(Duration::from_secs(1));

    // ##############################################################################

    // set both in the middle to "-"
    tm1637display.write_segment_raw(SpecialCharBits::Minus as u8, 1);
    tm1637display.write_segment_raw(SpecialCharBits::Minus as u8, 2);
    sleep(Duration::from_secs(1));

    // ##############################################################################

    // animation that increases the brightness of the display
    for _ in 0..3 {
        // Turn Display off
        tm1637display.set_display_state(DisplayState::OFF);
        tm1637display.write_display_state();
        sleep(Duration::from_millis(200));

        // Turn display on again
        tm1637display.set_display_state(DisplayState::ON);
        tm1637display.set_brightness(Brightness::L0);
        tm1637display.write_display_state();

        sleep(Duration::from_millis(200));
        tm1637display.set_brightness(Brightness::L2);
        tm1637display.write_display_state();

        sleep(Duration::from_millis(200));
        tm1637display.set_brightness(Brightness::L4);
        tm1637display.write_display_state();

        sleep(Duration::from_millis(200));
        tm1637display.set_brightness(Brightness::L7);
        tm1637display.write_display_state();

        sleep(Duration::from_millis(200));
    }

    sleep(Duration::from_secs(1));

    // ##############################################################################

    // display this text over and over again
    /*let sleep_fn = || sleep(Duration::from_millis(250));
    display_text_banner_in_loop(
        &mut tm1637display,
        // 4 spaces because we want the text to smoothly slide in and out :)
        "    0123456789 ABCDEFGHIJKLMNOPQRSTUVWXY abcdefghijklmnopqrstuvwxyz    ",
        &sleep_fn
    );*/

    // ##############################################################################


    // 1Hz: blinking double point clock (hh:mm)
    let tick_fn = || sleep(Duration::from_secs(1));
    let time_fn = || {
        let date = Local::now();

        // this is not so nice but I don't know a better solution

        //let l = date.format("%H").to_string();
        let l = date.format("%H").to_string();
        //let r = date.format("%S").to_string();
        let r = date.format("%M").to_string();
        // println!("{}:{}", l, r);
        (l, r)
    };
    display_current_time_in_loop(&mut tm1637display, &tick_fn, &time_fn);
}


