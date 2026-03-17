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

    let mut i = 0;
    for user in users {
        processor.users.insert(i, user);
        i = i + 1;
    }

    // This will be changed to be read over the network from the load generator side
    // For now for testing, just read it in from disk
    let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    let transactions: Vec<Transaction> = serde_json::from_str(&contents).unwrap();
    
    let approved = processor.process_transaction(&transactions[0]);

    println!("Users: {0}", processor.users.len());
    println!("Approved: {0}", approved);

    
}
