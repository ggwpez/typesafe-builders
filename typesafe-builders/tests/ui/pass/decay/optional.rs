mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		x: u8,
		#[builder(decay)]
		#[builder(optional)]
		y: Option<u8>,
		#[builder(decay)]
		#[builder(optional)]
		z: Box<u8>,
	}
}

fn main() {
	other::Struct::builder().x(5).build();
	other::Struct::builder().x(5).z(4).build();
	other::Struct::builder().x(5).y(8).build();
	other::Struct::builder().x(5).y(8).z(4).build();
}
