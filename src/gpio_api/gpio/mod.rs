//! Provides a setup function for the TM1637Adapter using the "gpio" crate.
//! Note that this comes with all restrictions that this has.
//! It uses "sysfs" which will be removed from linux kernel somewhere in 2020.
//!
//! Note: **This probably requires sudo on a Raspberry Pi, even if you are part of the gpio group!**
//!
//! This feature must be activated in your Cargo.toml if you want to use it.

use alloc::rc::Rc;
use alloc::boxed::Box;
use crate::{GpioPinValue, TM1637Adapter};
use gpio::{GpioOut, GpioIn, GpioValue};
use core::cell::{RefCell};
use gpio::sysfs::{SysFsGpioOutput, SysFsGpioInput};

// Abstract:
// We must prevent that pins get dropped after writing high/low, because this results in an
// "unexport" in sysfs, which resets the pin.
// Because of this the pin objects must survive invocations of the callbacks

enum PinKind {
    In(SysFsGpioInput),
    Out(SysFsGpioOutput),
}

impl PinKind {
    fn new_in(pin_num: u16) -> PinKind {
        PinKind::In(
            SysFsGpioInput::open(pin_num).expect("gpio sysfs: could not open pin")
        )
    }

    fn new_out(pin_num: u16) -> PinKind {
        PinKind::Out(
            SysFsGpioOutput::open(pin_num).expect("gpio sysfs: could not open pin")
        )
    }

    fn in_pin(&mut self) -> &mut SysFsGpioInput {
        if let PinKind::In(ref mut pin) = self { pin } else { panic!("Not an input pin!") }
    }

    fn out_pin(&mut self) -> &mut SysFsGpioOutput {
        if let PinKind::Out(ref mut pin) = self { pin } else { panic!("Not an output pin!") }
    }

    fn in_to_out(pin: &Rc<RefCell<Option<PinKind>>>, pin_num: u16) {
        // Reset old Pin due to drop
        pin.replace(None);
        pin.replace(Some(PinKind::new_out(pin_num)));
    }

    fn out_to_in(pin: &Rc<RefCell<Option<PinKind>>>, pin_num: u16) {
        // Reset old Pin due to drop
        pin.replace(None);
        pin.replace(Some(PinKind::new_in(pin_num)));
    }
}

/// Sets up the TM1637 Adapter using "gpio"-crate (that uses sysfs) as GPIO interface.
pub fn setup_gpio(clk_pin: u16,
                  dio_pin: u16,
                  bit_delay_fn: Box<dyn Fn() -> ()>) -> TM1637Adapter {
    // we must create the pins here
    // there must be references of them while the driver is running
    // otherwise the pins are dropped at every invocation which unexports them
    // which lets the kernel overwrite the last signal we wrote
    // e.g. "1" + unexport => 0 instead of it stays a 1

    let clk_pin = PinKind::new_out(clk_pin);
    let clk_pin  = Rc::from(RefCell::from(Option::from(clk_pin)));

    let dio_pin_num = dio_pin;
    let dio_pin = PinKind::new_out(dio_pin);
    let dio_pin = Rc::from(RefCell::from(Option::from(dio_pin)));

    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_write_fn = pin_write_fn_factory(clk_pin);
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin.clone());
    let pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue> = pin_read_fn_factory(dio_pin, dio_pin_num);
    // set up delay-fn: thread::sleep() is not available in lib because out lib is no-std

    // pass all wrapper functions to the adapter.
    TM1637Adapter::new(
        pin_clock_write_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn,
    )
}

/// Creates a function/closure for the given pin that changes the value of the pin.
fn pin_write_fn_factory(pin: Rc<RefCell<Option<PinKind>>>) -> Box<dyn Fn(GpioPinValue)> {
    Box::from(move |bit| {
        let mut pin = pin.borrow_mut();
        let pin = pin.as_mut().unwrap();
        let pin = pin.out_pin();
        pin.set_value(bit as u8).unwrap();
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
/// It fulfills the contract that the pin will be an out pin after this function is done!
/// Out-Pins are the default for this interface.
fn pin_read_fn_factory(pin: Rc<RefCell<Option<PinKind>>>, pin_num: u16) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        // we drop/unexport the pin in out-mode
        // then it can be an input pin
        PinKind::out_to_in(&pin, pin_num);

        // read value
        let mut res = pin.borrow_mut();
        let res = res.as_mut().unwrap().in_pin().read_value().unwrap();

        // we drop/unexport the pin in in-mode
        // then it can be an output pin again
        PinKind::in_to_out(&pin, pin_num);

        return if let GpioValue::High = res { GpioPinValue::HIGH } else { GpioPinValue::LOW }
    })
}
