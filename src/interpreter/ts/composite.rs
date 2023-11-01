use super::Interpreter;
use crate::{
    error::E,
    interpreter::Offset,
    nature::{Composite, Nature, Natures, Refered},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

impl Interpreter for Composite {
    fn reference(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Self::Vec(ty) => {
                if let Some(ty) = ty {
                    ty.reference(natures, buf, offset)?;
                    buf.write_all("[]".as_bytes())?;
                } else {
                    return Err(E::Parsing(String::from(
                        "Type Vec doesn't include reference to type",
                    )));
                }
            }
            Self::HashMap(key, ty) => {
                if let (Some(key), Some(ty)) = (key, ty) {
                    buf.write_all("Map<".as_bytes())?;
                    key.reference(natures, buf, offset.clone())?;
                    buf.write_all(", ".as_bytes())?;
                    ty.reference(natures, buf, offset)?;
                    buf.write_all(">".as_bytes())?;
                } else {
                    return Err(E::Parsing(String::from(
                        "Type HashMap doesn't include reference to type or key",
                    )));
                }
            }
            Self::Func(args, out, asyncness, constructor) => {
                buf.write_all("(".as_bytes())?;
                for (i, nature) in args.iter().enumerate() {
                    if let Nature::Refered(Refered::FuncArg(name, _context, nature, _)) = nature {
                        buf.write_all(format!("{name}: ").as_bytes())?;
                        nature.reference(natures, buf, offset.clone())?;
                        if i < args.len() - 1 {
                            buf.write_all(", ".as_bytes())?;
                        }
                    } else {
                        return Err(E::Parsing(String::from(
                            "Only Refered::FuncArg can be used as function's arguments",
                        )));
                    }
                }
                if *constructor {
                    buf.write_all(")".as_bytes())?;
                    return Ok(());
                }
                buf.write_all("): ".as_bytes())?;
                if *asyncness {
                    buf.write_all("Promise<".as_bytes())?;
                }
                if let Some(out) = out {
                    out.reference(natures, buf, offset.clone())?;
                } else {
                    buf.write_all("void".as_bytes())?;
                }
                if *asyncness {
                    buf.write_all(">".as_bytes())?;
                }
            }
            Self::Tuple(tys) => {
                buf.write_all("[".as_bytes())?;
                let last = tys.len() - 1;
                for (i, ty) in tys.iter().enumerate() {
                    ty.reference(natures, buf, offset.clone())?;
                    if i < last {
                        buf.write_all(", ".as_bytes())?;
                    }
                }
                buf.write_all("]".as_bytes())?;
            }
            Self::Option(ty) => {
                if let Some(ty) = ty {
                    ty.reference(natures, buf, offset)?;
                    buf.write_all(" | undefined".as_bytes())?;
                } else {
                    return Err(E::Parsing(String::from(
                        "Type Option doesn't include reference to type",
                    )));
                }
            }
            Self::Result(res, err) => {
                if let (Some(res), Some(err)) = (res, err) {
                    res.reference(natures, buf, offset.clone())?;
                    buf.write_all(" | ".as_bytes())?;
                    err.reference(natures, buf, offset)?;
                } else if let Some(res) = res {
                    res.reference(natures, buf, offset)?;
                    buf.write_all(" | undefined".as_bytes())?;
                } else if let Some(err) = err {
                    buf.write_all("undefined | ".as_bytes())?;
                    err.reference(natures, buf, offset)?;
                } else {
                    buf.write_all("undefined".as_bytes())?;
                }
            }
            Self::Undefined => {
                buf.write_all("undefined".as_bytes())?;
            }
        }
        Ok(())
    }
}
