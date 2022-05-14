use gdnative::prelude::*;

pub const WIDTH: usize = 32;
pub const WIDTH_F: f32 = WIDTH as f32;
pub const WIDTH_I8: i8 = WIDTH as i8;
pub const AREA: usize = WIDTH * WIDTH;
pub const VOLUME: usize = WIDTH * AREA;

pub type Voxel = u8;
pub type VPos = (i8, i8, i8);


pub type Chunk = Option<ChunkData>;
// pub enum Chunk {
// 	Ready(ChunkData),
// 	Empty,
// }

pub struct ChunkData {
	voxels: Box<[Voxel; VOLUME]>,
}


impl ChunkData {
	pub fn new() -> Self {
		Self {
			voxels: vec![0u8; VOLUME].into_boxed_slice().try_into().unwrap(),
		}
	}
	
	pub fn sphere(&mut self, center: Vector3, radius: f32, value: Voxel) {
		for i in 0..VOLUME {
			let pos = VPos::from_index(i).vector();
			let dist = pos.distance_to(center);
			if dist < radius {
				self.voxels[i] = value;
			}
		}
	}

	#[inline]
	pub fn get(&self, pos: VPos) -> Voxel {
		if pos.in_bounds() {
			self.voxels[pos.index()]
		}
		else {
			0
		}
	}

	#[inline]
	pub fn get_raw(&self, index: usize) -> Voxel{
		self.voxels[index]
	}

	#[inline]
	pub fn set(&mut self, pos: VPos, voxel: Voxel) {
		if pos.in_bounds() {
			self.set_unchecked(pos, voxel);
		}
	}

	#[inline]
	fn set_unchecked(&mut self, pos: VPos, voxel: Voxel) {
		self.voxels[pos.index()] = voxel;
	}
}

#[inline]
pub fn to_local(world_pos: Vector3) -> VPos {
	let p = world_pos.posmod(WIDTH_F);
	(p.x as i8, p.y as i8, p.z as i8)
}


pub trait VPosT {
	fn index(&self) -> usize;
	fn vector(&self) -> Vector3;
	fn from_index(index: usize) -> Self;
	fn in_bounds(&self) -> bool;

	fn add(&self, other: Self) -> Self;
}

impl VPosT for VPos {
	#[inline]
	fn index(&self) -> usize {
		self.0 as usize * AREA
		+ self.1 as usize * WIDTH
		+ self.2 as usize
	}

	#[inline]
	fn from_index(index: usize) -> Self{
		(
			(index / AREA) as i8,
			(index / WIDTH % WIDTH) as i8,
			(index % WIDTH) as i8
		)
	}

	#[inline]
	fn in_bounds(&self) -> bool {
		self.0 >= 0 && self.0 < WIDTH_I8 &&
		self.1 >= 0 && self.1 < WIDTH_I8 &&
		self.2 >= 0 && self.2 < WIDTH_I8
	}

	#[inline]
	fn vector(&self) -> Vector3 {
		Vector3::new(self.0 as f32, self.1 as f32, self.2 as f32)
	}

	#[inline]
	fn add(&self, other: Self) -> Self {
		(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}
}
