mod input;
mod target;

use crate::{config, error::E, nature::Nature};
use convert_case::{Case, Casing};
use input::Input;
use proc_macro_error::abort;
use std::{
    convert::{From, TryFrom},
    path::PathBuf
};
use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    AttrStyle, Attribute, Expr, Lit, Token, Meta,
};
pub use target::Target;

const ATTR_ALIAS: &str = "tslink";

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub inputs: Vec<Input>,
    pub targets: Vec<(Target, PathBuf)>,
    pub parent: Option<Box<Context>>,
    pub generics: Vec<Nature>,
}

impl Context {
    pub(self) fn new(inputs: Vec<Input>) -> Self {
        let targets: Vec<(Target, PathBuf)> = if let Some(Input::Target(targets)) =
            inputs.iter().find(|i| matches!(i, Input::Target(_)))
        {
            targets.clone()
        } else {
            vec![]
        };
        Self {
            inputs,
            targets,
            parent: None,
            generics: vec![],
        }
    }

    pub fn add_generics(&mut self, generics: Vec<Nature>) {
        self.generics.extend(generics.iter().cloned());
    }

    pub fn set_parent(&mut self, parent: Context) {
        self.parent = Some(Box::new(parent));
    }

    pub fn ignore_self(&self) -> bool {
        self.inputs.iter().any(|i| matches!(i, Input::IgnoreSelf))
    }

    pub fn is_ignored(&self, name: &String) -> bool {
        let a = if let Some(Input::Ignore(list)) =
            self.inputs.iter().find(|i| matches!(i, Input::Ignore(_)))
        {
            list.contains(name)
        } else {
            false
        };
        let b = if let Some(parent) = self.parent.as_ref() {
            parent.is_ignored(name)
        } else {
            false
        };
        a || b
    }

    pub fn ignored_list(&self) -> Vec<String> {
        if let Some(Input::Ignore(list)) =
            self.inputs.iter().find(|i| matches!(i, Input::Ignore(_)))
        {
            list.clone()
        } else {
            vec![]
        }
    }

    pub fn as_class(&self) -> bool {
        if self.inputs.iter().any(|i| matches!(i, Input::Class)) {
            return true;
        }
        if let Some(parent) = self.parent.as_ref() {
            parent.as_class()
        } else {
            false
        }
    }

    pub fn as_constructor(&self) -> bool {
        self.inputs.iter().any(|i| matches!(i, Input::Constructor))
    }

    pub fn exception_suppression(&self) -> Result<bool, E> {
        let config = config::get()?;
        Ok(config.exception_suppression || self.inputs.iter().any(|i| matches!(i, Input::ExceptionSuppression)))    
    } 

    fn rename(&self, origin: &str) -> Option<String> {
        if let Some(Input::Rename(name)) = self.inputs.iter().find(|i| matches!(i, Input::Rename(_))) {
            return Some(name.to_owned());
        }
        if self
            .inputs
            .iter()
            .any(|i| matches!(i, Input::SnakeCaseNaming))
        {
            Some(origin.to_case(Case::Camel))
        } else {
            None
        }
    }

    pub fn rename_field(&self, origin: &str) -> Result<String, E> {
        let config = config::get()?;
        self.rename(origin)
            .or_else(|| Some(config.rename_field(origin)))
            .ok_or(E::Other(String::from("Fail to get renaming settings")))
    }

    pub fn rename_method(&self, origin: &str) -> Result<String, E> {
        let config = config::get()?;
        self.rename(origin)
            .or_else(|| Some(config.rename_method(origin)))
            .ok_or(E::Other(String::from("Fail to get renaming settings")))
    }

    pub fn get_bound_args(&self) -> Vec<(String, String)> {
        if let Some(Input::Binding(arguments)) = self.inputs.iter().find(|i| matches!(i, Input::Binding(_))) {
            return arguments.iter().filter(|(n, _)| n != "result" && n != "error").cloned().collect();
        }
        vec![]
    }

    pub fn get_bound(&self, name: &str) -> Option<String> {
        if let Some(Input::Binding(arguments)) = self.inputs.iter().find(|i| matches!(i, Input::Binding(_))) {
            return arguments.iter().find_map(|(n, ref_name)| if n == name { Some(ref_name.to_owned())} else { None});
        }
        None
    }

    pub fn result_as_json(&self) -> Result<bool, E> {
        if let Some(Input::Binding(arguments)) = self.inputs.iter().find(|i| matches!(i, Input::Binding(_))) {
            if let Some((_, result_fmt)) = arguments.iter().find(|(n, _)| n == "result") {
                return if result_fmt.trim() == "json" {
                    Ok(true)
                } else {
                    Err(E::Parsing(String::from("Binding results to JSON string supported only for now. Use \"json\" keyword to activate")))
                };
            }
        }
        Ok(false)
    }

