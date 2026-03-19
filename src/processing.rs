use std::{collections::HashMap, f32::consts::PI};

use crate::structs::Transaction;
use crate::structs::User;

pub struct Processor {
    pub users: HashMap<u32, User>
}

impl Processor {
    pub fn process_transaction(&self, transaction: &Transaction) -> bool {
        let user: Option<&User> = self.users.get(&transaction.user_id); 

        println!("user: {0}", user.unwrap().f_name);

        match user {
            Some(_user) => {
                let distance: f64 = haversine(user.unwrap().home_lat, user.unwrap().home_long, transaction.merchant_lat, transaction.merchant_long);

                println!("Distance between transaction and user: {0}", distance);

                true
            },
            None => false
        }
    }
}

fn haversine(lat1: f64, long1: f64, lat2: f64, long2: f64) -> f64{
    // Using haversine formula:
    // https://www.geeksforgeeks.org/dsa/haversine-formula-to-find-distance-between-two-points-on-a-sphere/

    println!("Lat1: {0} Lon1: {1} Lat2: {2} Lon2: {3}", lat1, long1, lat2, long2);

    let d_lat: f64 = (lat2 - lat1) * PI as f64 / 180.0;
    let d_lon: f64 = (long2 - long1) * PI as f64 / 180.0;

    let temp_lat1 = lat1 * PI as f64 / 180.0;
    let temp_lat2 = lat2 * PI as f64 / 180.0;

    let a = (d_lat / 2.0).sin().powf(2.0) + 
                (d_lon / 2.0).sin().powf(2.0) *
                temp_lat1.cos() * temp_lat2.cos();

    let rad = 6371.0;
    let c = 2.0 * a.sqrt().asin();

    rad * c
}
