use gdnative::{prelude::*, api::{MeshInstance, ArrayMesh, Material, Mesh}};
use rayon::prelude::*;
use std::{collections::HashMap, mem, time::Instant};

use crate::chunk::*;
use crate::mesh;

const DEBUG_MESH_TIMES: bool = false;
const DEBUG_SMOOTH_TIMES: bool = false;


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
}
