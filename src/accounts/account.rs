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
    type ID = Uuid;
    fn get_id_as_string(&self) -> String {
        self.account_id.hyphenated().to_string()
    }

    fn get_id(&self) -> Self::ID {
        self.account_id
    }
}

impl Store for Account {
    const PATH: &'static str = "accounts";
}
