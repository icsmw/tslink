use super::Interpreter;
use crate::{
    config::cfg::EnumRepresentation,
    error::E,
    interpreter::Offset,
    nature::{Composite, Nature, Natures, Referred},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::Deref,
};

impl Interpreter for Referred {
    fn declaration(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Referred::Enum(name, _context, variants, repres) => {
                let flat = Referred::is_flat_varians(variants)?;
                if flat {
                    buf.write_all(format!("{offset}export enum {name} {{\n",).as_bytes())?;
                    for variant in variants.iter() {
                        variant.declaration(natures, buf, offset.inc())?;
                        buf.write_all(",\n".as_bytes())?;
                    }
                    buf.write_all(format!("{offset}}}\n",).as_bytes())?;
                } else {
                    match repres {
                        EnumRepresentation::Flat => {
                            buf.write_all(
                                format!("{offset}export interface {name} {{\n",).as_bytes(),
                            )?;
                            for variant in variants.iter() {
                                variant.declaration(natures, buf, offset.inc())?;
                                buf.write_all(";\n".as_bytes())?;
                            }
                            buf.write_all(format!("{offset}}}\n",).as_bytes())?;
                        }
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.write_all(format!("{offset}export type {name} =\n",).as_bytes())?;
                            for (n, variant) in variants.iter().enumerate() {
                                variant.declaration(natures, buf, offset.inc())?;
                                buf.write_all(
                                    if n == variants.len() - 1 { "" } else { " |\n" }.as_bytes(),
                                )?;
                            }
                            buf.write_all(format!("{offset};\n",).as_bytes())?;
                        }
                    }
                }
            }
            Referred::EnumVariant(name, _context, fields, flat, repres) => {
                let named = fields
                    .iter()
                    .any(|f| matches!(f, Nature::Referred(Referred::Field(..))));
                if fields.is_empty() {
                    if *flat {
                        buf.write_all(format!("{offset}{name}").as_bytes())?;
                    } else {
                        match repres {
                            EnumRepresentation::Flat => {
                                buf.write_all(format!("{offset}{name}?: null").as_bytes())?;
                            }
                            EnumRepresentation::Union => {
                                buf.write_all(format!("{offset}{{ {name}: null }}").as_bytes())?;
                            }
                            EnumRepresentation::DiscriminatedUnion => {
                                buf.write_all(format!("{offset}\"{name}\"").as_bytes())?;
                            }
                        }
                    }
                } else if fields.len() == 1 {
                    match repres {
                        EnumRepresentation::Flat => {
                            buf.write_all(format!("{offset}{name}?: ").as_bytes())?;
                        }
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.write_all(
                                format!(
                                    "{offset}{{{}{name}: ",
                                    if named {
                                        format!("\n{}", offset.inc())
                                    } else {
                                        " ".to_owned()
                                    }
                                )
                                .as_bytes(),
                            )?;
                        }
                    }
                    if named {
                        buf.write_all("{\n".as_bytes())?;
                    }
                    fields
                        .first()
                        .ok_or(E::Parsing(String::from(
                            "Expecting single field for Variant",
                        )))?
                        .reference(
                            natures,
                            buf,
                            match repres {
                                EnumRepresentation::Flat => offset.inc(),
                                EnumRepresentation::Union
                                | EnumRepresentation::DiscriminatedUnion => offset.inc().inc(),
                            },
                        )?;
                    if named {
                        buf.write_all(
                            format!(
                                "\n{}}}",
                                match repres {
                                    EnumRepresentation::Flat => offset.clone(),
                                    EnumRepresentation::Union
                                    | EnumRepresentation::DiscriminatedUnion => {
                                        offset.inc()
                                    }
                                }
                            )
                            .as_bytes(),
                        )?;
                    }
                    match repres {
                        EnumRepresentation::Flat => {}
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.write_all(
                                if named {
                                    format!("\n{offset}}}",)
                                } else {
                                    " }".to_owned()
                                }
                                .as_bytes(),
                            )?;
                        }
                    }
                } else {
                    match repres {
                        EnumRepresentation::Flat => {
                            buf.write_all(
                                format!("{offset}{name}?: {}", if named { "{\n" } else { "[" })
                                    .as_bytes(),
                            )?;
                        }
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.write_all(
                                format!(
                                    "{offset}{{{}{name}: {}",
                                    if named {
                                        format!("\n{}", offset.inc())
                                    } else {
                                        " ".to_owned()
                                    },
                                    if named { "{\n" } else { "[" }
                                )
                                .as_bytes(),
                            )?;
                        }
                    }
                    for (i, field) in fields.iter().enumerate() {
                        field.reference(
                            natures,
                            buf,
                            match repres {
                                EnumRepresentation::Flat => offset.inc(),
                                EnumRepresentation::Union
                                | EnumRepresentation::DiscriminatedUnion => offset.inc().inc(),
                            },
                        )?;
                        if i < fields.len() - 1 {
                            buf.write_all(if named { ";\n" } else { "," }.as_bytes())?;
                        }
                    }
                    buf.write_all(
                        if named {
                            format!(
                                "\n{}}}",
                                match repres {
                                    EnumRepresentation::Flat => offset.clone(),
                                    EnumRepresentation::Union
                                    | EnumRepresentation::DiscriminatedUnion => {
                                        offset.inc()
                                    }
                                }
                            )
                        } else {
                            "]".to_owned()
                        }
                        .as_bytes(),
                    )?;
                    match repres {
                        EnumRepresentation::Flat => {}
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.write_all(
                                if named {
                                    format!("\n{offset}}}",)
                                } else {
                                    " }".to_owned()
                                }
                                .as_bytes(),
                            )?;
                        }
                    }
                }
            }
            Referred::Field(name, context, ..) => {
                buf.write_all(format!("{}: ", context.rename_field(name)?).as_bytes())?
            }
            Referred::Func(name, context, func) => {
                if let Nature::Composite(Composite::Func(_, args, out, asyncness, constructor)) =
                    func.deref()
                {
                    if *constructor {
                        return Ok(());
                    }
                    let renamed = context.rename_method(name)?;
                    buf.write_all(
                        format!("{offset}export declare function {renamed}(",).as_bytes(),
                    )?;
                    for (i, ty) in args.iter().enumerate() {
                        ty.declaration(natures, buf, Offset::new())?;
                        if i < args.len() - 1 {
                            buf.write_all(", ".as_bytes())?;
                        }
                    }
                    buf.write_all("): ".as_bytes())?;
                    if *asyncness {
                        buf.write_all("Promise<".as_bytes())?;
                    }
                    if let Some(out) = out {
                        out.reference(natures, buf, offset.clone())?;
                    } else {
                        buf.write_all("void".as_bytes())?;
                    }
                    if *asyncness {
                        buf.write_all(">".as_bytes())?;
                    }
                    buf.write_all(";\n".as_bytes())?;
                } else {
                    return Err(E::Parsing(format!("Cannot find body of function {name}")));
                }
            }
            Referred::FuncArg(name, _context, nature, binding) => {
                buf.write_all(format!("{name}: ").as_bytes())?;
                if let Some(ref_name) = binding {
                    buf.write_all(ref_name.as_bytes())?;
                } else {
                    nature.reference(natures, buf, offset.clone())?;
                }
            }
            Referred::Struct(name, context, fields) => {
                buf.write_all(
                    format!(
                        "{offset}{} {name} {{\n",
                        if context.as_class() {
                            "export declare class"
                        } else {
                            "export interface"
                        },
                    )
                    .as_bytes(),
                )?;
                for field in fields {
                    if field.is_field_ignored() {
                        continue;
                    }
                    field.reference(natures, buf, offset.inc())?;
                    buf.write_all(";\n".as_bytes())?;
                }
                buf.write_all(format!("{offset}}}\n",).as_bytes())?;
            }
            Referred::TupleStruct(name, _context, field) => {
                buf.write_all(format!("{offset}export type {name} = ").as_bytes())?;
                if let Some(field) = field {
                    field.reference(natures, buf, Offset::new())?;
                } else {
                    buf.write_all("undefined".as_bytes())?;
                }
                buf.write_all(";\n".as_bytes())?;
            }
            Referred::Constant(name, _context, _ty, value) => {
                buf.write_all(format!("{offset}export const {name} = {value};\n").as_bytes())?;
            }
            Referred::Ref(ref_name, ..) => {
                return Err(E::Parsing(format!("Reference {ref_name} can be declared")));
            }
            Referred::Generic(alias, ..) => {
                return Err(E::Parsing(format!(
                    "Generic type cannot be rendered out of context; type alias = {alias}"
                )))
            }
        }
        Ok(())
    }

    fn reference(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Referred::Enum(name, ..) => buf.write_all(name.as_bytes())?,
            Referred::EnumVariant(name, ..) => {
                buf.write_all(format!("{offset}{name}: ").as_bytes())?
            }
            Referred::Field(name, context, nature, ..) => {
                if let Nature::Composite(Composite::Func(_, _, _, _, constructor)) = nature.deref()
                {
                    if *constructor {
                        buf.write_all(format!("{offset}constructor").as_bytes())?;
                    } else {
                        buf.write_all(
                            format!("{offset}{}", context.rename_method(name)?).as_bytes(),
                        )?;
                    }
                    nature.reference(natures, buf, offset)?;
                } else {
                    buf.write_all(format!("{offset}{}: ", context.rename_field(name)?).as_bytes())?;
                    if let Nature::Referred(Referred::Ref(ref_name, _)) = nature.deref() {
                        if let Some(generic) = context.get_generic(ref_name) {
                            generic.reference(natures, buf, offset)?;
                            return Ok(());
                        }
                    }
                    nature.reference(natures, buf, offset)?;
                }
            }
            Referred::Func(name, ..) => buf.write_all(name.as_bytes())?,
            Referred::FuncArg(name, ..) => {
                return Err(E::Parsing(format!(
                    "Function argument {name} can be refered"
                )));
            }
            Referred::Struct(name, ..) => buf.write_all(name.as_bytes())?,
            Referred::TupleStruct(name, ..) => buf.write_all(name.as_bytes())?,
            Referred::Ref(ref_name, context) => {
                if let Some(context) = context {
                    if let Some(nature) = context.get_generic(ref_name) {
                        nature.reference(natures, buf, offset)?;
                        return Ok(());
                    }
                }
                buf.write_all(ref_name.as_bytes())?;
            }
            Referred::Constant(name, ..) => {
                return Err(E::Parsing(format!("Constant {name} can be refered")));
            }
            Referred::Generic(alias, _) => {
                return Err(E::Parsing(format!(
                    "Generic type cannot be rendered out of context; type alias = {alias}"
                )))
            }
        }
        Ok(())
    }
}
