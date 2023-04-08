use typesafe_builders::prelude::*;

//! `Option` is *not* treated as optional.

#[derive(Builder)]
struct Struct {
	x: Option<u8>,
}

fn main() {
	Struct::builder().build();
}
