mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<T> {
		x: T,
	}
}

fn main() {
	other::Struct::builder().x(5).build();
}
