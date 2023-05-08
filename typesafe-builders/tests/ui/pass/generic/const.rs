mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<const LEN: usize> {
		x: [u8; LEN],
	}

	//#[derive(Builder)]
	//pub struct Struct2<const LEN: usize = 1> {
	//	x: [u8; LEN],
	//} TODO
}

fn main() {
	other::Struct::<1>::builder().x([1]).build();
}
