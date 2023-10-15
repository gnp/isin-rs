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
    /// The value string length is not exactly 12 characters.
    InvalidValueStringLength {
        /// The length we found
        was: usize,
    },
    /// The value byte array length is not exactly 12 bytes.
    InvalidValueArrayLength {
        /// The length we found
        was: usize,
    },
    /// The _Payload_ string length is not exactly 11 characters.
    InvalidPayloadStringLength {
        /// The length we found
        was: usize,
    },
    /// The _Payload_ byte array length is not exactly 11 bytes.
    InvalidPayloadArrayLength {
        /// The length we found
        was: usize,
    },
    /// The _Prefix_ string length is not exactly 2 characters.
    InvalidPrefixStringLength {
        /// The length we found
        was: usize,
    },
    /// The _Prefix_ byte array length is not exactly 2 bytes.
    InvalidPrefixArrayLength {
        /// The length we found
        was: usize,
    },
    /// The _Basic Code_ string length is not exactly 9 characters.
    InvalidBasicCodeStringLength {
        /// The length we found
        was: usize,
    },
    /// The _Basic Code_ byte array length is not exactly 9 bytes.
    InvalidBasicCodeArrayLength {
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
            Error::InvalidValueStringLength { was } => {
                write!(f, "InvalidValueStringLength {{ was: {was:?} }}")
            }
            Error::InvalidValueArrayLength { was } => {
                write!(f, "InvalidValueArrayLength {{ was: {was:?} }}")
            }
            Error::InvalidPayloadStringLength { was } => {
                write!(f, "InvalidPayloadStringLength {{ was: {was:?} }}")
            }
            Error::InvalidPayloadArrayLength { was } => {
                write!(f, "InvalidPayloadArrayLength {{ was: {was:?} }}")
            }
            Error::InvalidPrefixStringLength { was } => {
                write!(f, "InvalidPrefixStringLength {{ was: {was:?} }}")
            }
            Error::InvalidPrefixArrayLength { was } => {
                write!(f, "InvalidPrefixArrayLength {{ was: {was:?} }}")
            }
            Error::InvalidBasicCodeStringLength { was } => {
                write!(f, "InvalidBasicCodeStringLength {{ was: {was:?} }}")
            }
            Error::InvalidBasicCodeArrayLength { was } => {
                write!(f, "InvalidBasicCodeArrayLength {{ was: {was:?} }}")
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
            Error::InvalidValueStringLength { was } => {
                write!(
                    f,
                    "invalid value string length {was} characters when expecting 12"
                )
            }
            Error::InvalidValueArrayLength { was } => {
                write!(
                    f,
                    "invalid value array length {was} bytes when expecting 12"
                )
            }
            Error::InvalidPayloadStringLength { was } => {
                write!(
                    f,
                    "invalid Payload string length {was} characters when expecting 11"
                )
            }
            Error::InvalidPayloadArrayLength { was } => {
                write!(
                    f,
                    "invalid Payload array length {was} bytes when expecting 11"
                )
            }
            Error::InvalidPrefixArrayLength { was } => {
                write!(
                    f,
                    "invalid Prefix array length {was} bytes when expecting 2"
                )
            }
            Error::InvalidPrefixStringLength { was } => {
                write!(
                    f,
                    "invalid Prefix string length {was} characters when expecting 2"
                )
            }
            Error::InvalidBasicCodeStringLength { was } => {
                write!(
                    f,
                    "invalid Basic Code string length {was} characters when expecting 9"
                )
            }
            Error::InvalidBasicCodeArrayLength { was } => {
                write!(
                    f,
                    "invalid Basic Code array length {was} bytes when expecting 9"
                )
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
                Error::InvalidValueStringLength { was: 10 },
                "invalid value string length 10 characters when expecting 12",
            ),
            (
                Error::InvalidValueArrayLength { was: 10 },
                "invalid value array length 10 bytes when expecting 12",
            ),
            (
                Error::InvalidPayloadStringLength { was: 8 },
                "invalid Payload string length 8 characters when expecting 11",
            ),
            (
                Error::InvalidPayloadArrayLength { was: 8 },
                "invalid Payload array length 8 bytes when expecting 11",
            ),
            (
                Error::InvalidPrefixStringLength { was: 1 },
                "invalid Prefix string length 1 characters when expecting 2",
            ),
            (
                Error::InvalidPrefixArrayLength { was: 1 },
                "invalid Prefix array length 1 bytes when expecting 2",
            ),
            (
                Error::InvalidBasicCodeStringLength { was: 8 },
                "invalid Basic Code string length 8 characters when expecting 9",
            ),
            (
                Error::InvalidBasicCodeArrayLength { was: 8 },
                "invalid Basic Code array length 8 bytes when expecting 9",
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
                Error::InvalidValueStringLength { was: 10 },
                "InvalidValueStringLength { was: 10 }",
            ),
            (
                Error::InvalidValueArrayLength { was: 10 },
                "InvalidValueArrayLength { was: 10 }",
            ),
            (
                Error::InvalidPayloadStringLength { was: 8 },
                "InvalidPayloadStringLength { was: 8 }",
            ),
            (
                Error::InvalidPayloadArrayLength { was: 8 },
                "InvalidPayloadArrayLength { was: 8 }",
            ),
            (
                Error::InvalidPrefixArrayLength { was: 1 },
                "InvalidPrefixArrayLength { was: 1 }",
            ),
            (
                Error::InvalidBasicCodeArrayLength { was: 8 },
                "InvalidBasicCodeArrayLength { was: 8 }",
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
