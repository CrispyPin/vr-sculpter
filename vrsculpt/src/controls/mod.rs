use gdnative::{prelude::*, api::ARVROrigin};


#[derive(NativeClass)]
#[inherit(ARVROrigin)]
pub struct VRControls {

}

#[methods]
impl VRControls {
	fn new(_owner: &ARVROrigin) -> Self {
		Self {

		}
	}

	fn _ready(&self, _owner: &ARVROrigin) {
		godot_print!("aaaa");
	}
}
