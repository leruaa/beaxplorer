use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Token, Type,
};

extern crate proc_macro;

struct AttributeMetadata {
    model_type: Type,
    field_name: Ident,
    field_type: Ident,
}

impl Parse for AttributeMetadata {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let model_type = input.parse::<Type>()?;
        input.parse::<Token![,]>()?;
        let field_name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let field_type = input.parse::<Ident>()?;

        Ok(AttributeMetadata {
            model_type,
            field_name,
            field_type,
        })
    }
}

struct Input {
    field_struct: Ident,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<Token![pub]>()?;
        input.parse::<Token![struct]>()?;
        let field_struct = input.parse::<Ident>()?;
        input.parse::<Token![;]>()?;

        Ok(Input { field_struct })
    }
}

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
