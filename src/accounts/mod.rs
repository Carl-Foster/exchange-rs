use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

mod account;
mod balance;

use accounts::account::Account;

#[derive(Debug)]
struct Accounts {
  mapping: HashMap<Uuid, Mutex<Account>>,
}

impl Accounts {
  pub fn init() -> Accounts {
    Accounts {
      mapping: HashMap::new(),
    }
  }
}
