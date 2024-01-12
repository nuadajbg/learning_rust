# Exercise 003 -- Call a C function from Rust

## Objective

This exercise is a workspace containing:

* `libgeo`, small C library for geographical coordinates.
* `libgeo-rust`, Rust library to embed its own version of the C libray (assuming you own the sources.)
* `libgeo-wrapper`, Rust library wrapping a pre-compiled `libgeo`.

The goal is to encapsulate the `libgeo` library with two different approaches so that it can be called from Rust code.

## Step 1 -- `libgeo-rust`

In this step we concentrate on `libgeo-rust` only.
This is a Rust library with some of its content written in C.
Thus, it contains the C source files :

> * libgeo-rust
>    * Cargo.toml
>    * src
>        * libgeo.h
>        * libgeo.c
>        * lib.rs

### Step 1.1

* Take a look at the *.h and *.c sources to get the hang of it.
* Look at the tests defined in `lib.rs` to see what kind of Rust-ish interface we would like to expose.

### Step 1.2

* Configure the project to build the embedded C sources.

In the `Cargo.toml` of `libgeo-rust`, add to the `[package]` section, the specifiction of a `build.rs` script:

```toml
build = "build.rs"
```

We will also need some build-time dependencies for this script :

```toml
[build-dependencies]
bindgen = "0.59"
cc = "1"
```

### Step 1.3

In the `build.rs` file :

* Import the `bindgen` and `cc` dependencies.
* In a `main()` function, use `cc` to compile `libgeo.c`.
    * Look at [the `cc` documentation](https://docs.rs/cc/latest/cc/).
* In the same function, generate the bindings from `libgeo.h` using `bindgen`.
    * Look at [the `bindgen` documentation](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.header).
* Then, make sure that the generated bindings are written in a file named `bindings.rs` that should be placed in the output folder of cargo, known at the script's runtime in the `OUT_DIR` environment variable.

### Step 1.4

Fill in the `lib.rs` file by including the generated bindings using :

```rust
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

Compile and test with `cargo test`. What's happening?

### Step 1.5

Solve the problems identified at the previous step by:

* including the bindings in an internal module of the library,
* writing at the crate's root a clean Rust interface on top of this internal module.

Compile and test with `cargo test`.



## Step 2 -- `libgeo-wrapper`

In this second step, we concentrate on `libgeo-wrapper` .
The objective is to provide a clean Rust API on top a (presumably) existing C library, having only access to a binary and the headers. The sources for the C library are in `libgeo`.

### Step 2.1

* Build the library in `libgeo` by running the `build.sh` script within. This should produce a static library libgeo.a and a dynamic library libego.so on Linux. Adapt for your OS as required.

### Step 2.2

* Configure the project for the automatic generation of the bindings at build time.

In the `Cargo.toml` of `libgeo-wrapper`, add to the `[package]` section, the specifiction of a `build.rs` script:

```toml
build = "build.rs"
```

We will also need some build-time dependencies for this script :

```toml
[build-dependencies]
bindgen = "0.59"
```

### Step 2.3

In the `build.rs` file:

* Import `bindgen`.
* In a `main()` function, generate the bindings from `wrapper.h` using `bindgen`.
    * Look at [the `bindgen` documentation](https://docs.rs/bindgen/latest/bindgen/struct.Builder.html#method.header).
* Make sure that the generated bindings are written in a file named `bindings.rs` that should be placed in the output folder of cargo, known at the script's runtime in the `OUT_DIR` environment variable.
* Still in the `main` function, add the required output to configure `cargo` and `rustc` to refer to the binary C library `libgeo`.

```rust
// specify a folder to look into for native libraries
println!("cargo:rustc-link-search=native={}", location);
// Add a reference to the static library `libgeo`
println!("cargo:rustc-link-lib=static=geo");
```

> In this code, `location` must point to the `libgeo` folder.
> Use the environment variable `CARGO_MANIFEST_DIR` that gives the path to the folder that contains the `Cargo.toml` for the current crate, `libgeo-wrapper` in this case.

### Step 2.4

* Use the results from step 1.5 to fill in the gaps in `lib.rs`
* Compile and test with `cargo test`.
