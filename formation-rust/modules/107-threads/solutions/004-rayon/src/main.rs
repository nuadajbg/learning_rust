use rayon::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let input: Vec<u64> = (0..100).collect();
    input.par_iter().for_each(|i| {
        thread::sleep(Duration::from_millis(100));
        println!("Finished item {}", i);
    });
}
