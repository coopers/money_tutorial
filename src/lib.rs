pub fn parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse();
    if maybe_amount.is_err() {
        return (-1, "invalid".to_string());
    }
    let currency = parts[1].to_string();
    (maybe_amount.unwrap(), currency)
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
}