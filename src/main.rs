use std::collections::HashMap;
use std::fs;
use crate::{processing::{Processor, User}, transaction::Transaction};

mod transaction;
mod processing;

fn main() {
    let mut processor = Processor {
        users: HashMap::new()
    };

    let contents = fs::read_to_string("data/users.json").expect("Should have read file");
    let users: Vec<User> = serde_json::from_str(&contents).unwrap();

    for user in users {
        processor.users.insert(1, user);
    }

    let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    let transactions: Vec<Transaction> = serde_json::from_str(&contents).unwrap();
    
    let approved = processor.process_transaction(&transactions[0]);

    println!("Approved: {0}", approved);

    
}
