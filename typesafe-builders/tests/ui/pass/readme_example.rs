use typesafe_builders::prelude::*;

fn main() {
	#[derive(Builder)]
	struct Point {
		#[builder(constructor)]
		x: u8,
		y: u8,
		#[builder(optional)]
		z: Option<u8>,
	}

	// The `builder` function requires `x` since it is marked as `constructor`.
	let builder = Point::builder(1);
	// These do not compile:
	// partial.x(6); 		// `x` is already set
	// partial.build();		// `y` is not set

	// Set all required fields to enable the `build` function:
	let result = builder.y(2).build();

	assert_eq!(result.x, 1);
	assert_eq!(result.y, 2);
	assert_eq!(result.z, None);
}
