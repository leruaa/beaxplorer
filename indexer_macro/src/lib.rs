use darling::FromDeriveInput;
use persistable::{Model, PersistableOpts};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

mod persistable;

#[proc_macro_derive(Persistable, attributes(persistable, sortable))]
pub fn persistable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let mut opts = PersistableOpts::from_derive_input(&input).expect("Wrong options");
    let model_ident = opts.ident;
    let prefix = opts.prefix;

    let model_id = match &opts.id {
        Some(id) => quote! { #id },
        None => quote! { u64 },
    };

    opts.sortable_fields.extend(
        opts.data
            .take_struct()
            .unwrap()
            .into_iter()
            .filter_map(Into::into),
    );

    let field_names = opts
        .sortable_fields
        .iter()
        .map(|f| f.name.to_string())
        .collect::<Vec<_>>();

    let orderables = opts
        .sortable_fields
        .iter()
        .map(|f| match &f.with {
            Some(with) => quote! { #with(&m)},
            None => {
                let field_ident = &f.name;
                quote! { (m.id, m.model.#field_ident).into() }
            }
        })
        .collect::<Vec<_>>();

    let heap_fields = opts
        .sortable_fields
        .iter()
        .map(|f| format_ident!("{}_heap", f.name))
        .collect::<Vec<_>>();

    let heap_types = opts
        .sortable_fields
        .iter()
        .map(|f| f.ty.clone())
        .collect::<Vec<_>>();

    let model_with_id = match opts.model {
        Model::Collection => format_ident!("{}sWithId", model_ident),
        _ => format_ident!("{}WithId", model_ident),
    };

    let model_ty = match opts.model {
        Model::Default => quote! { #model_id, #model_ident },
        Model::Option => quote! { #model_id, Option<#model_ident> },
        Model::Collection => quote! { #model_id, Vec<#model_ident> },
    };

    let persist_iterator = match opts.model {
        Model::Default => {
            let persist_iterator = format_ident!("PersistIterator{}", model_ident);
            Some(quote! {
                pub trait #persist_iterator: Iterator<Item = #model_with_id> {

                    fn persist(mut self, base_dir: &str) -> Result<(), String>
                    where
                        Self: Sized,
                    {
                        self.try_for_each(|m| {
                            crate::persistable::ResolvablePersistable::save(&m, base_dir)
                        })
                    }

                    fn persist_sortables(self, base_dir: &str) -> Result<(), String>
                    where
                        Self: Sized,
                    {
                        let prefix = <Self::Item as crate::path::Prefix>::prefix();
                        let prefixed_dir = format!("{}/{}", base_dir, prefix);
                        #( let mut #heap_fields = crate::utils::FieldBinaryHeap::<#model_id, #heap_types>::new(); )*

                        for m in self {
                            #( #heap_fields.push(#orderables); )*
                        }

                        #( #heap_fields.persist(&prefixed_dir, #field_names)?; )*

                        Ok(())
                    }
                }

                impl<T: ?Sized> #persist_iterator for T where T: Iterator<Item = #model_with_id> { }
            })
        }
        _ => None,
    };

    let persistable_related = quote! {
        pub type #model_with_id = crate::model::ModelWithId<#model_ty>;

        #persist_iterator
    };

    let expanded = quote! {
        impl crate::persistable::MsgPackSerializable for #model_ident {}

        impl crate::path::Prefix for #model_ident {
            fn prefix() -> String {
                String::from(#prefix)
            }
        }

        impl crate::path::ToPath for #model_ident {
            type Id = #model_id;

            fn to_path(base: &str, id: &#model_id) -> String {
                format!("{}/{}/{}.msg", base, #prefix, id)
            }
        }

        impl crate::path::Dirs for #model_ident {
            fn dirs(base_dir: &str) -> Vec<std::path::PathBuf> {
                vec![
                    std::path::PathBuf::from(format!("{}{}", base_dir, <Self as crate::path::Prefix>::prefix())),
                    #( std::path::PathBuf::from(format!("{}{}", base_dir, <Self as crate::path::Prefix>::sortable_field_prefix(#field_names))), )*
                ]
            }
        }

        #persistable_related
    };

    TokenStream::from(expanded)
}
