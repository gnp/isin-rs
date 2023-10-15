#![warn(missing_docs)]
//! # isin::error
//!
//! Error type for ISIN parsing and building.

use std::fmt::Formatter;
use std::fmt::{Debug, Display};

/// All the ways parsing or building could fail.
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    /// The input length is not exactly 12 bytes.
    InvalidLength {
        /// The length we found
        was: usize,
    },
    /// The _Payload_ length is not exactly 11 bytes (checked when building).
    InvalidPayloadLength {
        /// The length we found
        was: usize,
    },
    /// The _Prefix_ length is not exactly 2 bytes (checked when building).
    InvalidPrefixLength {
        /// The length we found
        was: usize,
    },
    /// The _Basic Code_ length is not exactly 9 bytes (checked when building).
    InvalidBasicCodeLength {
        /// The length we found
        was: usize,
    },
    /// The input _Prefix_ is not two uppercase ASCII alphabetic characters.
    InvalidPrefix {
        /// The _Prefix_ we found
        was: [u8; 2],
    },
    /// The input _Basic Code_ is not nine uppercase ASCII alphanumeric characters.
    InvalidBasicCode {
        /// The _Basic Code_ we found
        was: [u8; 9],
    },
    /// The input _Check Digit_ is not a single ASCII decimal digit character.
    InvalidCheckDigit {
        /// The _Check Digit_ we found
        was: u8,
    },
    /// The input _Check Digit_ is in a valid format, but has an incorrect value.
    IncorrectCheckDigit {
        /// The _Check Digit_ we found
        was: u8,
        /// The _Check Digit_ we expected
        expected: u8,
    },
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLength { was } => {
                write!(f, "InvalidLength {{ was: {was:?} }}")
            }
            Error::InvalidPayloadLength { was } => {
                write!(f, "InvalidPayloadLength {{ was: {was:?} }}")
            }
            Error::InvalidPrefixLength { was } => {
                write!(f, "InvalidPrefixLength {{ was: {was:?} }}")
            }
            Error::InvalidBasicCodeLength { was } => {
                write!(f, "InvalidBasicCodeLength {{ was: {was:?} }}")
            }
            Error::InvalidPrefix { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidPrefix {{ was: {s:?} }}")
                }
                Err(_) => {
                    write!(f, "InvalidPrefix {{ was: (invalid UTF-8) {was:?} }}")
                }
            },
            Error::InvalidBasicCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidBasicCode {{ was: {s:?} }}")
                }
                Err(_) => {
                    write!(f, "InvalidBasicCode {{ was: (invalid UTF-8) {was:?} }}")
                }
            },
            Error::InvalidCheckDigit { was } => {
                write!(f, "InvalidCheckDigit {{ was: {:?} }}", char::from(*was))
            }
            Error::IncorrectCheckDigit { was, expected } => {
                write!(
                    f,
                    "IncorrectCheckDigit {{ was: {:?}, expected: {:?} }}",
                    char::from(*was),
                    char::from(*expected)
                )
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLength { was } => {
                write!(f, "invalid length {was} bytes when expecting 12")
            }
            Error::InvalidPayloadLength { was } => {
                write!(f, "invalid Payload length {was} bytes when expecting 11")
            }
            Error::InvalidPrefixLength { was } => {
                write!(f, "invalid Prefix length {was} bytes when expecting 2")
            }
            Error::InvalidBasicCodeLength { was } => {
                write!(f, "invalid Basic Code length {was} bytes when expecting 9")
            }
            Error::InvalidPrefix { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(
                        f,
                        "prefix {s:?} is not two uppercase ASCII alphabetic characters"
                    )
                }
                Err(_) => {
                    write!(f,
                    "prefix (invalid UTF-8) {was:?} is not two uppercase ASCII alphabetic characters"
                    )
                }
            },
            Error::InvalidBasicCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(
                        f,
                        "basic code {s:?} is not nine uppercase ASCII alphanumeric characters"
                    )
                }
                Err(_) => {
                    write!(f,
                "basic code (invalid UTF-8) {was:?} is not nine uppercase ASCII alphanumeric characters"
                    )
                }
            },
            Error::InvalidCheckDigit { was } => {
                write!(
                    f,
                    "check digit {:?} is not one ASCII decimal digit",
                    *was as char
                )
            }
            Error::IncorrectCheckDigit { was, expected } => {
                write!(
                    f,
                    "incorrect check digit {:?} when expecting {:?}",
                    char::from(*was),
                    char::from(*expected)
                )
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn render_display() {
        let errors = [
            (
                Error::InvalidLength { was: 10 },
                "invalid length 10 bytes when expecting 12",
            ),
            (
                Error::InvalidPayloadLength { was: 8 },
                "invalid Payload length 8 bytes when expecting 11",
            ),
            (
                Error::InvalidPrefixLength { was: 1 },
                "invalid Prefix length 1 bytes when expecting 2",
            ),
            (
                Error::InvalidBasicCodeLength { was: 8 },
                "invalid Basic Code length 8 bytes when expecting 9",
            ),
            (
                Error::InvalidPrefix { was: *b"A{" },
                "prefix \"A{\" is not two uppercase ASCII alphabetic characters",
            ),
            (
                Error::InvalidBasicCode { was: *b"ABCDEFGH{" },
                "basic code \"ABCDEFGH{\" is not nine uppercase ASCII alphanumeric characters",
            ),
            (
                Error::InvalidCheckDigit { was: b':' },
                "check digit ':' is not one ASCII decimal digit",
            ),
            (
                Error::IncorrectCheckDigit {
                    was: b'5',
                    expected: b'6',
                },
                "incorrect check digit '5' when expecting '6'",
            ),
        ];

        for (error, expected) in errors.iter() {
            assert_eq!(format!("{}", error), *expected);
        }
    }

    #[test]
    fn render_debug() {
        let errors = [
            (
                Error::InvalidLength { was: 10 },
                "InvalidLength { was: 10 }",
            ),
            (
                Error::InvalidPayloadLength { was: 8 },
                "InvalidPayloadLength { was: 8 }",
            ),
            (
                Error::InvalidPrefixLength { was: 1 },
                "InvalidPrefixLength { was: 1 }",
            ),
            (
                Error::InvalidBasicCodeLength { was: 8 },
                "InvalidBasicCodeLength { was: 8 }",
            ),
            (
                Error::InvalidPrefix { was: *b"A{" },
                "InvalidPrefix { was: \"A{\" }",
            ),
            (
                Error::InvalidBasicCode { was: *b"ABCDEFGH{" },
                "InvalidBasicCode { was: \"ABCDEFGH{\" }",
            ),
            (
                Error::InvalidCheckDigit { was: b':' },
                "InvalidCheckDigit { was: ':' }",
            ),
            (
                Error::IncorrectCheckDigit {
                    was: b'5',
                    expected: b'6',
                },
                "IncorrectCheckDigit { was: '5', expected: '6' }",
            ),
        ];

        for (error, expected) in errors.iter() {
            assert_eq!(format!("{:?}", error), *expected);
        }
    }
}
