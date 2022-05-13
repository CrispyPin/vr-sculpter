use gdnative::{prelude::*, api::{MeshInstance, ArrayMesh, Material, Mesh}};
use std::collections::HashMap;

use crate::chunk::*;
use crate::mesh;


pub type ChunkLoc = (i16, i16, i16);


pub struct Volume {
	chunks: HashMap<ChunkLoc, Chunk>,
	surface_indexes: HashMap<ChunkLoc, usize>,
	node: Ref<MeshInstance>,
	mesh: Ref<ArrayMesh>,
	surface_level: u8,
	material: Option<Ref<Material>>,
}

impl Volume {
	pub fn new() -> Self {
		let mesh = ArrayMesh::new().into_shared();
		let mesh_node = unsafe { MeshInstance::new().into_shared().assume_safe() };
		mesh_node.set_mesh(&mesh);
		
		let mut new = Self {
			chunks: HashMap::new(),
			surface_indexes: HashMap::new(),
			node: mesh_node.claim(),
			mesh,
			material: None,
			surface_level: 128,
		};
		let mut data = ChunkData::new();
		data.sphere(Vector3::new(15.0,15.0,15.0), 10.0, 255);
		new.chunks.insert((0,0,0), Some(data));
		new
	}

	pub fn node(&self) -> Ref<MeshInstance> {
		self.node
	}

	pub fn mesh_all(&mut self) {
		let mesh = unsafe { self.mesh.assume_safe() };
		mesh.clear_surfaces();

		for c in self.chunks.values() {
			if let Some(data) = c {
				mesh.add_surface_from_arrays(Mesh::PRIMITIVE_TRIANGLES, mesh::generate(data, self.surface_level), VariantArray::new_shared(), 0);
			}
		}
	}
}
