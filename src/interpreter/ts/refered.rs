use super::Interpreter;
use crate::{
    error::E,
    interpreter::{ts::Writer, Offset},
    nature::{Composite, Nature, Natures, Refered},
};
use std::ops::Deref;

impl Interpreter for Refered {
    fn declaration(
        &self,
        natures: &Natures,
        buf: &mut Writer,
        offset: Offset,
        parent: Option<String>,
    ) -> Result<(), E> {
        match self {
            Refered::Enum(name, _context, variants) => {
                let flat = Refered::is_flat_varians(variants)?;
                buf.push(format!(
                    "{offset}export {} {name} {{\n",
                    if flat { "enum" } else { "interface" }
                ));
                for variant in variants.iter() {
                    variant.declaration(natures, buf, offset.inc(), Some(name.to_owned()))?;
                    buf.push(if flat { ",\n" } else { ";\n" });
                }
                buf.push(format!("{offset}}}\n",));
            }
            Refered::EnumVariant(name, _context, fields, flat) => {
                if fields.is_empty() {
                    if *flat {
                        buf.push(format!("{offset}{name}"));
                    } else {
                        buf.push(format!("{offset}{name}?: null"));
                    }
                } else if fields.len() == 1 {
                    buf.push(format!("{offset}{name}?: "));
                    fields
                        .first()
                        .ok_or(E::Parsing(String::from(
                            "Expecting single field for Variant",
                        )))?
                        .reference(natures, buf, offset.inc(), parent)?;
                } else {
                    buf.push(format!("{offset}{name}?: ["));
                    for (i, field) in fields.iter().enumerate() {
                        field.reference(natures, buf, offset.inc(), parent.clone())?;
                        if i < fields.len() - 1 {
                            buf.push(", ");
                        }
                    }
                    buf.push("]");
                }
            }
            Refered::Field(name, context, _, _) => {
                buf.push(format!("{}: ", context.rename_field(name)?))
            }
            Refered::Func(name, context, func) => {
                if let Nature::Composite(Composite::Func(_, args, out, asyncness, constructor)) =
                    func.deref()
                {
                    if *constructor {
                        return Err(E::Parsing(
                            "Cannot declare constructor for abstract class".to_string(),
                        ));
                    }
                    buf.push(format!(
                        "{}export declare function {}(",
                        offset,
                        context.rename_method(name)?
                    ));
                    for (i, ty) in args.iter().enumerate() {
                        ty.declaration(natures, buf, Offset::new(), Some(name.to_owned()))?;
                        if i < args.len() - 1 {
                            buf.push(", ");
                        }
                    }
                    buf.push("): ");
                    if *asyncness {
                        buf.push("Promise<");
                    }
                    if let Some(out) = out {
                        out.reference(natures, buf, offset.clone(), Some(name.to_owned()))?;
                    } else {
                        buf.push("void");
                    }
                    if *asyncness {
                        buf.push(">");
                    }
                    buf.push(";\n");
                } else {
                    return Err(E::Parsing(format!("Cannot find body of function {name}")));
                }
            }
            Refered::FuncArg(name, _context, nature, _) => {
                buf.push(format!("{name}: "));
                nature.reference(natures, buf, offset.clone(), parent)?;
            }
            Refered::Struct(name, context, fields) => {
                buf.push(format!(
                    "{offset}{} {name} {{\n",
                    if context.as_class() {
                        "export abstract class"
                    } else {
                        "export interface"
                    },
                ));
                for field in fields {
                    if context.as_class() && field.is_method_constructor() {
                        continue;
                    }
                    if field.is_field_ignored() {
                        continue;
                    }
                    field.reference(natures, buf, offset.inc(), Some(name.to_owned()))?;
                    buf.push(";\n");
                }
                buf.push(format!("{offset}}}\n",));
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
        buf: &mut Writer,
        offset: Offset,
        parent: Option<String>,
    ) -> Result<(), E> {
        match self {
            Refered::Enum(name, _, _) => buf.push(name),
            Refered::EnumVariant(name, _, _, _) => buf.push(format!("{offset}{name}")),
            Refered::Field(name, context, nature, _) => {
                if let Nature::Composite(Composite::Func(_, _, _, _, constructor)) = nature.deref()
                {
                    if *constructor {
                        if context.as_class() {
                            return Ok(());
                        }
                        buf.push(format!("{offset}constructor"));
                    } else {
                        buf.push(format!(
                            "{offset}{}{}",
                            if context.as_class() {
                                "public abstract "
                            } else {
                                ""
                            },
                            context.rename_method(name)?,
                        ));
                    }
                    nature.reference(natures, buf, offset, parent)?;
                } else {
                    buf.push(format!("{offset}{}: ", context.rename_field(name)?));
                    if let Nature::Refered(Refered::Ref(ref_name, _)) = nature.deref() {
                        if let Some(generic) = context.get_generic(ref_name) {
                            generic.reference(natures, buf, offset, parent)?;
                            return Ok(());
                        }
                    }
                    nature.reference(natures, buf, offset, parent)?;
                }
            }
            Refered::Func(name, _context, _func) => buf.push(name),
            Refered::FuncArg(name, _context, _, _) => {
                return Err(E::Parsing(format!(
                    "Function argument {name} can be refered"
                )));
            }
            Refered::Struct(name, _, _) => buf.push(name),
            Refered::Ref(ref_name, _) => {
                if let Some(module) = parent.and_then(|p| natures.get_module_of(&p)) {
                    if let (Some(ref_mod), false) = (
                        natures.get_module_of(ref_name),
                        natures.exists_in_module(ref_name, &module),
                    ) {
                        buf.add_import(ref_name, ref_mod)?;
                    }
                }
                buf.push(ref_name)
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
