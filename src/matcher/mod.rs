use depth::Depth;
use order_match::OrderMatch;
use orders::{Direction, Order};

#[derive(Debug)]
struct Matcher {
  buy: Depth,
  sell: Depth,
  contract_id: String,
}

impl Matcher {
  fn new(mut orders: Vec<Order>, contract_id: &str) -> Matcher {
    let (buy_orders, sell_orders) = {
      let buy_orders = orders
        .drain_filter(|order| order.direction == Direction::Buy)
        .collect::<Vec<Order>>();
      let sell_orders = orders;
      (buy_orders, sell_orders)
    };
    Matcher {
      buy: Depth::hydrate(buy_orders, Direction::Buy),
      sell: Depth::hydrate(sell_orders, Direction::Sell),
      contract_id: contract_id.to_string(),
    }
  }

  fn place_order(&mut self, mut new_order: Order) -> Vec<OrderMatch> {
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
