mod enums;
mod structs;

use crate::{
    config::{self, Config},
    context::Context,
    error::E,
    interpreter::{self, serialize_name},
    modificator,
    nature::{Composite, Extract, ExtractGenerics, Nature, Natures, Referred},
    package,
};
use quote::ToTokens;
use std::ops::Deref;
use syn::{Fields, Item, ItemConst, ItemEnum, ItemStruct};

/// Main entry point for reading and interpreting a Rust item (`struct`, `enum`, `fn`, `impl`, `const`) into a typed [`Nature`] representation.
///
/// This function is responsible for analyzing the annotated Rust item, extracting structural type information,
/// and inserting the result into the shared [`Natures`] registry. Based on the item's kind, the function performs:
///
/// - For `struct` and `tuple struct`: Collects fields, determines representation, and stores as `Referred::Struct` or `Referred::TupleStruct`.
/// - For `enum`: Parses variants and stores as `Referred::Enum`.
/// - For `fn`: Extracts function signature (unless it's a method or constructor), stores as `Referred::Func`.
/// - For `impl`: Merges methods into the previously defined struct.
/// - For `const`: Stores as `Referred::Constant`.
///
/// If output generation is enabled (`io_allowed`), the function also invokes TypeScript/JavaScript generation
/// via `interpreter::ts`, `interpreter::dts`, and `interpreter::js`.
///
/// # Parameters
/// - `item`: The input Rust item to analyze and transform.
/// - `natures`: The registry to collect parsed types (`Natures`).
/// - `context`: Macro-level context, including configuration and attributes.
/// - `cfg`: Global generation configuration.
///
/// # Errors
/// - Returns an error if the item is unsupported, malformed, or already exists in the registry.
/// - Also returns detailed errors if generation of `.ts`, `.d.ts`, or `.js` files fails.
///
/// # Side Effects
/// - May perform file I/O if `cfg.io_allowed == true` and target paths are configured.
///
/// # Note
/// This function is the root of all type discovery and macro processing —
/// it's designed to be called once per `Item` encountered by a derive or attribute macro.
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
                    Nature::Referred(Referred::TupleStruct(
                        serialize_name(&name),
                        context.clone(),
                        None,
                    ))
                } else {
                    Nature::Referred(Referred::Struct(
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
                let mut nature = Nature::Referred(Referred::Enum(
                    serialize_name(&name),
                    context.clone(),
                    vec![],
                    cfg.enum_representation.clone(),
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
                    Nature::Referred(Referred::Func(
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
                Some(Nature::Referred(Referred::Struct(
                    serialize_name(&struct_name),
                    context.clone(),
                    vec![],
                ))),
                context.get_module(),
            ) {
                if let Nature::Referred(Referred::Struct(_, struct_context, _)) = nature.deref() {
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
        Item::Const(item_const) => {
            let ItemConst {
                ident, ty, expr, ..
            } = item_const;
            let name = ident.to_string();
            if natures.contains(&name) {
                Err(E::EntityExist(name))
            } else {
                let nature = Nature::Referred(Referred::Constant(
                    serialize_name(ident.to_string()),
                    context.to_owned(),
                    Box::new(Nature::extract(ty.as_ref(), context.to_owned(), cfg)?),
                    expr.as_ref().into_token_stream().to_string(),
                ));
                natures.insert(&name, nature, context.get_module())
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
