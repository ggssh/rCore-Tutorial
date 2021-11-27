# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# Binutils(have installed tools before)
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Platform
PLATFORM := qemu
SBI := rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(PLATFORM).bin

# KERNEL_ENTRY
KERNEL_ENTRY_ADD := 0x80200000


# build: $(KERNEL_BIN)

$(KERNEL_BIN): build
# $@表示目标文件,即KERNEL_BIN
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

build:
	@cargo build --release

clean:
	@cargo clean

run: $(KERNEL_BIN)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_ADD)

.PHONY: build clean run
