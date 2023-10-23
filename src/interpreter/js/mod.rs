use crate::{
    defs::{Entities, Entity},
    interpreter::Offset,
    CONFIG,
};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{BufWriter, Error, ErrorKind, Write},
};

mod enums;

pub const EXT: &str = "js";

pub trait Interpreter {
    fn declaration(
        &self,
        _entities: &Entities,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), Error> {
        Ok(())
    }
}

pub fn write(entities: &Entities) -> Result<(), Error> {
    let path = CONFIG
        .read()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        .get_path(None, EXT)?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    File::create(&path)?;
    let file = OpenOptions::new().write(true).append(true).open(&path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(
        format!(
            "\"use strict\";
Object.defineProperty(exports, \"__esModule\", {{ value: true }});\n"
        )
        .as_bytes(),
    )?;
    for (_name, entity) in entities.iter() {
        if let Entity::Enum(enm) = entity {
            if enm.is_flat() {
                enm.declaration(&entities, &mut buf_writer, Offset::new())?;
            }
        }
    }
    buf_writer.write_all(format!("const {{ ").as_bytes())?;
    for (i, (name, entity)) in entities.iter().enumerate() {
        let written = match entity {
            Entity::Struct(strct) => {
                if strct.args.as_class() {
                    buf_writer.write_all(format!("{}", strct.name).as_bytes())?;
                    true
                } else {
                    false
                }
            }
            Entity::Enum(_) => false,
            Entity::Detached(_detached) => {
                buf_writer.write_all(format!("{}", name).as_bytes())?;
                true
            }
        };
        if written && i < entities.len() - 1 {
            buf_writer.write_all(format!(", ").as_bytes())?;
        }
    }
    buf_writer.write_all(format!(" }} = native();").as_bytes())?;
    for (name, entity) in entities.iter() {
        if let Some(name) = match entity {
            Entity::Struct(strct) => {
                if strct.args.as_class() {
                    Some(&strct.name)
                } else {
                    None
                }
            }
            Entity::Enum(enm) => {
                if enm.is_flat() {
                    Some(&enm.name)
                } else {
                    None
                }
            }
            Entity::Detached(_detached) => Some(name),
        } {
            buf_writer.write_all(format!("\nexports.{name} = {name};").as_bytes())?;
        }
    }
    buf_writer.flush()?;
    Ok(())
}
