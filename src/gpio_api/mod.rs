//! Provides ready to use setup functions for concrete GPIO libraries/drivers.
//! You may use them but it is not required. You can learn from the code
//! and write setup functions by yourself!
//!
//! These features must be activated in your Cargo.toml of you want to use them.

// uses "wiringpi"-crate
#[cfg(feature = "gpio-api-wiringpi")]
mod wiringpi;
#[cfg(feature = "gpio-api-wiringpi")]
pub use super::gpio_api::wiringpi::setup_wiringpi;

// #############################################################################

// uses "gpio"-crate. This uses sysfs, which will be deprecated in linux kernel in 2020
// see https://crates.io/crates/gpio-cdev
#[cfg(feature = "gpio-api-gpio")]
mod gpio;
#[cfg(feature = "gpio-api-gpio")]
pub use super::gpio_api::gpio::setup_gpio;

// #############################################################################

// uses "sysfs_gpio"-crate
#[cfg(feature = "gpio-api-sysfs_gpio")]
mod sysfs_gpio;
#[cfg(feature = "gpio-api-sysfs_gpio")]
pub use super::gpio_api::sysfs_gpio::setup_sysfs_gpio;

// #############################################################################

// uses "gpio_cdev"-crate
#[cfg(feature = "gpio-api-gpio_cdev")]
mod gpio_cdev;
#[cfg(feature = "gpio-api-gpio_cdev")]
pub use super::gpio_api::gpio_cdev::setup_gpio_cdev;

// #############################################################################

// this module is only necessary/useful for testing and debugging on machines that
// do not have an gpio interface. This way one can set breakpoints.
#[cfg(feature = "dummy")]
mod dummy;
#[cfg(feature = "dummy")]
pub use super::gpio_api::dummy::setup_dummy;
