#![no_std]

// MIT License. See LICENSE file.

// Made by:
//   Philipp Schuster
//   phip1611@gmail.com

// Inspired by / Special thanks to:
// https://github.com/avishorp/TM1637

//! Zero-dependency generic GPIO driver for TM1637.
//! With this driver you can control for example the 4-digit 7-segment display from AZ-Delivery.
//! This is not dependent on a specific GPIO interface.
//! This library was tested on a Raspberry Pi with its GPIO interface.
//! This library doesn't support all features of TM1637 (yet).
//! Feel free to contribute. :)

// rust core library; no external crate; needed because no_std
extern crate alloc;

// use Box: we don't have std::prelude here
use alloc::boxed::Box;
use crate::SegmentBits::SegA;
use alloc::vec::Vec;

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

/// Shows which segment has which bit.
#[repr(u8)]
pub enum SegmentBits {
    SegA = 0b00000001,
    SegB = 0b00000010,
    SegC = 0b00000100,
    SegD = 0b00001000,
    SegE = 0b00010000,
    SegF = 0b00100000,
    SegG = 0b01000000,
    // double point on AzDelivery 4-digit 7 segment display.
    SegPoint = 0b10000000,
}

/// Array that maps a digit (0-9) to its bits representation
/// on the 7 segment display. Get the bits by indexing this array.
const DIGITS_TO_BITS: [u8; 10] = [
    // 0
    0b00111111,
    // 1
    0b00000110,
    // ...
    0b01011011,
    0b01001111,
    0b01100110,
    0b01101101,
    0b01111101,
    0b00000111,
    0b01111111,
    // 9
    0b01101111,
];

/// Maps a upper case/capital character to its 7-segment bit representation.
#[repr(u8)]
pub enum CapitalLettersToSegmentBits {
    A = SegmentBits::SegA | SegmentBits::SegB | SegmentBits::SegF | SegmentBits::SegG | SegmentBits::SegE | SegmentBits::SegC,
    C = SegmentBits::SegA | SegmentBits::SegF | SegmentBits::SegE | SegmentBits::SegD,
    E = SegmentBits::SegA | SegmentBits::SegF | SegmentBits::SegE | SegmentBits::SegD | SegmentBits::SegG,
    F = SegmentBits::SegA | SegmentBits::SegF | SegmentBits::SegE | SegmentBits::SegG,
    // and so on :)
    G = 0x3d,
    H = 0x76,
    I = 0x30,
    J = 0x1E,
    L = 0x38,
    O = 0x3F,
    P = 0x73,
    S = 0x6D,
    U = 0x3E
}

/// Maps a lower case character to its 7-segment bit representation.
#[repr(u8)]
pub enum LettersToSegmentBits {
    A = 0x5F,
    B = 0x7C,
    C = 0x58,
    D = 0x5E,
    H = 0x74,
    N = 0x54,
    O = 0x5c,
    Q = 0x67,
    R = 0x50,
    T = 0x78,
    U = 0x1C,
    Y = 0x6E
}

#[repr(u8)]
pub enum SymbolsToSegmentBits {
    SPACE = 0,
    MINUS = SegmentBits::SegG as u8,
    UNDERSCORE = SegmentBits::SegD as u8,
    EQUALS = SegmentBits::SegG | SegmentBits::SegD,
    QUESTION_MARK = SegmentBits::SegA | SegmentBits::SegB | SegmentBits::SegG | SegmentBits::SegE,
    DOT = SegmentBits::SegPoint as u8
}

/// Mode of GPIO Pins.
#[repr(u8)]
pub enum GpioPinMode {
    /// Input-Pin.
    INPUT,
    /// Output-Pin.
    OUTPUT,
}

