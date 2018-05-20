use chrono::prelude::{DateTime, Utc};
use order_match::OrderMatch;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: String,
    pub price: u32,
    pub quantity: u32,
    pub account_id: String,
    pub direction: Direction,
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub fn new(price: u32, quantity: u32, account_id: &str, direction: Direction) -> Order {
        Order {
            price,
            quantity,
            direction,
            account_id: account_id.to_string(),
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }

    pub fn update_remaining(&mut self, matched_quantity: u32) {
        assert!(
            self.quantity >= matched_quantity,
            "Quantity to remove is greater than order's current quantity"
        );
        self.quantity = self.quantity - matched_quantity;
    }

    pub fn match_with(&self, top_order: &Order) -> Option<OrderMatch> {
        assert_ne!(
            self.account_id, top_order.account_id,
            "Buy Order {} and Sell Order {} tried to match with same account_id",
            self.id, top_order.id
        );
        assert_ne!(
            self.direction, top_order.direction,
            "Buy Order {} and Sell Order {} tried to match with same direction",
            self.id, top_order.id
        );
        if self.quantity == 0 || !self.did_match(top_order) {
            None
        } else {
            Some(OrderMatch::new(self, top_order))
        }
    }

    fn did_match(&self, other: &Order) -> bool {
        match self.direction {
            Direction::Buy => self.price >= other.price,
            Direction::Sell => self.price <= other.price,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quantity_updates_with_valid_matched_quantity() {
        let mut order = Order::new(10, 10, "account1", Direction::Buy);
        order.update_remaining(5);
        assert_eq!(10 - 5, order.quantity);
    }

    #[test]
    #[should_panic(expected = "Quantity to remove is greater than")]
    fn update_remaining_panics_with_invalid_removal() {
        let mut order = Order::new(10, 10, "account", Direction::Buy);
        order.update_remaining(20);
    }

    #[test]
    fn low_buy_does_not_match_high_sell() {
        let buy_order = Order::new(5, 10, "account1", Direction::Buy);
        let sell_order = Order::new(10, 10, "account2", Direction::Sell);
        assert!(!buy_order.did_match(&sell_order));
    }

    #[test]
    fn higher_buy_does_match_with_sell() {
        let buy_order = Order::new(10, 10, "account1", Direction::Buy);
        let sell_order = Order::new(5, 5, "account2", Direction::Sell);
        assert!(buy_order.did_match(&sell_order));
    }

    #[test]
    #[should_panic(expected = "tried to match with same account_id")]
    fn same_account_cannot_match() {
        let buy_order = Order::new(10, 100, "account1", Direction::Buy);
        let sell_order = Order::new(10, 100, "account1", Direction::Sell);
        let _match = buy_order.match_with(&sell_order);
    }

    #[test]
    #[should_panic(expected = "tried to match with same direction")]
    fn matching_orders_must_be_different_directions() {
        let buy_order_1 = Order::new(10, 100, "account1", Direction::Buy);
        let buy_order_2 = Order::new(10, 100, "account2", Direction::Buy);
        let _match = buy_order_1.match_with(&buy_order_2);
    }
}
