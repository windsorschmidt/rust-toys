use std::thread;
use std::sync::mpsc;

/// Spawn threads and synchronize using channels (one rx, many tx).
///
pub fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let tx = tx.clone();

        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }
}
