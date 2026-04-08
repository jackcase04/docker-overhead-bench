use std::{collections::HashMap, f32::consts::PI, thread, time::Duration};

use crate::structs::RiskLevel;
use crate::structs::Transaction;
use crate::structs::User;

const SOFT_DIST: f64 = 500.0;
const HARD_DIST: f64 = 2000.0;

pub struct Processor {
    pub users: HashMap<u32, User>,
}

const LAT: u64 = 5;

impl Processor {
    pub fn process_transaction(&self, transaction: &Transaction) -> RiskLevel {
        let user: Option<&User> = self.users.get(&transaction.user_id);

        // LAT ms delay to simulate lookup
        thread::sleep(Duration::from_millis(LAT));

        if let Some(user) = user {
            let distance = haversine(
                user.home_lat,
                user.home_long,
                transaction.merchant_lat,
                transaction.merchant_long,
            );

            let cents = transaction.amount_cents;

            // CASE 1: Transaction is below the users limit, and below the soft flag distance.
            if cents <= user.max_trans_cents as u64 && distance < SOFT_DIST {
                RiskLevel::Approve
            }
            // CASE 2: Transaction is above the limit, but not 2x+ the limit.
            // Or, transaction is above soft distance flag but not exceeding the hard distance
            else if (cents > user.max_trans_cents as u64
                && cents < (user.max_trans_cents * 2) as u64)
                || (distance > SOFT_DIST && distance < HARD_DIST)
            {
                RiskLevel::SoftFlag
            }
            // CASE 3: Transaction exceeds their max by 2x+, or exceeds hard distance
            else {
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
