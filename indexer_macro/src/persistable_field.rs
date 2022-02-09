use syn::{
    parse::{Parse, ParseStream},
    Ident, Token, Type,
};

extern crate proc_macro;

pub struct AttributeMetadata {
    pub model_type: Type,
    pub field_name: Ident,
    pub field_type: Ident,
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

pub struct Input {
    pub field_struct: Ident,
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
