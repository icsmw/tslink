use super::Interpreter;
use crate::{
    error::E,
    interpreter::{ts::Writer, Offset},
    nature::{Composite, Nature, Natures, Refered, TypeAsString},
};

impl Interpreter for Composite {
    fn reference(
        &self,
        natures: &Natures,
        buf: &mut Writer,
        offset: Offset,
        parent: Option<String>,
    ) -> Result<(), E> {
        match self {
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
                    if let Nature::Refered(Refered::FuncArg(name, _context, nature, _)) = nature {
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
                    buf.push(format!(" | {}", err_ext));
                }
                if res.is_none() && *exception_suppression {
                    buf.push(format!("{} | void", err_ext));
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
