use std::env;
use std::fs;
use std::path::Path;

const LINKER_DIR_NAME: &str = "linker";

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let linker_dir = format!("{}/{}", root_dir, LINKER_DIR_NAME);

    let linker_scripts = fs::read_dir(linker_dir).unwrap();
    for ld in linker_scripts {
        let ld = ld.unwrap();
        fs::copy(
            ld.path(),
            &Path::new(&format!("{}/{}", out_dir.display(), ld.file_name().display()))
        )
        .unwrap();
        println!("cargo:rerun-if-changed={}", ld.path().display());
    }
    println!("cargo:rustc-link-search={}", out_dir.display());
}
