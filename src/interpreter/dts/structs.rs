use super::Interpreter;
use crate::{
    defs::{
        structs::{Field, Structs},
        Entities,
    },
    interpreter::Offset,
    types::{composite::Composite, Types},
    CONFIG,
};
use std::{
    fs::File,
    io::{BufWriter, Error, ErrorKind, Write},
};

impl Interpreter for Field {
    fn declaration(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), Error> {
        buf.write_all(format!("{}: ", self.name).as_bytes())?;
        Ok(())
    }
    fn reference(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), Error> {
        let config = CONFIG
            .read()
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        if matches!(self.ty, Types::Composite(Composite::Func(_, _, _, _))) {
            let name = if let Some(args) = self.args.as_ref() {
                if let Some(name) = args.rename(&self.name) {
                    name
                } else {
                    self.name.clone()
                }
            } else {
                config.rename_method(&self.name)
            };
            buf.write_all(format!("{}", offset).as_bytes())?;
            self.ty.reference(entities, buf, offset)
        } else {
            let name = if let Some(args) = self.args.as_ref() {
                if let Some(name) = args.rename(&self.name) {
                    name
                } else {
                    self.name.clone()
                }
            } else {
                config.rename_field(&self.name)
            };
            buf.write_all(format!("{}{name}: ", offset,).as_bytes())?;
            self.ty.reference(entities, buf, offset)
        }
    }
}

impl Interpreter for Structs {
    fn declaration(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), Error> {
        buf.write_all(
            format!(
                "{}{} {} {{\n",
                offset,
                if self.args.as_class() {
                    "export declare class"
                } else {
                    "export interface"
                },
                self.name
            )
            .as_bytes(),
        )?;
        for (_name, field) in &self.fields {
            field.reference(entities, buf, offset.inc())?;
            buf.write_all(format!(";\n").as_bytes())?;
        }
        buf.write_all(format!("{}}}\n", offset).as_bytes())?;
        Ok(())
    }
    fn reference(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), Error> {
        buf.write_all(format!("{}", self.name).as_bytes())
    }
}
