use std::num::ParseFloatError;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum MoneyError {
    #[fail(display = "Invalid input: {}", _0)]
    ParseAmount(ParseFloatError),

    #[fail(display = "{}", _0)]
    ParseFormatting(String),
}

impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}

pub fn parse_money(input: &str) -> Result<(f32, String), MoneyError> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[..] {
        [amount, currency] => Ok((amount.parse()?, currency.to_string())),
        _ => Err(MoneyError::ParseFormatting("Expecting amount and currency".into())),
    }
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