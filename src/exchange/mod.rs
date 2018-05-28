use std::collections::HashMap;
use std::sync::Mutex;

pub mod matcher;
mod store;

use self::matcher::{Matcher, Order, OrderMatch};

pub struct Exchange {
    matchers: HashMap<u32, Mutex<Matcher>>,
}

impl Exchange {
    pub fn init() -> Exchange {
        let mut matchers = HashMap::new();
        // TODO: Pass in via config
        for i in 1..5 {
            matchers.insert(i, Mutex::new(Matcher::new(Vec::new(), i)));
        }
        Exchange { matchers }
    }

    pub fn place_order(
        &self,
        new_order: Order,
        contract_id: u32,
    ) -> Result<Vec<OrderMatch>, String> {
        if let Some(matcher) = self.matchers.get(&contract_id) {
            let matches = matcher.lock().unwrap().place_order(new_order);
            // TODO: save all state (order, depth, ordermatches)
            Ok(matches)
        } else {
            Err(format!("Invalid contract_id {}", contract_id))
        }
    }

    pub fn get_depth(&self, contract_id: u32) -> Result<Vec<Order>, String> {
        if let Some(matcher) = self.matchers.get(&contract_id) {
            Ok(matcher.lock().unwrap().get_depth())
        } else {
            Err(format!("Invalid contract_id {}", contract_id))
        }
    }
}
