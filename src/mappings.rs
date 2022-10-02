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

//! Provides mappings from symbols to their bit representation on a 7-segment display.

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

#[repr(u8)]
pub enum NumCharBits {
    Zero = 0b00111111,
    One = 0b00000110,
    Two = 0b01011011,
    Three = 0b01001111,
    Four = 0b01100110,
    Five = 0b01101101,
    Six = 0b01111101,
    Seven = 0b00000111,
    Eight = 0b01111111,
    Nine = 0b01101111,
}

/// Maps a character to its closest possible representation on a 7-segment display.
/// The 8th segment is the dot.
#[repr(u8)]
pub enum UpCharBits {
    UpA = 0x77,
    UpC = 0x39,
    UpE = 0x79,
    // can be also done like this (OR'ing segment bits) :)
    UpF = SegmentBits::SegA as u8 | SegmentBits::SegF as u8 | SegmentBits::SegE as u8 | SegmentBits::SegG as u8,
    UpG = 0x3D,
    UpH = 0x76,
    UpI = 0x30,
    UpJ = 0x1E,
    UpL = 0x38,
    UpO = 0x3F,
    UpP = 0x73,
    UpS = 0x6D,
    UpU = 0x3E,
}

/// Maps a character to its closest possible representation on a 7-segment display.
/// The 8th segment is the dot.
#[repr(u8)]
pub enum LoCharBits {
    LoA = 0x5F,
    LoB = 0x7C,
    LoC = 0x58,
    LoD = 0x5E,
    LoH = 0x74,
    LoN = 0x54,
    LoO = 0x5C,
    LoQ = 0x67,
    LoR = 0x50,
    LoT = 0x78,
    LoU = 0x1C,
    LoY = 0x6E,
}

/// Maps a character to its closest possible representation on a 7-segment display.
/// The 8th segment is the dot.
#[repr(u8)]
pub enum SpecialCharBits {
    Space = 0,
    Minus = SegmentBits::SegG as u8,
    Underscore = SegmentBits::SegD as u8,
    Equals = SegmentBits::SegG as u8 | SegmentBits::SegD as u8,
    QuestionMark = SegmentBits::SegA as u8 | SegmentBits::SegB  as u8 | SegmentBits::SegG as u8 | SegmentBits::SegE as u8,
    Dot = SegmentBits::SegPoint as u8,
}
