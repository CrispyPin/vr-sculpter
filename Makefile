CRATE=vrsculpt
LIB_NAME=libvrsculpt.so

default: debug
debug:
	cd $(CRATE) && cargo build
	ln -sf ../../../$(CRATE)/target/debug/$(LIB_NAME) project/bin/linux/$(LIB_NAME)

r: release
release:
	cd $(CRATE) && cargo build --release
	ln -sf ../../../$(CRATE)/target/release/$(LIB_NAME) project/bin/linux/$(LIB_NAME)

c: clippy
clippy:
	cd $(CRATE) && cargo clippy

