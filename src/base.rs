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

#[derive(Display)]
pub enum Currency {
    BNB,
    BTC,
    ETH,
}

pub struct CurrencyPair {
    pub base: Currency,
    pub term: Currency,
}

#[derive(Debug, Enum, EnumIter, Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(PartialEq, Debug)]
struct PriceLevel {
    price: f64,
    size: f64,
}

#[derive(PartialEq, Debug)]
pub struct L2Side {
    levels: Vec<PriceLevel>,
}

impl L2Side {
    fn new() -> L2Side {
        L2Side { levels: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.levels.clear()
    }

    pub fn add(&mut self, price: f64, size: f64) {
        self.levels.push(PriceLevel { price, size })
    }
}

#[derive(PartialEq, Debug)]
struct SideMap<V> {
    sides: EnumMap<Side, V>,
}

#[derive(PartialEq, Debug)]
pub struct L2Increment {
    side_map: SideMap<L2Side>,
}

impl L2Increment {
    pub fn new() -> L2Increment {
        L2Increment {
            side_map: SideMap {
                sides: enum_map! {
                    Side::Bid=>L2Side::new(),
                    Side::Ask=>L2Side::new(),
                },
            },
        }
    }

    fn get(&self, side: Side) -> &L2Side {
        &self.side_map.sides[side]
    }

    pub fn get_mut(&mut self, side: Side) -> &mut L2Side {
        &mut self.side_map.sides[side]
    }
}
