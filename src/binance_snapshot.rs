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
