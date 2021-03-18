use isin::ISIN;

fn main() {
    let isin_string = "US0378331005";
    match ISIN::parse_strict(isin_string) {
        Ok(isin) => {
            println!("Parsed ISIN: {}", isin.value()); // "US0378331005"
            println!("  Country code: {}", isin.country_code()); // "US"
            println!("  Security identifier: {}", isin.security_identifier()); // "037833100"
            println!("  Check digit: {}", isin.check_digit()); // "5"
        }
        Err(err_string) => panic!("Unable to parse ISIN {}: {}", isin_string, err_string),
    }
}
