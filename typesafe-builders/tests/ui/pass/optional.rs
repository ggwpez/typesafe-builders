use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Struct {
	#[builder(optional)]
	x: Option<u8>,
	#[builder(optional)]
	y: Option<u8>,
}

fn main() {
	Struct::builder().build();
	
	Struct::builder().x(Some(1)).build();
	Struct::builder().x(None).build();
	Struct::builder().y(Some(1)).build();
	Struct::builder().y(None).build();

	Struct::builder().x(Some(1)).y(Some(1)).build();
	Struct::builder().x(None).y(Some(1)).build();
	Struct::builder().x(Some(1)).y(None).build();

	Struct::builder().y(Some(1)).x(Some(1)).build();
	Struct::builder().y(None).x(Some(1)).build();
	Struct::builder().y(Some(1)).x(None).build();

	Struct::builder().y(None).x(None).build();
}
