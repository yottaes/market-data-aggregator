use std::collections::HashMap;

use crate::model::{cup::Cup, normalized_update::NormalizedUpdate};

pub struct ExchangeBooks {
    pub cups: HashMap<String, Cup>,
}

impl ExchangeBooks {
    pub fn new() -> Self {
        ExchangeBooks {
            cups: HashMap::new(),
        }
    }

    pub fn apply_update(&mut self, order: NormalizedUpdate) {
        let cup = self.cups.entry(order.symbol.clone()).or_default();
        cup.apply_update(order.is_snapshot, order.bids, order.asks);
    }

    pub fn best_bid(&self, symbol: &str) -> Option<String> {
        self.cups
            .get(symbol)?
            .best_bid()
            .map(|(price, _)| price.to_string())
    }

    pub fn best_ask(&self, symbol: &str) -> Option<String> {
        self.cups
            .get(symbol)?
            .best_ask()
            .map(|(price, _)| price.to_string())
    }
}

// impl TryFrom<Vec<String>> for ExchangeBooks {
//     type Error = anyhow::Error;
//
//     fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {}
// }

impl Default for ExchangeBooks {
    fn default() -> Self {
        Self::new()
    }
}
