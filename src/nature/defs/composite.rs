use crate::{
    error::E,
    nature::{Nature, OriginType, Primitive, TypeAsString, TypeTokenStream, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Represents structured, non-primitive Rust types that require composition or generic typing.
///
/// This enum is used to capture high-level Rust types such as `Vec<T>`, `HashMap<K, V>`, `Option<T>`, function signatures,
/// and other composite forms. It allows the type system to retain detailed shape and origin information,
/// which is crucial when generating TypeScript bindings or transforming Rust code dynamically.
///
/// Each variant holds enough information to reconstruct both the Rust and TypeScript representation of the type,
/// including its subtypes and textual origin (`OriginType`).
#[derive(Clone, Debug)]
pub enum Composite {
    /// Represents array types like `[T; N]`.
    ///
    /// The inner boxed [`Nature`] describes the element type `T`.
    Array(Box<Nature>),

    /// Represents vector types like `Vec<T>`.
    ///
    /// - `OriginType` contains the literal `"Vec"` or similar.
    /// - `Option<Box<Nature>>` is the inner element type `T`, if known.
    Vec(OriginType, Option<Box<Nature>>),

    /// Represents map types such as `HashMap<K, V>`.
    ///
    /// - `OriginType` contains the string `"HashMap"` or equivalent.
    /// - `Option<Primitive>` is the key type `K` (should be a primitive for JS compatibility).
    /// - `Option<Box<Nature>>` is the value type `V`.
    HashMap(OriginType, Option<Primitive>, Option<Box<Nature>>),

    /// Represents tuple types like `(A, B, C)`.
    ///
    /// - `OriginType` holds the complete stringified form of the tuple type.
    /// - `Vec<Nature>` lists each element type in the tuple.
    Tuple(OriginType, Vec<Nature>),

    /// Represents optional types such as `Option<T>`.
    ///
    /// - `OriginType` is typically the literal `"Option"`.
    /// - `Option<Box<Nature>>` is the inner type `T`, if known.
    Option(OriginType, Option<Box<Nature>>),

    /// Represents result types like `Result<T, E>`.
    ///
    /// - `OriginType` typically holds `"Result"`.
    /// - `Option<Box<Nature>>` is the `Ok` type `T`.
    /// - `Option<Box<Nature>>` is the `Err` type `E`.
    /// - `bool` indicates whether exception suppression is applied.
    /// - `bool` indicates whether the result originates from an async context.
    Result(
        OriginType,
        Option<Box<Nature>>,
        Option<Box<Nature>>,
        bool,
        bool,
    ),

    /// Represents the unit type `()`.
    ///
    /// - `OriginType` holds the stringified representation of the unit type.
    Undefined(OriginType),

    /// Represents function or method signatures.
    ///
    /// - `OriginType` stores the full stringified signature.
    /// - `Vec<Nature>` is the list of argument types.
    /// - `Option<Box<Nature>>` is the return type.
    /// - `bool` indicates whether the function is asynchronous.
    /// - `bool` indicates whether the function is a constructor.
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
