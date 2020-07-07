# 2.0.0
### Breaking Changes
- Simplified constructor for `TM1637Adapter`. Removed the "set mode function"-parameters.
  ```
  /// Creates a new object to interact via GPIO with a TM1637.
  ///
  /// * `pin_clock_write_fn` function to write bit to CLK pin
  /// * `pin_dio_write_fn` function to write bit to DIO pin
  /// * `pin_dio_read_fn` function to read value from DIO pin
  /// * `bit_delay_fn` function that is invoked after a bit has been written to a pin.
  ///                  Probably 1 or even 0 µs are fine. This is just to be sure. It depends
  ///                  on your hardware and your GPIO driver.
  pub fn new(pin_clock_write_fn: Box<dyn Fn(GpioPinValue)>,
             pin_dio_write_fn: Box<dyn Fn(GpioPinValue)>,
             pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue>,
             bit_delay_fn: Box<dyn Fn() -> ()>) -> TM1637Adapter {}
  ```
### New features
- added the following crate features which can be activated in Cargo:
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

# 1.2.2
**breaking** Removed "n" parameter from `write_segments_raw` because Rust can figure out array length by itself.
(I know that the version number indicates it's a minor update but this is such a small thing..)

# 1.2.1
- bugfix in function `fourdigit7segdis::display_timer`

# 1.2.0
- added function `fourdigit7segdis::display_timer`

# 1.1.6
- Fixed test (CI build was broken)

# 1.1.5
- Bugfix in `fourdigit7segdis::display_text_banner_in_loop`
- **breaking**: renamed module `external_api` to `gpio_api`
- added `gpio_api::setup_dummy()` to support debugging in systems that don't
  have a GPIO interface, like Mac for example 

# 1.1.4
Fix in README example.

# 1.1.3 
More small code improvements + comments.

# 1.1.2
Better comments and documentation.

# 1.1.1
Replaced `thread::sleep` in examples by a busy waiting approach for way better latency!

# 1.1.0
The crate feature `gpio-api-wiringpi` has been added.
This way you can easily setup the driver using wiringpi
as GPIO interface. Also `fourdigit7segdis` is now a default feature
which could be deactivated in your Cargo.toml.

```
Cargo.toml:

[dependencies.tm1637-gpio-driver]
version = "1.1.0"
features = ["gpio-api-wiringpi"]


-------------

code.rs:

use tm1637_gpio_driver::extern_api::setup_wiringpi;
use std::thread::sleep;
use std::time::Duration;

let bit_delay_fn = || sleep(Duration::from_micros(100));
let bit_delay_fn = Box::from(bit_delay_fn);
let mut display = setup_wiringpi(clk_pin, dio_pin, bit_delay_fn);
// write "-" on Position 0
display.write_segment_raw(SpecialChars::Minus as u8, 0);
```

Note that with many GPIO crates/libs you probably loose `#[no-std]`-compliance.

### New functions
- `fourdigit7segdis::display_stopwatch`
- `TM1637Adapter::encode_number`

# 1.0.0
Initial release.