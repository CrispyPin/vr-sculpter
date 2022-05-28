# marching-cubes

## Todo:
- [X] marching cubes meshing
- [X] smoothing
- [X] multithreaded mesh generation
- [X] multithreaded smoothing operations
- [X] saving/loading
- [X] exporting as mesh
- [ ] VR support
  - [X] VR viewing
  - [X] Controls
  - [ ] menus/meta controls
- [ ] smoothing with bigger radius
- [ ] name


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
