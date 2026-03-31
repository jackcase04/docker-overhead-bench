use rand::Rng;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use docker_overhead_bench::utils::{init_config, init_transactions, send_transaction};

fn main() {
    let transactions = Arc::new(init_transactions());
    let config = Arc::new(init_config());
    let mut results: Vec<(Instant, Duration)> = Vec::new();
    let start = Instant::now();

    for _i in (0..config.iterations).step_by(config.concurrency as usize) {
        let mut handles: Vec<JoinHandle<(Instant, Duration)>> = Vec::new();

        for _j in 0..config.concurrency {
            let random = rand::thread_rng().gen_range(0..30) as usize;

            let conf = Arc::clone(&config);
            let trans = transactions[random].clone();

            let handle = thread::spawn(move || {
                let data: Vec<u8> = serde_json::to_vec(&trans).unwrap();
                
                let start = Instant::now();
                send_transaction(conf, data);
                (start, start.elapsed())
            });

            handles.push(handle);
        }

        for handle in handles {
            let result = handle.join().unwrap();
            results.push(result);
        }
    }

    results.sort_by(|a, b| a.0.cmp(&b.0));

    let mut file = BufWriter::new(File::create("results.csv").unwrap());
    writeln!(file, "timestamp_us,duration_us").unwrap();

    for (instant, duration) in &results {
        let timestamp_ns = instant.duration_since(start).as_micros() as u64;
        let duration_ns = duration.as_micros() as u64;
        writeln!(file, "{},{}", timestamp_ns, duration_ns).unwrap();
    }
}
