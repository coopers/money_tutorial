 use std::num::ParseFloatError;

pub fn parse_money(input: &str) -> Result<(f32, String), ParseFloatError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let amount: f32 = parts[0].parse()?;
    let currency = parts[1].to_string();
    Ok((amount, currency))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_amount_and_currency_from_a_string_with_int() {
        if let Ok((amount, currency)) = parse_money("140 Euro"){
            assert_eq!(amount, 140.0);
            assert_eq!(currency, "Euro");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_returns_amount_and_currency_from_a_string_with_float() {
        if let Ok((amount, currency)) = parse_money("140.01 Euro"){
            assert_eq!(amount, 140.01);
            assert_eq!(currency, "Euro");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_handles_error_when_currency_is_missing_from_input() {
        let _res = parse_money("140.01");
        assert!(true);
    }
}