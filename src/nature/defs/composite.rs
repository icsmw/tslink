use crate::{
    error::E,
    nature::{Nature, OriginType, Primitive, TypeAsString, TypeTokenStream, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Composite {
    Array(Box<Nature>),
    Vec(OriginType, Option<Box<Nature>>),
    HashMap(OriginType, Option<Primitive>, Option<Box<Nature>>),
    Tuple(OriginType, Vec<Nature>),
    Option(OriginType, Option<Box<Nature>>),
    // Ok, Err, exception_suppression, asyncness
    Result(
        OriginType,
        Option<Box<Nature>>,
        Option<Box<Nature>>,
        bool,
        bool,
    ),
    Undefined(OriginType),
    // (OriginType, Vec(Args), Output, asyncness, constructor)
    Func(OriginType, Vec<Nature>, Option<Box<Nature>>, bool, bool),
}

impl TypeTokenStream for Composite {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        match self {
            Self::Vec(ty, _) => ty,
            Self::HashMap(ty, _, _) => ty,
            Self::Tuple(ty, _) => ty,
            Self::Option(ty, _) => ty,
            Self::Result(ty, ..) => ty,
            Self::Undefined(ty) => ty,
            Self::Func(ty, ..) => ty,
            Self::Array(..) => {
                return Err(E::NotSupported(
                    "Array isn't supported in this context".to_owned(),
                ))
            }
        }
        .type_token_stream()
    }
}

impl TypeAsString for Composite {
    fn type_as_string(&self) -> Result<String, E> {
        match self {
            Self::Vec(ty, _) => ty,
            Self::HashMap(ty, _, _) => ty,
            Self::Tuple(ty, _) => ty,
            Self::Option(ty, _) => ty,
            Self::Result(ty, ..) => ty,
            Self::Undefined(ty) => ty,
            Self::Func(ty, ..) => ty,
            Self::Array(ty) => {
                return Ok(format!("{}[]", ty.type_as_string()?));
            }
        }
        .type_as_string()
    }
}

impl VariableTokenStream for Composite {
    fn variable_token_stream(
        &self,
        var_name: &str,
        err: Option<&Nature>,
    ) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        Ok(match self {
            Self::Option(..) | Self::Tuple(..) | Self::Vec(..) | Self::HashMap(..) => {
                if let Some(nature) = err {
                    let err_type_ref = nature.type_token_stream()?;
                    quote! {
                        serde_json::to_string(&#var_name).map_err(|e| Into::<#err_type_ref>::into(e))?
                    }
                } else {
                    quote! {
                        serde_json::to_string(&#var_name).expect("Converting to JSON string")
                    }
                }
            }
            Self::Undefined(_) => {
                quote! {
                    #var_name
                }
            }
            Self::Result(..) => {
                Err(E::Parsing(format!(
                    "<Result> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
            Self::Func(..) => {
                Err(E::Parsing(format!(
                    "<Func> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
            Self::Array(..) => {
                Err(E::Parsing(format!(
                    "<Array> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
        })
    }
}
