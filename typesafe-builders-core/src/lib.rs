#![allow(non_upper_case_globals)]

use quote::{quote, ToTokens};
use syn::spanned::Spanned;

pub type FieldAttrs = std::collections::HashMap<FieldAttrId, FieldAttrVal>;

#[derive(Debug)]
pub struct FieldAttr {
	pub id: FieldAttrId,
	pub val: FieldAttrVal,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FieldAttrId {
	Optional,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FieldAttrVal {
	/// Inherent the default value. This is the default.
	Inherit,

	/// Explicitly override the value to `true` or `false`.
	Override(bool),
}

pub trait BuilderFactory<Builder> {
	fn builder() -> Builder;
}

pub fn impl_derive_builder(ast: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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
	let mut builder_const_generics_all_unset = vec![];
	let mut builder_const_generics_all_set = vec![];

	// The dont-cares for optional fields:
	let mut builder_optional_generics = vec![];
	let mut builder_optional_generic_values = vec![];

	let mut setters = vec![];
	for (i, field) in s.fields.iter().enumerate() {
		let field_attrs = extract_attributes(&field)?;
		let optional = is_optional(field, &field_attrs);

		if optional {
			let field_name = syn::Ident::new(
				&format!("{}_set", field.ident.clone().unwrap()).to_uppercase(),
				field.ident.span(),
			);
			builder_optional_generics.push(quote! { const #field_name: bool });
			builder_optional_generic_values.push(quote! { #field_name });
		} else {
			builder_optional_generic_values.push(quote! { true });
		}

		let field_name = field.ident.clone().unwrap();
		let field_type = &field.ty;
		builder_field_names.push(field_name.clone());
		builder_field_types.push(field_type.clone());
		builder_fields.push(quote! { #field_name: Option<#field_type> });
		let const_generic_name =
			syn::Ident::new(&format!("{}_set", field_name).to_uppercase(), field_name.span());
		builder_const_generics.push(quote! { const #const_generic_name: bool });
		builder_const_generics_all_unset.push(quote! { false });
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
		// The `build` that requires all to be set. This one is always implemented.
		/*impl #builder_ident<#(#builder_const_generics_all_set),*> {
			#[allow(dead_code)]
			pub fn build(self) -> #name {
				#name {
					#(#builder_field_names: self.#builder_field_names.unwrap()),*
				}
			}
		}*/

		// The `build` that is available as soon as all mandatory fields are set.
		impl<#(#builder_optional_generics),*> #builder_ident<#(#builder_optional_generic_values),*> {
			#[allow(dead_code)]
			pub fn build(self) -> #name {
				#name {
					#(#builder_field_names: self.#builder_field_names.unwrap_or_default()),*
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
		impl ::typesafe_builders::prelude::BuilderFactory<#builder_ident<#(#builder_const_generics_all_unset),*>> for #name {
			#[allow(dead_code)]
			fn builder() -> #builder_ident<#(#builder_const_generics_all_unset),*> {
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

fn extract_attributes(field: &syn::Field) -> Result<FieldAttrs, syn::Error> {
	let mut field_attrs = FieldAttrs::new();

	for attr in field.attrs.iter() {
		let fattr: FieldAttr = attr.try_into()?;
		if field_attrs.contains_key(&fattr.id) {
			return Err(syn::Error::new_spanned(attr, "Duplicate attribute on struct field"))
		}
		field_attrs.insert(fattr.id, fattr.val);
	}

	Ok(field_attrs)
}

fn is_optional(_field: &syn::Field, attrs: &FieldAttrs) -> bool {
	match attrs.get(&FieldAttrId::Optional).unwrap_or(&FieldAttrVal::Inherit) {
		FieldAttrVal::Override(o) => *o,
		FieldAttrVal::Inherit => false,
	}
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
