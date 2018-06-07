use exchange::matcher::{Order, OrderMatch};
use exchange::Exchange;
use rocket::response::status::NotFound;
use rocket::State;
use rocket::{ignite, Rocket};
use rocket_contrib::Json;

#[post("/contracts/<id>/orders", data = "<order>")]
fn place_order(
    id: i32,
    order: Json<Order>,
    exchange: State<Exchange>,
) -> Result<Json<Vec<OrderMatch>>, NotFound<String>> {
    exchange
        .place_order(order.into_inner(), id)
        .map(|matches| Json(matches))
        .map_err(|e| NotFound(format!("{}", e)))
}

#[get("/contracts/<id>/orders")]
fn get_orders(
    id: i32,
    exchange: State<Exchange>,
) -> Result<Json<Vec<Order>>, NotFound<Json<String>>> {
    exchange
        .get_orders(id)
        .map(|orders| Json(orders.clone()))
        .map_err(|e| NotFound(Json(format!("{}", e))))
}

#[get("/contracts/<id>/matches")]
fn get_matches(
    id: i32,
    exchange: State<Exchange>,
) -> Result<Json<Vec<OrderMatch>>, NotFound<String>> {
    exchange
        .get_matches(id)
        .map(|matches| Json(matches.clone()))
        .map_err(|e| NotFound(format!("{}", e)))
}

#[get("/contracts/<id>/depth")]
fn get_depth(id: i32, exchange: State<Exchange>) -> Result<Json<Vec<Order>>, NotFound<String>> {
    exchange
        .get_depth(id)
        .map(|depth| Json(depth))
        .map_err(|e| NotFound(format!("{}", e)))
}

pub fn new_api(exchange: Exchange) -> Rocket {
    ignite()
        .mount(
            "/",
            routes![place_order, get_depth, get_orders, get_matches],
        )
        .manage(exchange)
}
