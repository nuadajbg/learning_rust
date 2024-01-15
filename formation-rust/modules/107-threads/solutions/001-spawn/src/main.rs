use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    step2();
}

fn step1() {
    for i in 0..10 {
        thread::spawn(move || {
            let amount = i * 100;
            thread::sleep(Duration::from_millis(amount));
            println!("Thread {} has waited {} ms", i, amount);
        });
    }
}

fn step2() {
    let handles: Vec<JoinHandle<_>> = (0..10).map(|i| {
        thread::spawn(move || {
            let amount = i * 100;
            thread::sleep(Duration::from_millis(amount));
            println!("Thread {} has waited {} ms", i, amount);
        })
    }).collect();
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}