use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	#[builder(constructor)]
	x: u8,
}

fn main() {
	let r = Struct::builder(2).x(3).build();
}
