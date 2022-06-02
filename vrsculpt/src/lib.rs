use gdnative::prelude::*;

mod sculpt;
mod tool;
mod vrsculpt;
use vrsculpt::VRSculpt;


fn init(handle: InitHandle) {
	handle.add_class::<VRSculpt>();
}

godot_init!(init);
