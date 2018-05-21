use std::collections::HashMap;

use matcher::Matcher;
use order_match::OrderMatch;
use orders::Order;

pub struct Exchange {
    matchers: HashMap<u32, Matcher>,
}

impl Exchange {
    pub fn init() -> Exchange {
        let mut matchers = HashMap::new();
        // TODO: Pass in via config
        for i in 1..5 {
            matchers.insert(i, Matcher::new(Vec::new(), i));
        }
        Exchange { matchers }
    }

    pub fn place_order(
        &mut self,
        new_order: Order,
        contract_id: u32,
    ) -> Result<Vec<OrderMatch>, String> {
        if let Some(matcher) = self.matchers.get_mut(&contract_id) {
            let matches = matcher.place_order(new_order);
            // TODO: save all state (order, depth, ordermatches)
            Ok(matches)
        } else {
            Err(format!("Invalid contract_id {}", contract_id))
        }
    }

    pub fn get_orders(&self, contract_id: u32) -> Result<Vec<Order>, String> {
        if let Some(matcher) = self.matchers.get(&contract_id) {
            Ok(matcher.get_orders())
        } else {
            Err(format!("Invalid contract_id {}", contract_id))
        }
    }
}
