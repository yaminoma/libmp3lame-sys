extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=mp3lame");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate libmp3lame bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("Environment variable `OUT_DIR' is not defined"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write libmp3lame bindings");
}
