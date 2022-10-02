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

//! This module is only for testing with a dummy. This way I can execute it on my
//! Mac without an actual GPIO interface. Because of this I can set breakpoints and so on..

use crate::TM1637Adapter;
use alloc::boxed::Box;
use crate::GpioPinValue::LOW;

/// Setups a dummy Adapter for testing.
pub fn setup_dummy() -> TM1637Adapter {
    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_write_fn = Box::from(|_| {});
    let pin_dio_write_fn = Box::from(|_| {});
    let pin_dio_read_fn = Box::from(|| LOW);
    let bit_delay_fn = Box::from(|| {});

    // pass all wrapper functions to the adapter.
    TM1637Adapter::new(
        pin_clock_write_fn,
        pin_dio_write_fn,
        pin_dio_read_fn,
        bit_delay_fn,
    )
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn test() {
        let mut f = setup_dummy();
        // don't check in because this breaks the CI build because of the infinite loop
        //fourdigit7segdis::display_text_banner_in_loop(&mut f, "    Hallo", &|| {});
    }
}
