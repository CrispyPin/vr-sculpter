use gdnative::{prelude::*, api::{ARVROrigin, ARVRController}};
use gdnative::api::GlobalConstants;

use crate::sculpt::voxel_object::VoxelObject;


#[derive(NativeClass)]
#[inherit(ARVROrigin)]
pub struct VRControls {
	voxel_object: Option<Instance<VoxelObject>>,
	hand_r: Option<Ref<ARVRController>>,
	hand_l: Option<Ref<ARVRController>>,
	radius: f32,
}

#[methods]
impl VRControls {
	fn new(_owner: &ARVROrigin) -> Self {
		Self {
			voxel_object: None,
			hand_l: None,
			hand_r: None,
			radius: 5.0,
		}
	}

	#[export]
	fn _ready(&mut self, owner: &ARVROrigin) {
		let voxel_object = unsafe {
			owner.get_node("/root/Main/VoxelObject")
			.unwrap()
			.assume_safe()
			.cast::<Spatial>()
			.unwrap()
			.cast_instance::<VoxelObject>()
			.unwrap()
		};
		self.voxel_object = Some(voxel_object.claim());
		
		self.voxel_object().map_mut(
			|obj, owner| {
				obj.set_sphere(owner, Vector3::new(25.0, 50.0, 0.0), 4.0, 255);
			}).unwrap();
		
		self.hand_r = Some(unsafe{owner.get_node("VRRight").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()});
		self.hand_l = Some(unsafe{owner.get_node("VRLeft").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()});
	}

	fn voxel_object(&mut self) -> TInstance<VoxelObject> {
		unsafe {
			self.voxel_object
				.as_mut()
				.unwrap()
				.assume_safe()
		}
	}

	fn right_hand(&self) -> TRef<ARVRController> {
		unsafe {
			self.hand_r
				.unwrap()
				.assume_safe()
		}
	}

	fn left_hand(&self) -> TRef<ARVRController> {
		unsafe {
			self.hand_l
				.unwrap()
				.assume_safe()
		}
	}

	#[export]
	fn _physics_process(&mut self, _owner: TRef<ARVROrigin>, _delta: f32) {
		{
			let trigger_right = self.right_hand().get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
			let radius = self.radius * trigger_right;
			if radius > 1.5 {
				let pos = self.right_hand().translation();
	
				self.voxel_object().map_mut(|voxel_object, owner| {
					voxel_object.set_sphere(owner, pos, radius, 255);
					voxel_object.smooth_sphere(owner, pos, radius + 1.0);
				}).unwrap();
			}
		}

		{
			let trigger_left = self.left_hand().get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
			let radius = self.radius * trigger_left;
			if radius > 1.5 {
				let pos = self.left_hand().translation();
	
				self.voxel_object().map_mut(|voxel_object, owner| {
					voxel_object.set_sphere(owner, pos, radius, 0);
					voxel_object.smooth_sphere(owner, pos, radius + 1.0);
				}).unwrap();	
			}
		}
	}
}
