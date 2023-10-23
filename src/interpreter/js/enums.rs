use super::Interpreter;
use crate::{
    defs::{enums::Enums, Entities},
    interpreter::Offset,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

impl Interpreter for Enums {
    fn declaration(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), std::io::Error> {
        if !self.is_flat() {
            return Ok(());
        }
        let name = self.name.clone();
        buf.write_all(format!("{}export const {name} = Object.freeze({{\n", offset).as_bytes())?;
        for (i, variant) in self.variants.iter().enumerate() {
            let v_name = &variant.name;
            buf.write_all(
                format!("{}{v_name}: {i}, \"{i}\": \"{v_name}\",\n", offset.inc()).as_bytes(),
            )?;
        }
        buf.write_all(format!("{}}});\n", offset).as_bytes())?;
        Ok(())
    }
}
