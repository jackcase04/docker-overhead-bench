use std::{
    thread,
    thread::JoinHandle,
    sync::Arc 
};

use docker_overhead_bench::{
    utils::{init_config, init_transactions, send_transaction}
};

fn main() {
    
    let transactions = Arc::new(init_transactions());
    let config= Arc::new(init_config());  

    for _i in (0..config.iterations).step_by(config.concurrency as usize) {
        let mut handles: Vec<JoinHandle<()>> = Vec::new();

        for _j in 0..config.concurrency {
            let conf = Arc::clone(&config);
            let trans = transactions[0].clone();

            let handle = thread::spawn(move || {
                send_transaction(conf, trans); 
            });

            handles.push(handle);

        }

        for handle in handles {
            handle.join().unwrap();
        }
    } 
}