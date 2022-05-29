ADDON_CRATE=marching-cubes
ADDON=voxel
ADDON_LIB=libmarching_cubes.so

default: debug
debug:
	cd $(ADDON_CRATE) && cargo build
	ln -sf ../../../../../$(ADDON_CRATE)/target/debug/$(ADDON_LIB) project/addons/$(ADDON)/bin/linux/$(ADDON_LIB)

r: release
release:
	cd $(ADDON_CRATE) && cargo build --release
	ln -sf ../../../../../$(ADDON_CRATE)/target/release/$(ADDON_LIB) project/addons/$(ADDON)/bin/linux/$(ADDON_LIB)

c: clippy
clippy:
	cd $(ADDON_CRATE) && cargo clippy
