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
//! Use the `ISIN::parse_loose()` or `ISIN::parse_strict()` methods to convert a string to a
//! validated ISIN.

use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

pub mod checksum;

use checksum::checksum_table;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The input contains non-ASCII characters.
    NonAscii { was: String },
    /// The input length is not exactly 12 bytes.
    InvalidLength { was: usize },
    /// The input country code is not two uppercase ASCII alphabetic characters.
    InvalidCountryCode { was: String },
    /// The input security identifier is not nine uppercase ASCII alphanumeric characters.
    InvalidSecurityIdentifier { was: String },
    /// The input check digit is not a single ASCII decimal digit character.
    InvalidCheckDigit { was: String },
    /// The input check digit has in a valid format, but has an incorrect value.
    IncorrectCheckDigit { was: char, expected: char },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NonAscii { was } => {
                write!(f, "one or more non-ASCII characters in '{}'", was)
            }
            ParseError::InvalidLength { was } => {
                write!(f, "invalid length {} bytes when expecting 12", was)
            }
            ParseError::InvalidCountryCode { was } => {
                write!(
                    f,
                    "invalid country code '{}' is not two uppercase ASCII alphabetic characters",
                    was
                )
            }
            ParseError::InvalidSecurityIdentifier { was } => {
                write!(f, "invalid security identifier '{}' is not nine uppercase ASCII alphanumeric characters", was)
            }
            ParseError::InvalidCheckDigit { was } => {
                write!(
                    f,
                    "invalid check digit '{}' is not one ASCII decimal digit",
                    was
                )
            }
            ParseError::IncorrectCheckDigit { was, expected } => {
                write!(
                    f,
                    "incorrect check digit '{}' when expecting '{}'",
                    was, expected
                )
            }
        }
    }
}

impl Error for ParseError {}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ISIN {
    value: String,
}

impl Display for ISIN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISIN({})", self.value)
    }
}

impl FromStr for ISIN {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ISIN::parse_loose(s)
    }
}

impl ISIN {
    /// Compute the _check digit_ for a string. No attempt is made to ensure the input string is in
    /// the ISIN payload format or length. If an illegal character (not an ASCII digit and not an
    /// ASCII uppercase letter) is encountered, this function will panic.
    pub fn compute_check_digit(s: &str) -> char {
        let sum = checksum_table(s);
        (b'0' + sum) as char
    }

    /// Parse a string to a valid ISIN or an error message, requiring the string to already be only
    /// uppercase alphanumerics with no leading or trailing whitespace in addition to being the
    /// right length and format.
    pub fn parse_strict<S>(value: S) -> Result<ISIN, ParseError>
    where
        S: Into<String>,
    {
        let v: String = value.into();

        if !v.is_ascii() {
            return Err(ParseError::NonAscii { was: v });
        }

        if v.len() != 12 {
            return Err(ParseError::InvalidLength { was: v.len() });
        }

        // Because the string is pure ASCII, we can slice fields assuming one-byte characters

        let cc = &v[0..2];
        let si = &v[2..11];
        let cd = &v[11..12];

        // Now, we validate the format of each fields

        let invalid_country_code =
            cc.contains(|c: char| !(c.is_ascii_alphabetic() && c.is_uppercase()));
        if invalid_country_code {
            return Err(ParseError::InvalidCountryCode {
                was: String::from(cc),
            });
        }

        let invalid_security_id = si.contains(|c: char| {
            (!c.is_ascii_alphanumeric()) || (c.is_ascii_alphabetic() && !c.is_uppercase())
        });
        if invalid_security_id {
            return Err(ParseError::InvalidSecurityIdentifier {
                was: String::from(si),
            });
        }

        let invalid_check_digit = cd.contains(|c: char| !c.is_ascii_digit());
        if invalid_check_digit {
            return Err(ParseError::InvalidCheckDigit {
                was: String::from(cd),
            });
        }

        // Now, we need to compute the correct check digit value from the "payload" (the country
        // code and security identifier fields).

        let payload = &v[0..11];

        let computed_check_digit = Self::compute_check_digit(payload);
        let input_check_digit = cd.as_bytes()[0] as char;

        let incorrect_check_digit = input_check_digit != computed_check_digit;
        if incorrect_check_digit {
            return Err(ParseError::IncorrectCheckDigit {
                was: input_check_digit,
                expected: computed_check_digit,
            });
        }

        Ok(ISIN { value: v })
    }

