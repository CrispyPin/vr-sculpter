default:
	cd marching-cubes && cargo build
	ln -sf ../../../../../marching-cubes/target/debug/libmarching_cubes.so project/addons/voxel/bin/linux/libmarching-cubes.so

r: release
release:
	cd marching-cubes && cargo build --release
	ln -sf ../../../../../marching-cubes/target/release/libmarching_cubes.so project/addons/voxel/bin/linux/libmarching-cubes.so

c: clippy
clippy:
	cd marching-cubes && cargo clippy
