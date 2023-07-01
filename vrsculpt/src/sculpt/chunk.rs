use gdnative::prelude::*;
use std::collections::HashMap;

use super::volume::ChunkLoc;
use super::volume::ChunkLocT;

pub const WIDTH: usize = 32;
pub const WIDTH_F: f32 = WIDTH as f32;
pub const WIDTH_I8: i8 = WIDTH as i8;
pub const AREA: usize = WIDTH * WIDTH;
pub const VOLUME: usize = WIDTH * AREA;

pub type Voxel = u8;
pub type VPos = (i8, i8, i8);

/// 2x2x2 chunk references
pub struct ChunkBox2<'a> {
	contents: [Option<&'a Chunk>; 8],
}

/// 3x3x3 chunk references
pub struct ChunkBox3<'a> {
	contents: [Option<&'a Chunk>; 27],
}

pub struct Chunk {
	pub voxels: Box<[Voxel; VOLUME]>,
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			voxels: vec![0u8; VOLUME].into_boxed_slice().try_into().unwrap(),
		}
	}

	pub fn set_sphere(&mut self, center: Vector3, radius: f32, value: Voxel) {
		for i in 0..VOLUME {
			let pos = VPos::from_index(i).vector();
			let dist = pos.distance_to(center);
			let surface_dist = dist - radius;
			if surface_dist < 0.0 {
				self.voxels[i] = value;
			} else if surface_dist < 1.0 {
				let old = self.voxels[i];
				if old < value {
					let interpolated_value = lerp(old, value, 1.0 - surface_dist);
					self.voxels[i] = interpolated_value;
				} else {
					let interpolated_value = lerp(value, old, surface_dist);
					self.voxels[i] = interpolated_value;
				}
			}
		}
	}

	pub fn smooth_sphere(old: ChunkBox3, center: Vector3, radius: f32) -> Self {
		let mut new = Self::new();
		for i in 0..VOLUME {
			let pos = VPos::from_index(i);
			let dist = pos.vector().distance_to(center);
			let old_val = old.get(pos);
			if dist <= radius {
				let mut new_val = old_val as u16;
				new_val += old.get(pos.add((1, 0, 0))) as u16;
				new_val += old.get(pos.add((-1, 0, 0))) as u16;
				new_val += old.get(pos.add((0, 1, 0))) as u16;
				new_val += old.get(pos.add((0, -1, 0))) as u16;
				new_val += old.get(pos.add((0, 0, 1))) as u16;
				new_val += old.get(pos.add((0, 0, -1))) as u16;
				new_val /= 7;
				new.voxels[i] = old_val.max(new_val as Voxel);
			} else {
				new.voxels[i] = old_val;
			}
		}
		new
	}

	#[inline]
	pub fn get_unchecked(&self, pos: VPos) -> Voxel {
		self.voxels[pos.index()]
	}
}

#[inline]
fn lerp(a: u8, b: u8, t: f32) -> u8 {
	a + (t * (b - a) as f32) as u8
}

impl<'a> ChunkBox2<'a> {
	pub fn new(chunks: &'a HashMap<ChunkLoc, Chunk>, loc: ChunkLoc) -> Self {
		Self {
			contents: [
				chunks.get(&loc),
				chunks.get(&loc.add((1, 0, 0))),
				chunks.get(&loc.add((0, 1, 0))),
				chunks.get(&loc.add((1, 1, 0))),
				chunks.get(&loc.add((0, 0, 1))),
				chunks.get(&loc.add((1, 0, 1))),
				chunks.get(&loc.add((0, 1, 1))),
				chunks.get(&loc.add((1, 1, 1))),
			],
		}
	}

	pub fn get(&self, pos: VPos) -> Voxel {
		let chunk_pos = pos.div(WIDTH_I8);
		let local_pos = pos.modulo(WIDTH_I8);
		let index = match chunk_pos {
			(0, 0, 0) => 0,
			(1, 0, 0) => 1,
			(0, 1, 0) => 2,
			(1, 1, 0) => 3,
			(0, 0, 1) => 4,
			(1, 0, 1) => 5,
			(0, 1, 1) => 6,
			(1, 1, 1) => 7,
			_ => panic!("ChunkBox2 bounds exceeded"),
		};
		if let Some(chunk) = self.contents[index] {
			return chunk.get_unchecked(local_pos);
		}
		0
	}
}

