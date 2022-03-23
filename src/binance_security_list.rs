use std::str::FromStr;

use reqwest::Client;
use serde::Deserialize;
use serde_json::Map;

use crate::base::{InstrumentSettings, SecurityList};
use crate::binance_utils::HTTP_ADDRESS;
use crate::currencies::Currency;
use crate::CurrencyPair;

pub async fn request_security_list() -> Result<(), String> {
    let address = HTTP_ADDRESS.to_owned() + "exchangeInfo";
    let client = Client::builder().build().expect("Failed to create client");
    let res = client
        .get(address)
        .send()
        .await
        .expect("Failed to send request");
    let message = res.text().await.expect("Failed to receive response");
    let base_list: BinanceSecurityList =
        serde_json::from_str(message.as_str()).expect("Failed to parse security list");
    // println!("{:?}", security_list);
    let security_list = get_security_list(&base_list);
    println!("{:?}", security_list);
    tungstenite::Result::Ok(())
}

fn get_security_list(base_list: &BinanceSecurityList) -> SecurityList {
    let mut res = SecurityList::new();
    for symbol in &base_list.symbols {
        let base_currency = match Currency::from_str(&symbol.base_asset) {
            Ok(currency) => currency,
            Err(_) => continue,
        };
        let term_currency = match Currency::from_str(&symbol.quote_asset) {
            Ok(currency) => currency,
            Err(_) => continue,
        };
        let step_lot = get_lot(symbol.base_asset_precision);
        let price_step = get_lot(symbol.quote_asset_precision);
        let settings = InstrumentSettings::new(
            CurrencyPair::new(base_currency, term_currency),
            step_lot as f64,
            price_step as f64,
        );
        res.add(settings);
    }
    res
}

#[derive(Deserialize, Debug)]
struct BinanceSecurityList {
    timezone: String,
    #[serde(rename = "serverTime")]
    server_time: u64,
    #[serde(rename = "rateLimits")]
    rate_limits: Vec<RateLimit>,
    #[serde(rename = "exchangeFilters")]
    exchange_filters: Vec<String>,
    symbols: Vec<BinanceSymbol>,
}

#[derive(Deserialize, Debug)]
struct RateLimit {
    #[serde(rename = "rateLimitType")]
    rate_limit_type: String,
    interval: String,
    #[serde(rename = "intervalNum")]
    interval_num: u32,
    limit: u32,
}

#[derive(Deserialize, Debug)]
struct BinanceSymbol {
    symbol: String,
    status: String,
    #[serde(rename = "baseAsset")]
    base_asset: String,
    #[serde(rename = "baseAssetPrecision")]
    base_asset_precision: i32,
    #[serde(rename = "quoteAsset")]
    quote_asset: String,
    #[serde(rename = "quotePrecision")]
    quote_precision: i32,
    #[serde(rename = "quoteAssetPrecision")]
    quote_asset_precision: i32,
    #[serde(rename = "orderTypes")]
    order_types: Vec<String>,
    #[serde(rename = "icebergAllowed")]
    iceberg_allowed: bool,
    #[serde(rename = "ocoAllowed")]
    oco_allowed: bool,
    #[serde(rename = "quoteOrderQtyMarketAllowed")]
    quote_order_qty_market_allowed: bool,
    #[serde(rename = "allowTrailingStop")]
    allow_trailing_stop: bool,
    #[serde(rename = "isSpotTradingAllowed")]
    is_spot_trading_allowed: bool,
    #[serde(rename = "isMarginTradingAllowed")]
    is_margin_trading_allowed: bool,
    filters: Vec<Map<String, serde_json::Value>>,
    permissions: Vec<String>,
}

fn get_lot(precision: i32) -> f64 {
    let base: f64 = 10.0;
    base.powi(-precision)
}

#[cfg(test)]
mod tests {
    use crate::binance_security_list::get_lot;

    #[test]
    fn test_precision_value() {
        let calculated = get_lot(8);
        let expected = 1e-8;
        assert_eq!(expected, calculated);
    }
}
