use crate::TS_IMPORTS;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Error, Write},
};
pub struct Writer {
    buf_writer: BufWriter<File>,
    buffer: String,
    imports: HashSet<(String, String, String)>,
}

impl Writer {
    pub fn new(file: File) -> Self {
        Self {
            buf_writer: BufWriter::new(file),
            buffer: String::new(),
            imports: HashSet::new(),
        }
    }
    pub fn write_all(&mut self) -> Result<(), Error> {
        for (name, module, _) in self.imports.iter() {
            self.buf_writer
                .write_all(format!("import {{ {name} }} from \"./{module}\";\n").as_bytes())?;
        }
        self.buf_writer.write_all(self.buffer.as_bytes())?;
        self.buf_writer.flush()?;
        Ok(())
    }
    pub fn push<S: AsRef<str>>(&mut self, data: S) {
        self.buffer.push_str(data.as_ref());
    }
    pub fn add_import<N: AsRef<str>, M: AsRef<str>, R: AsRef<str>>(
        &mut self,
        name: N,
        module: M,
        requester: R,
    ) {
        let entry = (
            name.as_ref().to_owned(),
            module.as_ref().to_owned(),
            requester.as_ref().to_owned(),
        );
        let mut global = TS_IMPORTS.write().expect("Read global imports list");
        if !global.contains(&entry) && !self.imports.contains(&entry) {
            self.imports.insert(entry.clone());
            global.insert(entry);
        }
    }
}
