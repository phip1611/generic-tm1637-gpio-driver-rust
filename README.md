# Generic TM1637 GPIO Driver

Zero-dependency generic GPIO driver for the TM1637 micro controller. 
Doesn't need std-lib and you can choose a GPIO interface/library on your own.

This library was created to understand the functionality of the 4-digit 7-segment display
by AZ-Delivery that uses an TM1637 micro controller. 

See this demo (gif) I made with my Raspberry Pi using regular GPIO pins:

![gpio demonstration](az-delivery-4-digit-7-segment-tm1637.gif)
 
## How does this work? How do I write a driver for that thing?
This was my first time writing a (super simple basic) kind of a device driver.
As of now I'm not that much experienced with micro controllers yet.
After some time I understood how it works by looking at the [data sheet](https://www.mcielectronics.cl/website_MCI/static/documents/Datasheet_TM1637.pdf 
). Have a look into my code. I tried to make as many comments as possible.


**This library doesn't support all features yet!** ~~~~For example I couldn't manage it yet to
properly address the points in the middle of the display you see in the gif.




## How can I use it?
My driver/library is independent from the GPIO interface you want to use.
You can use [crates.io: wiringpi](https://crates.io/crates/wiringpi) or [crates.io: gpio](https://crates.io/crates/gpio).
I tested both on my Raspberry Pi. My `TM1637Adapter` just needs functions/closures 
as parameters. These closures are wrapper-functions to write High/Low to the desired Pins.

## Does this work only on Raspberry Pi?
No! Although I can't test it because I don't have an Arduino or another similar device
this should work on every device where you can create a rust program for. Since this lib
uses no standard library this should work on embedded devices. If you use it let me know
what things you've built!

But yes, it was only tested using regular GPIO pins on my Raspberry Pi running 
Rasbperry Pi OS.
 
### Who Am I?
I'm Philipp :)
Feel free to contribute on [Github](https://github.com/phip1611/generic-tm1637-gpio-driver-rust), write me an Email (phip1611@gmail.com) or
message me on Twitter (https://twitter.com/phip1611)!
 
### Special thanks
Special thanks to https://github.com/avishorp/TM1637. His driver for the Arduino platform
helped me to understand how the TM1637 micro controller works. With his work and my
effort I put into understanding the data sheet I could make this driver.

I don't use any of his code directly. It just gave me inspiration.

### Other
There is another library on crates.io for the TM1637: https://github.com/igelbox/tm1637-rs
It uses the "embedded-hal"-crate and it's code is a little bit shorted/cleaner. On the other side
it's not zero dependency. Check this out too. :)
 
