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
        _natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Refered::Struct(struct_name, _, fields) => {
                buf.write_all(
                    format!("\nconst {{ {struct_name} }} = nativeModuleRef;").as_bytes(),
                )?;
                if Natures::is_any_bound(fields) {
                    buf.write_all(format!("\nexports.{struct_name} = {struct_name};").as_bytes())?;
                } else {
                    let alias = format!("$${struct_name}");
                    buf.write_all(
                        format!(
                            "\nclass {alias} {{
    #_origin;"
                        )
                        .as_bytes(),
                    )?;
                    // Render fields
                    for field in fields.iter() {
                        if let Nature::Refered(Refered::Field(name, _, nature, _)) = field.deref() {
                            if !matches!(
                                nature.deref(),
                                Nature::Composite(Composite::Func(_, _, _, _))
                            ) {
                                buf.write_all(
                                    format!(
                                        "\nget {name}() {{
        return this.#_origin.{name};
    }}
    set {name}(v) {{
        this.#_origin.{name} = v;
    }}"
                                    )
                                    .as_bytes(),
                                )?;
                            }
                        }
                    }
                    // Render constructor
                    let mut constuctor_rendered = false;
                    for field in fields.iter() {
                        if let Nature::Refered(Refered::Field(name, context, nature, _)) = &**field
                        {
                            if let Nature::Composite(Composite::Func(args, _, _, true)) = &**nature
                            {
                                let bound = context.get_bound_args();
                                if bound.is_empty() {
                                    let args = Natures::get_fn_args_names(&args).join(", ");
                                    buf.write_all(
                                        format!(
                                            "
    constructor({args}) {{
        this.#_origin = new {struct_name}({args});
    }}"
                                        )
                                        .as_bytes(),
                                    )?;
                                } else {
                                    let args = Natures::get_fn_args_names(&args);
                                    buf.write_all(
                                        format!(
                                            "\n
    constructor({}) {{
        this.#_origin = new {struct_name}({});
    }}",
                                            args.join(", "),
                                            args.iter()
                                                .map(|a| {
                                                    if bound.iter().any(|(name, _)| name == a) {
                                                        format!("JSON.stringify({a})")
                                                    } else {
                                                        a.to_owned()
                                                    }
                                                })
                                                .collect::<Vec<String>>()
                                                .join(", ")
                                        )
                                        .as_bytes(),
                                    )?;
                                }
                                constuctor_rendered = true;
                            }
                        }
                    }
                    if !constuctor_rendered {
                        buf.write_all(
                            format!(
                                "\n
    constructor() {{
        this.#_origin = new {struct_name}();
    }}"
                            )
                            .as_bytes(),
                        )?;
                    }
                    // Render methods
                    for field in fields.iter() {
                        if let Nature::Refered(Refered::Field(name, context, nature, _)) =
                            field.deref()
                        {
                            if let Nature::Composite(Composite::Func(args, _, _, constructor)) =
                                nature.deref()
                            {
                                if *constructor {
                                    continue;
                                }
                                let name = context.rename_field(name)?;
                                let bound = context.get_bound_args();
                                let args = Natures::get_fn_args_names(&args);
                                let call_exp = if bound.is_empty() {
                                    format!("this.#_origin.{name}({})", args.join(", "))
                                } else {
                                    format!(
                                        "this.#_origin.{name}({})",
                                        args.iter()
                                            .map(|a| {
                                                if bound.iter().any(|(name, _)| name == a) {
                                                    format!("JSON.stringify({a})")
                                                } else {
                                                    a.to_owned()
                                                }
                                            })
                                            .collect::<Vec<String>>()
                                            .join(", ")
                                    )
                                };
                                let call_exp = if !context.result_as_json()? {
                                    call_exp
                                } else {
                                    format!("JSON.parse({call_exp})")
                                };
                                buf.write_all(
                                    format!(
                                        "
    {name}({}){{
        return {call_exp};
    }}",
                                        args.join(", ")
                                    )
                                    .as_bytes(),
                                )?;
                            }
                        }
                    }
                    buf.write_all(format!("\n}}").as_bytes())?;
                    buf.write_all(format!("\nexports.{struct_name} = {alias};\n").as_bytes())?;
                }
            }
            Refered::Enum(name, _context, variants) => {
                buf.write_all(format!("{}exports.{name} = Object.freeze({{\n", offset).as_bytes())?;
                for (i, variant) in variants.iter().enumerate() {
                    if let Nature::Refered(Refered::EnumVariant(name, _, _, _)) = variant.deref() {
                        buf.write_all(
                            format!("{}{name}: {i}, \"{i}\": \"{name}\",\n", offset.inc())
                                .as_bytes(),
                        )?;
                    } else {
                        return Err(E::Parsing(String::from(
                            "Given nature isn't Enum's variant",
                        )));
                    }
                }
                buf.write_all(format!("{}}});\n", offset).as_bytes())?;
            }
            Refered::Func(fn_name, context, nature) => {
                let fn_name = context.rename_method(fn_name)?;
                let bound = context.get_bound_args();
                let json_res = context.result_as_json()?;
                buf.write_all(format!("\nconst {{ {fn_name} }} = nativeModuleRef;").as_bytes())?;
                if bound.is_empty() && !json_res {
                    buf.write_all(format!("\nexports.{fn_name} = {fn_name};").as_bytes())?;
                    return Ok(());
                }
                let args = nature.get_fn_args_names()?;
                let alias = format!("$${fn_name}");
                let call_exp = if bound.is_empty() {
                    format!("{fn_name}({})", args.join(", "))
                } else {
                    format!(
                        "{fn_name}({})",
                        args.iter()
                            .map(|a| {
                                if bound.iter().any(|(name, _)| name == a) {
                                    format!("JSON.stringify({a})")
                                } else {
                                    a.to_owned()
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                };
                let call_exp = if !context.result_as_json()? {
                    call_exp
                } else {
                    format!("JSON.parse({call_exp})")
                };
                buf.write_all(
                    format!(
                        "
function {alias}({}){{
    return {call_exp};
}}",
                        args.join(", ")
                    )
                    .as_bytes(),
                )?;
                buf.write_all(format!("\nexports.{fn_name} = {alias};\n").as_bytes())?;
            }
            _ => {
                return Err(E::Parsing(format!(
                    "Given nature cannot be declared for JS"
                )));
            }
        }
        Ok(())
    }
}
