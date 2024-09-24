use crate::{
    config::Config,
    context::Context,
    error::E,
    interpreter::serialize_name,
    nature::{Composite, Nature, OriginType, Primitive, Refered},
};
use syn::{
    punctuated::Punctuated, token::PathSep, FnArg, GenericArgument, Ident, ImplItemFn, ItemFn, Pat,
    PathArguments, PathSegment, ReturnType, Type, TypeTuple,
};

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

pub trait Extract<T> {
    fn extract(t: T, context: Context, cfg: &Config) -> Result<Nature, E>;
}

impl Extract<&GenericArgument> for Nature {
    fn extract(arg: &GenericArgument, context: Context, cfg: &Config) -> Result<Nature, E> {
        match arg {
            GenericArgument::Type(ty) => Nature::extract(ty, context, cfg),
            _ => Err(E::NotSupported("".to_owned())),
        }
    }
}

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
            ("String", ..) => Nature::Primitive(Primitive::String(OriginType::from(ident.clone()))),
            ("f128", ..) => {
                return Err(E::NotSupported(
                    "Type <f128> doesn't have direct equalent in JavaScript".to_owned(),
                ))
            }
            (a, ..) => Nature::Refered(Refered::Ref(serialize_name(a), Some(context.clone()))),
        })
    }
}

impl Extract<&Punctuated<PathSegment, PathSep>> for Nature {
    fn extract(
        segments: &Punctuated<PathSegment, PathSep>,
        context: Context,
        cfg: &Config,
    ) -> Result<Nature, E> {
        // if segments.len() > 1 {
        //     return Err(E::Parsing(String::from(
        //         "Not supported Other Type for more than 1 PathSegment",
        //     )));
        // }
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

impl Extract<&Type> for Nature {
    fn extract(ty: &Type, context: Context, cfg: &Config) -> Result<Nature, E> {
        match ty {
            Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident() {
                    Nature::extract(ident, context, cfg)
                } else {
                    Nature::extract(&type_path.path.segments, context, cfg)
                }
            }
            Type::Tuple(type_tuple) => Nature::extract(type_tuple, context, cfg),
            _ => Err(E::NotSupported("".to_owned())),
        }
    }
}

impl Extract<Type> for Nature {
    fn extract(ty: Type, context: Context, cfg: &Config) -> Result<Nature, E> {
        Self::extract(&ty, context, cfg)
    }
}

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
                args.push(Nature::Refered(Refered::FuncArg(
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
        let constructor = if let Some(Nature::Refered(Refered::Ref(re, _))) = out.as_deref() {
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
                args.push(Nature::Refered(Refered::FuncArg(
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
        let constructor = if let Some(Nature::Refered(Refered::Ref(re, _))) = out.as_deref() {
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
