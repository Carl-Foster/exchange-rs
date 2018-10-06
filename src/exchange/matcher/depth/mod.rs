use super::order_match::OrderMatch;
use super::orders::{Direction, Order};

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    direction: Direction,
    orders: Vec<Order>,
}

impl Depth {
    pub fn hydrate(orders: Vec<Order>, direction: Direction) -> Depth {
        for order in &orders {
            assert_eq!(
                direction, order.direction,
                "Depth trying to hydrate orders with wrong direction"
            );
        }
        let mut depth = Depth { direction, orders };
        depth.sort_orders();
        depth
    }

    pub fn match_order(&mut self, new_order: &mut Order) -> Vec<OrderMatch> {
        let mut order_matches: Vec<OrderMatch> = Vec::new();
        let matchable_orders = self.get_matchable_orders(new_order.account_id.clone());
        for top_order in matchable_orders {
            if new_order.quantity == 0 {
                break;
            }
            if let Some(order_match) = OrderMatch::match_orders(new_order, top_order) {
                new_order.update_remaining(order_match.quantity_matched);
                top_order.update_remaining(order_match.quantity_matched);
                order_matches.push(order_match);
            }
        }
        order_matches
    }

    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }

    pub fn add_order(&mut self, order: Order) {
        assert_eq!(
            self.direction, order.direction,
            "Depth tried to add order of wrong direction"
        );
        self.orders.push(order);
        self.sort_orders();
    }

    pub fn flush_filled_orders(&mut self) {
        self.orders.retain(|o| o.quantity > 0);
    }

    fn get_matchable_orders(&mut self, caller_account: String) -> impl Iterator<Item = &mut Order> {
        self.orders
            .iter_mut()
            .filter(move |order| order.account_id != caller_account)
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
        let empty_orders: Vec<Order> = Vec::new();
        let depth = Depth::hydrate(empty_orders, Direction::Buy);
        assert!(depth.orders.is_empty());
    }

    #[test]
    fn hydrate_sorts_buy_with_highest_price_first() {
        let mut orders: Vec<Order> = Vec::new();
        orders.push(Order::new(100, 10, "account1", Direction::Buy));
        orders.push(Order::new(120, 10, "account1", Direction::Buy));
        orders.push(Order::new(110, 10, "account1", Direction::Buy));
        let depth = Depth::hydrate(orders, Direction::Buy);

        assert!(!depth.orders.is_empty());
        if let Some(top_order) = depth.orders.first() {
            assert_eq!(120, top_order.price);
        } else {
            panic!();
        }
    }

    #[test]
    fn hydrate_sorts_sell_with_lowest_order_first() {
        let mut orders: Vec<Order> = Vec::new();
        orders.push(Order::new(100, 10, "account1", Direction::Sell));
        orders.push(Order::new(120, 10, "account1", Direction::Sell));
        orders.push(Order::new(110, 10, "account1", Direction::Sell));
        let depth = Depth::hydrate(orders, Direction::Sell);

        assert!(!depth.orders.is_empty());
        if let Some(top_order) = depth.orders.first() {
            assert_eq!(100, top_order.price);
        } else {
            panic!();
        }
    }
}
