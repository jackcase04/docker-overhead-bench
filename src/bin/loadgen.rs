use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let contents: Vec<u8> = fs::read("data/transactions.json").expect("Should have read file");

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    stream.write(&contents);

    
}