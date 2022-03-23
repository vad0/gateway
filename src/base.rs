use std::array::IntoIter;
use std::collections::HashMap;

use enum_map::{enum_map, Enum, EnumMap};
use strum_macros::Display as enum_display;

use crate::currencies::CurrencyPair;

#[derive(Debug, Enum, Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
}

impl Side {
    pub fn iter() -> IntoIter<Side, 2> {
        [Side::Bid, Side::Ask].into_iter()
    }
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

    pub fn add(&mut self, price: f64, size: f64) -> &mut Self {
        self.levels.push(PriceLevel { price, size });
        self
    }
}

#[derive(PartialEq, Debug)]
struct SideMap<V> {
    sides: EnumMap<Side, V>,
}

/// One data structure can be used for both snapshot and increment
#[derive(PartialEq, Debug)]
pub struct L2Update {
    side_map: SideMap<L2Side>,
}

impl L2Update {
    pub fn new() -> L2Update {
        L2Update {
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

#[derive(Debug)]
pub struct InstrumentSettings {
    currency_pair: CurrencyPair,
    step_lot: f64,
    price_step: f64,
}

impl InstrumentSettings {
    pub fn new(currency_pair: CurrencyPair, step_lot: f64, price_step: f64) -> InstrumentSettings {
        InstrumentSettings {
            currency_pair,
            step_lot,
            price_step,
        }
    }
}

#[derive(Debug)]
pub struct SecurityList {
    symbols: HashMap<CurrencyPair, InstrumentSettings>,
}

impl SecurityList {
    pub fn new() -> SecurityList {
        SecurityList {
            symbols: HashMap::new(),
        }
    }

    pub fn add(&mut self, instrument_settings: InstrumentSettings) {
        self.symbols.insert(
            instrument_settings.currency_pair.clone(),
            instrument_settings,
        );
    }
}
