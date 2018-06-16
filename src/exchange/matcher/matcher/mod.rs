use super::depth::Depth;
use super::order_match::OrderMatch;
use super::orders::{DepthOrder, Direction, Order};

#[derive(Debug)]
pub struct Matcher {
  orders: Vec<Order>,
  matches: Vec<OrderMatch>,
  buy: Depth,
  sell: Depth,
  contract_id: i32,
}

impl Matcher {
  pub fn new(orders: Vec<Order>, contract_id: i32) -> Matcher {
    let (buy_orders, sell_orders) = orders
      .into_iter()
      .partition(|order| order.direction == Direction::Buy);
    Matcher {
      contract_id,
      buy: Depth::hydrate(buy_orders, Direction::Buy),
      sell: Depth::hydrate(sell_orders, Direction::Sell),
      orders: Vec::new(),
      matches: Vec::new(),
    }
  }

  pub fn get_depth(&self, direction: Direction) -> Vec<DepthOrder> {
    let orders = {
      match direction {
        Direction::Buy => self.buy.get_orders(),
        Direction::Sell => self.sell.get_orders(),
      }
    };
    DepthOrder::from_orders(orders)
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

  pub fn get_orders(&self) -> &Vec<Order> {
    &self.orders
  }

  pub fn get_matches(&self) -> &Vec<OrderMatch> {
    &self.matches
  }
}
