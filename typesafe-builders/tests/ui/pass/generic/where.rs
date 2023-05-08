mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<T: Clone> {
		x: T,
		y: Option<T>,
	}
}

fn main() {
	other::Struct::<u8>::builder().x(5).y(Some(4)).build();
}
