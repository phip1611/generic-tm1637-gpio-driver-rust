// MIT License
//
// Copyright (c) 2022 Philipp Schuster <phip1611@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Utility functions on top of the TM1637 driver to display content on the 4-digit 7-segment
//! display by AZDelivery. You can use them but you don't have to. They show how the driver
//! works/could be used.

/// We have 4 displays so we can display 4 digits.
pub const DISPLAY_COUNT: usize = 4;

use crate::mappings::SegmentBits;
use crate::{Brightness, DisplayState, TM1637Adapter};
use alloc::string::String;

/// Displays a text over and over again. The text will move "animated" accross the
/// screen from right to left.
/// Blocks the calling thread because this is an infinite loop.
pub fn display_text_banner_in_loop(adapter: &mut TM1637Adapter, text: &str, sleep_fn: &dyn Fn()) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    // remove dots because this display only has one double point which looks weird.
    let data = text.replace('.', " ");
    let data = TM1637Adapter::encode_string(&data);

    // +1 because the upper border in a range is exclusive
    // otherwise last char is lost!
    let to = (data.len() - DISPLAY_COUNT) + 1;

    // display this text over and over again
    loop {
        for x in 0..to {
            let data_slice = &data[x..(x + DISPLAY_COUNT)];
            adapter.write_segments_raw(data_slice, 0);
            sleep_fn();
        }
    }
}

/// Displays "hh:mm" with blinking double point on the display.
/// Blocks the calling thread because this is an infinite loop.
pub fn display_current_time_in_loop(
    adapter: &mut TM1637Adapter,
    tick_fn: &dyn Fn(),
    time_fn: &dyn Fn() -> (String, String),
) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dots = false;
    loop {
        // could be hh:mm or mm::ss
        let (l, r): (String, String) = (time_fn)();
        #[allow(clippy::iter_nth_zero)]
        let mut data: [u8; DISPLAY_COUNT] = [
            TM1637Adapter::encode_char(l.chars().nth(0).unwrap()),
            TM1637Adapter::encode_char(l.chars().nth(1).unwrap()),
            TM1637Adapter::encode_char(r.chars().nth(0).unwrap()),
            TM1637Adapter::encode_char(r.chars().nth(1).unwrap()),
        ];

        if show_dots {
            data[1] |= SegmentBits::SegPoint as u8;
        }

        adapter.write_segments_raw(&data, 0);

        (tick_fn)();

        show_dots = !show_dots;
    }
}

/// Maximum value for stopwatch.
pub const STOPWATCH_MAX: u16 = 10_000;

/// Starts a stopwatch aka counter from 0 to 9999.
/// You need to provide a sleep_fn that waits 1s (for stopwatch).
pub fn display_stopwatch(adapter: &mut TM1637Adapter, sleep_fn: &dyn Fn(), to: u16, blink: bool) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dot = false;
    // 0 to 9999
    for i in 0..to {
        let mut data = TM1637Adapter::encode_number(i);
        if blink && show_dot {
            data[1] |= SegmentBits::SegPoint as u8;
        }
        adapter.write_segments_raw(&data, 0);
        show_dot = !show_dot;
        sleep_fn(); // probably this is always a function that sleeps 1s => 1Hz frequency
    }
}

/// Starts a timer from x to 0. Needs a sleep_fn (probably one that sleeps for one second / 1Hz).
/// Displays pure seconds. No minutes:seconds.
// don't to blink here, because it would look like "mins:secs" which is not true. In this case this
// is a pure seconds timer.
pub fn display_timer(adapter: &mut TM1637Adapter, sleep_fn: &dyn Fn(), from_val: u16) {
    adapter.set_display_state(DisplayState::ON);
    adapter.set_brightness(Brightness::L7);

    let mut show_dot = false;
    // 0 to 9999
    for i in 0..(from_val + 1) {
        let i = from_val - i;
        let data = TM1637Adapter::encode_number(i);
        adapter.write_segments_raw(&data, 0);
        show_dot = !show_dot;
        sleep_fn(); // probably this is always a function that sleeps 1s => 1Hz frequency
    }

    // blinking with just zeros to show that timer is done
    for i in 0..4 {
        let data = if i % 2 == 0 {
            [0; 4]
        } else {
            [TM1637Adapter::encode_digit(0); 4]
        };
        adapter.write_segments_raw(&data, 0);
        sleep_fn(); // probably this is always a function that sleeps 1s => 1Hz frequency
    }
    adapter.clear();
}
