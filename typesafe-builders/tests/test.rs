#![cfg(test)]

use typesafe_builders::prelude::*;

#[test]
fn it_works() {
	#[derive(Builder)]
	struct Struct {
		mandatory: u8,
		optional: Option<u8>,
	}

	let builder = Struct::builder();
}
