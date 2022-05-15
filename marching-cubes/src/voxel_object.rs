use gdnative::prelude::*;

use crate::volume::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelObject {
	volumes: Vec<Volume>,
	active: usize,
	time: f64,
}

#[methods]
impl VoxelObject {
	fn new(_owner: &Spatial) -> Self {
		Self {
			volumes: Vec::new(),
			active: 0,
			time: 0f64,
		}
	}

	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		self.create_volume(owner);
	}
	
	#[export]
	fn _process(&mut self, _owner: &Spatial, delta: f64) {
		self.time += delta;
		// self.volumes[0].surface_level = (f64::sin(self.time) * 50f64 + 160f64) as u8;
		self.volumes[0].mesh_modified();
	}

	#[export]
	fn brush_add(&mut self, _owner: &Spatial, pos: Vector3, radius: f32) {
		self.volumes[self.active].brush_add(pos, radius);
	}

	#[export]
	fn create_volume(&mut self, owner: &Spatial) {
		let new_volume = Volume::new();
		owner.add_child(new_volume.node(), true);
		self.volumes.push(new_volume);
	}

	#[export]
	fn get_active(&self, _owner: &Spatial) -> usize {
		self.active
	}

	#[export]
	fn set_active(&mut self, _owner: &Spatial, new_value: usize) {
		self.active = new_value.max(self.volumes.len() - 1);
	}
}
