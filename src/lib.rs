//! # isin
//!
//! `isin` provides an `ISIN` type for working with validated International Security Identifiers
//! (ISINs) as defined in [ISO 6166](https://www.iso.org/standard/78502.html).
//!
//! [The Association of National Numbering Agencies (ANNA)](https://www.anna-web.org/) has [a page
//! describing ISO 6166](https://www.anna-web.org/standards/isin-iso-6166/).
//!
//! An ISIN is comprised of 12 ASCII characters with the following parts, in order:
//!
//! 1. A two-letter [ISO 3166]() country code in upper-case, designating the issuer's country
//! of registration or legal domicile, or for OTC derivatives the special code `EZ`. Additional
//! codes may be allocated by subsequent revisions to the standard.
//! 2. A nine-character upper-case alphanumeric _Security Identifier_ assigned by the corresponding
//! National Numbering Agency, zero-padded on the left if the underlying code is shorter than nine
//! characters.
//! 3. A single decimal digit representing the _check digit_ computed using what the standard calls
//! the "modulus 10 'double-add-double' check digit".
//!
//! Use the `parse_loose()` or `parse_strict()` methods to convert a string to a
//! validated ISIN.

use std::error::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use bstr::ByteSlice;

pub mod checksum;

use checksum::checksum_table;

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input length is not exactly 12 bytes.
    InvalidLength { was: usize },
    /// The input country code is not two uppercase ASCII alphabetic characters.
    InvalidCountryCode { was: [u8; 2] },
    /// The input security identifier is not nine uppercase ASCII alphanumeric characters.
    InvalidSecurityIdentifier { was: [u8; 9] },
    /// The input check digit is not a single ASCII decimal digit character.
    InvalidCheckDigit { was: u8 },
    /// The input check digit has in a valid format, but has an incorrect value.
    IncorrectCheckDigit { was: u8, expected: u8 },
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLength { was } => {
                write!(f, "InvalidLength {{ was: {:?} }}", was)
            }
            ParseError::InvalidCountryCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidCountryCode {{ was: {:?} }}", s)
                }
                Err(_) => {
                    write!(f, "InvalidCountryCode {{ was: (invalid UTF-8) {:?} }}", was)
                }
            },
            ParseError::InvalidSecurityIdentifier { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f, "InvalidSecurityIdentifier {{ was: {:?} }}", s)
                }
                Err(_) => {
                    write!(
                        f,
                        "InvalidSecurityIdentifier {{ was: (invalid UTF-8) {:?} }}",
                        was
                    )
                }
            },
            ParseError::InvalidCheckDigit { was } => {
                write!(f, "InvalidCheckDigit {{ was: {:?} }}", char::from(*was))
            }
            ParseError::IncorrectCheckDigit { was, expected } => {
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLength { was } => {
                write!(f, "invalid length {} bytes when expecting 12", was)
            }
            ParseError::InvalidCountryCode { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f,
                    "invalid country code {:?} is not two uppercase ASCII alphabetic characters",
                     s)
                }
                Err(_) => {
                    write!(f,
                    "invalid country code (invalid UTF-8) {:?} is not two uppercase ASCII alphabetic characters",
                    was)
                }
            },
            ParseError::InvalidSecurityIdentifier { was } => match std::str::from_utf8(was) {
                Ok(s) => {
                    write!(f,
                "invalid security identifier {:?} is not nine uppercase ASCII alphanumeric characters",
                     s)
                }
                Err(_) => {
                    write!(f,
                "invalid security identifier (invalid UTF-8) {:?} is not nine uppercase ASCII alphanumeric characters",
                    was)
                }
            },
            ParseError::InvalidCheckDigit { was } => {
                write!(
                    f,
                    "invalid check digit {:?} is not one ASCII decimal digit",
                    *was as char
                )
            }
            ParseError::IncorrectCheckDigit { was, expected } => {
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

impl Error for ParseError {}

/// Compute the _check digit_ for an array of u8. No attempt is made to ensure the input string
/// is in the ISIN payload format or length. If an illegal character (not an ASCII digit and not
/// an ASCII uppercase letter) is encountered, this function will panic.
pub fn compute_check_digit(s: &[u8]) -> u8 {
    let sum = checksum_table(s);
    b'0' + sum
}

fn validate_country_code_format(cc: &[u8]) -> Result<(), ParseError> {
    for b in cc {
        if !(b.is_ascii_alphabetic() && b.is_ascii_uppercase()) {
            let mut cc_copy: [u8; 2] = [0; 2];
            cc_copy.copy_from_slice(cc);
            return Err(ParseError::InvalidCountryCode { was: cc_copy });
        }
    }
    Ok(())
}

fn validate_security_id_format(si: &[u8]) -> Result<(), ParseError> {
    for b in si {
        if !(b.is_ascii_digit() || (b.is_ascii_alphabetic() && b.is_ascii_uppercase())) {
            let mut si_copy: [u8; 9] = [0; 9];
            si_copy.copy_from_slice(si);
            return Err(ParseError::InvalidSecurityIdentifier { was: si_copy });
        }
    }
    Ok(())
}

fn validate_check_digit_format(cd: u8) -> Result<(), ParseError> {
    if !cd.is_ascii_digit() {
        Err(ParseError::InvalidCheckDigit { was: cd })
    } else {
        Ok(())
    }
}

/// Parse a string to a valid ISIN or an error message, requiring the string to already be only
/// uppercase alphanumerics with no leading or trailing whitespace in addition to being the
/// right length and format.
pub fn parse_strict(value: &str) -> Result<ISIN, ParseError> {
    let v: String = value.into();

    if v.len() != 12 {
        return Err(ParseError::InvalidLength { was: v.len() });
    }

    // We make the preliminary assumption that the string is pure ASCII, so we work with the
    // underlying bytes. If there is Unicode in the string, the bytes will be outside the
    // allowed range and format validations will fail.

    let b = v.as_bytes();

    // We slice out the three fields and validate their formats.

    let cc: &[u8] = &b[0..2];
    validate_country_code_format(cc)?;

    let si: &[u8] = &b[2..11];
    validate_security_id_format(si)?;

    let cd = b[11];
    validate_check_digit_format(cd)?;

    // Now, we need to compute the correct check digit value from the "payload" (the country
    // code and security identifier fields).

    let payload = &b[0..11];

    let computed_check_digit = compute_check_digit(payload);

    let incorrect_check_digit = cd != computed_check_digit;
    if incorrect_check_digit {
        return Err(ParseError::IncorrectCheckDigit {
            was: cd,
            expected: computed_check_digit,
        });
    }

    let mut b = [0u8; 12];
    b.copy_from_slice(v.as_bytes());

    Ok(ISIN(b))
}

/// Parse a string to a valid ISIN or an error message, allowing the string to contain leading
/// or trailing whitespace and/or lowercase letters as long as it is otherwise the right length
/// and format.
pub fn parse_loose(value: &str) -> Result<ISIN, ParseError> {
    let uc = value.to_ascii_uppercase();
    let temp = uc.trim();
    parse_strict(temp)
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
#[repr(transparent)]
pub struct ISIN([u8; 12]);

impl Display for ISIN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let temp = unsafe { self.0[..].to_str_unchecked() }; // This is safe because we know it is ASCII
        write!(f, "{}", temp)
    }
}

