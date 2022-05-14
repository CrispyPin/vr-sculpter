# marching-cubes

## structure: 
```
VoxelObject {
	volumes: Vec<Volume>
}

Volume {
	chunks: HashMap<Chunk>,
	surface_indexes: HashMap<usize>,
	node: Ref<MeshInstance>,
	mesh: Ref<ArrayMesh>,
	surface_level: u8,
	material: Ref<Material>,
}

Chunk = Option<ChunkData>

ChunkData {
	voxels: Box<[u8; 32^3]>
}

```
