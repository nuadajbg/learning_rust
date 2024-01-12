use std::env;
use std::path::PathBuf;

fn main() {
    // compile the C library
    cc::Build::new().file("src/libgeo.c").compile("geo");
    // generates the bindings for the header
    let bindings = bindgen::Builder::default()
        .header("src/libgeo.h")
        .generate()
        .expect("Unable to generate bindings");
    // get the path to the out directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // write the bindings
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
