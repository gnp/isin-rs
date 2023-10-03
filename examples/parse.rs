fn main() {
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
}
