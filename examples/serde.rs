use isin::ISIN;

fn main() {
    let isin: ISIN = "US0378331005".parse().unwrap();

    let serialized = serde_json::to_string(&isin).unwrap();
    println!("Serialized ISIN: {}", serialized);

    let deserialized: ISIN = serde_json::from_str(&serialized).unwrap();

    println!("Deserialized ISIN: {}", deserialized); // "US0378331005"
    println!("  Prefix: {}", deserialized.prefix()); // "US"
    println!("  Basic code: {}", deserialized.basic_code()); // "037833100"
    println!("  Check digit: {}", deserialized.check_digit()); // '5'
    assert_eq!(isin, deserialized);
}
