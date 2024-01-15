use std::env;
use std::path::PathBuf;

fn main() {
    // generates the bindings for the header
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    // get the path to the out directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // write the bindings
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // get the path to the project's location
    let location = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // sets the options for loading the library
    println!(
        "cargo:rustc-link-search=native={}/../libgeo",
        location.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=geo");
}
