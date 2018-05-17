mod orders;

use self::orders::{Direction, Order};

#[derive(Clone, Debug)]
struct Depth {
    direction: Direction,
    orders: Vec<Order>,
    contract_id: String,
}

impl Depth {
    pub fn hydrate(orders: &mut Vec<Order>, contract_id: &str, direction: Direction) -> Depth {
        orders.sort_by_key(|o| o.price);
        Depth {
            direction,
            orders: orders.to_vec(),
            contract_id: contract_id.to_owned(),
        }
    }

    fn get_top_order(&self, caller_account: String) -> Option<&Order> {
        match self.orders
            .binary_search_by(|probe| probe.account_id.cmp(&caller_account))
        {
            Ok(index) => self.orders.get(index),
            Err(_) => None,
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
        self.orders.sort_by_key(|o| o.price);
    }
}
