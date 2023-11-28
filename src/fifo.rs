use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    // Création d'un canal pour les messages
    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0));

    // Création de threads producteurs
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let message = format!("Message {} du thread", i);
            for _ in 0..100000 {
                tx_clone.send(message.clone()).unwrap();
                //println!("Thread {} a envoyé un message", i);
            }

        });
    }

    let counter_clone = Arc::clone(&counter);
    // Création d'un thread consommateur
    thread::spawn(move || {
        for received in rx {
            //println!("Traitement : {}", received);
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // Simuler le traitement du message
            //thread::sleep(Duration::from_millis(500));
        }
    });

    // Attente pour la démonstration (dans la pratique, utilisez un mécanisme de synchronisation)
    thread::sleep(Duration::from_secs(1));
    println!("Nombre de messages traités: {}", *counter.lock().unwrap());
}
