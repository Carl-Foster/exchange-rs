use exchange::matcher::{Order, OrderMatch};
use exchange::Exchange;
use rocket::State;
use rocket::{ignite, Rocket};
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

pub fn new_api(exchange: Exchange) -> Rocket {
    ignite()
        .mount("/", routes![new_order, orders])
        .manage(exchange)
}
