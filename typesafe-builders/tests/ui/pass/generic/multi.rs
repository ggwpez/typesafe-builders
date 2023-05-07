mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<T, S> {
		x: T,
		y: Option<S>
	}
}

fn main() {
	other::Struct::<u8, f32>::builder().x(5).y(Some(4.0)).build();
}
