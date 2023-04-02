extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;  

#[proc_macro_derive(Builder)]
pub fn derive_builder(stream: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(stream as syn::DeriveInput);
    impl_derive_builder(&ast).unwrap()
}

fn impl_derive_builder(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
    let syn::Data::Struct(ref s) = ast.data else {
        return Err("Only structs can be used with the Builder derive macro".to_string());
    };

    let name = &ast.ident;
    let builder_ident = syn::Ident::new(&format!("{}Builder", name), name.span());

    let mut setters = vec![];
    for fields in &s.fields {
        let field_name = &fields.ident;
        let field_type = &fields.ty;
        let setter_name = syn::Ident::new(&format!("with_{}", field_name.as_ref().unwrap()), field_name.span());
        let setter = quote! {
            pub fn #setter_name(self, #field_name: #field_type) -> Self {
                self
            }
        };
        setters.push(setter);
    }

    let builder = quote! {
        pub struct #builder_ident {}

        impl #builder_ident {
            #(#setters)*
        }
    };

    let gen = quote! {
        impl ::typesafe_builders::prelude::BuilderFactory<#builder_ident> for #name {
            fn builder() -> #builder_ident {
                #builder_ident {}
            }
        }
    };
    
    Ok(quote! {
        #builder
        #gen
    }.into())
}
