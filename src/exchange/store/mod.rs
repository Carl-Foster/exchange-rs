use diesel::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub mod schema;

use self::schema::order_matches;
use self::schema::orders;
use super::matcher::{Order, OrderMatch};

pub struct Store {
    conn: PgConnection,
}

impl Store {
    pub fn new() -> Store {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url).expect("Error connecting to database");
        Store { conn }
    }

    pub fn get_orders(&self, contract_id: u32) -> Vec<Order> {
        orders::table
            .load(&self.conn)
            .expect("Error getting orders from database")
    }

    pub fn get_order_matches(&self, contract_id: u32) -> Vec<OrderMatch> {
        order_matches::table
            .load(&self.conn)
            .expect("Error getting order matches from database")
    }

    pub fn save_order_and_matches(&self, order: &Order, matches: &Vec<OrderMatch>) {
        self.save_order(order);
        self.save_order_matches(matches);
    }

    pub fn save_order(&self, order: &Order) {
        insert_into(orders::table)
            .values(order)
            .execute(&self.conn)
            .expect("Error saving orders");
    }

    pub fn save_order_matches(&self, matches: &Vec<OrderMatch>) {
        insert_into(order_matches::table)
            .values(matches)
            .execute(&self.conn)
            .expect("Error saving order matches");
    }
}
