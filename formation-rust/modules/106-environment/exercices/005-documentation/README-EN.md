# Exercise 005 -- Writing documentation

The pre-package cargo project for this exercise is a library.
In the source folder (`src`), entry point is the `lib.rs` file.

## Step 1 -- module documentation

Write a small documentation for the root module at the start of the `lib.rs` file.

* Use the  `//!` prefix for documentation comment

Build the documentation with the command `cargo doc`.

Take a look at the documentation at `target/doc/exercice_005/index.html`.

## Step 2 -- document the function `safe_integer_divide`

`lib.rs` contains the definition of the `safe_integer_divide` function.
Write a documentation starter describing what the function does.

* Use the `///` prefix for symbol documentation

Build the documentation with the command `cargo doc` et regarder ce qui a été produit.

## Step 2 -- usage example

Supplement the documentation of `safe_integer_divide` with an example of how to use it.

* Create a new `Examples` section
* Use an assertion (`assert!`, etc.)

Build the documentation with the command `cargo doc` et regarder ce qui a été produit.

Execute the corresponding test with `cargo test`.

## Step 3 -- document error handling

Supplement the documentation of `safe_integer_divide` with a simple description of the function's output in case of errors.
Write an example of usage in the documentation to show how to handle the error.

Build the documentation with the command `cargo doc` et regarder ce qui a été produit.

Execute the corresponding test with `cargo test`.

## Step 4 -- hyperlinks in the documentation

In the module's documentation (starting with `//!`), add a hyperlink linking to the documentation for the `safe_integer_divide` function.

* In Markdown, a link is written as `[visible text](url)`.
* Look into `target/doc/exercice_005` to find the link's target.

Build the documentation with the command `cargo doc` et regarder ce qui a été produit.
