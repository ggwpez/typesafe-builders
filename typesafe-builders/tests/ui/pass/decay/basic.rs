mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		x: u8,
		#[builder(decay)]
		y: Option<u8>,
		#[builder(decay)]
		z: Box<u8>,
	}
}

fn main() {
	other::Struct::builder().x(5).y(8).z(4).build();
}
