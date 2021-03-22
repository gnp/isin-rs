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
    /// The input country code is not two uppercase ASCII alphabetic characters.
    InvalidCountryCode {
        /// The _Country Code_ we found
        was: [u8; 2],
    },
    /// The input security id is not nine uppercase ASCII alphanumeric characters.
    InvalidSecurityId {
        /// The _Security Identifier_ we found
        was: [u8; 9],
    },
    /// The input check digit is not a single ASCII decimal digit character.
    InvalidCheckDigit {
        /// The _Check Digit_ we found
        was: u8,
    },
    /// The input check digit has in a valid format, but has an incorrect value.
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
                write!(f, "InvalidLength {{ was: {:?} }}", was)
            }
            ISINError::InvalidCountryCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidCountryCode {{ was: {:?} }}", s)
                }
                Err(_) => {
                    write!(f, "InvalidCountryCode {{ was: (invalid UTF-8) {:?} }}", was)
                }
            },
            ISINError::InvalidSecurityId { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidSecurityId {{ was: {:?} }}", s)
                }
                Err(_) => {
                    write!(f, "InvalidSecurityId {{ was: (invalid UTF-8) {:?} }}", was)
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
                write!(f, "invalid length {} bytes when expecting 12", was)
            }
            ISINError::InvalidCountryCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(
                        f,
                        "country code {:?} is not two uppercase ASCII alphabetic characters",
                        s
                    )
                }
                Err(_) => {
                    write!(f,
                    "country code (invalid UTF-8) {:?} is not two uppercase ASCII alphabetic characters",
                    was)
                }
            },
            ISINError::InvalidSecurityId { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(
                        f,
                        "security id {:?} is not nine uppercase ASCII alphanumeric characters",
                        s
                    )
                }
                Err(_) => {
                    write!(f,
                "security id (invalid UTF-8) {:?} is not nine uppercase ASCII alphanumeric characters",
                    was)
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
