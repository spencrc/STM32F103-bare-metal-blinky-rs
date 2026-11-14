use std::env;
use std::fs;
use std::path::Path;

const LD_NAME: &str = "STM32F103";

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    fs::copy(
        &Path::new(&format!("{}/{}.ld", root_dir, LD_NAME)),
        &Path::new(&format!("{}/{}.ld", out_dir.display(), LD_NAME)),
    )
    .unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed={}.ld", LD_NAME);
}
