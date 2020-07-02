extern crate tm1637_gpio_driver;
extern crate wiringpi;

use std::env;
use wiringpi::pin::Value::{High, Low};
use wiringpi::pin::Value;
use std::rc::Rc;
use tm1637_gpio_driver::{GpioPinMode, TM1637Adapter, Brightness, GpioPinValue, LettersToSegmentBits};
use wiringpi::WiringPi;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    const DISPLAY_REGISTERS: usize = 4;
    let mut tm1637display = setup();

    let data: [u8; DISPLAY_REGISTERS] = [
        TM1637Adapter::encode_digit(1),
        TM1637Adapter::encode_digit(2),
        TM1637Adapter::encode_digit(3),
        TM1637Adapter::encode_digit(4),
    ];
    tm1637display.write_segments_raw(&data, DISPLAY_REGISTERS as u8, 0);
    sleep(Duration::from_secs(2));


    // set both in the middle to "-"
    tm1637display.write_segment_raw(LettersToSegmentBits::MINUS as u8, 1);
    tm1637display.write_segment_raw(LettersToSegmentBits::MINUS as u8, 2);
    sleep(Duration::from_secs(2));

    for _ in 0..10 {
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

    tm1637display.write_segment_raw(LettersToSegmentBits::A as u8, 0);
    tm1637display.write_segment_raw(LettersToSegmentBits::B as u8, 1);
    tm1637display.write_segment_raw(LettersToSegmentBits::C as u8, 2);
    tm1637display.write_segment_raw(LettersToSegmentBits::D as u8, 3);
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
            gpio.output_pin(gpio_pin_num).digital_write(High);
        } else {
            gpio.output_pin(gpio_pin_num).digital_write(Low);
        }
    })
}

/// Creates a function/closure for the given pin that reads its value in the moment of invocation.
fn pin_read_fn_factory(gpio_pin_num: u16, gpio: Rc<WiringPi<wiringpi::pin::Gpio>>) -> Box<dyn Fn() -> GpioPinValue> {
    Box::from(move || {
        let res: Value = gpio.input_pin(gpio_pin_num).digital_read();
        return if let Value::High = res { GpioPinValue::HIGH } else { GpioPinValue::LOW }
    })
}

