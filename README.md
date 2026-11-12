# STM32F103-bare-metal-blinky-rs
A near absolute bare-metal implementation of "Hello, Blinky!" for the STM32F103 using its own vector table and linker script. Everything is built using Cargo!

Only "near absolute" as this implementation relies on the `stm32f1` and `cortex-m` crates to safely manipulate peripherals and use inline ARM assembly. `cortex-m-rt` was intentionally left out, as it provides the vector table and *most* of the linker script.

## Building
### Requirements
Building will require the following to be installed:
- `thumbv7m-none-eabi` as a Rustup target (`rustup target add thumbv7m-none-eabi`)
- Any toolchain that will allow you to produce a valid firmware .BIN file (e.x. `gcc-arm-none-eabi`)

### Building .ELF Binary
To obtain an .ELF binary to use for flashing the STM32F103, run:
```bash
cargo build --release
```

**Note:** `--release` is **REQUIRED**, as the resulting .ELF binary file will be too big for the STM32F103's flash memory otherwise.

### Creating .BIN File
After obtaining the .ELF binary file, use your previously installed toolchain to create a .BIN file that can used for flashing:
```bash
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/blinky firmware.bin
```
**Note:** I use the `arm-none-eabi` toolchain for convenience, as it's already installed on my machine; however, alternative `objcopy` utilities will work as well. The flags you must pass may differ if you are using a different toolchain.

## Flashing
### Requirements
Before flashing, complete the [building section](#Building), and ensure you have your preferred toolchain installed (e.x. `st-console`) to flash the firmware onto your STM32F103.

Using `st-flash`, you can perform the flash like so:
```bash
st-flash --reset write firmware.bin 0x08000000
```