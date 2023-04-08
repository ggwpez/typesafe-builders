use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	x: u8,
}

fn main() {
	Struct::builder().build();
}
