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

use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug)]
pub struct ISIN {
    value: String,
}

impl fmt::Display for ISIN {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ISIN({})", self.value)
    }
}

impl FromStr for ISIN {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ISIN::parse_loose(s)
    }
}

impl ISIN {
    /* The width in "steps" each char value consumes when processed. All decimal digits have width
     * one, and all letters have width two (because their values are two digits, from 10 to 35
     * inclusive).
     */
    const WIDTHS: [u8; 36] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2,
    ];

    /* The net value added to the sum for each char value, if the step count (aka index) at the
     * start of processing that character is odd. Odds vs. evens differ because evens go through
     * doubling and potentially splitting into two digits before being summed to make the net value.
     */
    const ODDS: [u8; 36] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3,
        6, 7, 8, 9, 0, 1,
    ];

    /* The net value added to the sum for each char value, if the step count (aka index) at the
     * start of processing that character is even. Odds vs. evens differ because evens go through
     * doubling and potentially splitting into two digits before being summed to make the net value.
     */
    const EVENS: [u8; 36] = [
        0, 2, 4, 6, 8, 1, 3, 5, 7, 9, 1, 3, 5, 7, 9, 2, 4, 6, 8, 0, 2, 4, 6, 8, 0, 3, 5, 7, 9, 1,
        3, 5, 7, 9, 1, 4,
    ];

    /* The numeric value of a char. Digit characters '0' through '9' map to values 0 through 9, and
     * letter characters 'A' through 'Z' map to values 10 through 35.
     */
    fn char_value(c: char) -> u8 {
        if ('0'..='9').contains(&c) {
            (c as u8) - b'0'
        } else if ('A'..='Z').contains(&c) {
            (c as u8) - b'A' + 10
        } else {
            panic!("Non-ASCII-alphanumeric characters should be impossible here!");
        }
    }

    /** Compute the _checksum_ for a string. No attempt is made to ensure the input string is in
    the ISIN payload format or length. If an illegal character (not an ASCII digit and not an
    ASCII uppercase letter) is encountered, this function will panic. */
    pub fn compute_checksum(s: &str) -> u8 {
        let mut sum: u8 = 0;
        let mut idx: usize = 0;
        for c in s.chars().rev() {
            let v = Self::char_value(c);
            let w = Self::WIDTHS[v as usize];
            let x = if (idx % 2) == 0 {
                Self::EVENS[v as usize]
            } else {
                Self::ODDS[v as usize]
            };
            sum = (sum + x) % 10;
            idx += w as usize;
        }

        let diff = 10 - sum;
        if diff == 10 {
            0
        } else {
            diff
        }
    }

    /** Compute the _check digit_ for a string. No attempt is made to ensure the input string is in
    the ISIN payload format or length. If an illegal character (not an ASCII digit and not an
    ASCII uppercase letter) is encountered, this function will panic. */
    pub fn compute_check_digit(s: &str) -> char {
        let sum = Self::compute_checksum(s);
        (b'0' + sum) as char
    }

    /** Parse a string to a valid ISIN or an error message, requiring the string to already be only
    uppercase alphanumerics with no leading or trailing whitespace in addition to being the
    right length and format. */
    pub fn parse_strict<S>(value: S) -> Result<ISIN, String>
    where
        S: Into<String>,
    {
        let v: String = value.into();
        if v.len() != 12 {
            return Err(String::from("Value must be exactly 12 bytes long"));
        }

        // Here we assume all characters in the string are ASCII and thus one byte long.

        let cc = &v[0..2];
        let si = &v[2..11];
        let cd = &v[11..12];

        /* Now, we test that assumption on each field of the value, left to right. Documentation for
         * contains() does not say that it can panic, so not sure what it will do in the case of a
         * partial UTF-8 character at the start or end boundary of the str within the input String
         * value.
         */

        if cc.contains(|c: char| !c.is_ascii_alphabetic()) {
            return Err(String::from(
                "First two characters of value must all be ASCII letters",
            ));
        }

        if si.contains(|c: char| !c.is_ascii_alphanumeric()) {
            return Err(String::from(
                "Third through 11th characters of value must all be ASCII alphanumerics",
            ));
        }

        if cd.contains(|c: char| !c.is_ascii_digit()) {
            return Err(String::from(
                "Last character of value must be an ASCII digit",
            ));
        }

        /* Now, we need to compute the correct check digit value from the "payload" (the country
         * code and security identifier fields).
         */

        let payload = &v[0..11];

        let digit_char = Self::compute_check_digit(payload);

        if (cd.as_bytes()[0] as char) != digit_char {
            return Err(format!(
                "Payload check digit {} does not match computed check digit {}",
                cd, digit_char
            ));
        }

        Ok(ISIN { value: v })
    }

    /** Parse a string to a valid ISIN or an error message, allowing the string to contain leading
    or trailing whitespace and/or lowercase letters as long as it is otherwise the right length
    and format. */
    pub fn parse_loose<S>(value: S) -> Result<ISIN, String>
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

    // Direct translation of the formula definition, in the functional style.
    fn compute_checksum_functional_style(s: &str) -> u8 {
        fn digits_of(x: u8) -> Vec<u8> {
            if x >= 10 {
                vec![x / 10, x % 10]
            } else {
                vec![x]
            }
        }

        let sum: u32 = s
            .chars()
            .map(ISIN::char_value)
            .flat_map(|x| digits_of(x))
            .rev()
            .enumerate()
            .flat_map(|(i, x)| {
                if (i % 2) == 0 {
                    digits_of(x * 2)
                } else {
                    digits_of(x)
                }
            })
            .map(|x| x as u32)
            .sum();

        let sum = (sum % 10) as u8;

        let diff = 10 - sum;
        if diff == 10 {
            0
        } else {
            diff
        }
    }

    /* Ensure the table-driven method gets the same answer as the functional style implementation
     * for each allowed symbol by itself, which exercises the EVEN table, as counted from the right.
     */
    #[test]
    fn single_chars() {
        for c in ('0'..='9').into_iter().chain(('A'..='Z').into_iter()) {
            let s = c.to_string();
            let a = compute_checksum_functional_style(&s);
            let b = ISIN::compute_checksum(&s);
            assert_eq!(
                a, b,
                "checksum from library {} should equal that from functional style {} for \"{}\"",
                b, a, s
            );
        }
    }

    /* Ensure the table-driven method gets the same answer as the functional style implementation
     * for each allowed symbol followed just by a single zero, which exercises the ODD table, as
     * counted from the *right*.
     */
    #[test]
    fn single_chars_left_of_zero() {
        for c in ('0'..='9').into_iter().chain(('A'..='Z').into_iter()) {
            let s = format!("{}0", c);
            let a = compute_checksum_functional_style(&s);
            let b = ISIN::compute_checksum(&s);
            assert_eq!(
                a, b,
                "checksum from library {} should equal that from functional style {} for \"{}\"",
                b, a, s
            );
        }
    }

    #[test]
    fn parse_isin_for_apple_strict() -> Result<(), String> {
        let isin = ISIN::parse_strict("US0378331005")?;
        assert_eq!(isin.value(), "US0378331005");
        assert_eq!(isin.country_code(), "US");
        assert_eq!(isin.security_identifier(), "037833100");
        assert_eq!(isin.check_digit(), "5");
        Ok(())
    }

    #[test]
    fn parse_isin_for_apple_loose() -> Result<(), String> {
        let isin = ISIN::parse_loose("\tus0378331005    ")?;
        assert_eq!(isin.value(), "US0378331005");
        assert_eq!(isin.country_code(), "US");
        assert_eq!(isin.security_identifier(), "037833100");
        assert_eq!(isin.check_digit(), "5");
        Ok(())
    }

    #[test]
    fn reject_empty_string() {
        let res = ISIN::parse_strict("");
        assert!(res.is_err());
        assert_eq!(
            res.err(),
            Some(String::from("Value must be exactly 12 bytes long"))
        );
    }
}
