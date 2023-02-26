use darling::{FromDeriveInput, FromMeta};
use syn::Ident;

#[derive(FromDeriveInput)]
#[darling(attributes(persistable))]
pub struct PersistableOpts {
    pub ident: Ident,
    #[darling(default)]
    pub index: Option<Index>,
    pub prefix: String,
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub enum Index {
    #[default]
    Model,
    Option,
    Collection,
}
