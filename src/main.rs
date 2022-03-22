use enum_map::{enum_map, Enum, EnumMap};
use strum_macros::EnumIter;

use crate::Side::Bid;

mod binance_increment;
mod binance_snapshot;

fn main() {
    println!("Hello world!");
}

#[derive(Debug, Enum, EnumIter, Clone, Copy)]
enum Side {
    Bid,
    Ask,
}

#[derive(PartialEq, Debug)]
struct PriceLevel {
    price: f64,
    size: f64,
}

#[derive(PartialEq, Debug)]
struct L2Side {
    levels: Vec<PriceLevel>,
}

impl L2Side {
    fn new() -> L2Side {
        L2Side { levels: Vec::new() }
    }

    fn clear(&mut self) {
        self.levels.clear()
    }

    fn add(&mut self, price: f64, size: f64) {
        self.levels.push(PriceLevel { price, size })
    }
}

#[derive(PartialEq, Debug)]
struct SideMap<V> {
    sides: EnumMap<Side, V>,
}

#[derive(PartialEq, Debug)]
struct L2Increment {
    side_map: SideMap<L2Side>,
}

impl L2Increment {
    fn new() -> L2Increment {
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

    fn get_mut(&mut self, side: Side) -> &mut L2Side {
        &mut self.side_map.sides[side]
    }
}
