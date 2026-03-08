use rust_decimal::Decimal;
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

    pub fn apply_update(
        &mut self,
        is_snapshot: bool,
        bids: Vec<(String, String)>,
        asks: Vec<(String, String)>,
    ) {
        if is_snapshot {
            self.bids.clear();
            self.asks.clear();
        }

        Self::update_map(&mut self.bids, bids);
        Self::update_map(&mut self.asks, asks);
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
        let best_ask = self.best_ask()?.0;
        let best_bid = self.best_bid()?.0;
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
