use persistable_field::{AttributeMetadata, Input};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod persistable_field;

#[proc_macro_attribute]
pub fn persistable_field(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attribute_meta = parse_macro_input!(attr as AttributeMetadata);
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
            const FIELD_NAME: &'static str = "#field_name";

            fn get_value(value: &#model_type) -> Orderable<Self::Field> {
                (value.id, value.model.#field_name).into()
            }
        }
    };

    TokenStream::from(expanded)
}
