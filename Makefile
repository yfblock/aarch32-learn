TARGET := armv7a-none-eabi
KERNEL_ELF := target/$(TARGET)/release/rtos
KERNEL_BIN := $(KERNEL_ELF).bin

QEMU_OPTIONS := -M virt -m 1G -cpu cortex-a7 --nographic
ifeq ($(QEMU_DEBUG), y)
QEMU_OPTIONS += -D ./qemu.log -d in_asm,int,pcall,cpu_reset,guest_errors
endif

all: $(KERNEL_BIN)

$(KERNEL_BIN): build
	@rust-objcopy $(KERNEL_ELF) --strip-all -O binary $@

build:
	cargo build --release --target $(TARGET)

run:
	qemu-system-arm -kernel $(KERNEL_BIN) $(QEMU_OPTIONS)

dtb:
	qemu-system-arm $(QEMU_OPTIONS) -M dumpdtb=file.dtb

disasm:
	rust-objdump -S $(KERNEL_ELF)

.PHONY: all build run
