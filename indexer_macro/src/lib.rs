use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use persistable::{Index, PersistableOpts, SortableField};
use persistable_field::{Input, PersistableFieldAttributeMetadata};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;
mod persistable;
mod persistable_field;

#[proc_macro_attribute]
pub fn persistable_field(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attribute_meta = parse_macro_input!(attr as PersistableFieldAttributeMetadata);
    let input = parse_macro_input!(item as Input);

    let model_type = attribute_meta.model_type;
    let field_name = attribute_meta.field_name;
    let field_type = attribute_meta.field_type;
    let field_struct = input.field_struct;

    let expanded = quote! {
        pub struct #field_struct;
        // The generated impl.
        impl PersistableField<#model_type> for #field_struct {
            type Field = #field_type;
            const FIELD_NAME: &'static str = stringify!(#field_name);

            fn get_value(value: &#model_type) -> Orderable<Self::Field> {
                (value.id, value.model.#field_name.clone()).into()
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
    let prefix = opts.prefix;

    let expanded = match opts.index {
        Some(index) => {
            let model_with_id = match index {
                Index::Collection => format_ident!("{}sWithId", model),
                _ => format_ident!("{}WithId", model),
            };
            let model_ty = match index {
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

            let persist_iterator = match index {
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

                            fn persist_sortables(self, base_dir: &str)
                            where
                                Self: Sized,
                            {
                                #( let mut #heap_fields = crate::utils::FieldBinaryHeap::<#heap_types>::new(); )*

                                for m in self {
                                    #( #heap_fields.push(#orderables); )*
                                }

                                #( #heap_fields.persist(base_dir, #field_names); )*
                            }
                        }

                        impl<T: ?Sized> #persist_iterator for T where T: Iterator<Item = #model_with_id> { }
                    })
                }
                _ => None,
            };

            quote! {
                pub type #model_with_id = ModelWithId<#model_ty>;

                impl crate::path::ToPath<u64> for #model_with_id {
                    fn to_path(base: &str, id: u64) -> String {
                        format!("{}/{}/{}.msg", base, #prefix, id)
                    }
                }

                impl crate::persistable::Persistable for Option<#model_with_id>
                {
                    fn persist(&self, base_dir: &str) {
                        if let Some(p) = self {
                            p.persist(base_dir)
                        }
                    }
                }

                impl crate::persistable::Persistable for Vec<#model_with_id>
                {
                    fn persist(&self, base_dir: &str) {
                        for m in self {
                            m.persist(base_dir);
                        }
                    }
                }

                #persist_iterator
            }
        }
        None => {
            quote! {
                impl crate::path::ToPath<()> for #model {
                    fn to_path(base_dir: &str, id: ()) -> String {
                        format!("{}/{}.msg", base_dir, #prefix)
                    }
                }

                impl crate::persistable::Persistable for #model {
                    fn persist(&self, base_dir: &str) {
                        let path = format!("{}/{}.msg", base_dir, #prefix);
                        let mut f = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
                        self.serialize(&mut rmp_serde::encode::Serializer::new(&mut f)).unwrap();
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
