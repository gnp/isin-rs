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
//! * [CUSIP](https://crates.io/crates/cusip): Committee on Uniform Security Identification Procedures (ANSI X9.6-2020)
//! * [ISIN](https://crates.io/crates/isin): International Securities Identification Number (ISO 6166:2021)
//! * [LEI](https://crates.io/crates/lei): Legal Entity Identifier (ISO 17442:2020)
//!

use std::fmt;
use std::str::from_utf8_unchecked;
use std::str::FromStr;

pub mod checksum;

use checksum::checksum_table;

pub mod error;
pub use error::Error;

/// Compute the _Check Digit_ for an array of u8. No attempt is made to ensure the input string
/// is in the ISIN payload format or length. If an illegal character (not an ASCII digit and not
/// an ASCII uppercase letter) is encountered, this function will panic.
fn compute_check_digit(s: &[u8]) -> u8 {
    let sum = checksum_table(s);
    b'0' + sum
}

fn validate_prefix_format(prefix: &[u8]) -> Result<&[u8], Error> {
    if prefix.len() != 2 {
        return Err(Error::InvalidPrefixArrayLength { was: prefix.len() });
    }
    for b in prefix {
        if !(b.is_ascii_alphabetic() && b.is_ascii_uppercase()) {
            let mut prefix_copy: [u8; 2] = [0; 2];
            prefix_copy.copy_from_slice(prefix);
            return Err(Error::InvalidPrefix { was: prefix_copy });
        }
    }
    Ok(prefix)
}

fn validate_basic_code_format(basic_code: &[u8]) -> Result<&[u8], Error> {
    if basic_code.len() != 9 {
        return Err(Error::InvalidBasicCodeArrayLength {
            was: basic_code.len(),
        });
    }
    for b in basic_code {
        if !(b.is_ascii_digit() || (b.is_ascii_alphabetic() && b.is_ascii_uppercase())) {
            let mut basic_code_copy: [u8; 9] = [0; 9];
            basic_code_copy.copy_from_slice(basic_code);
            return Err(Error::InvalidBasicCode {
                was: basic_code_copy,
            });
        }
    }
    Ok(basic_code)
}

fn validate_check_digit_value(payload: &[u8], check_digit: u8) -> Result<u8, Error> {
    if !check_digit.is_ascii_digit() {
        Err(Error::InvalidCheckDigit { was: check_digit })
    } else {
        let computed_check_digit = compute_check_digit(payload);
        if check_digit != computed_check_digit {
            Err(Error::IncorrectCheckDigit {
                was: check_digit,
                expected: computed_check_digit,
            })
        } else {
            Ok(check_digit)
        }
    }
}

/// Parse a string to a valid ISIN or an error message, requiring the string to already be only
/// uppercase alphanumerics with no leading or trailing whitespace in addition to being the
/// right length and format.
pub fn parse(value: &str) -> Result<ISIN, Error> {
    let value = validate(value)?;

    let mut bb = [0u8; 12];
    bb.copy_from_slice(value);

    Ok(ISIN(bb))
}

/// Parse a string to a valid ISIN or an error, allowing the string to contain leading
/// or trailing whitespace and/or lowercase letters as long as it is otherwise the right length
/// and format.
pub fn parse_loose(value: &str) -> Result<ISIN, Error> {
    let uc = value.to_ascii_uppercase();
    let temp = uc.trim();
    parse(temp)
}

/// Build an ISIN from a _Payload_ (an already-concatenated _Prefix_ and _Basic Code_). The
/// _Check Digit_ is automatically computed.
pub fn build_from_payload(payload: &str) -> Result<ISIN, Error> {
    // We make the preliminary assumption that the string is pure ASCII, so we work with the
    // underlying bytes. If there is Unicode in the string, the bytes will be outside the
    // allowed range and format validations will fail.

    let b = payload.as_bytes();

    validate_payload_format(b)?;

    let mut bb = [0u8; 12];

    bb[0..11].copy_from_slice(b);
    bb[11] = compute_check_digit(b);

    Ok(ISIN(bb))
}

/// Build an ISIN from its parts: an _Prefix_ and an _Basic Code_. The _Check Digit_ is
/// automatically computed.
pub fn build_from_parts(prefix: &str, basic_code: &str) -> Result<ISIN, Error> {
    if prefix.len() != 2 {
        return Err(Error::InvalidPrefixStringLength { was: prefix.len() });
    }
    let prefix: &[u8] = &prefix.as_bytes()[0..2];
    validate_prefix_format(prefix)?;

    if basic_code.len() != 9 {
        return Err(Error::InvalidBasicCodeStringLength {
            was: basic_code.len(),
        });
    }
    let basic_code: &[u8] = &basic_code.as_bytes()[0..9];
    validate_basic_code_format(basic_code)?;

    let mut bb = [0u8; 12];

    bb[0..2].copy_from_slice(prefix);
    bb[2..11].copy_from_slice(basic_code);
    bb[11] = compute_check_digit(&bb[0..11]);

    Ok(ISIN(bb))
}

