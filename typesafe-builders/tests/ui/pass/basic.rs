mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		x: u8,
		y: u8,
		z: Option<u8>,
	}
}

fn main() {
	other::Struct::builder().x(5).y(8).z(None).build();
	other::Struct::builder().x(5).y(8).z(Some(7)).build();
}
