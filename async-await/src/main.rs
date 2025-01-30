/**
 * Windows: set RUST_LOG=info && cargo run
 * Linux: RUST_LOG=info cargo run
 */

use tokio::task;
use tokio::time::{sleep, Duration};
use rand::Rng;
use log::info;

async fn random_processing_task(id: usize) {
    //let mut rng = rand::rng(); // future returned by `random_processing_task` is not `Send`
    //let wait_time = rng.random_range(2..=10);
    let wait_time = {
        let mut rng = rand::rng();
        rng.random_range(2..=10)
    };

    sleep(Duration::from_secs(wait_time)).await;

    info!("Task {} completed after {} seconds", id, wait_time);
}

async fn start() {
    env_logger::init();
    info!("Application starting...");

    let mut tasks = vec![];

    for i in 0..1000 {
        tasks.push(task::spawn(random_processing_task(i)));
    }

    for t in tasks {
        let _ = t.await;
    }

    info!("Application shutting down...");
}

#[tokio::main]
async fn main() {
    start().await;
}
