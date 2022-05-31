use gdnative::prelude::*;

use crate::sculpt::voxel_object::VoxelObject;


pub struct ToolHand {
	tools: Vec<Tool>,
	min_trigger: f32,
	active: usize,
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
	pub fn new() -> Self {
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
				target._set_sphere(pos, radius, 255);
			},
			ToolType::Erase => {
				target._set_sphere(pos, radius, 0);
			},
			ToolType::Smooth => {
				target._smooth_sphere(pos, radius);
			},
		}
		if tool.smooth {
			target._smooth_sphere(pos, radius + 1.0);
		}
	}

	fn tool(&self) -> Tool {
		self.tools[self.active].clone()
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