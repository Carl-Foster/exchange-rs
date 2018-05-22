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
use order_match::OrderMatch;
use orders::Order;
use rocket::State;
use rocket_contrib::Json;

#[post("/contracts/<id>/orders", data = "<order>")]
fn new_order(
    id: u32,
    order: Json<Order>,
    exchange: State<Exchange>,
) -> Result<Json<Vec<OrderMatch>>, String> {
    match exchange.place_order(order.into_inner(), id) {
        Ok(matches) => Ok(Json(matches)),
        Err(error) => Err(error),
    }
}

#[get("/contracts/<id>/orders")]
fn orders(id: u32, exchange: State<Exchange>) -> Result<Json<Vec<Order>>, String> {
    match exchange.get_orders(id) {
        Ok(orders) => Ok(Json(orders)),
        Err(error) => Err(error),
    }
}

fn main() {
    let exchange = Exchange::init();
    rocket::ignite()
        .mount("/", routes![new_order, orders])
        .manage(exchange)
        .launch();
}
