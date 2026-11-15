OBJCOPY?=arm-none-eabi-objcopy
CARGO_FLAGS=
RS_TARGET=thumbv7m-none-eabi
ELF_NAME=blinky

.PHONY: all debug release clean

all: debug

debug release:
	cargo build $(CARGO_FLAGS)
	$(OBJCOPY) -O binary target/$(RS_TARGET)/$@/$(ELF_NAME) $(ELF_NAME)_$@.bin

release: CARGO_FLAGS=--release

clean:
	cargo clean
	rm -f *.bin
