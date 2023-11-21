mod enums;
mod structs;

use crate::{
    config,
    context::Context,
    error::E,
    interpreter, modificator,
    nature::{Composite, Extract, ExtractGenerics, Nature, Natures, Refered},
    package,
};
use std::ops::Deref;
use syn::{Item, ItemEnum, ItemStruct};

pub fn read(item: &mut Item, natures: &mut Natures, mut context: Context) -> Result<(), E> {
    let io_allowed = config::get()?.io_allowed;
    let item_ref = item.clone();
    match item {
        Item::Struct(item_struct) => {
            let ItemStruct { ident, fields, .. } = item_struct;
            let name = ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                context.add_generics(Nature::extract_generics(&item_struct.generics)?);
                let mut nature =
                    Nature::Refered(Refered::Struct(name.clone(), context.clone(), vec![]));
                structs::read_fields(fields, &mut nature, context.clone())?;
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
                enums::read(variants, &mut nature, context.clone())?;
                natures.insert(&name, nature)
            }
        }
        Item::Fn(item_fn) => {
            if structs::is_method(item_fn) {
                return Ok(());
            }
            context.add_generics(Nature::extract_generics(&item_fn.sig.generics)?);
            if let Nature::Composite(Composite::Func(_, _, _, _, constructor)) =
                Nature::extract(&*item_fn, context.clone())?
            {
                if constructor {
                    return Ok(());
                }
            }
            let name = item_fn.sig.ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                let fn_nature = Nature::extract(&*item_fn, context.clone())?;
                let _ = natures.insert(
                    &name,
                    Nature::Refered(Refered::Func(
                        name.clone(),
                        context.clone(),
                        Box::new(fn_nature.clone()),
                    )),
                );
                modificator::bind_fn(item_fn, &name, &context, &fn_nature)?;
                Ok(())
            }
        }
        Item::Impl(item_impl) => {
            let ident = match *item_impl.self_ty {
                syn::Type::Path(ref p) => p.path.get_ident(),
                _ => None,
            };
            let struct_name = if let Some(ident) = ident {
                ident.to_string()
            } else {
                return Err(E::FailIdentify);
            };
            if let Some(nature) = natures.get_mut(
                &struct_name,
                Some(Nature::Refered(Refered::Struct(
                    struct_name.clone(),
                    context.clone(),
                    vec![],
                ))),
            ) {
                if let Nature::Refered(Refered::Struct(_, struct_context, _)) = nature.deref() {
                    structs::read_impl(
                        &mut item_impl.items,
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
    if io_allowed {
        package::create()
            .map_err(|e| E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string())))?;
        interpreter::ts(natures)
            .map_err(|e| E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string())))?;
        interpreter::dts(natures)
            .map_err(|e| E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string())))?;
        interpreter::js(natures)
            .map_err(|e| E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string())))?;
    }
    Ok(())
}
