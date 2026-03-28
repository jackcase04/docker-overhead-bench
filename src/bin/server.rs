use docker_overhead_bench::{
    utils::{handle_connection, init_processor}
};

use std::{
    net::TcpListener, 
    thread,
    sync::Arc
};

fn main() { 

    let processor = Arc::new(init_processor());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        let proc = Arc::clone(&processor);

        thread::spawn(move || {
            handle_connection(stream, proc);
        }); 
    }    
}