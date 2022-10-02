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
#![no_std]

// Inspired by / Special thanks to:
// https://github.com/avishorp/TM1637

//! Generic GPIO driver for TM1637.
//! With this driver you can control for example the 4-digit 7-segment display from AZ-Delivery.
//! This is not dependent on a specific GPIO library.
//! This library was tested on a Raspberry Pi with its GPIO interface.
//! Feel free to contribute. :)

// from rust core library; no "external crate" in the manner that this is no crates.io dependency;
// needed because no_std
#[macro_use] // wee need the !format macro
extern crate alloc;

// Import our enums/arrays for the symbol mappings to the 7 segment display
pub mod mappings;
#[cfg(feature = "fourdigit7segdis")]
pub mod fourdigit7segdis;
// provides conditionally bindings to specific GPIO interfaces; can be activated via cargo features
pub mod gpio_api;

// to use Box: we don't have std::prelude here
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::mappings::{NumCharBits, UpCharBits, SpecialCharBits, LoCharBits};

//       A
//      ---
//  F  |   |  B
//      -G-
//  E  |   |  C
//      ---
//       D

/// According to the data sheet the TM1637 can address 6 display registers.
/// Note that not all devices using it do as well. For example the 4-digit
/// 7-segment display from AzDelivery only uses 4.
pub const DISPLAY_REGISTERS_COUNT: usize = 6;

/// The value of a GPIO pin.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GpioPinValue {
    /// Low.
    LOW,
    /// High.
    HIGH,
}

impl From<u8> for GpioPinValue {
    fn from(x: u8) -> Self {
        if x == 0 { GpioPinValue::LOW } else { GpioPinValue::HIGH }
    }
}

/// Adapter between your code and the TM1637 via GPIO interface.
/// You can use the GPIO interface/library that you want. Just provide
/// the corresponding "glue" functions so that this adapter can access GPIO.
///
/// Be wise when you choose a value for `bit_delay_us`. This delay is important
/// to ensure that changed signals are actually on the pins. My experience showed
/// that 100 (µs) is a safe value on the Raspberry Pi.
pub struct TM1637Adapter {
    /// Function that writes the value on the GPIO pin that acts as the clock.
    pin_clock_write_fn: Box<dyn Fn(GpioPinValue)>,
    /// Function that writes the value on the GPIO pin that acts as data in and out.
    pin_dio_write_fn: Box<dyn Fn(GpioPinValue)>,
    /// Function that reads from the data in and out pin.
    pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue>,
    /// Delay function after data bits and clock bits have been set. This is necessary
    /// because the changed bits must actually arrive on the hardware. Tests showed that
    /// at least 100µs is safe on Raspberry Pi with it's GPIO interface. Please be aware that
    /// high frequencies (clock) can become really fast really hard if cables get longer!
    bit_delay_fn: Box<dyn Fn() -> ()>,
    /// Representation of the display state in bits for the TM1637.
    /// Bits 7-4 are zero. Later the "display control"-command prefix will be there.
    /// Bits 3-0 are for display on/off and brightness.
    brightness: u8,
}

/// The level of brightness.
/// The TM1637 "DisplayControl"-command transports the brightness information
/// in bits 0 to 2.
#[repr(u8)]
pub enum Brightness {
    // useless assignment because it is default but it shows clearly
    // that 3 bits are used
    /// Lowest brightness.
    L0 = 0b000,
    L1 = 0b001,
    L2 = 0b010,
    L3 = 0b011,
    L4 = 0b100,
    L5 = 0b101,
    L6 = 0b110,
    /// Highest brightness.
    L7 = 0b111,
}

/// Whether the display is on or off.
/// The TM1637 "DisplayControl"-command transports the display on/off information
/// in the third bit (2^3) of the command.
#[repr(u8)]
pub enum DisplayState {
    /// Display off.
    OFF = 0b0000,
    /// Display On.
    ON = 0b1000,
}

