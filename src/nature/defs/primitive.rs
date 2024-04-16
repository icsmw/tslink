use crate::{
    error::E,
    nature::{Nature, OriginType, TypeAsString, TypeTokenStream, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Primitive {
    Number(OriginType),
    String(OriginType),
    Boolean(OriginType),
}

impl TypeTokenStream for Primitive {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        match self {
            Self::Number(ty) => ty,
            Self::String(ty) => ty,
            Self::Boolean(ty) => ty,
        }
        .type_token_stream()
    }
}

impl TypeAsString for Primitive {
    fn type_as_string(&self) -> Result<String, E> {
        match self {
            Self::Number(ty) => ty,
            Self::String(ty) => ty,
            Self::Boolean(ty) => ty,
        }
        .type_as_string()
    }
}

impl VariableTokenStream for Primitive {
    fn variable_token_stream(
        &self,
        var_name: &str,
        _err: Option<&Nature>,
    ) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        Ok(quote! {
            #var_name
        })
    }
}
