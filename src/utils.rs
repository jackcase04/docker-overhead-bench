use crate::processing::Processor;
use crate::structs::Config;
use crate::structs::RiskLevel;
use crate::structs::Transaction;
use crate::structs::User;

use std::sync::Arc;
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

const LAT: u64 = 5;

pub fn init_processor() -> Processor {
    let mut processor = Processor {
        users: HashMap::new(),
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

pub fn init_transactions() -> Vec<Transaction> {
    let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    let transactions: Vec<Transaction> = serde_json::from_str(&contents).unwrap();

    transactions
}

pub fn init_config() -> Config {
    let raw = std::fs::read_to_string("config.toml").expect("Should have read file");
    let config: Config = toml::from_str(&raw).expect("invalid config");

    config
}

pub fn handle_connection(mut stream: TcpStream, proc: Arc<Processor>) {
    thread::sleep(Duration::from_millis(LAT));

    let mut data = String::new();

    let _ = stream.read_to_string(&mut data);

    println!("Data: {0}", data);

    let transaction: Transaction = serde_json::from_str(&data).unwrap();

    let approved: RiskLevel = proc.process_transaction(&transaction);

    println!("Users: {0}", proc.users.len());
    println!("Result: {:?}", approved);

    let data = approved.to_string().into_bytes(); 

    let _ = stream.write_all(&data);
}

pub fn send_transaction(conf: Arc<Config>, trans: Transaction) {
    let mut stream = TcpStream::connect(&conf.address).unwrap();
    let data: Vec<u8> = serde_json::to_vec(&trans).unwrap();
    let _ = stream.write_all(&data);
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut data = String::new(); 

    let _ = stream.read_to_string(&mut data);

    println!("Result: {0}", data);
}
