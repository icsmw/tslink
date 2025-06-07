use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    modificator,
    nature::{Extract, ExtractGenerics, Nature, Referred},
};
use syn::{Fields, ImplItem, ItemFn};

pub fn is_method(item_fn: &ItemFn) -> bool {
    item_fn
        .sig
        .inputs
        .iter()
        .any(|input| matches!(input, syn::FnArg::Receiver(_)))
}

pub fn read_fields(
    fields: &Fields,
    parent: &mut Nature,
    parent_context: Context,
    cfg: &Config,
) -> Result<(), E> {
    if let Fields::Named(ref fields) = fields {
        for field in fields.named.iter() {
            let mut context = Context::try_from_or_default(&field.attrs)?;
            context.set_parent(parent_context.clone());
            if context.ignore_self() {
                continue;
            }
            let name = field.ident.clone().unwrap();
            parent.bind(Nature::Referred(Referred::Field(
                serialize_name(name.to_string()),
                context.clone(),
                Box::new(Nature::extract(&field.ty, context.clone(), cfg)?),
                context.get_bound(&name.to_string()),
            )))?;
        }
        parent.check_ignored_fields()?;
    } else if let Fields::Unnamed(ref fields) = fields {
        if let Some(field) = fields.unnamed.first() {
            let mut context = Context::default();
            context.set_parent(parent_context.clone());
            parent.bind(Nature::Referred(Referred::Field(
                String::new(),
                context.clone(),
                Box::new(Nature::extract(&field.ty, context.clone(), cfg)?),
                None,
            )))?;
        }
    } else {
        return Err(E::NotSupported(String::from("Unsupported type of fields")));
    }
    Ok(())
}

pub fn read_impl(
    items: &mut [ImplItem],
    parent: &mut Nature,
    struct_context: Context,
    _parent_context: Context,
    cfg: &Config,
) -> Result<(), E> {
    for item in items.iter_mut() {
        if let ImplItem::Fn(fn_item) = item {
            let mut context = Context::try_from_or_default(&fn_item.attrs)?;
            context.set_parent(struct_context.clone());
            context.add_generics(Nature::extract_generics(&fn_item.sig.generics, cfg)?);
            if context.ignore_self() {
                continue;
            }
            let name = fn_item.sig.ident.to_string();
            if context.is_ignored(&name) {
                continue;
            }
            let fn_nature = Nature::extract(&*fn_item, context.clone(), cfg)?;
            parent.bind(Nature::Referred(Referred::Field(
                serialize_name(&name),
                context.clone(),
                Box::new(fn_nature.clone()),
                context.get_bound(&name),
            )))?;
            modificator::bind_impl_fn(fn_item, &name, &context, &fn_nature)?;
        }
    }
    Ok(())
}
