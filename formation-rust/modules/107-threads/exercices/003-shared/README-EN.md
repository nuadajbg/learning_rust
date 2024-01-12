# Exercise 003 -- Data sharing between threads

## Etape 1 -- Data sharing with Arc and Mutex

* Spawn 10 threads and wait for their termination on the main thread.
* Each thread is numbered, from 0 to 9.
* In each thread, increment a shared counter using the thread's number.
    * The total must be 45,
    * Use [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) and [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
* In the main thread, after the termination of the 10 spawned threads, print the total.

## Step 2 -- Sharing with AtomicXXX

* Same thing but using a structure from [`atomic`](https://doc.rust-lang.org/core/sync/atomic/index.html), for example `AtomicI32`.
