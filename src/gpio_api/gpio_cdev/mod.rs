use crate::{TM1637Adapter, GpioPinValue};
use alloc::boxed::Box;
use gpio_cdev::{Chip, LineRequestFlags};
use alloc::rc::Rc;
use core::cell::RefCell;

/// Sets up the Driver using "gpio-cdev"-crate as GPIO interface/library.
/// This is better than wiringpi or "sysfs" because it uses the modern
/// character device based API/Driver in the linux kernel.
/// See https://docs.rs/gpio-cdev/0.3.0/gpio_cdev/
///
/// * `gpio_dev` is probably always "/dev/gpiochip0"
pub fn setup_gpio_cdev(clk_pin: u32,
                       dio_pin: u32,
                       bit_delay_fn: Box<dyn Fn() -> ()>,
                       gpio_dev: &str) -> TM1637Adapter {
    let mut gpio = Chip::new(gpio_dev).unwrap();
    let mut gpio = Rc::from(RefCell::from(gpio));

    let pin_clock_write_fn = pin_write_fn_factory(clk_pin, gpio.clone());
    let pin_dio_write_fn = pin_write_fn_factory(dio_pin, gpio.clone());
    let pin_dio_read_fn = pin_read_fn_factory(dio_pin, gpio);

    TM1637Adapter::new(
        pin_clock_write_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn
    )
}

/// Creates a function/closure for the given pin that changes the value of the pin.
fn pin_write_fn_factory(gpio_pin_num: u32, gpio: Rc<RefCell<Chip>>) -> Box<dyn Fn(GpioPinValue)> {
    Box::from(move |bit| {
        /*let line = gpio.borrow_mut().get_line(gpio_pin_num).unwrap();
        let line = line.request(
            LineRequestFlags::OUTPUT, 0, &format!("pin {}", gpio_pin_num)
        ).unwrap();
        line.set_value(bit as u8).unwrap();*/
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
fn pin_read_fn_factory(gpio_pin_num: u32, gpio: Rc<RefCell<Chip>>) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        /*let line = gpio.borrow_mut().get_line(gpio_pin_num).unwrap();
        let line = line.request(
            LineRequestFlags::OUTPUT, 0, &format!("pin {}", gpio_pin_num)
        ).unwrap();
        let res = line.get_value().unwrap();
        return if res == 0 { GpioPinValue::LOW } else { GpioPinValue::HIGH }*/
        GpioPinValue::LOW
    })
}