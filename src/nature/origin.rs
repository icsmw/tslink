use crate::{
    error::E,
    nature::{TypeAsString, TypeTokenStream},
};

use proc_macro2::TokenStream;
use quote::quote;
use std::{convert::From, str::FromStr};
use syn::{Ident, ImplItemFn, ItemFn, PathSegment, ReturnType, Type, TypeTuple};

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
