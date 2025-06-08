use super::Interpreter;
use crate::{
    error::E,
    interpreter::{ts::Writer, Offset},
    nature::{Composite, Nature, Natures, Referred, TypeAsString},
};

/// Generates a TypeScript type **reference** for a composite Rust type (e.g., `Vec<T>`, `Result<T, E>`, `Option<T>`, tuples, functions).
///
/// This method converts internal `Composite` variants into valid and idiomatic TypeScript type references for use in:
/// - function argument types
/// - return types
/// - struct field declarations
///
/// # Variant Behaviors
///
/// - `Array(T)`  
///   → Emits `T[]`
///
/// - `Vec<T>`  
///   → Emits `T[]`, or returns an error if element type is missing.
///
/// - `HashMap<K, V>`  
///   → Emits `Map<K, V>`, or returns an error if key or value type is missing.
///
/// - `Func`  
///   → Emits a function signature like `(arg1: T, arg2: U) => V` or `(...): Promise<V>` if async.  
///   → If marked as a constructor and contains generic args, an error is returned (unsupported case).  
///   → If it's a constructor, emits `(...)` only (no return).
///
/// - `Tuple`  
///   → Emits `[T1, T2, T3]`.
///
/// - `Option<T>`  
///   → Emits `T | null`, or returns an error if type is not set.
///
/// - `Result<T, E>`  
///   → Emits:
///     - `T` if async mode is enabled (async functions wrap result).
///     - `T | Error` or `T | (Error & { err?: E })` if `exception_suppression` is enabled.
///     - `void` if no result and no exception suppression.
///
/// - `Undefined`  
///   → Emits `void`.
///
/// # Parameters
/// - `natures`: Registry of known types (`Natures`), used for reference resolution.
/// - `buf`: Output buffer to write into.
/// - `offset`: Current indentation or nesting depth (used for formatting).
/// - `parent`: Optional parent context for nested declarations.
///
/// # Errors
/// Returns:
/// - `E::Parsing` if required information is missing (e.g., unbound type in `Vec`, `Option`, etc.).
/// - `E::Parsing` if a constructor has generic arguments (currently unsupported).
impl Interpreter for Composite {
    fn reference(
        &self,
        natures: &Natures,
        buf: &mut Writer,
        offset: Offset,
        parent: Option<String>,
    ) -> Result<(), E> {
        match self {
            Self::Array(ty) => {
                ty.reference(natures, buf, offset, parent)?;
                buf.push("[]");
            }
            Self::Vec(_, ty) => {
                if let Some(ty) = ty {
                    ty.reference(natures, buf, offset, parent)?;
                    buf.push("[]");
                } else {
                    return Err(E::Parsing(String::from(
                        "Type Vec doesn't include reference to type",
                    )));
                }
            }
            Self::HashMap(_, key, ty) => {
                if let (Some(key), Some(ty)) = (key, ty) {
                    buf.push("Map<");
                    key.reference(natures, buf, offset.clone(), parent.clone())?;
                    buf.push(", ");
                    ty.reference(natures, buf, offset, parent)?;
                    buf.push(">");
                } else {
                    return Err(E::Parsing(String::from(
                        "Type HashMap doesn't include reference to type or key",
                    )));
                }
            }
            Self::Func(_, args, out, asyncness, constructor) => {
                buf.push("(");
                let mut generic = false;
                for (i, nature) in args.iter().enumerate() {
                    if let Nature::Referred(Referred::FuncArg(name, _context, nature, _)) = nature {
                        buf.push(format!("{name}: "));
                        nature.reference(natures, buf, offset.clone(), parent.clone())?;
                    } else {
                        generic = true;
                        buf.push(format!("arg{i}: "));
                        nature.reference(natures, buf, offset.clone(), parent.clone())?;
                    }
                    if i < args.len() - 1 {
                        buf.push(", ");
                    }
                }
                if *constructor && generic {
                    return Err(E::Parsing(String::from(
                        "Constructor with generic types aren't supported",
                    )));
                }
                if *constructor {
                    buf.push(")");
                    return Ok(());
                }
                buf.push(format!("){} ", if generic { " =>" } else { ":" }));
                if *asyncness {
                    buf.push("Promise<");
                }
                if let Some(out) = out {
                    out.reference(natures, buf, offset.clone(), parent)?;
                } else {
                    buf.push("void");
                }
                if *asyncness {
                    buf.push(">");
                }
            }
            Self::Tuple(_, tys) => {
                buf.push("[");
                let last = tys.len() - 1;
                for (i, ty) in tys.iter().enumerate() {
                    ty.reference(natures, buf, offset.clone(), parent.clone())?;
                    if i < last {
                        buf.push(", ");
                    }
                }
                buf.push("]");
            }
            Self::Option(_, ty) => {
                if let Some(ty) = ty {
                    ty.reference(natures, buf, offset, parent)?;
                    buf.push(" | null");
                } else {
                    return Err(E::Parsing(String::from(
                        "Type Option doesn't include reference to type",
                    )));
                }
            }
            Self::Result(_, res, err, exception_suppression, asyncness) => {
                if let Some(res) = res {
                    res.reference(natures, buf, offset.clone(), parent)?;
                }
                if *asyncness {
                    if res.is_none() {
                        buf.push("void");
                    }
                    return Ok(());
                }
                let err_ext = if let Some(err) = err {
                    format!("(Error & {{ err?: {}}})", err.type_as_string()?)
                } else {
                    "Error".to_owned()
                };
                if res.is_some() && *exception_suppression {
                    buf.push(format!(" | {err_ext}",));
                }
                if res.is_none() && *exception_suppression {
                    buf.push(format!("{err_ext} | void",));
                }
                if res.is_none() && !*exception_suppression {
                    buf.push("void");
                }
            }
            Self::Undefined(_) => {
                buf.push("void");
            }
        }
        Ok(())
    }
}
