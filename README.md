# Generic TM1637 GPIO Driver

Generic GPIO driver for the TM1637 micro controller, primarily for educational purpose. 
It is used in the 4-digit 7-segment display by AZ-Delivery [(Link)](https://www.az-delivery.de/products/4-digit-display).
Generic means that it is not dependent on a specific GPIO interface. You can choose the GPIO 
interface/library on your own. 

# TL;DR: minimal setup
## Cargo.toml
```toml
[dependencies]
tm1637-gpio-driver = "<insert latest version>"
# or if you need no_std
tm1637-gpio-driver = { version = "<insert latest version>", default-features = false }
```
## Code
```rust
use std::thread::sleep;
use std::time::Duration;
use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::TM1637Adapter;

// example that definitely works on Raspberry Pi
fn main() {
    // use any GPIO pin you want. This is the number of the pin on the board.
    let (clk_pin, dio_pin) = (18, 23);

    let bit_delay_fn = Box::from(|| sleep(Duration::from_micros(10)));
    let tm1637display = setup_gpio_cdev(clk_pin, dio_pin, bit_delay_fn, "/dev/gpiochip0");

    // display "1 2 3 4"
    let data: [u8; 4] = [
        TM1637Adapter::encode_digit(1),
        TM1637Adapter::encode_digit(2),
        TM1637Adapter::encode_digit(3),
        TM1637Adapter::encode_digit(4),
    ];
    tm1637display.write_segments_raw(&data, 0);
}
```

---

## `no_std`
This crate is `#![no_std]` An allocator is necessary on embedded systems because `extern crate alloc` (core library) is used.
In `#![no_std]` you have to disable the default features in `Cargo.toml`.

This driver works with other displays too if they use a TM1637 micro controller with the same
I2C-like serial bus protocol specified in the [data sheet](https://www.mcielectronics.cl/website_MCI/static/documents/Datasheet_TM1637.pdf).

I created this library/driver for fun and to learn new things!

See this demo (gif) I made with my Raspberry Pi using regular GPIO pins:

Moving Text:

![gpio demonstration](example-moving-text.gif)

Time with blinking double point:

![gpio demonstration](example-time.gif)
 
## How does this work? How do I write a driver for that thing?
This was my first time writing a (super simple basic) kind of a device driver.
As of now I'm not that much experienced with micro controllers.
After some time I understood how it works by looking at the [data sheet](https://www.mcielectronics.cl/website_MCI/static/documents/Datasheet_TM1637.pdf 
). Have a look into my code too! I tried to make as many comments as possible.

## How can I use it?
You can find code examples in the [github repository](https://github.com/phip1611/generic-tm1637-gpio-driver-rust)!

My driver/library is not dependent on a specific GPIO interface.
You can use [crates.io: wiringpi](https://crates.io/crates/wiringpi) or [crates.io: gpio](https://crates.io/crates/gpio)
for example. I strongly recommend [crates.io: gpio_cdev](https://crates.io/crates/gpio_cdev).
I tested them on my Raspberry Pi. My `TM1637Adapter` needs functions/closures 
as parameters. These functions are wrappers to write High/Low to the desired Pins.

There are also utility functions on top of the driver in the module `fourdigit7segdis` for the 4-digit
7-segment display. You can use them, learn from them or just write your own functions on top of the driver.

**To add this driver to your project just add the [crate](https://crates.io/crates/tm1637-gpio-driver) to your Rust project.**

## Supported GPIO interfaces/libs/crates
As I already said this crate is independent from a specific strategy to access GPIO. But I provide several setup 
functions for different strategies as listed below (all of them need standard library). 
To use them activate on of the features in your Cargo.toml:
  - `gpio-api-gpio_cdev`
    - provides a setup function for the TM1637Adapter that uses "gpio_cdev"-crate as GPIO interface
    - `tm1637_gpio_driver::gpio_cdev::setup_gpio_cdev()`
    - this uses the character device driver-based api/interface in the Linux kernel
    - **This is the RECOMMENDED, modern way!** Sysfs is deprecated
  - `gpio-api-gpio`
    - provides a setup function for the TM1637Adapter that uses "gpio"-crate as GPIO interface
    - `tm1637_gpio_driver::gpio_api::setup_gpio()`
    - this uses the "sysfs"-Interface which probably requires root/sudo when executed
  - `gpio-api-sysfs_gpio`
    - provides a setup function for the TM1637Adapter that uses "sysfs_gpio"-crate as GPIO interface
    - `tm1637_gpio_driver::sysfs_gpio::setup_sysfs_gpio()`
    - this uses the "sysfs"-Interface which probably requires root/sudo when executed
    - this uses the "sysfs"-Interface which probably requires root/sudo when executed
  - `gpio-api-wiringpi`
    - provides a setup function for the TM1637Adapter that uses "wiringpi"-crate as GPIO interface
    - `tm1637_gpio_driver::sysfs_gpio::setup_wiringpi()`
    - make sure "wiringpi" is installed on your Pi

## Does this work only on Raspberry Pi?
Probably no! Although I can't test it because I don't have an Arduino or another similar device.
This should work on every device where you can write a Rust program for. Since this lib
uses no standard library this should work on embedded devices. If you use it let me know
what things you've built!

But yes, it was only tested using regular GPIO pins on my Raspberry Pi running Rasbperry Pi OS so far.
 
### Who Am I?
I'm Philipp :)
Feel free to contribute on [Github](https://github.com/phip1611/generic-tm1637-gpio-driver-rust) or 
message me on Twitter (https://twitter.com/phip1611)!
 
### Special thanks
Special thanks to the creator of the [driver for the Arduino](https://github.com/avishorp/TM1637). His/her (? - don't know) driver for the Arduino platform
helped me to understand how the TM1637 micro controller works. With this work and my
effort I put into understanding the data sheet I could make this driver.
I also learned a lot about serial data transfer and the I2C-like serial bus protocol used by
the TM1637.

I don't use any of the code. It just gave me some inspiration.

### Troubleshooting
- Data is not correctly displayed on display
  - either your device is broken (I ordered 3 and 1 of 3 were broken) or you probably have
    a to high frequency. Make sure the bit-delay for `TM1637Adapter::new` is not too short.
    100µs on Raspberry Pi should be totally fine (but 1µs worked also for me) 
  - check cables and GPIO-pins (clk, dio)
- Raspberry Pi / Raspberry Pi OS
  - "Permission denied"
    - make sure your user is part of the "gpio" group
    - `sudo usermod -a -G gpio <your-user-name>` 
- bit delay function: no difference between 1 and 100µs
  - if you use `thread::sleep()` as your bit delay function then you gonna have a problem when it comes
    to a few micro seconds: the operating system (or better to say the hardware) is not fast enough
    for switching threads in that short period of time
  - in that case you should use a "busy waiting"-like approach that doesn't send the thread into sleep mode
    but wait in a loop until a certain time has been reached.

### Trivia
- There is another library on crates.io for the TM1637: https://github.com/igelbox/tm1637-rs
It uses the "embedded-hal"-crate and takes another approach. Check this out too. :)

- I don't know if "driver" is the right word for this because it is not tied to the operating system.
  But the display is a device and my library can talk with it .. so yes.. basically a driver, right?
