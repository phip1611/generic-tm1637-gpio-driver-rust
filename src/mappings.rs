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

/// Maps the segment from the device to its bit.
#[repr(u8)]
#[derive(Debug)]
pub enum SegmentBits {
    /// A segment
    SegA = 0b00000001,
    /// B segment
    SegB = 0b00000010,
    /// C segment
    SegC = 0b00000100,
    /// D segment
    SegD = 0b00001000,
    /// E segment
    SegE = 0b00010000,
    /// F segment
    SegF = 0b00100000,
    /// G segment
    SegG = 0b01000000,
    /// Double point on AzDelivery 4-digit 7 segment display.
    SegPoint = 0b10000000,
}

/// Maps a digit to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug)]
pub enum NumCharBits {
    /// 0
    Zero = 0b00111111,
    /// 1
    One = 0b00000110,
    /// 2
    Two = 0b01011011,
    /// 3
    Three = 0b01001111,
    /// 4
    Four = 0b01100110,
    /// 5
    Five = 0b01101101,
    /// 6
    Six = 0b01111101,
    /// 7
    Seven = 0b00000111,
    /// 8
    Eight = 0b01111111,
    /// 9
    Nine = 0b01101111,
}

/// Maps a character to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug)]
pub enum UpCharBits {
    /// Uppercase A
    UpA = 0x77,
    /// Uppercase C
    UpC = 0x39,
    /// Uppercase E
    UpE = 0x79,
    // can be also done like this (OR'ing segment bits) :
    /// Uppercase F
    UpF = SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegG as u8,
    /// Uppercase G
    UpG = 0x3D,
    /// Uppercase H
    UpH = 0x76,
    /// Uppercase I
    UpI = 0x30,
    /// Uppercase J
    UpJ = 0x1E,
    /// Uppercase L
    UpL = 0x38,
    /// Uppercase O
    UpO = 0x3F,
    /// Uppercase P
    UpP = 0x73,
    /// Uppercase S
    UpS = 0x6D,
    /// Uppercase U
    UpU = 0x3E,
}

/// Maps a character to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug)]
pub enum LoCharBits {
    /// Lowercase A
    LoA = 0x5F,
    /// Lowercase B
    LoB = 0x7C,
    /// Lowercase C
    LoC = 0x58,
    /// Lowercase D
    LoD = 0x5E,
    /// Lowercase H
    LoH = 0x74,
    /// Lowercase N
    LoN = 0x54,
    /// Lowercase O
    LoO = 0x5C,
    /// Lowercase Q
    LoQ = 0x67,
    /// Lowercase R
    LoR = 0x50,
    /// Lowercase T
    LoT = 0x78,
    /// Lowercase U
    LoU = 0x1C,
    /// Lowercase Y
    LoY = 0x6E,
}

/// Maps a character to its closest possible representation on a 7-segment display.
/// The 8th segment is the dot.
#[repr(u8)]
#[derive(Debug)]
pub enum SpecialCharBits {
    /// Space symbol.
    Space = 0,
    /// Minus or dash symbol.
    Minus = SegmentBits::SegG as u8,
    /// Underscore (_).
    Underscore = SegmentBits::SegD as u8,
    /// Equal sign (=).
    Equals = SegmentBits::SegG as u8 | SegmentBits::SegD as u8,
    /// Question mark (?).
    QuestionMark = SegmentBits::SegA as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegG as u8
        | SegmentBits::SegE as u8,
    /// Dot (.).
    Dot = SegmentBits::SegPoint as u8,
}
