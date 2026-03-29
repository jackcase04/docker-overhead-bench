use std::thread::JoinHandle;
use std::{
    fs,
    thread,
    sync::Arc 
};
use std::io::prelude::*;
use std::net::TcpStream;
use docker_overhead_bench::structs::{
    Transaction,
    Config
};

fn main() {
    let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    let transactions: Vec<Transaction> = serde_json::from_str(&contents).unwrap();

    let raw = std::fs::read_to_string("config.toml").expect("Should have read file");
    let config: Config = toml::from_str(&raw).expect("invalid config");

    let transactions = Arc::new(transactions);
    let config= Arc::new(config);  

    for _i in (0..config.iterations).step_by(config.concurrency as usize) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for _j in 0..config.concurrency {
            let conf = Arc::clone(&config);
            let trans = Arc::clone(&transactions);

            let handle = thread::spawn(move || {
                let mut stream = TcpStream::connect(&conf.address).unwrap();

                let data: Vec<u8> = serde_json::to_vec(&trans[0]).unwrap();
                    
                let _ = stream.write_all(&data);
            });

            handles.push(handle);

        }

        for handle in handles {
            handle.join().unwrap();
        }
    } 
}