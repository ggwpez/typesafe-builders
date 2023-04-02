#![cfg(test)]

#[test]
fn example() {
	use typesafe_builders::prelude::*;
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

	// Set all fields to enable the `build` function:
	let complete = partial.with_y(8);
	let result = complete.build();

	assert_eq!(result.x, 5);
	assert_eq!(result.y, 8);
}
