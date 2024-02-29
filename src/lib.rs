 use std::num::ParseIntError;

pub fn parse_money(input: &str) -> Result<(i32, String), ParseIntError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let amount = parts[0].parse()?;
    let currency = parts[1].to_string();
    Ok((amount, currency))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_amount_and_currency_from_a_string() {
        if let Ok((amount, currency)) = parse_money("140 Euro"){
            assert_eq!(amount, 140);
            assert_eq!(currency, "Euro");
        } else {
            assert!(false)
        }
    }

    #[test]
    fn it_handles_error_when_amount_is_a_float() {
        let res = parse_money("140.01 Euro");
        assert!(res.is_err())
    }
}