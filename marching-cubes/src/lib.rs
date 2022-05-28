use gdnative::prelude::*;

mod voxel_object;
mod volume;
mod chunk;
mod mesh;
mod exporter;

use voxel_object::*;

fn init(handle: InitHandle) {
	handle.add_class::<VoxelObject>();
}

godot_init!(init);
