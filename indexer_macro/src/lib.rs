use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Type,
};

extern crate proc_macro;

struct PersistableFieldInput {
    modelType: Type,
    field: String,
}

impl Parse for PersistableFieldInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let modelType = input.parse::<Type>()?;
        let field = input.parse::<String>()?;

        let input = PersistableFieldInput { modelType, field };

        Ok(input)
    }
}

#[proc_macro_derive(PersistableField)]
pub fn derive_persistable_field(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("input: {:?}", input);

    let expanded = quote! {
        // The generated impl.
        impl crate::persistable_fields::PersistableField for EpochAttestationsCount {
            type Model = EpochModelWithId;
            type Field = usize;
            const FIELD_NAME: &'static str = "attestations_count";

            fn get_value(model: &Self::Model) -> Orderable<Self::Field> {
                (model.0, model.1.attestations_count).into()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn persistable_field(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as Type);

    println!("_attr: {:?}", input);
    let expanded = quote! {
        // The generated impl.
        impl crate::persistable_fields::PersistableField for EpochAttestationsCount {
            type Model = EpochModelWithId;
            type Field = usize;
            const FIELD_NAME: &'static str = "attestations_count";

            fn get_value(model: &Self::Model) -> Orderable<Self::Field> {
                (model.0, model.1.attestations_count).into()
            }
        }
    };

    item.extend(TokenStream::from(expanded));
    item
}
