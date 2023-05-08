mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a, 'b> {
		#[builder(decay)]
		x: Option<&'a str>,
		#[builder(decay)]
		y: Box<&'b Option<&'a str>>,
	}
}

fn main() {
	other::Struct::builder().x("hi").y(&Some("hi")).build();
}
