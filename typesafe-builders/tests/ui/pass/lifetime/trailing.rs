mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a> {
		x: &'a u8,
	}
}

fn main() {
}