/// The "ISA"/Commands of the TM1637. See data sheet
/// for more information. This is only a subset of the possible values.
#[repr(u8)]
pub enum ISA {
    /// Start instruction
    DataCommandWriteToDisplay = 0b0100_0000, // "write data to display register"-mode

    // send this + <recv ack> + send byte 0 + <recv ack> + ... send byte 3
    /// Starts at display address zero. Each further byte that is send will go
    /// into the next display address. The micro controller does an internal auto increment
    /// of the address. See the data sheet for more information.
    AddressCommandD0 = 0b1100_0000,
    AddressCommandD1 = 0b1100_0001,
    AddressCommandD2 = 0b1100_0010,
    AddressCommandD3 = 0b1100_0011,

    // bits 0 - 2 tell the brightness.
    // bit 3 is display on/off
    /// Command that sets the display off.
    DisplayControlOff = 0b1000_0000,
    /// Command that sets the display on with lowest brightness.
    DisplayControlOnL0 = 0b1000_1000,
    DisplayControlOnL1 = 0b1000_1001,
    DisplayControlOnL2 = 0b1000_1010,
    DisplayControlOnL3 = 0b1000_1011,
    DisplayControlOnL4 = 0b1000_1100,
    DisplayControlOnL5 = 0b1000_1101,
    DisplayControlOnL6 = 0b1000_1110,
    /// Command that sets the display on with highest brightness.
    DisplayControlOnL7 = 0b1000_1111,

    /*
    these are the 3 base commands: to see the meaning
    of bits 0 to 5 see data sheet;
    6 & 7 are reserved to mark the kind of command
    // data command
    COMM1_BASE = 0b0100_000,
    // addressing mode
    COMM2_BASE = 0b1100_000,
    // display control
    COMM3_BASE = 0b1000_000,*/
}

impl TM1637Adapter {
    /// Creates a new object to interact via GPIO with a TM1637.
    ///
    /// * `pin_clock_write_fn` function to write bit to CLK pin
    /// * `pin_dio_write_fn` function to write bit to DIO pin
    /// * `pin_dio_read_fn` function to read value from DIO pin
    /// * `bit_delay_fn` function that is invoked after a bit has been written to a pin.
    ///                  Probably 1 or even 0 µs are fine. This is just to be sure. It depends
    ///                  on your hardware and your GPIO driver.
    pub fn new(pin_clock_write_fn: Box<dyn Fn(GpioPinValue)>,
               pin_dio_write_fn: Box<dyn Fn(GpioPinValue)>,
               pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue>,
               bit_delay_fn: Box<dyn Fn() -> ()>) -> TM1637Adapter {

        // assume both are already output pins - this is the contract that needs to be fulfilled!
        (pin_clock_write_fn)(GpioPinValue::LOW);
        (pin_dio_write_fn)(GpioPinValue::LOW);

        TM1637Adapter {
            pin_clock_write_fn,
            pin_dio_write_fn,
            pin_dio_read_fn,
            bit_delay_fn,
            brightness: DisplayState::ON as u8 | Brightness::L7 as u8,
        }
    }

    /// Sets the display state. The display state is the 3rd bit of the
    /// "display control"-command.
    /// This setting is not committed until a write operation has been made.
    pub fn set_display_state(&mut self, ds: DisplayState) {
        // keep old state for brightness
        let old_brightness = self.brightness & 0b0000_0111;
        // take 3rd bit (the one that says display on/off) into the new value
        self.brightness = ds as u8 | old_brightness;
    }

    /// Sets the brightness of the screen. The brightness are the lower
    /// 3 bits of the "display control"-command.
    /// This setting is not committed until a write operation has been made.
    pub fn set_brightness(&mut self, brightness: Brightness) {
        // look if display is configured as on
        let display_on = self.brightness as u8 & 0b0000_1000;
        self.brightness = display_on | brightness as u8;
    }

