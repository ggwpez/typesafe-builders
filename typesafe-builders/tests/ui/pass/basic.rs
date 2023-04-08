use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	x: u8,
	y: u8,
}

fn main() {
	Struct::builder().x(5).y(8).build();
}
