use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    nature::{Extract, Nature, Referred},
};
use syn::{punctuated::Punctuated, token::Comma, Fields};

/// Parses the fields of an enum variant into a list of `Nature` values.
///
/// Supports named (`struct-like`), unnamed (`tuple-like`), and unit variants.
///
/// # Parameters
/// - `fields`: The fields of the enum variant (`syn::Fields`).
/// - `context`: The current macro context for tracking attributes and scope.
/// - `cfg`: The global configuration settings.
///
/// # Returns
/// A list of `Nature` instances representing the field types of the variant.
///
/// # Errors
/// Returns an error if any field type fails to be parsed or resolved.
fn read_variant(fields: &Fields, context: Context, cfg: &Config) -> Result<Vec<Nature>, E> {
    let mut values: Vec<Nature> = vec![];
    match fields {
        Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                let name = field.ident.clone().unwrap();
                values.push(Nature::Referred(Referred::Field(
                    serialize_name(name.to_string()),
                    context.clone(),
                    Box::new(Nature::extract(&field.ty, context.clone(), cfg)?),
                    None,
                )));
            }
        }
        Fields::Unnamed(ref fields) => {
            for field in fields.unnamed.iter() {
                values.push(Nature::extract(&field.ty, context.clone(), cfg)?);
            }
        }
        Fields::Unit => {}
    }
    Ok(values)
}

/// Reads all variants of an enum and attaches them to the given parent as `Referred::EnumVariant`.
///
/// This function filters out ignored variants, parses their fields via `read_variant`, and binds
/// each as a structured representation suitable for TypeScript generation.
///
/// # Parameters
/// - `variants`: The list of enum variants to process.
/// - `parent`: The parent `Nature`, expected to be a `Referred::Enum` into which the variants are bound.
/// - `context`: The current macro context.
/// - `cfg`: Global configuration settings for parsing behavior.
///
/// # Errors
/// Returns an error if field parsing fails or variant binding encounters an issue.
pub fn read(
    variants: &Punctuated<syn::Variant, Comma>,
    parent: &mut Nature,
    context: Context,
    cfg: &Config,
) -> Result<(), E> {
    let mut fields: Vec<(String, Vec<Nature>)> = vec![];
    for variant in variants {
        let name = variant.ident.to_string();
        if context.is_ignored(&name) {
            continue;
        }
        fields.push((name, read_variant(&variant.fields, context.clone(), cfg)?));
    }
    let not_flat = fields.iter().any(|(_, v)| !v.is_empty());
    for (name, values) in fields {
        parent.bind(Nature::Referred(Referred::EnumVariant(
            serialize_name(&name),
            context.clone(),
            values,
            !not_flat,
            cfg.enum_representation.clone(),
        )))?;
    }
    Ok(())
}
