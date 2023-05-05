mod other {
	use typesafe_builders::prelude::*;

	// The builder name of `Struct` is `StructBuilder`.

	#[derive(Builder)]
	pub struct Struct {
		x: u8,
	}

	#[derive(Builder)]
	pub struct CtorStruct {
		#[builder(constructor)]
		x: u8,
	}
}
use other::*;

fn main() {
	let _builder: StructBuilder = Struct::builder();
	let _builder: CtorStructBuilder = CtorStruct::builder(1);
}
