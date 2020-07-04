//! This module is only for testing with a dummy. This way I can execute it on my
//! Mac without an actual GPIO interface. Because of this I can set breakpoints and so on..

use crate::TM1637Adapter;
use alloc::boxed::Box;
use crate::GpioPinValue::LOW;

/// Setups a dummy Adapter for testing.
pub fn setup_dummy() -> TM1637Adapter {
    // set up all the wrapper functions that connects the tm1637-driver with wiringpi
    let pin_clock_mode_fn = Box::from(|_| {});
    let pin_clock_write_fn = Box::from(|_| {});
    let pin_dio_mode_fn = Box::from(|_| {});
    let pin_dio_write_fn = Box::from(|_| {});
    let pin_dio_read_fn = Box::from(|| LOW);
    let bit_delay_fn = Box::from(|| {});

    // pass all wrapper functions to the adapter.
    TM1637Adapter::new(
        pin_clock_mode_fn,
        pin_clock_write_fn,
        pin_dio_mode_fn,
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
        let mut f = setup_dummy();;
        fourdigit7segdis::display_text_banner_in_loop(&mut f, "    Hallo", &|| {});
    }
}
