//! Utility functions on top of the TM1637 driver to display content on the 4-digit 7-segment
//! display by AZDelivery. You can use them but you don't have to. They show how the driver
//! works/could be used.

pub const DISPLAY_COUNT: usize = 4;

use crate::{TM1637Adapter, DisplayState, Brightness};
use alloc::string::String;

/// Displays a text over and over again. The text will move "animated" accross the
/// screen from right to left.
pub fn display_text_banner_in_loop(ad: &mut TM1637Adapter, text: &str, sleep_fn: &dyn Fn()) {
    ad.set_display_state(DisplayState::ON);
    ad.set_brightness(Brightness::L7);

    // remove dots because this display only has one double point which looks weird.
    let text = text.replace(".", " ");

    // format to 7 segment bits
    let data = TM1637Adapter::encode_string(
        // 4 spaces because DISPLAY_COUNT == 4
        &format!("    {}    ", text)
    );

    // display this text over and over again
    loop {
        for x in 0..(data.len() - DISPLAY_COUNT) {
            tm1637display.write_segments_raw(&data[x..(x + DISPLAY_COUNT)], 4, 0);
            sleep_fn();
        }
    }
}
