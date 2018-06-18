use accounts::balance::Balance;
use std::collections::HashMap;
use std::sync::Mutex;
use store::GetID;
use store::Store;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    account_id: Uuid,
    balances: HashMap<i32, Mutex<Balance>>,
}

impl GetID for Account {
    fn get_id(&self) -> String {
        self.account_id.hyphenated().to_string()
    }
}

impl Store for Account {
    const PATH: &'static str = "accounts";
}
