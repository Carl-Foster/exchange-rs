use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

mod account;
mod balance;

use accounts::account::Account;
use store::{GetID, Store};

#[derive(Debug)]
pub struct Accounts {
  mapping: HashMap<Uuid, Mutex<Account>>,
}

impl Accounts {
  pub fn init() -> Accounts {
    let mut mapping = HashMap::new();
    let accounts_dir = Path::new("/store").join(Account::PATH);
    let accounts_option = fs::read_dir(accounts_dir)
      .map(|entries| {
        entries.map(|entry| {
          entry
            .ok()
            .and_then(|file| Account::init_from_store(file.path().file_stem().unwrap()))
        })
      })
      .ok();
    accounts_option.map(|accounts| {
      accounts.for_each(|account_option| {
        account_option.and_then(|account| mapping.insert(account.get_id(), Mutex::new(account)));
      })
    });

    Accounts { mapping }
  }
}
