use rand::Rng;
use std::{
    sync::{Arc, mpsc::channel},
    thread::{self},
    time::{Duration, Instant},
};

use docker_overhead_bench::utils::{
    init_transactions, parse_args, send_transaction, write_results,
};

fn main() {
    let (iterations, concurrency, trial, environment) = parse_args();

    let transactions = Arc::new(init_transactions());
    let mut results: Vec<(Instant, Duration)> = Vec::new();
    let start = Instant::now();

    let (sender, receiver) = channel::<(Instant, Duration)>();

    let mut threads_sent = 0;
    let mut threads_finished = 0;

    while threads_sent < concurrency {
        let sender = sender.clone();

        let random = rand::thread_rng().gen_range(0..30) as usize;
        let trans = transactions[random].clone();

        thread::spawn(move || {
            let data: Vec<u8> =
                serde_json::to_vec(&trans).expect("Should have parsed transaction properly");

            let start = Instant::now();
            send_transaction(data);
            sender
                .send((start, start.elapsed()))
                .expect("Should have sent result");
        });

        threads_sent += 1;
    }

    while threads_finished < iterations {
        let result = receiver.recv().expect("Should have receieved result");
        results.push(result);
        threads_finished += 1;

        if threads_sent < iterations {
            let sender = sender.clone();

            let random = rand::thread_rng().gen_range(0..30) as usize;
            let trans = transactions[random].clone();

            thread::spawn(move || {
                let data: Vec<u8> =
                    serde_json::to_vec(&trans).expect("Should have parsed transaction properly");

                let start = Instant::now();
                send_transaction(data);
                sender
                    .send((start, start.elapsed()))
                    .expect("Should have sent result");
            });

            threads_sent += 1;
        }
    }

    if let Some(trial) = trial
        && let Some(environment) = environment
    {
        write_results(start, results, concurrency, trial, environment);
    }
}
