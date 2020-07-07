use std::thread::sleep;
use std::time::Duration;
use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::TM1637Adapter;

// example that defiantly works on Raspberry Pi
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
