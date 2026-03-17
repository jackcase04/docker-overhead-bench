use std::{collections::HashMap, f32::consts::PI};
use serde::{Deserialize};

use crate::transaction::Transaction;

pub struct Processor {
    pub users: HashMap<u32, User>
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub f_name: String,
    pub l_name: String,
    pub home_lat: f64,
    pub home_long: f64
}

impl Processor {
    pub fn process_transaction(&self, transaction: &Transaction) -> bool {
        let user: Option<&User> = self.users.get(&transaction.user_id); 
        
        match user {
            Some(_user) => true,
            None => false
        }
    }
}

fn haversine(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64{
    // Using haversine formula:
    // https://www.geeksforgeeks.org/dsa/haversine-formula-to-find-distance-between-two-points-on-a-sphere/

    let d_lat: f64 = ((lat2 - lat1) * PI as f64 / 180.0);
    let d_lon: f64 = ((long2 - long1) * PI as f64 / 180.0);

    let temp_lat1 = lat1 * PI as f64 / 180.0;
    let temp_lat2 = lat2 * PI as f64 / 180.0;

    let a = (d_lat / 2.0).sin().powf(2.0) + 
                (d_lon / 2.0).sin().powf(2.0) *
                temp_lat1.cos() * temp_lat2.cos();

    let rad = 6371.0;
    let c = 2.0 * a.sqrt().asin();

    rad * c
}
