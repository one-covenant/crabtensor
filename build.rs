use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let metadata_path = Path::new(&out_dir).join("metadata.rs");

    // Use pre-generated metadata instead of fetching from network
    // This avoids metadata version compatibility issues
    let source_path = "metadata_finney.rs";

    fs::copy(source_path, &metadata_path)
        .expect("Failed to copy pre-generated metadata. Make sure metadata_finney.rs exists.");

    println!(
        "cargo:warning=Using pre-generated metadata from {}",
        source_path
    );
    println!("cargo:rerun-if-changed={}", source_path);
}