impl FromStr for ISIN {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_loose(s)
    }
}

impl ISIN {
    #[deprecated(since = "0.1.7", note = "please use `isin::parse_strict` instead")]
    pub fn parse_strict<S>(value: S) -> Result<ISIN, ParseError>
    where
        S: Into<String>,
    {
        let v: String = value.into();
        crate::parse_strict(&v)
    }

    #[deprecated(since = "0.1.7", note = "please use `isin::parse_loose` instead")]
    pub fn parse_loose<S>(value: S) -> Result<ISIN, ParseError>
    where
        S: Into<String>,
    {
        let v: String = value.into();
        crate::parse_loose(&v)
    }

    /// Return a string representation of the ISIN.
    #[deprecated(since = "0.1.7", note = "please use `to_string` instead")]
    pub fn value(&self) -> &str {
        unsafe { self.0[..].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return just the _country code_ portion of the ISIN.
    pub fn country_code(&self) -> &str {
        unsafe { self.0[0..2].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return just the _security identifier_ portion of the ISIN.
    pub fn security_identifier(&self) -> &str {
        unsafe { self.0[2..11].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return the &ldquo;payload&rdquo; &mdash; everything but the check digit.
    pub fn payload(&self) -> &str {
        unsafe { self.0[0..11].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return just the _check digit_ portion of the ISIN.
    pub fn check_digit(&self) -> char {
        self.0[11] as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parse_isin_for_apple_strict() {
        match parse_strict("US0378331005") {
            Ok(isin) => {
                assert_eq!(isin.to_string(), "US0378331005");
                assert_eq!(isin.country_code(), "US");
                assert_eq!(isin.security_identifier(), "037833100");
                assert_eq!(isin.check_digit(), '5');
            }
            Err(_) => assert!(false, "Did not expect parsing to fail"),
        }
    }

    #[test]
    fn parse_isin_for_apple_loose() {
        match parse_loose("\tus0378331005    ") {
            Ok(isin) => {
                assert_eq!(isin.to_string(), "US0378331005");
                assert_eq!(isin.country_code(), "US");
                assert_eq!(isin.security_identifier(), "037833100");
                assert_eq!(isin.check_digit(), '5');
            }
            Err(_) => assert!(false, "Did not expect parsing to fail"),
        }
    }

    #[test]
    fn reject_empty_string() {
        let res = parse_strict("");
        assert!(res.is_err());
    }

    #[test]
    fn reject_lowercase_country_code_if_strict() {
        let res = parse_strict("us0378331005");
        assert!(res.is_err());
    }

    #[test]
    fn reject_lowercase_security_id_if_strict() {
        let res = parse_strict("US09739d1000");
        assert!(res.is_err());
    }

    #[test]
    fn parse_isin_with_0_check_digit() {
        parse_strict("US09739D1000").unwrap(); // BCC aka Boise Cascade
    }

    #[test]
    fn parse_isin_with_1_check_digit() {
        parse_strict("US4581401001").unwrap(); // INTC aka Intel
    }

    #[test]
    fn parse_isin_with_2_check_digit() {
        parse_strict("US98421M1062").unwrap(); // XRX aka Xerox
    }

    #[test]
    fn parse_isin_with_3_check_digit() {
        parse_strict("US02376R1023").unwrap(); // AAL aka American Airlines
    }

    #[test]
    fn parse_isin_with_4_check_digit() {
        parse_strict("US9216591084").unwrap(); // VNDA aka Vanda Pharmaceuticals
    }

    #[test]
    fn parse_isin_with_5_check_digit() {
        parse_strict("US0207721095").unwrap(); // APT aka AlphaProTec
    }

    #[test]
    fn parse_isin_with_6_check_digit() {
        parse_strict("US71363P1066").unwrap(); // PRDO aka Perdoceo Education
    }

    #[test]
    fn parse_isin_with_7_check_digit() {
        parse_strict("US5915202007").unwrap(); // MEI aka Methode Electronics
    }

    #[test]
    fn parse_isin_with_8_check_digit() {
        parse_strict("US4570301048").unwrap(); // IMKTA aka Ingles Markets
    }

    #[test]
    fn parse_isin_with_9_check_digit() {
        parse_strict("US8684591089").unwrap(); // SUPN aka Supernus Pharmaceuticals
    }

    #[test]
    fn test_unicode_gibberish() {
        assert_eq!(true, parse_strict("𑴈𐎟 0 A").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            parse_strict(&s);
        }
    }
}
