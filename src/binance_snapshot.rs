use std::time::{Duration, SystemTime};

use enum_map::{enum_map, Enum, EnumMap};
use reqwest::Client;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

use crate::base::{L2Update, Side};
use crate::binance_utils::{parse_binance_update_side, BinanceUpdate};

pub async fn receive_snapshot() -> Result<(), reqwest::Error> {
    let snapshot_address = "https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=1000";
    let client = Client::builder()
        .build()
        .expect("Unable to create websocket client");
    let res = client.get(snapshot_address).send().await?;
    let message = res.text().await?;
    let mut snapshot = L2Update::new();
    parse_binance_snapshot(&mut snapshot, message.as_str());
    println!("{:?}", snapshot);
    tungstenite::Result::Ok(())
}

#[derive(Deserialize)]
struct BinanceSnapshot {
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

impl BinanceUpdate for BinanceSnapshot {
    fn get(&self, side: Side) -> &Vec<Vec<String>> {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

fn parse_binance_snapshot(result: &mut L2Update, data: &str) -> bool {
    let start = SystemTime::now();
    let snapshot: BinanceSnapshot = match serde_json::from_str(data) {
        Ok(snapshot) => snapshot,
        Err(_) => return false,
    };
    let success = Side::iter().all(|side| parse_binance_update_side(side, result, &snapshot));
    match start.elapsed() {
        Ok(elapsed) => println!("Snapshot parsing time: {}", elapsed.as_micros()),
        Err(e) => println!("Error: {}", e),
    }
    success
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
