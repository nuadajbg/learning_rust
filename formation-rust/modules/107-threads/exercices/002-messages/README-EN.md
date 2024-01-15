# Exercise 002 -- Passing messages between threads

## Step 1 -- Send a single message between 2 threads

* Use `thread::spawn` to spawn a new thread from the main one.
* Use [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) to create a couple of sender and receiver.
* The spawned thread must wait and received a single message (of type `String`) and print it.
* The main thead must send a single message (`"Hello!"`) and wait for the termination of the spawned thread using `join`.

## Etape 2 -- Pass multiple messages

Modify the previous solution in order to:

* Send 4 messages from the main thread:
    * Each message must be of the form `"Hello X!"`, where X is the message's number
    * Wait 1000 ms between two messages
* The spawned thread must wait for incoming messages and not assume that the number of messages is known.

> The program does not terminate. Determine why.

## Etape 3 -- Termination

Modify the previous solution to ensure the program's termination.
The listening thread must still not assume that the number of messages is known.

> bonus : find multiple solutions.

## Etape 4 -- Multiple senders

Modify the previous solution in order to:

* Spawn a new thread that will also send messages in addition to the main thread.
    * Support the ability for the receiver to known from which thread a message is coming from
    * Look at the documentation of [`Sender`](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html) to find a solution
