//! Provides a setup function for the TM1637Adapter using the "wiringpi" crate.
//! Note that this comes with all restrictions that wiringpi has. This means
//! wiringpi must be installed on your Pi and you can only use this on a Raspberry Pi.
//!
//! This feature must be activated in your Cargo.toml of you want to use it.

use alloc::rc::Rc;
use wiringpi::WiringPi;
use wiringpi::pin::Value as WiringPiVal;
use alloc::boxed::Box;
use crate::{GpioPinValue, TM1637Adapter};

/// Sets up the TM1637 Adapter using WiringPi as GPIO interface.
pub fn setup_wiringpi(clk_pin: u16,
                      dio_pin: u16,
                      bit_delay_fn: Box<dyn Fn() -> ()>) -> TM1637Adapter {
    let gpio = wiringpi::setup_gpio();
    let gpio = Rc::from(gpio);

    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_write_fn = pin_write_fn_factory(clk_pin, gpio.clone());
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin, gpio.clone());
    let pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue> = pin_read_fn_factory(dio_pin, gpio.clone());
    // set up delay-fn: thread::sleep() is not available in lib because our lib is no-std

    // pass all wrapper functions to the adapter.
    TM1637Adapter::new(
        pin_clock_write_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn,
    )
}

/// Creates a function/closure for the given pin that changes the value of the pin.
fn pin_write_fn_factory(gpio_pin_num: u16, gpio: Rc<WiringPi<wiringpi::pin::Gpio>>) -> Box<dyn Fn(GpioPinValue)> {
    Box::from(move |bit| {
        let pin = gpio.output_pin(gpio_pin_num);
        if let GpioPinValue::HIGH = bit {
            pin.digital_write(WiringPiVal::High);
        } else {
            pin.digital_write(WiringPiVal::Low);
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
