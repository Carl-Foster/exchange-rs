use exchange::balance::Balance;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    account_id: Uuid,
    balances: HashMap<&'static str, Mutex<Balance>>,
}
