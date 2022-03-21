use std::num::ParseFloatError;

use enum_map::{Enum, enum_map, EnumMap};
use serde::Deserialize;
use serde_json::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::Side::Bid;

mod binance_increment;

fn main() {
    println!("Hello world!");
}

#[derive(Debug, Enum, EnumIter, Clone, Copy)]
enum Side {
    Bid,
    Ask,
}

#[derive(PartialEq, Debug)]
struct PriceLevel {
    price: f64,
    size: f64,
}

#[derive(PartialEq, Debug)]
struct L2Side {
    levels: Vec<PriceLevel>,
}

impl L2Side {
    fn new() -> L2Side {
        L2Side { levels: Vec::new() }
    }
    fn clear(&mut self) {
        self.levels.clear()
    }

    fn add(&mut self, price: f64, size: f64) {
        self.levels.push(PriceLevel { price, size })
    }
}

#[derive(PartialEq, Debug)]
struct SideMap<V> {
    sides: EnumMap<Side, V>,
}

#[derive(PartialEq, Debug)]
struct L2Increment {
    side_map: SideMap<L2Side>,
}

impl L2Increment {
    fn new() -> L2Increment {
        L2Increment {
            side_map: SideMap {
                sides: enum_map! {
                    Side::Bid=>L2Side::new(),
                    Side::Ask=>L2Side::new(),
                }
            }
        }
    }

    fn get(&self, side: Side) -> &L2Side {
        &self.side_map.sides[side]
    }

    fn get_mut(&mut self, side: Side) -> &mut L2Side {
        &mut self.side_map.sides[side]
    }
}

fn parse_binance_increment_side(side: Side, increment: &mut L2Increment, binance_increment: &BinanceIncrement) -> bool {
    let result = increment.get_mut(side);
    result.clear();
    let data = binance_increment.get(side);
    for price_level in data {
        if price_level.len() != 2 {
            return false;
        }
        let price_result: Result<f64, ParseFloatError> = price_level[0].parse();
        if price_result.is_err() {
            return false;
        }
        let size_result: Result<f64, ParseFloatError> = price_level[1].parse();
        if size_result.is_err() {
            return false;
        }
        result.add(price_result.unwrap(), size_result.unwrap())
    }
    return true;
}

fn parse_binance_increment<'a>(result: &'a mut L2Increment, data: &str) -> Option<&'a L2Increment> {
    let inc_result: Result<BinanceIncrement, Error> = serde_json::from_str(data);
    if inc_result.is_err() {
        return None;
    }
    let increment = inc_result.unwrap();
    for side in Side::iter() {
        if !parse_binance_increment_side(side, result, &increment) {
            return None;
        }
    }
    Some(result)
}

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
    asks: Vec<Vec<String>>,
}

impl BinanceIncrement {
    fn get(&self, side: Side) -> &Vec<Vec<String>> {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bid, L2Increment, parse_binance_increment};
    use crate::Side::Ask;

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
        let parsed_increment = parse_binance_increment(holder, data).unwrap();
        assert_eq!(expected_increment, parsed_increment);
    }
}