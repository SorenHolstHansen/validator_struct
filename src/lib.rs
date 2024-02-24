extern crate proc_macro;

use syn::DeriveInput;

mod macros;
mod utils;

#[proc_macro_derive(ValidatorStruct, attributes(validator_struct))]
pub fn validator_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::validator_struct::validator_struct_inner(ast)
        .unwrap_or_else(|err| err.to_compile_error());
    toks.into()
}
