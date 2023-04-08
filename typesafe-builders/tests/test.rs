/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

#![cfg(test)]
use typesafe_builders::prelude::*;

mod ui;

#[test]
fn example() {
	#[derive(Builder)]
	struct Point {
		#[builder(constructor)]
		x: u8,
		y: u8,
		#[builder(optional)]
		z: Option<u8>,
	}

	let builder = Point::builder(1);
	let partial = builder.y(2);
	// These do not compile:
	// partial.x(6); 		// `x` is already set
	// partial.build();		// `y` is not set

	// Set all required fields to enable the `build` function:
	let complete = partial.z(Some(3));
	let result = complete.build();

	assert_eq!(result.x, 1);
	assert_eq!(result.y, 2);
	assert_eq!(result.z, Some(3));
}
