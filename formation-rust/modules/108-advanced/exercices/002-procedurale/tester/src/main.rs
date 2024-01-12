// import the trait and the derive macro
use mymacro::Printable;
use mymacro_derive::Printable;

// derive the Printable trait for MyStruct
#[derive(Printable)]
struct MyStruct {
    a: String,
    b: f32,
}

/// Main entry point
pub fn main() {
    let instance = MyStruct {
        a: "something".to_string(),
        b: 42.0,
    };
    instance.print();
}
