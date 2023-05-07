/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]
#![allow(non_upper_case_globals)]

use quote::quote;
use syn::spanned::Spanned;

pub type FieldAttrs = std::collections::HashMap<FieldAttrId, FieldAttrVal>;

#[derive(Debug)]
pub struct FieldAttr {
	pub id: FieldAttrId,
	pub val: FieldAttrVal,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FieldAttrId {
	/// A field *can*, but does not *have* to be set.
	Optional,
	/// A field *must* be set directly in the `builder` constructor.
	Constructor,
	/// Decay the type once for the setter function.
	///
	/// Eg `Option<T>` can be set directly instead of having to wrap it in `Some(_)`.
	///
	/// Note: The wording `decay` comes from C++ - maybe someone can point out a more *Rusty* term.
	Decay,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FieldAttrVal {
	/// Inherent the default value. This is the default.
	Inherit,

	/// Explicitly override the value to `true` or `false`.
	Override(bool),
}

#[derive(derive_syn_parse::Parse)]
struct ParsedFieldAttr {
	id: syn::Ident,
	_t: Option<syn::Token![=]>,
	#[parse_if(_t.is_some())]
	val: Option<syn::LitBool>,
}

pub fn impl_derive_builder(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
	let syn::Data::Struct(ref s) = ast.data else {
		return Err(syn::Error::new_spanned(
			ast,
			"derive(Builder) can only be used on structs",
		));
    };
	let user_generics = ast.generics.params.clone().into_iter().collect::<Vec<_>>();

	let vis = ast.vis.clone();
	let name = &ast.ident;
	let builder_ident = syn::Ident::new(&format!("Generic{}Builder", name), name.span());

	let (mut builder_field_names, mut builder_field_types, mut builder_fields) =
		(vec![], vec![], vec![]);
	let mut builder_field_assignments = vec![];
	let mut builder_const_generics = vec![];
	let mut builder_const_generics_all_unset = vec![];
	let mut builder_const_generics_all_set = vec![];

	// The dont-cares for optional fields:
	let mut builder_builder_func_generics = vec![];
	let mut build_function_generic_values = vec![];

	let mut constructor_arg_names = vec![];
	let mut constructor_arg_values = vec![];
	let mut constructor_args = vec![];

	let mut setters = vec![];
	for (i, field) in s.fields.iter().enumerate() {
		let Some(field_name) = field.ident.clone() else {
			return Err(syn::Error::new(field.span(), "Builder does not work on unnamed fields"));
		};
		let field_type = &field.ty;
		let field_attrs = extract_attributes(field)?;

		let (setter_type, setter_val) = if is_decay(&field_attrs) {
			// TODO
			(decay_type(field_type)?, quote! { Some(#field_name.into()) })
		} else {
			(field_type.clone(), quote! { Some(#field_name) })
		};

		if is_ctor(&field_attrs) {
			constructor_args.push(quote! {#field_name: #setter_type});
			constructor_arg_values.push(setter_val.clone());
			constructor_arg_names.push(field_name.clone());
		}

		if is_optional(&field_attrs) {
			let setter_name = syn::Ident::new(
				&format!("{}_set", field.ident.clone().unwrap()).to_uppercase(),
				field.ident.span(),
			);
			builder_builder_func_generics.push(quote! { const #setter_name: bool });
			build_function_generic_values.push(quote! { #setter_name });
			builder_field_assignments
				.push(quote! {#field_name: self.#field_name.unwrap_or_default()});
		} else {
			build_function_generic_values.push(quote! { true });
			builder_field_assignments.push(quote! {#field_name: self.#field_name.unwrap() });
		}

		builder_field_names.push(field_name.clone());
		builder_field_types.push(field_type.clone());
		builder_fields.push(quote! { #field_name: Option<#field_type> });
		let const_generic_name =
			syn::Ident::new(&format!("{}_set", field_name).to_uppercase(), field_name.span());
		builder_const_generics.push(quote! { const #const_generic_name: bool });
		if is_ctor(&field_attrs) {
			builder_const_generics_all_unset.push(quote! { true });
		} else {
			builder_const_generics_all_unset.push(quote! { false });
		}
		builder_const_generics_all_set.push(quote! { true });

		let setter_name = syn::Ident::new(&format!("{}", field_name), field_name.span());
		let mut const_generics = vec![];
		let mut const_generic_vars = vec![];
		let mut const_generic_return_vars = vec![];
		let mut all_expect_field_name = vec![];

		for (j, field) in s.fields.iter().enumerate() {
			if i != j {
				let field_name = syn::Ident::new(
					&format!("{}_set", field.ident.clone().unwrap()),
					field.ident.span(),
				);
				const_generics.push(quote! { const #field_name: bool });
				const_generic_vars.push(quote! { #field_name });
				const_generic_return_vars.push(quote! { #field_name });
				let ident = field.ident.clone().unwrap();
				all_expect_field_name.push(quote! { #ident });
			} else {
				const_generic_vars.push(quote! { false });
				const_generic_return_vars.push(quote! { true });
			}
		}
		setters.push(quote! {
			#[allow(non_upper_case_globals)]
			impl<
				#(#user_generics,)*
				#(#const_generics),*
			>
			#builder_ident<
				#(#user_generics,)*
				#(#const_generic_vars),*
			> {
				#[allow(dead_code)]
				pub fn #setter_name(self, #field_name: #setter_type) -> #builder_ident<
					#(#user_generics,)*
					#(#const_generic_return_vars),*
				> {
					#builder_ident {
						#field_name: #setter_val,
						#(#all_expect_field_name: self.#all_expect_field_name),*
					}
				}
			}
		});
	}
	let build_fn = quote! {
		impl<
			#(#user_generics,)*
			#(#builder_builder_func_generics),*
		> #builder_ident<
			#(#user_generics,)*
			#(#build_function_generic_values),*
		> {
			/// Infallible build the instance.
			#[allow(dead_code)]
			pub fn build(self) -> #name<#(#user_generics,)*> {
				#name {
					#(#builder_field_assignments),*
				}
			}
		}
	};

	let builder_struct = quote! {
		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		#vis struct #builder_ident<
			#(#user_generics,)*
			#(#builder_const_generics),
		*> {
			#(pub #builder_field_names: Option<#builder_field_types>),*
		}
	};

	let default_builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());

	let default_builder_type = quote! {
		#vis type #default_builder_name<
			#(#user_generics,)*
		> = #builder_ident<
			#(#user_generics,)*
			#(#builder_const_generics_all_unset),*
		>;
	};

	let builder = quote! {
		#builder_struct

		#default_builder_type

		#(#setters)*
		#build_fn
	};

	// Remove all constructor args from builder_field_names
	for field in constructor_arg_names.iter() {
		builder_field_names.retain(|x| x != field);
	}

	let gen = quote! {
		impl<#(#user_generics,)*> #name<#(#user_generics,)*> {
			#[allow(dead_code)]
			pub fn builder(#(#constructor_args),*) -> #builder_ident<
				#(#user_generics,)*
				#(#builder_const_generics_all_unset),*
			> {
				#builder_ident {
					#(#builder_field_names: None,)*
					#(#constructor_arg_names: #constructor_arg_values,)*
				}
			}
		}
	};

	Ok(quote! {
		#builder
		#gen
	})
}

fn extract_attributes(field: &syn::Field) -> syn::Result<FieldAttrs> {
	let mut field_attrs = FieldAttrs::new();

	for raw_attr in field.attrs.iter() {
		let attr: FieldAttr = parse_builder_attribute(raw_attr)?;
		if field_attrs.contains_key(&attr.id) {
			return Err(syn::Error::new_spanned(raw_attr, "Duplicate attribute on struct field"))
		}
		field_attrs.insert(attr.id, attr.val);
	}

	if is_optional(&field_attrs) && is_ctor(&field_attrs) {
		return Err(syn::Error::new_spanned(field, "Optional fields cannot be in the constructor"))
	}

	Ok(field_attrs)
}

fn parse_builder_attribute(attr: &syn::Attribute) -> syn::Result<FieldAttr> {
	match &attr.meta {
		syn::Meta::List(syn::MetaList { path, tokens, .. }) => {
			let spath = path_to_string(path);
			if spath != "builder" {
				return Err(syn::Error::new_spanned(
					attr,
					format!("Expected attribute `builder`, got {}", spath),
				))
			}

			let attr = syn::parse2::<ParsedFieldAttr>(tokens.clone())?;
			let id: FieldAttrId = attr.id.try_into()?;
			let val = attr.val.map(|v| v.value).unwrap_or(true);

			Ok(FieldAttr { id, val: FieldAttrVal::Override(val) })
		},
		_ => Err(syn::Error::new_spanned(attr, "Expected builder attribute to be a list")),
	}
}

/// Whether a field is optional.
fn is_optional(attrs: &FieldAttrs) -> bool {
	match attrs.get(&FieldAttrId::Optional).unwrap_or(&FieldAttrVal::Inherit) {
		FieldAttrVal::Override(o) => *o,
		FieldAttrVal::Inherit => false,
	}
}

/// Whether a field is optional.
fn is_decay(attrs: &FieldAttrs) -> bool {
	match attrs.get(&FieldAttrId::Decay).unwrap_or(&FieldAttrVal::Inherit) {
		FieldAttrVal::Override(o) => *o,
		FieldAttrVal::Inherit => false,
	}
}

/// Whether a field needs to be set in the constructor `builder` function.
fn is_ctor(attrs: &FieldAttrs) -> bool {
	match attrs.get(&FieldAttrId::Constructor).unwrap_or(&FieldAttrVal::Inherit) {
		FieldAttrVal::Override(o) => *o,
		FieldAttrVal::Inherit => false,
	}
}

impl TryFrom<syn::Ident> for FieldAttrId {
	type Error = syn::Error;

	fn try_from(ident: syn::Ident) -> Result<Self, Self::Error> {
		match ident.to_string().as_str() {
			"optional" => Ok(FieldAttrId::Optional),
			"constructor" => Ok(FieldAttrId::Constructor),
			"decay" => Ok(FieldAttrId::Decay),
			e => Err(syn::Error::new(ident.span(), format!("Unknown field attribute: {:?}", e))),
		}
	}
}

fn path_to_string(p: &syn::Path) -> String {
	p.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("")
}

fn decay_type(t: &syn::Type) -> syn::Result<syn::Type> {
	let syn::Type::Path(p) = t else {
		return Ok(t.clone());
	};

	let last = p.path.segments.last().unwrap();
	let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
		return Err(syn::Error::new(last.span(), "Expected one generic argument but got none"));
	};
	// TODO check for not-time
	if args.args.len() != 1 {
		return Err(syn::Error::new(
			args.args.span(),
			&format!("Need exactly one generic argument, but got {}", args.args.len()),
		))
	}
	match args.args.first().unwrap() {
		syn::GenericArgument::Type(inner) => Ok(inner.clone()),
		_ => todo!(),
	}
}
