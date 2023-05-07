mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		#[builder(decay)]
		x: Result<(), ()>,
	}
}

fn main() {
}
