mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		#[builder(constructor)]
		pub x: u8,
	}
}

fn main() {
	let r = other::Struct::builder(2).build();
	assert_eq!(r.x, 2);
}
