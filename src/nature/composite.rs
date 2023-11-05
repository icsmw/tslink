use crate::{
    error::E,
    nature::{Nature, Primitive, RustTypeName, VariableTokenStream},
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
    fn variable_token_stream(
        &self,
        var_name: &str,
        err: Option<&Nature>,
    ) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        Ok(match self {
            Self::Option(_) | Self::Tuple(_) | Self::Vec(_) | Self::HashMap(_, _) => {
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

impl RustTypeName for Composite {
    fn rust_type_name(&self) -> Result<String, E> {
        Ok(match self {
            Self::Option(Some(nature)) => format!("Option<{}>", nature.rust_type_name()?),
            Self::Vec(Some(nature)) => format!("Vec<{}>", nature.rust_type_name()?),
            Self::HashMap(Some(key), Some(value)) => format!(
                "HashMap<{}, {}>",
                key.rust_type_name()?,
                value.rust_type_name()?
            ),
            Self::Tuple(types) => {
                let mut tys = vec![];
                for ty in types.iter() {
                    tys.push(ty.rust_type_name()?);
                }
                format!("({})", tys.join(", "))
            }
            Self::Undefined => "()".to_owned(),
            Self::Result(Some(res), Some(err)) => format!(
                "Result<{}, {}>",
                res.rust_type_name()?,
                err.rust_type_name()?
            ),
            Self::Func(args, out, asyncness, false) => {
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
            _ => Err(E::Parsing(format!(
                "Cannot get rust name of given composite type"
            )))?,
        })
    }
}
