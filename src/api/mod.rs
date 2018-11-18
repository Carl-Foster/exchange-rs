use rocket::{ignite, Rocket};

mod contracts {
    use exchange::error::BadContractError;
    use exchange::Exchange;
    type NoContractResult<T> = Result<T, BadContractError>;
    use exchange::{DepthOrder, Direction, Order, OrderMatch};
    use rocket::State;
    use rocket_contrib::Json;
    use std::io;
    #[post("/<id>/orders", data = "<order>")]
    fn place_order(
        id: i32,
        order: Json<Order>,
        exchange: State<Exchange>,
    ) -> NoContractResult<io::Result<Json<Vec<OrderMatch>>>> {
        exchange
            .place_order(order.into_inner(), id)
            .map(|result| result.map(Json))
    }

    #[get("/<id>/orders")]
    fn get_orders(id: i32, exchange: State<Exchange>) -> NoContractResult<Json<Vec<Order>>> {
        exchange.get_orders(id).map(|orders| Json(orders))
    }

    #[get("/<id>/matches")]
    fn get_matches(id: i32, exchange: State<Exchange>) -> NoContractResult<Json<Vec<OrderMatch>>> {
        exchange.get_matches(id).map(|matches| Json(matches))
    }

    #[get("/<id>/depth/<direction>")]
    fn get_depth(
        id: i32,
        direction: Direction,
        exchange: State<Exchange>,
    ) -> NoContractResult<Json<Vec<DepthOrder>>> {
        exchange.get_depth(id, direction).map(|depth| Json(depth))
    }

}

struct Api(Rocket);

impl Api {
    pub fn init() -> Rocket {
        ignite().mount(
            "/contracts",
            routes![
                contracts::place_order,
                contracts::get_matches,
                contracts::get_orders,
                contracts::get_depth
            ],
        )
    }

    pub fn add_state<T: Send + Sync + 'static>(&mut self, state: T) -> Self {
        self.0.manage(state)
    }

    pub fn start(&mut self) {
        self.0.launch();
    }
}
