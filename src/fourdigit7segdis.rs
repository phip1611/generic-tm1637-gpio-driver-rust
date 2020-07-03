//! Utility functions on top of the TM1637 driver to display content on the 4-digit 7-segment
//! display by AZDelivery. You can use them but you don't have to. They show how the driver
//! works/could be used.

pub const DISPLAY_COUNT: usize = 4;

use crate::{TM1637Adapter, DisplayState, Brightness};
use crate::mappings::SegmentBits;
use alloc::string::String;

/// Displays a text over and over again. The text will move "animated" accross the
/// screen from right to left.
/// Blocks the calling thread because this is an infinite loop.
pub fn display_text_banner_in_loop(adapter: &mut TM1637Adapter, text: &str, sleep_fn: &dyn Fn()) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    // remove dots because this display only has one double point which looks weird.
    let data = text.replace(".", " ");
    let data = TM1637Adapter::encode_string(&data);

    // display this text over and over again
    loop {
        for x in 0..(data.len() - DISPLAY_COUNT) {
            adapter.write_segments_raw(&data[x..(x + DISPLAY_COUNT)], 4, 0);
            sleep_fn();
        }
    }
}

/// Displays "hh:mm" with blinking double point on the display.
/// Blocks the calling thread because this is an infinite loop.
pub fn display_current_time_in_loop(adapter: &mut TM1637Adapter,
                            tick_fn: &dyn Fn(),
                            time_fn: &dyn Fn() -> (String, String)) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dots = false;
    loop {
        // could be hh:mm or mm::ss
        let (l, r): (String, String) = (time_fn)();
        let mut data: [u8; DISPLAY_COUNT] = [
            TM1637Adapter::encode_char(l.chars().nth(0).unwrap()),
            TM1637Adapter::encode_char(l.chars().nth(1).unwrap()),
            TM1637Adapter::encode_char(r.chars().nth(0).unwrap()),
            TM1637Adapter::encode_char(r.chars().nth(1).unwrap()),
        ];

        if show_dots {
            data[1] = data[1] | SegmentBits::SegPoint as u8;
        }

        adapter.write_segments_raw(&data, 4, 0);

        (tick_fn)();

        show_dots = !show_dots;
    }
}

/// Maximum value for stopwatch.
pub const STOPWATCH_MAX: u16 = 10_000;

/// Starts a stopwatch aka counter from 0 to 9999.
/// You need to provide a sleep_fn that waits 1s (sets the frequency to 1Hz).
pub fn display_stopwatch(adapter: &mut TM1637Adapter, sleep_fn: &dyn Fn(), to: u16) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dot = false;
    // 0 to 9999
    for i in 0..to {
        let mut data = TM1637Adapter::encode_number(i);
        if show_dot {
            data[1] |= SegmentBits::SegPoint as u8;
        }
        adapter.write_segments_raw(&data, 4, 0);
        show_dot = !show_dot;
        sleep_fn(); // probably this is always a function that sleeps 1s => 1Hz frequency
    }
}
