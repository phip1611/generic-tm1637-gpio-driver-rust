//! Provides a setup function for the TM1637Adapter using the "sysfs_gpio" crate.
//! Note that sysfs interface is going to be deprecated in linux kernel somewhere in 2020.
//!
//! This feature must be activated in your Cargo.toml of you want to use t.

use alloc::rc::Rc;
use alloc::boxed::Box;
use crate::{GpioPinValue, TM1637Adapter};
use sysfs_gpio::{Pin, Direction};

/// Sets up the TM1637 Adapter using WiringPi as GPIO interface.
pub fn setup_sysfs_gpio(clk_pin: u64,
                      dio_pin: u64,
                      bit_delay_fn: Box<dyn Fn() -> ()>) -> TM1637Adapter {

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
        pin.set_value(bit as u8);
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
fn pin_read_fn_factory(pin_num: u64) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        let pin = Pin::new(pin_num);
        pin.export().unwrap();
        pin.set_direction(Direction::In).unwrap();
        let res = pin.get_value().unwrap();
        return if res == 0 { GpioPinValue::LOW } else { GpioPinValue::HIGH }
    })
}
