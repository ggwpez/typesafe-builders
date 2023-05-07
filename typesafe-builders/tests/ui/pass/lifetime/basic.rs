mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a> {
		x: &'a str,
	}
}

fn main() {
	other::Struct::builder().x("hello").build();
}