    pub fn error_as_json(&self) -> Result<bool, E> {
        if let Some(Input::Binding(arguments)) = self.inputs.iter().find(|i| matches!(i, Input::Binding(_))) {
            if let Some((_, result_fmt)) = arguments.iter().find(|(n, _)| n == "error") {
                return if result_fmt.trim() == "json" {
                    Ok(true)
                } else {
                    Err(E::Parsing(String::from("Binding results to JSON string supported only for now. Use \"json\" keyword to activate")))
                };
            }
        }
        Ok(false)
    }

    pub fn try_from_or_default(attrs: &[Attribute]) -> Result<Self, E> {
        for attr in attrs.iter() {
            if let (true, Some(ident)) = (
                matches!(attr.style, AttrStyle::Outer),
                attr.path().get_ident(),
            ) {
                if ident == ATTR_ALIAS {
                    if matches!(attr.meta, Meta::Path(_)) {
                        // No attributes
                        continue;
                    }
                    return attr
                        .parse_args_with(Context::parse)
                        .map_err(|e| E::PasringContext(e.to_string()));
                }
            }
        }
        Ok(Self::default())
    }
}

impl Parse for Context {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut inputs: Vec<Input> = vec![];
        let mut bindings: Vec<(String, String)> = vec![];
        Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .for_each(|expr| match expr {
                Expr::Assign(a) => {
                    let link = a.clone();
                    if let (Expr::Path(left), Expr::Lit(right)) = (*a.left, *a.right) {
                        if let (Some(left), Lit::Str(value)) = (left.path.get_ident(), right.lit) {
                            let value = value.value();
                            let mut input = match Input::try_from(left.to_string().as_ref()) {
                                Ok(input) => match input {
                                    Input::Rename(_) => Some(Input::Rename(value)),
                                    Input::Target(_) => {
                                        Some(Input::Target(
                                            value
                                                .split(';')
                                                .map(|s| {
                                                    let path = PathBuf::from(s.trim());
                                                    if let Some(ext) = path.extension() {
                                                        match Target::try_from(ext.to_string_lossy().to_string().as_str()) {
                                                            Ok(t) => (t, path),
                                                            Err(e) => {
                                                                abort!(
                                                                    left,
                                                                    format!("Unknown target: {s} ({e})")
                                                                )
                                                            }
                                                        }
                                                    } else {
                                                        abort!(
                                                            left,
                                                            format!("Cannot get extension of file: {s}; expecting .ts; .d.ts; .js")
                                                        )
                                                    }
                                                    
                                                })
                                                .collect::<Vec<(Target, PathBuf)>>()),
                                    )},
                                    Input::Ignore(_) => Some(Input::Ignore(
                                        value
                                            .split(';')
                                            .map(|s| s.trim().to_owned())
                                            .collect::<Vec<String>>(),
                                    )),
                                    _ => {
                                        abort!(
                                            left,
                                            "Attribute \"{}\" cannot be applied on this level",
                                            left
                                        )
                                    },
                                },
                                Err(_e) => {
                                    bindings.push((left.to_string(), value));
                                    None
                                },
                            };
                            if let Some(input) = input.take() {
                                inputs.push(input);
                            }
                        } else {
                            abort!(link, "Expecting expr like key = \"value as String\"");
                        }
                    } else {
                        abort!(link, "Expecting expr like key = \"value as String\"");
                    }
                }
                Expr::Path(p) => {
                    if let Some(ident) = p.path.get_ident() {
                        let input = match Input::try_from(ident.to_string().as_ref()) {
                            Ok(input) => match input {
                                Input::Ignore(_) => Input::IgnoreSelf,
                                Input::SnakeCaseNaming
                                | Input::Class
                                | Input::Interface
                                | Input::Constructor => input,
                                _ => abort!(
                                    p,
                                    "Attribute \"{}\" cannot be applied on this level",
                                    ident
                                ),
                            },
                            Err(e) => abort!(p, "Unknown attribute: {} ({})", ident, e),
                        };
                        inputs.push(input);
                    } else {
                        abort!(p, "Cannot extract identification");
                    }
                }
                _ => {
                    abort!(
                        expr,
                        "Expecting expr like [key = \"value as String\"] or [key]"
                    );
                }
            });
        if !bindings.is_empty() {
            inputs.push(Input::Binding(bindings));
        }
        Ok(Context::new(inputs))
    }
}
