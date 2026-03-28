use crate::processing::Processor;
use crate::structs::User;
use crate::structs::Transaction;

use std::sync::Arc;
use std::{
    collections::HashMap,
    fs,
    thread,
    time::Duration,
    io::Read,
    net::TcpStream
};

pub fn init_processor() -> Processor {
    let mut processor = Processor {
        users: HashMap::new()
    };

    let contents = fs::read_to_string("data/users.json").expect("Should have read file");
    let users: Vec<User> = serde_json::from_str(&contents).unwrap();

    let mut i = 1;
    for user in users {
        processor.users.insert(i, user);
        i = i + 1;
    }

    processor
}

pub fn handle_connection(mut stream: TcpStream, proc: Arc<Processor>) {
    thread::sleep(Duration::from_secs(5));

    let mut data = String::new();

    let _ = stream.read_to_string(&mut data);

    println!("Data: {0}", data);

    let transaction: Transaction = serde_json::from_str(&data).unwrap();
    
    let approved = proc.process_transaction(&transaction);

    println!("Users: {0}", proc.users.len());
    println!("Approved: {0}", approved);
}