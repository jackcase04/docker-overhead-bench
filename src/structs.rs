use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub transaction_id: u64,
    pub user_id: u32,
    pub amount_cents: u64,
    pub merchant_lat: f64,
    pub merchant_long: f64
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub f_name: String,
    pub l_name: String,
    pub max_trans_cents: u32,
    pub home_lat: f64,
    pub home_long: f64
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub iterations: u32,
    pub concurrency: u32,
    pub address: String
}