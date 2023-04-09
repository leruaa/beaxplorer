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
        None => match &opts.model {
            Some(_) => quote! { u64 },
            None => quote! { () },
        },
    };

    let to_path = if opts.id.is_some() || opts.model.is_some() {
        quote! { format!("{}/{}/{}.msg", base, #prefix, id) }
    } else {
        quote! { format!("{}/{}.msg", base, #prefix) }
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

    let all_from_fields = opts
        .sortable_fields
        .iter()
        .map(|f| format_ident!("all_from_{}", f.name))
        .collect::<Vec<_>>();

    let heap_types = opts
        .sortable_fields
        .iter()
        .map(|f| f.ty.clone())
        .collect::<Vec<_>>();

    let model_with_id_related = match opts.model {
        Some(model) => {
            let model_with_id = match model {
                Model::Collection => format_ident!("{}sWithId", model_ident),
                _ => format_ident!("{}WithId", model_ident),
            };

            let model_ty = match model {
                Model::Default => quote! { #model_id, #model_ident },
                Model::Option => quote! { #model_id, Option<#model_ident> },
                Model::Collection => quote! { #model_id, Vec<#model_ident> },
            };

            let persist_iterator = match model {
                Model::Default => {
                    let persist_iterator = format_ident!("PersistIterator{}", model_ident);
                    Some(quote! {
                        pub trait #persist_iterator: Iterator<Item = #model_with_id> {

                            fn persist(self, base_dir: &str)
                            where
                                Self: Sized,
                            {
                                for m in self {
                                    crate::persistable::Persistable::persist(&m, base_dir);
                                }
                            }

                            fn persist_sortables(self, base_dir: &str) -> Result<(), String>
                            where
                                Self: Sized,
                            {
                                let prefix = <Self::Item as crate::path::ToPath<#model_id>>::prefix();
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

            Some(quote! {
                pub type #model_with_id = ModelWithId<#model_ty>;

                impl crate::persistable::Persistable for Vec<#model_with_id>
                {
                    fn persist(&self, base_dir: &str) {
                        for m in self {
                            m.persist(base_dir);
                        }
                    }
                }

                #persist_iterator
            })
        }
        None => None,
    };

    let expanded = quote! {
        impl crate::path::ToPath<#model_id> for #model_ident {
            fn prefix() -> String {
                String::from(#prefix)
            }

            fn to_path(base: &str, id: &#model_id) -> String {
                #to_path
            }

            fn dirs(base_dir: &str) -> Vec<std::path::PathBuf> {
                vec![
                    std::path::PathBuf::from(format!("{}{}", base_dir, Self::prefix())),
                    #( std::path::PathBuf::from(format!("{}{}", base_dir, Self::sortable_field_prefix(#field_names))), )*
                ]
            }
        }

        #model_with_id_related
    };

    TokenStream::from(expanded)
}
