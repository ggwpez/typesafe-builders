#![allow(non_upper_case_globals)]

pub trait BuilderFactory<Builder> {
	fn builder() -> Builder;
}
