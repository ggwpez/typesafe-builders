mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<'a, T: ?Sized> {
		x: &'a T,
	}
}

fn main() {
	other::Struct::<str>::builder().x("5").build();
}
