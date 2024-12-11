mod enums;
mod structs;

use crate::{
    config::{self, Config},
    context::Context,
    error::E,
    interpreter::{self, serialize_name},
    modificator,
    nature::{Composite, Extract, ExtractGenerics, Nature, Natures, Refered},
    package,
};
use std::ops::Deref;
use syn::{Fields, Item, ItemEnum, ItemStruct};

pub fn read(
    item: &mut Item,
    natures: &mut Natures,
    mut context: Context,
    cfg: &Config,
) -> Result<(), E> {
    let io_allowed = config::get()?.io_allowed;
    let item_ref = item.clone();
    match item {
        Item::Struct(item_struct) => {
            let ItemStruct { ident, fields, .. } = item_struct;
            let name = ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(quote::quote! { #item }.to_string()))
            } else {
                context.add_generics(Nature::extract_generics(&item_struct.generics, cfg)?);
                let mut nature = if matches!(fields, Fields::Unnamed(..)) {
                    Nature::Refered(Refered::TupleStruct(
                        serialize_name(&name),
                        context.clone(),
                        None,
                    ))
                } else {
                    Nature::Refered(Refered::Struct(
                        serialize_name(&name),
                        context.clone(),
                        vec![],
                    ))
                };
                structs::read_fields(fields, &mut nature, context.clone(), cfg)?;
                natures.insert(&name, nature, context.get_module())
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
                let mut nature = Nature::Refered(Refered::Enum(
                    serialize_name(&name),
                    context.clone(),
                    vec![],
                ));
                enums::read(variants, &mut nature, context.clone(), cfg)?;
                natures.insert(&name, nature, context.get_module())
            }
        }
        Item::Fn(item_fn) => {
            if structs::is_method(item_fn) {
                return Ok(());
            }
            context.add_generics(Nature::extract_generics(&item_fn.sig.generics, cfg)?);
            if let Nature::Composite(Composite::Func(_, _, _, _, constructor)) =
                Nature::extract(&*item_fn, context.clone(), cfg)?
            {
                if constructor {
                    return Ok(());
                }
            }
            let name = item_fn.sig.ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                let fn_nature = Nature::extract(&*item_fn, context.clone(), cfg)?;
                let _ = natures.insert(
                    &name,
                    Nature::Refered(Refered::Func(
                        serialize_name(&name),
                        context.clone(),
                        Box::new(fn_nature.clone()),
                    )),
                    context.get_module(),
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
                    serialize_name(&struct_name),
                    context.clone(),
                    vec![],
                ))),
                context.get_module(),
            ) {
                if let Nature::Refered(Refered::Struct(_, struct_context, _)) = nature.deref() {
                    structs::read_impl(
                        &mut item_impl.items,
                        nature,
                        struct_context.clone(),
                        context.clone(),
                        cfg,
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
        interpreter::ts(natures)
            .map_err(|e| E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string())))?;
        if cfg.node_mod_filename.is_some() {
            package::create().map_err(|e| {
                E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string()))
            })?;
            interpreter::dts(natures).map_err(|e| {
                E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string()))
            })?;
            interpreter::js(natures).map_err(|e| {
                E::Compiler(syn::Error::new_spanned(item_ref.clone(), e.to_string()))
            })?;
        }
    }
    Ok(())
}
