pub fn parse_money(_input: &str) -> (i32, String) {
    return (0, "".to_string())
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