mod config;
mod context;
mod error;
mod interpreter;
mod nature;
mod package;
mod reader;

#[macro_use]
extern crate lazy_static;

use config::Config;
use context::Context;
use nature::Natures;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use std::sync::RwLock;
use syn::{parse_macro_input, Item, ItemEnum, ItemStruct};

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
    static ref NATURES: RwLock<Natures> = RwLock::new(Natures::new());
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn tslink(args: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(err) = config::setup() {
        panic!("{err}");
    }
    let context: Context = parse_macro_input!(args as Context);
    if context.ignore_self() {
        item
    } else {
        let mut natures = NATURES.write().expect("Get access to list of entities");
        let output = item.clone();
        let item = parse_macro_input!(item as Item);
        if let Err(err) = reader::read(item, &mut natures, context) {
            panic!("{err}");
            // abort!(output, err.to_string());
        }
        output
    }
}
