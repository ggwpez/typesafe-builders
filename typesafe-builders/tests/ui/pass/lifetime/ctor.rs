mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a> {
		#[builder(constructor)]
		x: &'a str,
	}
}

fn main() {
	other::Struct::builder("hi").build();
}
