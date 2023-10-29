mod input;
mod target;

use crate::{config, error::E};
use convert_case::{Case, Casing};
use input::Input;
use proc_macro_error::abort;
use std::{
    convert::{From, TryFrom},
    path::PathBuf,
};
use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    AttrStyle, Attribute, Expr, Lit, Token,
};
use target::Target;

const ATTR_ALIAS: &str = "tslink";

#[derive(Clone, Debug)]
pub struct Context {
    pub inputs: Vec<Input>,
    pub targets: Vec<Target>,
    pub parent: Option<Box<Context>>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            inputs: vec![],
            targets: vec![],
            parent: None,
        }
    }
}

impl Context {
    pub(self) fn new(inputs: Vec<Input>) -> Self {
        let targets: Vec<Target> = if let Some(Input::Target(targets)) =
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
        }
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

    pub fn as_interface(&self) -> bool {
        self.inputs.iter().any(|i| matches!(i, Input::Interface))
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

    pub fn set_as_class(&self) -> Self {
        let mut clonned = self.clone();
        if !clonned.as_class() {
            clonned.inputs.push(Input::Class);
        }
        clonned
    }

    fn rename(&self, origin: &str) -> Option<String> {
        if let Some(arg) = self.inputs.iter().find(|i| matches!(i, Input::Rename(_))) {
            if let Input::Rename(name) = arg {
                return Some(name.to_owned());
            }
        }
        if let Some(_arg) = self
            .inputs
            .iter()
            .find(|i| matches!(i, Input::SnakeCaseNaming))
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

    pub fn path(&self) -> Option<PathBuf> {
        if let Some(arg) = self.inputs.iter().find(|i| matches!(i, Input::Path(_))) {
            if let Input::Path(path) = arg {
                return Some(PathBuf::from(path));
            }
        }
        None
    }

    pub fn try_from_or_default(attrs: &Vec<Attribute>) -> Result<Self, E> {
        for attr in attrs.iter() {
            if let (true, Some(ident)) = (
                matches!(attr.style, AttrStyle::Outer),
                attr.path().get_ident(),
            ) {
                if ident == ATTR_ALIAS {
                    return Ok(attr
                        .parse_args_with(Context::parse)
                        .map_err(|e| E::PasringContext(e.to_string()))?);
                }
            }
        }
        Ok(Self::default())
    }
}

impl Parse for Context {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut inputs: Vec<Input> = vec![];
        Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .for_each(|expr| match expr {
                Expr::Assign(a) => {
                    let link = a.clone();
                    if let (Expr::Path(left), Expr::Lit(right)) = (*a.left, *a.right) {
                        if let (Some(left), Lit::Str(value)) = (left.path.get_ident(), right.lit) {
                            let value = value.value();
                            let input = match Input::try_from(left.to_string().as_ref()) {
                                Ok(input) => match input {
                                    Input::Rename(_) => Input::Rename(value),
                                    Input::Path(_) => Input::Path(value),
                                    Input::Target(_) => Input::Target(
                                        value
                                            .split(';')
                                            .map(|s| match Target::try_from(s.trim()) {
                                                Ok(t) => t,
                                                Err(e) => {
                                                    abort!(left, format!("Unknown target: {s}"))
                                                }
                                            })
                                            .collect::<Vec<Target>>(),
                                    ),
                                    Input::Ignore(_) => Input::Ignore(
                                        value
                                            .split(';')
                                            .map(|s| s.to_owned())
                                            .collect::<Vec<String>>(),
                                    ),
                                    _ => abort!(
                                        left,
                                        "Attribute \"{}\" cannot be applied on this level",
                                        left
                                    ),
                                },
                                Err(e) => abort!(left, "Unknown attribute: {}", left),
                            };
                            inputs.push(input);
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
                            Err(e) => abort!(p, "Unknown attribute: {}", ident),
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
        Ok(Context::new(inputs))
    }
}