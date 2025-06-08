mod input;
mod target;

use crate::{
    config,
    error::E,
    nature::{Nature, Referred},
};
use convert_case::{Case, Casing};
use input::Input;
use std::{
    collections::HashMap,
    convert::{From, TryFrom},
    ops::Deref,
    path::PathBuf,
};
use syn::{
    parse::{self, Parse, ParseStream},
    punctuated::Punctuated,
    AttrStyle, Attribute, Expr, Lit, Meta, Token,
};
pub use target::Target;

/// Holds the contextual information associated with a macro invocation.
///
/// `Context` provides metadata required during code generation,
/// including user-specified macro attributes, output targets,
/// parent scoping, and type bindings for generics.
///
/// It is attached to every entity (`Referred`, `Field`, `Func`, etc.)
/// that participates in the TypeScript code generation process,
/// and is used to propagate configuration and behavior settings.
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// List of macro inputs (`#[tslink(...)]`) that define generation rules
    /// such as renaming, ignoring, binding modes, etc.
    pub inputs: Vec<Input>,

    /// A list of `(Target, PathBuf)` pairs indicating where generated `.ts` or `.d.ts` code should be written.
    /// May be inherited or overridden depending on macro nesting.
    pub targets: Vec<(Target, PathBuf)>,

    /// Optional link to the parent context, used for resolving inherited rules in nested scopes
    /// (e.g., fields inside a struct or methods inside an impl block).
    pub parent: Option<Box<Context>>,

    /// Mapping of generic type identifiers to their resolved [`Nature`].
    /// This enables handling of type aliases and resolving bindings like `T: SomeType` for accurate generation.
    pub generics: HashMap<String, Nature>,
}

