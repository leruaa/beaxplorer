use darling::{ast::Data, FromDeriveInput, FromField, FromMeta};
use syn::{Ident, Path, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(persistable))]
pub struct PersistableOpts {
    pub ident: Ident,
    pub index: Index,
    #[darling(multiple)]
    #[darling(rename = "sortable_field")]
    pub sortable_fields: Vec<SortableField>,
    pub data: Data<(), Field>,
}

#[derive(FromField)]
pub struct Field {
    pub ident: Option<Ident>,
    pub ty: Type,
    #[darling(default)]
    pub sortable: bool,
}

#[derive(FromMeta)]
pub struct SortableField {
    pub name: Ident,
    pub ty: Type,
    pub with: Option<Path>,
}

impl From<Field> for Option<SortableField> {
    fn from(value: Field) -> Self {
        match value.ident {
            Some(name) => {
                if value.sortable {
                    Some(SortableField {
                        name,
                        ty: value.ty,
                        with: None,
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[derive(Default, FromMeta)]
pub enum Index {
    #[default]
    Model,
    Option,
    Collection,
}
