//! # isin::checksum
//!
//! Functions to implement the ISIN checksum formula, known as "modulus 10 'double-add-double' check
//! digit". The basic idea is to:
//!
//! 1. assign a numeric value to each character
//! 2. split that value into digits
//! 3. iterate over the digits in reverse order
//! 4. double every other digit starting with the right-most one
//! 5. split that result into digits
//! 6. sum over all the digits
//! 7. calculate the sum mod 10
//! 8. compute 10 minus that value
//! 9. if the value is 10, return 0 else return the value itself
//!
//! There is an implementation in a functional style, `checksum_functional()` that is used
//! internally for tests and as a comparison for performance benchmarks. It is more expensive than
//! the table-driven style because it does digit expansion on the fly. But, it is easier to
//! understand. The implementation maps directly to the description above.
//!
//! There is also an implementation in a table-driven style, `checksum_table()` which is the one
//! actually used when parsing and validating ISINs. The tables are pre-calculated for the net
//! effect each character has on the checksum accumulator at that point and how it effects
//! whether the next character is in a doubling position or not.
//!
//! Benchmarking shows the table-driven implementation to be around 70 to 90 times faster
//! than the functional style (on the test system, average run time decreases from around 2.1 to 2.9
//! us with the functional style to around 30.6 ns with the table-driven style). Input-dependent
//! variability in run time decreases also from about +/- 20% for the functional-style to negligible
//! for the table-driven style.

/// The numeric value of a u8 ASCII character. Digit characters '0' through '9' map to values 0
/// through 9, and letter characters 'A' through 'Z' map to values 10 through 35.
///
/// # Panics
///
/// If anything other than an uppercase ASCII alphanumeric character is passed in, this function
/// panics because it is only intended to be called from locations where the input has already been
/// validated to match the character set requirements.
fn char_value(c: &u8) -> u8 {
    if (b'0'..=b'9').contains(&c) {
        c - b'0'
    } else if (b'A'..=b'Z').contains(&c) {
        c - b'A' + 10
    } else {
        panic!("Non-ASCII-alphanumeric characters should be impossible here!");
    }
}

/// A direct translation of the formula definition, in the functional style.
#[allow(dead_code)]
pub fn checksum_functional(s: &[u8]) -> u8 {
    fn digits_of(x: u8) -> Vec<u8> {
        if x >= 10 {
            vec![x / 10, x % 10]
        } else {
            vec![x]
        }
    }

    let sum: u32 = s
        .iter()
        .map(char_value)
        .flat_map(digits_of)
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

/// The width in "steps" each char value consumes when processed. All decimal digits have width
/// one, and all letters have width two (because their values are two digits, from 10 to 35
/// inclusive).
#[rustfmt::skip]
const WIDTHS: [u8; 36] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2,
];

/// The net value added to the sum for each char value, if the step count (aka index) at the
/// start of processing that character is odd. Odds vs. evens differ because evens go through
/// doubling and potentially splitting into two digits before being summed to make the net value.
#[rustfmt::skip]
const ODDS: [u8; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    2, 3, 4, 5, 6, 7, 8, 9, 0, 1,
    4, 5, 6, 7, 8, 9, 0, 1, 2, 3,
    6, 7, 8, 9, 0, 1,
];

/// The net value added to the sum for each char value, if the step count (aka index) at the
/// start of processing that character is even. Odds vs. evens differ because evens go through
/// doubling and potentially splitting into two digits before being summed to make the net value.
#[rustfmt::skip]
const EVENS: [u8; 36] = [
    0, 2, 4, 6, 8,
    1, 3, 5, 7, 9,
    1, 3, 5, 7, 9,
    2, 4, 6, 8, 0,
    2, 4, 6, 8, 0,
    3, 5, 7, 9, 1,
    3, 5, 7, 9, 1,
    4,
];

/// Compute the _checksum_ for a u8 array. No attempt is made to ensure the input string is in
/// the ISIN payload format or length.
///
/// # Panics
///
/// If an illegal character (not an ASCII digit and not an
/// ASCII uppercase letter) is encountered, the char_value() function this calls will panic.
pub fn checksum_table(s: &[u8]) -> u8 {
    let mut sum: u8 = 0;
    let mut idx: usize = 0;
    for c in s.iter().rev() {
        let v = char_value(c);
        let w = WIDTHS[v as usize];
        let x = if (idx % 2) == 0 {
            EVENS[v as usize]
        } else {
            ODDS[v as usize]
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Ensure the table-driven method gets the same answer as the functional style implementation
    // for each allowed symbol by itself, which exercises the EVEN table, as counted from the right.
    #[test]
    fn single_chars() {
        for c in ('0'..='9').into_iter().chain(('A'..='Z').into_iter()) {
            let s = c.to_string();
            let ss = s.as_bytes();
            let a = checksum_functional(&ss);
            let b = checksum_table(&ss);
            assert_eq!(
                a, b,
                "checksum from library {} should equal that from functional style {} for \"{}\"",
                b, a, s
            );
        }
    }

    // Ensure the table-driven method gets the same answer as the functional style implementation
    // for each allowed symbol followed just by a single zero, which exercises the ODD table, as
    // counted from the *right*.
    #[test]
    fn single_chars_left_of_zero() {
        for c in ('0'..='9').into_iter().chain(('A'..='Z').into_iter()) {
            let s = format!("{}0", c);
            let ss = s.as_bytes();
            let a = checksum_functional(&ss);
            let b = checksum_table(&ss);
            assert_eq!(
                a, b,
                "checksum from library {} should equal that from functional style {} for \"{}\"",
                b, a, s
            );
        }
    }

    // Ensure the table-driven method gets the same answer as the functional style implementation
    // for each allowed symbol preceded just by a single nine, which exercises the WIDTH table.
    #[test]
    fn nine_left_of_single_chars() {
        for c in ('0'..='9').into_iter().chain(('A'..='Z').into_iter()) {
            let s = format!("9{}", c);
            let ss = s.as_bytes();
            let a = checksum_functional(&ss);
            let b = checksum_table(&ss);
            assert_eq!(
                a, b,
                "checksum from library {} should equal that from functional style {} for \"{}\"",
                b, a, s
            );
        }
    }

    proptest! {
        #[test]
        fn processes_all_valid_strings(s in "[A-Z]{2}[0-9A-Z]{9}") {
            let ss = s.as_bytes();
            let a = checksum_functional(&ss);
            let b = checksum_table(&ss);
            assert_eq!(
                a, b,
                "checksum from library {} should equal that from functional style {} for \"{}\"",
                b, a, s
            );
        }
    }
}
