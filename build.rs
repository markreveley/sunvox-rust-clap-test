// Build script for linking SunVox library

use std::env;
use std::path::PathBuf;

fn main() {
    // Get the path to the SunVox library
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(&manifest_dir)
        .join("sunvox_lib")
        .join("sunvox_lib")
        .join("linux")
        .join("lib_x86_64");

    // Tell cargo to look for libraries in the SunVox directory
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Tell cargo to link the sunvox library
    println!("cargo:rustc-link-lib=dylib=sunvox");

    // Add rpath so the library can be found at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    // Rerun if the library changes
    println!("cargo:rerun-if-changed={}/sunvox.so", lib_path.display());
}