/// Test whether or not the passed string is in valid ISIN _Payload_ format.
fn validate_payload_format(payload: &[u8]) -> Result<&[u8], Error> {
    if payload.len() != 11 {
        return Err(Error::InvalidPayloadArrayLength { was: payload.len() });
    }

    // We slice out the _Prefix_ and _Basic Code_ fields and validate their formats.

    let prefix: &[u8] = &payload[0..2];
    validate_prefix_format(prefix)?;

    let basic_code: &[u8] = &payload[2..11];
    validate_basic_code_format(basic_code)?;

    Ok(payload)
}

/// Test whether or not the passed string is in valid ISIN format, without producing a ISIN struct
/// value.
pub fn validate(value: &str) -> Result<&[u8], Error> {
    if value.len() != 12 {
        return Err(Error::InvalidValueStringLength { was: value.len() });
    }

    // We make the preliminary assumption that the string is pure ASCII, so we work with the
    // underlying bytes. If there is Unicode in the string, the bytes will be outside the
    // allowed range and format validations will fail.

    let b = value.as_bytes();

    if value.len() != 12 {
        return Err(Error::InvalidValueArrayLength { was: value.len() });
    }

    // We slice out the _Payload_ and _Check Digit_ and validate their formats, as well as the value of the _Check Digit_.

    let payload: &[u8] = &b[0..11];
    validate_payload_format(payload)?;

    let check_digit = b[11];
    validate_check_digit_value(payload, check_digit)?;

    Ok(b)
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// An ISIN in confirmed valid format.
///
/// You cannot construct an ISIN value manually. This does not compile:
///
/// ```compile_fail
/// use isin;
/// let cannot_construct = isin::ISIN([0_u8; 12]);
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[repr(transparent)]
#[allow(clippy::upper_case_acronyms)]
pub struct ISIN([u8; 12]);

impl AsRef<str> for ISIN {
    fn as_ref(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0[..]) } // This is safe because we know it is ASCII
    }
}

impl fmt::Display for ISIN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp = unsafe { from_utf8_unchecked(self.as_bytes()) }; // This is safe because we know it is ASCII
        write!(f, "{temp}")
    }
}

impl fmt::Debug for ISIN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp = unsafe { from_utf8_unchecked(self.as_bytes()) }; // This is safe because we know it is ASCII
        write!(f, "ISIN({temp})")
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ISIN {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = ISIN;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an ISIN")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                crate::parse(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ISIN {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl FromStr for ISIN {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_loose(s)
    }
}

