# Exercise 001 -- Launch multiple threads

In the presentation we have seen an example for launching threads:

```rust
use std::thread;
thread::spawn(|| {});
```

## Step 1

First, write a loop to launch 10 threads numbered from 0 to 9.
* Thread 0 must wait 0 milli-seconds before printing `Thread 0 has waited 0 ms`
* Thread 1 must wait 100 milli-seconds before printing `Thread 1 has waited 100 ms`
* etc.


Use the function [`thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html) to wait.

* Build and execute with `cargo run`.
* What do you see?

## Step 2

Wait for the termination of all threads using [`join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join).

* Accumulate all thread handles in a vector.
* Then, consume the handles with [`join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join) to wait for the termination of all threads.
    * Use [`into_iter()`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter) to iterate on the vector, taking the ownership of all elements successively.
