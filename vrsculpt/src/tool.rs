use gdnative::{prelude::*, api::{ARVRController, GlobalConstants}};

use crate::sculpt::voxel_object::VoxelObject;


pub struct ToolHand {
	tools: Vec<Tool>,
	min_trigger: f32,
	active: usize,
	controller: Ref<ARVRController>,
	sel_x: bool,
}

#[derive(Clone)]
pub struct Tool {
	action: ToolType,
	radius: f32,
	smooth: bool,
}

#[derive(Clone)]
pub enum ToolType {
	Add,
	Erase,
	Smooth,
}

impl ToolHand {
	pub fn new(controller: Ref<ARVRController>) -> Self {
		let tools = vec![
			Tool::new(ToolType::Add, false),
			Tool::new(ToolType::Add, true),
			Tool::new(ToolType::Erase, false),
			Tool::new(ToolType::Erase, true),
			Tool::new(ToolType::Smooth, false),
		];
		Self {
			tools,
			active: 0,
			min_trigger: 0.2,
			controller,
			sel_x: false,
		}
	}

	fn next(&mut self) {
		self.active = (self.active + 1) % self.tools.len()
	}

	fn prev(&mut self) {
		self.active = (self.active + self.tools.len() - 1) % self.tools.len()
	}

	pub fn update(&mut self, object: &mut VoxelObject) {
		let controller = unsafe { self.controller.assume_safe() };

		let trigger = controller.get_joystick_axis(GlobalConstants::JOY_VR_ANALOG_TRIGGER) as f32;
		let pos = controller.translation();
		self.apply(object, trigger, pos);


		let joy_x = controller.get_joystick_axis(GlobalConstants::JOY_AXIS_0);
		if joy_x.abs() < 0.3{
			self.sel_x = false;
		}
		if !self.sel_x {
			if joy_x > 0.8 {
				self.next();
				self.sel_x = true;
			}
			else if joy_x < -0.8 {
				self.prev();
				self.sel_x = true;
			}
		}
	}

	pub fn apply(&self, target: &mut VoxelObject, trigger: f32, pos: Vector3) {
		if trigger < self.min_trigger {
			return;
		}
		let tool = self.tool();
		let radius = tool.radius * trigger;
		match tool.action {
			ToolType::Add => {
				target.set_sphere(pos, radius, 255);
			},
			ToolType::Erase => {
				target.set_sphere(pos, radius, 0);
			},
			ToolType::Smooth => {
				target.smooth_sphere(pos, radius);
			},
		};
		if tool.smooth {
			target.smooth_sphere(pos, radius + 2.0);
		}
	}

	fn tool(&self) -> &Tool {
		&self.tools[self.active]
	}
}


impl Tool {
	fn new(action: ToolType, smooth: bool) -> Self {
		Self {
			action,
			radius: 5.0,
			smooth
		}
	}
}