use syn::{DeriveInput, Ident, Path, Visibility};

use crate::utils::{
    metadata::{DeriveInputExt, ValidatorStructMeta},
    occurrence_error,
};

#[derive(Debug, Clone, Default)]
pub struct ValidatorStructTypeProperties {
    pub derives: Vec<Path>,
    pub name: Option<Ident>,
    pub vis: Option<Visibility>,
}

pub trait HasTypeProperties {
    fn get_type_properties(&self) -> syn::Result<ValidatorStructTypeProperties>;
}

impl HasTypeProperties for DeriveInput {
    fn get_type_properties(&self) -> syn::Result<ValidatorStructTypeProperties> {
        let mut output = ValidatorStructTypeProperties::default();

        let discriminants_meta = self.get_validator_struct_metadata()?;

        let mut name_kw = None;
        let mut vis_kw = None;
        for meta in discriminants_meta {
            match meta {
                ValidatorStructMeta::Derive { paths, .. } => {
                    output.derives.extend(paths);
                }
                ValidatorStructMeta::Name { name, kw } => {
                    if let Some(fst_kw) = name_kw {
                        return Err(occurrence_error(fst_kw, kw, "name"));
                    }

                    name_kw = Some(kw);
                    output.name = Some(name);
                }
                ValidatorStructMeta::Vis { vis, kw } => {
                    if let Some(fst_kw) = vis_kw {
                        return Err(occurrence_error(fst_kw, kw, "vis"));
                    }

                    vis_kw = Some(kw);
                    output.vis = Some(vis);
                }
            }
        }

        Ok(output)
    }
}
