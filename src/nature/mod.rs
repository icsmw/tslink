mod defs;
mod fabric;
mod generic;
mod origin;
mod types;

pub use defs::Composite;
pub use defs::Primitive;
pub use defs::Refered;
pub use fabric::{TypeAsString, TypeTokenStream, VariableTokenStream};
pub use generic::ExtractGenerics;
pub use origin::OriginType;
pub use types::Extract;

use crate::{context::Context, error::E};
use std::{collections::HashMap, ops::Deref};

pub struct NatureDef {
    pub nature: Nature,
    pub module: Option<String>,
}

impl NatureDef {
    pub fn new(nature: Nature, module: Option<String>) -> Self {
        Self { nature, module }
    }
    pub fn get_mut(&mut self) -> &mut Nature {
        &mut self.nature
    }
    pub fn get(&self) -> &Nature {
        &self.nature
    }
    pub fn extract(&self) -> Nature {
        self.nature.clone()
    }
}
pub struct Natures(HashMap<String, NatureDef>);

impl Natures {
    pub fn new() -> Self {
        Natures(HashMap::new())
    }
    pub fn is_any_bound(natures: &[Nature]) -> bool {
        for nature in natures.iter() {
            if let Nature::Refered(Refered::Field(_, _, _, binding)) = nature {
                if binding.is_some() {
                    return true;
                }
            }
        }
        false
    }
    pub fn get_fn_args_names(args: &[Nature]) -> Vec<String> {
        args.iter()
            .filter_map(|arg| {
                if let Nature::Refered(Refered::FuncArg(name, _, _, _)) = arg {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    }
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
    pub fn insert(&mut self, name: &str, nature: Nature, module: Option<String>) -> Result<(), E> {
        if self.contains(name) {
            Err(E::EntityExist(name.to_owned()))
        } else {
            let _ = self
                .0
                .insert(name.to_owned(), NatureDef::new(nature, module));
            Ok(())
        }
    }
    pub fn get_module_of(&self, name: &str) -> Option<String> {
        self.0.get(name).and_then(|n| n.module.clone())
    }
    pub fn exists_in_module(&self, name: &str, module: &str) -> bool {
        self.0
            .get(name)
            .map(|n| n.module.as_ref().map(|m| m == module).unwrap_or_default())
            .unwrap_or_default()
    }
    pub fn get_mut(
        &mut self,
        name: &str,
        default_nature: Option<Nature>,
        default_module: Option<String>,
    ) -> Option<&mut Nature> {
        if let (exists, Some(default_nature)) = (self.0.contains_key(name), default_nature) {
            if !exists {
                let _ = self.0.insert(
                    name.to_owned(),
                    NatureDef::new(default_nature, default_module),
                );
            }
        }
        self.0.get_mut(name).map(|n| n.get_mut())
    }

    pub fn filter(&self, filter: fn(&Nature) -> bool) -> Vec<Nature> {
        let mut natures: Vec<Nature> = vec![];
        for (_, n) in self.0.iter() {
            if filter(n.get()) {
                natures.push(n.extract());
            }
        }
        natures
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Nature)> {
        self.0.iter().map(|(k, n)| (k, n.get()))
    }
}

#[derive(Clone, Debug)]
pub enum Nature {
    Primitive(Primitive),
    Refered(Refered),
    Composite(Composite),
}

impl Nature {
    pub fn get_fn_args_names(&self) -> Result<Vec<String>, E> {
        if let Nature::Composite(Composite::Func(_, args, _, _, _)) = self {
            Ok(Natures::get_fn_args_names(args))
        } else {
            Err(E::Parsing("Fail to find arguments of function".to_string()))
        }
    }

    pub fn is_fn_async(&self) -> Result<bool, E> {
        if let Nature::Composite(Composite::Func(_, _, _, asyncness, _)) = self {
            Ok(*asyncness)
        } else {
            Err(E::Parsing("Fail to find function".to_string()))
        }
    }

    pub fn bind(&mut self, nature: Nature) -> Result<(), E> {
        match self {
            Self::Primitive(_) => Err(E::Parsing(String::from("Primitive type cannot be bound"))),
            Self::Refered(re) => match re {
                Refered::Struct(_, _, natures) => {
                    natures.push(nature);
                    Ok(())
                }
                Refered::Enum(_, _, natures) => {
                    natures.push(nature);
                    Ok(())
                }
                Refered::EnumVariant(_, _, natures, _) => {
                    natures.push(nature);
                    Ok(())
                }
                _ => Err(E::NotSupported("".to_owned())),
            },
            Self::Composite(othr) => match othr {
                Composite::HashMap(_, k, v) => {
                    if k.is_none() {
                        if let Self::Primitive(p) = nature {
                            let _ = k.insert(p);
                            Ok(())
                        } else {
                            Err(E::Parsing(String::from(
                                "HashMap can use as key only Primitive type",
                            )))
                        }
                    } else if v.is_none() {
                        let _ = v.insert(Box::new(nature));
                        Ok(())
                    } else {
                        Err(E::Parsing(String::from(
                            "HashMap entity already has been bound",
                        )))
                    }
                }
                Composite::Option(_, o) => {
                    if o.is_some() {
                        Err(E::Parsing(String::from(
                            "Option entity already has been bound",
                        )))
                    } else {
                        let _ = o.insert(Box::new(nature));
                        Ok(())
                    }
                }
                Composite::Result(_, r, e, _, _) => {
                    if r.is_some() && e.is_some() {
                        Err(E::Parsing(String::from(
                            "Result entity already has been bound",
                        )))
                    } else if r.is_none() {
                        let _ = r.insert(Box::new(nature));

                        Ok(())
                    } else {
                        let _ = e.insert(Box::new(nature));
                        Ok(())
                    }
                }
                Composite::Tuple(_, tys) => {
                    tys.push(nature);
                    Ok(())
                }
                Composite::Vec(_, v) => {
                    if v.is_some() {
                        Err(E::Parsing(String::from(
                            "Vec entity already has been bound",
                        )))
                    } else {
                        let _ = v.insert(Box::new(nature));
                        Ok(())
                    }
                }
                _ => Err(E::NotSupported("".to_owned())),
            },
        }
    }

    pub fn is_method_constructor(&self) -> bool {
        if let Nature::Refered(Refered::Field(_, _, nature, _)) = self {
            if let Nature::Composite(Composite::Func(_, _, _, _, constructor)) = nature.deref() {
                return *constructor;
            }
        }
        false
    }

    pub fn is_field_ignored(&self) -> bool {
        if let Nature::Refered(Refered::Field(name, context, _, _)) = self {
            context.is_ignored(name)
        } else {
            false
        }
    }

    pub fn check_ignored_fields(&self) -> Result<(), E> {
        if let Nature::Refered(Refered::Struct(name, context, fields)) = self {
            let ignored = context.ignored_list();
            if ignored.is_empty() {
                return Ok(());
            }
            let existed = fields
                .iter()
                .filter_map(|f| {
                    if let Nature::Refered(Refered::Field(name, _, _, _)) = f {
                        Some(name.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();
            for n in ignored {
                if !existed.iter().any(|name| name == &n) {
                    return Err(E::Parsing(format!(
                        "Field in ignored list \"{n}\" isn't found in struct \"{name}\""
                    )));
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn get_context(&self) -> Result<&Context, E> {
        Ok(match self {
            Self::Primitive(_) => Err(E::Parsing(String::from("Primitives do not have context")))?,
            Self::Composite(_composite) => {
                Err(E::Parsing(String::from("Composite do not have context")))?
            }
            Self::Refered(refered) => match refered {
                Refered::Enum(_, context, _) => context,
                Refered::EnumVariant(_, context, _, _) => context,
                Refered::Field(_, context, _, _) => context,
                Refered::Func(_, context, _) => context,
                Refered::FuncArg(_, context, _, _) => context,
                Refered::Struct(_, context, _) => context,
                Refered::Ref(_, _) => {
                    Err(E::Parsing(String::from("Reference do not have context")))?
                }
                Refered::Generic(_, _) => {
                    Err(E::Parsing(String::from("Generic do not have context")))?
                }
            },
        })
    }
}
