use gdnative::prelude::*;

mod sculpt;
use sculpt::voxel_object::VoxelObject;
mod controls;
use controls::*;


fn init(handle: InitHandle) {
	handle.add_class::<VoxelObject>();
	handle.add_class::<VRControls>();
}

godot_init!(init);
