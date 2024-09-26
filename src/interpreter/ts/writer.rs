use crate::{
    interpreter::ts::{Export, Import},
    TS_IMPORTS,
};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufWriter, Error, ErrorKind, Write},
    path::PathBuf,
};

pub struct Writer {
    buf_writer: BufWriter<File>,
    file_name: PathBuf,
    location: PathBuf,
    buffer: String,
    imports: HashSet<Import>,
}

impl Writer {
    pub fn new(file: File, file_name: PathBuf) -> Result<Self, Error> {
        let location = file_name
            .parent()
            .ok_or(Error::new(
                ErrorKind::NotFound,
                format!("Fail to find parent of: {}", file_name.display()),
            ))?
            .to_path_buf();
        Ok(Self {
            buf_writer: BufWriter::new(file),
            file_name,
            location,
            buffer: String::new(),
            imports: HashSet::new(),
        })
    }
    pub fn write_all(&mut self) -> Result<(), Error> {
        for import in self.imports.iter() {
            self.buf_writer
                .write_all(format!("{import}\n").as_bytes())?;
        }
        self.buf_writer.write_all(self.buffer.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }
    pub fn push<S: AsRef<str>>(&mut self, data: S) {
        self.buffer.push_str(data.as_ref());
    }
    pub fn add_import<N: AsRef<str>, M: AsRef<str>>(
        &mut self,
        name: N,
        module: M,
    ) -> Result<(), Error> {
        let entity = Import {
            entity: name.as_ref().to_owned(),
            module: module.as_ref().to_owned(),
        };
        let content = fs::read_to_string(&self.file_name)?;
        if !content.contains(&entity.to_string()) {
            self.imports.insert(entity);
        }
        Ok(())
    }
    pub fn add_export<N: AsRef<str>, M: AsRef<str>>(
        &self,
        name: N,
        module: M,
    ) -> Result<(), Error> {
        let entity = Export {
            entity: name.as_ref().to_owned(),
            module: module.as_ref().to_owned(),
        };
        TS_IMPORTS
            .write()
            .map_err(|_| Error::new(ErrorKind::Other, "Fail access to a list of imports"))?
            .add(&self.location, entity);
        Ok(())
    }
}
