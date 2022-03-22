#[cfg(test)]
mod tests {
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

    }
}
use reqwest::Client;

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
