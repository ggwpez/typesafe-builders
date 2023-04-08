/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Exports the `#[derive(Builder)]` attribute macro.
//!
//! There is nothing else going on here - the implementation is in `typesafe-builders-core` since
//! rust enforces that nothing but proc macros are exported from a `proc_macro` crate…

extern crate proc_macro;
use proc_macro::TokenStream;

/// Derive a builder for your struct via `#[derive(Builder)]`.
///
/// ## Field Attributes
///
/// All attributes must be wrapped in a `builder`, eg. `builder(optional)`.
///
/// - `optional` - A field can be set, but is not required to.
/// - `constructor` - A field must already be set in the `builder` function.
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(stream: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(stream as syn::DeriveInput);
	let ts2 = typesafe_builders_core::impl_derive_builder(&ast);

	// The magical part: convert the proc macro error to a compiler error:
	ts2.unwrap_or_else(syn::Error::into_compile_error).into()
}
