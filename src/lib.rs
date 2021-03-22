#![warn(missing_docs)]
//! # isin
//!
//! `isin` provides an `ISIN` type for working with validated International Securities
//! Identification Numbers (ISINs) as defined in [ISO 6166:2021 Financial services — International securities
//! identification number (ISIN)](https://www.iso.org/standard/78502.html) ("The Standard").
//!
//! [The Association of National Numbering Agencies (ANNA)](https://www.anna-web.org/) has [a page
//! describing ISO 6166](https://www.anna-web.org/standards/isin-iso-6166/).
//!
//! An ISIN is comprised of 12 ASCII characters with the following parts, in order:
//!
//! 1. A two-letter _Prefix_ in uppercase, designating the issuer's country
//! of registration or legal domicile, or for OTC derivatives the special code `EZ`. Additional
//! codes may be allocated by subsequent revisions to The Standard. Country codes follow the
//! [ISO 3166](https://www.iso.org/iso-3166-country-codes.html) standard.
//! 2. A nine-character uppercase alphanumeric _Basic Code_ assigned by the corresponding
//! National Numbering Agency, zero-padded on the left if the underlying code is shorter than nine
//! characters.
//! 3. A single decimal digit representing the _Check Digit_ computed using what the standard calls
//! the "modulus 10 'double-add-double' check digit".
//!
//! Use the `parse()` or `parse_loose()` methods on the ISIN type to convert a string to a validated
//! ISIN.
//!
//! ## Related crates
//!
//! This crate is part of the Financial Identifiers series:
//!
//! * [CUSIP](https://crates.io/crates/cusip)
//! * ISIN
//!

use std::fmt::Formatter;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use bstr::ByteSlice;

pub mod checksum;

use checksum::checksum_table;

pub mod error;
pub use error::ISINError;

/// Type alias for backward compatibility. Do not use in new code.
#[deprecated(since = "0.1.8", note = "please use `ISINError` instead")]
pub type ParseError = ISINError;

/// Compute the _check digit_ for an array of u8. No attempt is made to ensure the input string
/// is in the ISIN payload format or length. If an illegal character (not an ASCII digit and not
/// an ASCII uppercase letter) is encountered, this function will panic.
pub fn compute_check_digit(s: &[u8]) -> u8 {
    let sum = checksum_table(s);
    b'0' + sum
}

fn validate_prefix_format(cc: &[u8]) -> Result<(), ISINError> {
    for b in cc {
        if !(b.is_ascii_alphabetic() && b.is_ascii_uppercase()) {
            let mut cc_copy: [u8; 2] = [0; 2];
            cc_copy.copy_from_slice(cc);
            return Err(ISINError::InvalidPrefix { was: cc_copy });
        }
    }
    Ok(())
}

fn validate_basic_code_format(si: &[u8]) -> Result<(), ISINError> {
    for b in si {
        if !(b.is_ascii_digit() || (b.is_ascii_alphabetic() && b.is_ascii_uppercase())) {
            let mut si_copy: [u8; 9] = [0; 9];
            si_copy.copy_from_slice(si);
            return Err(ISINError::InvalidBasicCode { was: si_copy });
        }
    }
    Ok(())
}

fn validate_check_digit_format(cd: u8) -> Result<(), ISINError> {
    if !cd.is_ascii_digit() {
        Err(ISINError::InvalidCheckDigit { was: cd })
    } else {
        Ok(())
    }
}

/// Parse a string to a valid ISIN or an error message, requiring the string to already be only
/// uppercase alphanumerics with no leading or trailing whitespace in addition to being the
/// right length and format.
pub fn parse(value: &str) -> Result<ISIN, ISINError> {
    let v: String = value.into();

    if v.len() != 12 {
        return Err(ISINError::InvalidLength { was: v.len() });
    }

    // We make the preliminary assumption that the string is pure ASCII, so we work with the
    // underlying bytes. If there is Unicode in the string, the bytes will be outside the
    // allowed range and format validations will fail.

    let b = v.as_bytes();

    // We slice out the three fields and validate their formats.

    let cc: &[u8] = &b[0..2];
    validate_prefix_format(cc)?;

    let si: &[u8] = &b[2..11];
    validate_basic_code_format(si)?;

    let cd = b[11];
    validate_check_digit_format(cd)?;

    // Now, we need to compute the correct check digit value from the "payload" (everything except
    // the check digit).

    let payload = &b[0..11];

    let computed_check_digit = compute_check_digit(payload);

    let incorrect_check_digit = cd != computed_check_digit;
    if incorrect_check_digit {
        return Err(ISINError::IncorrectCheckDigit {
            was: cd,
            expected: computed_check_digit,
        });
    }

    let mut bb = [0u8; 12];
    bb.copy_from_slice(b);

    Ok(ISIN(bb))
}

/// Forwards to `parse()` for backward compatibility. Do not use in new code.
#[deprecated(since = "0.1.7", note = "please use `isin::parse` instead")]
pub fn parse_strict(value: &str) -> Result<ISIN, ISINError> {
    parse(value)
}

/// Parse a string to a valid ISIN or an error, allowing the string to contain leading
/// or trailing whitespace and/or lowercase letters as long as it is otherwise the right length
/// and format.
pub fn parse_loose(value: &str) -> Result<ISIN, ISINError> {
    let uc = value.to_ascii_uppercase();
    let temp = uc.trim();
    parse(temp)
}

