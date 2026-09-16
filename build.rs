#[path = "src/css/definitions.rs"]
mod definitions;

fn main() {
    println!("cargo:rerun-if-changed=src/css/definitions.rs");

    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Generate the embedded copy independently so ferriswatch-generate-css can bootstrap a
    // missing tracked stylesheet. The commit/CI check verifies the asset copy.
    std::fs::write(output.join("default.css"), definitions::render()).unwrap();
}
