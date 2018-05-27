use chrono::prelude::{DateTime, Utc};
use uuid::Uuid;

use exchange::schema::orders;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[table_name = "orders"]
pub struct Order {
    #[serde(default = "Order::new_id")]
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub price: u32,
    pub quantity: u32,
    pub account_id: String,
    pub direction: Direction,
    #[serde(default = "Utc::now")]
    #[serde(skip_deserializing)]
    pub created_at: DateTime<Utc>,
}

impl Order {
    #[cfg(test)]
    pub fn new(price: u32, quantity: u32, account_id: &str, direction: Direction) -> Order {
        Order {
            price,
            quantity,
            direction,
            account_id: account_id.to_string(),
            id: Order::new_id(),
            created_at: Utc::now(),
        }
    }

    fn new_id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn update_remaining(&mut self, matched_quantity: u32) {
        assert!(
            self.quantity >= matched_quantity,
            "Quantity to remove is greater than order's current quantity"
        );
        self.quantity -= matched_quantity;
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
}
