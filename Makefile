ADDON_CRATE=vrsculpt
ADDON_LIB=libvrsculpt.so

# MAIN_CRATE=vr-sculpter
# MAIN_LIB=libvr_sculpter.so

default: debug
debug:
	cd $(ADDON_CRATE) && cargo build
	ln -sf ../../../$(ADDON_CRATE)/target/debug/$(ADDON_LIB) project/bin/linux/$(ADDON_LIB)
#
#	cd $(MAIN_CRATE) && cargo build
#	ln -sf ../../../$(MAIN_CRATE)/target/debug/$(MAIN_LIB) project/bin/linux/$(MAIN_LIB)

r: release
release:
	cd $(ADDON_CRATE) && cargo build --release
	ln -sf ../../../$(ADDON_CRATE)/target/release/$(ADDON_LIB) project/bin/linux/$(ADDON_LIB)

#	cd $(MAIN_CRATE) && cargo build --release
#	ln -sf ../../../$(MAIN_CRATE)/target/release/$(MAIN_LIB) project/bin/linux/$(MAIN_LIB)
#
c: clippy
clippy:
	cd $(ADDON_CRATE) && cargo clippy
#	cd $(MAIN_CRATE) && cargo clippy
