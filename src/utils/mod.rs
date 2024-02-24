use quote::ToTokens;

mod metadata;
pub mod type_props;

pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("Found multiple occurrences of strum({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first one here"));
    e
}
