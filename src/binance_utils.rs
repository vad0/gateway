use crate::CurrencyPair;

pub fn symbol(currency_pair: &CurrencyPair) -> String {
    format!("{}{}", currency_pair.base, currency_pair.term)
}
