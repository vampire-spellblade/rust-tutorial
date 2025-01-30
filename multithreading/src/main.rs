/**
 * Windows: set RUST_LOG=info && cargo run
 * Linux: RUST_LOG=info cargo run
 */

use std::thread;
use std::time::Duration;
use rand::Rng;
use log::info;

fn random_processing_handle(id: usize) {
    let mut rng = rand::rng();
    let wait_time = rng.random_range(2..=10);

    thread::sleep(Duration::from_secs(wait_time));

    info!("Task {} completed after {} seconds", id, wait_time);
}

fn start() {
    env_logger::init();
    info!("Application starting...");

    let mut handles = vec![];

    for i in 0..1000 {
        handles.push(thread::spawn(move || random_processing_handle(i)));
    }

    for h in handles {
        h.join().unwrap();
    }

    info!("Application shutting down...");
}

fn main() {
    start();
}
