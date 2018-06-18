use serde_json;
use std::io::{Error, Read};
use std::{fs::File, io};

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

  pub fn place_order(&mut self, new_order: Order) -> Result<Vec<OrderMatch>, Error> {
    self.orders.push(new_order.clone());
    let order_matches = self.match_order(new_order);
    self.matches.append(&mut order_matches.clone());
    self.save_state()?;
    Ok(order_matches)
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

  pub fn save_state(&self) -> io::Result<()> {
    let filename = format!("matcher_{}.json", self.contract_id);
    File::create(&filename)
      .map(|file| serde_json::to_writer(file, self))
      .map(|_| ())
  }

  pub fn init_matcher_from_store(contract_id: i32) -> Option<Matcher> {
    let hydrate_file = format!("matcher_{}.json", contract_id);
    let contents = File::open(&hydrate_file).and_then(|mut file| {
      let mut s = String::new();
      file.read_to_string(&mut s)?;
      Ok(s)
    });
    match contents {
      Ok(s) => serde_json::from_str(&s).ok(),
      Err(_) => None,
    }
  }
}
