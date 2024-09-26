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
use interpreter::ts::Indexer;
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
    #[doc(hidden)]
    static ref TS_IMPORTS: RwLock<Indexer> = RwLock::new(Indexer::default());
}

/// Binds given entity with TypeScript type and generates JavaScript representation for it. This can be applied to:
///
/// ## Declaration of `struct`
///
/// ```
/// # #[macro_use] extern crate tslink;
/// # use tslink::tslink;
/// #[tslink]
/// struct MyStruct {
///     pub p8: u8,
///     pub p16: u16,
///     pub p32: u32,
///     pub p64: u64,
///     pub a64: u64,
/// }
/// ```
///
/// ## Implementation of `struct` and its methods
///
/// ```
/// # #[macro_use] extern crate tslink;
/// # use tslink::tslink;
///
/// struct MyStruct { }
///
/// #[tslink]
/// impl MyStruct {
///     #[tslink]
///     fn inc_num(&self, num: i32) -> i32 {
///         num + 1
///     }
/// }
/// ```
///
/// ## Declaration of `enum`
///
/// ```
/// # #[macro_use] extern crate tslink;
/// # use tslink::tslink;
/// #[tslink]
/// enum MyEnum {
///     One,
///     Two(i32),
///     Three(String),
///     Four(i32,i32),
/// }
/// ```
///
/// ## Functions `fn`
///
/// ```
/// # #[macro_use] extern crate tslink;
/// # use tslink::tslink;
///
/// #[tslink]
/// fn inc_num(num: i32) -> i32 {
///     num + 1
/// }
/// ```
///
/// `#[tslink]` uses multiple attributes to give flexibility with configuration and producing `*.js`/`*.d.ts`/`*.ts` artifacts. Please read more in the documentation.
///
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
        let cfg = CONFIG.read().expect("Read configuration");
        if let Err(err) = reader::read(&mut item, &mut natures, context, &cfg) {
            let str_err = err.to_string();
            return TryInto::<syn::Error>::try_into(err)
                .unwrap_or(syn::Error::new_spanned(item_ref.to_string(), str_err))
                .into_compile_error()
                .into();
        }
        proc_macro::TokenStream::from(item.to_token_stream())
    }
}
