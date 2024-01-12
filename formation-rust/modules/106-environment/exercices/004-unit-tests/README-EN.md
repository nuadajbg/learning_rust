# Exercise 004 -- Writing unit tests

The pre-package cargo project for this exercise is a library.
In the source folder (`src`), entry point is the `lib.rs` file.

## Step 1 -- simple tests

Write two unit tests for the `is_even` function in `src/lib.rs`.

* Use the `#[test]` annotation on the test functions.
* Use the macros `assert_eq!(x, y)` and `assert!(xx)` for writing test assertions.

Execute the tests with `cargo test`.

## Step 2 -- test module

Still in `lib.rs`, define a test-specific module annotated with `#[cfg(test)]`.

In this new module, write an `is_odd` function on the model of the `is_even` function.
Then, write two unit tests for the `is_odd` function in the new module.

Execute the tests with `cargo test`.

## Step 3 -- integration tests

At the project's root, create a new `tests` folder and a `.rs` file within it. For example `tests/more.rs`.

In this new file, add an import for the `is_even` function.

```rust
use exercice_unit_tests::is_even;
```

Write new test functions for `is_even` in this new file (or copy here the existing one).

Execute the tests with `cargo test`.

## Step 4 -- failing tests

Modify a test to make it fail.

Execute the tests with `cargo test`.

(don't forget to make it work again)

## Step 5 -- output in tests

Modify a test to display the return value of `is_even` or `is_odd` with :

```rust
println!("value of is_even: {}", value);
```

Execute the tests with `cargo test`.

What do you notice?

Execute the tests with `cargo test -- --nocapture`.

## Step 6 -- selecting tests to execute

Execute tests with name containing `xxx` with `cargo test xxx`.

## Step 7 -- tests with `should_panic`

In the `lib.rs` file, write two unit tests for the `must_be_more_than_ten` function.
One of them for tests the function with the value 10 and the other with the value 11.

* Use the annotation `#[should_panic]` on the test for the value 10.

Execute the tests with `cargo test`, try with and without `#[should_panic]`.

## Step 8 -- tests that return `Result<T,E>`

In the `lib.rs` file, write tow unit tests for the `safe_integer_divide` function.

* One test for a=4, b=2
* One test for a=4, b=0
* Test functions must return `Result<(), String>` :

```rust
#[test]
fn safe_integer_divide_ok() -> Result<(), String> { /* TODO: complete here */ }
```
