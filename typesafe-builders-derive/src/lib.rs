//! Exports the `#[derive(Builder)]` attribute macro.
//!
//! There is nothing else going on here - the implementation is in `typesafe-builders-core` since
//! rust enforces that nothing but proc macros are exported from a `proc_macro` crate…

extern crate proc_macro;
use proc_macro::TokenStream;

/// Derive a builder for your struct via `#[derive(Builder)]`.
///
/// # Attributes
///
/// - `#[optional]` - Mark a field as optional. This is the default if the type is an `Option`.
#[proc_macro_derive(Builder, attributes(optional))]
pub fn derive_builder(stream: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(stream as syn::DeriveInput);
	typesafe_builders_core::impl_derive_builder(&ast).unwrap()
}
