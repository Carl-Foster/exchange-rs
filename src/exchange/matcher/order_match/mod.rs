use super::orders::{Direction, Order};
use chrono::prelude::{DateTime, Utc};
use std::cmp;
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderMatch {
    id: String,
    buy_order_id: String,
    sell_order_id: String,
    pub quantity_matched: u32,
    price_matched: u32,
    created_at: DateTime<Utc>,
}

impl OrderMatch {
    fn new(new_order: &Order, top_order: &Order) -> OrderMatch {
        let quantity_matched = cmp::min(new_order.quantity, top_order.quantity);
        let price_matched = top_order.price;
        let (buy_order_id, sell_order_id) = {
            match new_order.direction {
                Direction::Buy => (new_order.id.clone(), top_order.id.clone()),
                Direction::Sell => (top_order.id.clone(), new_order.id.clone()),
            }
        };
        OrderMatch {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            buy_order_id,
            sell_order_id,
            quantity_matched,
            price_matched,
        }
    }

    pub fn match_orders(new_order: &Order, top_order: &Order) -> Option<OrderMatch> {
        assert_ne!(
            new_order.account_id, top_order.account_id,
            "Order {} and Order {} tried to match with same account_id",
            new_order.id, top_order.id
        );
        assert_ne!(
            new_order.direction, top_order.direction,
            "Order {} and Order {} tried to match with same direction",
            new_order.id, top_order.id
        );
        assert!(
            top_order.quantity > 0,
            "Top Order does not have valid quantity"
        );
        if new_order.quantity == 0 || !OrderMatch::did_match(new_order, top_order) {
            None
        } else {
            Some(OrderMatch::new(new_order, top_order))
        }
    }

    fn did_match(new_order: &Order, top_order: &Order) -> bool {
        match new_order.direction {
            Direction::Buy => new_order.price >= top_order.price,
            Direction::Sell => new_order.price <= top_order.price,
        }
    }
}
