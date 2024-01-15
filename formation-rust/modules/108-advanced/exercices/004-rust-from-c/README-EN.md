# Exercice 004 -- Call a Rust function from C

## Objective

This exercise is a workspace that contain:

* `libgeo`, a Rust library for geographical coordinates.
* `cuser`, a C program that want to use the Rust lib.

## Step 1

Compile and test `libgeo` with `cargo test`.

## Step 2

* Configure the `libgeo` project to produce a system dynamic library, in `Cargo.toml` :

```toml
[lib]
crate-type=["rlib", "dylib"]
#                   ^^^^^^^ will produce .so in linux
#           ^^^^^^ binary Rust library
```

## Step 3

* Annotate the `GeoCoordinate` structure and the `offset` function so that they can be used from C:
    * Add `#[repr(C)]` on `GeoCoordinate`.
    * Add `#[no_mangle]` or `#[export_name = "offset"]` on `offset`.

Compile with `cargo build`.

## Step 4

* Compile the `cuser` project with the provided `build.sh` within.
* Execute the program by running `execute.sh`.
