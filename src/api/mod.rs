use exchange::error::BadContractError;
use exchange::matcher::{DepthOrder, Direction, Order, OrderMatch};
use exchange::Exchange;
use rocket::response::status::NotFound;
use rocket::State;
use rocket::{ignite, Rocket};
use rocket_contrib::Json;
use std::io;

type NoContractResult<T> = Result<T, NotFound<BadContractError>>;

#[post("/contracts/<id>/orders", data = "<order>")]
fn place_order(
    id: i32,
    order: Json<Order>,
    exchange: State<Exchange>,
) -> NoContractResult<io::Result<Json<Vec<OrderMatch>>>> {
    exchange
        .place_order(order.into_inner(), id)
        .map(|result| result.map(Json))
        .map_err(|e| NotFound(e))
}

#[get("/contracts/<id>/orders")]
fn get_orders(id: i32, exchange: State<Exchange>) -> NoContractResult<Json<Vec<Order>>> {
    exchange
        .get_orders(id)
        .map(|orders| Json(orders.clone()))
        .map_err(|e| NotFound(e))
}

#[get("/contracts/<id>/matches")]
fn get_matches(id: i32, exchange: State<Exchange>) -> NoContractResult<Json<Vec<OrderMatch>>> {
    exchange
        .get_matches(id)
        .map(|matches| Json(matches.clone()))
        .map_err(|e| NotFound(e))
}

#[get("/contracts/<id>/depth/<direction>")]
fn get_depth(
    id: i32,
    direction: Direction,
    exchange: State<Exchange>,
) -> NoContractResult<Json<Vec<DepthOrder>>> {
    exchange
        .get_depth(id, direction)
        .map(|depth| Json(depth))
        .map_err(|e| NotFound(e))
}

pub fn new_api(exchange: Exchange) -> Rocket {
    ignite()
        .mount(
            "/",
            routes![place_order, get_depth, get_orders, get_matches],
        )
        .manage(exchange)
}
