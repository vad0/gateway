use std::time::SystemTime;

use futures_util::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::base::{L2Update, Side};
use crate::binance_utils;
use crate::binance_utils::BinanceUpdate;
use crate::currencies::CurrencyPair;

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

impl BinanceUpdate for BinanceIncrement {
    fn get(&self, side: Side) -> &Vec<Vec<String>> {
        match side {
            Side::Bid => &self.bids,
            Side::Ask => &self.asks,
        }
    }
}

fn parse_binance_increment(result: &mut L2Update, data: &str) -> bool {
    let start = SystemTime::now();
    let increment: BinanceIncrement = match serde_json::from_str(data) {
        Ok(increment) => increment,
        Err(_) => return false,
    };
    let success =
        Side::iter().all(|side| binance_utils::parse_binance_update_side(side, result, &increment));
    match start.elapsed() {
        Ok(elapsed) => println!("Increment parsing time: {}", elapsed.as_micros()),
        Err(e) => println!("Error: {}", e),
    }
    success
}

pub async fn listen_increments() -> tungstenite::Result<()> {
    let increment_address = "wss://stream.binance.com:9443/ws/bnbbtc@depth";
    let url =
        Url::parse(increment_address).expect(format!("Can't parse {}", increment_address).as_str());
    let (mut socket, _) = connect_async(url)
        .await
        .expect(format!("Can't connect to {}", increment_address).as_str());
    let mut increment = L2Update::new();
    while let Some(msg) = socket.next().await {
        parse_binance_increment(&mut increment, msg.unwrap().to_string().as_str());
        println!("{:?}", increment)
    }
    Ok(())
}

#[derive(Serialize)]
struct BinanceMdRequest {
    method: String,
    topic: String,
    symbols: Vec<String>,
}

impl BinanceMdRequest {
    fn subscribe(currency_pairs: Vec<CurrencyPair>) -> String {
        let request = BinanceMdRequest {
            method: "subscribe".to_string(),
            topic: "marketDepth".to_string(),
            symbols: currency_pairs
                .iter()
                .map(crate::binance_utils::symbol)
                .collect(),
        };
        serde_json::to_string(&request).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::base::L2Update;
    use crate::base::Side::Ask;
    use crate::binance_increment::parse_binance_increment;
    use crate::Bid;

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
        let mut expected_increment = L2Update::new();
        expected_increment.get_mut(Bid).add(0.0024, 10.0);
        expected_increment.get_mut(Ask).add(0.0026, 100.0);

        compare_updates(data, &expected_increment);
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
        let mut expected_increment = L2Update::new();
        expected_increment.get_mut(Bid).add(0.017564, 3.431);

        compare_updates(data, &expected_increment);
    }

    fn compare_updates(data: &str, expected_update: &L2Update) {
        let holder = &mut L2Update::new();
        let success = parse_binance_increment(holder, data);
        assert!(success);
        assert_eq!(expected_update, holder);
    }
}
