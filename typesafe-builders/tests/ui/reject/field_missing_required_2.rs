//! `Option` is *not* treated as optional.

use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	x: Option<u8>,
}

fn main() {
	Struct::builder().build();
}
