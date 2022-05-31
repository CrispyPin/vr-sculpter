use std::cell::RefCell;

use gdnative::{prelude::*, api::{ARVROrigin, ARVRController}};
use gdnative::api::GlobalConstants;

use crate::sculpt::voxel_object::VoxelObject;
mod tool;
use tool::*;


#[derive(NativeClass)]
#[inherit(ARVROrigin)]
pub struct VRControls {
	voxel_object: Option<RefCell<Instance<VoxelObject>>>,
	hand_r: Option<Ref<ARVRController>>,
	hand_l: Option<Ref<ARVRController>>,
	tool_r: ToolHand,
	tool_l: ToolHand,
}

#[methods]
impl VRControls {
	fn new(_owner: &ARVROrigin) -> Self {
		Self {
			voxel_object: None,
			hand_l: None,
			hand_r: None,
			tool_l: ToolHand::new(),
			tool_r: ToolHand::new(),
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
		self.voxel_object = Some(RefCell::new(voxel_object.claim()));
		
		self.voxel_object().map_mut(
			|obj, _owner| {
				obj._set_sphere(Vector3::new(25.0, 50.0, 0.0), 4.0, 255);
			}).unwrap();
		
		self.hand_r = Some(unsafe{owner.get_node("VRRight").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()});
		self.hand_l = Some(unsafe{owner.get_node("VRLeft").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()});
	}

	fn voxel_object(&self) -> TInstance<VoxelObject> {
		unsafe {
			self.voxel_object
				.as_ref()
				.unwrap()
				.borrow_mut()
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
			let right_hand = self.right_hand();
			let trigger = right_hand.get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
			let pos = right_hand.translation();
	
			self.voxel_object().map_mut(|voxel_object, _owner| {
				self.tool_r.apply(voxel_object, trigger, pos);
			}).unwrap();
		}

		{
			let left_hand = self.left_hand();
			let trigger = left_hand.get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
			let pos = left_hand.translation();
	
			self.voxel_object().map_mut(|voxel_object, _owner| {
				self.tool_l.apply(voxel_object, trigger, pos);
			}).unwrap();	
		}
	}
}