    /// Writes all raw segments data beginning at the position into the display registers.
    /// It uses auto increment internally to write into all further registers.
    /// This functions does an internal check so that not more than 6 registers can be
    /// addressed/written.
    /// * `segments` Raw data describing the bits of the 7 segment display.
    /// * `n` Length of segments array.
    /// * `pos` The start position of the display register. While bytes are
    ///         written, address is adjusted internally via auto increment.
    ///         Usually this is 0, if you want to write data to all 7 segment
    ///         displays.
    pub fn write_segments_raw(&self, segments: &[u8], pos: u8) {
        let mut n = segments.len() as u8;
        // beeing a little bit more failure tolerant
        if n == 0 { return; } // nothing to do
        let pos = pos % DISPLAY_REGISTERS_COUNT as u8; // only valid positions/registers

        // valid values are
        //   n = 1, pos = {0, 1, 2, 3, 4, 5}
        //   n = 2, pos = {0, 1, 2, 3, 4}
        //   n = 3, pos = {0, 1, 2, 3}
        //   n = 4, pos = {0, 1, 2}
        //   n = 5, pos = {0, 1}
        //   n = 6, pos = {0}
        // => n + pos must be <= DISPLAY_REGISTERS_COUNT

        if n + pos > DISPLAY_REGISTERS_COUNT as u8 {
            // only write as much data as registers are available
            n = DISPLAY_REGISTERS_COUNT as u8 - pos;
        }

        // Command 1
        // for more information about this flow: see data sheet / specification of TM1637
        // or AZDelivery's 7 segment display
        self.start();
        self.write_byte_and_wait_ack(ISA::DataCommandWriteToDisplay as u8);
        self.stop();

        // Write COMM2
        self.start();
        self.write_byte_and_wait_ack(ISA::AddressCommandD0 as u8 | pos);

        // Write the remaining data bytes
        // TM1637 does auto increment internally

        //println!("n={}, pos={}, data={:#?}", n, pos, segments);

        for i in 0..n {
            self.write_byte_and_wait_ack(segments[i as usize]);
        }
        self.stop();

        // we do this everytime because it will be a common flow that people write something
        // and expect the display to be on
        self.write_display_state();
    }


    /// This uses fixed address mode (see data sheet) internally to write data to
    /// a specific position of the display.
    /// Position is 0, 1, 2, or 3.
    pub fn write_segment_raw(&self, segments: u8, position: u8) {
        self.write_segments_raw(&[segments], position)
    }

    /// Send command that sets the display state on the micro controller.
    pub fn write_display_state(&self) {
        self.start();
        // bits 0-2 brightness; bit 3 is on/off
        self.write_byte_and_wait_ack(ISA::DisplayControlOff as u8 | self.brightness);
        self.stop();
    }

    /// Clears the display.
    pub fn clear(&self) {
        // begin at position 0 and write 0 into display registers 0 to 5
        self.write_segments_raw(&[0, 0, 0, 0, 0, 0], 0);
    }

    /// Writes a byte bit by bit and waits for the acknowledge.
    fn write_byte_and_wait_ack(&self, byte: u8) {
        let mut data = byte;

        // 8 bits
        for _ in 0..8 {
            // CLK low
            (self.pin_clock_write_fn)(GpioPinValue::LOW);
            // Set data bit (we send one bit of our byte per iteration)
            // LSF (least significant bit) first
            // => target has (probably?) shift register => this way the byte has the
            // correct order on the target
            (self.pin_dio_write_fn)(GpioPinValue::from(data & 0x01));
            self.bit_delay();

            // CLK high
            (self.pin_clock_write_fn)(GpioPinValue::HIGH);
            self.bit_delay();

            // shift to next bit
            data = data >> 1;
        }

        self.recv_ack();
    }

    /// Encodes a number from 0 to 9999 on the display.
    pub fn encode_number(num: u16) -> [u8; 4] {
        let mut num = num % 10000;
        let mut bits: [u8; 4] = [0; 4];
        for i in 0..4 {
            let digit = (num % 10) as u8;
            bits[3 - i] = TM1637Adapter::encode_digit(digit);
            num = num / 10;
        }
        bits
    }

