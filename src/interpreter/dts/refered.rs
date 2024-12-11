use super::Interpreter;
use crate::{
    error::E,
    interpreter::Offset,
    nature::{Composite, Nature, Natures, Refered},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::Deref,
};

impl Interpreter for Refered {
    fn declaration(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Refered::Enum(name, _context, variants) => {
                let flat = Refered::is_flat_varians(variants)?;
                buf.write_all(
                    format!(
                        "{offset}export {} {name} {{\n",
                        if flat { "enum" } else { "interface" }
                    )
                    .as_bytes(),
                )?;
                for variant in variants.iter() {
                    variant.declaration(natures, buf, offset.inc())?;
                    buf.write_all(if flat { ",\n" } else { ";\n" }.as_bytes())?;
                }
                buf.write_all(format!("{offset}}}\n",).as_bytes())?;
            }
            Refered::EnumVariant(name, _context, fields, flat) => {
                if fields.is_empty() {
                    if *flat {
                        buf.write_all(format!("{offset}{name}").as_bytes())?;
                    } else {
                        buf.write_all(format!("{offset}{name}?: null").as_bytes())?;
                    }
                } else if fields.len() == 1 {
                    buf.write_all(format!("{offset}{name}?: ").as_bytes())?;
                    fields
                        .first()
                        .ok_or(E::Parsing(String::from(
                            "Expecting single field for Variant",
                        )))?
                        .reference(natures, buf, offset.inc())?;
                } else {
                    buf.write_all(format!("{offset}{name}?: [").as_bytes())?;
                    for (i, field) in fields.iter().enumerate() {
                        field.reference(natures, buf, offset.inc())?;
                        if i < fields.len() - 1 {
                            buf.write_all(", ".as_bytes())?;
                        }
                    }
                    buf.write_all("]".as_bytes())?;
                }
            }
            Refered::Field(name, context, _, _) => {
                buf.write_all(format!("{}: ", context.rename_field(name)?).as_bytes())?
            }
            Refered::Func(name, context, func) => {
                if let Nature::Composite(Composite::Func(_, args, out, asyncness, constructor)) =
                    func.deref()
                {
                    if *constructor {
                        return Ok(());
                    }
                    let renamed = context.rename_method(name)?;
                    buf.write_all(
                        format!("{}export declare function {renamed}(", offset).as_bytes(),
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
            Refered::FuncArg(name, _context, nature, binding) => {
                buf.write_all(format!("{name}: ").as_bytes())?;
                if let Some(ref_name) = binding {
                    buf.write_all(ref_name.as_bytes())?;
                } else {
                    nature.reference(natures, buf, offset.clone())?;
                }
            }
            Refered::Struct(name, context, fields) => {
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
            Refered::TupleStruct(name, _context, field) => {
                buf.write_all(format!("{offset}export type {name} = ").as_bytes())?;
                if let Some(field) = field {
                    field.reference(natures, buf, Offset::new())?;
                } else {
                    buf.write_all("undefined".as_bytes())?;
                }
                buf.write_all(";\n".as_bytes())?;
            }
            Refered::Ref(ref_name, _) => {
                return Err(E::Parsing(format!("Reference {ref_name} can be declared")));
            }
            Refered::Generic(alias, _) => {
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
            Refered::Enum(name, _, _) => buf.write_all(name.as_bytes())?,
            Refered::EnumVariant(name, _, _, _) => {
                buf.write_all(format!("{offset}{name}: ").as_bytes())?
            }
            Refered::Field(name, context, nature, _) => {
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
                    if let Nature::Refered(Refered::Ref(ref_name, _)) = nature.deref() {
                        if let Some(generic) = context.get_generic(ref_name) {
                            generic.reference(natures, buf, offset)?;
                            return Ok(());
                        }
                    }
                    nature.reference(natures, buf, offset)?;
                }
            }
            Refered::Func(name, _, _) => buf.write_all(name.as_bytes())?,
            Refered::FuncArg(name, _, _, _) => {
                return Err(E::Parsing(format!(
                    "Function argument {name} can be refered"
                )));
            }
            Refered::Struct(name, _, _) => buf.write_all(name.as_bytes())?,
            Refered::TupleStruct(name, _, _) => buf.write_all(name.as_bytes())?,
            Refered::Ref(ref_name, context) => {
                if let Some(context) = context {
                    if let Some(nature) = context.get_generic(ref_name) {
                        nature.reference(natures, buf, offset)?;
                        return Ok(());
                    }
                }
                buf.write_all(ref_name.as_bytes())?;
            }
            Refered::Generic(alias, _) => {
                return Err(E::Parsing(format!(
                    "Generic type cannot be rendered out of context; type alias = {alias}"
                )))
            }
        }
        Ok(())
    }
}
