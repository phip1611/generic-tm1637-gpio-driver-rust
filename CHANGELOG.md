# 1.1.0
The crate feature "gpio-api-wiringpi" has been added.
This way you can easily setup the driver using wiringpi
as GPIO interface.

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

let bit_delay_fn = || sleep(Duration::from_millis(100));
let bit_delay_fn = Box::from(bit_delay_fn);
let mut display = setup_wiringpi(clk_pin, dio_pin, bit_delay_fn);
// write "-" on Position 0
display.write_segments_raw(&[SpecialChars::Minus], 1, 0);
```

# 1.0.0
Initial release.