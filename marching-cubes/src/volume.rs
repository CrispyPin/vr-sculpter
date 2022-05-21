use gdnative::{prelude::*, api::{MeshInstance, ArrayMesh, Material, Mesh}};
use rayon::prelude::*;
use std::{collections::HashMap, mem::{self, transmute_copy, size_of}, time::Instant, path::Path, fs::File, io::{Write, Read}};

use crate::chunk::*;
use crate::mesh;

const DEBUG_MESH_TIMES: bool = false;
const DEBUG_SMOOTH_TIMES: bool = false;

const FILE_HEADER: &[u8] = b"voxel volume data";

pub type ChunkLoc = (i16, i16, i16);


pub struct Volume {
	chunks: HashMap<ChunkLoc, Chunk>,
	surface_indexes: HashMap<ChunkLoc, i64>,
	modified: Vec<ChunkLoc>,
	node: Ref<MeshInstance>,
	mesh: Ref<ArrayMesh>,
	pub surface_level: u8,
	material: Option<Ref<Material>>,
}


impl Volume {
	pub fn new() -> Self {
		let mesh = ArrayMesh::new().into_shared();
		let mesh_node = unsafe { MeshInstance::new().into_shared().assume_safe() };
		mesh_node.set_mesh(&mesh);
		
		Self {
			chunks: HashMap::new(),
			surface_indexes: HashMap::new(),
			modified: Vec::new(),
			node: mesh_node.claim(),
			mesh,
			material: None,
			surface_level: 128,
		}
	}

	pub fn node(&self) -> Ref<MeshInstance> {
		self.node
	}

	pub fn set_sphere(&mut self, pos: Vector3, radius: f32, value: Voxel) {
		let locs = locs_in_sphere(pos, radius);
		for loc in locs {
			let chunk = self.ensure_chunk(loc);
			let local_pos = pos - loc.as_wpos();
			chunk.set_sphere(local_pos, radius, value);
			self.modified.push(loc);
		}
	}

	pub fn smooth(&mut self, pos: Vector3, radius: f32) {
		let start = Instant::now();

		let locs = locs_in_sphere(pos, radius);

		let new_chunks: Vec<Option<(&ChunkLoc, Chunk)>> = locs.par_iter().map(|loc| {
			if !self.chunks.contains_key(loc) {
				return None;
			}
			let local_pos = pos - loc.as_wpos();
			let neighbors = ChunkBox3::new(&self.chunks, *loc);
			let new_chunk = Chunk::smooth_sphere(neighbors, local_pos, radius);
			Some((loc, new_chunk))
		}).collect();

		for (&loc, chunk) in new_chunks.into_iter().flatten() {
			self.chunks.insert(loc, chunk);
			self.modified.push(loc);
		}
		if DEBUG_SMOOTH_TIMES {
			godot_print!("smoothing took: {}ms", start.elapsed().as_micros() as f32 / 1000.0);
		}
	}
	
