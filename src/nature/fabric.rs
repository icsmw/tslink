use crate::{error::E, nature::Nature};
use proc_macro2::TokenStream;

pub trait TypeTokenStream {
    fn type_token_stream(&self) -> Result<TokenStream, E>;
}

impl TypeTokenStream for Nature {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        match self {
            Nature::Composite(ty) => ty.type_token_stream(),
            Nature::Primitive(ty) => ty.type_token_stream(),
            Nature::Refered(ty) => ty.type_token_stream(),
        }
    }
}
pub trait TypeAsString {
    fn type_as_string(&self) -> Result<String, E>;
}

impl TypeAsString for Nature {
    fn type_as_string(&self) -> Result<String, E> {
        match self {
            Nature::Composite(ty) => ty.type_as_string(),
            Nature::Primitive(ty) => ty.type_as_string(),
            Nature::Refered(ty) => ty.type_as_string(),
        }
    }
}

pub trait VariableTokenStream {
    fn variable_token_stream(&self, var_name: &str, err: Option<&Nature>)
        -> Result<TokenStream, E>;
}

impl VariableTokenStream for Nature {
    fn variable_token_stream(
        &self,
        var_name: &str,
        err: Option<&Nature>,
    ) -> Result<TokenStream, E> {
        match self {
            Nature::Composite(v) => v.variable_token_stream(var_name, err),
            Nature::Primitive(v) => v.variable_token_stream(var_name, err),
            Nature::Refered(v) => v.variable_token_stream(var_name, err),
        }
    }
}
