Concurrency refers to the ability of a system to manage multiple tasks simultaneously. It's a concept used in computer science to enable efficient execution of processes or threads within a program, particularly in the context of systems with multiple CPUs or cores. Concurrency aims to improve the throughput and computational speed of a system by utilizing resources more effectively.


Example of Concurrency in C
```c
#include <stdio.h>   // Inclut la bibliothèque standard d'entrée/sortie.
#include <pthread.h> // Inclut la bibliothèque pthread pour la programmation multi-thread.

#define NUM_THREADS 10   // Définit le nombre de threads à créer.
#define NUM_INCREMENTS 1000 // Définit le nombre d'incrémentations par thread.

int shared_variable = 0; // Déclare une variable partagée entre tous les threads.

// Fonction exécutée par chaque thread.
void *increment(void *arg) {
    for (int i = 0; i < NUM_INCREMENTS; i++) {
        shared_variable++; // Incrémente la variable partagée.
    }
    return NULL; // Termine la fonction du thread.
}

int main() {
    pthread_t threads[NUM_THREADS]; // Tableau pour stocker les identifiants de threads.

    // Création des threads.
    for (int i = 0; i < NUM_THREADS; i++) {
        // Crée un nouveau thread qui exécutera la fonction 'increment'.
        if (pthread_create(&threads[i], NULL, increment, NULL) != 0) {
            perror("Failed to create thread"); // Affiche un message d'erreur si la création échoue.
            return 1; // Termine le programme avec un code d'erreur.
        }
    }

    // Boucle pour attendre la fin de tous les threads.
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL); // Attend que le thread 'i' se termine.
    }

    // Affiche le résultat final de la variable partagée.
    printf("Résultat final : %d\n", shared_variable);
    return 0; // Termine le programme avec succès.
}```



This C code is a simple example of concurrent programming using POSIX Threads (pthreads). It creates multiple threads, each executing the increment function to modify a shared variable.

Limitations and Issues

```bash

└─(09:55:48 on master ✭)──> ./src/main                                               ──
Résultat final : 10000
┌─(~/Documents/rust_project/mutex_tuto)──────────────(usr)─┐
└─(09:55:48 on master ✭)──> ./src/main                                               ──
Résultat final : 9763
┌─(~/Documents/rust_project/mutex_tuto)──────────────(usr)─┐
└─(09:55:49 on master ✭)──> ./src/main                                               ──
Résultat final : 10000
```

The primary issue in your C code example is the lack of synchronization when accessing the shared variable (shared_variable). This leads to a race condition, a type of concurrency bug where the output depends on the sequence or timing of other uncontrollable events (in this case, thread scheduling).
Race Condition:

    - Cause: Multiple threads increment the shared variable without any mechanism to prevent simultaneous access.

    - Effect: The final value of shared_variable is unpredictable and often incorrect, as seen in the varying results of your program executions. Sometimes it may be 10000 (which is the expected result), other times it's less, indicating some increments were lost.

# how rust handles concurrent access
Rust's approach to handling concurrent access is fundamentally different from that of C. The Rust compiler, through its strict ownership and borrowing rules, inherently disallows the kind of unsynchronized access to shared data seen in the C code example. Instead, Rust mandates the use of safe concurrency primitives, such as mutexes, for managing access to shared resources, thereby ensuring that concurrent access is handled in a controlled and safe manner. This design enforces thread safety and prevents data races, which are common issues in traditional concurrent programming in languages like C. 

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            for _ in 0..1000 {
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
```

In this Rust example, the Arc (Atomic Reference Count) and Mutex (mutual exclusion) are used to safely share and modify data across multiple threads:

    - Arc: A thread-safe reference-counting pointer. Arc is used to wrap the shared data, allowing multiple threads to own a reference to the data.
    
    - Mutex: Ensures that only one thread at a time can access the shared data. When a thread wants to read or write the shared data, it must first acquire the mutex's lock.

This design pattern effectively prevents race conditions and ensures that the shared data is accessed in a controlled and safe manner, a sharp contrast to the C example where such safety is not enforced by the language.

# Locking time for shared data

```rust
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
```

```rust
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
```

```bash
Asynchronous mutex:
Tentative de verrouillage du Mutex
Tentative de verrouillage du Mutex
Mutex verrouillé, attente: 8.748µs
Tentative de verrouillage du Mutex
Mutex verrouillé, attente: 828.275µs
Mutex verrouillé, attente: 1.617312ms
Valeur finale: 300000

Simple mutex:
Tentative de verrouillage du Mutex
Mutex verrouillé, attente: 5.128µs
Tentative de verrouillage du Mutex
Tentative de verrouillage du Mutex
Mutex verrouillé, attente: 828.515µs
Mutex verrouillé, attente: 1.60011ms
Valeur finale: 300000
```
# fifo
```rust
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
```

Certainly! We can expand the table with additional parameters that are relevant to the evaluation of synchronization mechanisms:

| State                      | C Var Global   | Rust `std::sync::Mutex` | Rust `tokio::sync::Mutex` (async) | Rust FIFO (e.g., channels) |
|----------------------------|----------------|-------------------------|-----------------------------------|----------------------------|
| Guaranteed Ordering        | Yes            | No                      | No                                | Yes                        |
| Queue Data                 | Implicitly     | No                      | No                                | Explicitly                 |
| Potential Starvation       | Yes            | Yes                     | Yes (less with fair scheduling)   | No (with fair scheduling)  |
| Deadlock Potential         | Yes            | Yes                     | Yes                               | Less Likely                |
| Overhead                   | Moderate       | Moderate                | Higher (due to async overhead)    | Varies                     |
| Throughput                 | Low to Moderate| Moderate                | High                              | High                       |
| Fairness                   | Depends on impl| No                      | Yes (if fair scheduling enabled)  | Yes                        |
| Context Switch Required    | Yes            | Yes                     | No (asynchronous)                 | No (asynchronous)          |
| Blocking                   | Yes            | Yes                     | No                                | No                         |
| Suitable for High Contention| No             | Yes                     | Yes                               | Yes                        |
| Ease of Use                | Low            | Moderate                | Moderate to High                  | High                       |

Additional parameters explained:

- **Overhead**: This refers to the resource and performance costs associated with using the synchronization mechanism. Async mutexes tend to have higher overhead due to the complexity of the async runtime.

- **Throughput**: The rate at which work is completed. FIFO queues and async mutexes typically allow higher throughput due to non-blocking behavior and concurrent task handling.

- **Fairness**: Whether the synchronization mechanism ensures that each task gets a fair chance at execution without being starved by others.

- **Context Switch Required**: Whether the mechanism requires the operating system to perform a context switch, which is a significant performance cost. Asynchronous operations typically do not require a context switch because they are managed within the runtime at the user-space level.

- **Blocking**: Indicates whether a thread or task is put to sleep (blocked) while waiting for the resource.

- **Suitable for High Contention**: Whether the mechanism is well-suited for scenarios where there is a high degree of competition for resources.

- **Ease of Use**: The relative simplicity or difficulty of correctly implementing and using the synchronization mechanism in a concurrent programming context.