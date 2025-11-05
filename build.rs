// Build script for linking SunVox library

use std::env;
use std::path::PathBuf;

fn main() {
    // Get the path to the SunVox library
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let base_path = PathBuf::from(&manifest_dir)
        .join("sunvox_lib")
        .join("sunvox_lib");

    // Detect platform and set appropriate library path
    let (lib_path, lib_name) = if cfg!(target_os = "macos") {
        let arch = if cfg!(target_arch = "aarch64") {
            "lib_arm64"
        } else {
            "lib_x86_64"
        };
        (base_path.join("macos").join(arch), "sunvox.dylib")
    } else if cfg!(target_os = "linux") {
        (base_path.join("linux").join("lib_x86_64"), "libsunvox.so")
    } else if cfg!(target_os = "windows") {
        (base_path.join("windows").join("lib_x86_64"), "sunvox.dll")
    } else {
        panic!("Unsupported platform");
    };

    // Tell cargo to look for libraries in the SunVox directory
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Tell cargo to link the sunvox library
    println!("cargo:rustc-link-lib=dylib=sunvox");

    // Add rpath so the library can be found at runtime (Unix-like systems)
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    // Rerun if the library changes
    println!("cargo:rerun-if-changed={}/{}", lib_path.display(), lib_name);
}
