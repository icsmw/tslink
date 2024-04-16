use crate::{
    context::Context,
    error::E,
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
            let return_ty = Nature::extract(*ty.clone(), context.clone())?;
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
    fn extract(t: T, context: Context) -> Result<Nature, E>;
}

impl Extract<&GenericArgument> for Nature {
    fn extract(arg: &GenericArgument, context: Context) -> Result<Nature, E> {
        match arg {
            GenericArgument::Type(ty) => Nature::extract(ty, context),
            _ => Err(E::NotSupported),
        }
    }
}

impl Extract<&Ident> for Nature {
    fn extract(ident: &Ident, context: Context) -> Result<Nature, E> {
        let origin = ident.to_string();
        Ok(match origin.as_str() {
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" | "u64" | "i64" | "usize" => {
                Nature::Primitive(Primitive::Number(OriginType::from(ident.clone())))
            }
            "bool" => Nature::Primitive(Primitive::Boolean(OriginType::from(ident.clone()))),
            "String" => Nature::Primitive(Primitive::String(OriginType::from(ident.clone()))),
            a => Nature::Refered(Refered::Ref(a.to_string(), Some(context.clone()))),
        })
    }
}

impl Extract<&Punctuated<PathSegment, PathSep>> for Nature {
    fn extract(segments: &Punctuated<PathSegment, PathSep>, context: Context) -> Result<Nature, E> {
        if segments.len() > 1 {
            return Err(E::Parsing(String::from(
                "Not supported Other Type for more than 1 PathSegment",
            )));
        }
        if let Some(segment) = segments.first() {
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
                _ => {
                    return Err(E::Parsing(String::from(
                        "Only Vec, HashMap, Option and Result are supported",
                    )))
                }
            };
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    for arg in args.args.iter() {
                        ty.bind(Nature::extract(arg, context.clone())?)?;
                    }
                }
                _ => return Err(E::NotSupported),
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
    fn extract(ty: &TypeTuple, context: Context) -> Result<Nature, E> {
        if ty.elems.is_empty() {
            Ok(Nature::Composite(Composite::Undefined(OriginType::from(
                ty.clone(),
            ))))
        } else {
            let mut nature =
                Nature::Composite(Composite::Tuple(OriginType::from(ty.clone()), vec![]));
            for element in ty.elems.iter() {
                nature.bind(Nature::extract(element, context.clone())?)?;
            }
            Ok(nature)
        }
    }
}

impl Extract<&Type> for Nature {
    fn extract(ty: &Type, context: Context) -> Result<Nature, E> {
        match ty {
            Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident() {
                    Nature::extract(ident, context)
                } else {
                    Nature::extract(&type_path.path.segments, context)
                }
            }
            Type::Tuple(type_tuple) => Nature::extract(type_tuple, context),
            _ => Err(E::NotSupported),
        }
    }
}

impl Extract<Type> for Nature {
    fn extract(ty: Type, context: Context) -> Result<Nature, E> {
        Self::extract(&ty, context)
    }
}

impl Extract<&ImplItemFn> for Nature {
    fn extract(fn_item: &ImplItemFn, context: Context) -> Result<Nature, E> {
        let mut args = vec![];
        for fn_arg in fn_item.sig.inputs.iter() {
            if let FnArg::Typed(ty) = fn_arg {
                let arg_name = if let Pat::Ident(id) = *ty.pat.clone() {
                    id.ident.to_string()
                } else {
                    return Err(E::Parsing(String::from("Cannot find ident for FnArg")));
                };
                args.push(Nature::Refered(Refered::FuncArg(
                    arg_name.clone(),
                    context.clone(),
                    Box::new(Nature::extract(*ty.ty.clone(), context.clone())?),
                    context.get_bound(&arg_name),
                )));
            }
        }
        let out = get_fn_return(
            &fn_item.sig.output,
            &context,
            fn_item.sig.asyncness.is_some(),
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
    fn extract(fn_item: &ItemFn, context: Context) -> Result<Nature, E> {
        let mut args = vec![];
        for fn_arg in fn_item.sig.inputs.iter() {
            if let FnArg::Typed(ty) = fn_arg {
                let arg_name = if let Pat::Ident(id) = *ty.pat.clone() {
                    id.ident.to_string()
                } else {
                    return Err(E::Parsing(String::from("Cannot find ident for FnArg")));
                };
                args.push(Nature::Refered(Refered::FuncArg(
                    arg_name.clone(),
                    context.clone(),
                    Box::new(Nature::extract(*ty.ty.clone(), context.clone())?),
                    context.get_bound(&arg_name),
                )));
            }
        }
        let out = get_fn_return(
            &fn_item.sig.output,
            &context,
            fn_item.sig.asyncness.is_some(),
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
