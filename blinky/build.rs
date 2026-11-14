const LD_NAME: &str = "STM32F103";

fn main() {
    println!("cargo:rustc-link-arg=-T{}.ld", LD_NAME);
}
