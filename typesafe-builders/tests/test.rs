#![cfg(test)]

use typesafe_builders::prelude::*;

#[test]
fn example() {
	#[derive(Builder)]
	struct Point {
		x: u8,
		y: u8,
	}

	let builder = Point::builder();
	let partial = builder.with_x(5);
	// These do not compile:
	// partial.with_x(8);
	// partial.build();

	// Infallibly construct an instance:
	let result = partial.with_y(8).build();

	assert_eq!(result.x, 5);
	assert_eq!(result.y, 8);
}
