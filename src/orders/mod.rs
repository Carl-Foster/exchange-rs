use std::cmp;
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
}

impl Order {
    pub fn new(price: u32, quantity: u32, account_id: &str, direction: Direction) -> Order {
        Order {
            price,
            quantity,
            direction,
            account_id: account_id.to_string(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn match_order(&mut self, other: &mut Order) {
        assert_ne!(
            self.account_id, other.account_id,
            "Order {} and order {} tried to match with same account_id {}",
            self.id, other.id, self.account_id
        );
        assert_ne!(
            self.direction, other.direction,
            "Order {} and order {} tried to match with same direction",
            self.id, other.id
        );
        let quantity_matched = self.get_quantity_matched(other);
        if quantity_matched > 0 {
            self.update_remaining(quantity_matched);
            other.update_remaining(quantity_matched);
        }
    }

    fn get_quantity_matched(&self, top_order: &Order) -> u32 {
        let did_match = {
            match self.direction {
                Direction::Buy => self.price >= top_order.price,
                Direction::Sell => self.price <= top_order.price,
            }
        };
        if did_match {
            cmp::min(self.quantity, top_order.quantity)
        } else {
            0
        }
    }

    pub fn update_remaining(&mut self, matched_quantity: u32) {
        self.quantity = self.quantity - matched_quantity;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn low_buy_does_not_match_high_sell() {
        let new_order = Order::new(5, 10, "account1", Direction::Buy);
        let top_order = Order::new(10, 10, "account2", Direction::Sell);
        assert_eq!(0, new_order.get_quantity_matched(&top_order));
    }

    #[test]
    fn matches_lowest_quantity() {
        let buy_order = Order::new(10, 100, "account1", Direction::Buy);
        let sell_order = Order::new(10, 5, "account2", Direction::Sell);
        assert_eq!(5, buy_order.get_quantity_matched(&sell_order));
    }

    #[test]
    #[should_panic(expected = "tried to match with same account_id")]
    fn same_account_cannot_match() {
        let mut buy_order = Order::new(10, 100, "account1", Direction::Buy);
        let mut sell_order = Order::new(10, 100, "account1", Direction::Sell);
        buy_order.match_order(&mut sell_order);
    }

    #[test]
    #[should_panic(expected = "tried to match with same direction")]
    fn matching_orders_must_be_different_directions() {
        let mut buy_order_1 = Order::new(10, 100, "account1", Direction::Buy);
        let mut buy_order_2 = Order::new(10, 100, "account2", Direction::Buy);
        buy_order_1.match_order(&mut buy_order_2);
    }

    #[test]
    fn matched_order_updates_both_quantities() {
        let mut buy_order = Order::new(120, 10, "account1", Direction::Buy);
        let mut sell_order = Order::new(110, 5, "account2", Direction::Sell);
        buy_order.match_order(&mut sell_order);

        assert_eq!(5, buy_order.quantity);
        assert_eq!(0, sell_order.quantity);
    }

    #[test]
    fn unmatched_order_does_not_update_quantities() {
        let mut buy_order = Order::new(50, 10, "account1", Direction::Buy);
        let mut sell_order = Order::new(100, 10, "account2", Direction::Sell);
        buy_order.match_order(&mut sell_order);
        assert_eq!(10, buy_order.quantity);
        assert_eq!(10, sell_order.quantity);
    }

}
