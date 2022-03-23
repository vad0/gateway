use std::fmt::{Debug, Display, Formatter};

use strum_macros::{Display, EnumString};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString)]
pub enum Currency {
    BNB,
    BTC,
    LTC,
    ETH,
    GBP,
    USDT,
    XRP,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyPair {
    pub base: Currency,
    pub term: Currency,
}

impl CurrencyPair {
    pub fn new(base: Currency, term: Currency) -> CurrencyPair {
        CurrencyPair { base, term }
    }
}

impl Display for CurrencyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.base, self.term)
    }
}

impl Debug for CurrencyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::currencies::Currency;

    #[test]
    fn test_parse_currency() {
        let currency = Currency::from_str("BTC").unwrap();
        assert_eq!(Currency::BTC, currency);
    }

    #[test]
    fn test_print_currency() {
        let currency = Currency::from_str("BTC").unwrap();
        assert_eq!(Currency::BTC, currency);
    }
}