/// The value of a GPIO pin.
#[repr(u8)]
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
    /// Function that changes the mode of the data in and out pin.
    pin_dio_mode_fn: Box<dyn Fn(GpioPinMode)>,
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
    pub fn new(pin_clock_mode_fn: Box<dyn Fn(GpioPinMode)>,
               pin_clock_write_fn: Box<dyn Fn(GpioPinValue)>,
               pin_dio_mode_fn: Box<dyn Fn(GpioPinMode)>,
               pin_dio_write_fn: Box<dyn Fn(GpioPinValue)>,
               pin_dio_read_fn: Box<dyn Fn() -> GpioPinValue>,
               bit_delay_fn: Box<dyn Fn() -> ()>) -> TM1637Adapter {

        // assume both are already output pins - this is the contract that needs to be fulfilled!
        (pin_clock_mode_fn)(GpioPinMode::OUTPUT);
        (pin_dio_mode_fn)(GpioPinMode::OUTPUT);
        (pin_clock_write_fn)(GpioPinValue::LOW);
        (pin_dio_write_fn)(GpioPinValue::LOW);

        TM1637Adapter {
            pin_clock_write_fn,
            pin_dio_mode_fn,
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
    pub fn write_segments_raw(&self, segments: &[u8], mut n: u8, pos: u8) {
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
        self.write_segments_raw(&[segments], 1, position)
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
        self.write_segments_raw(&[0, 0, 0, 0, 0, 0], 6, 0);
    }

    /// Writes a byte bit by bit and waits for the acknowledge.
    fn write_byte_and_wait_ack(&self, byte: u8) {
        let mut data = byte;

        // 8 bits
        for _ in 0..8 {
            // CLK low
            (self.pin_clock_write_fn)(GpioPinValue::LOW);
            self.bit_delay();

            // Set data bit (we send one bit of our byte per iteration)
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


    /// Encodes a digit from 0 to 9 to it's bit representation on the display.
    pub fn encode_digit(digit: u8) -> u8 {
        let digit = digit % 10;
        DIGITS_TO_BITS[digit as usize]
    }

    /// Encodes a char for the 7-segment display.
    /// Unknown chars will be a zero byte (space).
    pub fn encode_char(c: char) -> u8 {
        {
            if c == 'A' { CapitalLettersToSegmentBits::A }
            else if c == 'a' { LettersToSegmentBits::A }
            else if c == 'b' { LettersToSegmentBits::B }
            else if c == 'C' { CapitalLettersToSegmentBits::C }
            else if c == 'd' { LettersToSegmentBits::D }
            else if c == 'E' { CapitalLettersToSegmentBits::E }
            else if c == 'F' { CapitalLettersToSegmentBits::F }
            else if c == 'G' { CapitalLettersToSegmentBits::G }
            else if c == 'H' { CapitalLettersToSegmentBits::H }
            else if c == 'h' { LettersToSegmentBits::H }
            else if c == 'I' { CapitalLettersToSegmentBits::I }
            else if c == 'J' { CapitalLettersToSegmentBits::J }
            else if c == 'L' { CapitalLettersToSegmentBits::L }
            else if c == 'n' { LettersToSegmentBits::N }
            else if c == 'O' { CapitalLettersToSegmentBits::O }
            else if c == 'o' { LettersToSegmentBits::O }
            else if c == 'P' { CapitalLettersToSegmentBits::P }
            else if c == 'q' { LettersToSegmentBits::Q }
            else if c == 'r' { LettersToSegmentBits::R }
            else if c == 'S' { CapitalLettersToSegmentBits::S }
            else if c == 't' { LettersToSegmentBits::T }
            else if c == 'U' { CapitalLettersToSegmentBits::U }
            else if c == 'u' { LettersToSegmentBits::U }
            else if c == 'y' { LettersToSegmentBits::Y }
            else if c == ' ' { SymbolsToSegmentBits::SPACE }
            else if c == '?' { SymbolsToSegmentBits::QUESTION_MARK }
            else if c == '-' { SymbolsToSegmentBits::MINUS }
            else if c == '_' { SymbolsToSegmentBits::UNDERSCORE }
            else if c == '=' { SymbolsToSegmentBits::EQUALS }
            else if c == '.' { SymbolsToSegmentBits::DOT }
            else { SymbolsToSegmentBits::SPACE }
        } as u8
    }

    /// Encodes a string for the 7-segment display.
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

        // prepare read
        (self.pin_dio_mode_fn)(GpioPinMode::INPUT);
        self.bit_delay();
        let ack: GpioPinValue = (self.pin_dio_read_fn)();
        // todo maybe wait 3-5 cycles until 0 was found to be more failsafe?!
        if ack as u8 != 0 {
            // ACK should be one clock with zero on data lane
            // not possible with no_std; TODO provide debug function
            // eprintln!("ack is not 0! Probably not a problem, tho.")
        }

        (self.pin_dio_mode_fn)(GpioPinMode::OUTPUT);
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
