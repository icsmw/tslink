use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    nature::{Composite, Nature, OriginType, Primitive, Referred},
};
use syn::{
    punctuated::Punctuated, token::PathSep, FnArg, GenericArgument, Ident, ImplItemFn, ItemFn, Pat,
    PathArguments, PathSegment, ReturnType, Type, TypeTuple,
};

use super::TypeAsString;

/// Extracts and wraps the return type of a Rust function as a [`Composite::Result`] `Nature`.
///
/// This function standardizes all function return values to a `Result`-like composite representation
/// for TypeScript code generation, regardless of whether the original return type is `()` or some other type.
///
/// # Behavior
/// - If the return type is omitted (`ReturnType::Default`), it is treated as `Result<(), E>` with no `Ok`/`Err` types set.
/// - If the return type is explicitly provided:
///   - If it’s already a `Composite::Result`, the function only updates its asyncness flag.
///   - Otherwise, wraps the return type in `Composite::Result` as the `Ok` variant.
///
/// # Parameters
/// - `output`: The return type node from the function signature (`syn::ReturnType`).
/// - `context`: The current macro context to resolve exception suppression and bindings.
/// - `asyncness`: Whether the function is `async`.
/// - `cfg`: Global code generation configuration.
///
/// # Returns
/// An optional boxed `Nature` representing the return value. Always present unless parsing fails.
///
/// # Errors
/// Returns an error if the return type cannot be extracted or exception suppression configuration is invalid.
pub fn get_fn_return(
    output: &ReturnType,
    context: &Context,
    asyncness: bool,
    cfg: &Config,
) -> Result<Option<Box<Nature>>, E> {
    Ok(match output {
        ReturnType::Default => Some(Box::new(Nature::Composite(Composite::Result(
            OriginType::from(output.clone()),
            None,
            None,
            context.exception_suppression()?,
            asyncness,
        )))),
        ReturnType::Type(_, ty) => {
            let return_ty = Nature::extract(*ty.clone(), context.clone(), cfg)?;
            Some(
                if let Nature::Composite(Composite::Result(a, b, c, d, _)) = return_ty {
                    Box::new(Nature::Composite(Composite::Result(a, b, c, d, asyncness)))
                } else {
                    Box::new(Nature::Composite(Composite::Result(
                        OriginType::from(*ty.clone()),
                        Some(Box::new(return_ty)),
                        None,
                        context.exception_suppression()?,
                        asyncness,
                    )))
                },
            )
        }
    })
}

/// Trait for extracting a [`Nature`] from a syntax element of type `T`.
///
/// This trait defines how various Rust syntax nodes (e.g., types, identifiers, generic arguments)
/// are transformed into the internal representation used for code generation.
///
/// It allows recursive and type-safe parsing of the Rust AST with contextual and configuration-aware behavior.
pub trait Extract<T> {
    /// Extracts a [`Nature`] from the given input `t`, using the macro context and generation config.
    ///
    /// # Parameters
    /// - `t`: The syntax element to extract.
    /// - `context`: The active macro context.
    /// - `cfg`: Global configuration settings.
    ///
    /// # Returns
    /// A `Nature` describing the type structure or semantic meaning of the input.
    ///
    /// # Errors
    /// Returns an error if the node is unsupported or contains semantic conflicts.
    fn extract(t: T, context: Context, cfg: &Config) -> Result<Nature, E>;
}

/// Extracts a `Nature` from a generic argument like `T` in `Vec<T>`, supporting only type arguments.
impl Extract<&GenericArgument> for Nature {
    fn extract(arg: &GenericArgument, context: Context, cfg: &Config) -> Result<Nature, E> {
        match arg {
            GenericArgument::Type(ty) => Nature::extract(ty, context, cfg),
            _ => Err(E::NotSupported("GenericArgument".to_owned())),
        }
    }
}

