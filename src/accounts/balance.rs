#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    asset_id: i32,
    amount: i32,
    reserved: i32,
}
