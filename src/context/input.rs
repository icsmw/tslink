use crate::context::Target;
use std::{convert::TryFrom, fmt, path::PathBuf};

/// Describes macro-level attributes that control TypeScript code generation behavior.
///
/// Each variant corresponds to a specific directive or modifier used in procedural macros
/// to guide how Rust types and functions should be translated into TypeScript declarations or JavaScript wrappers.
#[derive(Clone, Debug)]
pub enum Input {
    /// Specifies a list of field names to ignore during code generation.
    Ignore(Vec<String>),

    /// Renames the current item to the specified string in the generated output.
    Rename(String),

    /// A list of `(Target, PathBuf)` pairs indicating where `.ts` files should be written.
    Target(Vec<(Target, PathBuf)>),

    /// Specifies the TypeScript module name to be used in the generated output.
    Module(String),

    /// Instructs the generator to ignore this specific field or method.
    IgnoreSelf,

    /// Marks a function as a constructor (used for struct instantiation).
    Constructor,

    /// Applies snake_case renaming to the given field or method.
    SnakeCaseNaming,

    /// Instructs the generator to emit a `TypeScript` interface for the struct.
    Interface,

    /// Enables argument/result binding mode for JS interop.
    ///
    /// Used to convert complex Rust types (e.g., structs) into JSON strings
    /// before returning them to JavaScript. The TypeScript wrapper will re-parse the string
    /// into the expected structure on the JS side.
    ///
    /// Each pair maps an input or output name to a known structure name.
    Binding(Vec<(String, String)>),

    /// Instructs the generator to emit a `TypeScript` class for the struct.
    Class,

    /// Enables error suppression mode for JS wrappers.
    ///
    /// Functions and method calls will be wrapped in `try { ... } catch (e) {}` blocks,
    /// and their output type will be adjusted to `T | Error`.
    ExceptionSuppression,
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
        } else if Input::Module(String::new()).to_string() == value {
            Ok(Input::Module(String::new()))
        } else if Input::ExceptionSuppression.to_string() == value {
            Ok(Input::ExceptionSuppression)
        } else {
            Err(format!("Unknown attribute \"{value}\""))
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ignore(..) => "ignore",
                Self::IgnoreSelf => "ignore_self",
                Self::Constructor => "constructor",
                Self::Rename(..) => "rename",
                Self::SnakeCaseNaming => "snake_case_naming",
                Self::Interface => "interface",
                Self::Target(..) => "target",
                Self::Module(..) => "module",
                Self::Binding(..) => "**THIS_IS_RESERVED_KEY_WORD**",
                Self::Class => "class",
                Self::ExceptionSuppression => "exception_suppression",
            }
        )
    }
}
