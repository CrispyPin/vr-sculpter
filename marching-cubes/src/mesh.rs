use gdnative::{prelude::*, api::Mesh};

use crate::chunk::*;

pub fn generate(chunk: &ChunkData, surface_level: Voxel) -> VariantArray {
	let mut vertexes: PoolArray<Vector3> = PoolArray::new();

	for i in 0..VOLUME {
		if chunk.get_raw(i) > surface_level {
			let pos = VPos::from_index(i).vector();
			vertexes.push(pos);
			vertexes.push(pos + Vector3::new(1.0, 0.0, 1.0));
			vertexes.push(pos + Vector3::new(1.0, 0.0, 0.0));
		}
	}

	let mesh_data = VariantArray::new_thread_local();
	mesh_data.resize(Mesh::ARRAY_MAX as i32);
	mesh_data.set(Mesh::ARRAY_VERTEX as i32, &vertexes);
	unsafe { mesh_data.assume_unique().into_shared() }
}