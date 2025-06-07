use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    nature::{Extract, Nature, Referred},
};
use syn::{punctuated::Punctuated, token::Comma, Fields};

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
