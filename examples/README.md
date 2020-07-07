Simple examples that shows you how you can use the driver to display
content on the 4-digit 7-segment display by AZDelivery (TM1637).
This is the display shown here:

##### Moving text:

![gpio demonstration](../example-moving-text.gif)

##### Time with blinking double point:

![gpio demonstration](../example-time.gif)

There are multiple examples that runs one after another.
You can use the driver directly or use some of the utility functions provided in module `fourdigit7segdis`.

There are multiple binary files. They all use the same demo.
They just use different gpio apis/libs/interfaces. They are provided
in the main src directory in `gpio_api/`. If you use "gpio" or "sysfs_gpio"
(both crates use sysfs Linux API) you probably must use sudo. The most modern way
is using `gpio-cdev` which uses the new Linux character device API.
