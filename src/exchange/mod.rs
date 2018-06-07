use std::collections::HashMap;
use std::sync::Mutex;

pub mod matcher;

use self::matcher::{Matcher, Order, OrderMatch};

pub struct Exchange {
    matchers: HashMap<i32, Mutex<Matcher>>,
    orders: Vec<Order>,
    order_matches: Vec<OrderMatch>,
}

impl Exchange {
    pub fn init() -> Exchange {
        let mut matchers = HashMap::new();
        // TODO: Pass in via config
        for i in 1..5 {
            matchers.insert(i, Mutex::new(Matcher::new(Vec::new(), i)));
        }
        Exchange {
            matchers,
            orders: Vec::new(),
            order_matches: Vec::new(),
        }
    }

    pub fn place_order(&self, new_order: Order) -> Result<Vec<OrderMatch>, String> {
        if let Some(matcher) = self.matchers.get(&new_order.contract_id) {
            let matches = { matcher.lock().unwrap().place_order(new_order.clone()) };
            Ok(matches)
        } else {
            Err(format!("Invalid contract_id {}", &new_order.contract_id))
        }
    }

    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }

    pub fn get_matches(&self) -> &Vec<OrderMatch> {
        &self.order_matches
    }

    pub fn get_depth(&self, contract_id: i32) -> Result<Vec<Order>, String> {
        if let Some(matcher) = self.matchers.get(&contract_id) {
            Ok(matcher.lock().unwrap().get_depth())
        } else {
            Err(format!("Invalid contract_id {}", contract_id))
        }
    }
}
