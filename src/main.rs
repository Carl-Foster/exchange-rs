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

mod accounts;
mod api;
mod exchange;
mod store;

use accounts::Accounts;
use api::new_api;
use exchange::Exchange;

fn main() {
    dotenv::dotenv().ok();
    let exchange = Exchange::init();
    let accounts = Accounts::init();

    Api::new().add_state(exchange).add_state(accounts).start();
}
