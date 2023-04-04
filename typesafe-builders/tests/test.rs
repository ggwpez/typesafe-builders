/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

#![cfg(test)]
use typesafe_builders::prelude::*;

#[test]
fn example() {
	#[derive(Builder)]
	struct Point {
		x: u8,
		y: u8,
		#[optional]
		z: Option<u8>,
	}

	let builder = Point::builder();
	let partial = builder.x(5);
	// These do not compile:
	// partial.x(6); 		// `x` is already set
	// partial.build();		// `y` is not set

	// Set all required fields to enable the `build` function:
	let complete = partial.y(8);
	let result = complete.build();

	assert_eq!(result.x, 5);
	assert_eq!(result.y, 8);
	assert_eq!(result.z, None);
}
