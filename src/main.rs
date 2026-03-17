use std::collections::HashMap;
use crate::{processing::{Processor, User}, transaction::Transaction};

mod transaction;
mod processing;
fn main() {
    let mut processor = Processor {
        users: HashMap::new()
    };

    processor.users.insert(1, User {
        f_name: String::from("Jack"),
        l_name: String::from("Case"),
        home_lat: 38.79989156147256,
        home_long: -90.4827405141186
    });

    let transaction = Transaction {
        transaction_id: 1,
        user_id: 2,
        amount_cents: 500,
        merchant_lat: 38.78371799775802,
        merchant_long: -90.5008508851079
    };
    
    let approved = processor.process_transaction(transaction);

    println!("Approved: {0}", approved);
}
