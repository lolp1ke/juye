# SPDX-License-Identifier: Apache-2.0

CARGO := cargo
TARGET := aarch64-unknown-none
PROFILE := release

QEMU := qemu-system-aarch64
ELF := target/$(TARGET)/$(PROFILE)/boot

all: run

build: 
	$(CARGO) build --target $(TARGET) --$(PROFILE)

run: build
	$(QEMU) \
		-machine virt \
		-cpu cortex-a72 \
		-m 2G \
		-smp 4 \
		-nographic \
		-kernel $(ELF)

clean:
	$(CARGO) clean
