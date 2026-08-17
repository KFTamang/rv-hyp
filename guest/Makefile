# guest/Makefile
CARGO_TARGET := riscv64gc-unknown-none-elf
BUILD_DIR    := ../build
BIN_NAME     := guest_kernel

.PHONY: build clean

build:
	mkdir -p $(BUILD_DIR)
	cargo build --release --target $(CARGO_TARGET)
	cp target/$(CARGO_TARGET)/release/$(BIN_NAME) $(BUILD_DIR)/$(BIN_NAME).elf

clean:
	cargo clean
	rm -f $(BUILD_DIR)/$(BIN_NAME).elf
