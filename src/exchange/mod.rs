use std::collections::HashMap;
use std::sync::Mutex;

use matcher::Matcher;
use order_match::OrderMatch;
use orders::Order;

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

    // TODO: Get orders from DB to not lock exchange
    pub fn get_orders(&self, contract_id: u32) -> Result<Vec<Order>, String> {
        if let Some(matcher) = self.matchers.get(&contract_id) {
            Ok(matcher.lock().unwrap().get_orders())
        } else {
            Err(format!("Invalid contract_id {}", contract_id))
        }
    }
}
