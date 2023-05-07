mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a> {
		#[builder(decay)]
		pub x: Option<&'a str>,
	}
}

fn main() {
	other::Struct::<'static>::builder()
		.x("hello")
		.build();
}
