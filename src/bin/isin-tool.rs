//! This simple tool reads potential ISINs from stdin, one per line, and parses them. It will panic
//! if any fail to parse. This can be used as a simple bulk test of a file of purported ISINs to
//! ensure there are no malformed entries present. If you have a known-good file of valid ISINs, it
//! can be used to validate this crate considers them valid.
//!
//! As part of the `isin` crate's initial validation, this tool was run on a file of 5,273,047
//! unique ISINs produced by processing a file mapping LEIs to ISINs obtained from GLEIF. The
//! [GLEIF file](https://www.gleif.org/en/lei-data/lei-mapping/download-isin-to-lei-relationship-files)
//! is very large (the version from 2021-02-09 was about 170MB). Here are a few example records from
//! the beginning of the file (the first line is the header row with field names):
//!
//! ```sh
//! head -11 ISIN_LEI_20210209.csv | tail -10
//! S6XOOCT0IEG5ABCC6L87,US3137A3KN83
//! XZYUUT6IYN31D9K77X08,DE000JC86RE7
//! 378900EB75D7D2C73323,ZAG000163650
//! 254900EDYO1UYWLWP146,US12613N2027
//! 549300DRQQI75D2JP341,US05531GQN42
//! S6XOOCT0IEG5ABCC6L87,US31394GAX16
//! K6Q0W1PS1L1O4IQL9C32,DE000SLA61X8
//! 529900W18LQJJN6SJ336,DE000CL78501
//! S6XOOCT0IEG5ABCC6L87,US3137ASGH19
//! G5GSEF7VJP5I7OUK5573,US06741RAP64
//! ```
//!
//! You can use a command like this to subset just the ISINs:
//!
//! ```sh
//! sed -e 's/^.*,//' ISIN_LEI_20210209.csv \
//!   | grep -v '^ISIN$' \
//!   | sort | uniq | gzip -9 \
//!   > isins.txt.gz
//! ```
//!
//! This file was about 16.6MB for the version tested and contained over 5.2 million ISINs.
//!
//! Having produced the file, it is now possible to run it through this tool. From the source
//! directory of this crate, you can run:
//!
//! ```sh
//! gzcat isins.txt.gz | cargo run isin-tool
//! ```
//!
//! And, output will be something like this:
//!
//! ```text
//! Read 5273047 values; 5273047 were valid ISINs and 0 were not.
//! ```
//!
//! If no bad values were found, the tool will exit with zero status, else non-zero.
//!
//! ## Fix mode
//!
//! If you run with argument `--fix`, then any input ISINs that are only wrong due to incorrect
//! _Check Digit_ will be fixed. In this mode, every good and every fixable input ISIN is printed
//! to standard output.

use std::env;
use std::io;
use std::io::prelude::*;
use std::str::from_utf8_unchecked;

#[doc(hidden)]
fn main() {
    let mut fix: bool = false;

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "--fix" {
        fix = true;
    } else if args.len() != 1 {
        eprintln!("usage: isin-tool [--fix]");
        std::process::exit(1);
    }

    let mut good = 0u64;
    let mut bad = 0u64;
    let mut fixed = 0u64;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match isin::parse(&line) {
            Ok(isin) => {
                good += 1;
                if fix {
                    println!("{isin}");
                }
            }
            Err(isin::ISINError::IncorrectCheckDigit {
                was: _,
                expected: _,
            }) => {
                bad += 1;
                if fix {
                    let payload = &line.as_bytes()[0..11]; // We know it was the right length
                    let payload = unsafe { from_utf8_unchecked(payload) }; // We know it is ASCII

                    // We know the Check Digit was the only problem, so we can safely unwrap()
                    let isin = isin::build_from_payload(payload).unwrap();
                    println!("{isin}");
                    fixed += 1;
                }
            }
            Err(err) => {
                eprintln!("Input: {line}; Error: {err}");
                bad += 1;
            }
        }
    }

    if fix {
        eprintln!(
            "Read {} values; {} were valid ISINs and {} were not. Fixed {}; Omitted {}.",
            good + bad,
            good,
            bad,
            fixed,
            bad - fixed
        );

        if bad > fixed {
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    } else {
        eprintln!(
            "Read {} values; {} were valid ISINs and {} were not.",
            good + bad,
            good,
            bad
        );

        let result = (bad == 0) as i32;
        std::process::exit(result);
    }
}
