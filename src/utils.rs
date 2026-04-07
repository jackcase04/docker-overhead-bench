use crate::processing::Processor;
use crate::structs::Config;
use crate::structs::RiskLevel;
use crate::structs::Transaction;
use crate::structs::User;

use std::sync::Arc;
use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

const LAT: u64 = 5;

pub fn parse_args_server() -> String {
    let result: String;
    
    match env::args().nth(1) {
        Some(val) => result = val,
        None => result = String::from("0.0.0.0")
    }

    result   
}

pub fn parse_args() -> (u32, u32, Option<u32>, Option<String>) {
    let iterations: u32 = env::args()
        .nth(1)
        .expect("Expected iterations argument")
        .trim()
        .parse()
        .expect("Iterations must be a u32");

    let concurrency: u32 = env::args()
        .nth(2)
        .expect("Expected concurrency argument")
        .trim()
        .parse()
        .expect("Concurrency must be a u32");

    let trial: Option<u32>;

    match env::args().nth(3) {
        Some(val) => trial = Some(val.trim().parse().expect("Trial must be u32")),
        None => trial = None,
    }

    let environment: Option<String>;

    match env::args().nth(4) {
        Some(val) => environment = Some(val),
        None => environment = None,
    }

    (iterations, concurrency, trial, environment)
}

pub fn init_processor() -> Processor {
    let mut processor = Processor {
        users: HashMap::new(),
    };

    let contents = fs::read_to_string("data/users.json").expect("Should have read file");
    let users: Vec<User> =
        serde_json::from_str(&contents).expect("Should have parsed users correctly");

    let mut i = 1;
    for user in users {
        processor.users.insert(i, user);
        i = i + 1;
    }

    processor
}

pub fn init_transactions() -> Vec<Transaction> {
    let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    let transactions: Vec<Transaction> =
        serde_json::from_str(&contents).expect("Should have parsed transactions");

    transactions
}

pub fn init_config(iterations: u32, concurrency: u32) -> Config {
    let config = Config {
        iterations: iterations,
        concurrency: concurrency,
        address: String::from("127.0.0.1:7878"),
    };

    config
}

pub fn handle_connection(mut stream: TcpStream, proc: Arc<Processor>) {
    thread::sleep(Duration::from_millis(LAT));

    let mut data = String::new();

    let _ = stream.read_to_string(&mut data);

    let transaction: Transaction =
        serde_json::from_str(&data).expect("Should have parsed single transaction");

    let approved: RiskLevel = proc.process_transaction(&transaction);

    let data = approved.to_string().into_bytes();

    let _ = stream.write_all(&data);
}

pub fn send_transaction(conf: Arc<Config>, data: Vec<u8>) {
    let mut stream = TcpStream::connect(&conf.address).expect("Should have connected to address");
    let _ = stream.write_all(&data);
    
    let _ = stream
        .shutdown(std::net::Shutdown::Write);

    let mut data = String::new();
    let _ = stream.read_to_string(&mut data);

    // println!("Result: {0}", data);
}
