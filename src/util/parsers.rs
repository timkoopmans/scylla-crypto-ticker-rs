use regex::Regex;

pub fn extract_prices_and_amounts(levels_str: &str) -> Vec<(f64, f64)> {
    let price_pattern = Regex::new(r"price: (\d+\.\d+).+?amount: (\d+\.\d+)").unwrap();
    price_pattern
        .captures_iter(levels_str)
        .map(|capture|
                 (
                     capture[1].parse().expect("Failed to parse price"),
                     capture[2].parse().expect("Failed to parse amount"),
                 )
            )
        .collect()
}
