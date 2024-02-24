use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{Data, DeriveInput};

pub fn validator_struct_inner(ast: DeriveInput) -> syn::Result<TokenStream> {
    let name = ast.ident;
    let struct_data = match ast.data {
        Data::Struct(d) => d,
        _ => panic!("This macro only supports structs"),
    };

    let error_struct_name = format_ident!("{}ErrorMessage", ast.ident);
    let struct_name = ast.ident;

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

    Ok(TokenStream::from(expanded))
}