	pub fn mesh_modified(&mut self) {
		let start = Instant::now();
		let array_mesh: TRef<ArrayMesh> = unsafe { self.mesh.assume_safe() };
		if self.modified.is_empty() {
			return;
		}

		let mut modified = mem::take(&mut self.modified);
		modified.sort_unstable();
		modified.dedup();
		
		for loc in &modified {
			if self.surface_indexes.contains_key(loc) {
				// "move" indexes down one if they come after this surface, since it will be removed
				let &i = self.surface_indexes.get(loc).unwrap();
				
				self.surface_indexes.values_mut().for_each(|x: &mut i64|
					if *x > i {
						*x -= 1;
					}
				);
				// remove this chunks surface
				self.surface_indexes.remove(loc);
				array_mesh.surface_remove(i);
			}
		}
		let meshes:Vec<(ChunkLoc, Option<VariantArray>)> = modified.par_iter().map(|&loc| {
			let offset = loc.as_wpos();
			let chunks = ChunkBox2::new(&self.chunks, loc);
			(loc, mesh::generate(chunks, offset, self.surface_level))
		}).collect();
		
		let mesh = unsafe { self.mesh.assume_safe() };
		for mesh_data in meshes {
			if let (loc, Some(mesh_arr)) = mesh_data {
				self.surface_indexes.insert(loc, mesh.get_surface_count());
				mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, mesh_arr, VariantArray::new_shared(), 0);
			}
		}
		if DEBUG_MESH_TIMES {
			godot_print!("meshes took: {}ms", start.elapsed().as_micros() as f32 / 1000.0);
		}
	}

	fn ensure_chunk(&mut self, loc: ChunkLoc) -> &mut Chunk{
		if !self.chunks.contains_key(&loc) {
			self.create_chunk(loc);
		}
		self.chunks.get_mut(&loc).unwrap()
	}

	fn create_chunk(&mut self, loc: ChunkLoc) {
		self.chunks.insert(loc, Chunk::new());
	}

	pub fn save(&self, path: &Path, index: usize) {
		let filename = format!("{}.bin", index);
		let mut file = File::create(path.join(filename)).unwrap();

		file.write_all(FILE_HEADER).unwrap();

		let transform = unsafe { self.node.assume_safe() }.transform();
		let transform_bytes: [u8; size_of::<Transform>()] = unsafe { transmute_copy(&transform) };
		file.write_all(&transform_bytes).unwrap();

		for (loc, chunk) in &self.chunks {
			let loc_bytes: [u8; size_of::<ChunkLoc>()] = loc.as_bytes();
			file.write_all(&loc_bytes).unwrap();
			file.write_all(chunk.voxels.as_slice()).unwrap();
		}
	}

	pub fn load(path: &Path, index: usize) -> Option<Self> {
		let filename = format!("{}.bin", index);
		let mut file = match File::open(path.join(filename)) {
			Ok(f) => f,
			Err(e) => {
				godot_print!("Error loading volume {}: {}", index, e);
				return None;
			},
		};

		let mut header = [0; FILE_HEADER.len()];
		file.read_exact(&mut header).unwrap();

		if header != FILE_HEADER {
			godot_print!("File header mismatch in volume {}", index);
			return None;
		}

		let mut new_volume = Self::new();

		let mut transform_bytes = [0; size_of::<Transform>()];
		file.read_exact(&mut transform_bytes).unwrap();
		let transform: Transform = unsafe { transmute_copy(&transform_bytes) };
		unsafe {
			new_volume.node.assume_safe().set_transform(transform);
		}

		let mut voxel_data = [0; VOLUME];
		let mut chunks = 0;
		loop {
			let mut loc_bytes = [0; size_of::<ChunkLoc>()];
			if file.read_exact(&mut loc_bytes).is_err() {
				break;
			}
			let loc = ChunkLoc::from_bytes(loc_bytes);

			file.read_exact(&mut voxel_data).unwrap();
			let chunk = Chunk { voxels: Box::new(voxel_data) };
			new_volume.chunks.insert(loc, chunk);
			new_volume.modified.push(loc);
			chunks += 1;
		}
		godot_print!("added {} chunks", chunks);
		new_volume.mesh_modified();
		Some(new_volume)
	}

	pub fn export(&self, file: &mut File, index: usize) {
		let header = format!("\no Volume{}\ns off", index);
		file.write_all(header.as_bytes()).unwrap();

		let mesh = unsafe {self.mesh.assume_safe()};
		let verts = mesh.get_faces();
		let vert_count = verts.len() as usize;
		let verts = verts.read();
		for i in (0..vert_count).step_by(3) {
			let face = format!(
				"\nv {} {} {}\nv {} {} {}\nv {} {} {}\nf {} {} {}",
				verts[i].x, verts[i].y, verts[i].z,
				verts[i+1].x, verts[i+1].y, verts[i+1].z,
				verts[i+2].x, verts[i+2].y, verts[i+2].z,
				i+1, i+2, i+3/* why does OBJ index from 1?? */
			);
			file.write_all(&face.into_bytes()).unwrap();
		}
	}
}


fn locs_in_sphere(pos: Vector3, radius: f32) -> Vec<ChunkLoc> {
	let center = ChunkLoc::from_wpos(pos);
	let chunk_r = (radius / WIDTH_F) as i16 + 1;
	let mut out = Vec::new();

	for x in (-chunk_r)..(chunk_r+1) {
		for y in (-chunk_r)..(chunk_r+1) {
			for z in (-chunk_r)..(chunk_r+1) {
				let loc = center.add((x, y, z));
				out.push(loc);
			}
		}
	}
	out
}


pub trait ChunkLocT {
	fn from_wpos(wpos: Vector3) -> Self;
	fn as_wpos(&self) -> Vector3;
	fn add(&self, other: Self) -> Self;
	fn as_bytes(&self) -> [u8; 6];
	fn from_bytes(bytes: [u8; 6]) -> Self;
}

impl ChunkLocT for ChunkLoc {
	#[inline]
	fn from_wpos(wpos: Vector3) -> Self {
		let p = (wpos / WIDTH_F).floor();
		(p.x as i16, p.y as i16, p.z as i16)
	}
	
	fn as_wpos(&self) -> Vector3 {
		Vector3::new(
			self.0 as f32 * WIDTH_F,
			self.1 as f32 * WIDTH_F,
			self.2 as f32 * WIDTH_F
		)
	}
	
	#[inline]
	fn add(&self, other: Self) -> Self {
		(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}

	fn as_bytes(&self) -> [u8; 6] {
		let x = self.0.to_le_bytes();
		let y = self.1.to_le_bytes();
		let z = self.2.to_le_bytes();
		[x[0], x[1], y[0], y[1], z[0], z[1]]
	}

	fn from_bytes(bytes: [u8; 6]) -> Self {
		let x: [u8; 2] = [bytes[0], bytes[1]];
		let y: [u8; 2] = [bytes[2], bytes[3]];
		let z: [u8; 2] = [bytes[4], bytes[5]];
		(
			i16::from_le_bytes(x),
			i16::from_le_bytes(y),
			i16::from_le_bytes(z),
		)
	}
}
