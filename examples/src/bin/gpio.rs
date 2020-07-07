use tm1637_example::{run_demo, sleep_busy_waiting};
use tm1637_gpio_driver::gpio_api::setup_gpio;

/// Simple example that shows you how you can use the driver along with crate "gpio" to display
/// content on the 4-digit 7-segment display by AZDelivery. This is the display shown in the gif of the readme.
///
/// This demo shows multiple kinds of using the display to show data.
///
/// Be aware that you probably need sudo to run this, because this crate uses the "sysfs" API!
fn main() {
    // use any GPIO pin you want. This is the number of the pin on the board.
    // The numbers in the example here are available on the Raspberry Pi for example.
    let clk_pin = 18;
    let dio_pin = 23;

    // setup
    // 100µs should be totally save; less could work; depends on cable length and other factors
    // high frequencies are tricky; it even worked with zero waiting for me.. but better be safe!
    let bit_delay_fn = || sleep_busy_waiting(10);
    let bit_delay_fn = Box::from(bit_delay_fn);
    let tm1637display = setup_gpio(clk_pin, dio_pin, bit_delay_fn);

    run_demo(tm1637display);
}
