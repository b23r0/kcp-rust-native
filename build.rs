extern crate bindgen;

use std::env;
use std::path::{PathBuf, Path};
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=kcp");
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let fulldir = Path::new(&dir).join("kcp");

    Command::new("cc").args(&["-c" , "ikcp.c", "-o" , "libkcp.o"])
                        .current_dir(fulldir.clone())
                        .status().unwrap();

    Command::new("ar").args(&["rcs", "-o", "libkcp.a", "libkcp.o"])
                        .current_dir(fulldir.clone())
                        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", fulldir.display());
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}