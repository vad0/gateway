use base::Side::Bid;
use currencies::CurrencyPair;

mod base;
mod binance_increment;
mod binance_security_list;
mod binance_snapshot;
mod binance_utils;
mod currencies;

#[tokio::main]
async fn main() {
    // binance_snapshot::receive_snapshot().await;
    // binance_increment::listen_increments().await;
    binance_security_list::request_security_list().await;
}
