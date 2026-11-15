const LD_NAME: &str = "cortex_m";

fn main() {
    println!("cargo:rustc-link-arg=-T{}.ld", LD_NAME);
}
