mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<T, S> {
		#[builder(decay)]
		pub x: Option<T>,
		
		#[builder(decay)]
		pub y: Option<Option<S>>,

		#[builder(decay)]
		pub z: Option<(T, S)>,

		#[builder(decay)]
		pub w: Box<[u8; 2]>,
	}
}

fn main() {
	other::Struct::<u8, f32>::builder()
		.x(4)
		.y(Some(4.0))
		.z((4, 4.0))
		.w([4, 4])
		.build();
}
