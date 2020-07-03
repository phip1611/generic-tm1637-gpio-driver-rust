//! Utility functions on top of the TM1637 driver to display content on the 4-digit 7-segment
//! display by AZDelivery. You can use them but you don't have to. They show how the driver
//! works/could be used.

pub const DISPLAY_COUNT: usize = 4;

use crate::{TM1637Adapter, DisplayState, Brightness};
use crate::mappings::SegmentBits;

/// Displays a text over and over again. The text will move "animated" accross the
/// screen from right to left.
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

pub fn display_current_time(adapter: &mut TM1637Adapter,
                                  tick_fn: &dyn Fn(),
                                  time_fn: &dyn Fn() -> (&str, &str)) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dots = false;
    loop {
        // could be hh:mm or mm::ss
        let (l, r) = (time_fn)();
        let mut data: [u8; DISPLAY_COUNT] = [
            TM1637Adapter::encode_char(l[0]),
            TM1637Adapter::encode_char(l[1]),
            TM1637Adapter::encode_char(r[0]),
            TM1637Adapter::encode_char(r[1]),
        ];

        if show_dots {
            data[1] = data[1] | SegmentBits::SegPoint as u8;
        }

        (tick_fn)();

        show_dots = !show_dots;
    }
}