/// Extracts a `Primitive` or `Referred::Ref` from a simple identifier (e.g., `i32`, `String`, `MyType`).
///
/// Applies primitive classification and configuration-based overrides (`type_map`, etc.).
impl Extract<&Ident> for Nature {
    fn extract(ident: &Ident, context: Context, cfg: &Config) -> Result<Nature, E> {
        let origin = ident.to_string();
        Ok(match (origin.as_str(), cfg.int_over_32_as_big_int) {
            ("u8" | "u16" | "u32" | "i8" | "i16" | "i32" | "f16" | "f32" | "f64", true) => {
                Nature::Primitive(Primitive::Number(OriginType::from(ident.clone())))
            }
            ("u64" | "u128" | "i64" | "i128" | "usize" | "isize", true) => {
                Nature::Primitive(Primitive::BigInt(OriginType::from(ident.clone())))
            }
            (
                "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64" | "isize"
                | "f16" | "f32" | "f64",
                false,
            ) => Nature::Primitive(Primitive::Number(OriginType::from(ident.clone()))),
            ("u128" | "i128", false) => {
                Nature::Primitive(Primitive::BigInt(OriginType::from(ident.clone())))
            }
            ("bool", ..) => Nature::Primitive(Primitive::Boolean(OriginType::from(ident.clone()))),
            ("String" | "str", ..) => {
                Nature::Primitive(Primitive::String(OriginType::from(ident.clone())))
            }
            ("f128", ..) => {
                return Err(E::NotSupported(
                    "Type <f128> doesn't have direct equalent in JavaScript".to_owned(),
                ))
            }
            (a, ..) => {
                let serialized = cfg.overwrite_reftype(serialize_name(a));
                match serialized.as_ref() {
                    "boolean" => {
                        Nature::Primitive(Primitive::Boolean(OriginType::from(ident.clone())))
                    }
                    "string" => {
                        Nature::Primitive(Primitive::String(OriginType::from(ident.clone())))
                    }
                    "number" => {
                        Nature::Primitive(Primitive::Number(OriginType::from(ident.clone())))
                    }
                    _ => Nature::Referred(Referred::Ref(serialized, Some(context.clone()))),
                }
            }
        })
    }
}

/// Extracts a composite or referred type from a path like `Vec<u8>`, `HashMap<K, V>`, or `MyType<T>`.
///
/// Applies logic for generics and known container types like `Vec`, `Option`, `Result`, and `HashMap`.
impl Extract<&Punctuated<PathSegment, PathSep>> for Nature {
    fn extract(
        segments: &Punctuated<PathSegment, PathSep>,
        context: Context,
        cfg: &Config,
    ) -> Result<Nature, E> {
        if let Some(segment) = segments.last() {
            let mut ty = match segment.ident.to_string().as_str() {
                "Vec" => Nature::Composite(Composite::Vec(OriginType::from(segment.clone()), None)),
                "HashMap" => Nature::Composite(Composite::HashMap(
                    OriginType::from(segment.clone()),
                    None,
                    None,
                )),
                "Option" => {
                    Nature::Composite(Composite::Option(OriginType::from(segment.clone()), None))
                }
                "Result" => Nature::Composite(Composite::Result(
                    OriginType::from(segment.clone()),
                    None,
                    None,
                    context.exception_suppression()?,
                    false,
                )),
                _ => Nature::extract(&segment.ident, context.clone(), cfg)?,
            };
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                for arg in args.args.iter() {
                    ty.bind(Nature::extract(arg, context.clone(), cfg)?)?;
                }
            }
            Ok(ty)
        } else {
            Err(E::Parsing(String::from(
                "For not primitive types expected at least one segment",
            )))
        }
    }
}

/// Extracts a tuple type, mapping each element into a `Nature`.
///
/// Unit tuples (`()`) are mapped to `Composite::Undefined`.
impl Extract<&TypeTuple> for Nature {
    fn extract(ty: &TypeTuple, context: Context, cfg: &Config) -> Result<Nature, E> {
        if ty.elems.is_empty() {
            Ok(Nature::Composite(Composite::Undefined(OriginType::from(
                ty.clone(),
            ))))
        } else {
            let mut nature =
                Nature::Composite(Composite::Tuple(OriginType::from(ty.clone()), vec![]));
            for element in ty.elems.iter() {
                nature.bind(Nature::extract(element, context.clone(), cfg)?)?;
            }
            Ok(nature)
        }
    }
}

