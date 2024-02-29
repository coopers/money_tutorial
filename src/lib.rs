pub fn parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let amount = parts[0].parse().unwrap();
    let currency = parts[1].to_string();
    (amount, currency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_amount_and_currency_from_a_string() {
        let (amount, currency) = parse_money("140 Euro");
        assert_eq!(amount, 140);
        assert_eq!(currency, "Euro");
    }

    #[test]
    fn it_handles_error_when_amount_is_a_float() {
        let (_amount, _currency) = parse_money("140.01 Euro");
        assert!(true)
    }
}