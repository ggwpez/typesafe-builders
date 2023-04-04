#![cfg(test)]
use typesafe_builders::prelude::*;

#[test]
fn example() {
	#[derive(Builder)]
	struct Point {
		#[optional]
		x: u8,
		y: u8,
	}

	let builder = Point::builder();
	let partial = builder.x(5);
	// These do not compile:
	// partial.x(8);
	// partial.build();

	// Set all fields to enable the `build` function:
	let complete = partial.y(8);
	let result = complete.build();

	assert_eq!(result.x, 5);
	assert_eq!(result.y, 8);
}
