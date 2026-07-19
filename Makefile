AS=i686-elf-as
CC=i686-elf-gcc
PROFILE:=
CARGOFLAGS = build -Zjson-target-spec $(PROFILE)
$(shell mkdir -p build build/bin build/obj)
SRCS_S = $(shell find src/ -type f -name "*.s")
OBJS = $(patsubst src/%.s, build/obj/%.o, $(SRCS_S))
RUSTLIB = build/obj/librustnix.a

all: build

build: $(OBJS)
	@echo "Compiling the Rust files"
	@RUSTFLAGS="-C relocation-model=static" cargo $(CARGOFLAGS)
	@cp target/i686-unknown-none/debug/librustnix.a build/obj/
	@echo "Linking the files"
	@$(CC) -nostdlib -T kernel.ld $(OBJS) $(RUSTLIB) -lgcc -o build/bin/kernel.elf > /dev/null 2>&1
	@echo "Done"

build/obj/%.o: src/%.s
	@mkdir -p $(dir $@)
	@echo "Compiling the Assembly files"
	@$(AS) --32 $< -o $@

run:
	@qemu-system-i386 -kernel build/bin/kernel.elf