use std::collections::HashMap;

use exchange::matcher::orders::Order;

#[derive(Debug, Serialize)]
pub struct DepthOrder {
  quantity: i32,
  price: i32,
}

impl DepthOrder {
  pub fn from_orders(orders: &[Order]) -> Vec<DepthOrder> {
    let mut price_map: HashMap<i32, i32> = HashMap::new();
    for order in orders.to_owned() {
      let quantity = {
        let existing_quantity = price_map.get_mut(&order.price);
        if let Some(stored) = existing_quantity {
          *stored
        } else {
          0
        }
      };
      price_map.insert(order.price, quantity + order.quantity);
    }
    price_map
      .iter()
      .map(|(price, quantity)| DepthOrder {
        price: *price,
        quantity: *quantity,
      })
      .collect()
  }
}
