//! Provides ready to use setup functions for concrete GPIO libraries/drivers.
//! You may use them but it is not required. You can learn from the code
//! and write setup functions by yourself!
//!
//! These features must be activated in your Cargo.toml of you want to use them.

#[cfg(feature = "gpio-api-wiringpi")]
mod wiringpi;
#[cfg(feature = "gpio-api-wiringpi")]
pub use super::extern_api::wiringpi::setup_wiringpi;


