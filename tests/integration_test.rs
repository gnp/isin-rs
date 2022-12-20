#[test]
fn test_readme_example_main() {
    use isin;

    // let cannot_construct = isin::ISIN([0_u8; 12]); // You cannot construct manually

    let isin_string = "US0378331005";
    match isin::parse(isin_string) {
        Ok(isin) => {
            println!("Parsed ISIN: {}", isin.to_string()); // "US0378331005"
            println!("  Prefix: {}", isin.prefix()); // "US"
            println!("  Basic code: {}", isin.basic_code()); // "037833100"
            println!("  Check digit: {}", isin.check_digit()); // "5"
        }
        Err(err) => panic!("Unable to parse ISIN {}: {}", isin_string, err),
    }
}
