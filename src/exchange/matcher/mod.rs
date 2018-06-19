use std::io;
use store::GetID;
use store::Store;

use super::depth::Depth;
use super::order_match::OrderMatch;
use super::orders::{DepthOrder, Direction, Order};

#[derive(Debug, Serialize, Deserialize)]
pub struct Matcher {
  orders: Vec<Order>,
  matches: Vec<OrderMatch>,
  buy: Depth,
  sell: Depth,
  contract_id: i32,
}

impl GetID for Matcher {
  type ID = i32;
  fn get_id_as_string(&self) -> String {
    self.contract_id.to_string()
  }

  fn get_id(&self) -> Self::ID {
    self.contract_id
  }
}

impl Store for Matcher {
  const PATH: &'static str = "matchers";
}

impl Matcher {
  pub fn new(contract_id: i32) -> Matcher {
    Matcher {
      contract_id,
      buy: Depth::hydrate(Vec::new(), Direction::Buy),
      sell: Depth::hydrate(Vec::new(), Direction::Sell),
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

  pub fn place_order(&mut self, new_order: Order) -> io::Result<Vec<OrderMatch>> {
    self.orders.push(new_order.clone());
    let order_matches = self.match_order(new_order);
    self.matches.append(&mut order_matches.clone());
    self.save_state().map(|_| order_matches)
  }

  fn match_order(&mut self, mut new_order: Order) -> Vec<OrderMatch> {
    let (depth_to_match, depth_to_add) = {
      match new_order.direction {
        Direction::Buy => (&mut self.sell, &mut self.buy),
        Direction::Sell => (&mut self.buy, &mut self.sell),
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
