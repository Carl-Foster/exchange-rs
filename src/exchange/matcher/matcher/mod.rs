use super::depth::Depth;
use super::order_match::OrderMatch;
use super::orders::{Direction, Order};

#[derive(Debug)]
pub struct Matcher {
  buy: Depth,
  sell: Depth,
  contract_id: u32,
}

impl Matcher {
  pub fn new(mut orders: Vec<Order>, contract_id: u32) -> Matcher {
    let (buy_orders, sell_orders) = {
      let buy_orders = orders
        .drain_filter(|order| order.direction == Direction::Buy)
        .collect::<Vec<Order>>();
      let sell_orders = orders;
      (buy_orders, sell_orders)
    };
    Matcher {
      contract_id,
      buy: Depth::hydrate(buy_orders, Direction::Buy),
      sell: Depth::hydrate(sell_orders, Direction::Sell),
    }
  }

  pub fn get_orders(&self) -> Vec<Order> {
    let buy_orders = self.buy.get_orders();
    let sell_orders = self.sell.get_orders();

    let mut orders: Vec<Order> = Vec::new();
    orders.append(&mut buy_orders.clone());
    orders.append(&mut sell_orders.clone());

    orders
  }

  pub fn place_order(&mut self, mut new_order: Order) -> Vec<OrderMatch> {
    let (depth_to_match, depth_to_add) = {
      if let Direction::Buy = new_order.direction {
        (&mut self.sell, &mut self.buy)
      } else {
        (&mut self.buy, &mut self.sell)
      }
    };

    let order_matches = depth_to_match.match_order(&mut new_order);
    depth_to_match.flush_filled_orders();
    if new_order.quantity > 0 {
      depth_to_add.add_order(new_order);
    }
    order_matches
  }
}
