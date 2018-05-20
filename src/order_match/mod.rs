use chrono::prelude::{DateTime, Utc};
use orders::{Direction, Order};
use std::cmp;
use uuid::Uuid;

pub struct OrderMatch {
    id: String,
    buy_order_id: String,
    sell_order_id: String,
    pub quantity_matched: u32,
    price_matched: u32,
    created_at: DateTime<Utc>,
}

impl OrderMatch {
    pub fn new(new_order: &Order, top_order: &Order) -> OrderMatch {
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
}
