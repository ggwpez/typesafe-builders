mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	struct Struct {
		x: u8,
		y: u8,
	}
}

fn main() {
	let _ = other::Struct::builder();
}
