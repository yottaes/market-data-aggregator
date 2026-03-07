use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use std::{collections::BTreeMap, str::FromStr};
pub struct Cup {
    pub bids: BTreeMap<Decimal, Decimal>,
    pub asks: BTreeMap<Decimal, Decimal>,
}

impl Cup {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn apply(&mut self, order: OrderBook) {
        if order.order_type == "snapshot" {
            self.bids.clear();
            self.asks.clear();

            Self::update_map(&mut self.bids, order.data.bids);
            Self::update_map(&mut self.asks, order.data.asks);

            return;
        }

        if order.order_type == "delta" {
            Self::update_map(&mut self.bids, order.data.bids);
            Self::update_map(&mut self.asks, order.data.asks);
        }
    }

    fn update_map(map: &mut BTreeMap<Decimal, Decimal>, source: Vec<(String, String)>) {
        for rec in source {
            let key = Decimal::from_str(rec.0.as_str()).unwrap();
            let val = Decimal::from_str(rec.1.as_str()).unwrap();
            if val.is_zero() {
                let _ = map.remove(&key);
                continue;
            }
            map.insert(key, val);
        }
    }
    pub fn get_spread(&self) -> Option<Decimal> {
        // .first_key_value() и .last_key_value() доступны в Rust 1.66+
        let best_ask = self.best_ask()?.0; // Самая низкая цена продажи
        let best_bid = self.best_bid()?.0; // Самая высокая цена покупки

        Some(best_ask - best_bid)
    }

    pub fn best_ask(&self) -> Option<(&Decimal, &Decimal)> {
        self.asks.first_key_value()
    }

    pub fn best_bid(&self) -> Option<(&Decimal, &Decimal)> {
        self.bids.last_key_value()
    }
}

impl Default for Cup {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Serialize, Debug, Display)]
#[display("{}", serde_json::to_string(self).unwrap())]
pub struct OrderBook {
    pub topic: String,
    pub ts: u64,
    #[serde(rename = "type")]
    pub order_type: String,
    pub data: OrderBookData,
    pub cts: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderBookData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
    #[serde(rename = "u")]
    pub update_id: u64,
    pub seq: u64,
}
