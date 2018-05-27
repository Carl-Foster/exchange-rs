#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate matcher;
extern crate rocket;
extern crate rocket_contrib;

mod api;
mod exchange;

use api::new_api;
use exchange::Exchange;

fn main() {
    let exchange = Exchange::init();
    new_api(exchange).launch();
}
