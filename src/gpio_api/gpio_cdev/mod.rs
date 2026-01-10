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

//! Provides a setup function for the TM1637Adapter using the [`gpio_cdev`] crate.
//! **This is the recommended way!** It uses the character device driver bases Linux API.
//! (Alternative is sysfs-API, which is deprecated in linux kernel somewhere in 2020.)
//!
//! This feature must be activated in your Cargo.toml of you want to use it.

use crate::{GpioPinValue, TM1637Adapter};
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use gpio_cdev::{Chip, Line, LineHandle, LineRequestFlags};

/// Describes the persistent info/state of a "line" which is a Pin in the
/// character device driver-based terminology. We need this to retain control of
/// Pins across multiple invocations. Otherwise Pins are released immediately.
struct LineState {
    line: Line,
    handle: RefCell<Option<LineHandle>>,
}

impl LineState {
    fn new(chip: &mut Chip, pin_num: u32) -> Self {
        let line = chip.get_line(pin_num).unwrap();
        if line.info().unwrap().is_kernel() {
            panic!("Pin {} is already used in kernel!", pin_num)
        }
        Self {
            line,
            handle: RefCell::from(None),
        }
    }

    fn switch_to_out(ls: &Rc<Self>) {
        ls.handle.replace(None);
        ls.handle.replace(Some(
            ls.line
                .request(
                    LineRequestFlags::OUTPUT,
                    0,
                    &format!(
                        "tm1637-adapter-out-pin {}",
                        ls.line.info().unwrap().line().offset()
                    ),
                )
                .unwrap(),
        ));
    }

    fn switch_to_in(ls: &Rc<Self>) {
        ls.handle.replace(None);
        ls.handle.replace(Some(
            ls.line
                .request(
                    LineRequestFlags::INPUT,
                    0,
                    &format!(
                        "tm1637-adapter-in-pin {}",
                        ls.line.info().unwrap().line().offset()
                    ),
                )
                .unwrap(),
        ));
    }
}

/// Sets up the Driver using "gpio-cdev"-crate as GPIO interface/library.
///
/// This is better than wiringpi or "sysfs" because it uses the modern
/// character device based API/Driver in the linux kernel.
/// See <https://docs.rs/gpio-cdev/0.3.0/gpio_cdev/>
///
/// * `gpio_dev` is probably always "/dev/gpiochip0"
pub fn setup_gpio_cdev(
    clk_pin: u32,
    dio_pin: u32,
    bit_delay_fn: Box<dyn Fn()>,
    gpio_dev: &str,
) -> TM1637Adapter {
    let mut gpio = Chip::new(gpio_dev).unwrap();

    // prepare both pins and set them as output!

    let clk_pin = LineState::new(&mut gpio, clk_pin);
    let clk_pin = Rc::from(clk_pin);
    LineState::switch_to_out(&clk_pin);

    let dio_pin = LineState::new(&mut gpio, dio_pin);
    let dio_pin = Rc::from(dio_pin);
    LineState::switch_to_out(&dio_pin);

    let pin_clock_write_fn = pin_write_fn_factory(clk_pin);
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin.clone());
    let pin_dio_read_fn = pin_read_fn_factory(dio_pin);

    TM1637Adapter::new(
        pin_clock_write_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn,
    )
}

/// Creates a function/closure for the given pin that changes the value of the pin.
fn pin_write_fn_factory(pin: Rc<LineState>) -> Box<dyn Fn(GpioPinValue)> {
    Box::from(move |bit| {
        let h = pin.handle.borrow_mut();
        let h = h.as_ref().unwrap();
        h.set_value(bit as u8).unwrap();
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
fn pin_read_fn_factory(pin: Rc<LineState>) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        LineState::switch_to_in(&pin);
        let res = pin.handle.borrow().as_ref().unwrap().get_value().unwrap();
        LineState::switch_to_out(&pin);
        if res == 0 {
            GpioPinValue::LOW
        } else {
            GpioPinValue::HIGH
        }
    })
}
