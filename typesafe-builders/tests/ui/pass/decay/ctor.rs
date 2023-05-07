mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		#[builder(decay)]
		#[builder(constructor)]
		pub x: Option<u8>,
		#[builder(decay)]
		#[builder(constructor)]
		pub y: Option<Box<u8>>,
	}
}

fn main() {
	let s = other::Struct::builder(3, 4.into()).build();
	assert_eq!(s.x, Some(3));
	assert_eq!(s.y, Some(Box::new(4)));
}