impl Context {
    /// Creates a new `Context` instance from the given list of macro inputs.
    ///
    /// Automatically extracts `.ts` target paths from the input attributes if any.
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
            generics: HashMap::new(),
        }
    }

    /// Parses the outer macro attributes (`#[tslink(...)]`) into a `Context`.
    ///
    /// Returns a default context if no relevant attributes are found.
    pub fn try_from_or_default(attrs: &[Attribute]) -> Result<Self, E> {
        for attr in attrs.iter() {
            if matches!(attr.style, AttrStyle::Outer) {
                if matches!(attr.meta, Meta::Path(_)) {
                    // No attributes
                    continue;
                }
                let attr_name = attr.path().segments.last().unwrap().ident.to_string();
                if attr_name != env!("CARGO_PKG_NAME") {
                    continue;
                }
                return attr
                    .parse_args_with(Context::parse)
                    .map_err(|e| E::PasringContext(e.to_string()));
            }
        }
        Ok(Self::default())
    }

    /// Retrieves the optional module name specified in the attributes.
    pub fn get_module(&self) -> Option<String> {
        self.inputs.iter().find_map(|inp| {
            if let Input::Module(module) = inp {
                Some(module.to_owned())
            } else {
                None
            }
        })
    }

    /// Adds generic bindings from the provided list of `Nature::Referred::Generic(...)` types.
    pub fn add_generics(&mut self, generics: Vec<Nature>) {
        generics.iter().for_each(|n| {
            if let Nature::Referred(Referred::Generic(k, n)) = n {
                self.generics.insert(k.clone(), n.deref().clone());
            }
        });
    }

    /// Looks up a generic binding by name, traversing up the context tree if needed.
    pub fn get_generic(&self, key: &str) -> Option<&Nature> {
        self.generics.get(key).or_else(|| {
            if let Some(context) = self.parent.as_ref() {
                context.get_generic(key)
            } else {
                None
            }
        })
    }

    /// Sets the parent context (used for inheritance in nested scopes).
    pub fn set_parent(&mut self, parent: Context) {
        self.parent = Some(Box::new(parent));
    }

    /// Returns `true` if the current item is marked with `#[tslink(ignore)]`.
    pub fn ignore_self(&self) -> bool {
        self.inputs.iter().any(|i| matches!(i, Input::IgnoreSelf))
    }

    /// Checks whether a specific field or member name is listed in the ignore list (recursively).
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

    /// Returns a copy of the explicitly ignored field names for this context.
    pub fn ignored_list(&self) -> Vec<String> {
        if let Some(Input::Ignore(list)) =
            self.inputs.iter().find(|i| matches!(i, Input::Ignore(_)))
        {
            list.clone()
        } else {
            vec![]
        }
    }

    /// Returns `true` if the item is marked as a TypeScript class (or inherits the flag from its parent).
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

    /// Returns `true` if the item is explicitly marked as a constructor method.
    pub fn as_constructor(&self) -> bool {
        self.inputs.iter().any(|i| matches!(i, Input::Constructor))
    }

    /// Returns `true` if exception suppression is enabled either via macro attribute or global config.
    ///
    /// This mode wraps JS calls in `try/catch` and changes the output type to `T | Error`.
    pub fn exception_suppression(&self) -> Result<bool, E> {
        let config = config::get()?;
        Ok(config.exception_suppression
            || self
                .inputs
                .iter()
                .any(|i| matches!(i, Input::ExceptionSuppression)))
    }

    /// Resolves the renamed field name for TypeScript, using the attribute or global config fallback.
    pub fn rename_field(&self, origin: &str) -> Result<String, E> {
        let config = config::get()?;
        self.rename(origin)
            .or_else(|| Some(config.rename_field(origin)))
            .ok_or(E::Other(String::from("Fail to get renaming settings")))
    }

    /// Resolves the renamed method name for TypeScript, using the attribute or global config fallback.
    pub fn rename_method(&self, origin: &str) -> Result<String, E> {
        let config = config::get()?;
        self.rename(origin)
            .or_else(|| Some(config.rename_method(origin)))
            .ok_or(E::Other(String::from("Fail to get renaming settings")))
    }

    /// Returns a list of bound function arguments (excluding special bindings like "result" or "error").
    pub fn get_bound_args(&self) -> Vec<(String, String)> {
        if let Some(Input::Binding(arguments)) =
            self.inputs.iter().find(|i| matches!(i, Input::Binding(_)))
        {
            return arguments
                .iter()
                .filter(|(n, _)| n != "result" && n != "error")
                .cloned()
                .collect();
        }
        vec![]
    }

    /// Retrieves the bound alias for a given argument name, if present.
    pub fn get_bound(&self, name: &str) -> Option<String> {
        if let Some(Input::Binding(arguments)) =
            self.inputs.iter().find(|i| matches!(i, Input::Binding(_)))
        {
            return arguments.iter().find_map(|(n, ref_name)| {
                if n == name {
                    Some(ref_name.to_owned())
                } else {
                    None
                }
            });
        }
        None
    }

    /// Returns `true` if the function result is expected to be returned as a JSON string.
    pub fn result_as_json(&self) -> Result<bool, E> {
        if let Some(Input::Binding(arguments)) =
            self.inputs.iter().find(|i| matches!(i, Input::Binding(_)))
        {
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

    /// Returns `true` if the function error value should be returned as a JSON string.
    pub fn error_as_json(&self) -> Result<bool, E> {
        if let Some(Input::Binding(arguments)) =
            self.inputs.iter().find(|i| matches!(i, Input::Binding(_)))
        {
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

    /// Resolves an inline renaming instruction (if present), or applies `snake_case → camelCase` if requested.
    ///
    /// Used internally by renaming logic.
    fn rename(&self, origin: &str) -> Option<String> {
        if let Some(Input::Rename(name)) =
            self.inputs.iter().find(|i| matches!(i, Input::Rename(_)))
        {
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
}

impl Parse for Context {
    /// Parses macro input attributes in the form accepted by `#[tslink(...)]`.
    ///
    /// Supports the following syntaxes:
    ///
    /// - Standalone flags:
    ///   ```ignore
    ///   #[tslink(ignore)]
    ///   #[tslink(interface)]
    ///   #[tslink(exception_suppression)]
    ///   ```
    ///
    /// - Key-value assignments:
    ///   ```ignore
    ///   #[tslink(rename = "NewName")]
    ///   #[tslink(module = "MyModule")]
    ///   #[tslink(target = "out.d.ts;other.ts")]
    ///   #[tslink(ignore = "field1;field2")]
    ///   #[tslink(result = "json", error = "json")]
    ///   ```
    ///
    /// Any unrecognized key-value assignment is interpreted as a **binding** —
    /// associating a function argument or result/error with a transformation (e.g., `"json"`).
    ///
    /// # Errors
    ///
    /// Returns a `syn::Error` if:
    /// - The syntax is invalid (e.g., incorrect assignment structure).
    /// - A value is not a string literal.
    /// - The key is unsupported in the current context (e.g., `target` on a field).
    /// - A file path is invalid or lacks a known extension (`.ts`, `.d.ts`, `.js`).
    ///
    /// # Returns
    ///
    /// A fully constructed `Context` with parsed `inputs` and optional `bindings`.
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut inputs: Vec<Input> = vec![];
        let mut bindings: Vec<(String, String)> = vec![];
        for expr in Punctuated::<Expr, Token![,]>::parse_terminated(input)?.into_iter() {
            match expr {
                Expr::Assign(a) => {
                    let link = a.clone();
                    if let (Expr::Path(left), Expr::Lit(right)) = (*a.left, *a.right) {
                        if let (Some(left), Lit::Str(value)) = (left.path.get_ident(), right.lit) {
                            let value = value.value();
                            let mut input = match Input::try_from(left.to_string().as_ref()) {
                                Ok(input) => match input {
                                    Input::Rename(..) => Some(Input::Rename(value)),
                                    Input::Module(..) => Some(Input::Module(value)),
                                    Input::Target(..) => {
                                        let mut targets: Vec<(Target, PathBuf)> = vec![];
                                        for s in value.split(';') {
                                            let path = PathBuf::from(s.trim());
                                            if let Some(ext) = path.extension() {
                                                match Target::try_from(
                                                    ext.to_string_lossy().to_string().as_str(),
                                                ) {
                                                    Ok(t) => targets.push((t, path)),
                                                    Err(e) => {
                                                        return Err(syn::Error::new(
                                                            left.span(),
                                                            format!("Unknown target: {s} ({e})"),
                                                        ));
                                                    }
                                                }
                                            } else {
                                                return Err(syn::Error::new(
                                                    left.span(),
                                                    format!("Cannot get extension of file: {s}; expecting .ts; .d.ts; .js"),
                                                ));
                                            }
                                        }
                                        Some(Input::Target(targets))
                                    }
                                    Input::Ignore(_) => Some(Input::Ignore(
                                        value
                                            .split(';')
                                            .map(|s| s.trim().to_owned())
                                            .collect::<Vec<String>>(),
                                    )),
                                    _ => {
                                        return Err(syn::Error::new(
                                            left.span(),
                                            format!("Attribute \"{left}\" cannot be applied on this level"),
                                        ));
                                    }
                                },
                                Err(_e) => {
                                    bindings.push((left.to_string(), value));
                                    None
                                }
                            };
                            if let Some(input) = input.take() {
                                inputs.push(input);
                            }
                        } else {
                            return Err(syn::Error::new(
                                link.eq_token.span,
                                "Expecting expr like key = \"value as String\"",
                            ));
                        }
                    } else {
                        return Err(syn::Error::new(
                            link.eq_token.span,
                            "Expecting expr like key = \"value as String\"",
                        ));
                    }
                }
                Expr::Path(p) => {
                    if let Some(ident) = p.path.get_ident() {
                        let input = match Input::try_from(ident.to_string().as_ref()) {
                            Ok(input) => {
                                match input {
                                    Input::Ignore(_) => Input::IgnoreSelf,
                                    Input::SnakeCaseNaming
                                    | Input::Class
                                    | Input::Interface
                                    | Input::ExceptionSuppression
                                    | Input::Constructor => input,
                                    _ => {
                                        return Err(syn::Error::new(
                                            ident.span(),
                                            format!("Attribute \"{ident}\" cannot be applied on this level"),
                                        ));
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(syn::Error::new(
                                    ident.span(),
                                    format!("Unknown attribute: {ident} ({e})"),
                                ));
                            }
                        };
                        inputs.push(input);
                    } else {
                        return Err(syn::Error::new_spanned(p, "Cannot extract identification"));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "Expecting expr like [key = \"value as String\"] or [key]",
                    ));
                }
            }
        }
        if !bindings.is_empty() {
            inputs.push(Input::Binding(bindings));
        }
        Ok(Context::new(inputs))
    }
}
