#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, __private::TokenStream2};

#[proc_macro_derive(ValidatorStruct)]
pub fn validator_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_data = match input.data {
        syn::Data::Struct(d) => d,
        _ => panic!("This macros only supports structs"),
    };

    let error_struct_name = format_ident!("{}Error", input.ident);
    let struct_name = input.ident;

    let mut fields: Vec<TokenStream2> = Vec::new();
    let mut error_fields: Vec<TokenStream2> = Vec::new();
    for field in struct_data.fields {
        if !field.attrs.is_empty() {
            if let Some(ident) = field.ident {
                fields.push(quote! { #ident: Option<Vec<validator::ValidationError>> });
                error_fields.push(
                    quote! { #ident: field_errors.remove(std::stringify!(#ident)).map(|v| v.clone()) },
                );
            }
        }
    }

    let error_struct = quote! {
        #[derive(Debug)]
        struct #error_struct_name {
            #(#fields),*
        }
    };

    let impl_block = quote! {
        impl #struct_name {
            fn validate_struct(&self) -> Result<(), #error_struct_name> {
                match self.validate() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let mut field_errors = e.field_errors();
                        Err(#error_struct_name {
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

    TokenStream::from(expanded)
}

#[proc_macro_derive(ValidatorMessageStruct)]
pub fn validator_messages_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_data = match input.data {
        syn::Data::Struct(d) => d,
        _ => panic!("This macros only supports structs"),
    };

    let error_struct_name = format_ident!("{}ErrorMessage", input.ident);
    let struct_name = input.ident;

    let mut fields: Vec<TokenStream2> = Vec::new();
    let mut error_fields: Vec<TokenStream2> = Vec::new();
    for field in struct_data.fields {
        if !field.attrs.is_empty() {
            if let Some(ident) = field.ident {
                fields.push(quote! { #ident: Option<Vec<String>> });
                error_fields.push(
                    quote! { #ident: field_errors.remove(std::stringify!(#ident)).map(|v| {
                        v.iter().map(|e| e.message.clone().unwrap().to_string()).collect()
                    }) },
                );
            }
        }
    }

    let error_struct = quote! {
        #[derive(Debug)]
        struct #error_struct_name {
            #(#fields),*
        }
    };

    let impl_block = quote! {
        impl #struct_name {
            fn validate_message_struct(&self) -> Result<(), #error_struct_name> {
                match self.validate() {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let mut field_errors = e.field_errors();
                        Err(#error_struct_name {
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

    TokenStream::from(expanded)
}
