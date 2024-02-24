use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, DeriveInput, Ident, Path, Token, Visibility,
};

pub mod kw {
    use syn::custom_keyword;

    // validator struct metadata
    custom_keyword!(derive);
    custom_keyword!(name);
    custom_keyword!(vis);
}

pub enum ValidatorStructMeta {
    Derive { kw: kw::derive, paths: Vec<Path> },
    Name { kw: kw::name, name: Ident },
    Vis { kw: kw::vis, vis: Visibility },
}

impl Parse for ValidatorStructMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::derive) {
            let kw = input.parse()?;
            let content;
            parenthesized!(content in input);
            let paths = content.parse_terminated(Path::parse, Token![,])?;
            Ok(ValidatorStructMeta::Derive {
                kw,
                paths: paths.into_iter().collect(),
            })
        } else if input.peek(kw::name) {
            let kw = input.parse()?;
            let content;
            parenthesized!(content in input);
            let name = content.parse()?;
            Ok(ValidatorStructMeta::Name { kw, name })
        } else {
            let kw = input.parse()?;
            let content;
            parenthesized!(content in input);
            let vis = content.parse()?;
            Ok(ValidatorStructMeta::Vis { kw, vis })
        }
    }
}

pub trait DeriveInputExt {
    /// Get all the `validator_struct` metadata associated with a struct.
    fn get_validator_struct_metadata(&self) -> syn::Result<Vec<ValidatorStructMeta>>;
}

impl DeriveInputExt for DeriveInput {
    fn get_validator_struct_metadata(&self) -> syn::Result<Vec<ValidatorStructMeta>> {
        get_metadata_inner("validator_struct", &self.attrs)
    }
}

fn get_metadata_inner<'a, T: Parse>(
    ident: &str,
    it: impl IntoIterator<Item = &'a Attribute>,
) -> syn::Result<Vec<T>> {
    it.into_iter()
        .filter(|attr| attr.path().is_ident(ident))
        .try_fold(Vec::new(), |mut vec, attr| {
            vec.extend(attr.parse_args_with(Punctuated::<T, Token![,]>::parse_terminated)?);
            Ok(vec)
        })
}
