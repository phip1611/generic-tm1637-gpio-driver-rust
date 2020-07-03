#[cfg(feature = "gpio-api-wiringpi")]
mod wiringpi;
#[cfg(feature = "gpio-api-wiringpi")]
pub use super::extern_api::wiringpi::setup_wiringpi;
