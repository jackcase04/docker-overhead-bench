use docker_overhead_bench::{
    processing::Processor,
    utils::{handle_connection, parse_args_server},
};

use std::{net::TcpListener, sync::Arc, thread};

fn main() {
    let addr: String = String::from(parse_args_server() + ":7878");

    let processor = Arc::new(Processor::new());
    let listener = TcpListener::bind(addr).expect("Server should have bound to addr");

    for stream in listener.incoming() {
        let stream = stream.expect("Should have been able to set stream");
        let proc = Arc::clone(&processor);

        thread::spawn(move || {
            handle_connection(stream, proc);
        });
    }
}
