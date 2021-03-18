use isin::ISIN;

fn main() {
    let isin: ISIN = "US0378331005".parse().unwrap();
    println!("Parsed ISIN: {}", isin.value()); // "US0378331005"
    println!("  Country code: {}", isin.country_code()); // "US"
    println!("  Security identifier: {}", isin.security_identifier()); // "037833100"
    println!("  Check digit: {}", isin.check_digit()); // "5"
}
