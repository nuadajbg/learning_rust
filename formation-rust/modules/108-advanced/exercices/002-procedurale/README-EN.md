# Exercise 002 -- Procedural macro

## Objective

The objective in this exercise is to write a procedural macro to automatically derive a trait on a structure.
The trait is as follow:
```rust
/// Trait for structures that can be printed on the console
pub trait Printable {
    /// Prints a description of this structure on the console
    fn print(&self);
}
```

We want to derive it on a a structure such as:

```rust
struct MyStruct {
    a: String,
    b: f32,
}
```

Then call the `print()` method on an instance of this structure, for example :

```rust
let instance = MyStruct {
    a: "something".to_string(),
    b: 42.0,
};
instance.print();
```

The expected output is:

```
a=something
b=42
```

## Step 1 -- Structure

The exercise is compose of sub-projects :

* `mymacro` defines the `Printable` trait.
* `mymacro_derive` defines the macro that we want to write in this exercise.
* `tester` uses the two above libraries and performs some test.

At the root of this exercise, you will also find a `Cargo.toml` file that acts at the definition of a workspace that contains the three sub-projects.
You can invoke `cargo` commands directly at that level.

Read the code already written down.

Try to build with `cargo build` at the exercise's root.

Build should failt at the end with the following error:

```
error[E0599]: no method named `print` found for type `MyStruct` in the current scope
```

## Etape 2 -- Make it compile!

* Modify the function `impl_printable` in `mymacro_derive` so that the whole project compiles correctly.
    * Fill-in the body of the `quote!` macro in order to produce an implemenation of the `Printable` trait. You may leave the implement of the `print` method empty for now. [Documentation of quote](https://docs.rs/quote/)
    * Look at the [documentation of `DeriveInput`](https://docs.rs/syn/latest/syn/struct.DeriveInput.html) to find a way to get the name of the target structure on which the trait is to be implemented.
* Build and execute with:

```sh
./target/debug/exercice_002_tester
```

* The output should be empty.

## Etape 3 -- Fill-in the rest

Finalise the implementation of `impl_printable` and test.

* Look at the [documentation of `DeriveInput`](https://docs.rs/syn/latest/syn/struct.DeriveInput.html) to find a way to find the structure's fields.
* You will probably need
    * [`proc_macro2::TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html)
    * [`quote::TokenStreamExt`](https://docs.rs/quote/latest/quote/trait.TokenStreamExt.html)
        * Methods `append` and `append_all`.
