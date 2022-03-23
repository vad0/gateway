use enum_map::{enum_map, Enum, EnumMap};
use reqwest::Client;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

use crate::base::{L2Update, Side};

pub async fn receive_snapshot() -> Result<(), reqwest::Error> {
    let snapshot_address = "https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=1000";
    let client = Client::builder()
        .build()
        .expect("Unable to create websocket client");
    let res = client.get(snapshot_address).send().await?;
    let snapshot = res.text().await?;
    println!("{}", snapshot);
    tungstenite::Result::Ok(())
}

#[derive(Deserialize)]
struct BinanceSnapshot {
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

impl BinanceSnapshot {
    fn get(&self, side: Side) -> &Vec<Vec<String>> {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

fn parse_binance_snapshot(result: &mut L2Update, data: &str) -> bool {
    let increment: BinanceSnapshot = match serde_json::from_str(data) {
        Ok(snapshot) => snapshot,
        Err(_) => return false,
    };
    Side::iter().all(|side| parse_binance_snapshot_side(side, result, &increment))
}

fn parse_binance_snapshot_side(
    side: Side,
    holder: &mut L2Update,
    binance_snapshot: &BinanceSnapshot,
) -> bool {
    let result = holder.get_mut(side);
    result.clear();
    let data = binance_snapshot.get(side);
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

#[cfg(test)]
mod tests {
    use crate::base::L2Update;
    use crate::base::Side::Ask;
    use crate::binance_snapshot::parse_binance_snapshot;
    use crate::Bid;

    #[test]
    fn parse_snapshot() {
        let data = r#"{
          "lastUpdateId": 441791238,
          "bids": [
            [
              "0.00216090",
              "22.67000000"
            ],
            [
              "0.00216080",
              "29.43000000"
            ]
          ],
          "asks": [
            [
              "0.00216100",
              "6.87000000"
            ],
            [
              "0.00216150",
              "15.24000000"
            ]
          ]
        }"#;
        let mut expected_snapshot = L2Update::new();
        expected_snapshot
            .get_mut(Bid)
            .add(0.00216090, 22.67)
            .add(0.0021608, 29.43);
        expected_snapshot
            .get_mut(Ask)
            .add(0.002161, 6.87)
            .add(0.0021615, 15.24);

        compare_updates(data, &expected_snapshot);
    }

    fn compare_updates(data: &str, expected_update: &L2Update) {
        let holder = &mut L2Update::new();
        let success = parse_binance_snapshot(holder, data);
        assert!(success);
        assert_eq!(expected_update, holder);
    }
}
