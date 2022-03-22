use serde::Deserialize;
use strum::IntoEnumIterator;

use crate::{L2Increment, Side};

#[derive(Deserialize)]
struct BinanceIncrement {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(skip)]
    symbol: String,
    #[serde(skip)]
    first_update_id: i64,
    #[serde(rename = "u")]
    last_update_id: i64,
    #[serde(rename = "b")]
    bids: Vec<Vec<String>>,
    #[serde(rename = "a")]
    asks: Vec<Vec<String>>
}

impl BinanceIncrement {
    fn get(&self, side: Side) -> &Vec<Vec<String>> {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

fn parse_binance_increment(result: &mut L2Increment, data: &str) -> bool {
    let increment: BinanceIncrement = match serde_json::from_str(data) {
        Ok(increment) => increment,
        Err(_) => return false,
    };
    for side in Side::iter() {
        if !parse_binance_increment_side(side, result, &increment) {
            return false;
        }
    }
    true
}

/// Parses one side of the [`BinanceIncrement`]. If success, then returns `true`, otherwise returns
/// `false`. Output is saved in `holder` argument.
fn parse_binance_increment_side(
    side: Side,
    holder: &mut L2Increment,
    binance_increment: &BinanceIncrement,
) -> bool {
    let result = holder.get_mut(side);
    result.clear();
    let data = binance_increment.get(side);
    for price_level in data {
        if price_level.len() != 2 {
            return false;
        }
        let price: f64 = match price_level[0].parse() {
            Ok(p) => p,
            Err(_) => return false,
        };
        let size: f64 = match price_level[1].parse() {
            Ok(p) => p,
            Err(_) => return false,
        };
        result.add(price, size)
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::binance_increment::parse_binance_increment;
    use crate::Side::Ask;
    use crate::{Bid, L2Increment};

    #[test]
    fn parse_increment_1() {
        let data = r#"{
          "e": "depthUpdate",
          "E": 123456789,
          "s": "BNBBTC",
          "U": 157,
          "u": 160,
          "b": [
            [
              "0.0024",
              "10"
            ]
          ],
          "a": [
            [
              "0.0026",
              "100"
            ]
          ]
        }"#;
        let mut expected_increment = L2Increment::new();
        expected_increment.get_mut(Bid).add(0.0024, 10.0);
        expected_increment.get_mut(Ask).add(0.0026, 100.0);

        compare_increments(data, &mut expected_increment);
    }

    #[test]
    fn parse_increment_2() {
        let data = r#"{
          "e": "depthUpdate",
          "E": 1568223206826,
          "s": "ETHBTC",
          "U": 774645960,
          "u": 774645960,
          "b": [
            [
              "0.01756400",
              "3.43100000"
            ]
          ],
          "a": []
        }"#;
        let mut expected_increment = L2Increment::new();
        expected_increment.get_mut(Bid).add(0.017564, 3.431);

        compare_increments(data, &mut expected_increment);
    }

    fn compare_increments(data: &str, expected_increment: &L2Increment) {
        let holder = &mut L2Increment::new();
        let success = parse_binance_increment(holder, data);
        assert!(success);
        assert_eq!(expected_increment, holder);
    }
}
