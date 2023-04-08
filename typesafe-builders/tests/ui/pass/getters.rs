use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	x: u8,
	y: u8,
}

fn main() {
	assert_eq!(Struct::builder().x, None);
	assert_eq!(Struct::builder().x(1).x, Some(1));

	assert_eq!(Struct::builder().y(2).x, None);
	assert_eq!(Struct::builder().y(2).y, Some(2));

	assert_eq!(Struct::builder().x(1).y(2).x, Some(1));
	assert_eq!(Struct::builder().x(1).y(2).y, Some(2));
}
