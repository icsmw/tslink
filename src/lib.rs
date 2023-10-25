mod args;
mod config;
mod defs;
mod interpreter;
mod package;
mod types;

#[macro_use]
extern crate lazy_static;

use args::Args;
use config::Config;
use defs::{detached::Detached, enums::Enums, structs::Structs, Entities, Entity};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use std::{collections::HashMap, sync::RwLock};
use syn::{parse_macro_input, Item, ItemEnum, ItemStruct};
use types::Types;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
    static ref ENTITIES: RwLock<Entities> = RwLock::new(HashMap::new());
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn tslink(args: TokenStream, item: TokenStream) -> TokenStream {
    if let Err(err) = config::setup() {
        panic!("{err}");
    }
    let args = parse_macro_input!(args as Args);
    if args.ignore_self() {
        item
    } else {
        let mut entities = ENTITIES.write().expect("Get access to list of entities");
        let output = item.clone();
        let item = parse_macro_input!(item as Item);
        read(item, &mut entities, args).expect("Read attribute");
        output
    }
}

fn read(item: Item, entities: &mut Entities, args: Args) -> Result<(), String> {
    let item_ref = item.clone();
    match item {
        Item::Struct(item_struct) => {
            let ItemStruct { ident, fields, .. } = item_struct;
            let name = ident.to_string();
            if entities.get(&name).is_some() {
                Err(format!("Struct \"{name}\" already exists"))
            } else {
                let mut strct = Structs::new(&name, args);
                strct.read(&fields)?;
                entities.insert(name, Entity::Struct(strct));
                Ok(())
            }
        }
        Item::Enum(item_enum) => {
            let ItemEnum {
                ident, variants, ..
            } = item_enum;
            let name = ident.to_string();
            if entities.get(&name).is_some() {
                Err(format!("Struct \"{name}\" already exists"))
            } else {
                let mut enums = Enums::new(&name, args);
                enums.read(&variants)?;
                entities.insert(name, Entity::Enum(enums));
                Ok(())
            }
        }
        Item::Fn(item_fn) => {
            if Structs::is_struct_method(&item_fn) {
                return Ok(());
            }
            let name = item_fn.sig.ident.to_string();
            if entities.get(&name).is_some() {
                Err(format!("Fn \"{name}\" already exists"))
            } else {
                entities.insert(
                    name,
                    Entity::Detached(Detached::new(Types::from(&item_fn), args)),
                );
                Ok(())
            }
        }
        Item::Impl(item_impl) => {
            let ident = match *item_impl.self_ty {
                syn::Type::Path(ref p) => p.path.get_ident(),
                _ => None,
            };
            let parent = if let Some(ident) = ident {
                ident.to_string()
            } else {
                return Err(format!("Fail to find a struct for method"));
            };
            entities
                .entry(parent.clone())
                .or_insert(Entity::Struct(Structs::new(&parent, args.set_as_class())));
            if let Some(entity) = entities.get_mut(&parent) {
                if let Entity::Struct(strct) = entity {
                    strct.read_impl(&item_impl.items, &args)?;
                    Ok(())
                } else {
                    Err(format!(
                        "Cannot find struct \"{parent}\" to parse implementation even found some entity with same name"
                    ))
                }
            } else {
                Err(format!(
                    "Cannot find struct \"{parent}\" to parse implementation"
                ))
            }
        }
        _ => Ok(()),
    }?;
    if let Err(err) = package::create() {
        abort!(item_ref, err.to_string());
    }
    if let Err(err) = interpreter::ts(entities) {
        abort!(item_ref, err.to_string());
    }
    if let Err(err) = interpreter::dts(entities) {
        abort!(item_ref, err.to_string());
    }
    if let Err(err) = interpreter::js(entities) {
        abort!(item_ref, err.to_string());
    }
    Ok(())
}
