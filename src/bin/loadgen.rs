use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use docker_overhead_bench::structs::Transaction;
use serde::{Deserialize, Serialize};

fn main() {
    let contents = fs::read_to_string("data/transactions.json").expect("Should have read file");
    let transactions: Vec<Transaction> = serde_json::from_str(&contents).unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let data: Vec<u8> = serde_json::to_vec(&transactions[0]).unwrap();
    
    stream.write(&data);

    
}