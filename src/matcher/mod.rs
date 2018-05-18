use depth::Depth;
use orders::{Direction, Order};

#[derive(Debug)]
struct Matcher {
  buy: Depth,
  sell: Depth,
  contract_id: String,
}

impl Matcher {
  fn new(orders: &mut Vec<Order>, contract_id: &str) -> Matcher {
    let (mut buy_orders, mut sell_orders) = {
      let buy_orders = orders
        .drain_filter(|order| order.direction == Direction::Buy)
        .collect::<Vec<Order>>();
      let sell_orders = orders;
      (buy_orders, sell_orders)
    };
    Matcher {
      buy: Depth::hydrate(&mut buy_orders, Direction::Buy),
      sell: Depth::hydrate(&mut sell_orders, Direction::Sell),
      contract_id: contract_id.to_string(),
    }
  }

  fn place_order(&mut self, new_order: &mut Order) {
    let (depth_to_match, depth_to_add) = {
      if let Direction::Buy = new_order.direction {
        (&mut self.sell, &mut self.buy)
      } else {
        (&mut self.buy, &mut self.sell)
      }
    };

    if let Some(unmatched_order) = depth_to_match.match_order(new_order) {
      depth_to_add.add_order(unmatched_order);
    }
  }
}
