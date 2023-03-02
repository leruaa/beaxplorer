use darling::FromDeriveInput;
use persistable::{Index, PersistableOpts};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;
use to_path::PathOpts;

mod persistable;
mod to_path;

#[proc_macro_derive(ToPath, attributes(to_path))]
pub fn to_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = PathOpts::from_derive_input(&input).expect("Wrong options");
    let st = opts.ident;
    let prefix = opts.prefix;

    let expanded = quote! {
        impl crate::path::ToPath<()> for #st {
            fn prefix() -> String {
                String::from(#prefix)
            }

            fn to_path(base: &str, id: ()) -> String {
                format!("{}/{}.msg", base, #prefix)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToPathWithId, attributes(to_path))]
pub fn to_path_with_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = PathOpts::from_derive_input(&input).expect("Wrong options");
    let st = opts.ident;
    let prefix = opts.prefix;

    let expanded = quote! {
        impl crate::path::ToPath<u64> for #st {
            fn prefix() -> String {
                String::from(#prefix)
            }

            fn to_path(base: &str, id: u64) -> String {
                format!("{}/{}/{}.msg", base, #prefix, id)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Persistable, attributes(persistable, sortable))]
pub fn persistable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let mut opts = PersistableOpts::from_derive_input(&input).expect("Wrong options");
    let model = opts.ident;

    let model_with_id = match opts.index {
        Index::Collection => format_ident!("{}sWithId", model),
        _ => format_ident!("{}WithId", model),
    };
    let model_ty = match opts.index {
        Index::Model => quote! { #model },
        Index::Option => quote! { Option<#model> },
        Index::Collection => quote! { Vec<#model> },
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

    let (heap_fields, heap_types) = opts
        .sortable_fields
        .iter()
        .map(|f| (format_ident!("{}_heap", f.name), f.ty.clone()))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let persist_iterator = match opts.index {
        Index::Model => {
            let persist_iterator = format_ident!("PersistIterator{}", model);
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
                        let prefix = <Self::Item as crate::path::ToPath<u64>>::prefix();
                        let prefixed_dir = format!("{}/{}", base_dir, prefix);
                        #( let mut #heap_fields = crate::utils::FieldBinaryHeap::<#heap_types>::new(); )*

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

    let expanded = quote! {
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
    };

    TokenStream::from(expanded)
}
