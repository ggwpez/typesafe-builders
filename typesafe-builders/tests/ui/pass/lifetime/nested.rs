mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a, 'b, 'c> {
		x: &'a Box<&'b Option<&'c str>>, // yikes
	}
}

fn main() {
	other::Struct::builder().x(&Box::new(&Some("hi"))).build();
}
