LIB_NAME=libvrsculpt.so

default: release
release:
	cargo build --release
	ln -sf ../../../target/release/$(LIB_NAME) project/bin/linux/$(LIB_NAME)

d: debug
debug:
	cargo build
	ln -sf ../../../target/debug/$(LIB_NAME) project/bin/linux/$(LIB_NAME)

lint:
	cargo clippy -- -W clippy::pedantic

