extern crate tm1637_gpio_driver;
extern crate wiringpi;
//extern crate crono;

use chrono::prelude::*;

use wiringpi::pin::Value as WiringPiVal;
use std::rc::Rc;
use tm1637_gpio_driver::{GpioPinMode, TM1637Adapter, Brightness, GpioPinValue, DisplayState};
use wiringpi::WiringPi;
use std::thread::sleep;
use std::time::Duration;
use tm1637_gpio_driver::mappings::SpecialCharBits;
use tm1637_gpio_driver::fourdigit7segdis::{display_text_banner_in_loop, display_current_time};
use chrono::Local;

/// Simple example that shows you how you can use the driver along with crate "wiringpi" to display
/// content on the 4-digit 7-segment display by AZDelivery.
/// This is the display shown in the gif of the readme.
///
/// This demo shows 4 kinds of using the display to show data.
fn main() {
    // We have 4 displays
    const DISPLAYS_COUNT: usize = 4;
    let mut tm1637display = setup();


    // 1Hz: blinking double point clock (hh:mm)
    let tick_fn = || sleep(Duration::from_secs(1));
    let time_fn = || {
        let date = Local::now();

        // this is not so nice but I don't know a better solution

        //let l = date.format("%H").to_string();
        let l = date.format("%H").to_string();
        //let r = date.format("%S").to_string();
        let r = date.format("%M").to_string();
        println!("{}:{}", l, r);
        (l, r)
    };
    display_current_time(&mut tm1637display, &tick_fn, &time_fn);
}

/// Creates a function/closure for the given pin that changes the mode of the pin.
fn pin_mode_fn_factory(gpio_pin_num: u16, gpio: Rc<WiringPi<wiringpi::pin::Gpio>>) -> Box<dyn Fn(GpioPinMode)> {
    Box::from(move |mode| {
        if let GpioPinMode::INPUT = mode {
            gpio.input_pin(gpio_pin_num);
        } else {
            gpio.output_pin(gpio_pin_num);
        }
    })
}

/// Sets up the TM1637Adapter using wiringpi as GPIO interface.
fn setup() -> TM1637Adapter {
    // use any GPIO pin you want
    let clk_pin = 18;
    let dio_pin = 23;

    let gpio = wiringpi::setup_gpio();
    let gpio = Rc::from(gpio);

    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_mode_fn = pin_mode_fn_factory(clk_pin, gpio.clone());
    let pin_clock_write_fn = pin_write_fn_factory(clk_pin, gpio.clone());
    let pin_dio_mode_fn = pin_mode_fn_factory(clk_pin, gpio.clone());
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin, gpio.clone());
    let pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue> = pin_read_fn_factory(dio_pin, gpio.clone());
    // set up delay-fn: sleep() is not available in lib because it is zero dependency
    let bit_delay_fn: Box<dyn Fn() -> ()> = Box::from(|| {
        sleep(Duration::from_micros(10));
    });

    // pass all wrapper functions to the adapter.
    TM1637Adapter::new(
        pin_clock_mode_fn,
        pin_clock_write_fn,
        pin_dio_mode_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn,
    )
}

/// Creates a function/closure for the given pin that changes the value of the pin.
fn pin_write_fn_factory(gpio_pin_num: u16, gpio: Rc<WiringPi<wiringpi::pin::Gpio>>) -> Box<dyn Fn(GpioPinValue)> {
    Box::from(move |bit| {
        if let GpioPinValue::HIGH = bit {
            gpio.output_pin(gpio_pin_num).digital_write(WiringPiVal::High);
        } else {
            gpio.output_pin(gpio_pin_num).digital_write(WiringPiVal::Low);
        }
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
fn pin_read_fn_factory(gpio_pin_num: u16, gpio: Rc<WiringPi<wiringpi::pin::Gpio>>) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        let res: WiringPiVal = gpio.input_pin(gpio_pin_num).digital_read();
        return if let WiringPiVal::High = res { GpioPinValue::HIGH } else { GpioPinValue::LOW }
    })
}

