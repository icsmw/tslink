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

fn wrap_output(call_exp: String, result_as_json: bool) -> String {
    format!(
        "{}{call_exp}{}",
        if result_as_json { "JSON.parse(" } else { "" },
        if result_as_json { ")" } else { "" }
    )
}

fn wrap_err(error_as_json: bool, asyncness: bool, exception_suppression: bool) -> String {
    let open = if asyncness { "Promise.reject(" } else { "" };
    let close = if asyncness { ")" } else { "" };
    let returning = if exception_suppression || asyncness {
        "return"
    } else {
        "throw"
    };
    let parsing_err_block = if exception_suppression || asyncness {
        format!("try {{
                    const err = new Error(`Function/method returns error;`);
                    err.err = JSON.parse(e);
                    return {open}err{close};
                }} catch(err_parsing) {{
                    return {open}new Error(`Function/method returns error; fail to parse error; origin error: ${{e}}; error: ${{err_parsing}}`){close};
                }}")
    } else {
        "const err = new Error(`Function/method returns error;`);
                try {{
                    err.err = JSON.parse(e);
                }} catch(err_parsing) {{
                    throw new Error(`Function/method returns error; fail to parse error; origin error: ${{e}}; error: ${{err_parsing}}`);
                }}
                throw err;".to_string()
    };
    if error_as_json {
        format!("if (e instanceof Error) {{
                {returning} {open}e{close};
            }}
            if (typeof e === 'string') {{
                {parsing_err_block}
            }} else {{
                const err = new Error(`Function/method returns error; property [err] = ${{typeof e === 'object' && e !== null ? JSON.stringify(e) : e}}`);
                err.err = e;
                {returning} {open}err{close};
            }}")
    } else {
        format!("if (e instanceof Error) {{
                {returning} {open}e{close};
            }}
            if (typeof e === 'string') {{
                {returning} {open}new Error(e){close};
            }} else {{
                const err = new Error(`Function/method returns error; property [err] = ${{typeof e === 'object' && e !== null ? JSON.stringify(e) : e}}`);
                err.err = e;
                {returning} {open}err{close};
            }}")
    }
}
fn fn_body(
    call_exp: String,
    exception_suppression: bool,
    result_as_json: bool,
    error_as_json: bool,
    asyncness: bool,
) -> String {
    let error_handeling_block = wrap_err(error_as_json, asyncness, exception_suppression);
    if asyncness {
        format!(
            "return {call_exp}.then((result) => {{
                try {{
                    return Promise.resolve({});
                }} catch (e) {{
                    return Promise.reject(e);
                }}
            }}).catch((e) => {{
                {error_handeling_block}
            }});",
            wrap_output("result".to_string(), result_as_json)
        )
    } else if exception_suppression || error_as_json {
        format!(
            "try {{
            return {};
        }} catch(e) {{
            {error_handeling_block}
        }}",
            wrap_output(call_exp, result_as_json)
        )
    } else {
        format!("return {};", wrap_output(call_exp, result_as_json))
    }
}

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
                        if let Nature::Refered(Refered::Field(name, _, nature, _)) = field {
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
                        if let Nature::Refered(Refered::Field(_, context, nature, _)) = field {
                            if let Nature::Composite(Composite::Func(args, _, _, true)) = &**nature
                            {
                                let bound = context.get_bound_args();
                                if bound.is_empty() {
                                    let args = Natures::get_fn_args_names(args).join(", ");
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
                                    let args = Natures::get_fn_args_names(args);
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
                        if let Nature::Refered(Refered::Field(name, context, nature, _)) = field {
                            if let Nature::Composite(Composite::Func(
                                args,
                                _,
                                asyncness,
                                constructor,
                            )) = nature.deref()
                            {
                                if *constructor {
                                    continue;
                                }
                                let name = context.rename_field(name)?;
                                let bound = context.get_bound_args();
                                let args = Natures::get_fn_args_names(args);
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
                                buf.write_all(
                                    format!(
                                        "
    {name}({}) {{
        {}      
    }}",
                                        args.join(", "),
                                        fn_body(
                                            call_exp,
                                            context.exception_suppression()?,
                                            context.result_as_json()?,
                                            context.error_as_json()?,
                                            *asyncness
                                        )
                                    )
                                    .as_bytes(),
                                )?;
                            }
                        }
                    }
                    buf.write_all("\n}".as_bytes())?;
                    buf.write_all(format!("\nexports.{struct_name} = {alias};\n").as_bytes())?;
                }
            }
            Refered::Enum(name, _context, variants) => {
                buf.write_all(format!("{}exports.{name} = Object.freeze({{\n", offset).as_bytes())?;
                for (i, variant) in variants.iter().enumerate() {
                    if let Nature::Refered(Refered::EnumVariant(name, _, _, _)) = variant {
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
function {alias}({}) {{
    {}
}}",
                        args.join(", "),
                        fn_body(
                            call_exp,
                            context.exception_suppression()?,
                            context.result_as_json()?,
                            context.error_as_json()?,
                            nature.is_fn_async()?,
                        )
                    )
                    .as_bytes(),
                )?;
                buf.write_all(format!("\nexports.{fn_name} = {alias};\n").as_bytes())?;
            }
            _ => {
                return Err(E::Parsing(
                    "Given nature cannot be declared for JS".to_string(),
                ));
            }
        }
        Ok(())
    }
}
