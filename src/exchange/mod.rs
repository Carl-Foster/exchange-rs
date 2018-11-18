use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

pub mod error;
pub mod matcher;

pub use self::matcher::{Matcher, Depth, DepthOrder, Direction, Order, OrderMatch};

use self::error::BadContractError;

pub type MatcherResult<T> = Result<T, BadContractError>;

pub struct Exchange {
    matchers: HashMap<i32, Mutex<Matcher>>,
}

impl Exchange {
    pub fn init() -> Exchange {
        let mut matchers = HashMap::new();
        // TODO: Pass in via config
        for i in 1..2 {
            let matcher = Matcher::init_matcher_from_store(i).unwrap_or_else(|| Matcher::new(i));
            matchers.insert(i, Mutex::new(matcher));
        }
        Exchange { matchers }
    }

    pub fn place_order(
        &self,
        new_order: Order,
        contract_id: i32,
    ) -> MatcherResult<Vec<OrderMatch>> {
        self.get_matcher(contract_id)
            .map(|mut matcher| matcher.place_order(new_order))
    }

    pub fn get_orders(&self, contract_id: i32) -> MatcherResult<Vec<Order>> {
        self.get_matcher(contract_id)
            .map(|matcher| matcher.get_orders().clone())
    }

    pub fn get_matches(&self, contract_id: i32) -> MatcherResult<Vec<OrderMatch>> {
        self.get_matcher(contract_id)
            .map(|matcher| matcher.get_matches().clone())
    }

    pub fn get_depth(
        &self,
        contract_id: i32,
        direction: Direction,
    ) -> MatcherResult<Vec<DepthOrder>> {
        self.get_matcher(contract_id)
            .map(|matcher| matcher.get_anonymous_depth(direction))
    }

    fn get_matcher(&self, contract_id: i32) -> MatcherResult<MutexGuard<Matcher>> {
        match self.matchers.get(&contract_id) {
            Some(matcher) => Ok(matcher.lock().unwrap()),
            None => Err(error::BadContractError(contract_id)),
        }
    }
}
