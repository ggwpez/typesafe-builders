mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a> {
		x: &'a str,
	}
}

fn main() {
	other::Struct::builder().x("hello").build();
	other::Struct::<'static>::builder().x("hello").build();
}
