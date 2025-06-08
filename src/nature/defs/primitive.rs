use crate::{
    error::E,
    nature::{Nature, OriginType, TypeAsString, TypeTokenStream, VariableTokenStream},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Represents a primitive type as it will be exposed in the generated TypeScript code.
///
/// This enum defines the basic mapping of primitive types from Rust to TypeScript,
/// with each variant carrying the original Rust type (`OriginType`) for reference and potential code generation.
///
/// # Variants
///
/// - `Number(OriginType)` — Corresponds to TypeScript's `number` type (e.g., `f64`, `i32`).
/// - `BigInt(OriginType)` — Corresponds to TypeScript's `bigint` type (e.g., `i64`, `u64`).
/// - `String(OriginType)` — Corresponds to TypeScript's `string` type (e.g., `String`, `&str`).
/// - `Boolean(OriginType)` — Corresponds to TypeScript's `boolean` type (e.g., `bool`).
///
/// The associated `OriginType` allows the generator to keep track of the exact source type
/// and apply more specific rules or type checks if needed during code generation.
#[derive(Clone, Debug)]
pub enum Primitive {
    Number(OriginType),
    BigInt(OriginType),
    String(OriginType),
    Boolean(OriginType),
}

impl TypeTokenStream for Primitive {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        match self {
            Self::Number(ty) => ty,
            Self::BigInt(ty) => ty,
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
            Self::BigInt(ty) => ty,
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
