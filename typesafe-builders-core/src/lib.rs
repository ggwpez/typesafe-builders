pub trait BuilderFactory<Builder> {
	fn builder() -> Builder;
}
