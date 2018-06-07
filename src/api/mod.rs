use exchange::matcher::{Order, OrderMatch};
use exchange::Exchange;
use exchange::MatcherResult;
use rocket::State;
use rocket::{ignite, Rocket};
use rocket_contrib::Json;

#[post("/contracts/<id>/orders", data = "<order>")]
fn place_order(
    id: i32,
    order: Json<Order>,
    exchange: State<Exchange>,
) -> MatcherResult<Json<Vec<OrderMatch>>> {
    exchange
        .place_order(order.into_inner(), id)
        .map(|matches| Json(matches))
}

#[get("/contracts/<id>/orders")]
fn get_orders(id: i32, exchange: State<Exchange>) -> MatcherResult<Json<Vec<Order>>> {
    exchange.get_orders(id).map(|orders| Json(orders.clone()))
}

#[get("/contracts/<id>/matches")]
fn get_matches(id: i32, exchange: State<Exchange>) -> MatcherResult<Json<Vec<OrderMatch>>> {
    exchange
        .get_matches(id)
        .map(|matches| Json(matches.clone()))
}

#[get("/contracts/<id>/depth")]
fn get_depth(id: i32, exchange: State<Exchange>) -> MatcherResult<Json<Vec<Order>>> {
    exchange.get_depth(id).map(|depth| Json(depth))
}

pub fn new_api(exchange: Exchange) -> Rocket {
    ignite()
        .mount(
            "/",
            routes![place_order, get_depth, get_orders, get_matches],
        )
        .manage(exchange)
}
