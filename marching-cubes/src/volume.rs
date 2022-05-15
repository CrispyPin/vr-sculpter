use gdnative::{prelude::*, api::{MeshInstance, ArrayMesh, Material, Mesh}};
use std::{collections::HashMap, mem};

use crate::chunk::*;
use crate::mesh;


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

	pub fn brush_add(&mut self, pos: Vector3, radius: f32) {
		let center_chunk = ChunkLoc::from_wpos(pos);
		let chunk_r = (radius / WIDTH_F) as i16 + 1;

		for x in (-chunk_r)..(chunk_r+1) {
			for y in (-chunk_r)..(chunk_r+1) {
				for z in (-chunk_r)..(chunk_r+1) {
					let loc = center_chunk.add((x, y, z));
					let chunk = self.ensure_chunk(loc);
					let local_pos = pos - loc.as_wpos();
					chunk.sphere(local_pos, radius, 255);
					self.modified.push(loc);
				}
			}
		}
	}

	pub fn mesh_modified(&mut self) {
		let modified = mem::take(&mut self.modified);
		for loc in modified {
			if self.surface_indexes.contains_key(&loc) {
				let i = *self.surface_indexes.get(&loc).unwrap();
				self.surface_indexes.values_mut().for_each(|x: &mut i64|
					if *x > i {
						*x -= 1;
					}
				);
				unsafe { self.mesh.assume_safe().surface_remove(i) };
			}
			self.remesh_chunk(loc);
		}
	}

	pub fn mesh_all(&mut self) {
		let mesh = unsafe { self.mesh.assume_safe() };
		mesh.clear_surfaces();
		
		let mut to_mesh = Vec::new();
		for &loc in self.chunks.keys() {
			to_mesh.push(loc);
		}

		for loc in to_mesh {
			self.remesh_chunk(loc);
		}
	}
	
	fn remesh_chunk(&mut self, loc: ChunkLoc) {
		let mesh = unsafe { self.mesh.assume_safe() };
		let chunks = ChunkBox::new([
			self.chunks.get(&loc),
			self.chunks.get(&loc.add((1, 0, 0))),
			self.chunks.get(&loc.add((0, 1, 0))),
			self.chunks.get(&loc.add((1, 1, 0))),
			self.chunks.get(&loc.add((0, 0, 1))),
			self.chunks.get(&loc.add((1, 0, 1))),
			self.chunks.get(&loc.add((0, 1, 1))),
			self.chunks.get(&loc.add((1, 1, 1))),
		]);

		let offset = loc.as_wpos();
		let mesh_data = mesh::generate(chunks, offset, self.surface_level);
		if let Some(mesh_data) = mesh_data {
			self.surface_indexes.insert(loc, mesh.get_surface_count());
			mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, mesh_data, VariantArray::new_shared(), 0);
		}
	}

	fn ensure_chunk(&mut self, loc: ChunkLoc) -> &mut Chunk{
		if !self.chunks.contains_key(&loc) {
			self.create_chunk(loc);
		}
		self.chunks.get_mut(&loc).unwrap()
	}

	fn create_chunk(&mut self, loc: ChunkLoc) {
		godot_print!("added chunk at {:?}", loc);
		self.chunks.insert(loc, Chunk::new());
	}
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