impl ISIN {
    /// Internal convenience function for treating the ASCII characters as a byte-array slice.
    fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    /// Return just the _Prefix_ portion of the ISIN.
    pub fn prefix(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0[0..2]) } // This is safe because we know it is ASCII
    }

    /// Return just the _Basic Code_ portion of the ISIN.
    pub fn basic_code(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0[2..11]) } // This is safe because we know it is ASCII
    }

    /// Return the _Payload_ &mdash; everything except the _Check Digit_.
    pub fn payload(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0[0..11]) } // This is safe because we know it is ASCII
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
            Err(err) => panic!("Did not expect parsing to fail: {}", err),
        }
    }

    #[test]
    fn build_isin_for_apple_from_payload() {
        match build_from_payload("US037833100") {
            Ok(isin) => {
                assert_eq!(isin.to_string(), "US0378331005");
                assert_eq!(isin.prefix(), "US");
                assert_eq!(isin.basic_code(), "037833100");
                assert_eq!(isin.check_digit(), '5');
            }
            Err(err) => panic!("Did not expect building to fail: {}", err),
        }
    }

    #[test]
    fn build_isin_for_apple_from_parts() {
        match build_from_parts("US", "037833100") {
            Ok(isin) => {
                assert_eq!(isin.to_string(), "US0378331005");
                assert_eq!(isin.prefix(), "US");
                assert_eq!(isin.basic_code(), "037833100");
                assert_eq!(isin.check_digit(), '5');
            }
            Err(err) => panic!("Did not expect building to fail: {}", err),
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
            Err(err) => panic!("Did not expect parsing to fail: {}", err),
        }
    }

    #[test]
    fn validate_examples_from_standard_annex_c() {
        assert!(validate("ES0SI0000005").is_ok()); // Example 1, page 10: "IBEX 35"
        assert!(validate("JP3788600009").is_ok()); // Example 2, page 11: "Hitachi Ltd. Shares"
        assert!(validate("DE000A0GNPZ3").is_ok()); // Example 3, page 11: "Allianz Finance II 5 3/8% without expiration date"
    }

    #[test]
    fn validate_examples_from_standard_annex_e() {
        assert!(validate("JP3788600009").is_ok()); // Page 13
        assert!(validate("US9047847093").is_ok()); // Page 13
        assert!(validate("IE00BFXC1P95").is_ok()); // Page 13
        assert!(validate("DE000A0GNPZ3").is_ok()); // Page 13
        assert!(validate("XS2021448886").is_ok()); // Page 13
        assert!(validate("US36962GXZ26").is_ok()); // Page 13
        assert!(validate("FR0000571077").is_ok()); // Page 13
        assert!(validate("US277847UB38").is_ok()); // Page 13
        assert!(validate("US65412AEW80").is_ok()); // Page 13
        assert!(validate("GB00BF0FCW58").is_ok()); // Page 13
        assert!(validate("FR0000312928").is_ok()); // Page 13
        assert!(validate("DE000DL3T7M1").is_ok()); // Page 13

        assert!(validate("ES0A02234250").is_ok()); // Page 14
        assert!(validate("EZR9HY1361L7").is_ok()); // Page 14
        assert!(validate("CH0107166065").is_ok()); // Page 14
        assert!(validate("XS0313614355").is_ok()); // Page 14
        assert!(validate("DE000A0AE077").is_ok()); // Page 14
        assert!(validate("CH0002813860").is_ok()); // Page 14
        assert!(validate("TRLTCMB00045").is_ok()); // Page 14
        assert!(validate("ES0SI0000005").is_ok()); // Page 14
        assert!(validate("GB00B56Z6W79").is_ok()); // Page 14
        assert!(validate("AU000000SKI7").is_ok()); // Page 14
        assert!(validate("EU000A1RRN98").is_ok()); // Page 14
        assert!(validate("LI0024807526").is_ok()); // Page 14
    }

    #[test]
    fn reject_empty_string() {
        let res = parse("");
        assert!(res.is_err());
    }

    #[test]
    fn reject_lowercase_prefix_if_strict() {
        match parse("us0378331005") {
            Err(Error::InvalidPrefix { was: _ }) => {} // Ok
            Err(err) => {
                panic!(
                    "Expected Err(InvalidPrefix {{ ... }}), but got: Err({:?})",
                    err
                )
            }
            Ok(isin) => {
                panic!(
                    "Expected Err(InvalidPrefix {{ ... }}), but got: Ok({:?})",
                    isin
                )
            }
        }
    }

    #[test]
    fn reject_lowercase_basic_code_if_strict() {
        match parse("US09739d1000") {
            Err(Error::InvalidBasicCode { was: _ }) => {} // Ok
            Err(err) => {
                panic!(
                    "Expected Err(InvalidBasicCode {{ ... }}), but got: Err({:?})",
                    err
                )
            }
            Ok(isin) => {
                panic!(
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
        assert!(parse("𑴈𐎟 0 A").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            parse(&s);
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use crate::ISIN;

        use proptest::{prop_assert, prop_assert_eq, proptest};
        use serde::de::value::{self, StrDeserializer};
        use serde::Deserialize as _;

        #[test]
        fn deserialize_apple() {
            let isin = ISIN::deserialize(StrDeserializer::<value::Error>::new("US0378331005"))
                .expect("successful deserialization");
            assert_eq!(isin.to_string(), "US0378331005");
            assert_eq!(isin.prefix(), "US");
            assert_eq!(isin.basic_code(), "037833100");
            assert_eq!(isin.check_digit(), '5');
        }

        #[test]
        fn reject_empty_string() {
            let _ = ISIN::deserialize(StrDeserializer::<value::Error>::new(""))
                .expect_err("unsuccessful deserialization");
        }

        #[test]
        fn reject_lowercase_prefix_if_strict() {
            let _ = ISIN::deserialize(StrDeserializer::<value::Error>::new("us0378331005"))
                .expect_err("unsuccessful deserialization");
        }

        #[test]
        fn reject_lowercase_basic_code_if_strict() {
            let _ = ISIN::deserialize(StrDeserializer::<value::Error>::new("US09739d1000"))
                .expect_err("unsuccessful deserialization");
        }

        proptest! {
            #[test]
            fn doesnt_crash(s in "\\PC*") {
                let _ = ISIN::deserialize(StrDeserializer::<value::Error>::new(&s));
            }

            #[test]
            fn matches_parse(s in "\\PC*") {
                let parse_result = crate::parse(&s);
                let deserialize_result = ISIN::deserialize(StrDeserializer::<value::Error>::new(&s));

                match (parse_result, deserialize_result)
                {
                    (Ok(parsed_isin), Ok(deserialized_isin)) => prop_assert_eq!(parsed_isin, deserialized_isin),
                    (Ok(_), Err(_)) | (Err(_), Ok(_)) => prop_assert!(false),
                    (Err(_), Err(_)) => {}
                 }
            }
        }
    }
}
