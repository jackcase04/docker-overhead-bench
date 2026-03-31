use rand::Rng;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Arc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use docker_overhead_bench::utils::{init_config, init_transactions, parse_args, send_transaction};

fn main() {
    let (iterations, concurrency, trial, environment) = parse_args();

    let transactions = Arc::new(init_transactions());
    let config = Arc::new(init_config(iterations, concurrency));
    let mut results: Vec<(Instant, Duration)> = Vec::new();
    let start = Instant::now();

    for _i in (0..config.iterations).step_by(config.concurrency as usize) {
        let mut handles: Vec<JoinHandle<(Instant, Duration)>> = Vec::new();

        for _j in 0..config.concurrency {
            let random = rand::thread_rng().gen_range(0..30) as usize;

            let conf = Arc::clone(&config);
            let trans = transactions[random].clone();

            let handle = thread::spawn(move || {
                let data: Vec<u8> =
                    serde_json::to_vec(&trans).expect("Should have parsed transaction properly");

                let start = Instant::now();
                send_transaction(conf, data);
                (start, start.elapsed())
            });

            handles.push(handle);
        }

        for handle in handles {
            let result = handle
                .join()
                .expect("Couldn't join on the associated thread");
            results.push(result);
        }
    }

    if let Some(trial) = trial
        && let Some(environment) = environment
    {
        results.sort_by(|a, b| a.0.cmp(&b.0));

        let filename = format!("results_{}_{}_{}.csv", concurrency, trial, environment);
        let path = PathBuf::from("csv").join(filename);
        std::fs::create_dir_all("csv").expect("Should have created csv folder");
        
        let mut file = BufWriter::new(File::create(path).expect("Should have created file"));

        for (instant, duration) in &results {
            let timestamp = instant.duration_since(start).as_micros() as u64;
            let duration = duration.as_micros() as u64;
            writeln!(file, "{},{}", timestamp, duration).expect("Couldn't write CSV line");
        }
    }
}
