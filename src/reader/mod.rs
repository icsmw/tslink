mod enums;
mod structs;

use std::ops::Deref;

use crate::{
    context::Context,
    error::E,
    interpreter,
    nature::{Extract, Nature, Natures, Refered},
    package,
};
use proc_macro_error::abort;
use syn::{Item, ItemEnum, ItemStruct};

pub fn read(item: Item, natures: &mut Natures, context: Context) -> Result<(), E> {
    let item_ref = item.clone();
    match item {
        Item::Struct(item_struct) => {
            let ItemStruct { ident, fields, .. } = item_struct;
            let name = ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                let mut nature =
                    Nature::Refered(Refered::Struct(name.clone(), context.clone(), vec![]));
                structs::read_fields(&fields, &mut nature, context.clone())?;
                natures.insert(&name, nature)
            }
        }
        Item::Enum(item_enum) => {
            let ItemEnum {
                ident, variants, ..
            } = item_enum;
            let name = ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                let mut nature =
                    Nature::Refered(Refered::Enum(name.clone(), context.clone(), vec![]));
                enums::read(&variants, &mut nature, context.clone())?;
                natures.insert(&name, nature)
            }
        }
        Item::Fn(item_fn) => {
            if structs::is_method(&item_fn) {
                return Ok(());
            }
            let name = item_fn.sig.ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                natures.insert(
                    &name,
                    Nature::Refered(Refered::Func(
                        name.clone(),
                        context.clone(),
                        Box::new(Nature::extract(&item_fn, context.clone())?),
                    )),
                )
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
                return Err(E::FailIdentify);
            };
            if let Some(nature) = natures.get_mut(&parent) {
                if let Nature::Refered(Refered::Struct(_, struct_context, _)) = nature.deref() {
                    structs::read_impl(
                        &item_impl.items,
                        nature,
                        struct_context.clone(),
                        context.clone(),
                    )
                } else {
                    Err(E::NotFoundStruct)
                }
            } else {
                Err(E::NotFoundStruct)
            }
        }
        _ => Ok(()),
    }?;
    if let Err(err) = package::create() {
        abort!(item_ref, err.to_string());
    }
    if let Err(err) = interpreter::ts(natures) {
        abort!(item_ref, err.to_string());
    }
    if let Err(err) = interpreter::dts(natures) {
        abort!(item_ref, err.to_string());
    }
    if let Err(err) = interpreter::js(natures) {
        abort!(item_ref, err.to_string());
    }
    Ok(())
}
