// MIT License
//
// Copyright (c) 2022 Philipp Schuster <phip1611@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Provides a setup function for the TM1637Adapter using the "wiringpi" crate.
//! Note that this comes with all restrictions that wiringpi has. This means
//! wiringpi must be installed on your Pi and you can only use this on a Raspberry Pi.
//!
//! This feature must be activated in your Cargo.toml of you want to use it.

use crate::{GpioPinValue, TM1637Adapter};
use alloc::boxed::Box;
use alloc::rc::Rc;
use wiringpi::pin::Value as WiringPiVal;
use wiringpi::WiringPi;

/// Sets up the TM1637 Adapter using WiringPi as GPIO interface.
pub fn setup_wiringpi(clk_pin: u16, dio_pin: u16, bit_delay_fn: Box<dyn Fn()>) -> TM1637Adapter {
    let gpio = wiringpi::setup_gpio();
    let gpio = Rc::from(gpio);

    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_write_fn = pin_write_fn_factory(clk_pin, gpio.clone());
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin, gpio.clone());
    let pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue> = pin_read_fn_factory(dio_pin, gpio);
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
fn pin_write_fn_factory(
    gpio_pin_num: u16,
    gpio: Rc<WiringPi<wiringpi::pin::Gpio>>,
) -> Box<dyn Fn(GpioPinValue)> {
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
fn pin_read_fn_factory(
    gpio_pin_num: u16,
    gpio: Rc<WiringPi<wiringpi::pin::Gpio>>,
) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        let res: WiringPiVal = gpio.input_pin(gpio_pin_num).digital_read();
        if res == WiringPiVal::High {
            GpioPinValue::HIGH
        } else {
            GpioPinValue::LOW
        }
    })
}
