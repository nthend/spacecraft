use num::*;
use la::vec::*;
use la::mat::*;

use core::pos::*;
use core::ori::*;

use gl4u::gl::types::*;

use view::model::*;
use view::proj::*;

pub struct Camera {
	pub pos_: vec3d,
	pub ori_: mat3d,
	pub proj: Proj,
}

impl Camera {
	pub fn new() -> Self {
		Camera {
			pos_: vec3d::zero(),
			ori_: mat3d::one(),
			proj: Proj::new(),
		}
	}

	pub fn aspect(&mut self, ar: f64) {
		let n = self.proj.n;
		self.proj.set_wh(ar*n, n);
	}
}

impl_pos_mut!(Camera, pos_);
impl_ori_mut!(Camera, ori_);

impl_model!(Camera);