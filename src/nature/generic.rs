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

pub trait ExtractGeneric<T> {
    fn extract_generic(
        t: T,
        generic_ref: Option<String>,
        cfg: &Config,
    ) -> Result<Option<Nature>, E>;
}

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

pub trait ExtractGenerics<T> {
    fn extract_generics(t: T, cfg: &Config) -> Result<Vec<Nature>, E>;
}

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
