use crate::{
    error::E,
    nature::{Nature, Primitive, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Composite {
    Vec(Option<Box<Nature>>),
    HashMap(Option<Primitive>, Option<Box<Nature>>),
    Tuple(Vec<Nature>),
    Option(Option<Box<Nature>>),
    Result(Option<Box<Nature>>, Option<Box<Nature>>),
    Undefined,
    // (Vec(Args), Output, async, constructor)
    Func(Vec<Nature>, Option<Box<Nature>>, bool, bool),
}

impl VariableTokenStream for Composite {
    fn token_stream(&self, var_name: &str) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        Ok(match self {
            Self::Option(_) | Self::Tuple(_) | Self::Vec(_) | Self::HashMap(_, _) => {
                quote! {
                    serde_json::to_string(&#var_name).map_err(|e| e.to_string())?
                }
            }
            Self::Undefined => {
                quote! {
                    #var_name
                }
            }
            Self::Result(_, _) => {
                Err(E::Parsing(format!(
                    "<Result> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
            Self::Func(_, _, _, _) => {
                Err(E::Parsing(format!(
                    "<Func> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
        })
    }
}
