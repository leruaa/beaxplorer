use darling::FromDeriveInput;
use persistable::{Index, PersistableOpts};
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

#[proc_macro_derive(Persistable, attributes(persistable))]
pub fn persistable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = PersistableOpts::from_derive_input(&input).expect("Wrong options");
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
            quote! {
                pub type #model_with_id = ModelWithId<#model_ty>;

                impl crate::path::ToPath<u64> for #model_with_id {
                    fn to_path(base: &str, id: u64) -> String {
                        format!("{}/{}/{}.msg", base, #prefix, id)
                    }
                }
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
                    fn persist(self, base_dir: &str) {
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
