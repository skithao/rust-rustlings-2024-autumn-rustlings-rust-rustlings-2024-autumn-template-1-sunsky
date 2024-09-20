// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(tx: mpsc::Sender<u32>, qc: Arc<Queue>) {
    // Clone the sender for the first thread
    let tx_for_first_half = tx.clone();
    let qc1 = Arc::clone(&qc);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx_for_first_half.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Clone the sender for the second thread
    let tx_for_second_half = tx.clone();
    let qc2 = Arc::clone(&qc);

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx_for_second_half.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;
    let qc = Arc::new(queue);

    // Pass the sender to the function
    send_tx(tx, qc);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}