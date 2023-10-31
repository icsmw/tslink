use crate::{error::E, nature::VariableTokenStream};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Primitive {
    Number,
    BigInt,
    String,
    Boolean,
}

impl VariableTokenStream for Primitive {
    fn token_stream(&self, var_name: &str) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        Ok(quote! {
            #var_name
        })
    }
}
