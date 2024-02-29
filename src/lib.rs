use std::num::ParseFloatError;
use failure::Fail;


#[derive(Debug, Fail, PartialEq)]
pub enum MoneyError {
    #[fail(display = "Invalid input: {}", _0)]
    ParseAmount(ParseFloatError),
    
    #[fail(display = "{}", _0)]
    ParseCurrency(String),
    
    #[fail(display = "{}", _0)]
    ParseFormatting(String),
}

impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}

#[derive(Debug, PartialEq)]
enum Currency {
    Dollar,
    Euro,
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dollar" | "$" => Ok(Currency::Dollar),
            "euro" | "eur" | "€" => Ok(Currency::Euro),
            _ => Err(MoneyError::ParseCurrency("Unknown currency".into())),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Money {
    amount: f32,
    currency: Currency,
}

impl Money {
    fn new(amount: f32, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

impl std::str::FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        
        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseFormatting("Expecting amount and currency".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors() {
        assert_eq!(
            "140.01".parse::<Money>(),
            Err(MoneyError::ParseFormatting(
                "Expecting amount and currency".into()
            ))
        );

        let result = "OneMillion Euro".parse::<Money>();
        assert!(result.is_err());
    }

    #[test]
    fn test_successful_parsing() {
        let testcases = vec![
            (
                "100 Euro",
                Money {
                    amount: 100.0,
                    currency: Currency::Euro,
                },
            ),
            (
                "10 $",
                Money {
                    amount: 10.0,
                    currency: Currency::Dollar,
                },
            ),
            (
                "42.4 DOLLAR",
                Money {
                    amount: 42.4,
                    currency: Currency::Dollar,
                },
            ),
        ];

        for (input, output) in testcases {
            assert_eq!(input.parse::<Money>(), Ok(output));
        }
    }
}
