use orders::{Direction, Order};

#[derive(Debug)]
pub struct Depth {
    direction: Direction,
    orders: Vec<Order>,
}

impl Depth {
    pub fn hydrate(orders: &mut Vec<Order>, direction: Direction) -> Depth {
        let mut depth = Depth {
            direction,
            orders: orders.to_vec(),
        };
        depth.sort_orders();
        depth
    }

    pub fn match_order(&mut self, new_order: &mut Order) -> Option<Order> {
        while let Some(top_order) = self.get_valid_orders(new_order.account_id.clone()).next() {
            new_order.match_order(top_order);
            if new_order.quantity == 0 {
                return None;
            }
        }
        if new_order.quantity > 0 {
            Some(new_order.clone())
        } else {
            None
        }
    }

    pub fn add_order(&mut self, order: Order) {
        assert_eq!(
            self.direction, order.direction,
            "Depth tried to add order of wrong direction"
        );
        self.orders.push(order);
        self.sort_orders();
    }

    fn get_valid_orders(&mut self, caller_account: String) -> impl Iterator<Item = &mut Order> {
        self.orders
            .iter_mut()
            .filter(move |order| order.account_id != *caller_account)
    }

    fn remove_order(&mut self, order: &Order) {
        self.orders.retain(|o| o.id != order.id)
    }

    fn sort_orders(&mut self) {
        self.orders.sort_by_key(|o| o.price);
        if self.direction == Direction::Buy {
            self.orders.reverse();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hydrate_with_no_orders_is_fine() {
        let mut empty_orders: Vec<Order> = Vec::new();
        let depth = Depth::hydrate(&mut empty_orders, Direction::Buy);
        assert!(depth.orders.is_empty());
    }

    #[test]
    fn hydrate_has_sorted_orders() {
        let mut orders: Vec<Order> = Vec::new();
        orders.push(Order::new(120, 10, "account1", Direction::Buy));
        orders.push(Order::new(100, 10, "account1", Direction::Buy));
        orders.push(Order::new(110, 10, "account1", Direction::Buy));
        let mut depth = Depth::hydrate(&mut orders, Direction::Buy);

        let valid_orders: Vec<&mut Order> =
            depth.get_valid_orders("account2".to_string()).collect();
        assert!(!valid_orders.is_empty());
        if let Some(top_order) = valid_orders.first() {
            assert_eq!(120, top_order.price);
        } else {
            panic!();
        }
    }
}
