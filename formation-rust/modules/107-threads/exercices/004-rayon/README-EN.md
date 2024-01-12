# Exercise 004 -- Data parallelism with `rayon`

## Etape 1 -- Concurrent tasks

Use [`rayon`](https://docs.rs/rayon/) to execute 100 concurrent tasks defined as:

```rust
thread::sleep(Duration::from_millis(100));
println!("Finished item {}", i); // i is the task number
```

> bonus : how many threads did rayon used?

## Etape 2 -- With a result

* Build a vector of numbers from 0 to 99.
* Use [`rayon`](https://docs.rs/rayon/) to compute the square of each number in parallel.
* Get all the results in a vector, in the same order.
