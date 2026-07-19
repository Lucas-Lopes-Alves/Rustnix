AS=i686-elf-as
CC=i686-elf-gcc
LDFLAGS= -ffreestanding -nostdlib -lgcc
CARGOFLAGS = +nightly check -Zjson-target-spec -Z build-std=core,compiler_builtins --target i686-unknown-none.json
$(shell mkdir -p build build/bin build/obj)
SRCS_S = $(shell find src/ -type f -name "*.s")
OBJS = $(patsubst src/%.s, build/obj/%.o, $(SRCS_S))
RUSTOBJS = build/obj/librustnix.a

all: build

build: $(OBJS)
	@RUSTFLAGS="-C relocation-model=static" cargo $(CARGOFLAGS)
	@cp target/i686-unknown-none/debug/librustnix.a build/obj/
	@$(CC) $(LDFLAGS) -T kernel.ld $(OBJS) $(RUSTOBJS) -o build/bin/kernel.elf
	@echo "Done"

build/obj/%.o: src/%.s
	@mkdir -p $(dir $@)
	@echo "Compiling the Assembly files"
	@$(AS) $< -o $@

run:
	@qemu-system-i386 -kernel build/bin/kernel.elf