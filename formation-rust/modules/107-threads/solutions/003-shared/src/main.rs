use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn main() {
    step2();
}

fn step1() {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<JoinHandle<_>> = (0..10)
        .map(|i| {
            let my_counter = counter.clone();
            thread::spawn(move || {
                let mut c = my_counter.lock().unwrap();
                *c += i;
            })
        })
        .collect();
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    let c = counter.lock().unwrap();
    println!("total={}", c);
}

fn step2() {
    let counter = Arc::new(AtomicI32::new(0));
    let handles: Vec<JoinHandle<_>> = (0..10)
        .map(|i| {
            let my_counter = counter.clone();
            thread::spawn(move || {
                my_counter.fetch_add(i, Ordering::SeqCst);
            })
        })
        .collect();
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    let c = counter.load(Ordering::SeqCst);
    println!("total={}", c);
}