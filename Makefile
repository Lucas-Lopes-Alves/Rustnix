AS=i686-elf-as
CC=i686-elf-gcc
CARGOFLAGS = build -Zjson-target-spec
$(shell mkdir -p build build/bin build/obj)
SRCS_S = $(shell find src/ -type f -name "*.s")
OBJS = $(patsubst src/%.s, build/obj/%.o, $(SRCS_S))
RUSTLIB = build/obj/librustnix.a

all: build

build: $(OBJS)
	@RUSTFLAGS="-C relocation-model=static" cargo $(CARGOFLAGS)
	@cp target/i686-unknown-none/debug/librustnix.a build/obj/
	@$(CC) -nostdlib -T kernel.ld $(OBJS) $(RUSTLIB) -lgcc -o build/bin/kernel.elf
	@echo "Done"

build/obj/%.o: src/%.s
	@mkdir -p $(dir $@)
	@echo "Compiling the Assembly files"
	@$(AS) --32 $< -o $@

run:
	@qemu-system-i386 -kernel build/bin/kernel.elf