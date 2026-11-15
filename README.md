# stm32f103-bare-metal-rs
A near absolute bare-metal implementation of "Hello, Blinky!" for the STM32F103 using its own reset handler, vector table, and linker script. Everything is built using Cargo!

Only "near absolute" as this implementation relies on the `stm32f1` and `cortex-m` crates to safely manipulate peripherals and use inline ARM assembly.

## Building
### Requirements
Building will require the following to be installed:
- `thumbv7m-none-eabi` as a Rustup target (`rustup target add thumbv7m-none-eabi`)
- Any toolchain that will allow you to copy an ELF binary to raw binary for flashing (e.x. `gcc-arm-none-eabi`)
- `make` (optional, as you can also just run the commands in the `Makefile` yourself)

### Creating .BIN File
Building the ELF binary file and copying it to raw binary has been rolled into one command using `make`, supporting both `debug` (default) and `release` builds:
```bash
# The recommended way, for the smallest resulting binary:
make release
# If you want to override the objcopy utility:
OBJCOPY=arm-none-eabi-objcopy make debug
```
The resulting .BIN file will be created in the project's root directory and will be named `blinky_<BUILD_CONFIGURATION>.bin`.

**Note:** the `arm-none-eabi` toolchain is used by default, as it's already installed on my machine; however, alternative `objcopy` utilities will work as well. The flags you must pass may differ if you are using a different toolchain.

## Flashing
### Requirements
Before flashing, complete the [building section](#Building), and ensure you have your preferred toolchain installed (e.x. `st-console`) to flash the firmware onto your STM32F103.

Using `st-flash`, you can perform the flash like so:
```bash
st-flash --reset write blinky_release.bin 0x08000000
```