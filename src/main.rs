use std::time::Duration;

use tokio::{
    runtime::{Handle, RuntimeMetrics},
    task::JoinSet,
};

#[tokio::main]
pub async fn main() {
    println!("1. Metrics before submitting any requests:");
    let m = Handle::current().metrics();
    print_tokio_metrics(&m);

    println!("2. Metrics after submitting one request:");
    let client = reqwest::Client::new();
    let _ = client.get("https://rust-lang.org").send().await;
    print_tokio_metrics(&m);

    const N: usize = 20;
    println!("3. Submitting {N} requests...\n");
    let mut handles = JoinSet::new();
    for _ in 0..N {
        handles.spawn(client.get("https://rust-lang.org").send());
    }

    println!("4. Metrics just after submitting all requests:");
    print_tokio_metrics(&m);

    println!("5. Waiting for all requests to complete...\n");

    handles.join_all().await;
    println!("6. Finished receiving all {N} requests!\n");

    println!("7. Metrics just after receiving all requests:");
    print_tokio_metrics(&m);

    println!("8. Waiting 10 seconds for blocking threads to be dropped...\n");

    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("9. Metrics 10 seconds after receiving all requests:");
    print_tokio_metrics(&m);
}

fn print_tokio_metrics(m: &RuntimeMetrics) {
    print!("spawned_tasks:{:>3}, ", m.spawned_tasks_count());
    print!("alive_tasks:{:>3}, ", m.num_alive_tasks());
    print!("blocking_threads:{:>3}\n\n", m.num_blocking_threads());
}
