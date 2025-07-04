use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    modificator,
    nature::{Extract, ExtractGenerics, Nature, Referred},
};
use syn::{Fields, ImplItem, ItemFn};

/// Returns `true` if the given function is an instance method (i.e., has a `self` receiver).
///
/// # Parameters
/// - `item_fn`: A parsed function item (`syn::ItemFn`).
///
/// # Returns
/// `true` if the function is a method (`fn self(...)`), `false` if it is a static function.
pub fn is_method(item_fn: &ItemFn) -> bool {
    item_fn
        .sig
        .inputs
        .iter()
        .any(|input| matches!(input, syn::FnArg::Receiver(_)))
}

/// Parses the fields of a Rust struct or tuple struct and binds them to the given parent `Nature`.
///
/// For named fields (`struct Foo { ... }`), each field is resolved into a `Referred::Field` with proper context,
/// including attribute processing and optional bindings.
///
/// For tuple structs (`struct Foo(Type);`), only the first unnamed field is parsed and bound.
///
/// # Parameters
/// - `fields`: The struct fields (`syn::Fields`).
/// - `parent`: The parent `Nature`, typically a `Referred::Struct` or `TupleStruct`.
/// - `parent_context`: Inherited macro context from the parent type.
/// - `cfg`: The global configuration settings.
///
/// # Errors
/// - If field type extraction fails.
/// - If the field layout is unsupported.
/// - If ignored fields are listed in attributes but not found in the struct definition.
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

/// Parses and binds methods from an `impl` block into the parent type as fields with function types.
///
/// For each method:
/// - Applies attribute context (`#[tslink(...)]`)
/// - Detects and propagates generics
/// - Extracts function type and associates it as `Referred::Field`
/// - Applies bound name mapping and invokes optional post-processing
///
/// # Parameters
/// - `items`: List of `ImplItem`s from the `impl` block.
/// - `parent`: The parent `Nature` (typically a `Referred::Struct`).
/// - `struct_context`: Context of the struct being implemented.
/// - `_parent_context`: Optional context from the outer scope (not used here).
/// - `cfg`: Global configuration settings.
///
/// # Errors
/// - If method parsing or binding fails.
/// - If context resolution fails.
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
