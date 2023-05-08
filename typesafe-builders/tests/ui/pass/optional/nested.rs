mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		#[builder(optional)]
		x: Option<Option<u8>>,
		#[builder(optional)]
		y: Box<Option<u8>>,
	}
}
use other::Struct;

fn main() {
	Struct::builder().build();
	
	Struct::builder().x(Some(Some(5))).build();
	Struct::builder().y(Box::new(Some(6))).build();

	Struct::builder().x(Some(Some(5))).y(Box::new(Some(6))).build();
	Struct::builder().x(Some(Some(5))).y(Box::new(None)).build();
}
