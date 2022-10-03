fn main() {
    // Link to Raylib library
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rerun-if-changed=src/main.rs");
}
