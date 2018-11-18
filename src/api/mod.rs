use rocket::{ignite, Rocket};

mod contracts {
    use exchange::error::BadContractError;
    use exchange::Exchange;
    type NoContractResult<T> = Result<T, BadContractError>;
    use exchange::{DepthOrder, Direction, Order, OrderMatch};
    use rocket::State;
    use rocket_contrib::Json;

    #[post("/<id>/orders", data = "<order>")]
    fn place_order(
        id: i32,
        order: Json<Order>,
        exchange: State<Exchange>,
    ) -> NoContractResult<Json<Vec<OrderMatch>>> {
        exchange.place_order(order.into_inner(), id).map(Json)
    }

    #[get("/<id>/orders")]
    fn get_orders(id: i32, exchange: State<Exchange>) -> NoContractResult<Json<Vec<Order>>> {
        exchange.get_orders(id).map(Json)
    }

    #[get("/<id>/matches")]
    fn get_matches(id: i32, exchange: State<Exchange>) -> NoContractResult<Json<Vec<OrderMatch>>> {
        exchange.get_matches(id).map(Json)
    }

    #[get("/<id>/depth/<direction>")]
    fn get_depth(
        id: i32,
        direction: Direction,
        exchange: State<Exchange>,
    ) -> NoContractResult<Json<Vec<DepthOrder>>> {
        exchange.get_depth(id, direction).map(Json)
    }

}

pub struct Api(Rocket);

impl Api {
    pub fn init() -> Api {
        Api(ignite().mount(
            "/contracts",
            routes![
                contracts::place_order,
                contracts::get_matches,
                contracts::get_orders,
                contracts::get_depth
            ],
        ))
    }

    pub fn add_state<T: Send + Sync + 'static>(mut self, state: T) -> Self {
        self.0 = self.0.manage(state);
        self
    }

    pub fn start(self) {
        self.0.launch();
    }
}
