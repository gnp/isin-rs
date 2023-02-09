#![warn(missing_docs)]
//! # isin::error
//!
//! Error type for ISIN parsing and building.

use std::error::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

/// All the ways parsing or building could fail.
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq)]
pub enum ISINError {
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

impl Debug for ISINError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ISINError::InvalidLength { was } => {
                write!(f, "InvalidLength {{ was: {was:?} }}")
            }
            ISINError::InvalidPayloadLength { was } => {
                write!(f, "InvalidPayloadLength {{ was: {was:?} }}")
            }
            ISINError::InvalidPrefixLength { was } => {
                write!(f, "InvalidPrefixLength {{ was: {was:?} }}")
            }
            ISINError::InvalidBasicCodeLength { was } => {
                write!(f, "InvalidBasicCodeLength {{ was: {was:?} }}")
            }
            ISINError::InvalidPrefix { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidPrefix {{ was: {s:?} }}")
                }
                Err(_) => {
                    write!(f, "InvalidPrefix {{ was: (invalid UTF-8) {was:?} }}")
                }
            },
            ISINError::InvalidBasicCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidBasicCode {{ was: {s:?} }}")
                }
                Err(_) => {
                    write!(f, "InvalidBasicCode {{ was: (invalid UTF-8) {was:?} }}")
                }
            },
            ISINError::InvalidCheckDigit { was } => {
                write!(f, "InvalidCheckDigit {{ was: {:?} }}", char::from(*was))
            }
            ISINError::IncorrectCheckDigit { was, expected } => {
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

impl Display for ISINError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ISINError::InvalidLength { was } => {
                write!(f, "invalid length {was} bytes when expecting 12")
            }
            ISINError::InvalidPayloadLength { was } => {
                write!(f, "invalid Payload length {was} bytes when expecting 11")
            }
            ISINError::InvalidPrefixLength { was } => {
                write!(f, "invalid Prefix length {was} bytes when expecting 2")
            }
            ISINError::InvalidBasicCodeLength { was } => {
                write!(f, "invalid Basic Code length {was} bytes when expecting 9")
            }
            ISINError::InvalidPrefix { was } => match std::str::from_utf8(was) {
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
            ISINError::InvalidBasicCode { was } => match std::str::from_utf8(was) {
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
            ISINError::InvalidCheckDigit { was } => {
                write!(
                    f,
                    "check digit {:?} is not one ASCII decimal digit",
                    *was as char
                )
            }
            ISINError::IncorrectCheckDigit { was, expected } => {
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

impl Error for ISINError {}
