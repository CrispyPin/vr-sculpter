use gdnative::{api::OS, prelude::*};
use std::{fs::{self, File}, io::{Write, Read}, path::{PathBuf}};

use crate::volume::*;
use crate::chunk::*;

const SAVE_DATA_VERSION: u16 = 1;
const SAVE_FILE: &str = "object.meta";

const EXPORT_HEADER: &[u8] = b"# exported from <name here>\n";

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelObject {
	volumes: Vec<Volume>,
	active: usize,
	#[property]
	name: String,
	export_path: PathBuf,
	saves_path: PathBuf,
}

#[methods]
impl VoxelObject {
	fn new(_owner: &Spatial) -> Self {
		let data_dir = PathBuf::from(OS::godot_singleton().get_user_data_dir().to_string());

		Self {
			volumes: Vec::new(),
			active: 0,
			name: "untitled1".into(),
			export_path: data_dir.join("export"),
			saves_path: data_dir.join("saves"),
		}
	}

	#[export]
	fn set_name(&mut self, _owner: &Spatial, new: String) {
		self.name = new;
	}

	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		self.create_volume(owner);
	}
	
	#[export]
	fn _process(&mut self, _owner: &Spatial, _delta: f64) {
		if self.volumes.is_empty() {
			return;
		}
		self.volumes[0].mesh_modified();
	}

	#[export]
	fn set_sphere(&mut self, _owner: &Spatial, pos: Vector3, radius: f32, value: Voxel) {
		self.volumes[self.active].set_sphere(pos, radius, value);
	}

	#[export]
	fn smooth_sphere(&mut self, _owner: &Spatial, pos: Vector3, radius: f32) {
		self.volumes[self.active].smooth(pos, radius);
	}

	#[export]
	fn save(&self, _owner: &Spatial) {
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

	#[export]
	fn load(&mut self, owner: &Spatial) {
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
				self.add_volume(owner, new_volume);
			}
		}
	}

	#[export]
	fn export(&self, _owner: &Spatial) {
		let path = &self.export_path;
		if !path.exists() {
			fs::create_dir_all(&path).unwrap();
		}
		godot_print!("exporting to {}", path.display());

		let filename = format!("{}.obj", &self.name);
		let mut file = File::create(path.join(filename)).unwrap();
		file.write_all(EXPORT_HEADER).unwrap();

		for (index, volume) in self.volumes.iter().enumerate()	{
			volume.export(&mut file, index)
		}
	}

	#[export]
	fn create_volume(&mut self, owner: &Spatial) {
		self.add_volume(owner, Volume::new());
	}
	
	fn add_volume(&mut self, owner: &Spatial, volume: Volume) {
		owner.add_child(volume.node(), true);
		self.volumes.push(volume);
	}

	#[export]
	fn get_active(&self, _owner: &Spatial) -> usize {
		self.active
	}

	#[export]
	fn set_active(&mut self, _owner: &Spatial, new_value: usize) {
		self.active = new_value.max(self.volumes.len() - 1);
	}

	fn clear(&mut self) {
		for volume in &self.volumes {
			unsafe{ volume.node().assume_safe().queue_free(); }
		}
		self.volumes.clear();
		self.active = 0;
	}
}
