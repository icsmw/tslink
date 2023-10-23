use super::Interpreter;
use crate::{
    defs::{
        structs::{Field, Structs},
        Entities,
    },
    interpreter::Offset,
    types::{composite::Composite, Types},
};
use std::{
    fs::File,
    io::{BufWriter, Error, Write},
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
        if matches!(self.ty, Types::Composite(Composite::Func(_, _, _, _))) {
            buf.write_all(format!("{}{}", offset, self.name).as_bytes())?;
        } else {
            buf.write_all(format!("{}{}: ", offset, self.name).as_bytes())?;
        }
        self.ty.reference(entities, buf, offset)
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
                    "export abstract class"
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
