cwd = $(shell pwd)/out

setup:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview

release:
	cargo build --release
	$(MAKE) translate

dev: build
	dosbox -C 'MOUNT C $(cwd)' -C 'C:' -C 'DOSMOS.COM'

build:
	cargo build
	$(MAKE) translate

translate:
	mkdir -p out
	cargo objcopy --release -- -O binary --binary-architecture=i386:x86 out/dosmos.com

