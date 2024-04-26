use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Data, DeriveInput};

use crate::utils::type_props::HasTypeProperties;

pub fn validator_struct_inner(ast: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast.ident;

    let type_properties = ast.get_type_properties()?;

    let struct_data = match ast.data {
        Data::Struct(d) => d,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports structs.",
            ))
        }
    };

    let name = type_properties
        .name
        .unwrap_or_else(|| syn::Ident::new(&format!("{}Error", struct_name), Span::call_site()));
    let vis = type_properties.vis.unwrap_or_else(|| ast.vis.clone());

    let derives = type_properties.derives;
    let derives = quote! {
        #[derive(Debug, #(#derives),*)]
    };

    let mut fields: Vec<TokenStream2> = Vec::new();
    let mut error_fields: Vec<TokenStream2> = Vec::new();
    for field in struct_data.fields {
        if !field.attrs.is_empty() {
            if let Some(ident) = field.ident {
                fields.push(quote! { #ident: Option<Vec<String>> });
                error_fields.push(
                    quote! { #ident: field_errors.remove(std::stringify!(#ident)).map(|v| {
                        v.iter().map(|e| e.message.clone().map(|m| m.to_string()).unwrap_or(e.code.to_string())).collect()
                    }) },
                );
            }
        }
    }

    let error_struct = quote! {
        #derives
        #vis struct #name {
            #(#fields),*
        }
    };

    let impl_block = quote! {
        impl #struct_name {
            pub fn validate_struct(&self) -> Result<(), #name> {
                match self.validate() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let mut field_errors = e.field_errors();
                        Err(#name {
                            #(#error_fields),*
                        })
                    }
                }
            }
        }
    };

    let expanded = quote! {
        #error_struct
        #impl_block
    };

    Ok(TokenStream::from(expanded))
}
