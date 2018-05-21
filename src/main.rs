#![feature(plugin)]
#![feature(drain_filter)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

mod depth;
mod exchange;
mod matcher;
mod order_match;
mod orders;

use exchange::Exchange;
use orders::Order;
use rocket::State;
use rocket_contrib::Json;
use std::sync::Mutex;

#[post("/contract/<id>/order", format = "application/json", data = "<order>")]
fn new_order(id: u32, order: Json<Order>, exchange: State<Mutex<Exchange>>) -> String {
    let mut manager = exchange.lock().expect("exchange lock.");
    match manager.place_order(order.into_inner(), id) {
        Ok(matches) => serde_json::to_string(&matches).unwrap(),
        Err(error) => error,
    }
}

fn main() {
    let exchange = Exchange::init();
    rocket::ignite()
        .mount("/", routes![new_order])
        .manage(Mutex::new(exchange))
        .launch();
}
