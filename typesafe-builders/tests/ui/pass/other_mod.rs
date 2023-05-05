mod other {
	use typesafe_builders::prelude::*;
	
	#[derive(Builder)]
	pub struct Struct {
		x: u8,
		y: u8,
	}
}

fn main() {
	other::Struct::builder().x(5).y(8).build();
}
