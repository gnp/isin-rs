isin
====
An `ISIN` type for working with validated International Security Identifiers (ISINs) as defined in
[ISO 6166](https://www.iso.org/standard/78502.html).

The checksum calculation uses a table-driven algorithm to minimize overhead _vs._ a direct translation of the formula
definition.

This crate is part of the Financial Identifiers series:

* [CIK](https://crates.io/crates/cik): Central Index Key (SEC EDGAR)
* [CUSIP](https://crates.io/crates/cusip): Committee on Uniform Security Identification Procedures (ANSI X9.6-2020)
* [ISIN](https://crates.io/crates/isin): International Securities Identification Number (ISO 6166:2021)
* [LEI](https://crates.io/crates/lei): Legal Entity Identifier (ISO 17442:2020)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
isin = "0.1"
```


## Example

```rust
use isin;
let isin_string = "US0378331005";
match isin::parse(isin_string) {
    Ok(isin) => {
        println!("Parsed ISIN: {}", isin.to_string()); // "US0378331005"
        println!("  Prefix: {}", isin.prefix()); // "US"
        println!("  Basic code: {}", isin.basic_code()); // "037833100"
        println!("  Check digit: {}", isin.check_digit()); // '5'
    }
    Err(err) => panic!("Unable to parse ISIN {}: {}", isin_string, err),
}
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
