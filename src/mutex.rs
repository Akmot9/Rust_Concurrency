use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let start = Instant::now();
            println!("Tentative de verrouillage du Mutex");
            let mut num = counter_clone.lock().unwrap();
            let end = Instant::now();
            println!("Mutex verrouillé, attente: {:?}", end - start);

            for _ in 0..100000 {
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Valeur finale: {}", *counter.lock().unwrap());
}
