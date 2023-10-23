use crate::{
    args::{Args, ArgsWriter},
    types::Types,
};
use std::{collections::HashMap, convert::From};
use syn::{Fields, ImplItem, ItemFn};

#[derive(Clone, Debug)]
pub struct Field {
    pub ty: Types,
    pub name: String,
    pub args: Option<Args>,
}

impl Field {
    pub fn new(name: String, ty: Types, args: Option<Args>) -> Self {
        Self { name, ty, args }
    }
}

#[derive(Clone, Debug)]
pub struct Structs {
    pub fields: HashMap<String, Field>,
    pub args: Args,
    pub name: String,
}

impl ArgsWriter for Structs {
    fn get_args(&self) -> &Args {
        &self.args
    }
}

impl Structs {
    pub fn is_struct_method(item_fn: &ItemFn) -> bool {
        if item_fn
            .sig
            .inputs
            .iter()
            .find(|input| matches!(input, syn::FnArg::Receiver(_)))
            .is_some()
        {
            true
        } else {
            Types::from(item_fn).is_self_returned()
        }
    }
    fn ignore(args: &Option<Args>) -> bool {
        if let Some(args) = args {
            args.ignore_self()
        } else {
            false
        }
    }

    pub fn new(name: &str, args: Args) -> Self {
        Structs {
            fields: HashMap::new(),
            args,
            name: name.to_string(),
        }
    }

    pub fn read(&mut self, fields: &Fields) -> Result<(), String> {
        match fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    let args = Args::from_attrs(&field.attrs);
                    if Self::ignore(&args) {
                        continue;
                    }
                    let name = field.ident.clone().unwrap();
                    self.fields.insert(
                        name.to_string(),
                        Field::new(name.to_string(), Types::from(&field.ty), args),
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn read_impl(&mut self, items: &Vec<ImplItem>, parent_args: &Args) -> Result<(), String> {
        for item in items.iter() {
            match item {
                ImplItem::Fn(fn_item) => {
                    let args = Args::from_attrs(&fn_item.attrs);
                    if Self::ignore(&args) {
                        continue;
                    }
                    let name = fn_item.sig.ident.to_string();
                    if parent_args.is_ignored(&name) {
                        continue;
                    }
                    self.fields
                        .insert(name.clone(), Field::new(name, Types::from(fn_item), args));
                }
                _ => {}
            }
        }
        Ok(())
    }
}
