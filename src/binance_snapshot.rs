pub async fn receive_snapshot() -> Result<(), reqwest::Error> {
    let snapshot_address = "https://api.binance.com/api/v3/depth?symbol=BNBBTC&limit=1000";
    let res = reqwest::get(snapshot_address).await?;
    let snapshot = res.text().await?;
    println!("{}", snapshot);
    tungstenite::Result::Ok(())
}
