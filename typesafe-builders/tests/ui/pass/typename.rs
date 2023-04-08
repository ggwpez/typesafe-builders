use typesafe_builders::prelude::*;

// The builder name of `Struct` is `StructBuilder`.

#[derive(Builder)]
struct Struct {
	x: u8,
}

#[derive(Builder)]
struct CtorStruct {
	#[builder(constructor)]
	x: u8,
}

fn main() {
	let _builder: StructBuilder = Struct::builder();
	let _builder: CtorStructBuilder = CtorStruct::builder(1);
}
