extern crate actix;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

mod depth;
mod exchange;
mod matcher;
mod order_match;
mod orders;

use exchange::Exchange;

fn main() {
    let exchange = Exchange::init();
}
