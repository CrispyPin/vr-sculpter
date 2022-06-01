use gdnative::{prelude::*, api::ARVROrigin};

use crate::{controls::VRControls, sculpt::voxel_object::VoxelObject};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct VRSculpt {
	controls: VRControls,
	objects: Vec<VoxelObject>,
	active: Option<usize>,
}

#[methods]
impl VRSculpt {
	fn new(_owner: &Node) -> Self {
		Self {
			controls: VRControls::new(/* arvr_origin */),
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
		self.controls.init(arvr_origin);
		self.create_object(owner);
		self.active = Some(0);
	}
	
	#[export]
	fn _process(&mut self, _owner: &Node, _delta: f64) {
		for obj in self.objects.iter_mut() {
			obj.update_meshes();
		}
		if let Some(active) = self.active {
			self.controls.update(&mut self.objects[active]);
		}
	}

	fn create_object(&mut self, owner: &Node) {
		let mut object = VoxelObject::new();
		object.create_volume();
		owner.add_child(object.node(), true);
		self.objects.push(object);
	}
}