    /// Encodes a number/digit from 0 to 9 to it's bit representation on the display.
    /// This is not the char (ASCII) representation. It's a number/integer.
    pub fn encode_digit(digit: u8) -> u8 {
        let digit = digit % 10;
        if digit == 0 { NumCharBits::Zero as u8 }
        else if digit == 1 { NumCharBits::One as u8 }
        else if digit == 2 { NumCharBits::Two as u8 }
        else if digit == 3 { NumCharBits::Three as u8 }
        else if digit == 4 { NumCharBits::Four as u8 }
        else if digit == 5 { NumCharBits::Five as u8 }
        else if digit == 6 { NumCharBits::Six as u8 }
        else if digit == 7 { NumCharBits::Seven as u8 }
        else if digit == 8 { NumCharBits::Eight as u8 }
        // else if digit == 9 { NumCharBits::Nine as u8 }
        else { NumCharBits::Nine as u8 }
    }

    /// Encodes a char for the 7-segment display. Unknown chars will be a zero byte (space).
    /// Uses `mappings::UpCharBits` and `mappings::LoCharBits` for the chars. Since there is
    /// no representation for every char in each case (lower, upper) there will be an replacement
    /// for lowercase charts by their uppercase counterpart and vice versa.
    pub fn encode_char(c: char) -> u8 {
        // nums
        if c == '0' { NumCharBits::Zero as u8 }
        else if c == '1' { NumCharBits::One as u8 }
        else if c == '2' { NumCharBits::Two as u8 }
        else if c == '3' { NumCharBits::Three as u8 }
        else if c == '4' { NumCharBits::Four as u8 }
        else if c == '5' { NumCharBits::Five as u8 }
        else if c == '6' { NumCharBits::Six as u8 }
        else if c == '7' { NumCharBits::Seven as u8 }
        else if c == '8' { NumCharBits::Eight as u8 }
        else if c == '9' { NumCharBits::Nine as u8 }

        // latin chars
        // we map as accurate as possible,
        // e.g.: a => lowercase a, A => uppercase A
        //
        // but in cases where we only have on mapping available
        // like: b => lowercase b, B => undefined
        // we map B => lowercase b
        else if c == 'A' { UpCharBits::UpA as u8 }
        else if c == 'a' { LoCharBits::LoA as u8 }
        else if c == 'B' { LoCharBits::LoB as u8 }
        else if c == 'b' { LoCharBits::LoB as u8 }
        else if c == 'C' { UpCharBits::UpC as u8 }
        else if c == 'c' { UpCharBits::UpC as u8 }
        else if c == 'D' { LoCharBits::LoD as u8 }
        else if c == 'd' { LoCharBits::LoD as u8 }
        else if c == 'E' { UpCharBits::UpE as u8 }
        else if c == 'e' { UpCharBits::UpE as u8 }
        else if c == 'F' { UpCharBits::UpF as u8 }
        else if c == 'f' { UpCharBits::UpF as u8 }
        else if c == 'G' { UpCharBits::UpG as u8 }
        else if c == 'g' { UpCharBits::UpG as u8 }
        else if c == 'H' { UpCharBits::UpH as u8 }
        else if c == 'h' { LoCharBits::LoH as u8 }
        else if c == 'I' { UpCharBits::UpI as u8 }
        else if c == 'i' { UpCharBits::UpI as u8 }
        else if c == 'J' { UpCharBits::UpJ as u8 }
        else if c == 'j' { UpCharBits::UpJ as u8 }
        else if c == 'L' { UpCharBits::UpL as u8 }
        else if c == 'l' { UpCharBits::UpL as u8 }
        else if c == 'N' { LoCharBits::LoN as u8 }
        else if c == 'n' { LoCharBits::LoN as u8 }
        else if c == 'O' { UpCharBits::UpO as u8 }
        else if c == 'o' { LoCharBits::LoO as u8 }
        else if c == 'P' { UpCharBits::UpP as u8 }
        else if c == 'p' { UpCharBits::UpP as u8 }
        else if c == 'Q' { LoCharBits::LoQ as u8 }
        else if c == 'q' { LoCharBits::LoQ as u8 }
        else if c == 'R' { LoCharBits::LoR as u8 }
        else if c == 'r' { LoCharBits::LoR as u8 }
        else if c == 'S' { UpCharBits::UpS as u8 }
        else if c == 's' { UpCharBits::UpS as u8 }
        else if c == 'T' { LoCharBits::LoT as u8 }
        else if c == 't' { LoCharBits::LoT as u8 }
        else if c == 'U' { UpCharBits::UpU as u8 }
        else if c == 'u' { LoCharBits::LoU as u8 }
        else if c == 'Y' { LoCharBits::LoY as u8 }
        else if c == 'y' { LoCharBits::LoY as u8 }

        // special chars
        else if c == ' ' { SpecialCharBits::Space as u8 }
        else if c == '?' { SpecialCharBits::QuestionMark as u8 }
        else if c == '-' { SpecialCharBits::Minus as u8 }
        else if c == '_' { SpecialCharBits::Underscore as u8 }
        else if c == '=' { SpecialCharBits::Equals as u8 }
        else if c == '.' { SpecialCharBits::Dot as u8 }
        else { SpecialCharBits::Space as u8 }
    }