impl<'a> ChunkBox3<'a> {
	pub fn new(chunks: &'a HashMap<ChunkLoc, Chunk>, loc: ChunkLoc) -> Self {
		Self {
			contents: [
				chunks.get(&loc.add((-1, -1, -1))),
				chunks.get(&loc.add((0, -1, -1))),
				chunks.get(&loc.add((1, -1, -1))),
				chunks.get(&loc.add((-1, 0, -1))),
				chunks.get(&loc.add((0, 0, -1))),
				chunks.get(&loc.add((1, 0, -1))),
				chunks.get(&loc.add((-1, 1, -1))),
				chunks.get(&loc.add((0, 1, -1))),
				chunks.get(&loc.add((1, 1, -1))),
				chunks.get(&loc.add((-1, -1, 0))),
				chunks.get(&loc.add((0, -1, 0))),
				chunks.get(&loc.add((1, -1, 0))),
				chunks.get(&loc.add((-1, 0, 0))),
				chunks.get(&loc.add((0, 0, 0))),
				chunks.get(&loc.add((1, 0, 0))),
				chunks.get(&loc.add((-1, 1, 0))),
				chunks.get(&loc.add((0, 1, 0))),
				chunks.get(&loc.add((1, 1, 0))),
				chunks.get(&loc.add((-1, -1, 1))),
				chunks.get(&loc.add((0, -1, 1))),
				chunks.get(&loc.add((1, -1, 1))),
				chunks.get(&loc.add((-1, 0, 1))),
				chunks.get(&loc.add((0, 0, 1))),
				chunks.get(&loc.add((1, 0, 1))),
				chunks.get(&loc.add((-1, 1, 1))),
				chunks.get(&loc.add((0, 1, 1))),
				chunks.get(&loc.add((1, 1, 1))),
			],
		}
	}

	pub fn get(&self, pos: VPos) -> Voxel {
		let chunk_pos = pos.div(WIDTH_I8);
		let index = match chunk_pos {
			(-1, -1, -1) => 0,
			(0, -1, -1) => 1,
			(1, -1, -1) => 2,
			(-1, 0, -1) => 3,
			(0, 0, -1) => 4,
			(1, 0, -1) => 5,
			(-1, 1, -1) => 6,
			(0, 1, -1) => 7,
			(1, 1, -1) => 8,
			(-1, -1, 0) => 9,
			(0, -1, 0) => 10,
			(1, -1, 0) => 11,
			(-1, 0, 0) => 12,
			(0, 0, 0) => 13,
			(1, 0, 0) => 14,
			(-1, 1, 0) => 15,
			(0, 1, 0) => 16,
			(1, 1, 0) => 17,
			(-1, -1, 1) => 18,
			(0, -1, 1) => 19,
			(1, -1, 1) => 20,
			(-1, 0, 1) => 21,
			(0, 0, 1) => 22,
			(1, 0, 1) => 23,
			(-1, 1, 1) => 24,
			(0, 1, 1) => 25,
			(1, 1, 1) => 26,
			_ => panic!("ChunkBox3 bounds exceeded"),
		};
		if let Some(chunk) = self.contents[index] {
			let local_pos = pos.posmod(WIDTH_I8);
			return chunk.get_unchecked(local_pos);
		}
		0
	}
}

pub trait VPosT {
	fn index(&self) -> usize;
	fn vector(&self) -> Vector3;
	fn from_index(index: usize) -> Self;
	fn in_bounds(&self) -> bool;

	fn add(&self, other: Self) -> Self;
	fn div(&self, other: i8) -> Self;
	fn modulo(&self, other: i8) -> Self;
	fn posmod(&self, other: i8) -> Self;
}

impl VPosT for VPos {
	#[inline]
	fn index(&self) -> usize {
		self.0 as usize * AREA + self.1 as usize * WIDTH + self.2 as usize
	}

	#[inline]
	fn from_index(index: usize) -> Self {
		(
			(index / AREA) as i8,
			(index / WIDTH % WIDTH) as i8,
			(index % WIDTH) as i8,
		)
	}

	#[inline]
	fn in_bounds(&self) -> bool {
		self.0 >= 0
			&& self.0 < WIDTH_I8
			&& self.1 >= 0
			&& self.1 < WIDTH_I8
			&& self.2 >= 0
			&& self.2 < WIDTH_I8
	}

	#[inline]
	fn vector(&self) -> Vector3 {
		Vector3::new(self.0 as f32, self.1 as f32, self.2 as f32)
	}

	#[inline]
	fn add(&self, other: Self) -> Self {
		(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}

	#[inline]
	fn div(&self, other: i8) -> Self {
		(
			self.0.div_euclid(other),
			self.1.div_euclid(other),
			self.2.div_euclid(other),
		)
	}

	#[inline]
	fn modulo(&self, other: i8) -> Self {
		(self.0 % other, self.1 % other, self.2 % other)
	}

	#[inline]
	fn posmod(&self, other: i8) -> Self {
		self.add((other, other, other)).modulo(other)
	}
}
