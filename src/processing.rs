use std::{collections::HashMap, f32::consts::PI};

use crate::structs::RiskLevel;
use crate::structs::Transaction;
use crate::structs::User;

const SOFT_DIST: f64 = 500.0;
const HARD_DIST: f64 = 50000.0;
pub struct Processor {
    pub users: HashMap<u32, User>,
}

impl Processor {
    pub fn process_transaction(&self, transaction: &Transaction) -> RiskLevel {
        let user: Option<&User> = self.users.get(&transaction.user_id);

        if let Some(user) = user {
            println!("user: {0}", user.f_name);

            let distance = haversine(
                user.home_lat,
                user.home_long,
                transaction.merchant_lat,
                transaction.merchant_long,
            );
            println!("Distance between transaction and user: {0}", distance);

            let cents = transaction.amount_cents;
            println!("Transaction cents amount: {0}", cents);

            if cents < user.max_trans_cents as u64 && distance < HARD_DIST {
                RiskLevel::Approve
            } else {
                RiskLevel::HardFlag
            }
        } else {
            RiskLevel::HardFlag
        }
    }
}

fn haversine(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64 {
    // Using haversine formula:
    // https://www.geeksforgeeks.org/dsa/haversine-formula-to-find-distance-between-two-points-on-a-sphere/

    println!(
        "Lat1: {0} Lon1: {1} Lat2: {2} Lon2: {3}",
        lat1, long1, lat2, long2
    );

    let d_lat: f64 = (lat2 - lat1) * PI as f64 / 180.0;
    let d_lon: f64 = (long2 - long1) * PI as f64 / 180.0;

    let temp_lat1 = lat1 * PI as f64 / 180.0;
    let temp_lat2 = lat2 * PI as f64 / 180.0;

    let a = (d_lat / 2.0).sin().powf(2.0)
        + (d_lon / 2.0).sin().powf(2.0) * temp_lat1.cos() * temp_lat2.cos();

    let rad = 6371.0;
    let c = 2.0 * a.sqrt().asin();

    rad * c
}
