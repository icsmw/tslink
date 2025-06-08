use crate::{
    error::E,
    nature::{TypeAsString, TypeTokenStream},
};

use proc_macro2::TokenStream;
use quote::quote;
use std::{convert::From, str::FromStr};
use syn::{Ident, ImplItemFn, ItemFn, PathSegment, ReturnType, Type, TypeTuple};

/// Stores the original textual representation of a primitive Rust type (e.g., `u8`, `i32`, `bool`, `String`).
///
/// Represents the original textual form of a Rust type, primarily used for primitive or path-based types.
///
/// `OriginType` is a lightweight wrapper around a `String` that holds the source-level representation
/// of a Rust type, as extracted from `syn` AST nodes. It is mainly used in scenarios involving code generation,
/// such as converting Rust types into TypeScript declarations or regenerating Rust code segments.
///
/// # Purpose
///
/// This type enables uniform treatment of various Rust types when only their string form is needed —
/// such as for generating `.ts`, `.d.ts`, or transformed Rust code with procedural macros.
///
/// # Supported Conversions
///
/// The type supports conversion from multiple syntax tree node types:
///
/// - [`Ident`] — for primitive types like `i32`, `bool`, `String`.
/// - [`TypeTuple`] — e.g., `(i32, bool)`.
/// - [`PathSegment`] — for segments like `std` or `Option`.
/// - [`ReturnType`] — to capture the return part of function signatures.
/// - [`Type`] — any supported Rust type syntax.
/// - [`ImplItemFn`] / [`ItemFn`] — for extracting the full type signature of functions.
///
/// All conversions rely on `quote!` to stringify the tokens into source-formatted Rust.
///
/// # Implemented Traits
///
/// - [`TypeTokenStream`] — emits the stored string as a `TokenStream`.
/// - [`TypeAsString`] — returns the stored string as a `String`.
///
/// # Limitations
///
/// This struct is not suitable for deeply structured or semantically rich type analysis.
/// It serves primarily as a *stringification utility* in macro contexts.
///
/// [`Ident`]: syn::Ident
/// [`TypeTuple`]: syn::TypeTuple
/// [`PathSegment`]: syn::PathSegment
/// [`ReturnType`]: syn::ReturnType
/// [`Type`]: syn::Type
/// [`ImplItemFn`]: syn::ImplItemFn
/// [`ItemFn`]: syn::ItemFn
/// [`TypeTokenStream`]: crate::TypeTokenStream
/// [`TypeAsString`]: crate::TypeAsString
#[derive(Debug, Clone)]
pub struct OriginType(String);

impl From<Ident> for OriginType {
    fn from(ty: Ident) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl From<TypeTuple> for OriginType {
    fn from(ty: TypeTuple) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl From<PathSegment> for OriginType {
    fn from(ty: PathSegment) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl From<ReturnType> for OriginType {
    fn from(ty: ReturnType) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl From<Type> for OriginType {
    fn from(ty: Type) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl From<ImplItemFn> for OriginType {
    fn from(ty: ImplItemFn) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl From<ItemFn> for OriginType {
    fn from(ty: ItemFn) -> Self {
        let token = quote! { #ty };
        OriginType(format!("{token}"))
    }
}

impl TypeTokenStream for OriginType {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        Ok(TokenStream::from_str(&self.0)?)
    }
}

impl TypeAsString for OriginType {
    fn type_as_string(&self) -> Result<String, E> {
        Ok(self.0.clone())
    }
}
