use orders::{Direction, Order};

#[derive(Debug)]
pub struct Depth {
    direction: Direction,
    orders: Vec<Order>,
}

impl Depth {
    pub fn hydrate(orders: &mut Vec<Order>, direction: Direction) -> Depth {
        orders.sort_by_key(|o| o.price);
        Depth {
            direction,
            orders: orders.to_vec(),
        }
    }

    pub fn match_order(&mut self, new_order: &mut Order) -> Option<Order> {
        while let Some(top_order) = self.get_top_order(new_order.account_id.clone()).next() {
            new_order.match_order(top_order);
        }
        if new_order.quantity > 0 {
            Some(new_order.clone())
        } else {
            None
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
        self.orders.sort_by_key(|o| o.price);
    }

    fn get_top_order(&mut self, caller_account: String) -> impl Iterator<Item = &mut Order> {
        self.orders
            .iter_mut()
            .filter(move |order| order.account_id == *caller_account)
    }

    fn remove_order(&mut self, order: &Order) {
        self.orders.retain(|o| o.id != order.id)
    }
}