    /// Encodes a string for the 7-segment display. This uses
    /// `encode_char` for each character.
    pub fn encode_string(str: &str) -> Vec<u8> {
        str.chars().into_iter()
            .map(|c| TM1637Adapter::encode_char(c))
            .collect()
    }

    /// This tells the TM1637 that data input starts.
    /// This information stands in the official data sheet.
    #[inline]
    fn start(&self) {
        (self.pin_dio_write_fn)(GpioPinValue::HIGH);
        (self.pin_clock_write_fn)(GpioPinValue::HIGH);
        self.bit_delay();
        (self.pin_dio_write_fn)(GpioPinValue::LOW);
        self.bit_delay();

        // transition from high to low on DIO while CLK is high
        // means: data starts at next clock
    }

    /// This tells the TM1637 that data input stops.
    /// This information stands in the official data sheet.
    #[inline]
    fn stop(&self) {
        (self.pin_dio_write_fn)(GpioPinValue::LOW);
        (self.pin_clock_write_fn)(GpioPinValue::HIGH);
        self.bit_delay();
        (self.pin_dio_write_fn)(GpioPinValue::HIGH);
        self.bit_delay();
    }

    /// Receives one acknowledgment after a byte was sent.
    fn recv_ack(&self) {
        (self.pin_clock_write_fn)(GpioPinValue::LOW);
        (self.pin_dio_write_fn)(GpioPinValue::LOW);
        self.bit_delay();
        (self.pin_clock_write_fn)(GpioPinValue::HIGH);

        let ack: GpioPinValue = (self.pin_dio_read_fn)();

        // wait a few cycles for ACK to be more fail safe
        for _ in 0..10 {
            if ack as u8 == 0 { break; }
            else {
                // ACK should be one clock with zero on data lane

                // not possible with no_std; TODO provide debug function
                // eprintln!("ack is not 0! Probably not a problem, tho.")
            }
        }

        (self.pin_clock_write_fn)(GpioPinValue::LOW);
        (self.pin_dio_write_fn)(GpioPinValue::LOW);
        self.bit_delay();
    }

    /// Let the current thread sleep for the configured amount of µs.
    /// This is necessary so that changed values on the pins (High, Low)
    /// are applied. The best value here depends on your platform.
    /// 100µs on Raspberry Pi with GPIO-Pins seems perfectly fine.
    #[inline]
    fn bit_delay(&self) {
        (self.bit_delay_fn)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_number() {
        let f = TM1637Adapter::encode_digit;
        assert_eq!([f(1), f(2), f(3), f(4)], TM1637Adapter::encode_number(1234));
        assert_eq!([f(9), f(9), f(9), f(9)], TM1637Adapter::encode_number(9999));
        assert_eq!([f(0), f(0), f(0), f(0)], TM1637Adapter::encode_number(10000));
        assert_eq!([f(7), f(6), f(5), f(4)], TM1637Adapter::encode_number(7654));
    }
}
