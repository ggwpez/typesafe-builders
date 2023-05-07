use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	#[builder(constructor)]
	x: u8,
}

fn main() {
	Struct::builder().build();
}
