use hello_rust::line;
use hello_rust::tx;
use hello_rust::tx::Tx;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

type SharedCounter = Arc<AtomicU64>;
type SharedTxMempool = Arc<Mutex<Vec<Tx>>>;

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    // initialize shared amount
    let shared_counter = Arc::new(AtomicU64::new(0));

    // initialize channel
    let (sender, receiver) = mpsc::channel::<Tx>(100);

    // initialize mempool
    let mempool: SharedTxMempool = Arc::new(Mutex::new(Vec::new()));

    // producer
    let mempool_producer = mempool.clone();
    tokio::spawn(async move {
        let mut amount = 100;
        loop {
            amount += 1;
            let tx = Tx {
                from: "Alice".to_string(),
                to: "Bob".to_string(),
                amount: amount,
            };
            if sender.send(tx).await.is_err() {
                println!("producer channel is full");
                break;
            }
            // println!("producer tick");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    // consumer
    let shared_counter_consumer = shared_counter.clone();
    let mempool_consumer = mempool.clone();
    tokio::spawn(async move {
        let mut receiver = receiver; // make receiver mutable
        while let Some(tx) = receiver.recv().await {
            println!("received tx: {:?}", tx);
            shared_counter_consumer.fetch_add(1, Ordering::Relaxed);
            let mut mempool = mempool_consumer.lock().await;
            mempool.push(tx);
        }
        // println!("consumer tick");
        tokio::time::sleep(Duration::from_millis(800)).await;
    });

    // monitor
    let shared_counter_monitor = shared_counter.clone();
    let mempool_monitor = mempool.clone();
    tokio::spawn(async move {
        loop {
            {
                let mempool = mempool_monitor.lock().await;
                println!("mempool size: {}", mempool.len());
            }
            let counter = shared_counter_monitor.load(Ordering::Relaxed);
            let elapsed = start_time.elapsed();
            let tps = counter as f64 / elapsed.as_secs_f64();
            println!("shared counter: {counter}, tps: {tps:.2}");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::time::sleep(Duration::from_secs(5)).await;
}