/// An ISIN in confirmed valid format.
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
    type Err = ISINError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_loose(s)
    }
}

impl ISIN {
    /// Forwards to crate-level `parse()` for backward compatibility. Do not use in new code.
    #[deprecated(since = "0.1.7", note = "please use `isin::parse` instead")]
    pub fn parse_strict<S>(value: S) -> Result<ISIN, ISINError>
    where
        S: Into<String>,
    {
        let v: String = value.into();
        crate::parse(&v)
    }

    /// Forwards to crate-level `parse_loose()` for backward compatibility. Do not use in new code.
    #[deprecated(since = "0.1.7", note = "please use `isin::parse_loose` instead")]
    pub fn parse_loose<S>(value: S) -> Result<ISIN, ISINError>
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

    /// Return just the _Prefix_ portion of the ISIN.
    pub fn prefix(&self) -> &str {
        unsafe { self.0[0..2].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return just the _Prefix_ portion of the ISIN.
    #[deprecated(since = "0.1.8", note = "please use `prefix` instead")]
    pub fn country_code(&self) -> &str {
        self.prefix()
    }

    /// Return just the _Basic Code_ portion of the ISIN.
    pub fn basic_code(&self) -> &str {
        unsafe { self.0[2..11].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return just the _Basic Code_ portion of the ISIN.
    #[deprecated(since = "0.1.8", note = "please use `basic_code` instead")]
    pub fn security_identifier(&self) -> &str {
        self.basic_code()
    }

    /// Return the _Payload_ &mdash; everything except the _Check Digit_.
    pub fn payload(&self) -> &str {
        unsafe { self.0[0..11].to_str_unchecked() } // This is safe because we know it is ASCII
    }

    /// Return just the _Check Digit_ portion of the ISIN.
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
        match parse("US0378331005") {
            Ok(isin) => {
                assert_eq!(isin.to_string(), "US0378331005");
                assert_eq!(isin.prefix(), "US");
                assert_eq!(isin.basic_code(), "037833100");
                assert_eq!(isin.check_digit(), '5');
            }
            Err(err) => assert!(false, "Did not expect parsing to fail: {}", err),
        }
    }

    #[test]
    fn parse_isin_for_apple_loose() {
        match parse_loose("\tus0378331005    ") {
            Ok(isin) => {
                assert_eq!(isin.to_string(), "US0378331005");
                assert_eq!(isin.prefix(), "US");
                assert_eq!(isin.basic_code(), "037833100");
                assert_eq!(isin.check_digit(), '5');
            }
            Err(err) => assert!(false, "Did not expect parsing to fail: {}", err),
        }
    }

    #[test]
    fn reject_empty_string() {
        let res = parse("");
        assert!(res.is_err());
    }

    #[test]
    fn reject_lowercase_prefix_if_strict() {
        match parse("us0378331005") {
            Err(ISINError::InvalidPrefix { was: _ }) => {} // Ok
            Err(err) => {
                assert!(
                    false,
                    "Expected Err(InvalidPrefix {{ ... }}), but got: Err({:?})",
                    err
                )
            }
            Ok(isin) => {
                assert!(
                    false,
                    "Expected Err(InvalidPrefix {{ ... }}), but got: Ok({:?})",
                    isin
                )
            }
        }
    }

    #[test]
    fn reject_lowercase_basic_code_if_strict() {
        match parse("US09739d1000") {
            Err(ISINError::InvalidBasicCode { was: _ }) => {} // Ok
            Err(err) => {
                assert!(
                    false,
                    "Expected Err(InvalidBasicCode {{ ... }}), but got: Err({:?})",
                    err
                )
            }
            Ok(isin) => {
                assert!(
                    false,
                    "Expected Err(InvalidBasicCode {{ ... }}), but got: Ok({:?})",
                    isin
                )
            }
        }
    }

    #[test]
    fn parse_isin_with_0_check_digit() {
        parse("US09739D1000").unwrap(); // BCC aka Boise Cascade
    }

    #[test]
    fn parse_isin_with_1_check_digit() {
        parse("US4581401001").unwrap(); // INTC aka Intel
    }

    #[test]
    fn parse_isin_with_2_check_digit() {
        parse("US98421M1062").unwrap(); // XRX aka Xerox
    }

    #[test]
    fn parse_isin_with_3_check_digit() {
        parse("US02376R1023").unwrap(); // AAL aka American Airlines
    }

    #[test]
    fn parse_isin_with_4_check_digit() {
        parse("US9216591084").unwrap(); // VNDA aka Vanda Pharmaceuticals
    }

    #[test]
    fn parse_isin_with_5_check_digit() {
        parse("US0207721095").unwrap(); // APT aka AlphaProTec
    }

    #[test]
    fn parse_isin_with_6_check_digit() {
        parse("US71363P1066").unwrap(); // PRDO aka Perdoceo Education
    }

    #[test]
    fn parse_isin_with_7_check_digit() {
        parse("US5915202007").unwrap(); // MEI aka Methode Electronics
    }

    #[test]
    fn parse_isin_with_8_check_digit() {
        parse("US4570301048").unwrap(); // IMKTA aka Ingles Markets
    }

    #[test]
    fn parse_isin_with_9_check_digit() {
        parse("US8684591089").unwrap(); // SUPN aka Supernus Pharmaceuticals
    }

    #[test]
    fn test_unicode_gibberish() {
        assert_eq!(true, parse("𑴈𐎟 0 A").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            parse(&s);
        }
    }
}
