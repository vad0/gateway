use futures_util::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::base::{L2Increment, Side};
use crate::CurrencyPair;

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

/// Parses one side of the [`BinanceIncrement`]. If success, then returns
/// `true`, otherwise returns `false`. Output is saved in `holder` argument.
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
    use crate::base::L2Increment;
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

async fn listen_increments() -> tungstenite::Result<()> {
    let increment_address = "wss://stream.binance.com:9443/ws/bnbbtc@depth";
    let snapshot_address = "https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=1000";
    let url =
        Url::parse(increment_address).expect(format!("Can't parse {}", increment_address).as_str());
    let (mut socket, response) = connect_async(url)
        .await
        .expect(format!("Can't connect to {}", increment_address).as_str());
    // let pair = CurrencyPair { base: Currency::BNB, term: Currency::BTC };
    // let subscription = BinanceMdRequest::subscribe(vec![pair]);
    // socket.send(Message::text(subscription.clone())).await?;
    // println!("sent {}", subscription);
    while let Some(msg) = socket.next().await {
        println!("{}", msg.unwrap())
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
