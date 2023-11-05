use crate::{
    error::E,
    nature::{Nature, RustTypeName, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Primitive {
    Number(String),
    BigInt(String),
    String,
    Boolean,
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
            Self::Number(origin) => origin.to_owned(),
            Self::BigInt(origin) => origin.to_owned(),
            Self::Boolean => "bool".to_owned(),
            Self::String => "String".to_owned(),
        })
    }
}
