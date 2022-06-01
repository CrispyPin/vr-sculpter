use gdnative::{prelude::*, api::{ARVROrigin, ARVRController}};
use gdnative::api::GlobalConstants;

mod tool;
use tool::*;
mod input;
use input::*;

use crate::sculpt::voxel_object::VoxelObject;


pub struct VRControls {
	arvr_origin: Option<Ref<ARVROrigin>>,
	right: Option<ToolHand>,
	left: Option<ToolHand>,
}

impl VRControls {
	pub fn new(/* arvr_origin: Ref<ARVROrigin> */) -> Self {
		
		Self {
			arvr_origin: None,
			left: None,
			right: None,
		}
	}
	
	pub fn init(&mut self, arvr_origin: Ref<ARVROrigin>) {	
		let hand_r = unsafe{arvr_origin.assume_safe().get_node("VRRight").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()};
		let hand_l = unsafe{arvr_origin.assume_safe().get_node("VRLeft").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()};

		self.left = Some(ToolHand::new(hand_l));
		self.right = Some(ToolHand::new(hand_r));
		// self.hand_r = Some(unsafe{owner.get_node("VRRight").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()});
		// self.hand_l = Some(unsafe{owner.get_node("VRLeft").unwrap().assume_safe().cast::<ARVRController>().unwrap().claim()});
	}
/* 
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
	} */
	
	pub fn update(&mut self, object: &mut VoxelObject) {
		self.right.as_mut().unwrap().update(object);
		self.left.as_mut().unwrap().update(object);
	/* 	{
			let right_hand = self.right_hand();
			let trigger = right_hand.get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
			let pos = right_hand.translation();

			self.right.apply(object, trigger, pos);
		}
		
		{
			let left_hand = self.left_hand();
			let trigger = left_hand.get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
			let pos = left_hand.translation();

			self.left.apply(object, trigger, pos);
		} */
/* 
		let right_x = self.right_hand().get_joystick_axis(GlobalConstants::JOY_AXIS_0);
		if right_x.abs() < 0.3{
			self.state_r.sel_x = false;
		}
		if !self.state_r.sel_x {
			if right_x > 0.8 {
				self.right.next();
				self.state_r.sel_x = true;
			}
			else if right_x < -0.8 {
				self.right.prev();
				self.state_r.sel_x = true;
			}
		} */
	}
}
