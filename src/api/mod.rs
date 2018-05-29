use exchange::matcher::{Order, OrderMatch};
use exchange::Exchange;
use rocket::State;
use rocket::{ignite, Rocket};
use rocket_contrib::Json;

#[post("/contracts/<id>/orders", data = "<order>")]
fn place_order(
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
fn get_orders(id: u32, exchange: State<Exchange>) -> Result<Json<Vec<Order>>, String> {
    match exchange.get_orders(id) {
        Ok(orders) => Ok(Json(orders)),
        Err(error) => Err(error),
    }
}

#[get("/contracts/<id>/matches")]
fn get_matches(id: u32, exchange: State<Exchange>) -> Result<Json<Vec<OrderMatch>>, String> {
    match exchange.get_matches(id) {
        Ok(matches) => Ok(Json(matches)),
        Err(error) => Err(error),
    }
}

#[get("/contracts/<id>/depth")]
fn get_depth(id: u32, exchange: State<Exchange>) -> Result<Json<Vec<Order>>, String> {
    match exchange.get_depth(id) {
        Ok(orders) => Ok(Json(orders)),
        Err(error) => Err(error),
    }
}

pub fn new_api(exchange: Exchange) -> Rocket {
    ignite()
        .mount(
            "/",
            routes![place_order, get_depth, get_orders, get_matches],
        )
        .manage(exchange)
}
