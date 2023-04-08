use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	#[builder(yikes)]
	x: u8,
}

fn main() { }
