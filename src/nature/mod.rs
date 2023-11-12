mod composite;
mod primitive;
mod refered;

use crate::{context::Context, error::E};
pub use composite::Composite;
pub use primitive::Primitive;
use proc_macro2::TokenStream;
pub use refered::Refered;
use std::{
    collections::{hash_map::Iter, HashMap},
    ops::Deref,
};
use syn::{
    punctuated::Punctuated,
    token::{Comma, PathSep},
    FnArg, GenericArgument, GenericParam, Generics, Ident, ImplItemFn, ItemFn, Pat, PathArguments,
    PathSegment, PredicateType, ReturnType, TraitBound, Type, TypeParam, TypeParamBound,
    WherePredicate,
};

pub struct Natures(HashMap<String, Nature>);

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
    pub fn insert(&mut self, name: &str, nature: Nature) -> Result<(), E> {
        if self.contains(name) {
            Err(E::EntityExist(name.to_owned()))
        } else {
            let _ = self.0.insert(name.to_owned(), nature);
            Ok(())
        }
    }

    pub fn get_mut(&mut self, name: &str, defaults: Option<Nature>) -> Option<&mut Nature> {
        if let (exists, Some(defaults)) = (self.0.contains_key(name), defaults) {
            if !exists {
                let _ = self.0.insert(name.to_owned(), defaults);
            }
        }
        self.0.get_mut(name)
    }

    pub fn filter(&self, filter: fn(&Nature) -> bool) -> Vec<Nature> {
        let mut natures: Vec<Nature> = vec![];
        for (_, n) in self.0.iter() {
            if filter(n) {
                natures.push(n.clone());
            }
        }
        natures
    }

    pub fn iter(&self) -> Iter<'_, std::string::String, Nature> {
        self.0.iter()
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
        if let Nature::Composite(Composite::Func(args, _, _, _)) = self {
            Ok(Natures::get_fn_args_names(args))
        } else {
            Err(E::Parsing("Fail to find arguments of function".to_string()))
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
                _ => Err(E::NotSupported),
            },
            Self::Composite(othr) => match othr {
                composite::Composite::HashMap(k, v) => {
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
                composite::Composite::Option(o) => {
                    if o.is_some() {
                        Err(E::Parsing(String::from(
                            "Option entity already has been bound",
                        )))
                    } else {
                        let _ = o.insert(Box::new(nature));
                        Ok(())
                    }
                }
                composite::Composite::Result(r, e, _) => {
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
                composite::Composite::Tuple(tys) => {
                    tys.push(nature);
                    Ok(())
                }
                composite::Composite::Vec(v) => {
                    if v.is_some() {
                        Err(E::Parsing(String::from(
                            "Vec entity already has been bound",
                        )))
                    } else {
                        let _ = v.insert(Box::new(nature));
                        Ok(())
                    }
                }
                _ => Err(E::NotSupported),
            },
        }
    }

    pub fn is_method_constructor(&self) -> bool {
        if let Nature::Refered(Refered::Field(_, _, nature, _)) = self {
            if let Nature::Composite(Composite::Func(_, _, _, constructor)) = nature.deref() {
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

pub trait ExtractGeneric<T> {
    fn extract_generic(t: T, generic_ref: Option<String>) -> Result<Option<Nature>, E>;
}

impl ExtractGeneric<&TraitBound> for Nature {
    fn extract_generic(tr: &TraitBound, generic_ref: Option<String>) -> Result<Option<Nature>, E> {
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
                            fn_args.push(Nature::extract(input, Context::default())?);
                        }
                        (fn_args, get_fn_return(&arg.output, &Context::default())?)
                    }
                };
                return Ok(Some(Nature::Refered(Refered::Generic(
                    generic_ref,
                    Box::new(Nature::Composite(Composite::Func(
                        fn_args, output, false, false,
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
                    return Nature::extract_generic(tr, Some(generic_ref));
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
    ) -> Result<Option<Nature>, E> {
        let generic_ref = if let Nature::Refered(Refered::Ref(name, _)) =
            Nature::extract(&pre_type.bounded_ty, Context::default())?
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
                    return Nature::extract_generic(tr, Some(generic_ref));
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
    fn extract_generics(t: T) -> Result<Vec<Nature>, E>;
}

impl ExtractGenerics<&Generics> for Nature {
    fn extract_generics(generics: &Generics) -> Result<Vec<Nature>, E> {
        let mut natures = vec![];
        for generic in generics.params.iter() {
            match &generic {
                GenericParam::Const(_) => {}
                GenericParam::Type(ty) => {
                    if let Some(generic) = Nature::extract_generic(ty, None)? {
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
                        if let Some(generic) = Nature::extract_generic(ty, None)? {
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
pub trait Extract<T> {
    fn extract(t: T, context: Context) -> Result<Nature, E>;
}

pub trait VariableTokenStream {
    fn variable_token_stream(&self, var_name: &str, err: Option<&Nature>)
        -> Result<TokenStream, E>;
}

pub trait RustTypeName {
    fn rust_type_name(&self) -> Result<String, E>;
}

impl VariableTokenStream for Nature {
    fn variable_token_stream(
        &self,
        var_name: &str,
        err: Option<&Nature>,
    ) -> Result<TokenStream, E> {
        match self {
            Nature::Composite(v) => v.variable_token_stream(var_name, err),
            Nature::Primitive(v) => v.variable_token_stream(var_name, err),
            Nature::Refered(v) => v.variable_token_stream(var_name, err),
        }
    }
}

impl RustTypeName for Nature {
    fn rust_type_name(&self) -> Result<String, E> {
        match self {
            Nature::Composite(v) => v.rust_type_name(),
            Nature::Primitive(v) => v.rust_type_name(),
            Nature::Refered(v) => v.rust_type_name(),
        }
    }
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
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" | "usize" => {
                Nature::Primitive(Primitive::Number(origin.clone()))
            }
            "u64" | "i64" => Nature::Primitive(Primitive::BigInt(origin.clone())),
            "bool" => Nature::Primitive(Primitive::Boolean),
            "String" => Nature::Primitive(Primitive::String),
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
                "Vec" => Nature::Composite(composite::Composite::Vec(None)),
                "HashMap" => Nature::Composite(composite::Composite::HashMap(None, None)),
                "Option" => Nature::Composite(composite::Composite::Option(None)),
                "Result" => Nature::Composite(composite::Composite::Result(
                    None,
                    None,
                    context.exception_suppression()?,
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

impl Extract<&Punctuated<Type, Comma>> for Nature {
    fn extract(elements: &Punctuated<Type, Comma>, context: Context) -> Result<Nature, E> {
        if elements.is_empty() {
            Ok(Nature::Composite(Composite::Undefined))
        } else {
            let mut ty = Nature::Composite(composite::Composite::Tuple(vec![]));
            for element in elements.iter() {
                ty.bind(Nature::extract(element, context.clone())?)?;
            }
            Ok(ty)
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
            Type::Tuple(type_tuple) => Nature::extract(&type_tuple.elems, context),
            _ => Err(E::NotSupported),
        }
    }
}

impl Extract<Type> for Nature {
    fn extract(ty: Type, context: Context) -> Result<Nature, E> {
        Self::extract(&ty, context)
    }
}

fn get_fn_return(output: &ReturnType, context: &Context) -> Result<Option<Box<Nature>>, E> {
    Ok(match output {
        ReturnType::Default => Some(Box::new(Nature::Composite(Composite::Result(
            None,
            None,
            context.exception_suppression()?,
        )))),
        ReturnType::Type(_, ty) => {
            let return_ty = Nature::extract(*ty.clone(), context.clone())?;
            Some(
                if matches!(return_ty, Nature::Composite(Composite::Result(_, _, _))) {
                    Box::new(return_ty)
                } else {
                    Box::new(Nature::Composite(Composite::Result(
                        Some(Box::new(return_ty)),
                        None,
                        context.exception_suppression()?,
                    )))
                },
            )
        }
    })
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
        let out = get_fn_return(&fn_item.sig.output, &context)?;
        let constructor = if let Some(Nature::Refered(Refered::Ref(re, _))) = out.as_deref() {
            re == "Self"
        } else {
            false
        } || context.as_constructor();
        Ok(Self::Composite(Composite::Func(
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
        let out = get_fn_return(&fn_item.sig.output, &context)?;
        let constructor = if let Some(Nature::Refered(Refered::Ref(re, _))) = out.as_deref() {
            re == "Self"
        } else {
            false
        } || context.as_constructor();
        Ok(Self::Composite(Composite::Func(
            args,
            out,
            fn_item.sig.asyncness.is_some(),
            constructor,
        )))
    }
}
