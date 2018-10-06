#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod api;
mod exchange;

use api::new_api;
use exchange::Exchange;

fn main() {
    dotenv::dotenv().ok();
    let exchange = Exchange::init();
    new_api(exchange).launch();
}
