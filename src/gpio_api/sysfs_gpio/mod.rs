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

//! Provides a setup function for the TM1637Adapter using the [`sysfs_gpio`] crate.
//! Note that sysfs interface is going to be deprecated in linux kernel somewhere in 2020.
//!
//! Note: **This probably requires sudo on a Raspberry Pi, even if you are part of the gpio group!**
//!
//! This feature must be activated in your Cargo.toml of you want to use it.

use crate::{GpioPinValue, TM1637Adapter};
use alloc::boxed::Box;
use sysfs_gpio::{Direction, Pin};

/// Sets up the TM1637 Adapter using WiringPi as GPIO interface.
pub fn setup_sysfs_gpio(clk_pin: u64, dio_pin: u64, bit_delay_fn: Box<dyn Fn()>) -> TM1637Adapter {
    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_write_fn = pin_write_fn_factory(clk_pin);
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin);
    let pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue> = pin_read_fn_factory(dio_pin);
    // set up delay-fn: sleep() is not available in our lib because we use no-std

    // pass all wrapper functions to the adapter.
    TM1637Adapter::new(
        pin_clock_write_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn,
    )
}

/// Creates a function/closure for the given pin that changes the value of the pin.
fn pin_write_fn_factory(pin_num: u64) -> Box<dyn Fn(GpioPinValue)> {
    Box::from(move |bit| {
        let pin = Pin::new(pin_num);
        pin.export().unwrap();
        pin.set_direction(Direction::Out).unwrap();
        pin.set_value(bit as u8).unwrap();
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
fn pin_read_fn_factory(pin_num: u64) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        let pin = Pin::new(pin_num);
        pin.export().unwrap();
        pin.set_direction(Direction::In).unwrap();
        let res = pin.get_value().unwrap();
        if res == 0 {
            GpioPinValue::LOW
        } else {
            GpioPinValue::HIGH
        }
    })
}
