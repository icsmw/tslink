use super::Interpreter;
use crate::{
    config::cfg::EnumRepresentation,
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
            Refered::Enum(name, _context, variants, repres) => {
                if let Some(module) = natures.get_module_of(name) {
                    if natures.exists_in_module(name, &module) {
                        buf.add_export(name, &module)?;
                    }
                }
                let flat = Refered::is_flat_varians(variants)?;
                if flat {
                    buf.push(format!("{offset}export enum {name} {{\n",));
                    for variant in variants.iter() {
                        variant.declaration(natures, buf, offset.inc(), Some(name.to_owned()))?;
                        buf.push(",\n");
                    }
                    buf.push(format!("{offset}}}\n",));
                } else {
                    match repres {
                        EnumRepresentation::Flat => {
                            buf.push(format!("{offset}export interface {name} {{\n",));
                            for variant in variants.iter() {
                                variant.declaration(
                                    natures,
                                    buf,
                                    offset.inc(),
                                    Some(name.to_owned()),
                                )?;
                                buf.push(";\n");
                            }
                            buf.push(format!("{offset}}}\n",));
                        }
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.push(format!("{offset}export type {name} =\n",));
                            for (n, variant) in variants.iter().enumerate() {
                                variant.declaration(
                                    natures,
                                    buf,
                                    offset.inc(),
                                    Some(name.to_owned()),
                                )?;
                                buf.push(if n == variants.len() - 1 { "" } else { " |\n" });
                            }
                            buf.push(format!("{offset};\n",));
                        }
                    }
                }
            }
            Refered::EnumVariant(name, _context, fields, flat, repres) => {
                let named = fields
                    .iter()
                    .any(|f| matches!(f, Nature::Refered(Refered::Field(..))));
                if fields.is_empty() {
                    if *flat {
                        buf.push(format!("{offset}{name}"));
                    } else {
                        match repres {
                            EnumRepresentation::Flat => {
                                buf.push(format!("{offset}{name}?: null"));
                            }
                            EnumRepresentation::Union => {
                                buf.push(format!("{offset}{{ {name}: null }}"));
                            }
                            EnumRepresentation::DiscriminatedUnion => {
                                buf.push(format!("{offset}\"{name}\""));
                            }
                        }
                    }
                } else if fields.len() == 1 {
                    match repres {
                        EnumRepresentation::Flat => {
                            buf.push(format!("{offset}{name}?: "));
                        }
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.push(format!(
                                "{offset}{{{}{name}: ",
                                if named {
                                    format!("\n{}", offset.inc())
                                } else {
                                    " ".to_owned()
                                }
                            ));
                        }
                    }
                    if named {
                        buf.push("{\n");
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
                            parent,
                        )?;
                    if named {
                        buf.push(format!(
                            "\n{}}}",
                            match repres {
                                EnumRepresentation::Flat => offset.clone(),
                                EnumRepresentation::Union
                                | EnumRepresentation::DiscriminatedUnion => {
                                    offset.inc()
                                }
                            }
                        ));
                    }
                    match repres {
                        EnumRepresentation::Flat => {}
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.push(if named {
                                format!("\n{}}}", offset)
                            } else {
                                " }".to_owned()
                            });
                        }
                    }
                } else {
                    match repres {
                        EnumRepresentation::Flat => {
                            buf.push(format!(
                                "{offset}{name}?: {}",
                                if named { "{\n" } else { "[" }
                            ));
                        }
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.push(format!(
                                "{offset}{{{}{name}: {}",
                                if named {
                                    format!("\n{}", offset.inc())
                                } else {
                                    " ".to_owned()
                                },
                                if named { "{\n" } else { "[" }
                            ));
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
                            parent.clone(),
                        )?;
                        if i < fields.len() - 1 {
                            buf.push(if named { ";\n" } else { "," });
                        }
                    }
                    buf.push(if named {
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
                    });
                    match repres {
                        EnumRepresentation::Flat => {}
                        EnumRepresentation::Union | EnumRepresentation::DiscriminatedUnion => {
                            buf.push(if named {
                                format!("\n{}}}", offset)
                            } else {
                                " }".to_owned()
                            });
                        }
                    }
                }
            }
            Refered::Field(name, context, ..) => {
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
                if let Some(module) = natures.get_module_of(name) {
                    if natures.exists_in_module(name, &module) {
                        buf.add_export(name, &module)?;
                    }
                }
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
            Refered::TupleStruct(name, _context, field) => {
                buf.push(format!("{offset}export type {name} = ",));
                if let Some(module) = natures.get_module_of(name) {
                    if natures.exists_in_module(name, &module) {
                        buf.add_export(name, &module)?;
                    }
                }
                if let Some(field) = field {
                    field.reference(natures, buf, Offset::new(), Some(name.to_owned()))?;
                } else {
                    buf.push("undefined");
                }
                buf.push(";\n");
            }
            Refered::Ref(ref_name, ..) => {
                return Err(E::Parsing(format!("Reference {ref_name} can be declared")));
            }
            Refered::Generic(alias, ..) => {
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
            Refered::Enum(name, ..) => buf.push(name),
            Refered::EnumVariant(name, ..) => buf.push(format!("{offset}{name}")),
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
                    if name.is_empty() {
                        // This is name of unnamed field of TupleStruct
                        buf.push(&(context.rename_field(name)?));
                    } else {
                        buf.push(format!("{offset}{}: ", context.rename_field(name)?));
                        if let Nature::Refered(Refered::Ref(ref_name, _)) = nature.deref() {
                            if let Some(generic) = context.get_generic(ref_name) {
                                generic.reference(natures, buf, offset, parent)?;
                                return Ok(());
                            }
                        }
                    }
                    nature.reference(natures, buf, offset, parent)?;
                }
            }
            Refered::Func(name, ..) => buf.push(name),
            Refered::FuncArg(name, ..) => {
                return Err(E::Parsing(format!(
                    "Function argument {name} can be refered"
                )));
            }
            Refered::Struct(name, ..) => buf.push(name),
            Refered::TupleStruct(name, ..) => buf.push(name),
            Refered::Ref(ref_name, ..) => {
                if let Some(module) = parent.and_then(|p| natures.get_module_of(&p)) {
                    if let (Some(ref_mod), false) = (
                        natures.get_module_of(ref_name),
                        natures.exists_in_module(ref_name, &module),
                    ) {
                        buf.add_import(ref_name, ref_mod)?;
                    }
                    if natures.exists_in_module(ref_name, &module) {
                        buf.add_export(ref_name, &module)?;
                    }
                }
                buf.push(ref_name)
            }
            Refered::Generic(alias, ..) => {
                return Err(E::Parsing(format!(
                    "Generic type cannot be rendered out of context; type alias = {alias}"
                )))
            }
        }
        Ok(())
    }
}
