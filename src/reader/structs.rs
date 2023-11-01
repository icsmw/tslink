use crate::{
    context::Context,
    error::E,
    modificator,
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
                    context.get_bound(&name.to_string()),
                )))?;
            }
            parent.check_ignored_fields()?;
        }
        _ => {}
    }
    Ok(())
}

pub fn read_impl(
    items: &mut Vec<ImplItem>,
    parent: &mut Nature,
    struct_context: Context,
    _parent_context: Context,
) -> Result<(), E> {
    for item in items.iter_mut() {
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
                let fn_nature = Nature::extract(&*fn_item, context.clone())?;
                parent.bind(Nature::Refered(Refered::Field(
                    name.clone(),
                    context.clone(),
                    Box::new(fn_nature.clone()),
                    context.get_bound(&name),
                )))?;
                modificator::bind_impl_fn(fn_item, &name, &context, &fn_nature)?;
            }
            _ => {}
        }
    }
    Ok(())
}
