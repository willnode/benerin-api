use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let dir_var = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir_var);

    let libpostal_dir = dir.join("libpostal").trim();

    // Check if libpostal directory exists
    assert!(Path::new(libpostal_dir).exists(), "libpostal directory does not exist");

    // Set PKG_CONFIG_PATH to include the libpostal installation path
    let libpostal_lib_dir = format!("{}/src/.libs", libpostal_dir);
    let libpostal_include_dir = format!("{}/src", libpostal_dir);
    let current_pkg_config_path = env::var("PKG_CONFIG_PATH").unwrap_or_else(|_| String::new());
    let new_pkg_config_path = if current_pkg_config_path.is_empty() {
        libpostal_lib_dir.clone()
    } else {
        format!("{}:{}", libpostal_lib_dir, current_pkg_config_path)
    };
    env::set_var("PKG_CONFIG_PATH", &new_pkg_config_path);

    // Ensure libpostal can be found by the linker
    println!("cargo:rustc-link-search=native={}", libpostal_lib_dir);
    println!("cargo:rustc-link-lib=static=postal");

    // Add include path for headers
    println!("cargo:include={}", libpostal_include_dir);

    // Print any other relevant information for the build
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build_libpostal.sh");
}
