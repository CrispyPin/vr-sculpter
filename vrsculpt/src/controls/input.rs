use gdnative::prelude::*;

pub struct HandState {
	pub sel_x: bool,
	pub sel_y: bool,
}

impl HandState {
	pub fn new() -> Self {
		Self {
			sel_x: false,
			sel_y: false,
		}
	}
}