/// Extracts a `Nature` from a full Rust type node (`Type`), including arrays, references, paths, and tuples.
///
/// Supports recursive parsing and validates `Array` element constraints.
impl Extract<&Type> for Nature {
    fn extract(ty: &Type, context: Context, cfg: &Config) -> Result<Nature, E> {
        match ty {
            Type::Array(ty_array) => {
                let inner = Nature::extract(ty_array.elem.as_ref(), context, cfg)?;
                if !matches!(inner, Nature::Primitive(..)) {
                    return Err(E::NotSupported(format!(
                        "{} isn't supported in Array",
                        inner.type_as_string()?
                    )));
                }
                Ok(Nature::Composite(Composite::Array(Box::new(inner))))
            }
            Type::Reference(ty_ref) => Nature::extract(ty_ref.elem.as_ref(), context, cfg),
            Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident() {
                    Nature::extract(ident, context, cfg)
                } else {
                    Nature::extract(&type_path.path.segments, context, cfg)
                }
            }
            Type::Tuple(type_tuple) => Nature::extract(type_tuple, context, cfg),
            _ => Err(E::NotSupported("Type".to_owned())),
        }
    }
}

/// Convenience wrapper to allow consuming `Type` by value (delegates to reference-based impl).
impl Extract<Type> for Nature {
    fn extract(ty: Type, context: Context, cfg: &Config) -> Result<Nature, E> {
        Self::extract(&ty, context, cfg)
    }
}

/// Extracts the function signature from an `impl` method (`ImplItemFn`) as `Composite::Func`.
///
/// Parses function arguments, detects constructor status, handles async flags, and binds outputs.
impl Extract<&ImplItemFn> for Nature {
    fn extract(fn_item: &ImplItemFn, context: Context, cfg: &Config) -> Result<Nature, E> {
        let mut args = vec![];
        for fn_arg in fn_item.sig.inputs.iter() {
            if let FnArg::Typed(ty) = fn_arg {
                let arg_name = if let Pat::Ident(id) = *ty.pat.clone() {
                    id.ident.to_string()
                } else {
                    return Err(E::Parsing(String::from("Cannot find ident for FnArg")));
                };
                args.push(Nature::Referred(Referred::FuncArg(
                    serialize_name(&arg_name),
                    context.clone(),
                    Box::new(Nature::extract(*ty.ty.clone(), context.clone(), cfg)?),
                    context.get_bound(&arg_name),
                )));
            }
        }
        let out = get_fn_return(
            &fn_item.sig.output,
            &context,
            fn_item.sig.asyncness.is_some(),
            cfg,
        )?;
        let constructor = if let Some(Nature::Referred(Referred::Ref(re, _))) = out.as_deref() {
            re == "Self"
        } else {
            false
        } || context.as_constructor();
        Ok(Self::Composite(Composite::Func(
            OriginType::from(fn_item.clone()),
            args,
            out,
            fn_item.sig.asyncness.is_some(),
            constructor,
        )))
    }
}

/// Extracts the function signature from a freestanding `fn` item (`ItemFn`) as `Composite::Func`.
///
/// Similar to `ImplItemFn` but excludes `self` and handles top-level function cases.
impl Extract<&ItemFn> for Nature {
    fn extract(fn_item: &ItemFn, context: Context, cfg: &Config) -> Result<Nature, E> {
        let mut args = vec![];
        for fn_arg in fn_item.sig.inputs.iter() {
            if let FnArg::Typed(ty) = fn_arg {
                let arg_name = if let Pat::Ident(id) = *ty.pat.clone() {
                    id.ident.to_string()
                } else {
                    return Err(E::Parsing(String::from("Cannot find ident for FnArg")));
                };
                args.push(Nature::Referred(Referred::FuncArg(
                    serialize_name(&arg_name),
                    context.clone(),
                    Box::new(Nature::extract(*ty.ty.clone(), context.clone(), cfg)?),
                    context.get_bound(&arg_name),
                )));
            }
        }
        let out = get_fn_return(
            &fn_item.sig.output,
            &context,
            fn_item.sig.asyncness.is_some(),
            cfg,
        )?;
        let constructor = if let Some(Nature::Referred(Referred::Ref(re, _))) = out.as_deref() {
            re == "Self"
        } else {
            false
        } || context.as_constructor();
        Ok(Self::Composite(Composite::Func(
            OriginType::from(fn_item.clone()),
            args,
            out,
            fn_item.sig.asyncness.is_some(),
            constructor,
        )))
    }
}
