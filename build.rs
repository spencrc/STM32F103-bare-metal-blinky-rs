use std::env;
use std::fs;
use std::path::Path;

const LD_NAME: &str = "STM32F103";
const STARTUP_NAME: &str = "STM32F103_startup";

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    fs::copy(
        &Path::new(&format!("{}/startup/{}.ld", root_dir, LD_NAME)),
        &Path::new(&format!("{}/{}.ld", out_dir.display(), LD_NAME)),
    ).unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg=-T{}.ld", LD_NAME);
    println!("cargo:rerun-if-changed=startup/{}.ld", LD_NAME);

    cc::Build::new().file(&format!("startup/{}.c", STARTUP_NAME)).compile(STARTUP_NAME);
    println!("cargo::rerun-if-changed=startup/{}.c", STARTUP_NAME);
}
