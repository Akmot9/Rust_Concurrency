use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;
use std::time::Instant;

#[tokio::main]
pub async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter_clone = counter.clone();
        let handle = task::spawn(async move {
            let start = Instant::now();
            println!("Tentative de verrouillage du Mutex");
            let mut num = counter_clone.lock().await;
            let end = Instant::now();
            println!("Mutex verrouillé, attente: {:?}", end - start);

            for _ in 0..100000 {
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Valeur finale: {}", *counter.lock().await);
}
