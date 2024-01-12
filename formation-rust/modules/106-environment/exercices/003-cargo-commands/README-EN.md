# Exercise 003 -- Cargo usage

Use the cargo project created in exercise 002 and execute the following commands:

## Step 1

Compile in release mod with optimisations:

```sh
cargo build --release
```

Execute the release-mod binary:

```sh
./target/release/cargo_hello
```

## Step 2

Delete all cargo output:

```sh
cargo clean
```

Check that the `target` folder has been deleted.

## Step 3

Compile but do not generate the binary:

```sh
cargo check
```

## Step 4

Execute unit tests

```sh
cargo test
```

## Step 5

Generate the documentation:

```sh
cargo doc
```
