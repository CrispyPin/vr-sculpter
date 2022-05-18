# marching-cubes

## Todo:
- [X] marching cubes meshing
- [X] smoothing
- [ ] multithreading mesh generation
- [ ] multithreading smooth operations
- [ ] smoothing with bigger radius
- [ ] name
- [ ] VR support
- [ ] saving/loading
- [ ] exporting mesh


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

Chunk {
	voxels: Box<[u8; 32^3]>
}

```
