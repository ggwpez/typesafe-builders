mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct {
		#[builder(decay)]
		y: Option<Box<u8>>,
		#[builder(decay)]
		z: Box<Option<u8>>,
	}

	#[derive(Builder)]
	pub struct Optional {
		#[builder(decay)]
		#[builder(optional)]
		y: Option<Box<u8>>,
		#[builder(decay)]
		#[builder(optional)]
		z: Box<Option<u8>>,
	}
}
use other::*;

fn main() {
	Struct::builder().y(Box::new(8)).z(Some(4)).build();
	Struct::builder().y(8.into()).z(4.into()).build();

	Optional::builder().build();
	Optional::builder().y(Box::new(8)).build();
	Optional::builder().z(Some(4)).build();
	Optional::builder().y(Box::new(8)).z(Some(4)).build();
	Optional::builder().y(8.into()).z(4.into()).build();
}
