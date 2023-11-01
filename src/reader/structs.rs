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
        .any(|input| matches!(input, syn::FnArg::Receiver(_)))
}

pub fn read_fields(fields: &Fields, parent: &mut Nature, parent_context: Context) -> Result<(), E> {
    if let Fields::Named(ref fields) = fields {
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
    Ok(())
}

pub fn read_impl(
    items: &mut [ImplItem],
    parent: &mut Nature,
    struct_context: Context,
    _parent_context: Context,
) -> Result<(), E> {
    for item in items.iter_mut() {
        if let ImplItem::Fn(fn_item) = item {
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
    }
    Ok(())
}
