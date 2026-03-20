use std::collections::HashMap;
use std::fs;
use std::io::Read;
use docker_overhead_bench::processing::Processor;
use docker_overhead_bench::structs::User;
use docker_overhead_bench::structs::Transaction;

use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
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

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");

        let mut data = String::new();

        stream.read_to_string(&mut data);

        println!("Data: {0}", data);

        let transactions: Vec<Transaction> = serde_json::from_str(&data).unwrap();
        
        let approved = processor.process_transaction(&transactions[0]);

        println!("Users: {0}", processor.users.len());
        println!("Approved: {0}", approved);

    }

    // This will be changed to be read over the network from the load generator side
    // For now for testing, just read it in from disk
    // let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    // let transactions: Vec<Transaction> = serde_json::from_str(&contents).unwrap();
    
    // let approved = processor.process_transaction(&transactions[0]);

    // println!("Users: {0}", processor.users.len());
    // println!("Approved: {0}", approved);

    
}