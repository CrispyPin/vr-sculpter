use gdnative::api::ARVRController;
use gdnative::{prelude::*, api::ARVROrigin};

use crate::sculpt::voxel_object::VoxelObject;

use crate::tool::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct VRSculpt {
	controls: Option<VRControls>,
	objects: Vec<VoxelObject>,
	active: Option<usize>,
}

struct VRControls {
	arvr_origin: Ref<ARVROrigin>,
	right: ToolHand,
	left: ToolHand,
}

#[methods]
impl VRSculpt {
	fn new(_owner: &Node) -> Self {
		Self {
			controls: None,
			objects: Vec::new(),
			active: None,
		}
	}

	#[export]
	fn _ready(&mut self, owner: &Node) {
		let arvr_origin = unsafe { owner
			.get_node("/root/Main/VRViewport/ARVROrigin")
			.expect("missing Main/VRViewport/ARVROrigin")
			.assume_safe()
			.cast::<ARVROrigin>()
			.unwrap()
			.claim()
		};
		self.controls = Some(VRControls::new(arvr_origin));
		self.create_object(owner);
		self.active = Some(0);
	}
	
	#[export]
	fn _process(&mut self, _owner: &Node) {
		for obj in self.objects.iter_mut() {
			obj.update_meshes();
		}
		if let Some(controls) = &mut self.controls {
			if let Some(active) = self.active {
				controls.right.update(&mut self.objects[active]);
				controls.left.update(&mut self.objects[active]);
			}
		}
	}

	fn create_object(&mut self, owner: &Node) {
		let mut object = VoxelObject::new();
		object.create_volume();
		owner.add_child(object.node(), true);
		self.objects.push(object);
	}
}


impl VRControls {
	pub fn new(arvr_origin: Ref<ARVROrigin>) -> Self {
		let hand_r = unsafe{arvr_origin.assume_safe().get_node("VRRight").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()};
		let hand_l = unsafe{arvr_origin.assume_safe().get_node("VRLeft").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()};
		
		Self {
			arvr_origin,
			left: ToolHand::new(hand_l),
			right: ToolHand::new(hand_r),
		}
	}
}