    /// Parse a string to a valid ISIN or an error message, allowing the string to contain leading
    /// or trailing whitespace and/or lowercase letters as long as it is otherwise the right length
    /// and format.
    pub fn parse_loose<S>(value: S) -> Result<ISIN, ParseError>
    where
        S: Into<String>,
    {
        let uc = value.into().to_ascii_uppercase();
        let temp = uc.trim();
        Self::parse_strict(temp)
    }

    /// Return the underlying string value of the ISIN.
    pub fn value(&self) -> &str {
        &*(self.value)
    }

    /// Return just the _country code_ portion of the ISIN.
    pub fn country_code(&self) -> &str {
        &(self.value)[0..2]
    }

    /// Return just the _security identifier_ portion of the ISIN.
    pub fn security_identifier(&self) -> &str {
        &(self.value)[2..11]
    }

    /// Return the &ldquo;payload&rdquo; &mdash; everything but the check digit.
    pub fn payload(&self) -> &str {
        &(self.value)[0..11]
    }

    /// Return just the _check digit_ portion of the ISIN.
    pub fn check_digit(&self) -> &str {
        &(self.value)[11..12]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parse_isin_for_apple_strict() {
        match ISIN::parse_strict("US0378331005") {
            Ok(isin) => {
                assert_eq!(isin.value(), "US0378331005");
                assert_eq!(isin.country_code(), "US");
                assert_eq!(isin.security_identifier(), "037833100");
                assert_eq!(isin.check_digit(), "5");
            }
            Err(_) => assert!(false, "Did not expect parsing to fail"),
        }
    }

    #[test]
    fn parse_isin_for_apple_loose() {
        match ISIN::parse_loose("\tus0378331005    ") {
            Ok(isin) => {
                assert_eq!(isin.value(), "US0378331005");
                assert_eq!(isin.country_code(), "US");
                assert_eq!(isin.security_identifier(), "037833100");
                assert_eq!(isin.check_digit(), "5");
            }
            Err(_) => assert!(false, "Did not expect parsing to fail"),
        }
    }

    #[test]
    fn reject_empty_string() {
        let res = ISIN::parse_strict("");
        assert!(res.is_err());
    }

    #[test]
    fn reject_lowercase_country_code_if_strict() {
        let res = ISIN::parse_strict("us0378331005");
        assert!(res.is_err());
    }

    #[test]
    fn reject_lowercase_security_id_if_strict() {
        let res = ISIN::parse_strict("US09739d1000");
        assert!(res.is_err());
    }

    #[test]
    fn parse_isin_with_0_check_digit() {
        ISIN::parse_strict("US09739D1000").unwrap(); // BCC aka Boise Cascade
    }

    #[test]
    fn parse_isin_with_1_check_digit() {
        ISIN::parse_strict("US4581401001").unwrap(); // INTC aka Intel
    }

    #[test]
    fn parse_isin_with_2_check_digit() {
        ISIN::parse_strict("US98421M1062").unwrap(); // XRX aka Xerox
    }

    #[test]
    fn parse_isin_with_3_check_digit() {
        ISIN::parse_strict("US02376R1023").unwrap(); // AAL aka American Airlines
    }

    #[test]
    fn parse_isin_with_4_check_digit() {
        ISIN::parse_strict("US9216591084").unwrap(); // VNDA aka Vanda Pharmaceuticals
    }

    #[test]
    fn parse_isin_with_5_check_digit() {
        ISIN::parse_strict("US0207721095").unwrap(); // APT aka AlphaProTec
    }

    #[test]
    fn parse_isin_with_6_check_digit() {
        ISIN::parse_strict("US71363P1066").unwrap(); // PRDO aka Perdoceo Education
    }

    #[test]
    fn parse_isin_with_7_check_digit() {
        ISIN::parse_strict("US5915202007").unwrap(); // MEI aka Methode Electronics
    }

    #[test]
    fn parse_isin_with_8_check_digit() {
        ISIN::parse_strict("US4570301048").unwrap(); // IMKTA aka Ingles Markets
    }

    #[test]
    fn parse_isin_with_9_check_digit() {
        ISIN::parse_strict("US8684591089").unwrap(); // SUPN aka Supernus Pharmaceuticals
    }

    #[test]
    fn test_unicode_gibberish() {
        assert_eq!(true, ISIN::parse_strict("𑴈𐎟 0 A").is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            ISIN::parse_strict(&s);
        }
    }
}
