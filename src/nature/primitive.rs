use crate::{
    error::E,
    nature::{Nature, OriginType, RustTypeName, TypeTokenStream, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Primitive {
    Number(OriginType, String),
    String(OriginType),
    Boolean(OriginType),
}

impl TypeTokenStream for Primitive {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        match self {
            Self::Number(ty, _) => ty,
            Self::String(ty) => ty,
            Self::Boolean(ty) => ty,
        }
        .type_token_stream()
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

impl RustTypeName for Primitive {
    fn rust_type_name(&self) -> Result<String, E> {
        Ok(match self {
            Self::Number(_, origin) => origin.to_owned(),
            Self::Boolean(_) => "bool".to_owned(),
            Self::String(_) => "String".to_owned(),
        })
    }
}
