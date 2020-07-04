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