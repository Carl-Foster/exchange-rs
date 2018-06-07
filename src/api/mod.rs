use exchange::matcher::{Order, OrderMatch};
use exchange::Exchange;
use rocket::State;
use rocket::{ignite, Rocket};
use rocket_contrib::Json;

#[post("/orders", data = "<order>")]
fn place_order(
    order: Json<Order>,
    exchange: State<Exchange>,
) -> Result<Json<Vec<OrderMatch>>, String> {
    match exchange.place_order(order.into_inner()) {
        Ok(matches) => Ok(Json(matches)),
        Err(error) => Err(error),
    }
}

#[get("/contracts/<id>/orders")]
fn get_orders(id: i32, exchange: State<Exchange>) -> Json<Vec<Order>> {
    let orders = exchange
        .get_orders()
        .iter()
        .filter(|order| order.contract_id == id)
        .cloned()
        .collect();

    Json(orders)
}

#[get("/contracts/<id>/matches")]
fn get_matches(id: i32, exchange: State<Exchange>) -> Json<Vec<OrderMatch>> {
    let matches = exchange
        .get_matches()
        .iter()
        .filter(|order_match| order_match.contract_id == id)
        .cloned()
        .collect();
    Json(matches)
}

#[get("/contracts/<id>/depth")]
fn get_depth(id: i32, exchange: State<Exchange>) -> Result<Json<Vec<Order>>, String> {
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
