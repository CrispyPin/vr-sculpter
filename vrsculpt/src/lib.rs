use gdnative::prelude::*;

mod sculpt;
// use sculpt::voxel_object::VoxelObject;
mod controls;
// use controls::*;
mod vrsculpt;
use vrsculpt::*;


fn init(handle: InitHandle) {
	// handle.add_class::<VoxelObject>();
	// handle.add_class::<VRControls>();
	handle.add_class::<VRSculpt>();
}

godot_init!(init);
