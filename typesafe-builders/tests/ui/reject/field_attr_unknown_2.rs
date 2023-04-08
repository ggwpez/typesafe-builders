use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	#[optional]
	x: u8,
}

fn main() { }
