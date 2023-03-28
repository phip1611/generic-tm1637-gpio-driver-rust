use std::ops::Add;
use std::time::{Duration, Instant};
use tm1637_gpio_driver::{Brightness, DisplayState, TM1637Adapter};
use tm1637_gpio_driver::fourdigit7segdis::{display_current_time_in_loop, display_stopwatch, display_timer, STOPWATCH_MAX};
use tm1637_gpio_driver::mappings::SpecialCharBits;

// We have 4 displays
pub const DISPLAYS_COUNT: usize = 4;

/// One second in Micro seconds.
pub const SECOND: u64 = 1E6 as u64;

pub fn run_demo(mut tm1637display: TM1637Adapter) {

    sleep_busy_waiting(SECOND);

    // ##############################################################################

    // set both in the middle to "-"
    tm1637display.write_segment_raw(SpecialCharBits::Minus as u8, 1);
    tm1637display.write_segment_raw(SpecialCharBits::Minus as u8, 2);
    sleep_busy_waiting(SECOND);

    // ##############################################################################

    // animation that increases the brightness of the display
    for _ in 0..3 {
        // Turn Display off
        tm1637display.set_display_state(DisplayState::OFF);
        tm1637display.write_display_state();
        sleep_busy_waiting(200 * 1000); // 200 milliseconds

        // Turn display on again
        tm1637display.set_display_state(DisplayState::ON);
        tm1637display.set_brightness(Brightness::L0);
        tm1637display.write_display_state();

        sleep_busy_waiting(200 * 1000); // 200 milliseconds
        tm1637display.set_brightness(Brightness::L2);
        tm1637display.write_display_state();

        sleep_busy_waiting(200 * 1000); // 200 milliseconds
        tm1637display.set_brightness(Brightness::L4);
        tm1637display.write_display_state();

        sleep_busy_waiting(200 * 1000); // 200 milliseconds
        tm1637display.set_brightness(Brightness::L7);
        tm1637display.write_display_state();

        sleep_busy_waiting(200 * 1000); // 200 milliseconds
    }

    // ##############################################################################

    // display this text over and over again
    /*let sleep_fn = || sleep_busy_waiting(SECOND);
    display_text_banner_in_loop(
        &mut tm1637display,
        // 4 spaces because we want the text to smoothly slide in and out :)
        "    0123456789 ABCDEFGHIJKLMNOPQRSTUVWXY abcdefghijklmnopqrstuvwxyz ?-_=.    ",
        &sleep_fn
    );*/

    // ##############################################################################

    // timer from 10 to 0 in 10 seconds
    display_timer(&mut tm1637display, &|| sleep_busy_waiting(SECOND), 10);

    // stopwatch from 0 to 10 in 10 seconds
    display_stopwatch(&mut tm1637display, &|| sleep_busy_waiting(SECOND), 10, true);

    // counter from 0 to 9999 with max speed
    display_stopwatch(&mut tm1637display, &|| {}, STOPWATCH_MAX, false);

    // ##############################################################################

    // 1Hz: blinking double point clock (hh:mm)
    let tick_fn = || sleep_busy_waiting(SECOND);
    let time_fn = || {
        let date = time::OffsetDateTime::now_utc();
        let l = format!("{:02}", date.hour());
        let r = format!("{:02}", date.minute());
        (l, r)
    };
    display_current_time_in_loop(&mut tm1637display, &tick_fn, &time_fn);
}

/// Sleeps/waits actively for x µs. Doesn't send the thread to sleep.
/// This solution is way more accurate when it comes to times <= 100µs.
/// Because context switches of the threads is too slow on Raspberry Pi (3).
pub fn sleep_busy_waiting(us: u64) {
    let target_time = Instant::now().add(Duration::from_micros(us));
    loop {
        if Instant::now() >= target_time {
            break;
        }
    }
}
