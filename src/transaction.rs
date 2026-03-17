use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub transaction_id: u64,
    pub user_id: u32,
    pub amount_cents: u64,
    pub merchant_lat: f64,
    pub merchant_long: f64
}