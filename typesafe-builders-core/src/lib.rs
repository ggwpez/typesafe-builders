#![allow(non_upper_case_globals)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

pub struct FieldAttrs(pub std::collections::HashMap<FieldAttrId, FieldAttrVal>);

#[derive(Debug)]
pub struct FieldAttr {
	pub id: FieldAttrId,
	pub val: FieldAttrVal,
}

#[derive(Debug)]
pub enum FieldAttrId {
	Optional,
}

#[derive(Debug)]
pub enum FieldAttrVal {
	/// Inherent the default value. This is the default.
	Inherit,

	/// Explicitly override the value to `true` or `false`.
	Override(bool),
}

pub trait BuilderFactory<Builder> {
	fn builder() -> Builder;
}

pub fn impl_derive_builder(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
	let syn::Data::Struct(ref s) = ast.data else {
		return Err(syn::Error::new_spanned(
			ast,
			"derive(Builder) can only be used on structs",
		));
    };

	let vis = ast.vis.clone();
	let name = &ast.ident;
	let builder_ident = syn::Ident::new(&format!("{}Builder", name), name.span());

	let (mut builder_field_names, mut builder_field_types, mut builder_fields) =
		(vec![], vec![], vec![]);
	let mut builder_const_generics = vec![];
	let mut builder_const_generics_all_false = vec![];
	let mut builder_const_generics_all_true = vec![];

	let mut setters = vec![];
	for (i, field) in s.fields.iter().enumerate() {
		let mut field_attrs = vec![];
		for attr in field.attrs.iter() {
			let attr: FieldAttr = attr.try_into()?;
			field_attrs.push(attr);
		}

		let field_name = field.ident.clone().unwrap();
		let field_type = &field.ty;
		builder_field_names.push(field_name.clone());
		builder_field_types.push(field_type.clone());
		builder_fields.push(quote! { #field_name: Option<#field_type> });
		let const_generic_name = syn::Ident::new(&format!("{}_set", field_name), field_name.span());
		builder_const_generics.push(quote! { const #const_generic_name: bool });
		builder_const_generics_all_false.push(quote! { false });
		builder_const_generics_all_true.push(quote! { true });

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
            impl<#(#const_generics),*> #builder_ident<#(#const_generic_vars),*> {
                #[allow(dead_code)]
                pub fn #setter_name(self, #field_name: #field_type) -> #builder_ident<#(#const_generic_return_vars),*> {
                    #builder_ident {
                        #field_name: Some(#field_name),
                        #(#all_expect_field_name: self.#all_expect_field_name),*
                    }
                }
            }
        });
	}
	let build_fn = quote! {
		impl #builder_ident<#(#builder_const_generics_all_true),*> {
			#[allow(dead_code)]
			pub fn build(self) -> #name {
				#name {
					#(#builder_field_names: self.#builder_field_names.unwrap()),*
				}
			}
		}
	};

	let builder_struct = quote! {
		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		#vis struct #builder_ident<#(#builder_const_generics),*> {
			#(#builder_field_names: Option<#builder_field_types>),*
		}
	};

	let builder = quote! {
		#builder_struct

		#(#setters)*
		#build_fn
	};

	let gen = quote! {
		impl ::typesafe_builders::prelude::BuilderFactory<#builder_ident<#(#builder_const_generics_all_false),*>> for #name {
			#[allow(dead_code)]
			fn builder() -> #builder_ident<#(#builder_const_generics_all_false),*> {
				#builder_ident {
					#(#builder_field_names: None),*
				}
			}
		}
	};

	Ok(quote! {
		#builder
		#gen
	}
	.into())
}

impl TryFrom<&syn::Attribute> for FieldAttr {
	type Error = syn::Error;

	fn try_from(attr: &syn::Attribute) -> Result<Self, Self::Error> {
		match &attr.meta {
			// Single paths are treated as `true`:
			syn::Meta::Path(ref path) => {
				let id = path.get_ident().ok_or_else(|| {
					syn::Error::new(
						path.span(),
						format!(
							"Expected identifier for field attribute: {}",
							path.to_token_stream().to_string()
						),
					)
				})?;

				Ok(FieldAttr { id: FieldAttrId::try_from(id)?, val: FieldAttrVal::Override(true) })
			},
			syn::Meta::NameValue(syn::MetaNameValue { path, value, .. }) => {
				let id = path.get_ident().ok_or_else(|| {
					syn::Error::new(
						path.span(),
						format!(
							"Expected identifier for field attribute: {}",
							path.to_token_stream().to_string()
						),
					)
				})?;

				let val = match value {
					syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(b), .. }) =>
						FieldAttrVal::Override(b.value),
					_ =>
						return Err(syn::Error::new(
							value.span(),
							format!(
								"Expected boolean value for field attribute: {}, but got: {}",
								path.to_token_stream().to_string(),
								value.to_token_stream().to_string()
							),
						)),
				};

				Ok(FieldAttr { id: FieldAttrId::try_from(id)?, val })
			},
			_ => Err(syn::Error::new(
				attr.span(),
				"Expected field attribute to be a path or name-value pair",
			)),
		}
	}
}

impl TryFrom<&syn::Ident> for FieldAttrId {
	type Error = syn::Error;

	fn try_from(ident: &syn::Ident) -> Result<Self, Self::Error> {
		match ident.to_string().as_str() {
			"optional" => Ok(FieldAttrId::Optional),
			e => Err(syn::Error::new(ident.span(), format!("Unknown field attribute: {:?}", e))),
		}
	}
}
