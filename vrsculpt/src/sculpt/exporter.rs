use std::{path::Path, fs::File, io::Write};
use gdnative::api::OS;

use super::volume::*;
use super::voxel_object::*;

const EXPORT_HEADER: &[u8] = b"# exported from vr-sculpter\n";


pub trait ExportOBJ {
	fn export_obj(&self, path: &Path);
}


impl ExportOBJ for VoxelObject {
	fn export_obj(&self, path: &Path) {
		let t = DateTime::now();
		let timestamp = format!("{:02}-{:02} {:02}.{:02}.{:02}", t.month, t.day, t.hour, t.minute, t.second);
		let filename = format!("{} {}.obj", self.name, timestamp);
		let mut file = File::create(path.join(filename)).unwrap();
		file.write_all(EXPORT_HEADER).unwrap();

		for (index, volume) in self.volumes.iter().enumerate()	{
			export_volume(volume, &mut file, index);
		}
	}
}


fn export_volume(volume: &Volume, file: &mut File, index: usize) {
	let header = format!("\no Volume{}\ns off", index);
	file.write_all(header.as_bytes()).unwrap();

	let verts = volume.get_mesh().get_faces();
	let vert_count = verts.len() as usize;
	let verts = verts.read();
	for i in (0..vert_count).step_by(3) {
		let face = format!(
			"\nv {} {} {}\nv {} {} {}\nv {} {} {}\nf {} {} {}",
			verts[i].x, verts[i].y, verts[i].z,
			verts[i+1].x, verts[i+1].y, verts[i+1].z,
			verts[i+2].x, verts[i+2].y, verts[i+2].z,
			i+1, i+2, i+3/* why does OBJ index from 1??? */
		);
		file.write_all(&face.into_bytes()).unwrap();
	}
}


pub struct DateTime {
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
}


impl DateTime {
	/// wrapper for  Godot's OS.get_datetime()
	pub fn now() -> Self {
		let d = OS::godot_singleton().get_datetime(false);
		Self {
			year: d.get("year").unwrap().to().unwrap(),
			month: d.get("month").unwrap().to().unwrap(),
			day: d.get("day").unwrap().to().unwrap(),
			hour: d.get("hour").unwrap().to().unwrap(),
			minute: d.get("minute").unwrap().to().unwrap(),
			second: d.get("second").unwrap().to().unwrap(),
		}
	}
}
