pub mod composite;
pub mod primitives;

use composite::Composite;
use primitives::Primitive;
use proc_macro_error::abort;
use std::convert::From;
use syn::{
    punctuated::Punctuated,
    token::{Comma, PathSep},
    FnArg, GenericArgument, Ident, ImplItemFn, ItemFn, Pat, PathArguments, PathSegment, ReturnType,
    Type,
};

#[derive(Clone, Debug)]
pub enum Types {
    Primitive(primitives::Primitive),
    Composite(composite::Composite),
}

impl Types {
    fn bind(&mut self, ty: Types) {
        match self {
            Self::Primitive(_) => abort!("error", "Primitive type cannot be bound"),
            Self::Composite(othr) => match othr {
                composite::Composite::HashMap(k, v) => {
                    if k.is_none() {
                        if let Self::Primitive(p) = ty {
                            let _ = k.insert(p);
                        } else {
                            abort!("error", "HashMap can use as key only Primitive type")
                        }
                    } else if v.is_none() {
                        let _ = v.insert(Box::new(ty));
                    } else {
                        abort!("error", "HashMap entity already has been bound")
                    }
                }
                composite::Composite::Option(o) => {
                    if o.is_some() {
                        abort!("error", "HashMap entity already has been bound")
                    } else {
                        let _ = o.insert(Box::new(ty));
                    }
                }
                composite::Composite::Tuple(tys) => {
                    tys.push(Box::new(ty));
                }
                composite::Composite::Vec(v) => {
                    if v.is_some() {
                        abort!("error", "Vec entity already has been bound")
                    } else {
                        let _ = v.insert(Box::new(ty));
                    }
                }
                _ => {
                    abort!("error", "Unsupported type")
                }
            },
        }
    }
}

impl From<&GenericArgument> for Types {
    fn from(arg: &GenericArgument) -> Self {
        match arg {
            GenericArgument::Type(ty) => Types::from(ty),
            _ => abort!(arg, "Not supported"),
        }
    }
}

impl From<&Ident> for Types {
    fn from(ident: &Ident) -> Self {
        match ident.to_string().as_str() {
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" => Types::Primitive(Primitive::Number),
            "u64" | "i64" => Types::Primitive(Primitive::BigInt),
            "bool" => Types::Primitive(Primitive::Boolean),
            "String" => Types::Primitive(Primitive::String),
            a => Types::Composite(Composite::RefByName(a.to_string())),
        }
    }
}

impl From<&Punctuated<PathSegment, PathSep>> for Types {
    fn from(segments: &Punctuated<PathSegment, PathSep>) -> Self {
        if segments.len() > 1 {
            abort!(
                segments,
                "Not supported Other Type for more than 1 PathSegment"
            )
        }
        if let Some(segment) = segments.first() {
            let mut ty = match segment.ident.to_string().as_str() {
                "Vec" => Types::Composite(composite::Composite::Vec(None)),
                "HashMap" => Types::Composite(composite::Composite::HashMap(None, None)),
                "Option" => Types::Composite(composite::Composite::Option(None)),
                _ => {
                    abort!(segment, "Only Vec, HashMap and Option are supported")
                }
            };
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    for arg in args.args.iter() {
                        ty.bind(Types::from(arg));
                    }
                }
                _ => abort!(segment, "Not supported"),
            }
            ty
        } else {
            abort!(
                segments,
                "For not primitive types expected at least one segment"
            )
        }
    }
}

impl From<&Punctuated<Type, Comma>> for Types {
    fn from(elements: &Punctuated<Type, Comma>) -> Self {
        let mut ty = Types::Composite(composite::Composite::Tuple(vec![]));
        for element in elements.iter() {
            ty.bind(Types::from(element));
        }
        ty
    }
}

impl From<&Type> for Types {
    fn from(ty: &Type) -> Self {
        match ty {
            Type::Path(type_path) => {
                if let Some(ident) = type_path.path.get_ident() {
                    Types::from(ident)
                } else {
                    Types::from(&type_path.path.segments)
                }
            }
            Type::Tuple(type_tuple) => Types::from(&type_tuple.elems),
            _ => abort!(
                ty,
                "This type is not supported. Use #[tslink(ignore)] to ignore it"
            ),
        }
    }
}

impl From<Type> for Types {
    fn from(ty: Type) -> Self {
        Self::from(&ty)
    }
}

impl From<&ImplItemFn> for Types {
    fn from(fn_item: &ImplItemFn) -> Self {
        let name = fn_item.sig.ident.to_string();
        let mut args = vec![];
        for fn_arg in fn_item.sig.inputs.iter() {
            if let FnArg::Typed(ty) = fn_arg {
                let arg_name = if let Pat::Ident(id) = *ty.pat.clone() {
                    id.ident.to_string()
                } else {
                    abort!(fn_item, "Cannot find ident for FnArg");
                };
                args.push((arg_name, Box::new(Types::from(*ty.ty.clone()))));
            }
        }
        let out = match &fn_item.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => Some(Box::new(Self::from(*ty.clone()))),
        };
        Self::Composite(composite::Composite::Func(
            name,
            args,
            out,
            fn_item.sig.asyncness.is_some(),
        ))
    }
}

impl From<&ItemFn> for Types {
    fn from(fn_item: &ItemFn) -> Self {
        let name = fn_item.sig.ident.to_string();
        let mut args = vec![];
        for fn_arg in fn_item.sig.inputs.iter() {
            if let FnArg::Typed(ty) = fn_arg {
                let arg_name = if let Pat::Ident(id) = *ty.pat.clone() {
                    id.ident.to_string()
                } else {
                    abort!(fn_arg, "Cannot find ident for FnArg");
                };
                args.push((arg_name, Box::new(Types::from(*ty.ty.clone()))));
            }
        }
        let out = match &fn_item.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => Some(Box::new(Self::from(*ty.clone()))),
        };
        Self::Composite(composite::Composite::Func(
            name,
            args,
            out,
            fn_item.sig.asyncness.is_some(),
        ))
    }
}
