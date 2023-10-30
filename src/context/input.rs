use crate::context::Target;
use std::{convert::TryFrom, fmt, path::PathBuf};

#[derive(Clone, Debug)]
pub enum Input {
    Ignore(Vec<String>),
    Rename(String),
    Target(Vec<(Target, PathBuf)>),
    IgnoreSelf,
    Constructor,
    SnakeCaseNaming,
    Interface,
    Binding(Vec<(String, String)>),
    Class,
}

impl TryFrom<&str> for Input {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if Input::Ignore(vec![]).to_string() == value {
            Ok(Input::Ignore(vec![]))
        } else if Input::IgnoreSelf.to_string() == value {
            Ok(Input::IgnoreSelf)
        } else if Input::Constructor.to_string() == value {
            Ok(Input::Constructor)
        } else if Input::SnakeCaseNaming.to_string() == value {
            Ok(Input::SnakeCaseNaming)
        } else if Input::Interface.to_string() == value {
            Ok(Input::Interface)
        } else if Input::Class.to_string() == value {
            Ok(Input::Class)
        } else if Input::Rename(String::new()).to_string() == value {
            Ok(Input::Rename(String::new()))
        } else if Input::Target(vec![]).to_string() == value {
            Ok(Input::Target(vec![]))
        } else {
            Err(format!("Unknown attribute \"{value}\""))
        }
    }
}

impl Input {
    pub fn is_ignore(input: &str) -> bool {
        Input::Ignore(vec![]).to_string() == input
    }
    pub fn is_ignore_self(input: &str) -> bool {
        Input::IgnoreSelf.to_string() == input
    }
    pub fn is_constructor(input: &str) -> bool {
        Input::Constructor.to_string() == input
    }
}

impl fmt::Display for Input {
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
                Self::Binding(_) => "**THIS_IS_RESERVED_KEY_WORD**",
                Self::Class => "class",
            }
        )
    }
}
