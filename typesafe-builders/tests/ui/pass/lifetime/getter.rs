mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a> {
		x: &'a str,
	}
}

fn main() {
	let partial = other::Struct::builder();
	assert_eq!(partial.x, None);
	let partial = partial.x("hello");
	assert_eq!(partial.x, Some("hello"));
	partial.build();
}
