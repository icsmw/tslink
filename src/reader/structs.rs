use crate::{
    context::Context,
    error::E,
    nature::{Extract, Nature, Refered},
};
use syn::{Fields, ImplItem, ItemFn};

pub fn is_method(item_fn: &ItemFn) -> bool {
    item_fn
        .sig
        .inputs
        .iter()
        .find(|input| matches!(input, syn::FnArg::Receiver(_)))
        .is_some()
}

// pub fn is_method(item_fn: &ItemFn, context: Context) -> Result<bool, E> {
//     if item_fn
//         .sig
//         .inputs
//         .iter()
//         .find(|input| matches!(input, syn::FnArg::Receiver(_)))
//         .is_some()
//     {
//         Ok(true)
//     } else {
//         Ok(Nature::extract(item_fn, context)?.is_self_returned())
//     }
// }

pub fn read_fields(fields: &Fields, parent: &mut Nature, parent_context: Context) -> Result<(), E> {
    match fields {
        Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                let mut context = Context::try_from_or_default(&field.attrs)?;
                context.set_parent(parent_context.clone());
                if context.ignore_self() {
                    continue;
                }
                let name = field.ident.clone().unwrap();
                parent.bind(Nature::Refered(Refered::Field(
                    name.to_string(),
                    context.clone(),
                    Box::new(Nature::extract(&field.ty, context.clone())?),
                )))?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn read_impl(
    items: &Vec<ImplItem>,
    parent: &mut Nature,
    struct_context: Context,
    _parent_context: Context,
) -> Result<(), E> {
    for item in items.iter() {
        match item {
            ImplItem::Fn(fn_item) => {
                let mut context = Context::try_from_or_default(&fn_item.attrs)?;
                context.set_parent(struct_context.clone());
                if context.ignore_self() {
                    continue;
                }
                let name = fn_item.sig.ident.to_string();
                if context.is_ignored(&name) {
                    continue;
                }
                parent.bind(Nature::Refered(Refered::Field(
                    name.to_string(),
                    context.clone(),
                    Box::new(Nature::extract(fn_item, context.clone())?),
                )))?;
            }
            _ => {}
        }
    }
    Ok(())
}
