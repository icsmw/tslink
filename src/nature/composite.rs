use crate::{
    error::E,
    nature::{Nature, OriginType, Primitive, RustTypeName, TypeTokenStream, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Clone, Debug)]
pub enum Composite {
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
            Self::Result(ty, _, _, _, _) => ty,
            Self::Undefined(ty) => ty,
            Self::Func(ty, _, _, _, _) => ty,
        }
        .type_token_stream()
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
            Self::Option(_, _) | Self::Tuple(_, _) | Self::Vec(_, _) | Self::HashMap(_, _, _) => {
                if let Some(nature) = err {
                    let err_type_ref = format_ident!("{}", nature.rust_type_name()?);
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
            Self::Result(_, _, _, _, _) => {
                Err(E::Parsing(format!(
                    "<Result> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
            Self::Func(_, _, _, _, _) => {
                Err(E::Parsing(format!(
                    "<Func> cannot be converted to JSON string (field: {var_name})"
                )))
            }?,
        })
    }
}

impl RustTypeName for Composite {
    fn rust_type_name(&self) -> Result<String, E> {
        Ok(match self {
            Self::Option(_, Some(nature)) => format!("Option<{}>", nature.rust_type_name()?),
            Self::Vec(_, Some(nature)) => format!("Vec<{}>", nature.rust_type_name()?),
            Self::HashMap(_, Some(key), Some(value)) => format!(
                "HashMap<{}, {}>",
                key.rust_type_name()?,
                value.rust_type_name()?
            ),
            Self::Tuple(_, types) => {
                let mut tys = vec![];
                for ty in types.iter() {
                    tys.push(ty.rust_type_name()?);
                }
                format!("({})", tys.join(", "))
            }
            Self::Undefined(_) => "()".to_owned(),
            Self::Result(_, Some(res), Some(err), _, _) => format!(
                "Result<{}, {}>",
                res.rust_type_name()?,
                err.rust_type_name()?
            ),
            Self::Func(_, args, out, asyncness, false) => {
                let mut args_refs = vec![];
                for arg in args.iter() {
                    args_refs.push(arg.rust_type_name()?);
                }
                if *asyncness {
                    //TODO: add test for asyncness
                    format!(
                        "Fn({}) -> Pin<Box<dyn Future<Output = {}>>>",
                        args_refs.join(", "),
                        if let Some(out) = out {
                            out.rust_type_name()?
                        } else {
                            "()".to_owned()
                        }
                    )
                } else {
                    format!(
                        "Fn({}) -> {}",
                        args_refs.join(", "),
                        if let Some(out) = out {
                            out.rust_type_name()?
                        } else {
                            "()".to_owned()
                        }
                    )
                }
            }
            _ => Err(E::Parsing(
                "Cannot get rust name of given composite type".to_string(),
            ))?,
        })
    }
}
