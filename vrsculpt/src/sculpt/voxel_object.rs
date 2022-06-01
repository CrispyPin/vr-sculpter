use gdnative::{api::OS, prelude::*};
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::PathBuf;

use super::exporter::*;
use super::volume::*;
use super::chunk::*;

const SAVE_DATA_VERSION: u16 = 1;
const SAVE_FILE: &str = "object.meta";

// #[derive(NativeClass)]
// #[inherit(Spatial)]
pub struct VoxelObject {
	node: Ref<Spatial>,
	pub volumes: Vec<Volume>,
	active: usize,
	pub name: String,
	pub export_path: PathBuf,
	saves_path: PathBuf,
}

// #[methods]
impl VoxelObject {
	pub fn new() -> Self {
		let data_dir = PathBuf::from(OS::godot_singleton().get_user_data_dir().to_string());
		let node = unsafe { Spatial::new().assume_shared() };

		Self {
			node,
			volumes: Vec::new(),
			active: 0,
			name: "untitled1".into(),
			export_path: data_dir.join("export"),
			saves_path: data_dir.join("saves"),
		}
	}

	pub fn node(&self) -> Ref<Spatial> {
		self.node
	}

	pub fn update_meshes(&mut self) {
		for v in self.volumes.iter_mut() {
			v.mesh_modified();
		}
	}
	
	pub fn set_sphere(&mut self, pos: Vector3, radius: f32, value: Voxel) {
		self.volumes[self.active].set_sphere(pos, radius, value);
	}
	
	pub fn smooth_sphere(&mut self, pos: Vector3, radius: f32) {
		self.volumes[self.active].smooth(pos, radius);
	}

	pub fn save(&self, _owner: &Spatial) {
		let path = self.saves_path.join(&self.name);
		if !path.exists() {
			fs::create_dir_all(&path).unwrap();
		}
		godot_print!("saving to {}", path.display());

		// store metadata
		let mut indexfile = match File::create(path.join(SAVE_FILE)) {
			Err(err) => {
				godot_print!("Error saving file: {}", err);
				return;
			},
			Ok(file) => file,
		};

		let save_data = format!("version: {};{};", SAVE_DATA_VERSION, self.volumes.len());
		indexfile.write_all(save_data.as_bytes()).unwrap();
		indexfile.flush().unwrap();

		// store voxel data
		for (i, volume) in self.volumes.iter().enumerate() {
			volume.save(&path, i);
		}
	}

	pub fn load(&mut self) {
		let path = self.saves_path.join(&self.name);
		if !path.join(SAVE_FILE).exists() {
			godot_print!("No save file exists for '{}'", &self.name);
			return;
		}
		self.clear();
		let mut indexfile = File::open(path.join(SAVE_FILE)).unwrap();

		let mut data = Vec::new();
		indexfile.read_to_end(&mut data).unwrap();
		// parse metadata
		let data = String::from_utf8_lossy(&data);
		let (_version_str, volume_count_str) = data.split_once(';').unwrap();
		let volume_count_str = volume_count_str.split_once(';').unwrap().0;
		let volume_count: usize = volume_count_str.parse().unwrap();
		godot_print!("volumes: {}", volume_count);

		// load volumes
		for i in 0..volume_count {
			if let Some(new_volume) = Volume::load(&path, i) {
				self.add_volume(new_volume);
			}
		}
	}

	pub fn export(&self, _owner: &Spatial) {
		let path = &self.export_path;
		if !path.exists() {
			fs::create_dir_all(&path).unwrap();
		}
		godot_print!("exporting to {}", path.display());
		self.export_obj(path);
	}

	pub fn create_volume(&mut self) {
		self.add_volume(Volume::new());
	}
	
	pub fn add_volume(&mut self, volume: Volume) {
		unsafe {
			self.node.assume_safe().add_child(volume.node(), true);
		}
		self.volumes.push(volume);
	}

	pub fn active(&self, _owner: &Spatial) -> usize {
		self.active
	}

	pub fn set_active(&mut self, _owner: &Spatial, new_value: usize) {
		self.active = new_value.max(self.volumes.len() - 1);
	}

	pub fn clear(&mut self) {
		for volume in &self.volumes {
			unsafe{ volume.node().assume_safe().queue_free(); }
		}
		self.volumes.clear();
		self.active = 0;
		self.create_volume();
	}
}
