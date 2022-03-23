use crate::base::{L2Update, Side};
use crate::CurrencyPair;

pub fn symbol(currency_pair: &CurrencyPair) -> String {
    format!("{}{}", currency_pair.base, currency_pair.term)
}

pub trait BinanceUpdate {
    fn get(&self, side: Side) -> &Vec<Vec<String>>;
}

/// Parses one side of the [`BinanceUpdate`]. If success, then returns
/// `true`, otherwise returns `false`. Output is saved in `holder` argument.
pub fn parse_binance_update_side(
    side: Side,
    holder: &mut L2Update,
    binance_update: &dyn BinanceUpdate,
) -> bool {
    let result = holder.get_mut(side);
    result.clear();
    let data = binance_update.get(side);
    for price_level in data {
        if price_level.len() != 2 {
            return false;
        }
        let price: f64 = match price_level[0].parse() {
            Ok(p) => p,
            Err(_) => return false,
        };
        let size: f64 = match price_level[1].parse() {
            Ok(s) => s,
            Err(_) => return false,
        };
        result.add(price, size);
    }
    return true;
}
