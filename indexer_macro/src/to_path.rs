use darling::FromDeriveInput;
use syn::Ident;

#[derive(FromDeriveInput)]
#[darling(attributes(to_path))]
pub struct PathOpts {
    pub ident: Ident,
    pub prefix: String,
}
