use isin::ISIN;

fn main() {
    let isin: ISIN = "US0378331005".parse().unwrap();
    println!("Parsed ISIN: {}", isin.to_string()); // "US0378331005"
    println!("  Prefix: {}", isin.prefix()); // "US"
    println!("  Basic code: {}", isin.basic_code()); // "037833100"
    println!("  Check digit: {}", isin.check_digit()); // '5'
}
