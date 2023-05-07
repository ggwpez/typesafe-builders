mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		#[builder(decay)]
		x: u8,
	}
}

fn main() {
}
