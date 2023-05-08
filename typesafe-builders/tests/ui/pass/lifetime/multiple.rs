mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a, 'b, 'yea> {
		x: &'a str,
		y: &'b str,
		z: &'yea str,
	}
}

fn main() {
	other::Struct::builder().x("hello").y("world").z("yea").build();
}
