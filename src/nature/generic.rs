use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    nature::{types, Composite, Extract, Nature, OriginType, Referred},
};
use syn::{
    GenericParam, Generics, PathArguments, PredicateType, TraitBound, TypeParam, TypeParamBound,
    WherePredicate,
};

/// Trait for extracting a [`Nature`] representation from a generic constraint or type bound.
///
/// This is used when analyzing generics in the form of `T: Trait`, `where T: Fn(...) -> ...`, etc.,
/// and is particularly important for supporting function-like generics in generated code.
pub trait ExtractGeneric<T> {
    /// Attempts to extract a typed `Nature` from a generic bound or predicate.
    ///
    /// # Parameters
    /// - `t`: The syntax node representing a trait bound or type parameter.
    /// - `generic_ref`: An optional reference to the alias or identifier being bound (e.g., `"T"`).
    /// - `cfg`: Global configuration settings.
    ///
    /// # Returns
    /// - `Some(Nature)`: If a valid representation is recognized (e.g., a generic function type).
    /// - `None`: If the bound is irrelevant or ignored (e.g., lifetimes, unknown constraints).
    ///
    /// # Errors
    /// Returns an error if parsing fails or required alias is missing.
    fn extract_generic(
        t: T,
        generic_ref: Option<String>,
        cfg: &Config,
    ) -> Result<Option<Nature>, E>;
}

/// Parses a `TraitBound` (e.g., `T: Fn(...) -> ...`) and constructs a function-type `Nature::Referred::Generic`.
///
/// # Errors
/// - Returns an error if `generic_ref` is not provided.
/// - Only supports `Fn(...) -> ...` style traits; others are ignored.
impl ExtractGeneric<&TraitBound> for Nature {
    fn extract_generic(
        tr: &TraitBound,
        generic_ref: Option<String>,
        cfg: &Config,
    ) -> Result<Option<Nature>, E> {
        let generic_ref = if let Some(generic_ref) = generic_ref {
            generic_ref
        } else {
            return Err(E::Parsing(
                "Parsing generic from TraitBound isn't possible without generic reference (alias)"
                    .to_string(),
            ));
        };
        for tr in tr.path.segments.iter() {
            if let "Fn" = tr.ident.to_string().as_str() {
                let (fn_args, output): (Vec<Nature>, Option<Box<Nature>>) = match &tr.arguments {
                    PathArguments::AngleBracketed(_) => Err(E::Parsing(
                        "Unexpected PathArguments::AngleBracketed".to_string(),
                    ))?,
                    PathArguments::None => {
                        Err(E::Parsing("Unexpected PathArguments::None".to_string()))?
                    }
                    PathArguments::Parenthesized(arg) => {
                        let mut fn_args: Vec<Nature> = vec![];
                        for input in arg.inputs.iter() {
                            fn_args.push(Nature::extract(input, Context::default(), cfg)?);
                        }
                        (
                            fn_args,
                            types::get_fn_return(&arg.output, &Context::default(), false, cfg)?,
                        )
                    }
                };
                return Ok(Some(Nature::Referred(Referred::Generic(
                    serialize_name(generic_ref),
                    Box::new(Nature::Composite(Composite::Func(
                        OriginType::from(tr.clone()),
                        fn_args,
                        output,
                        false,
                        false,
                    ))),
                ))));
            }
        }
        Ok(None)
    }
}

/// Parses a `TypeParam` from a `<T: Trait>` declaration and delegates to `TraitBound` extraction.
impl ExtractGeneric<&TypeParam> for Nature {
    fn extract_generic(
        type_param: &TypeParam,
        _generic_ref: Option<String>,
        cfg: &Config,
    ) -> Result<Option<Nature>, E> {
        let generic_ref = type_param.ident.to_string();
        for bound in type_param.bounds.iter() {
            match bound {
                TypeParamBound::Lifetime(_) => {
                    // Ignore
                }
                TypeParamBound::Verbatim(_) => {
                    // Ignore
                }
                TypeParamBound::Trait(tr) => {
                    return Nature::extract_generic(tr, Some(generic_ref), cfg);
                }
                _ => {
                    // Ignore
                }
            }
        }
        Ok(None)
    }
}

/// Parses a `PredicateType` from a `where` clause (`where T: Trait`) and attempts generic extraction.
impl ExtractGeneric<&PredicateType> for Nature {
    fn extract_generic(
        pre_type: &PredicateType,
        _generic_ref: Option<String>,
        cfg: &Config,
    ) -> Result<Option<Nature>, E> {
        let generic_ref = if let Nature::Referred(Referred::Ref(name, _)) =
            Nature::extract(&pre_type.bounded_ty, Context::default(), cfg)?
        {
            name
        } else {
            return Err(E::Parsing(
                "Cannot detect name/alias of generic parameter in where section".to_string(),
            ));
        };
        for bound in pre_type.bounds.iter() {
            match bound {
                TypeParamBound::Lifetime(_) => {
                    // Ignore
                }
                TypeParamBound::Verbatim(_) => {
                    // Ignore
                }
                TypeParamBound::Trait(tr) => {
                    return Nature::extract_generic(tr, Some(generic_ref), cfg);
                }
                _ => {
                    // Ignore
                }
            }
        }
        Ok(None)
    }
}

/// Trait for extracting a list of [`Nature`] values from a set of Rust generics.
///
/// This trait is designed to be used on full `Generics` blocks, including both inline parameters (`<T: ...>`)
/// and `where` clauses.
pub trait ExtractGenerics<T> {
    /// Extracts all generic constraints as `Nature` definitions.
    ///
    /// # Parameters
    /// - `t`: The generics block to process.
    /// - `cfg`: Global configuration.
    ///
    /// # Returns
    /// A vector of `Nature::Referred::Generic` entries for use in contextual binding.
    fn extract_generics(t: T, cfg: &Config) -> Result<Vec<Nature>, E>;
}

/// Extracts all usable generic constraints (from both type parameters and where clauses) into `Nature` definitions.
///
/// Ignores lifetimes and unsupported constructs like `T: 'a` or `const N: usize`.
impl ExtractGenerics<&Generics> for Nature {
    fn extract_generics(generics: &Generics, cfg: &Config) -> Result<Vec<Nature>, E> {
        let mut natures = vec![];
        for generic in generics.params.iter() {
            match &generic {
                GenericParam::Const(_) => {}
                GenericParam::Type(ty) => {
                    if let Some(generic) = Nature::extract_generic(ty, None, cfg)? {
                        natures.push(generic);
                    }
                }
                GenericParam::Lifetime(_) => {}
            }
        }
        if let Some(where_clause) = generics.where_clause.as_ref() {
            for generic in where_clause.predicates.iter() {
                match generic {
                    WherePredicate::Type(ty) => {
                        if let Some(generic) = Nature::extract_generic(ty, None, cfg)? {
                            natures.push(generic);
                        }
                    }
                    WherePredicate::Lifetime(_) => {}
                    _ => {}
                }
            }
        }
        Ok(natures)
    }
}
