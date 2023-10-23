use proc_macro_error::abort;
use std::{collections::HashMap, fmt, path::PathBuf};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    AttrStyle, Attribute, Expr, Lit, Token,
};

use convert_case::{Case, Casing};

const ATTR_ALIAS: &str = "tslink";

pub trait ArgsWriter {
    fn get_args(&self) -> &Args;
}

#[derive(Debug, Clone)]
pub enum Target {
    Ts,
    DTs,
    Js,
}

impl Target {
    fn from(value: &str) -> Option<Self> {
        if value == &Target::Ts.to_string() {
            Some(Target::Ts)
        } else if value == &Target::DTs.to_string() {
            Some(Target::DTs)
        } else if value == &Target::Js.to_string() {
            Some(Target::Js)
        } else {
            None
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ts => ".ts",
                Self::DTs => ".d.ts",
                Self::Js => ".js",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Arg {
    Ignore(Vec<String>),
    IgnoreSelf,
    Rename(String),
    Constructor,
    SnakeCaseNaming,
    Interface,
    Target(Vec<Target>),
    Class,
    Path(String),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ignore(_) => "ignore",
                Self::IgnoreSelf => "ignore_self",
                Self::Constructor => "constructor",
                Self::Rename(_) => "rename",
                Self::SnakeCaseNaming => "snake_case_naming",
                Self::Interface => "interface",
                Self::Target(_) => "target",
                Self::Class => "class",
                Self::Path(_) => "path",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    args: HashMap<String, Arg>,
    pub targets: Vec<Target>,
}

impl Args {
    pub(self) fn new(args: HashMap<String, Arg>) -> Self {
        let targets: Vec<Target> =
            if let Some(Arg::Target(targets)) = args.get(&Arg::Target(vec![]).to_string()) {
                targets.clone()
            } else {
                vec![]
            };
        Self { args, targets }
    }

    pub fn ignore_self(&self) -> bool {
        self.args.contains_key(&Arg::IgnoreSelf.to_string())
    }

    pub fn is_ignored(&self, name: &String) -> bool {
        if let Some(Arg::Ignore(list)) = self.args.get(&Arg::Ignore(vec![]).to_string()) {
            list.contains(name)
        } else {
            false
        }
    }

    pub fn as_interface(&self) -> bool {
        self.args.contains_key(&Arg::Interface.to_string())
    }

    pub fn as_class(&self) -> bool {
        self.args.contains_key(&Arg::Class.to_string())
    }

    pub fn set_as_class(&self) -> Self {
        let mut clonned = self.clone();
        if !clonned.as_class() {
            clonned.args.insert(Arg::Class.to_string(), Arg::Class);
        }
        clonned
    }

    pub fn rename(&self, origin: &str) -> Option<String> {
        if let Some(arg) = self.args.get(&Arg::Rename(String::new()).to_string()) {
            if let Arg::Rename(name) = arg {
                return Some(name.to_owned());
            }
        }
        if let Some(_arg) = self.args.get(&Arg::SnakeCaseNaming.to_string()) {
            Some(origin.to_case(Case::Camel));
        }
        None
    }

    pub fn path(&self) -> Option<PathBuf> {
        if let Some(arg) = self.args.get(&Arg::Path(String::new()).to_string()) {
            if let Arg::Path(path) = arg {
                return Some(PathBuf::from(path));
            }
        }
        None
    }

    pub fn from_attrs(attrs: &Vec<Attribute>) -> Option<Self> {
        for attr in attrs.iter() {
            if matches!(attr.style, AttrStyle::Outer) {
                if let Some(ident) = attr.path().get_ident() {
                    if ident == ATTR_ALIAS {
                        return attr
                            .parse_args_with(Args::parse)
                            .map_or(None, |args| Some(args));
                    }
                }
            }
        }
        None
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = HashMap::new();
        Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .for_each(|expr| match expr {
                Expr::Assign(a) => {
                    let link = a.clone();
                    if let (Expr::Path(left), Expr::Lit(right)) = (*a.left, *a.right) {
                        if let (Some(left), Lit::Str(value)) = (left.path.get_ident(), right.lit) {
                            let value: String = value.value();
                            if left == &Arg::Rename(String::new()).to_string() {
                                args.insert(
                                    Arg::Rename(String::new()).to_string(),
                                    Arg::Rename(value.to_string()),
                                );
                            } else if left == &Arg::Path(String::new()).to_string() {
                                args.insert(
                                    Arg::Path(String::new()).to_string(),
                                    Arg::Path(value.to_string()),
                                );
                            } else if left == &Arg::Target(vec![]).to_string() {
                                let targets = value
                                    .split(';')
                                    .map(|s| {
                                        if let Some(target) = Target::from(s.trim()) {
                                            target
                                        } else {
                                            abort!(left, format!("Unknown target: {s}"));
                                        }
                                    })
                                    .collect::<Vec<Target>>();
                                args.insert(Arg::Target(vec![]).to_string(), Arg::Target(targets));
                            } else if left == &Arg::Ignore(vec![]).to_string() {
                                let names = value
                                    .split(';')
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<String>>();
                                args.insert(Arg::Ignore(vec![]).to_string(), Arg::Ignore(names));
                            } else {
                                abort!(left, "Unknown attribute: {}", left);
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
                        if ident.to_string() == Arg::Ignore(vec![]).to_string() {
                            args.insert(Arg::IgnoreSelf.to_string(), Arg::IgnoreSelf);
                        } else if ident.to_string() == Arg::SnakeCaseNaming.to_string() {
                            args.insert(Arg::SnakeCaseNaming.to_string(), Arg::SnakeCaseNaming);
                        } else if ident.to_string() == Arg::Class.to_string() {
                            args.insert(Arg::Class.to_string(), Arg::Class);
                        } else if ident.to_string() == Arg::Interface.to_string() {
                            args.insert(Arg::Interface.to_string(), Arg::Interface);
                        } else if ident.to_string() == Arg::Constructor.to_string() {
                            args.insert(Arg::Constructor.to_string(), Arg::Constructor);
                        } else {
                            abort!(p, "Unknown attribute: {}", ident);
                        }
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
        Ok(Args::new(args))
    }
}
