use std::cmp;

#[derive(Clone, Debug)]
pub enum Direction {
    Buy,
    Sell,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: String,
    pub price: u32,
    pub quantity: u32,
    pub account_id: String,
    pub direction: Direction,
}

impl Order {
    fn get_quantity_matched(&self, top_order: &Order) -> Option<u32> {
        let did_match = {
            match self.direction {
                Direction::Buy => self.price >= top_order.price,
                Direction::Sell => self.price <= top_order.price,
            }
        };
        if did_match {
            Some(cmp::min(self.quantity, top_order.quantity))
        } else {
            None
        }
    }

    fn update_remaining(&mut self, matched_quantity: u32) {
        self.quantity = self.quantity - matched_quantity;
    }
}
