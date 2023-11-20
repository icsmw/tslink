#![doc = include_str!("../README.md")]

mod config;
mod context;
mod error;
mod interpreter;
mod modificator;
mod nature;
mod package;
mod reader;

#[macro_use]
extern crate lazy_static;

use config::Config;
use context::Context;
use nature::Natures;
use proc_macro::TokenStream;
use quote::ToTokens;
use std::{convert::TryInto, sync::RwLock};
use syn::{parse_macro_input, Item};

lazy_static! {
    #[doc(hidden)]
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
    #[doc(hidden)]
    static ref NATURES: RwLock<Natures> = RwLock::new(Natures::new());
}

#[proc_macro_attribute]
pub fn tslink(args: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(err) = config::setup() {
        return syn::Error::new_spanned(item.to_string(), err.to_string())
            .into_compile_error()
            .into();
    }
    let item_ref = item.clone();
    let context: Context = parse_macro_input!(args as Context);
    if context.ignore_self() {
        item
    } else {
        let mut natures = NATURES.write().expect("Get access to list of entities");
        let mut item = parse_macro_input!(item as Item);
        if let Err(err) = reader::read(&mut item, &mut natures, context) {
            let str_err = err.to_string();
            return TryInto::<syn::Error>::try_into(err)
                .unwrap_or(syn::Error::new_spanned(item_ref.to_string(), str_err))
                .into_compile_error()
                .into();
        }
        proc_macro::TokenStream::from(item.to_token_stream())
    }
}
