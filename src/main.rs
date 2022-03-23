use base::CurrencyPair;
use base::Side::Bid;
use enum_map::{enum_map, Enum, EnumMap};
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use serde::Serialize;
use strum_macros::Display;
use strum_macros::EnumIter;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::frame::coding::CloseCode::Error;
use tungstenite::Message;
use url::Url;

mod base;
mod binance_increment;
mod binance_snapshot;
mod binance_utils;

#[tokio::main]
async fn main() {
    // binance_snapshot::receive_snapshot().await;
    binance_increment::listen_increments().await;
}
